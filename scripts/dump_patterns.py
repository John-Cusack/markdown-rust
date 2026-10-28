"""Dump every upstream compiled regex (pattern source + flags) to JSON and
emit machine-exact Rust `&str` consts. Run with the probe venv:

  probevenv/bin/python scripts/dump_patterns.py

Outputs (checked in):
  markdown-core/src/patterns_gen.rs   -- `pub const NAME: &str`
  markdown-core/tests/data/patterns.json -- {name: [source, flags]}
  markdown-core/tests/data/pattern_cases.json -- differential match cases

Covers core now; extension patterns get appended by the same script when
extension ports land (extension modules imported below).
"""

import io
import json
import os
import re

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT_RS = os.path.join(REPO_ROOT, "markdown-core", "src", "patterns_gen.rs")
OUT_JSON = os.path.join(REPO_ROOT, "markdown-core", "tests", "data", "patterns.json")
OUT_CASES = os.path.join(REPO_ROOT, "markdown-core", "tests", "data", "pattern_cases.json")


def flags_to_str(flags):
    out = ""
    if flags & re.IGNORECASE:
        out += "i"
    if flags & re.MULTILINE:
        out += "m"
    if flags & re.DOTALL:
        out += "s"
    if flags & re.VERBOSE:
        out += "x"
    return out


def collect():
    import markdown
    from markdown import (
        blockparser,
        blockprocessors,
        htmlparser,
        inlinepatterns,
        postprocessors,
        preprocessors,
        serializers,
        treeprocessors,
        util,
    )
    found: dict = {}
    _ESC_FIX = re.compile(r"(?<!\\)\\([<>])")

    def add(name, rx):
        if isinstance(rx, re.Pattern):
            # Python `re` treats `\<` / `\>` as literal angle brackets;
            # fancy-regex reads them as GNU word boundaries. Normalize to
            # the literal (machine-checked by `patterns_diff.rs`).
            src = _ESC_FIX.sub(r"\1", rx.pattern)
            if name in found:
                assert found[name][0] == src, "conflict: %s" % name
            found[name] = [src, flags_to_str(rx.flags)]
    def walk_module(mod, prefix):
        for key, val in sorted(vars(mod).items()):
            if isinstance(val, re.Pattern):
                add(prefix + key, val)

    for mod, prefix in [
        (preprocessors, "pre_"),
        (blockparser, "bp_"),
        (blockprocessors, "blk_"),
        (inlinepatterns, "il_"),
        (treeprocessors, "tp_"),
        (postprocessors, "pp_"),
        (serializers, "ser_"),
        (util, "util_"),
        (htmlparser, "html_"),
    ]:
        walk_module(mod, prefix)
        # Class-level compiled patterns.
        for key in dir(mod):
            try:
                obj = getattr(mod, key)
            except Exception:
                continue
            if isinstance(obj, type):
                for attr, val in sorted(vars(obj).items()):
                    if isinstance(val, re.Pattern):
                        add("%s%s_%s" % (prefix, key, attr), val)
    # Per-instance patterns (default tab_length=4, default configs).
    md = markdown.Markdown()
    for i, pat in enumerate(md.preprocessors):
        rx = getattr(pat, "compiled_re", None)
        if rx is None:
            continue
        add("inst_pre_%d_%s" % (i, type(pat).__name__), rx)
    for i, pat in enumerate(md.parser.blockprocessors):
        for attr in ("RE", "SEARCH_RE", "CHILD_RE", "INDENT_RE"):
            rx = getattr(pat, attr, None)
            if isinstance(rx, re.Pattern):
                add("inst_blk_%s_%s" % (type(pat).__name__, attr), rx)
    for i, pat in enumerate(md.inlinePatterns):
        rx = getattr(pat, "compiled_re", None)
        if isinstance(rx, re.Pattern):
            add("inst_il_%02d_%s" % (i, type(pat).__name__), rx)
    # Smarty's private pattern registry (not reachable via md registries).
    try:
        from markdown.extensions.smarty import SmartyExtension
        _smd = markdown.Markdown()
        _sext = SmartyExtension()
        _sext.extendMarkdown(_smd)
        for _j, _pat in enumerate(_sext.inlinePatterns):
            _rx = getattr(_pat, "compiled_re", None)
            if isinstance(_rx, re.Pattern):
                add("smarty_%02d_%s" % (_j, type(_pat).__name__), _rx)
    except Exception as e:
        print("smarty dump skipped: %r" % (e,))
    # Inline shebang regex from CodeHilite._parseHeader (verbose source).
    import textwrap as _tw
    _sh = re.compile(
        _tw.dedent('''
            (?:(?:^::+)|(?P<shebang>^[#]!))
            (?P<path>(?:/\\w+)*[/ ])?
            (?P<lang>[\\w#.+-]*)
            \\s*
            (hl_lines=(?P<quot>"|')(?P<hl_lines>.*?)(?P=quot))?
        '''),
        re.VERBOSE,
    )
    add("codehilite_shebang", _sh)
    # DelimiterProcessor internals for every (token, smart) combo, so the
    # Rust boundary builder is machine-checked on all four, not just the
    # two instantiated by default.
    from markdown.inlinepatterns import Delimiter as PyDelimiter
    for tok, smart in [("*", False), ("*", True), ("_", False), ("_", True)]:
        dd = PyDelimiter(tok, "strong,em", smart, False)
        stag = tok.replace("*", "star").replace("_", "us")
        mode = "smart" if smart else "dumb"
        add("delim_%s_%s_boundary" % (stag, mode), dd.boundary)
        add("delim_%s_%s_bad" % (stag, mode), dd.bad)
    # DelimiterProcessor internals (all delimiter kinds incl. smart `_`).
    delim = md.delimiters
    for tok, d in delim.delimiters.items():
        add("inst_delim_boundary_%s" % tok.replace("*", "star").replace("_", "us"), d.boundary)
        add("inst_delim_bad_%s" % tok.replace("*", "star").replace("_", "us"), d.bad)
    for i, pat in enumerate(md.treeprocessors):
        rx = getattr(pat, "RE", None)
        if isinstance(rx, re.Pattern):
            add("inst_tp_%s" % type(pat).__name__, rx)
    for i, pat in enumerate(md.postprocessors):
        for attr in ("RE", "BLOCK_LEVEL_REGEX"):
            rx = getattr(pat, attr, None)
            if isinstance(rx, re.Pattern):
                add("inst_pp_%s_%s" % (type(pat).__name__, attr), rx)
    # HtmlStash / placeholder res.
    add("inst_util_HTML_PLACEHOLDER_RE", util.HTML_PLACEHOLDER_RE)
    add("inst_util_INLINE_PLACEHOLDER_RE", util.INLINE_PLACEHOLDER_RE)

    # Extension instances (default configs) and their class patterns.
    import importlib

    for ext in [
        "abbr", "admonition", "attr_list", "codehilite", "def_list", "fenced_code",
        "footnotes", "legacy_attrs", "legacy_em", "md_in_html", "meta", "nl2br",
        "sane_lists", "smarty", "tables", "toc", "wikilinks",
    ]:
        mod = importlib.import_module("markdown.extensions." + ext)
        walk_module(mod, "ext_" + ext + "_")
        for key in dir(mod):
            try:
                obj = getattr(mod, key)
            except Exception:
                continue
            if isinstance(obj, type):
                for attr, val in sorted(vars(obj).items()):
                    if isinstance(val, re.Pattern):
                        add("ext_%s_%s_%s" % (ext, key, attr), val)
        mdx = markdown.Markdown(extensions=[ext])
        for reg in (mdx.preprocessors, mdx.inlinePatterns, mdx.treeprocessors,
                    mdx.postprocessors, mdx.parser.blockprocessors):
            for j, pat in enumerate(reg):
                for attr in ("RE", "SEARCH_RE", "CHILD_RE", "INDENT_RE",
                             "compiled_re", "boundary", "bad", "PARAM_RE",
                             "BR_RE", "SMART_RE", "HTML_RE", "ENTITY_RE"):
                    rx = getattr(pat, attr, None)
                    if isinstance(rx, re.Pattern):
                        add("extinst_%s_%02d_%s_%s" % (ext, j, type(pat).__name__, attr), rx)
    return found

def collect_html_std(found):
    """Patterns living on the vendored stdlib `html.parser` copy
    (`markdown.htmlparser.htmlparser`), incl. the monkeypatched ones."""
    import html.parser as stdlib
    from markdown.htmlparser import htmlparser as stdcopy

    def add(name, rx):
        if isinstance(rx, re.Pattern):
            found[name] = [rx.pattern, flags_to_str(rx.flags)]

    add("html_std_locatestarttagend_tolerant", stdcopy.locatestarttagend_tolerant)
    add("html_std_locatetagend", stdcopy.locatetagend)
    add("html_std_tagfind_tolerant", stdlib.tagfind_tolerant)
    add("html_std_attrfind_tolerant", stdlib.attrfind_tolerant)
    return found


def rust_escape(s):
    # Raw STX/ETX would be invisible control bytes in the .rs file; use escapes.
    # (Backslashes first: the \u{2} replacement itself contains one.)
    return (s.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")
            .replace("\x02", "\\u{2}").replace("\x03", "\\u{3}"))


def main():
    found = collect_html_std(collect())
    os.makedirs(os.path.dirname(OUT_JSON), exist_ok=True)
    with open(OUT_JSON, "w") as fh:
        json.dump(found, fh, indent=1, sort_keys=True)

    # Emit Rust consts (machine-exact).
    def const_name(name):
        return "RE_" + re.sub(r"[^A-Za-z0-9]+", "_", name).upper()

    names = {}
    lines = [
        "//! Machine-generated upstream regex sources (`scripts/dump_patterns.py`).",
        "//! DO NOT EDIT: fidelity of every pattern is covered by",
        "//! `markdown-core/tests/patterns_diff.rs` against `tests/data/`.",
        "",
    ]
    for name in sorted(found):
        src, flags = found[name]
        c = const_name(name)
        assert c not in names, "const collision: %s" % c
        names[c] = name
        inline = "".join(sorted(flags))
        full = ("(?%s)" % inline + src) if inline else src
        lines.append("/// upstream `%s` (flags `%s`)" % (name, flags or "-"))
        lines.append('pub const %s: &str = "%s";' % (c, rust_escape(full)))
    with open(OUT_RS, "w") as fh:
        fh.write("\n".join(lines) + "\n")

    # Differential cases: first `search` hit per corpus doc per pattern.
    import markdown

    corpus = []
    up = "/home/john/firstmate/data/rs-port-markdown/scratch/upstream/markdown-3.11.2/tests"
    for root, _, files in os.walk(up):
        for f in sorted(files):
            if f.endswith((".txt", ".md", ".text")) and not f.endswith(".html"):
                p = os.path.join(root, f)
                with open(p, encoding="utf-8", errors="replace") as fh:
                    corpus.append(fh.read())
    cases = {}
    for name, (src, flags) in sorted(found.items()):
        fl = 0
        if "i" in flags:
            fl |= re.IGNORECASE
        if "m" in flags:
            fl |= re.MULTILINE
        if "s" in flags:
            fl |= re.DOTALL
        if "x" in flags:
            fl |= re.VERBOSE
        try:
            rx = re.compile(src, fl)
        except re.error as e:
            cases[name] = {"compile_error": str(e)}
            continue
        hits = []
        for i, text in enumerate(corpus):
            try:
                m = rx.search(text)
            except Exception as e:  # noqa: BLE001
                hits.append([i, {"error": str(e)}])
                continue
            if m is None:
                continue
            hits.append([i, [m.start(), m.end(), list(m.groups())]])
            if len(hits) >= 12:
                break
        cases[name] = hits
    with open(OUT_CASES, "w") as fh:
        json.dump({"corpus_size": len(corpus), "texts": corpus, "cases": cases}, fh, sort_keys=True)
    print("patterns: %d -> %s" % (len(found), OUT_RS))


if __name__ == "__main__":
    raise SystemExit(main())
