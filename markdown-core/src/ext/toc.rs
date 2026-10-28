//! Port of `markdown/extensions/toc.py`
//! (`toc` treeprocessor, priority 5).
//!
//! Builds the nested token list, `md.toc` div and per-header anchors /
//! permalinks. Custom `slugify` callables decline (the facade only sends
//! the `default` / `unicode` markers); everything else is scalar.

use crate::RenderError;
use crate::consts::compile;
use crate::context::Context;
use crate::model::{Element, Text};


fn header_rgx() -> fancy_regex::Regex {
    compile("[Hh][123456]")
}

fn idcount_re() -> fancy_regex::Regex {
    compile("^(.*)_([0-9]+)$")
}

/// `slugify(value, sep, unicode)`.
fn slugify(value: &str, sep: &str, unicode: bool) -> String {
    use unicode_normalization::UnicodeNormalization;
    let mut v = value.to_string();
    if !unicode {
        v = v.nfkd().filter(|c| c.is_ascii()).collect();
    }
    let re_keep = compile(r"[^\w\s-]");
    v = re_keep.replace_all(&v, "").into_owned();
    v = v.trim().to_lowercase();
    let pat = format!("[{}\\s]+", regex::escape(sep));
    let re_sep = compile(&pat);
    re_sep.replace_all(&v, sep).into_owned()
}

fn slugify_unicode(value: &str, sep: &str) -> String {
    slugify(value, sep, true)
}

/// `unique(id, ids)`.
fn unique(id: &str, ids: &mut std::collections::HashSet<String>) -> String {
    let mut id = id.to_string();
    loop {
        if !ids.contains(&id) && !id.is_empty() {
            break;
        }
        let m = idcount_re().captures(&id).unwrap();
        if let Some(m) = m {
            if m.get(0).unwrap().start() == 0 {
                let base = m.get(1).unwrap().as_str().to_string();
                let n: u64 = m.get(2).unwrap().as_str().parse().unwrap_or(0);
                id = format!("{base}_{}", n + 1);
                continue;
            }
        }
        id = format!("{id}_1");
    }
    ids.insert(id.clone());
    id
}

/// `html.unescape` (full HTML5 algorithm over `entities_html5_gen`).
pub(crate) fn html_unescape(s: &str) -> String {
    if !s.contains('&') {
        return s.to_string();
    }
    let re = compile("&(#[0-9]+;?|#[xX][0-9a-fA-F]+;?|[^\\t\\n\\r <&#;]{1,32};?)");
    re.replace_all(s, |caps: &fancy_regex::Captures| {
        replace_charref(caps.get(1).unwrap().as_str())
    })
    .into_owned()
}

fn replace_charref(s: &str) -> String {
    if let Some(hexed) = s.strip_prefix('#') {
        let hex = hexed.starts_with('x') || hexed.starts_with('X');
        let digits = if hex { &hexed[1..] } else { hexed };
        let digits = digits.strip_suffix(';').unwrap_or(digits);
        let num = u32::from_str_radix(digits, if hex { 16 } else { 10 }).unwrap_or(0);
        if let Some(r) = crate::entities_invalid_gen::INVALID_CHARREFS
            .iter()
            .find(|(n, _)| *n == num)
        {
            return r.1.to_string();
        }
        if (0xD800..=0xDFFF).contains(&num) || num > 0x10FFFF {
            return '\u{FFFD}'.to_string();
        }
        if crate::entities_invalid_gen::INVALID_CODEPOINTS
            .binary_search(&num)
            .is_ok()
        {
            return String::new();
        }
        return char::from_u32(num).unwrap_or('\u{FFFD}').to_string();
    }
    let table = crate::entities_html5_gen::HTML5;
    if let Ok(i) = table.binary_search_by(|e| e.0.cmp(s)) {
        return table[i].1.to_string();
    }
    // Longest matching name.
    let b = s.as_bytes();
    let mut x = s.len().saturating_sub(1);
    while x > 1 {
        while x > 1 && !s.is_char_boundary(x) {
            x -= 1;
        }
        if x <= 1 {
            break;
        }
        let prefix = &s[..x];
        if table.binary_search_by(|e| e.0.cmp(prefix)).is_ok() {
            let i = table.binary_search_by(|e| e.0.cmp(prefix)).unwrap();
            return format!("{}{}", table[i].1, &s[x..]);
        }
        let _ = b;
        x -= 1;
    }
    format!("&{s}")
}

/// `strip_tags`.
fn strip_tags(text: &str) -> String {
    let mut text = text.to_string();
    while let Some(start) = text.find("<!--") {
        let end = match text[start..].find("-->") {
            Some(i) => start + i + 3,
            None => break,
        };
        text = format!("{}{}", &text[..start], &text[end..]);
    }
    while let Some(start) = text.find('<') {
        let end = match text[start..].find('>') {
            Some(i) => start + i + 1,
            None => break,
        };
        text = format!("{}{}", &text[..start], &text[end..]);
    }
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// `escape_cdata`.
fn escape_cdata(text: &str) -> String {
    let mut t = text.to_string();
    if t.contains('&') {
        t = crate::consts::RE_AMP.replace_all(&t, "&amp;").into_owned();
    }
    if t.contains('<') {
        t = t.replace('<', "&lt;");
    }
    if t.contains('>') {
        t = t.replace('>', "&gt;");
    }
    t
}

/// `unescape` (STX-digit-ETX).
fn unescape_stx(text: &str) -> String {
    crate::consts::RE_UNESCAPE
        .replace_all(text, |caps: &fancy_regex::Captures| {
            let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
            char::from_u32(n).unwrap_or('\u{FFFD}').to_string()
        })
        .into_owned()
}

#[derive(Debug, Clone)]
struct TocEntry {
    level: u32,
    id: String,
    name: String,
    html: String,
    data_toc_label: String,
    children: Vec<TocEntry>,
}

/// `nest_toc_tokens` — faithful port (dict aliasing reproduced with
/// index paths; `last`/`parents`/`levels` evolve identically).
fn nest_toc_tokens(mut toc_list: Vec<TocEntry>) -> Vec<TocEntry> {
    let mut ordered: Vec<TocEntry> = Vec::new();
    if toc_list.is_empty() {
        return ordered;
    }
    let mut first = toc_list.remove(0);
    first.children = Vec::new();
    let mut levels = vec![first.level];
    ordered.push(first);
    let mut parents: Vec<Vec<usize>> = Vec::new();
    let mut last_path: Vec<usize> = vec![0];
    for mut t in toc_list {
        t.children = Vec::new();
        let current_level = t.level;
        if current_level < *levels.last().unwrap() {
            levels.pop();
            let mut to_pop = 0usize;
            for p in parents.iter().rev() {
                if current_level <= get_level(&ordered, p) {
                    to_pop += 1;
                } else {
                    break;
                }
            }
            if to_pop > 0 {
                levels.truncate(levels.len().saturating_sub(to_pop));
                parents.truncate(parents.len().saturating_sub(to_pop));
            }
            levels.push(current_level);
        }
        if current_level == *levels.last().unwrap() {
            if let Some(pp) = parents.last().cloned() {
                let parent = get_mut(&mut ordered, &pp);
                parent.children.push(t);
                let mut np = pp.clone();
                np.push(parent.children.len() - 1);
                last_path = np;
            } else {
                ordered.push(t);
                last_path = vec![ordered.len() - 1];
            }
        } else {
            let lp = last_path.clone();
            let parent = get_mut(&mut ordered, &lp);
            parent.children.push(t);
            let mut np = lp.clone();
            np.push(parent.children.len() - 1);
            parents.push(lp);
            levels.push(current_level);
            last_path = np;
        }
    }
    ordered
}

fn get_level(ordered: &[TocEntry], path: &[usize]) -> u32 {
    let mut node = None;
    let mut cur = ordered;
    for (i, &p) in path.iter().enumerate() {
        if i == path.len() - 1 {
            node = Some(&cur[p]);
        } else {
            cur = &cur[p].children;
        }
    }
    node.map(|n| n.level).unwrap_or(0)
}

fn get_mut<'x>(ordered: &'x mut [TocEntry], path: &[usize]) -> &'x mut TocEntry {
    // Navigate to parent then index (path always non-empty here).
    let mut cur: &'x mut [TocEntry] = ordered;
    for (i, &p) in path.iter().enumerate() {
        if i + 1 == path.len() {
            return &mut cur[p];
        }
        cur = &mut cur[p].children;
    }
    unreachable!()
}
/// Parsed toc configs.
struct TocCfg {
    marker: String,
    title: String,
    title_class: String,
    toc_class: String,
    anchorlink: bool,
    anchorlink_class: String,
    use_permalinks: Perma,
    permalink_class: String,
    permalink_title: String,
    permalink_leading: bool,
    base_level: u32,
    slugify_unicode: bool,
    sep: String,
    toc_top: u32,
    toc_bottom: u32,
}

#[derive(Debug, Clone)]
enum Perma {
    Off,
    Symbol,
    Text(String),
}

fn parse_bool_none(v: &crate::ConfigValue) -> Option<bool> {
    match v {
        crate::ConfigValue::Bool(b) => Some(*b),
        crate::ConfigValue::Int(i) => Some(*i != 0),
        // Upstream `parseBoolValue(s, fail_on_errors=False)` returns `None`
        // for unrecognized strings (including `""`); callers decide.
        crate::ConfigValue::Str(s) => match s.to_lowercase().as_str() {
            "true" | "yes" | "y" | "on" | "1" => Some(true),
            "false" | "no" | "n" | "off" | "0" | "none" => Some(false),
            _ => None,
        },
        crate::ConfigValue::StrList(l) => Some(!l.is_empty()),
        crate::ConfigValue::StrMap(m) => Some(!m.is_empty()),
        crate::ConfigValue::None => None,
    }
}

/// Upstream `permalink` tri-state: `parseBoolValue(v, False)` decides
/// on/off, but `None` (unparseable, including `""`) keeps the raw string
/// as custom link text; Python `None` means off.
fn permalink_cfg(v: Option<&crate::ConfigValue>) -> Result<Perma, RenderError> {
    let v = match v {
        None | Some(crate::ConfigValue::None) => return Ok(Perma::Off),
        Some(v) => v,
    };
    if v == &crate::ConfigValue::Bool(true) {
        return Ok(Perma::Symbol);
    }
    match parse_bool_none(v) {
        Some(true) => Ok(Perma::Symbol),
        Some(false) => Ok(Perma::Off),
        None => match v {
            crate::ConfigValue::Str(s) => Ok(Perma::Text(s.clone())),
            crate::ConfigValue::Int(i) => Ok(Perma::Text(i.to_string())),
            _ => Err(RenderError::Unsupported("toc permalink".into())),
        },
    }
}
fn read_cfg(ctx: &Context) -> Result<TocCfg, RenderError> {
    let g = |k: &str| ctx.ext_config("toc", k);
    let gs = |k: &str, d: &str| ctx.ext_str("toc", k, d);
    let anchorlink = match g("anchorlink") {
        Some(v) => parse_bool_none(v).ok_or_else(|| RenderError::Unsupported("toc anchorlink".into()))?,
        None => false,
    };
    let use_permalinks = permalink_cfg(g("permalink"))?;
    let permalink_leading = match g("permalink_leading") {
        Some(v) => parse_bool_none(v).unwrap_or(false),
        None => false,
    };
    // Upstream stores `int(baselevel) - 1`.
    let base_raw: u32 = match g("baselevel") {
        None => 1,
        Some(crate::ConfigValue::Int(n)) => (*n).max(0) as u32,
        Some(crate::ConfigValue::Bool(b)) => {
            if *b {
                1
            } else {
                0
            }
        }
        Some(crate::ConfigValue::Str(s)) => s
            .parse::<u32>()
            .map_err(|_| RenderError::Unsupported("toc baselevel".into()))?,
        _ => return Err(RenderError::Unsupported("toc baselevel".into())),
    };
    let base_level: u32 = base_raw.saturating_sub(1);
    let (toc_top, toc_bottom) = match g("toc_depth") {
        Some(crate::ConfigValue::Int(n)) => (1u32, *n as u32),
        Some(crate::ConfigValue::Str(s)) if s.contains('-') => {
            let mut it = s.splitn(2, '-');
            let t: u32 = it
                .next()
                .unwrap_or("1")
                .parse()
                .map_err(|_| RenderError::Unsupported("toc depth".into()))?;
            let b: u32 = it
                .next()
                .unwrap_or("6")
                .parse()
                .map_err(|_| RenderError::Unsupported("toc depth".into()))?;
            (t, b)
        }
        Some(crate::ConfigValue::Str(s)) => (
            1,
            s.parse()
                .map_err(|_| RenderError::Unsupported("toc depth".into()))?,
        ),
        _ => (1, 6),
    };
    let slugify_unicode = match ctx
        .ext_config("toc", "slugify")
        .map(|v| match v {
            crate::ConfigValue::Str(s) => s.clone(),
            _ => "default".to_string(),
        })
        .unwrap_or_else(|| "default".to_string())
        .as_str()
    {
        "unicode" => true,
        "default" => false,
        _ => return Err(RenderError::Unsupported("toc slugify".into())),
    };
    Ok(TocCfg {
        marker: gs("marker", "[TOC]"),
        title: gs("title", ""),
        title_class: gs("title_class", "toctitle"),
        toc_class: gs("toc_class", "toc"),
        anchorlink,
        anchorlink_class: gs("anchorlink_class", "toclink"),
        use_permalinks,
        permalink_class: gs("permalink_class", "headerlink"),
        permalink_title: gs("permalink_title", "Permanent link"),
        permalink_leading,
        base_level,
        slugify_unicode,
        sep: gs("separator", "-"),
        toc_top,
        toc_bottom,
    })
}

/// `TocTreeprocessor.run` — returns `(toc_html, toc_tokens)`.
pub fn run_tree(
    ctx: &mut Context,
    root: &mut Element,
) -> Result<(String, Vec<crate::TocToken>), RenderError> {
    let cfg = read_cfg(ctx)?;
    // used_ids: all pre-existing id attributes.
    let mut used_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
    {
        let mut stack = vec![root as *const Element];
        while let Some(p) = stack.pop() {
            let el = unsafe { &*p };
            if let Some(id) = el.get_attr("id") {
                used_ids.insert(id.to_string());
            }
            for c in &el.children {
                stack.push(c as *const _);
            }
        }
    }
    let mut tokens: Vec<TocEntry> = Vec::new();
    // Walk headers in document order.
 let mut header_paths: Vec<Path> = Vec::new();
    {
        fn walk(root: &Element, path: &mut Path, out: &mut Vec<Path>, rgx: &fancy_regex::Regex) {
            let node = {
                let mut n = root;
                for &i in path.iter() {
                    n = &n.children[i];
                }
                n
            };
            if rgx.find(&node.tag).ok().flatten().map(|m| m.start() == 0).unwrap_or(false) {
                out.push(path.clone());
            }
            for i in 0..node.children.len() {
                path.push(i);
                walk(root, path, out, rgx);
                path.pop();
            }
        }
        let rgx = header_rgx();
        walk(root, &mut Vec::new(), &mut header_paths, &rgx);
    }
    for hp in header_paths {
        // set_level
        {
            let h = nav_mut(root, &hp);
            let lvl: u32 = h.tag[1..].parse().unwrap_or(1);
            let mut l = lvl + cfg.base_level;
            if l > 6 {
                l = 6;
            }
            h.tag = format!("h{l}");
        }
        // innerhtml (remove fnrefs on a clone)
        let el_clone = nav(root, &hp).clone();
        let el_nofn = remove_fnrefs(&el_clone);
        let innerhtml = render_inner_html(ctx, &el_nofn);
        let mut name = strip_tags(&innerhtml);
        // id
        let existing_id = nav(root, &hp).get_attr("id").map(str::to_string);
        let elem_id = if let Some(id) = existing_id {
            id
        } else {
            let slug_src = html_unescape(&name);
            let slug = if cfg.slugify_unicode {
                slugify_unicode(&slug_src, &cfg.sep)
            } else {
                slugify(&slug_src, &cfg.sep, false)
            };
            let id = unique(&slug, &mut used_ids);
            nav_mut(root, &hp).set_attr("id", &id);
            id
        };
        // data-toc-label
        let mut data_label = String::new();
        let dtl = nav(root, &hp).get_attr("data-toc-label").map(str::to_string);
        if let Some(v) = dtl {
            // Upstream keeps the postprocessed label verbatim (no
            // strip/escape); only `name` is stripped + escaped.
            let run_once = run_postprocessors(ctx, &unescape_stx(&v));
            data_label = run_once.clone();
            name = escape_cdata(&strip_tags(&run_once));
            let h = nav_mut(root, &hp);
            h.attrib.retain(|(k, _)| k != "data-toc-label");
        }
        let lvl: u32 = nav(root, &hp).tag[1..].parse().unwrap_or(1);
        if lvl >= cfg.toc_top && lvl <= cfg.toc_bottom {
            tokens.push(TocEntry {
                level: lvl,
                id: unescape_stx(&elem_id),
                name: name.clone(),
                html: innerhtml.clone(),
                data_toc_label: data_label,
                children: Vec::new(),
            });
        }
        if cfg.anchorlink {
            add_anchor(nav_mut(root, &hp), &elem_id, &cfg.anchorlink_class);
        }
        if !matches!(cfg.use_permalinks, Perma::Off) {
            add_permalink(nav_mut(root, &hp), &elem_id, &cfg);
        }
        let _ = name;
    }
    let nested = nest_toc_tokens(tokens);
    // build_toc_div + marker replace + serialize + postprocess
    let mut div = Element::new("div");
    div.set_attr("class", &cfg.toc_class);
    if !cfg.title.is_empty() {
        let mut header = Element::new("span");
        if !cfg.title_class.is_empty() {
            header.set_attr("class", &cfg.title_class);
        }
        header.text = Some(Text::Normal(cfg.title.clone()));
        div.children.push(header);
    }
    build_ul(&nested, &mut div);
    crate::tree::prettify(ctx, &mut div);
    if !cfg.marker.is_empty() {
        replace_marker(root, div.clone(), &cfg.marker);
    }
    // `md.toc`: serialize the div itself + run postprocessors (mirrors
    // `md.serializer(div)` then each postprocessor, in order).
    let ser = crate::ser::serialize(&div, ctx.output_format);
    let toc = run_postprocessors(ctx, &ser);
    Ok((toc, convert_tokens(&nested)))
}

fn nav<'x>(root: &'x Element, path: &[usize]) -> &'x Element {
    let mut node = root;
    for &i in path {
        node = &node.children[i];
    }
    node
}

fn nav_mut<'x>(root: &'x mut Element, path: &[usize]) -> &'x mut Element {
    let mut node = root;
    for &i in path {
        node = &mut node.children[i];
    }
    node
}

type Path = Vec<usize>;
/// Toc takes no format-slot templates; nothing to validate. (The
/// permalink/backlink texts are used verbatim.)
pub(crate) fn validate_templates(_ctx: &Context) -> Result<(), RenderError> {
    Ok(())
}

fn convert_tokens(entries: &[TocEntry]) -> Vec<crate::TocToken> {
    entries
        .iter()
        .map(|e| crate::TocToken {
            level: e.level,
            id: e.id.clone(),
            name: e.name.clone(),
            html: e.html.clone(),
            data_toc_label: e.data_toc_label.clone(),
            children: convert_tokens(&e.children),
        })
        .collect()
}

fn build_ul(toc_list: &[TocEntry], parent: &mut Element) {
    let mut ul = Element::new("ul");
    for item in toc_list {
        let mut li = Element::new("li");
        let mut link = Element::new("a");
        link.text = Some(Text::Normal(item.name.clone()));
        link.set_attr("href", &format!("#{}", item.id));
        li.children.push(link);
        if !item.children.is_empty() {
            build_ul(&item.children, &mut li);
        }
        ul.children.push(li);
    }
    parent.children.push(ul);
}

/// `replace_marker` (skips headers/pre/code subtrees).
fn replace_marker(root: &mut Element, div: Element, marker: &str) {
    fn walk(root: &mut Element, path: &[usize], div: &Option<Element>, marker: &str) -> bool {
        let tag = nav(root, path).tag.clone();
        if header_rgx().find(&tag).ok().flatten().map(|m| m.start() == 0).unwrap_or(false)
            || tag == "pre"
            || tag == "code"
        {
            return false;
        }
        let n = nav(root, path).children.len();
        for i in 0..n {
            let mut sub = path.to_vec();
            sub.push(i);
            // Upstream filters the CHILD (never yields headers/pre/code,
            // never descends into them).
            let ctag = nav(root, &sub).tag.clone();
            if header_rgx().find(&ctag).ok().flatten().map(|m| m.start() == 0).unwrap_or(false)
                || ctag == "pre"
                || ctag == "code"
            {
                continue;
            }
            let t = nav(root, &sub).text.clone();
            if let Some(t) = t {
                if !t.is_atomic() && t.as_str().trim() == marker {
                    let has_el = !nav(root, &sub).children.is_empty();
                    if !has_el {
                        if let Some(d) = div {
                            let node = nav_mut(root, path);
                            node.children[i] = d.clone();
                        }
                        return true;
                    }
                }
            }
            if walk(root, &sub, div, marker) {
                return true;
            }
        }
        false
    }
    let d = Some(div);
    walk(root, &[], &d, marker);
}

fn add_anchor(c: &mut Element, elem_id: &str, anchorlink_class: &str) {
    let mut anchor = Element::new("a");
    anchor.text = c.text.take();
    anchor.set_attr("href", &format!("#{elem_id}"));
    anchor.set_attr("class", anchorlink_class);
    for child in std::mem::take(&mut c.children) {
        anchor.children.push(child);
    }
    c.children.push(anchor);
}

fn add_permalink(c: &mut Element, elem_id: &str, cfg: &TocCfg) {
    let mut permalink = Element::new("a");
    match &cfg.use_permalinks {
        // Upstream `"%spara;" % AMP_SUBSTITUTE`.
        Perma::Symbol => {
            permalink.text = Some(Text::Normal(format!("{}para;", crate::consts::AMP_SUBSTITUTE)));
        }
        Perma::Text(t) => {
            permalink.text = Some(Text::Normal(t.clone()));
        }
        Perma::Off => {}
    }
    permalink.set_attr("href", &format!("#{elem_id}"));
    permalink.set_attr("class", &cfg.permalink_class);
    if !cfg.permalink_title.is_empty() {
        permalink.set_attr("title", &cfg.permalink_title);
    }
    if cfg.permalink_leading {
        permalink.tail = c.text.take();
        c.text = Some(Text::Normal(String::new()));
        c.children.insert(0, permalink);
    } else {
        c.children.push(permalink);
    }
}

/// `render_inner_html(el, md)`.
fn render_inner_html(ctx: &Context, el: &Element) -> String {
    let text = unescape_stx(&crate::ser::serialize(el, ctx.output_format));
    let start = match text.find('>') {
        Some(i) => i + 1,
        None => return String::new(),
    };
    let end = match text.rfind('<') {
        Some(i) => i,
        None => return String::new(),
    };
    if end < start {
        return String::new();
    }
    let inner = text[start..end].trim().to_string();
    run_postprocessors(ctx, &inner)
}

/// `run_postprocessors(text, md)`.
fn run_postprocessors(ctx: &Context, text: &str) -> String {
    let mut out = crate::post::raw_html(ctx, text);
    if ctx.has_extension("footnotes") {
        out = crate::ext::footnotes::post(ctx, &out);
    }
    crate::post::amp_substitute(&out)
}

/// `remove_fnrefs` (deepcopy iff a `sup` exists).
fn remove_fnrefs(el: &Element) -> Element {
    fn has_sup(el: &Element) -> bool {
        if el.tag == "sup" {
            return true;
        }
        el.children.iter().any(has_sup)
    }
    if !has_sup(el) {
        return el.clone();
    }
    let mut out = el.clone();
    // Collect parent paths containing sup descendants (document order).
    // Walk every element as a potential parent, children reversed
    // (matches upstream per-parent loop).
    fn strip_parent(el: &mut Element) {
        let mut carry = String::new();
        let mut i = el.children.len();
        while i > 0 {
            i -= 1;
            let is_sup = el.children[i].tag == "sup"
                && el.children[i]
                    .get_attr("id")
                    .map(|s| s.starts_with("fnref"))
                    .unwrap_or(false);
            if is_sup {
                let tail = el.children[i].tail.take().map(|t| t.into_string()).unwrap_or_default();
                carry = format!("{tail}{carry}");
                el.children.remove(i);
            } else if !carry.is_empty() {
                let tail = el.children[i].tail.take().map(|t| t.into_string()).unwrap_or_default();
                el.children[i].tail = Some(Text::Normal(format!("{tail}{carry}")));
                carry.clear();
            }
        }
        if !carry.is_empty() {
            let text = el.text.take().map(|t| t.into_string()).unwrap_or_default();
            el.text = Some(Text::Normal(format!("{text}{carry}")));
        }
        for c in el.children.iter_mut() {
            strip_parent(c);
        }
    }
    strip_parent(&mut out);
    out
}
