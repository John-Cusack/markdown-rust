//! Port of `markdown/extensions/meta.py` (`meta` preprocessor, priority 27).
//!
//! Parses `Key: value` (and `    continuation`) lines at the document
//! start into `md.Meta`. The facade writes `ctx.meta` back after render.

use crate::consts::compile;
use crate::context::Context;
use crate::patterns_gen as P;

fn meta_re() -> fancy_regex::Regex {
    compile(P::RE_EXT_META_META_RE)
}
fn meta_more_re() -> fancy_regex::Regex {
    compile(P::RE_EXT_META_META_MORE_RE)
}
fn begin_re() -> fancy_regex::Regex {
    compile(P::RE_EXT_META_BEGIN_RE)
}
fn end_re() -> fancy_regex::Regex {
    compile(P::RE_EXT_META_END_RE)
}

/// `MetaPreprocessor.run`.
pub fn run(ctx: &mut Context, mut lines: Vec<String>) -> Vec<String> {
    let mut meta: Vec<(String, Vec<String>)> = Vec::new();
    let mut key: Option<String> = None;
    if !lines.is_empty()
        && begin_re()
            .find(&lines[0])
            .ok()
            .flatten()
            .map(|m| m.start() == 0)
            .unwrap_or(false)
    {
        lines.remove(0);
    }
    while !lines.is_empty() {
        let line = lines.remove(0);
        let m1 = meta_re()
            .find(&line)
            .ok()
            .flatten()
            .filter(|m| m.start() == 0);
        if line.trim().is_empty()
            || end_re()
                .find(&line)
                .ok()
                .flatten()
                .map(|m| m.start() == 0)
                .unwrap_or(false)
        {
            break;
        }
        if let Some(m) = m1 {
            let caps = meta_re().captures(&line).unwrap().unwrap();
            let k = caps.get(1).unwrap().as_str().to_lowercase();
            let k = k.trim().to_string();
            let v = caps.get(2).unwrap().as_str().trim().to_string();
            let _ = m;
            key = Some(k.clone());
            match meta.iter_mut().find(|(kk, _)| *kk == k) {
                Some((_, vs)) => vs.push(v),
                None => meta.push((k, vec![v])),
            }
        } else {
            let m2 = meta_more_re()
                .find(&line)
                .ok()
                .flatten()
                .filter(|m| m.start() == 0);
            if let (Some(m), Some(k)) = (m2, key.clone()) {
                let caps = meta_more_re().captures(&line).unwrap().unwrap();
                let v = caps.get(1).unwrap().as_str().trim().to_string();
                let _ = m;
                match meta.iter_mut().find(|(kk, _)| *kk == k) {
                    Some((_, vs)) => vs.push(v),
                    None => meta.push((k, vec![v])),
                }
            } else {
                lines.insert(0, line);
                break;
            }
        }
    }
    for (k, vs) in meta {
        for v in vs {
            ctx.meta_append(k.clone(), v);
        }
    }
    lines
}
