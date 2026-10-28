//! Differential HTML-extraction check: `html::extract_html` must reproduce
//! the oracle `HTMLExtractor.cleandoc` + stash on every dumped case.
//!
//! Data: `tests/data/html_cases.json` (emitted by
//! `scripts/dump_html_cases.py` under the pristine upstream).

use serde_json::Value;

use markdown_rust_core::context::{Context, StashEntry};
use markdown_rust_core::{html, OutputFormat};

#[test]
fn html_extraction_matches_oracle() {
    let raw = include_str!("data/html_cases.json");
    let cases: Vec<Value> = serde_json::from_str(raw).expect("html_cases.json parses");
    assert!(!cases.is_empty());
    let mut checked = 0;
    for (i, case) in cases.iter().enumerate() {
        let text = case["in"].as_str().unwrap();
        // Mirror the oracle input prep: NormalizeWhitespace via our own
        // `pre` (covered separately against the same oracle function).
        let lines: Vec<String> = text.split('\n').map(str::to_string).collect();
        let norm = markdown_rust_core::pre::normalize_whitespace(&lines, 4);
        let mut ctx = Context::new(4, OutputFormat::Xhtml, vec![], 2, 1000, None);
        let cleandoc = html::extract_html(&mut ctx, &norm.join("\n"));
        let exp_clean: Vec<String> = case["cleandoc"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        assert_eq!(cleandoc, exp_clean, "case {i} cleandoc");
        let exp_stash: Vec<String> = case["stash"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| e[1].as_str().unwrap().to_string())
            .collect();
        let got_stash: Vec<String> = ctx
            .stash
            .iter()
            .map(|e| match e {
                StashEntry::Html(h) => h.clone(),
                StashEntry::Tree(_) => "<TREE-UNEXPECTED>".to_string(),
            })
            .collect();
        assert_eq!(got_stash, exp_stash, "case {i} stash");
        checked += 1;
    }
    eprintln!("html extraction: {checked} cases match");
}
