#!/usr/bin/env python3
"""Summary, figure and PDF of research/ksum_fast_ic_20261118 from the
committed CSV extracts in raw/ (library sessions libA, libB; the fast
solver's configurations). Writes summary.json, speedup.svg, speedup.png
and report.pdf (the PNG/PDF need the Node Playwright package; see
render.js).

    python3 research/ksum_fast_ic_20261118/analyze.py
"""
import csv
import glob
import html
import json
import math
import os
import random
import statistics as st
import subprocess
from pathlib import Path

D = Path(__file__).resolve().parent


def q(v, p):
    v = sorted(v)
    i = (len(v) - 1) * p
    lo = int(i)
    hi = min(lo + 1, len(v) - 1)
    return v[lo] + (v[hi] - v[lo]) * (i - lo)


def boot(num, den, n=4000, seed=20261118):
    """95 % percentile bootstrap of median(num) / median(den)."""
    rnd = random.Random(seed)
    out = []
    for _ in range(n):
        a = [rnd.choice(num) for _ in num]
        b = [rnd.choice(den) for _ in den]
        out.append(st.median(a) / st.median(b))
    return q(out, 0.025), q(out, 0.975)


def rows(path):
    with open(path) as f:
        return list(csv.DictReader(f))


LABELS1 = {
    "persolve-A19-t1": "per solve, A = 19, 1 thread",
    "persolve-A17-t1": "per solve, A = 17, 1 thread",
    "persolve-A19-t4": "per solve, A = 19, 4 threads",
    "persolve-A17-t4": "per solve, A = 17, 4 threads",
    "amortized-A19-t4": "table once per curve, A = 19, 4 threads",
    "amortized-A21-t4": "table once per curve, A = 21, 4 threads",
}
LABELS2 = {
    "sib-A16-L480-t1": "sibling probe, A = 16, 480 lanes, 1 thread",
    "nosib-A17-L256-t4": "no sibling (round 1 kernel), A = 17, 4 threads",
    "sib-A16-L256-t4": "sibling probe, A = 16, 256 lanes, 4 threads",
    "sib-A17-L480-t4": "sibling probe, A = 17, 480 lanes, 4 threads",
    "sib-A16-L480-t4": "sibling probe, A = 16, 480 lanes, 4 threads",
    "sib-amortized-A21-L480-t4": "sibling, table once per curve, A = 21, 4 threads",
}

def analyze_round(R, labels, title):
    D = R
    lib = {}
    for sess in ("libA", "libB"):
        for r in rows(D / "raw" / f"library-{sess}.csv"):
            if r["warmup"] == "1":
                continue
            lib.setdefault((sess, r["arm"]), []).append(r)

    summary = {"library": {}, "fast": {}}
    for (sess, arm), rs in sorted(lib.items()):
        sv = [float(r["solve_ms"]) for r in rs]
        summary["library"][f"{sess}:{arm}"] = {
            "n": len(rs),
            "verified": sum(r["verified"] == "1" and r["matches_target"] == "1" for r in rs),
            "solve_ms_median": st.median(sv),
            "solve_ms_iqr": [q(sv, 0.25), q(sv, 0.75)],
            "oracle_setup_ms_median": st.median(float(r["oracle_setup_ms"]) for r in rs),
            "relations_ms_median": st.median(float(r["relations_ms"]) for r in rs),
            "S_median": st.median(float(r["S"]) for r in rs),
        }
    lib_k = [float(r["solve_ms"]) for s in ("libA", "libB") for r in lib[(s, "ksum5")]]
    summary["library_ksum5_pooled_median_ms"] = st.median(lib_k)

    order = []
    for path in sorted(glob.glob(str(D / "raw" / "fast-*.csv"))):
        name = os.path.basename(path)[5:-4]
        rs = rows(path)
        sv = [float(r["solve_ms"]) for r in rs]
        ok = sum(r["solved"] == "True" and r["log_matches_known"] == "True" and r["x_g_equals_q"] == "True" for r in rs)
        lo, hi = boot(lib_k, sv)
        summary["fast"][name] = {
            "n": len(rs),
            "solved_and_checked": ok,
            "hits_rejected": sum(int(r["hits_rejected"]) for r in rs),
            "solve_ms_median": st.median(sv),
            "solve_ms_iqr": [q(sv, 0.25), q(sv, 0.75)],
            "build_ms_median": st.median(float(r["build_ms"]) for r in rs),
            "walk_ms_median": st.median(float(r["walk_ms"]) for r in rs),
            "relations_median": st.median(int(r["relations"]) for r in rs),
            "walk_steps_median": st.median(int(r["walk_steps"]) for r in rs),
            "speedup_vs_library_ksum5": st.median(lib_k) / st.median(sv),
            "speedup_95ci": [lo, hi],
        }
        order.append(name)
    (D / "summary.json").write_text(json.dumps(summary, indent=1, sort_keys=True) + "\n")

    # Figure: speed-up of each configuration over the library pipeline.
    want = [k for k in labels if k in summary["fast"]]
    W, left, right, top, rowh = 820, 330, 110, 46, 34
    H = top + rowh * len(want) + 50
    lo_v, hi_v = 1.0, 300.0
    span = math.log10(hi_v) - math.log10(lo_v)


    def x(v):
        return left + (W - left - right) * (math.log10(v) - math.log10(lo_v)) / span


    svg = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica,Arial,sans-serif" font-size="12">',
        '<rect width="100%" height="100%" fill="#ffffff"/>',
        f'<text x="{W/2}" y="20" text-anchor="middle" font-size="14" font-weight="bold">{html.escape(title)}</text>',
        f'<text x="{W/2}" y="36" text-anchor="middle" fill="#555">same curve, 48 targets, 2 rounds; library median {st.median(lib_k):.1f} ms a solve (one thread); bars: median ratio, whiskers: 95 % bootstrap</text>',
    ]
    for t in (1, 2, 5, 10, 20, 50, 100, 200):
        svg.append(f'<line x1="{x(t):.1f}" y1="{top-4}" x2="{x(t):.1f}" y2="{H-38}" stroke="#e5e5e5"/>')
        svg.append(f'<text x="{x(t):.1f}" y="{H-24}" text-anchor="middle" fill="#555">{t}x</text>')
    svg.append(f'<line x1="{x(100):.1f}" y1="{top-4}" x2="{x(100):.1f}" y2="{H-38}" stroke="#c0392b" stroke-dasharray="5,4"/>')
    svg.append(f'<text x="{x(100)+4:.1f}" y="{top+6}" fill="#c0392b">100x</text>')
    for i, k in enumerate(want):
        d = summary["fast"][k]
        y = top + i * rowh
        col = "#9aa5b1" if "amortized" in k else "#2f6db5"
        m = d["speedup_vs_library_ksum5"]
        a, b = d["speedup_95ci"]
        svg.append(f'<text x="{left-8}" y="{y+17}" text-anchor="end">{html.escape(labels[k])}</text>')
        svg.append(f'<rect x="{x(lo_v):.1f}" y="{y+5}" width="{x(m)-x(lo_v):.1f}" height="18" fill="{col}"/>')
        svg.append(f'<line x1="{x(a):.1f}" y1="{y+14}" x2="{x(b):.1f}" y2="{y+14}" stroke="#111"/>')
        svg.append(f'<text x="{x(b)+6:.1f}" y="{y+18}">{m:.0f}x ({d["solve_ms_median"]:.2f} ms)</text>')
    svg.append(f'<text x="{(left+W-right)/2}" y="{H-6}" text-anchor="middle" fill="#333">speed-up (log scale); grey: table built once per curve (multi-target accounting, not the per-solve parity number)</text>')
    svg.append("</svg>")
    (D / "speedup.svg").write_text("\n".join(svg))

    return summary


summary = analyze_round(D, LABELS1, "Round 1: k = 5 sum-table IC, 32-bit prime, speed-up over the library pipeline")
summary2 = analyze_round(D / "round2", LABELS2, "Round 2 (sibling probe): speed-up over the library pipeline")

# Report: RESULTS.md as HTML with the figure, then PDF + PNG via Node Playwright.
md = (D / "RESULTS.md").read_text()


def inline(t):
    import re

    t = html.escape(t)
    t = re.sub(r"`([^`]+)`", r"<code>\1</code>", t)
    return re.sub(r"\*\*([^*]+)\*\*", r"<b>\1</b>", t)


out, para, table, lst = [], [], [], []


def flush():
    import re

    if para:
        out.append("<p>" + inline(" ".join(para)) + "</p>")
        para.clear()
    if table:
        body = [r for r in table if not re.match(r"^\|[\s\-:|]+\|$", r)]
        hrow = "".join(f"<th>{inline(c.strip())}</th>" for c in body[0].strip("|").split("|"))
        trs = "".join("<tr>" + "".join(f"<td>{inline(c.strip())}</td>" for c in r.strip("|").split("|")) + "</tr>" for r in body[1:])
        out.append(f"<table><tr>{hrow}</tr>{trs}</table>")
        table.clear()
    if lst:
        out.append("<ul>" + "".join(f"<li>{inline(i)}</li>" for i in lst) + "</ul>")
        lst.clear()


import re

for line in md.splitlines():
    if line.startswith("#"):
        flush()
        n = len(line) - len(line.lstrip("#"))
        out.append(f"<h{n}>{inline(line[n:].strip())}</h{n}>")
    elif line.startswith("|"):
        if para or lst:
            flush()
        table.append(line)
    elif re.match(r"^\s*[-*] ", line):
        if para or table:
            flush()
        lst.append(re.sub(r"^\s*[-*] ", "", line))
    elif re.match(r"^\s{2,}\S", line) and lst:
        lst[-1] += " " + line.strip()
    elif not line.strip():
        flush()
    else:
        if table or lst:
            flush()
        para.append(line.strip())
flush()
page = f"""<!doctype html><html><head><meta charset="utf-8"><title>k-sum IC fast</title>
<style>body{{font-family:Helvetica,Arial,sans-serif;max-width:840px;margin:24px auto;font-size:12px;line-height:1.45}}
table{{border-collapse:collapse;margin:8px 0}}td,th{{border:1px solid #bbb;padding:3px 6px}}code{{font-size:11px}}h1{{font-size:20px}}h2{{font-size:16px;margin-top:22px}}</style></head>
<body>{''.join(out)}<h2>Figures</h2>{(D / 'speedup.svg').read_text()}<br/>{(D / 'round2' / 'speedup.svg').read_text()}</body></html>"""
(D / "report.html").write_text(page)
npm_root = subprocess.run(["npm", "root", "-g"], capture_output=True, text=True).stdout.strip()
subprocess.run(["node", str(D / "render.js"), str(D)], check=True, cwd="/", env={**os.environ, "NODE_PATH": npm_root})
(D / "report.html").unlink()
print("wrote summary.json, speedup.svg, speedup.png, report.pdf")
