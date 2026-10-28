"""Dump HTMLExtractor oracle cases: cleandoc + stash per corpus doc.

Run with the probe venv (pristine upstream): `probevenv/bin/python
scripts/dump_html_cases.py`.
"""

import json
import os

import markdown
from markdown.htmlparser import HTMLExtractor
from markdown.preprocessors import NormalizeWhitespace

OUT = "/home/john/firstmate/data/rs-port-markdown/scratch/markdown-rust/markdown-core/tests/data/html_cases.json"
def collect():
    corpus = []
    up = "/home/john/firstmate/data/rs-port-markdown/scratch/upstream/markdown-3.11.2/tests"
    for root, _, files in os.walk(up):
        for f in sorted(files):
            if f.endswith((".txt", ".md", ".text")) and not f.endswith(".html"):
                p = os.path.join(root, f)
                with open(p, encoding="utf-8", errors="replace") as fh:
                    corpus.append(fh.read())
    # Plus targeted edge cases (trailing garbage, cdata, decls, nesting).
    corpus += [
        "<?y",
        "<!z",
        "<!--x",
        "</div",
        "<div",
        "&foo",
        "&#12x",
        "<?y?><?z?>",
        "<![CDATA[q]]>",
        "<!DOCTYPE html>",
        "<div>\n\nfoo\n\n</div>",
        "<div>\n<script>if (a</b) { x(); }</script>\n</div>",
        "<span>mid <b>bold <!--c--> tail</b> end</span>\n\nnext para\n",
        "<div markdown=\"1\">_foo_</div>\n\ntext\n",
        "<hr>\n\ntext\n",
        "text\n<hr/>",
        "<p>a</p><p>b</p>",
        "<pre>\ncode\n</pre>\n\ntail\n",
        "<!-- multi\n\nline -->\n\ntext\n",
        "<![if !IE]>\nfoo\n<![endif]>\n",
        "<a href=\"?x=1&amp;y=2\">t</a>\n",
        "</>",
        "</> text",
        "<div><span>unclosed",
        "a < b and c > d\n",
    ]
    return corpus


def main():
    cases = []
    for text in collect():
        md = markdown.Markdown()
        lines = NormalizeWhitespace(md).run(text.split("\n"))
        p = HTMLExtractor(md)
        p.feed("\n".join(lines))
        p.close()
        stash = [
            ("tree" if not isinstance(b, str) else "str", b if isinstance(b, str) else None)
            for b in md.htmlStash.rawHtmlBlocks
        ]
        cases.append({"in": text, "cleandoc": p.cleandoc, "stash": stash})
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w") as fh:
        json.dump(cases, fh, ensure_ascii=False)
    print("html cases: %d" % len(cases))


if __name__ == "__main__":
    raise SystemExit(main())
