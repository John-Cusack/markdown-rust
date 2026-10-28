"""Seam tests: fallback paths stay exact when the Rust fast path declines.

The oracle is the upstream 3.11.2 install in the probe venv (subprocess,
so there is no import shadowing); the port package under test is imported
from the repo checkout / installed wheel.
"""

import os
import subprocess
import sys

import pytest

import markdown
from markdown import _bridge
from markdown.core import Markdown

ORACLE_PY = "/home/john/firstmate/data/rs-port-markdown/scratch/probevenv/bin/python"
ORACLE_SNIPPET = (
    "import sys, markdown; "
    "md = markdown.Markdown(extensions={exts!r}, extension_configs={cfgs!r}, "
    "output_format={fmt!r}, tab_length={tab!r}); "
    "sys.stdout.write(md.convert(sys.stdin.read()))"
)


def oracle_convert(text, exts=None, cfgs=None, fmt="xhtml", tab=4):
    src = ORACLE_SNIPPET.format(exts=exts or [], cfgs=cfgs or {}, fmt=fmt, tab=tab)
    return subprocess.run(
        [ORACLE_PY, "-c", src], input=text, capture_output=True, text=True,
    ).stdout


def setup_function(_):
    os.environ.pop("MARKDOWN_PURE", None)


def teardown_function(_):
    os.environ.pop("MARKDOWN_PURE", None)


def test_pure_env_fallback():
    os.environ["MARKDOWN_PURE"] = "1"
    md = Markdown(extensions=["extra"])
    assert not _bridge.use_rust(md)
    assert md.convert("# hi") == "<h1>hi</h1>"


def test_missing_extension_fallback(monkeypatch):
    monkeypatch.setitem(sys.modules, "markdown._markdown_rs", None)
    monkeypatch.delattr(markdown, "_markdown_rs", raising=False)
    md = Markdown()
    assert not _bridge.use_rust(md)
    assert not _bridge.core_available()
    assert md.convert("a *b* c") == "<p>a <em>b</em> c</p>"


def test_scaffold_declines_everything():
    # Until pipeline modules land, the core supports no extensions: the
    # fast path is attempted for a bare render, declined (`unsupported:`),
    # and the exact pure path runs.
    from markdown import _markdown_rs
    assert _markdown_rs.supported_extensions() == []
    md = Markdown()
    text = "# Title\n\nSome *emph* and **strong** with [a link](http://e.com/x).\n"
    assert _bridge.convert_fast(md, text) is None
    assert md.convert(text) == oracle_convert(text)


def test_unknown_string_extension_fallback():
    # Unknown extension names raise from the normal `registerExtensions`
    # loop, exactly like upstream (record_specs poisons the spec but never
    # masks the original `build_extension` error).
    with pytest.raises(ModuleNotFoundError):
        Markdown(extensions=["not_a_real_extension_xyz"])


def test_subclass_fallback():
    class Sub(Markdown):
        pass

    md = Sub()
    assert not _bridge.use_rust(md)
    assert md.convert("# hi") == "<h1>hi</h1>"


def test_output_format_validation():
    md = Markdown(output_format="html5")
    assert md.output_format == "html"
    with pytest.raises(KeyError):
        Markdown(output_format="bogus")




def test_writeback_attrs_absent_without_extensions():
    md = Markdown()
    assert not hasattr(md, "toc")
    md.convert("# hi")
    assert not hasattr(md, "toc")
    assert not hasattr(md, "Meta")
