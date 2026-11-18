#!/usr/bin/env python3
"""Build the graphs and the PDF of research/rootfind_levers_20261118 from the
raw rows in this directory (runs.jsonl, floor-thishost.jsonl,
baseline-thishost.jsonl).  Writes stack.svg, stack-256.svg, lanes.svg, their
1x PNG renders and report.pdf.  PNG/PDF need Playwright's Chromium (Python,
or the Node package through render.js).

    python3 research/rootfind_levers_20261118/make_report.py
"""
import html
import json
import statistics as st
from pathlib import Path

D = Path(__file__).resolve().parent


def rows(name):
    return [json.loads(l) for l in (D / name).read_text().splitlines() if l.strip()]


def med(vals):
    return st.median(vals), min(vals), max(vals)


probe = rows("runs.jsonl")
round2 = rows("probe-side/round2-runs.jsonl")
round3 = rows("probe-side/round3-runs.jsonl")
floor = rows("floor-thishost.jsonl")
base = [r for r in rows("baseline-thishost.jsonl") if "arm" in r]


def stack(a):
    p = [r for r in probe if r["abscissae"] == a]
    fl = [r for r in floor if r["abscissae"] == a]
    arm = {r["arm"]: r["ns_per_add_median"] for r in base if r["abscissae"] == a}
    pick = p[0]["lever4_filter_bits"]
    out = [
        ("rootfind-mitm (default oracle)", (arm["rootfind-mitm"],) * 3, "library, 5 rounds"),
        ("rootfind-subtract (best existing arm)", (arm["rootfind-subtract"],) * 3, "library, 5 rounds"),
        ("lever 1: shared denominators, 16-bit filter", med([r["ns_per_point_with_filter"] for r in fl]), "scalar, 1 target"),
        ("+ lever 2: 16 targets lockstep, AVX-512", med([r["pipeline_ns_per_point_filter16"] for r in p]), "16-bit filter"),
        ("+ lever 3: 8-bit filter (1 MiB at A=1024)", med([r["pipeline_ns_per_point_filter8"] for r in p]), ""),
        ("+ fused L1 probe blocks", med([r["fused_ns_per_point_filter8_std_set"] for r in p]), "post hoc arm"),
        ("+ mix-hash exact set", med([r["fused_ns_per_point_filter8_mix_set"] for r in p]), "post hoc arm"),
        ("+ lever 4: 4 threads", med([r["lever4_ns_per_point_by_threads"]["4"] for r in p]), f"fused, {pick}-bit filter"),
    ]
    r3 = [r for r in round3 if r["abscissae"] == a]
    out += [
        ("round 3: one-multiply filter hash, 1 thread", med([r["cheap_hash_fused_ns_per_point"] for r in r3]), "post hoc arm"),
        ("round 3: + 4 threads", med([r["probe_side_best_arm_ns_per_point_by_threads"]["4"] for r in r3]), ""),
    ]
    return out


def bar_svg(title, data, ref, width=760):
    """Horizontal log-scale bars with min-max whiskers; data = [(label, (m, lo, hi), note)]."""
    import math

    left, right, top, rowh = 300, 90, 40, 30
    h = top + rowh * len(data) + 40
    lo_v, hi_v = 0.5, 200.0
    span = math.log10(hi_v) - math.log10(lo_v)

    def x(v):
        return left + (width - left - right) * (math.log10(v) - math.log10(lo_v)) / span

    s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{h}" font-family="Helvetica,Arial,sans-serif" font-size="12">',
         '<rect width="100%" height="100%" fill="#ffffff"/>',
         f'<text x="{width/2}" y="20" text-anchor="middle" font-size="14" font-weight="bold">{html.escape(title)}</text>']
    for t in (0.5, 1, 2, 5, 10, 20, 50, 100, 200):
        s.append(f'<line x1="{x(t):.1f}" y1="{top-6}" x2="{x(t):.1f}" y2="{h-34}" stroke="#e3e3e3"/>')
        s.append(f'<text x="{x(t):.1f}" y="{h-20}" text-anchor="middle" fill="#555">{t:g}</text>')
    s.append(f'<text x="{(left+width-right)/2}" y="{h-5}" text-anchor="middle" fill="#333">ns per signed point (log scale; whiskers: min–max)</text>')
    for i, (lab, (m, lo, hi), note) in enumerate(data):
        y = top + i * rowh
        col = "#9aa5b1" if i < 2 else "#2f6db5"
        s.append(f'<text x="{left-8}" y="{y+15}" text-anchor="end">{html.escape(lab)}</text>')
        s.append(f'<rect x="{x(lo_v):.1f}" y="{y+4}" width="{x(m)-x(lo_v):.1f}" height="16" fill="{col}"/>')
        if hi > lo:
            s.append(f'<line x1="{x(lo):.1f}" y1="{y+12}" x2="{x(hi):.1f}" y2="{y+12}" stroke="#111"/>')
        s.append(f'<text x="{x(max(m, hi))+6:.1f}" y="{y+16}" fill="#111">{m:.2f} ({ref/m:.1f}x)</text>')
    s.append("</svg>")
    return "\n".join(s)


def lanes_svg(a, width=760):
    p = [r for r in probe if r["abscissae"] == a]
    ks = ["1", "2", "4", "8", "16"]
    left, top, bw, gap, h = 70, 40, 26, 40, 320
    ymax = 24.0
    s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{h}" font-family="Helvetica,Arial,sans-serif" font-size="12">',
         '<rect width="100%" height="100%" fill="#ffffff"/>',
         f'<text x="{width/2}" y="20" text-anchor="middle" font-size="14" font-weight="bold">Lever 2: arithmetic only, A = {a}, by targets in lockstep (median of {len(p)} runs)</text>']
    base_y = h - 50

    def y(v):
        return base_y - (base_y - top) * v / ymax

    for t in (0, 2, 4, 6, 8, 12, 16, 20, 24):
        s.append(f'<line x1="{left}" y1="{y(t):.1f}" x2="{width-20}" y2="{y(t):.1f}" stroke="#eee"/>')
        s.append(f'<text x="{left-6}" y="{y(t)+4:.1f}" text-anchor="end" fill="#555">{t}</text>')
    s.append(f'<text x="18" y="{(top+base_y)/2}" transform="rotate(-90 18 {(top+base_y)/2})" text-anchor="middle">ns per point</text>')
    for i, k in enumerate(ks):
        x0 = left + 30 + i * (2 * bw + gap + 30)
        for j, (key, col, lab) in enumerate([("lever2_ns_per_point_by_lanes", "#9aa5b1", "default target"),
                                              ("lever2_avx512_ns_per_point_by_lanes", "#2f6db5", "AVX-512")]):
            m = st.median([r[key][k] for r in p])
            xx = x0 + j * (bw + 4)
            s.append(f'<rect x="{xx}" y="{y(m):.1f}" width="{bw}" height="{base_y-y(m):.1f}" fill="{col}"/>')
            s.append(f'<text x="{xx+bw/2}" y="{y(m)-4:.1f}" text-anchor="middle" font-size="10">{m:.2f}</text>')
        s.append(f'<text x="{x0+bw+2}" y="{base_y+16}" text-anchor="middle">K = {k}</text>')
    sc = st.median([r["lever2_ns_per_point_scalar"] for r in p])
    s.append(f'<line x1="{left}" y1="{y(sc):.1f}" x2="{width-20}" y2="{y(sc):.1f}" stroke="#c0392b" stroke-dasharray="5,4"/>')
    s.append(f'<text x="{width-24}" y="{y(sc)-5:.1f}" text-anchor="end" fill="#c0392b">lever-1 scalar kernel {sc:.2f}</text>')
    s.append(f'<line x1="{left}" y1="{y(4):.1f}" x2="{width-20}" y2="{y(4):.1f}" stroke="#555" stroke-dasharray="2,3"/>')
    s.append(f'<text x="{width-24}" y="{y(4)+14:.1f}" text-anchor="end" fill="#555">falsification line, 4 ns</text>')
    s.append(f'<rect x="{width-250}" y="{h-30}" width="12" height="12" fill="#9aa5b1"/><text x="{width-234}" y="{h-20}">default target</text>')
    s.append(f'<rect x="{width-140}" y="{h-30}" width="12" height="12" fill="#2f6db5"/><text x="{width-124}" y="{h-20}">AVX-512 build</text>')
    s.append("</svg>")
    return "\n".join(s)


def probe_side_svg(a):
    r2 = [r for r in round2 if r["abscissae"] == a]
    data = []
    for k in r2[0]["probe_side_ns_per_point_by_arm"]:
        data.append((k, med([r["probe_side_ns_per_point_by_arm"][k] for r in r2]), ""))
    data.append(("fused + one-multiply hash", med([r["cheap_hash_fused_ns_per_point"] for r in r2]), ""))
    return bar_svg(f"Probe-side arms, one thread, A = {a} (round 2, {len(r2)} runs, interleaved)", data, data[0][1][0])


def main():
    st1024 = stack(1024)
    st256 = stack(256)
    svg_stack = bar_svg("Root-finding kernel, prime 32-bit, A = 1024: the measured stack (same host)", st1024, st1024[0][1][0])
    svg_stack256 = bar_svg("Same stack at A = 256", st256, st256[0][1][0])
    svg_lanes = lanes_svg(1024)
    (D / "stack.svg").write_text(svg_stack)
    (D / "stack-256.svg").write_text(svg_stack256)
    (D / "lanes.svg").write_text(svg_lanes)
    svg_probe = probe_side_svg(1024)
    (D / "probe-side.svg").write_text(svg_probe)
    md = (D / "RESULTS.md").read_text()
    body = md_to_html(md)
    page = f"""<!doctype html><html><head><meta charset="utf-8"><title>Root-finding levers 2-4</title>
<style>body{{font-family:Helvetica,Arial,sans-serif;max-width:820px;margin:24px auto;font-size:12px;line-height:1.45}}
table{{border-collapse:collapse;margin:8px 0}}td,th{{border:1px solid #bbb;padding:3px 6px}}code{{font-size:11px}}h1{{font-size:20px}}h2{{font-size:16px;margin-top:22px}}</style></head>
<body>{body}<h2>Figures</h2>{svg_stack}<br/>{svg_stack256}<br/>{svg_lanes}<br/>{svg_probe}</body></html>"""
    (D / "report.html").write_text(page)
    try:
        from playwright.sync_api import sync_playwright
    except ImportError:
        # Python Playwright is absent on some hosts; the Node one is pre-installed.
        import os
        import subprocess

        npm_root = subprocess.run(["npm", "root", "-g"], capture_output=True, text=True).stdout.strip()
        subprocess.run(["node", str(D / "render.js"), str(D)], check=True, cwd="/",
                       env={**os.environ, "NODE_PATH": npm_root})
        (D / "report.html").unlink()
        print("wrote stack.svg, stack-256.svg, lanes.svg, PNGs, report.pdf (node renderer)")
        return
    with sync_playwright() as pw:
        b = pw.chromium.launch()
        pg = b.new_page(device_scale_factor=1)
        for name in ("stack", "stack-256", "lanes", "probe-side"):
            pg.set_content((D / f"{name}.svg").read_text())
            pg.locator("svg").screenshot(path=str(D / f"{name}.png"))
        pg.goto((D / "report.html").as_uri())
        pg.pdf(path=str(D / "report.pdf"), format="A4", print_background=True,
               margin={"top": "12mm", "bottom": "12mm", "left": "10mm", "right": "10mm"})
        b.close()
    (D / "report.html").unlink()
    print("wrote stack.svg, stack-256.svg, lanes.svg, PNGs, report.pdf")


def md_to_html(md):
    """Small Markdown subset: headings, tables, lists, paragraphs, code spans."""
    import re

    out, para, table, lst = [], [], [], []

    def inline(t):
        t = html.escape(t)
        t = re.sub(r"`([^`]+)`", r"<code>\1</code>", t)
        t = re.sub(r"\*\*([^*]+)\*\*", r"<b>\1</b>", t)
        return t

    def flush():
        if para:
            out.append("<p>" + inline(" ".join(para)) + "</p>")
            para.clear()
        if table:
            rows_ = [r for r in table if not re.match(r"^\|[\s\-:|]+\|$", r)]
            h = "".join(f"<th>{inline(c.strip())}</th>" for c in rows_[0].strip("|").split("|"))
            b = "".join("<tr>" + "".join(f"<td>{inline(c.strip())}</td>" for c in r.strip("|").split("|")) + "</tr>" for r in rows_[1:])
            out.append(f"<table><tr>{h}</tr>{b}</table>")
            table.clear()
        if lst:
            out.append("<ul>" + "".join(f"<li>{inline(i)}</li>" for i in lst) + "</ul>")
            lst.clear()

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
    return "\n".join(out)


if __name__ == "__main__":
    main()
