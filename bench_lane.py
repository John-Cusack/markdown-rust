"""Two-track bench: wall time + peak RSS, pure-Python vs Rust, per WORKLOAD.md."""
import os
import resource
import statistics
import sys
import time

PURE = os.environ.get('MARKDOWN_PURE') == '1'
import markdown
from markdown import Markdown

with open('/home/john/orca/workspaces/rustsmith/fm-rs-port-markdown/README.md') as f:
    DOCS_PAGE = f.read()
SMALL = "# Hello\n\nThis is a *test* with [a link](http://example.com) and `code`.\n\n- item one\n- item two\n"
LARGE = DOCS_PAGE * 8

_md_small = Markdown(extensions=["extra"])
_md_docs = Markdown(extensions=["extra", "toc", "tables", "fenced_code", "codehilite",
                                "footnotes", "admonition", "sane_lists", "smarty"])
_md_large = Markdown(extensions=["extra", "toc"])


def bench(fn, iters):
    fn()  # warmup
    ts = []
    for _ in range(iters):
        t0 = time.perf_counter()
        fn()
        ts.append(time.perf_counter() - t0)
    return statistics.median(ts)


def workload_small():
    _md_small.reset()
    return _md_small.convert(SMALL)


def workload_docs():
    _md_docs.reset()
    return _md_docs.convert(DOCS_PAGE)


def workload_large():
    _md_large.reset()
    return _md_large.convert(LARGE)


if __name__ == '__main__':
    which = sys.argv[1] if len(sys.argv) > 1 else 'all'
    iters = int(sys.argv[2]) if len(sys.argv) > 2 else 5
    cases = {'small': (workload_small, len(SMALL)), 'docs': (workload_docs, len(DOCS_PAGE)),
             'large': (workload_large, len(LARGE))}
    for name, (fn, size) in cases.items():
        if which != 'all' and which != name:
            continue
        out = fn()
        dt = bench(fn, iters)
        rss = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        print(f"{name}: bytes={len(out)} wall={dt * 1000:.1f}ms rss={rss / 1024:.1f}MiB pure={PURE}")
