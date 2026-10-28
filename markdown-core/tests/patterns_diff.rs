//! Differential regex check: every dumped upstream pattern must compile
//! under `fancy-regex` and reproduce Python `re.search` hits (span +
//! groups) on the dumped corpus cases.
//!
//! Data: `tests/data/patterns.json` + `tests/data/pattern_cases.json`
//! (both emitted by `scripts/dump_patterns.py`; machine-exact sources).

use fancy_regex::Regex as F;
use serde_json::Value;

fn cases() -> Value {
    let raw = include_str!("data/pattern_cases.json");
    serde_json::from_str(raw).expect("pattern_cases.json parses")
}

fn patterns() -> Value {
    let raw = include_str!("data/patterns.json");
    serde_json::from_str(raw).expect("patterns.json parses")
}

#[test]
fn patterns_match_python_re() {
    let pats = patterns();
    let data = cases();
    let texts = data["texts"].as_array().unwrap();
    let all = data["cases"].as_object().unwrap();
    let mut checked = 0;
    let mut skipped = 0;
    for (name, entry) in all {
        if entry.is_object() {
            // compile_error recorded by the dumper (upstream-only syntax).
            skipped += 1;
            continue;
        }
        let src = pats[name][0].as_str().unwrap();
        let flags = pats[name][1].as_str().unwrap();
        let mut inline = String::new();
        for f in ["i", "m", "s", "x"] {
            if flags.contains(f) {
                inline.push_str(f);
            }
        }
        let full = if inline.is_empty() {
            src.to_string()
        } else {
            format!("(?{inline}){src}")
        };
        let re = F::new(&full)
            .unwrap_or_else(|e| panic!("pattern {name} failed to compile: {e}"));
        for hit in entry.as_array().unwrap() {
            let idx = hit[0].as_u64().unwrap() as usize;
            let exp = &hit[1];
            let text = texts[idx].as_str().unwrap();
            let got = re
                .captures(text)
                .unwrap_or_else(|e| panic!("pattern {name} search error: {e}"));
            match (got, exp.as_array()) {
                (None, _) => panic!("pattern {name} missed on text {idx}"),
                (Some(caps), Some(arr)) => {
                    let m0 = caps.get(0).unwrap();
                    // fancy-regex reports byte spans; Python `re` reports char spans.
                    let to_chars = |b: usize| text[..b].chars().count();
                    assert_eq!(to_chars(m0.start()), arr[0].as_u64().unwrap() as usize, "{name} t{idx} start");
                    assert_eq!(to_chars(m0.end()), arr[1].as_u64().unwrap() as usize, "{name} t{idx} end");
                    let exp_groups = arr[2].as_array().unwrap();
                    for (gi, eg) in exp_groups.iter().enumerate() {
                        let got_g = caps.get(gi + 1).map(|m| m.as_str().to_string());
                        let exp_g = eg.as_str().map(str::to_string);
                        assert_eq!(got_g, exp_g, "{name} t{idx} group {}", gi + 1);
                    }
                }
                (Some(_), None) => panic!("pattern {name} t{idx}: expected error, got match"),
                (None, None) => panic!("pattern {name} t{idx}: expected error, got miss"),
            }
            checked += 1;
        }
    }
    assert!(checked > 1000, "too few checks: {checked}");
    eprintln!("patterns differential: {checked} hits match, {skipped} skipped (upstream compile errors)");
}
