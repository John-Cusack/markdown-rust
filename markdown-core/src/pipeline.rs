//! `Markdown.convert` orchestration (`core.py:322-397`).
//!
//! Whole document crosses once (P1): preprocess → block parse → inline +
//! tree passes → serialize → div-strip → postprocess. Extension-conditioned
//! stages dispatch on `ctx.extensions`; anything unimplemented returns
//! `Err(Unsupported)` and the facade falls back (byte-identical upstream).
//!
//! Currently implemented: the bare core (no extensions). Each extension
//! lands with its stages + a row in `implemented_extensions()`.


use crate::context::Context;
use crate::{ConfigValue, ExtensionSpec, RenderError, RenderRequest, RenderResponse};

/// Built-in extension short names this core build implements.
pub const SUPPORTED: &[&str] = &["tables", "fenced_code", "footnotes", "def_list", "abbr", "attr_list", "meta", "toc", "smarty", "sane_lists", "nl2br", "wikilinks", "codehilite", "admonition", "legacy_attrs", "legacy_em", "md_in_html"];

pub fn implemented_extensions() -> Vec<String> {
    SUPPORTED.iter().map(|s| s.to_string()).collect()
}

fn dedupe_specs(specs: &[ExtensionSpec]) -> Vec<ExtensionSpec> {
    // Upstream `Registry.register` replaces same-name items: last wins.
    let mut out: Vec<ExtensionSpec> = Vec::new();
    for s in specs {
        if let Some(i) = out.iter().position(|e| e.name == s.name) {
            out[i] = s.clone();
        } else {
            out.push(s.clone());
        }
    }
    out
}

pub fn render_full(
    req: &RenderRequest,
    py_entry_depth: usize,
    recursion_limit: usize,
    footnote_unique_prefix: usize,
) -> Result<RenderResponse, RenderError> {
    let specs = dedupe_specs(&req.extensions);
    for s in &specs {
        if !SUPPORTED.contains(&s.name.as_str()) {
            return Err(RenderError::Unsupported(format!("extension: {}", s.name)));
        }
    }
    // Non-scalar configs never reach here (the facade filters), but refuse
    // defensively: only scalars cross the boundary.
    for s in &specs {
        for (_, v) in &s.configs {
            match v {
                ConfigValue::Bool(_)
                | ConfigValue::Int(_)
                | ConfigValue::Str(_)
                | ConfigValue::StrList(_)
                | ConfigValue::StrMap(_)
                | ConfigValue::None => {}
            }
        }
    }

    let mut ctx = Context::new(
        req.tab_length,
        req.output_format,
        specs,
        py_entry_depth,
        recursion_limit,
        req.block_level_elements.clone(),
    );
    ctx.unique_prefix = footnote_unique_prefix;

    // `if not source.strip(): return ''`.
    if req.text.trim().is_empty() {
        return Ok(empty_response());
    }

    // Preprocessors in priority order: normalize_whitespace(30).
    let lines: Vec<String> = req.text.split('\n').map(str::to_string).collect();
    let mut lines = crate::pre::normalize_whitespace(&lines, ctx.tab_length);
    if ctx.has_extension("meta") {
        lines = crate::ext::meta::run(&mut ctx, lines);
    }
    let lines = if ctx.has_extension("fenced_code") {
        crate::ext::fenced_code::run(&mut ctx, lines)?
    } else {
        lines
    };
    // `extract_html` branches internally on `md_in_html` (Extra mode).
    let lines = crate::pre::html_block(&mut ctx, lines);

    // Seed per-render extension state that block processors read.
    if ctx.has_extension("abbr") {
        crate::ext::abbr::seed(&mut ctx);
    }
    // Parse the high-level elements.
    let mut root = crate::block::parse_document(&mut ctx, &lines);
    if ctx.declined {
        return Err(RenderError::Unsupported("recursion-budget".into()));
    }
    // Treeprocessors (priority order): footnote-div(50), inline(20),
    // footnote-reorder(19), footnote-duplicate(15), prettify(10),
    // unescape(0). (hilite/attr_list/abbr/smarty/toc slot in as they land.)
    if ctx.has_extension("footnotes") {
        crate::ext::footnotes::validate_templates(&ctx)?;
        crate::ext::footnotes::build_div(&mut ctx, &mut root)?;
        if ctx.declined {
            return Err(RenderError::Unsupported("recursion-budget".into()));
        }
    }
    // `hilite` treeprocessor (30): plain (non-Pygments) highlight of
    // `pre > code`. The bridge declines exactly when upstream would
    // engage Pygments, so the plain branch below is always exact here.
    if ctx.has_extension("codehilite") {
        let (cssclass, linenums, lang_prefix) = crate::ext::codehilite::plain_config(&ctx);
        crate::ext::codehilite::run_tree(&mut ctx, &mut root, &cssclass, linenums, &lang_prefix)?;
    }
    crate::inline::Inline::run(&mut ctx, &mut root);
    if ctx.has_extension("footnotes") && !ctx.ext_bool("footnotes", "USE_DEFINITION_ORDER", true) {
        crate::ext::footnotes::reorder(&mut ctx, &mut root);
    }
    // Priority-15 pair in registration order: `footnote-duplicate`
    // vs `legacyattrs` (Registry is stable for ties).
    {
        let pos = |n: &str| ctx.extensions.iter().position(|e| e.name == n);
        let dup_first = match (pos("footnotes"), pos("legacy_attrs")) {
            (Some(f), Some(l)) => f < l,
            (Some(_), None) => true,
            _ => false,
        };
        if dup_first && ctx.has_extension("footnotes") {
            crate::ext::footnotes::duplicates(&mut ctx, &mut root);
        }
        if ctx.has_extension("legacy_attrs") {
            crate::ext::legacy_attrs::run_tree(&ctx, &mut root);
        }
        if !dup_first && ctx.has_extension("footnotes") {
            crate::ext::footnotes::duplicates(&mut ctx, &mut root);
        }
    }
    // Non-scalar extension configs that upstream would reject at render.
    if ctx.has_extension("wikilinks") {
        match ctx.ext_config("wikilinks", "build_url") {
            Some(crate::ConfigValue::Str(s)) if s == "default" => {}
            _ => return Err(RenderError::Unsupported("wikilinks build_url".into())),
        }
    }
    crate::tree::prettify(&ctx, &mut root);
    if ctx.has_extension("attr_list") {
        crate::ext::attr_list_tree::run_tree(&ctx, &mut root);
    }
    if ctx.has_extension("abbr") {
        crate::ext::abbr::run_tree(&ctx, &mut root);
    }
    if ctx.has_extension("smarty") {
        crate::inline::Inline::run_smarty(&mut ctx, &mut root);
        if ctx.declined {
            return Err(RenderError::Unsupported("recursion-budget".into()));
        }
    }
    let mut toc_html = String::new();
    let mut toc_tokens: Vec<crate::TocToken> = Vec::new();
    if ctx.has_extension("toc") {
        crate::ext::toc::validate_templates(&ctx)?;
        let (h, t) = crate::ext::toc::run_tree(&mut ctx, &mut root)?;
        toc_html = h;
        toc_tokens = t;
        if ctx.declined {
            return Err(RenderError::Unsupported("recursion-budget".into()));
        }
    }
    crate::tree::unescape(&ctx, &mut root);
    let output = crate::ser::serialize(&root, ctx.output_format);
    let start = match output.find("<div>") {
        Some(i) => i + "<div>".len(),
        None => return Err(RenderError::Unsupported("div-strip".into())),
    };
    let end = match output.rfind("</div>") {
        Some(i) => i,
        None => return Err(RenderError::Unsupported("div-strip".into())),
    };
    if end < start {
        return Err(RenderError::Unsupported("div-strip".into()));
    }
    let mut output = output[start..end].trim().to_string();

    // Postprocessors: raw_html(30), footnote(25), amp_substitute(20).
    output = crate::post::raw_html(&ctx, &output);
    if ctx.has_extension("footnotes") {
        output = crate::ext::footnotes::post(&ctx, &output);
    }
    output = crate::post::amp_substitute(&output);

    Ok(RenderResponse {
        html: output.trim().to_string(),
        toc: toc_html,
        toc_tokens,
        meta: ctx.meta.clone(),
        abbrs: ctx.abbrs.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
    })
}

fn empty_response() -> RenderResponse {
    RenderResponse {
        html: String::new(),
        toc: String::new(),
        toc_tokens: Vec::new(),
        meta: Vec::new(),
        abbrs: Vec::new(),
    }
}
