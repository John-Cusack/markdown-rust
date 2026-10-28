//! Port of `markdown/extensions/legacy_attrs.py`
//! (`LegacyAttrs` treeprocessor, priority 15).

use crate::consts::compile;
use crate::context::Context;
use crate::model::{Element, Text};
use crate::patterns_gen as P;

fn attr_re() -> fancy_regex::Regex {
    compile(P::RE_EXT_LEGACY_ATTRS_ATTR_RE)
}

/// `LegacyAttrs.run`: `{@key=value}` in `alt` attributes, text and
/// tails sets the attribute and is removed (matches are dropped, as
/// upstream's callback returns `None`).
pub fn run_tree(ctx: &Context, root: &mut Element) {
    fn walk(root: &Element, path: &[usize], out: &mut Vec<Vec<usize>>) {
        out.push(path.to_vec());
        let node = {
            let mut n = root;
            for &i in path {
                n = &n.children[i];
            }
            n
        };
        for i in 0..node.children.len() {
            let mut sub = path.to_vec();
            sub.push(i);
            walk(root, &sub, out);
        }
    }
    let mut paths = Vec::new();
    walk(root, &[], &mut paths);
    let _ = ctx;
    for p in paths {
        let mut node: &mut Element = &mut *root;
        for &i in &p {
            node = &mut node.children[i];
        }
        // `alt` attribute first (upstream order: alt, text, tail).
        if node.get_attr("alt").is_some() {
            let alt = node.get_attr("alt").unwrap().to_string();
            let new_alt = process_text(node, &alt);
            node.set_attr("alt", &new_alt);
        }
        if let Some(Text::Normal(t)) = node.text.clone() {
            node.text = Some(Text::Normal(process_text(node, &t)));
        }
        if let Some(Text::Normal(t)) = node.tail.clone() {
            node.tail = Some(Text::Normal(process_text(node, &t)));
        }
    }
}

/// Apply `{@k=v}` substitutions to one string, setting attributes.
fn process_text(el: &mut Element, txt: &str) -> String {
    let re = attr_re();
    let mut out = String::new();
    let mut pos = 0usize;
    let mut caps_iter = re.captures_iter(txt);
    loop {
        let next = caps_iter.next();
        let caps = match next {
            Some(Ok(c)) => c,
            _ => break,
        };
        let m = caps.get(0).unwrap();
        out.push_str(&txt[pos..m.start()]);
        let key = caps.get(1).unwrap().as_str();
        let val: String = caps
            .get(2)
            .unwrap()
            .as_str()
            .replace('\n', " ");
        el.set_attr(&crate::ext::attr_list::sanitize_name(key), &val);
        pos = m.end();
    }
    out.push_str(&txt[pos..]);
    out
}
