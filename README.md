# markdown-rust

Rust port of [Python-Markdown/markdown](https://github.com/Python-Markdown/markdown):
`markdown-core` crate + `markdown` PyO3 binding, BSD-3, upstream-attributed
(see NOTICE).

Status: full port — the Rust core renders the whole document in one
crossing (preprocessors, block parsers, inline patterns incl. the
CommonMark-style delimiter stack, serializer, and all 17 built-in
extensions (Pygments highlighting declines to the exact pure path).
Upstream suite green (809 passed + 210 md_in_html), equivalence corpus
byte-identical; two-track benches in `bench_lane.py`.

Compliance target: **Python-Markdown's own behavior**, not CommonMark.

## Layout

- `markdown-core/` — reusable Rust core crate (no Python dependency):
  whole-document render kernel (`render(&RenderRequest) ->
  RenderResponse`), `OutputFormat`/`ExtensionSpec`/`TocToken` types,
  `model` (Element tree mirroring the consumed `ElementTree` subset).
- `src/` — PyO3 binding (`render`, `supported_extensions`); no `unsafe`.
- `markdown/` — facade: byte-identical upstream 3.11.2 except
  `_bridge.py` (routing: fingerprint + spec log + one-shot core call with
  `toc`/`toc_tokens`/`Meta` writeback) and three marked hooks in
  `core.py` (`# port addition`).
- `tests/` — seam tests (fallback triggers stay exact) + parity tests
  (differential vs the upstream oracle) + the upstream suite verbatim.
- `benches/` — two-track (wall + peak RSS) benches, every figure labeled
  by workload (`data/rs-port-markdown/WORKLOAD.md` in rustsmith).

## Seams (documented)

- `MARKDOWN_PURE=1` or missing `_markdown_rs` extension → pure path.
- Third-party extensions (instances of unknown classes, entry-point or
  `module:Class` names that do not resolve to a built-in), non-scalar
  extension configs (custom `slugify`, `build_url`, pygments formatters)
  → pure path per call.
- `output_format` outside {`html`, `xhtml`} (after upstream normalization)
  → pure path.
- `Markdown` subclasses (custom `build_parser` etc.), or post-`__init__`
  mutation of registries / `tab_length` / `stripTopLevelTags` / `doc_tag` /
  `ESCAPED_CHARS` (fingerprinted) → pure path. `block_level_elements`
  appends flow live per render.
- Non-`str` input → pure path (original errors preserved).
- Core `unsupported:` refusal → pure path per call (routing, never a
  caller-visible error).

## License

BSD-3-Clause — see [LICENSE](LICENSE). Upstream attribution in [NOTICE](NOTICE).
