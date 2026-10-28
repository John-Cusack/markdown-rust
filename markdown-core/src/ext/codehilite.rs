//! Port of `markdown/extensions/codehilite.py` (plain path only).
//!
//! `hilite` treeprocessor (priority 30): code-unescapes `pre > code`
//! blocks, runs the plain (non-Pygments) highlight, stashes the result.
//! The Pygments path declines (no Pygments in Rust); the bridge only
//! routes here when Pygments is absent or `use_pygments` is falsy.

use crate::consts::compile;
use crate::context::Context;
use crate::model::{Element, Text};
use crate::patterns_gen as P;
use crate::RenderError;

fn shebang_re() -> fancy_regex::Regex {
    compile(P::RE_CODEHILITE_SHEBANG)
}

/// `CodeHilite._parseHeader` (only the parts the plain path needs).
fn parse_header(src: &str) -> (Option<String>, String, bool) {
    let mut lines: Vec<&str> = src.split('\n').collect();
    if lines.is_empty() {
        return (None, String::new(), false);
    }
    let fl = lines.remove(0);
    let m = match shebang_re().captures(fl) {
        Ok(Some(m)) if m.get(0).unwrap().start() == 0 => m,
        _ => {
            return (None, src.to_string(), false);
        }
    };
    let lang = m
        .name("lang")
        .map(|g| g.as_str().to_lowercase())
        .filter(|s| !s.is_empty());
    let has_path = m
        .name("path")
        .map(|g| !g.as_str().is_empty())
        .unwrap_or(false);
    let mut rest = lines;
    if has_path {
        rest.insert(0, fl);
    }
    let mut linenums = false;
    let shebang = m.name("shebang").is_some();
    if shebang {
        linenums = true;
    }
    let _ = m.name("hl_lines");
    (lang, rest.join("\n").trim_matches('\n').to_string(), linenums)
}

/// Plain (non-Pygments) `CodeHilite.hilite()`.
pub fn hilite_plain(
    src: &str,
    lang: Option<&str>,
    cssclass: &str,
    linenums: bool,
    lang_prefix: &str,
    shebang: bool,
) -> String {
    let src = src.trim_matches('\n').to_string();
    let (mut lang, mut src, mut linenums) = (lang.map(str::to_string), src, linenums);
    if lang.is_none() && shebang {
        let (l, s, ln) = parse_header(&src);
        lang = l;
        src = s;
        if ln {
            linenums = true;
        }
    }
    let mut txt = src.replace('&', "&amp;");
    txt = txt.replace('<', "&lt;");
    txt = txt.replace('>', "&gt;");
    txt = txt.replace('"', "&quot;");
    let mut classes = Vec::new();
    if let Some(l) = &lang {
        if !l.is_empty() {
            classes.push(format!("{lang_prefix}{l}"));
        }
    }
    if linenums {
        classes.push("linenums".to_string());
    }
    let class_str = if classes.is_empty() {
        String::new()
    } else {
        format!(" class=\"{}\"", classes.join(" "))
    };
    format!("<pre class=\"{cssclass}\"><code{class_str}>{txt}\n</code></pre>\n")
}

fn code_unescape(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

/// `HiliteTreeprocessor.run`. `pygments_active` mirrors
/// `bool(pygments and use_pygments)`: when true the whole render was
/// already declined upstream of here.
pub fn run_tree(
    ctx: &mut Context,
    root: &mut Element,
    cssclass: &str,
    linenums: bool,
    lang_prefix: &str,
) -> Result<(), RenderError> {
    // Collect `pre > code` targets first (indices stay valid: no removals).
    let mut targets: Vec<Vec<usize>> = Vec::new();
    fn walk(el: &Element, path: &[usize], out: &mut Vec<Vec<usize>>) {
        for (i, c) in el.children.iter().enumerate() {
            let mut p = path.to_vec();
            p.push(i);
            if c.tag == "pre" && c.children.len() == 1 && c.children[0].tag == "code" {
                out.push(p.clone());
            }
            walk(c, &p, out);
        }
    }
    walk(root, &[], &mut targets);
    for t in targets {
        let mut node: &mut Element = &mut *root;
        for &i in &t {
            node = &mut node.children[i];
        }
        let text = match node.children[0].text.take() {
            Some(Text::Normal(s)) | Some(Text::Atomic(s)) => s,
            None => continue,
        };
        let code = hilite_plain(
            &code_unescape(&text),
            None,
            cssclass,
            linenums,
            lang_prefix,
            true,
        );
        let key = ctx.stash.len();
        ctx.stash
            .push(crate::context::StashEntry::Html(code));
        let ph = crate::consts::placeholder_html(key);
        node.children.clear();
        node.tag = "p".to_string();
        node.text = Some(Text::Normal(ph));
    }
    Ok(())
}

/// Read plain-path config from the `codehilite` extension spec.
pub fn plain_config(ctx: &Context) -> (String, bool, String) {
    let cssclass = ctx.ext_str("codehilite", "css_class", "codehilite");
    let linenums = ctx.ext_bool("codehilite", "linenums", false)
        || matches!(
            ctx.ext_config("codehilite", "linenums"),
            Some(crate::ConfigValue::Str(s)) if s == "table" || s == "inline"
        );
    let lang_prefix = ctx.ext_str("codehilite", "lang_prefix", "language-");
    (cssclass, linenums, lang_prefix)
}
