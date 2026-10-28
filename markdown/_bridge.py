"""Rust fast-path bridge (port addition, not upstream code).

Routes `Markdown.convert` through `markdown._markdown_rs` with exactly one
string crossing each way (whole document in, finished HTML + per-document
state out). Everything else — registries, extension loading, the
pure-Python fallback — is byte-identical upstream code.

Fallback triggers (all documented in the mirror README seam section):
- the extension is not importable,
- `MARKDOWN_PURE` is set in the environment,
- any extension outside the core's `supported_extensions()` set
  (third-party extensions, `module:Class` paths, entry-points),
- non-scalar extension configs (notably callables: custom `slugify`,
  `build_url`, pygments formatters),
- `output_format` outside {`html`, `xhtml`} (after upstream normalization),
- a `Markdown` subclass instance, or post-`__init__` mutation of the
  registries / scalar knobs (fingerprinted),
- the input is not `str` (the pure path raises the original errors),
- the core declines the request (`unsupported:` routing signal).
"""

import os

_PYGMENTS_MISSING = object()
_pygments_cache = _PYGMENTS_MISSING


def _pygments_present():
    """Whether Pygments is importable (cached `find_spec`)."""
    global _pygments_cache
    if _pygments_cache is _PYGMENTS_MISSING:
        try:
            import importlib.util
            _pygments_cache = importlib.util.find_spec('pygments') is not None
        except Exception:
            _pygments_cache = False
    return _pygments_cache

#: `type(ext).__module__ + '.' + type(ext).__qualname__` -> short name.
CLASS_TO_NAME = {
    'markdown.extensions.abbr.AbbrExtension': 'abbr',
    'markdown.extensions.admonition.AdmonitionExtension': 'admonition',
    'markdown.extensions.attr_list.AttrListExtension': 'attr_list',
    'markdown.extensions.codehilite.CodeHiliteExtension': 'codehilite',
    'markdown.extensions.def_list.DefListExtension': 'def_list',
    'markdown.extensions.extra.ExtraExtension': None,  # transparent: sub-exts recorded by inner call
    'markdown.extensions.fenced_code.FencedCodeExtension': 'fenced_code',
    'markdown.extensions.footnotes.FootnoteExtension': 'footnotes',
    'markdown.extensions.legacy_attrs.LegacyAttrExtension': 'legacy_attrs',
    'markdown.extensions.legacy_em.LegacyEmExtension': 'legacy_em',
    'markdown.extensions.md_in_html.MarkdownInHtmlExtension': 'md_in_html',
    'markdown.extensions.meta.MetaExtension': 'meta',
    'markdown.extensions.nl2br.Nl2BrExtension': 'nl2br',
    'markdown.extensions.sane_lists.SaneListExtension': 'sane_lists',
    'markdown.extensions.smarty.SmartyExtension': 'smarty',
    'markdown.extensions.tables.TableExtension': 'tables',
    'markdown.extensions.toc.TocExtension': 'toc',
    'markdown.extensions.wikilinks.WikiLinkExtension': 'wikilinks',
}


def core_available():
    try:
        import markdown._markdown_rs  # noqa: F401
    except ImportError:
        return False
    return True


def core_supported():
    from markdown import _markdown_rs
    return set(_markdown_rs.supported_extensions())


def _registry_fingerprint(reg):
    items = list(reg)  # forces lazy sort
    out = []
    for p in reg._priority:
        item = reg._data[p.name]
        cls = type(item)
        out.append((p.name, p.priority, cls.__module__ + '.' + cls.__qualname__))
    return tuple(out)


def fingerprint(md):
    """Conservative identity of everything the render reads besides the
    input text: registry composition, scalar knobs, escape/table classes."""
    return (
        ('tab_length', md.tab_length),
        ('stripTopLevelTags', md.stripTopLevelTags),
        ('doc_tag', md.doc_tag),
        ('ESCAPED_CHARS', tuple(md.ESCAPED_CHARS)),
        # `block_level_elements` flows live per render in the request
        # (append-after-construction stays on the fast path).
        ('output_format', md.output_format),
        ('preprocessors', _registry_fingerprint(md.preprocessors)),
        ('blockprocessors', _registry_fingerprint(md.parser.blockprocessors)),
        ('inlinePatterns', _registry_fingerprint(md.inlinePatterns)),
        ('treeprocessors', _registry_fingerprint(md.treeprocessors)),
        ('postprocessors', _registry_fingerprint(md.postprocessors)),
    )


def _normalize_callables(name, cfgs):
    """Map known-default callables to scalar markers; poison the spec
    (`None`) on anything the core cannot take (custom `slugify`,
    `build_url`, formatters). The core implements the defaults; the
    fallback runs everything else byte-identically."""
    out = dict(cfgs)
    if name == 'toc':
        v = out.get('slugify')
        if callable(v):
            mod = getattr(v, '__module__', '') + '.' + getattr(v, '__qualname__', '')
            if mod == 'markdown.extensions.toc.slugify':
                out['slugify'] = 'default'
            elif mod == 'markdown.extensions.toc.slugify_unicode':
                out['slugify'] = 'unicode'
            else:
                return None
    if name == 'wikilinks':
        v = out.get('build_url')
        if callable(v):
            mod = getattr(v, '__module__', '') + '.' + getattr(v, '__qualname__', '')
            if mod == 'markdown.extensions.wikilinks.build_url':
                out['build_url'] = 'default'
            else:
                return None
    if name == 'codehilite':
        # `pygments_formatter` is unread on the plain (non-pygments) path;
        # drop a callable there so scalar validation passes exactly when
        # upstream would ignore it.
        up = out.get('use_pygments', True)
        if up is False and callable(out.get('pygments_formatter')):
            del out['pygments_formatter']
    return out


def record_specs(md, extensions, configs):
    """Resolve one `registerExtensions` call to spec entries.

    Appended to `md._rust_ext_specs` in registration order; a `None` entry
    poisons the fast path (unknown extension). Called from the patched
    `registerExtensions`. String names resolve through the normal
    `build_extension` path first, so instance configs are exactly what
    upstream would use (bool parsing, unknown-key errors identical —
    they propagate from the same call upstream makes). The `extra` bundle
    is expanded statically (same order, same per-sub-extension configs
    as the nested call upstream performs), so explicit-vs-bundle
    conflicts resolve identically.
    """
    from markdown.extensions import Extension
    from markdown.extensions.extra import ExtraExtension
    from markdown.extensions.extra import extensions as _bundle
    if extensions is _bundle:
        # Nested call from `ExtraExtension.extendMarkdown`: the outer call
        # already expanded these statically (`_expand_extra`); recording
        # again would double the specs. (Identity check: only the exact
        # module-level bundle list takes this path.)
        return
    specs = getattr(md, '_rust_ext_specs', None)
    if specs is None:
        specs = md._rust_ext_specs = []
    for ext in extensions:
        if isinstance(ext, str):
            try:
                built = md.build_extension(ext, configs.get(ext, {}))
            except Exception:
                specs.append(None)
            else:
                record_specs(md, [built], configs)
        elif isinstance(ext, ExtraExtension):
            # Transparent wrapper: expand each bundled extension through
            # the normal build path (exact config normalization).
            from markdown.extensions.extra import extensions as bundle
            for sub in bundle:
                try:
                    built = md.build_extension(sub, ext.config.get(sub, {}))
                except Exception:
                    specs.append(None)
                else:
                    record_specs(md, [built], configs)
        elif isinstance(ext, Extension):
            key = type(ext).__module__ + '.' + type(ext).__qualname__
            name = CLASS_TO_NAME.get(key, 'unknown:' + key)
            if name.startswith('unknown:'):
                specs.append(None)
            else:
                try:
                    cfgs = _normalize_callables(name, dict(ext.getConfigs()))
                except Exception:
                    specs.append(None)
                else:
                    specs.append(None if cfgs is None else (name, cfgs))
        elif ext is not None:
            specs.append(None)


def _scalar_ok(value):
    if value is None or isinstance(value, (bool, int, str)):
        return True
    if isinstance(value, (list, tuple)):
        return all(isinstance(v, str) for v in value)
    # Plain string maps (e.g. the `abbr` glossary) cross as `StrMap`.
    if isinstance(value, dict):
        return all(isinstance(k, str) and isinstance(v, str) for k, v in value.items())
    return False


def use_rust(md):
    """True when `md.convert` may take the Rust fast path."""
    from markdown.core import Markdown
    if os.environ.get('MARKDOWN_PURE'):
        return False
    if not core_available():
        return False
    if type(md) is not Markdown:
        return False
    if getattr(md, '_rust_ext_specs', None) is None:
        return False
    if fingerprint(md) != md._rust_snapshot:
        return False
    try:
        supported = core_supported()
    except Exception:
        return False
    for spec in md._rust_ext_specs:
        if spec is None:
            return False
        name, cfgs = spec
        if name not in supported:
            return False
        if not all(isinstance(k, str) and _scalar_ok(v) for k, v in cfgs.items()):
            return False
        if name == 'codehilite':
            # The Pygments highlight path has no Rust equivalent: decline
            # exactly when upstream would engage it (`pygments and
            # use_pygments`). Otherwise the plain branch below is exact.
            up = cfgs.get('use_pygments', True)
            if up is True or (isinstance(up, str) and up not in ('False', 'false', 'no', 'n', 'off', '0', 'none', '')):
                if up and _pygments_present():
                    return False
    if md.output_format not in ('html', 'xhtml'):
        return False
    return True


def convert_fast(md, source):
    """One core call; writes back per-document state; returns HTML.

    Raises `ValueError('unsupported: ...')` when the core declines (the
    caller falls back); any other exception propagates (core bug, fix it).
    """
    from markdown import _markdown_rs
    exts = [
        (name, [(k, v) for k, v in cfgs.items()])
        for name, cfgs in md._rust_ext_specs
    ]
    # The `abbr` glossary is live state (`load_glossary` may run after
    # registration): re-read the instance map each render, like upstream
    # (which shares the same dict between its block and tree processors).
    abbr_ext = None
    for ext in md.registeredExtensions:
        if type(ext).__name__ == 'AbbrExtension' and type(ext).__module__ == 'markdown.extensions.abbr':
            abbr_ext = ext
            break
    if abbr_ext is not None:
        live = {k: v for k, v in abbr_ext.abbrs.items()
                if isinstance(k, str) and isinstance(v, str)}
        exts = [(n, ([(k, v) for k, v in cfgs if k != 'glossary'] + [('glossary', live)]
                    if n == 'abbr' else [(k, v) for k, v in cfgs]))
                for n, cfgs in exts]
    try:
        import sys
        f = sys._getframe()
        depth = 0
        while f is not None:
            depth += 1
            f = f.f_back
        # Per-render state the core cannot see: footnote `unique_prefix`
        # is read off the live extension instance (exact across resets).
        unique_prefix = 0
        for ext in md.registeredExtensions:
            if type(ext).__name__ == 'FootnoteExtension' and type(ext).__module__ == 'markdown.extensions.footnotes':
                unique_prefix = ext.unique_prefix
                break
        resp = _markdown_rs.render(
            source,
            output_format=md.output_format,
            tab_length=md.tab_length,
            extensions=exts,
            py_entry_depth=depth,
            recursion_limit=sys.getrecursionlimit(),
            footnote_unique_prefix=unique_prefix,
            block_level_elements=list(md.block_level_elements),
        )
    except ValueError as e:
        if str(e).startswith('unsupported:'):
            return None
        raise
    names = {name for name, _ in md._rust_ext_specs}
    if 'toc' in names:
        def _tok(t):
            return dict(level=t['level'], id=t['id'], name=t['name'], html=t['html'],
                        **{'data-toc-label': t['data-toc-label']},
                        children=[_tok(c) for c in t['children']])
        md.toc = resp['toc']
        md.toc_tokens = [_tok(t) for t in resp['toc_tokens']]
    if meta_names := names & {'meta', 'wikilinks'}:
        raw = resp['meta'] if isinstance(resp.get('meta'), dict) else dict(resp.get('meta') or [])
        md.Meta = {k: list(v) for k, v in raw.items()}
    if 'abbr' in names and abbr_ext is not None:
        # Upstream mutates the shared `abbrs` dict in place during the
        # render (definitions accumulate until `reset()`); mirror that.
        abbr_ext.abbrs.update(dict(resp.get('abbrs') or {}))
    return resp['html']
