"""Differential equivalence runner: port package vs upstream oracle.

Compares `markdown.convert` output byte-for-byte across a corpus of
documents x extension configs. The oracle is the pristine upstream 3.11.2
install in the probe venv, driven through ONE long-lived subprocess
(robust: zero import shadowing). The port is the repo `markdown` package
in-process (fast path when the core accepts, pure fallback otherwise).

Usage:
  python scripts/equiv.py [--corpus DIR] [--configs CONFIGS] [--limit N]
                          [--fail-fast] [--write-mismatch DIR]

Exit 0 when all comparisons match; prints a divergence table otherwise.
"""

import argparse
import base64
import json
import os
import subprocess
import sys
import traceback

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ORACLE_PY = "/home/john/firstmate/data/rs-port-markdown/scratch/probevenv/bin/python"

WORKER = r"""
import base64, json, sys
import markdown
out = sys.stdout
inp = sys.stdin
while True:
    line = inp.readline()
    if not line:
        break
    cfg = json.loads(line)
    text = base64.b64decode(inp.readline().strip()).decode("utf-8")
    try:
        # Fresh instance per render, mirroring the port harness (upstream
        # `convert` carries per-document state; no implicit reset).
        md = markdown.Markdown(extensions=list(cfg["exts"]),
                               extension_configs=dict(cfg["cfgs"]),
                               output_format=cfg["fmt"],
                               tab_length=cfg["tab"])
        res = md.convert(text)
        state = ""
        if hasattr(md, "toc"):
            state += "|TOC:%s|TOK:%r" % (md.toc, getattr(md, "toc_tokens", None))
        if hasattr(md, "Meta"):
            state += "|META:%r" % (md.Meta,)
        payload = base64.b64encode((res + state).encode("utf-8")).decode()
        out.write("OK " + payload + "\n")
    except Exception as e:
        out.write("RAISED %r\n" % (e,))
    out.flush()
"""

CONFIGS = {
    "bare": ([], {}, "xhtml", 4),
    "extra": (["extra"], {}, "xhtml", 4),
    "tables": (["tables"], {}, "xhtml", 4),
    "fenced_code": (["fenced_code"], {}, "xhtml", 4),
    "footnotes": (["footnotes"], {}, "xhtml", 4),
    "tables_fenced_footnotes": (["tables", "fenced_code", "footnotes"], {}, "xhtml", 4),
    "def_list": (["def_list"], {}, "xhtml", 4),
    "full": (["extra", "toc", "sane_lists", "smarty", "nl2br", "wikilinks",
              "legacy_attrs", "legacy_em", "meta", "admonition"],
             {}, "xhtml", 4),
    "html": (["extra"], {}, "html", 4),
    "tab8": (["extra"], {}, "xhtml", 8),
}


class Oracle:
    def __init__(self):
        self.proc = subprocess.Popen(
            [ORACLE_PY, "-c", WORKER],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            text=True, bufsize=1,
        )

    def render(self, text, exts, cfgs, fmt, tab):
        cfg = {"exts": exts, "cfgs": cfgs, "fmt": fmt, "tab": tab}
        self.proc.stdin.write(json.dumps(cfg) + "\n")
        self.proc.stdin.write(base64.b64encode(text.encode("utf-8")).decode() + "\n")
        self.proc.stdin.flush()
        line = self.proc.stdout.readline()
        if line.startswith("OK "):
            return base64.b64decode(line[3:].strip()).decode("utf-8")
        return line.strip()

    def close(self):
        try:
            self.proc.stdin.close()
        except BrokenPipeError:
            pass
        self.proc.wait(timeout=30)


def collect_inputs(corpus_dir):
    inputs = []
    for root, _, files in os.walk(corpus_dir):
        for f in sorted(files):
            if f.endswith((".txt", ".md", ".text")) and not f.endswith(".html"):
                p = os.path.join(root, f)
                with open(p, encoding="utf-8", errors="replace") as fh:
                    inputs.append((os.path.relpath(p, corpus_dir), fh.read()))
    return inputs


def render(mod, text, exts, cfgs, fmt, tab):
    md = mod.Markdown(extensions=list(exts), extension_configs=dict(cfgs),
                      output_format=fmt, tab_length=tab)
    try:
        out = md.convert(text)
    except Exception as e:  # noqa: BLE001 - record, don't stop
        return "RAISED %r" % (e,)
    state = ""
    if hasattr(md, "toc"):
        state += "|TOC:%s|TOK:%r" % (md.toc, getattr(md, "toc_tokens", None))
    if hasattr(md, "Meta"):
        state += "|META:%r" % (md.Meta,)
    return out + state


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--corpus", default=os.path.join(REPO_ROOT, "tests", "upstream"))
    ap.add_argument("--configs", default="bare,extra,full,html")
    ap.add_argument("--limit", type=int, default=0)
    ap.add_argument("--fail-fast", action="store_true")
    ap.add_argument("--write-mismatch", default="")
    args = ap.parse_args()

    sys.path.insert(0, REPO_ROOT)  # port package first
    import markdown as port
    oracle = Oracle()

    wanted = [c for c in args.configs.split(",") if c in CONFIGS]
    inputs = collect_inputs(args.corpus)
    if args.limit:
        inputs = inputs[: args.limit]

    total = mism = 0
    try:
        for rel, text in inputs:
            for cname in wanted:
                exts, cfgs, fmt, tab = CONFIGS[cname]
                try:
                    p = render(port, text, exts, cfgs, fmt, tab)
                except Exception:  # noqa: BLE001
                    p = "HARNESS-RAISED port:\n" + traceback.format_exc()
                o = oracle.render(text, exts, cfgs, fmt, tab)
                total += 1
                if p != o:
                    mism += 1
                    print("DIVERGE %s [%s] port=%dB oracle=%dB"
                          % (rel, cname, len(p), len(o)))
                    for i, (a, b) in enumerate(zip(p.splitlines(), o.splitlines())):
                        if a != b:
                            print("  first diff at line %d:\n    port:   %r\n    oracle: %r"
                                  % (i, a[:300], b[:300]))
                            break
                    else:
                        print("  (same lines, different count/trailing: %d vs %d)" % (
                            len(p.splitlines()), len(o.splitlines())))
                    if args.write_mismatch:
                        os.makedirs(args.write_mismatch, exist_ok=True)
                        tag = "%s__%s" % (rel.replace("/", "__"), cname)
                        with open(os.path.join(args.write_mismatch, tag + ".port.html"), "w") as fh:
                            fh.write(p)
                        with open(os.path.join(args.write_mismatch, tag + ".oracle.html"), "w") as fh:
                            fh.write(o)
                    if args.fail_fast:
                        return 1
    finally:
        oracle.close()
    print("equiv: %d/%d match (%d divergences)" % (total - mism, total, mism))
    return 1 if mism else 0


if __name__ == "__main__":
    raise SystemExit(main())
