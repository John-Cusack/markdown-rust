//! Port of `markdown/extensions/fenced_code.py`
//! (`fenced_code_block` preprocessor, priority 25).
//!
//! The `codehilite`-with-pygments path declines to the pure-Python
//! fallback (no Pygments in Rust); the plain path and the
//! `attr_list`-aware `kv_pairs` path are ported. Crash-edges replicate
//! exactly: non-string `kv` values that upstream's `_escape_attrib_html`
//! rejects (`bool`, `None`) decline so the fallback raises the identical
//! `TypeError`; `hl_lines` lists stringify like upstream.

use std::collections::HashMap;

use crate::consts::compile;
use crate::context::Context;
use crate::patterns_gen as P;

fn fenced_re() -> fancy_regex::Regex {
    compile(P::RE_EXT_FENCED_CODE_FENCEDBLOCKPREPROCESSOR_FENCED_BLOCK_RE)
}

fn escape_fenced(s: &str) -> String {
    // `_escape`: `&`, `<`, `>`, `"` (note the quote: unlike `code_escape`).
    let mut t = s.replace('&', "&amp;");
    t = t.replace('<', "&lt;");
    t = t.replace('>', "&gt;");
    t = t.replace('"', "&quot;");
    t
}

fn escape_attrib(s: &str) -> String {
    let mut t = s.to_string();
    if t.contains('&') {
        t = crate::consts::RE_AMP.replace_all(&t, "&amp;").into_owned();
    }
    if t.contains('<') {
        t = t.replace('<', "&lt;");
    }
    if t.contains('>') {
        t = t.replace('>', "&gt;");
    }
    if t.contains('"') {
        t = t.replace('"', "&quot;");
    }
    t
}

fn parse_bool_value(v: &str) -> Option<bool> {
    // `parseBoolValue(v, fail_on_errors=False, preserve_none=True)`.
    match v.to_lowercase().as_str() {
        "none" => None,
        "true" | "yes" | "y" | "on" | "1" => Some(true),
        "false" | "no" | "n" | "off" | "0" => Some(false),
        _ => None,
    }
}

fn parse_hl_lines(expr: &str) -> Vec<u32> {
    if expr.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for w in expr.split_whitespace() {
        match w.parse::<u32>() {
            Ok(n) => out.push(n),
            Err(_) => return Vec::new(),
        }
    }
    out
}

#[derive(Debug, Clone)]
enum CfgVal {
    Str(String),
    Bool(Option<bool>),
    HlLines(Vec<u32>),
}

fn handle_attrs(attrs: &[(String, String)]) -> (String, Vec<String>, HashMap<String, CfgVal>) {
    let bool_options = ["linenums", "guess_lang", "noclasses", "use_pygments"];
    let mut id = String::new();
    let mut classes = Vec::new();
    let mut configs: HashMap<String, CfgVal> = HashMap::new();
    for (k, v) in attrs {
        if k == "id" {
            id = v.clone();
        } else if k == "." {
            classes.push(v.clone());
        } else if k == "hl_lines" {
            configs.insert(k.clone(), CfgVal::HlLines(parse_hl_lines(v)));
        } else if bool_options.contains(&k.as_str()) {
            configs.insert(k.clone(), CfgVal::Bool(parse_bool_value(v)));
        } else {
            configs.insert(k.clone(), CfgVal::Str(v.clone()));
        }
    }
    (id, classes, configs)
}

fn cfg_is_truthy(v: Option<&CfgVal>) -> bool {
    match v {
        Some(CfgVal::Bool(Some(b))) => *b,
        Some(CfgVal::Bool(None)) => false,
        Some(CfgVal::Str(s)) => !matches!(
            s.to_lowercase().as_str(),
            "false" | "no" | "n" | "off" | "0" | "none" | ""
        ),
        Some(CfgVal::HlLines(_)) => true,
        None => false,
    }
}

/// `FencedBlockPreprocessor.run`. Returns `Err(Unsupported)` when the
/// codehilite/pygments path would be taken, or when a `kv` value would
/// crash upstream's escaper (the fallback raises the identical error).
pub fn run(ctx: &mut Context, lines: Vec<String>) -> Result<Vec<String>, crate::RenderError> {
    use crate::RenderError;
    let codehilite_conf: Option<HashMap<String, String>> = if ctx.has_extension("codehilite") {
        let mut m = HashMap::new();
        for key in [
            "css_class",
            "linenums",
            "guess_lang",
            "noclasses",
            "use_pygments",
            "lang_prefix",
            "pygments_style",
        ] {
            if let Some(v) = ctx.ext_config("codehilite", key) {
                m.insert(
                    key.to_string(),
                    match v {
                        crate::ConfigValue::Bool(b) => b.to_string(),
                        crate::ConfigValue::Int(i) => i.to_string(),
                        crate::ConfigValue::Str(s) => s.clone(),
                        crate::ConfigValue::StrList(l) => l.join(","),
                        crate::ConfigValue::StrMap(kvs) => kvs
                            .iter()
                            .map(|(k, v)| format!("{k}={v}"))
                            .collect::<Vec<_>>()
                            .join(","),
                        crate::ConfigValue::None => String::new(),
                    },
                );
            }
        }
        Some(m)
    } else {
        None
    };
    let use_attr_list = ctx.has_extension("attr_list");
    let lang_prefix = ctx.ext_str("fenced_code", "lang_prefix", "language-");

    let mut text = lines.join("\n");
    let re = fenced_re();
    let mut index = 0usize;
    while let Ok(Some(m)) = re.captures_from_pos(&text, index) {
        let (ms, me) = (m.get(0).unwrap().start(), m.get(0).unwrap().end());
        let attrs_g = m.name("attrs").map(|g| g.as_str().to_string());
        let attrs_end = m.name("attrs").map(|g| g.end());
        let lang_g = m.name("lang").map(|g| g.as_str().to_string());
        let hl_g = m.name("hl_lines").map(|g| g.as_str().to_string());
        let code_g = m.name("code").map(|g| g.as_str().to_string()).unwrap_or_default();

        let mut lang: Option<String> = None;
        let mut id = String::new();
        let mut classes: Vec<String> = Vec::new();
        let mut config: HashMap<String, CfgVal> = HashMap::new();
        if let Some(attrs_s) = attrs_g {
            let (attrs, remainder) = crate::ext::attr_list::get_attrs_and_remainder(&attrs_s);
            if !remainder.is_empty() {
                // Invalid syntax: skip over this (explicit anti-infinite-loop).
                index = attrs_end.unwrap_or(me);
                continue;
            }
            let (i, c, cfg) = handle_attrs(&attrs);
            id = i;
            classes = c;
            config = cfg;
            if !classes.is_empty() {
                lang = Some(classes.remove(0));
            }
        } else {
            if let Some(l) = lang_g {
                if !l.is_empty() {
                    lang = Some(l);
                }
            }
            if let Some(h) = hl_g {
                config.insert("hl_lines".to_string(), CfgVal::HlLines(parse_hl_lines(&h)));
            }
        }

        // CodeHilite gate: `codehilite_conf and use_pygments and
        // config.get('use_pygments', True)`. When true the bridge has
        // already guaranteed Pygments is absent, so the plain
        // (non-Pygments) `hilite()` branch is exact; otherwise fall to
        // the raw `<pre><code>` path below.
        let ch_use_pygments = codehilite_conf
            .as_ref()
            .and_then(|c| c.get("use_pygments"))
            .map(|v| {
                !matches!(
                    v.to_lowercase().as_str(),
                    "false" | "no" | "n" | "off" | "0" | "none" | ""
                )
            })
            .unwrap_or(true);
        let cfg_use_pygments = cfg_is_truthy(config.get("use_pygments"));
        // Upstream default when absent: `config.get('use_pygments', True)`.
        let cfg_use_pygments =
            cfg_use_pygments || !config.contains_key("use_pygments");
        let hilite_gate = codehilite_conf.is_some() && ch_use_pygments && cfg_use_pygments;

        let code = if hilite_gate {
            // Plain `CodeHilite(...).hilite(shebang=False)`: merged
            // `local_config` (per-block overrides extension), classes
            // folded into `css_class`, `style` ignored (Pygments-only).
            let conf = codehilite_conf.as_ref().unwrap();
            let cfg_str = |k: &str| match config.get(k) {
                Some(CfgVal::Str(s)) => Some(s.clone()),
                _ => None,
            };
            let mut css_class = cfg_str("css_class")
                .or_else(|| conf.get("css_class").cloned())
                .unwrap_or_else(|| "codehilite".to_string());
            if !classes.is_empty() {
                css_class = format!("{} {css_class}", classes.join(" "));
            }
            let linenums = config
                .get("linenums")
                .map(|v| cfg_is_truthy(Some(v)))
                .or_else(|| {
                    conf.get("linenums").map(|v| {
                        !matches!(
                            v.to_lowercase().as_str(),
                            "false" | "no" | "n" | "off" | "0" | "none" | ""
                        )
                    })
                })
                .unwrap_or(false);
            let lang_prefix = cfg_str("lang_prefix")
                .or_else(|| conf.get("lang_prefix").cloned())
                .unwrap_or_else(|| "language-".to_string());
            crate::ext::codehilite::hilite_plain(
                &code_g,
                lang.as_deref(),
                &css_class,
                linenums,
                &lang_prefix,
                false,
            )
        } else {
        let mut id_attr = String::new();
        let mut lang_attr = String::new();
        let mut class_attr = String::new();
        let mut kv_pairs = String::new();
        if let Some(l) = &lang {
            lang_attr = format!(" class=\"{}{}\"", lang_prefix, escape_attrib(l));
        }
        if !classes.is_empty() {
            class_attr = format!(" class=\"{}\"", escape_attrib(&classes.join(" ")));
        }
        if !id.is_empty() {
            id_attr = format!(" id=\"{}\"", escape_attrib(&id));
        }
    if use_attr_list && !config.is_empty() && !cfg_is_truthy(config.get("use_pygments")) {
            for (k, v) in &config {
                if k != "use_pygments" {
                    match v {
                        CfgVal::Str(s) => {
                            kv_pairs.push_str(&format!(
                                " {}=\"{}\"",
                                crate::ext::attr_list::sanitize_name(k),
                                escape_attrib(s)
                            ));
                        }
                        CfgVal::HlLines(ns) => {
                            // Upstream stringifies the list via f-string.
                            let s = format!("{ns:?}");
                            kv_pairs.push_str(&format!(
                                " {}=\"{}\"",
                                crate::ext::attr_list::sanitize_name(k),
                                escape_attrib(&s)
                            ));
                        }
                        // `bool`/`None` crash upstream's escaper
                        // (`TypeError: cannot serialize ...`): decline so
                        // the fallback raises identically.
                        CfgVal::Bool(_) => {
                            return Err(RenderError::Unsupported(
                                "fenced kv non-string".into(),
                            ));
                        }
                    }
                }
            }
        }
        let escaped = escape_fenced(&code_g);
            format!("<pre{id_attr}{class_attr}><code{lang_attr}{kv_pairs}>{escaped}</code></pre>")
        };
        let ph = {
            let key = ctx.stash.len();
            ctx.stash.push(crate::context::StashEntry::Html(code));
            crate::consts::placeholder_html(key)
        };
        text = format!("{}\n{}\n{}", &text[..ms], ph, &text[me..]);
        index = ms + 1 + ph.len();
    }
    Ok(text.split('\n').map(str::to_string).collect())
}
