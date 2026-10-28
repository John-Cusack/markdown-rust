//! Port of `markdown/extensions/smarty.py`.
//!
//! A dedicated inline pass (own pattern registry, tree priority 6)
//! running after the main inline pass. `SubstituteTextPattern` results
//! are stashed strings (entities restored by the final raw-html pass).

use fancy_regex::Regex as F;

use crate::consts::compile;
use crate::context::Context;
use crate::patterns_gen as P;

fn default_subs() -> Vec<(String, String)> {
    vec![
        ("mdash".to_string(), "&mdash;".to_string()),
        ("ndash".to_string(), "&ndash;".to_string()),
        ("ellipsis".to_string(), "&hellip;".to_string()),
        ("left-angle-quote".to_string(), "&laquo;".to_string()),
        ("right-angle-quote".to_string(), "&raquo;".to_string()),
        ("left-single-quote".to_string(), "&lsquo;".to_string()),
        ("right-single-quote".to_string(), "&rsquo;".to_string()),
        ("left-double-quote".to_string(), "&ldquo;".to_string()),
        ("right-double-quote".to_string(), "&rdquo;".to_string()),
    ]
}

fn subs(ctx: &Context) -> Vec<(String, String)> {
    let mut out = default_subs();
    if let Some(crate::ConfigValue::StrMap(over)) = ctx.ext_config("smarty", "substitutions") {
        for (k, v) in over {
            if let Some(slot) = out.iter_mut().find(|(kk, _)| kk == k) {
                slot.1 = v.clone();
            } else {
                out.push((k.clone(), v.clone()));
            }
        }
    }
    out
}

fn sub(_ctx: &Context, subs: &[(String, String)], key: &str) -> String {
    subs.iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| {
            default_subs()
                .into_iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v)
                .unwrap_or_default()
        })
        .clone()
}

/// A replacement part: capture group or literal (stashed) text.
#[derive(Debug, Clone)]
pub enum Repl {
    Group(usize),
    Text(String),
}

/// One smarty pattern: (priority, regex, replacements).
pub struct SmartyPat {
    pub prio: u32,
    pub re: F,
    pub replace: Vec<Repl>,
    /// Trigger-byte mask (bit positions match inline's T_* constants:
    /// bit9 `-`, bit1 `<`, bit13 `>`, bit11 `'`, bit12 `"`, bit10 `.`).
    pub mask: u16,
}

fn sp(pat: &str, replace: Vec<Repl>, prio: u32, mask: u16) -> (u32, SmartyPat) {
    (
        prio,
        SmartyPat {
            prio,
            re: compile(pat),
            replace,
            mask,
        },
    )
}

fn t(s: &str) -> Repl {
    Repl::Text(s.to_string())
}
fn g(i: usize) -> Repl {
    Repl::Group(i)
}

/// Build the smarty pattern list for this render (priority order).
pub fn patterns_for(ctx: &Context) -> Vec<(u32, SmartyPat)> {
    let subs = subs(ctx);
    let s = |k: &str| sub(ctx, &subs, k);
    let mut out = Vec::new();
    if ctx.ext_bool("smarty", "smart_dashes", true) {
        out.push(sp(P::RE_SMARTY_00_SUBSTITUTETEXTPATTERN, vec![t(&s("mdash"))], 50, 1 << 9));
        out.push(sp(P::RE_SMARTY_01_SUBSTITUTETEXTPATTERN, vec![t(&s("ndash"))], 45, 1 << 9));
    }
    if ctx.ext_bool("smarty", "smart_angled_quotes", false) {
        // Upstream `r'\<\<'` is literal `<<` in Python `re`; fancy-regex
        // reads `\<` as a word boundary, so spell the literal here.
        out.push(sp("(?s)<<", vec![t(&s("left-angle-quote"))], 40, 16));
        out.push(sp("(?s)>>", vec![t(&s("right-angle-quote"))], 35, 1 << 13));
    }
    if ctx.ext_bool("smarty", "smart_quotes", true) {
        let lsquo = s("left-single-quote");
        let rsquo = s("right-single-quote");
        let ldquo = s("left-double-quote");
        let rdquo = s("right-double-quote");
        let pats = [
            (P::RE_SMARTY_02_SUBSTITUTETEXTPATTERN, vec![t(&rsquo)], 30, 2048),
            (P::RE_SMARTY_03_SUBSTITUTETEXTPATTERN, vec![t(&rdquo)], 29, 4096),
            (P::RE_SMARTY_04_SUBSTITUTETEXTPATTERN, vec![t(&(ldquo.clone() + &lsquo))], 28, 6144),
            (P::RE_SMARTY_05_SUBSTITUTETEXTPATTERN, vec![t(&(lsquo.clone() + &ldquo))], 27, 6144),
            (P::RE_SMARTY_06_SUBSTITUTETEXTPATTERN, vec![t(&(rsquo.clone() + &rdquo))], 26, 6144),
            (P::RE_SMARTY_07_SUBSTITUTETEXTPATTERN, vec![t(&(rdquo.clone() + &rsquo))], 25, 6144),
            (P::RE_SMARTY_08_SUBSTITUTETEXTPATTERN, vec![t(&rsquo)], 24, 2048),
            (P::RE_SMARTY_09_SUBSTITUTETEXTPATTERN, vec![g(1), t(&lsquo)], 23, 2048),
            (P::RE_SMARTY_10_SUBSTITUTETEXTPATTERN, vec![t(&rsquo)], 22, 2048),
            (P::RE_SMARTY_11_SUBSTITUTETEXTPATTERN, vec![t(&rsquo), g(1)], 21, 2048),
            (P::RE_SMARTY_12_SUBSTITUTETEXTPATTERN, vec![t(&lsquo)], 20, 2048),
            (P::RE_SMARTY_13_SUBSTITUTETEXTPATTERN, vec![g(1), t(&ldquo)], 19, 4096),
            (P::RE_SMARTY_14_SUBSTITUTETEXTPATTERN, vec![t(&rdquo)], 18, 4096),
            (P::RE_SMARTY_15_SUBSTITUTETEXTPATTERN, vec![t(&rdquo)], 17, 4096),
            (P::RE_SMARTY_16_SUBSTITUTETEXTPATTERN, vec![t(&ldquo)], 16, 4096),
        ];
        for (rx, rep, prio, mask) in pats {
            out.push(sp(rx, rep, prio, mask));
        }
    }
    if ctx.ext_bool("smarty", "smart_ellipses", true) {
        out.push(sp(
            P::RE_SMARTY_17_SUBSTITUTETEXTPATTERN,
            vec![t(&s("ellipsis"))],
            10,
            1 << 10,
        ));
    }
    out.sort_by_key(|a| std::cmp::Reverse(a.0));
    out
}
