//! Shared attribute-list helpers (`markdown/extensions/attr_list.py`):
//! `get_attrs_and_remainder`, `sanitize_name`.

use fancy_regex::Regex as F;

use crate::consts::compile;
use crate::patterns_gen as P;

fn scanner_res() -> Vec<F> {
    vec![
        compile(r#"[^ =}]+=".*?""#),
        compile(r#"[^ =}]+='.*?'"#),
        compile(r#"[^ =}]+=[^ =}]+"#),
        compile(r#"[^ =}]+"#),
        compile(r#" "#),
    ]
}

/// `re.Scanner` replication: first matching rule (in order) wins at each
/// position; a space is skipped; the first unmatched char stops the scan
/// and starts the remainder.
pub fn get_attrs_and_remainder(s: &str) -> (Vec<(String, String)>, String) {
    let pats = scanner_res();
    let mut attrs = Vec::new();
    let mut pos = 0usize;
    while pos < s.len() {
        let mut advanced = false;
        for (i, re) in pats.iter().enumerate() {
            if let Ok(Some(m)) = re.find_from_pos(s, pos) {
                if m.start() != pos {
                    continue;
                }
                let t = m.as_str();
                match i {
                    0 => {
                        // `_handle_double_quote`.
                        if let Some(eq) = t.find('=') {
                            attrs.push((
                                t[..eq].to_string(),
                                t[eq + 1..].trim_matches('"').to_string(),
                            ));
                        }
                    }
                    1 => {
                        if let Some(eq) = t.find('=') {
                            attrs.push((
                                t[..eq].to_string(),
                                t[eq + 1..].trim_matches('\'').to_string(),
                            ));
                        }
                    }
                    2 => {
                        if let Some(eq) = t.find('=') {
                            attrs.push((t[..eq].to_string(), t[eq + 1..].to_string()));
                        }
                    }
                    3 => {
                        if let Some(stripped) = t.strip_prefix('.') {
                            attrs.push((".".to_string(), stripped.to_string()));
                        } else if let Some(stripped) = t.strip_prefix('#') {
                            attrs.push(("id".to_string(), stripped.to_string()));
                        } else {
                            attrs.push((t.to_string(), t.to_string()));
                        }
                    }
                    _ => {}
                }
                pos = m.end();
                advanced = true;
                break;
            }
        }
        if !advanced {
            break;
        }
    }
    let mut remainder = s[pos..].to_string();
    // To keep historic behavior, discard all unparsable text prior to '}'.
    if let Some(index) = remainder.find('}') {
        remainder = remainder[index..].to_string();
    } else {
        remainder = String::new();
    }
    (attrs, remainder)
}

/// `sanitize_name` ("an XML Name, minus the `:`").
pub fn sanitize_name(name: &str) -> String {
    let re = compile(P::RE_EXT_ATTR_LIST_NAME_RE);
    re.replace_all(name, "_").into_owned()
}
