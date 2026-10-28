//! `markdown._markdown_rs`: compiled half of the port.
//!
//! Thin PyO3 translation over [`markdown_rust_core`]: the whole document
//! crosses once in, finished HTML plus per-document state cross once out.
//! All Python-visible behavior (the `Markdown` class, registries,
//! extension loading, fallback routing) stays in the `markdown/` facade;
//! this file only translates between Python objects and core types.
//! No `unsafe` in this port.

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};

use markdown_rust_core::{
    ConfigValue, ExtensionSpec, OutputFormat, RenderRequest,
};

/// Convert one extension-config value. Only JSON-like scalars cross;
/// anything else (notably callables such as a custom `slugify` or
/// `build_url`) is refused so the facade falls back to pure Python.
fn to_config_value(v: &Bound<'_, PyAny>) -> PyResult<ConfigValue> {
    if v.is_none() {
        return Ok(ConfigValue::None);
    }
    if let Ok(b) = v.extract::<bool>() {
        return Ok(ConfigValue::Bool(b));
    }
    if let Ok(i) = v.extract::<i64>() {
        return Ok(ConfigValue::Int(i));
    }
    if let Ok(s) = v.extract::<String>() {
        return Ok(ConfigValue::Str(s));
    }
    if let Ok(m) = v.extract::<std::collections::HashMap<String, String>>() {
        let mut kvs: Vec<(String, String)> = m.into_iter().collect();
        kvs.sort();
        return Ok(ConfigValue::StrMap(kvs));
    }
    if let Ok(items) = v.extract::<Vec<String>>() {
        return Ok(ConfigValue::StrList(items));
    }
    Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
        "markdown-rust: unsupported extension config type (pure-Python fallback)",
    ))
}
/// One extension spec: short name + resolved scalar configs.
type ExtSpec<'py> = (String, Vec<(String, Bound<'py, PyAny>)>);

/// Render one document through the Rust core.
///
/// `extensions` is a list of `(name, [(key, value)])` with values already
/// restricted to scalars by the facade; non-scalars raise `ValueError`
/// (facade treats any failure here as fallback, never a caller error).
/// `py_entry_depth`/`recursion_limit` feed the recursion model.
/// Returns a dict `{html, toc, toc_tokens, meta}`; raises `ValueError`
/// with an `unsupported:` message when the core declines the request.
#[pyfunction]
#[pyo3(signature = (text, output_format="xhtml", tab_length=4, extensions=None, py_entry_depth=0, recursion_limit=1000, footnote_unique_prefix=0, block_level_elements=None))]
/// PyO3 entry points take flat parameter lists by construction.
#[allow(clippy::too_many_arguments)]
fn render(
    py: Python<'_>,
    text: &str,
    output_format: &str,
    tab_length: usize,
    extensions: Option<Vec<ExtSpec<'_>>>,
    py_entry_depth: usize,
    recursion_limit: usize,
    footnote_unique_prefix: usize,
    block_level_elements: Option<Vec<String>>,
) -> PyResult<PyObject> {
    let mut specs = Vec::new();
    if let Some(exts) = extensions {
        for (name, cfgs) in exts {
            let mut configs = Vec::with_capacity(cfgs.len());
            for (k, v) in cfgs {
                configs.push((k, to_config_value(&v)?));
            }
            specs.push(ExtensionSpec { name, configs });
        }
    }
    let format = OutputFormat::parse(output_format).ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "markdown-rust: unsupported output format {output_format:?}"
        ))
    })?;
    let req = RenderRequest {
        text: text.to_string(),
        output_format: format,
        tab_length,
        extensions: specs,
        block_level_elements,
    };
    let resp = markdown_rust_core::render(&req, py_entry_depth, recursion_limit, footnote_unique_prefix).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("unsupported: {e:?}"))
    })?;
    let out = PyDict::new(py);
    out.set_item("html", resp.html)?;
    out.set_item("toc", resp.toc)?;
    fn token_dict<'py>(
        py: Python<'py>,
        t: &markdown_rust_core::TocToken,
    ) -> PyResult<Bound<'py, PyDict>> {
        let d = PyDict::new(py);
        d.set_item("level", t.level)?;
        d.set_item("id", &t.id)?;
        d.set_item("name", &t.name)?;
        d.set_item("html", &t.html)?;
        d.set_item("data-toc-label", &t.data_toc_label)?;
        let kids = pyo3::types::PyList::empty(py);
        for c in &t.children {
            kids.append(token_dict(py, c)?)?;
        }
        d.set_item("children", kids)?;
        Ok(d)
    }
    let tokens = pyo3::types::PyList::empty(py);
    for t in &resp.toc_tokens {
        tokens.append(token_dict(py, t)?)?;
    }
    out.set_item("toc_tokens", tokens)?;
    let meta = PyDict::new(py);
    for (k, vs) in &resp.meta {
        meta.set_item(k, vs)?;
    }
    out.set_item("meta", meta)?;
    let abbrs = PyDict::new(py);
    for (k, v) in &resp.abbrs {
        abbrs.set_item(k, v)?;
    }
    out.set_item("abbrs", abbrs)?;
    Ok(out.into())
}

/// Built-in extension short names this core build implements.
#[pyfunction]
fn supported_extensions() -> Vec<String> {
    markdown_rust_core::supported_extensions()
}

#[pymodule]
fn _markdown_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render, m)?)?;
    m.add_function(wrap_pyfunction!(supported_extensions, m)?)?;
    m.add("_CORE_UPSTREAM_VERSION", markdown_rust_core::UPSTREAM_VERSION)?;
    Ok(())
}
