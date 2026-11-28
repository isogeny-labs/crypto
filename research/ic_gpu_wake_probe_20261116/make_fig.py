#!/usr/bin/env python3
"""fig_startup.svg: left, cumulative mean round trip vs calls with the b + F/n fit; right, the steady-state call (arm B)."""
import json, os, statistics as st
D = os.path.dirname(os.path.abspath(__file__)) + "/"
A, B = json.load(open(D + "startup_fit_A.json")), json.load(open(D + "startup_fit.json"))
R = json.load(open(D + "../ic_gpu_ipc_split_20261116/results.json"))
W, H = 900, 400
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="system-ui, -apple-system, Segoe UI, sans-serif" role="img" aria-labelledby="t d">',
     '<title id="t">fes-f2-gpu round trip: a one-time start-up cost, and the steady-state call</title>',
     '<desc id="d">Left: the cumulative mean worker round trip per call against the number of calls falls as a constant plus a one-time cost divided by the calls, for the text-protocol build (A) and current main (B) at 24 and 26 unknowns. Right: the steady-state call on main at 24 and 26 unknowns split into kernel, other worker work, pipe floor, Rust preparation and verification, and a small remainder.</desc>',
     '<style>.bg{fill:#fcfcfb}.a{stroke:#eb6834;fill:#eb6834}.b{stroke:#2a78d6;fill:#2a78d6}.c1{fill:#2a78d6}.c2{fill:#7fb3f0}.c3{fill:#1baf7a}.c4{fill:#eb6834}.c5{fill:#9d9c97}.p{fill:#0b0b0b}.q{fill:#52514e}.g{stroke:#e4e3df}.ln{fill:none;stroke-width:1.6}@media (prefers-color-scheme: dark){.bg{fill:#1a1a19}.a{stroke:#d95926;fill:#d95926}.b{stroke:#3987e5;fill:#3987e5}.c1{fill:#3987e5}.c2{fill:#5a96d8}.c3{fill:#199e70}.c4{fill:#d95926}.c5{fill:#797873}.p{fill:#f0efec}.q{fill:#c3c2b7}.g{stroke:#383835}}</style>',
     f'<rect class="bg" width="{W}" height="{H}"/>',
     '<text class="p" x="16" y="24" font-size="15" font-weight="600">fes-f2-gpu on an A100 80GB PCIe: the "unlocated" time is a one-time start-up</text>',
     '<text class="q" x="16" y="42" font-size="12">left: mean round trip per call over the first n calls (printed every 500 calls), 3 runs per arm; lines are the fit b + F/n. right: the steady-state call, arm B</text>']
x0, x1, y0, y1 = 70, 500, 330, 70
def X(n): return x0 + n / 2400 * (x1 - x0)
def Y(v): return y0 - v / 700 * (y0 - y1)
for t in range(0, 701, 100):
    o.append(f'<line class="g" x1="{x0}" y1="{Y(t):.1f}" x2="{x1}" y2="{Y(t):.1f}"/><text class="q" x="{x0 - 6}" y="{Y(t) + 4:.1f}" font-size="10" text-anchor="end">{t}</text>')
for n in range(0, 2401, 500):
    o.append(f'<text class="q" x="{X(n):.1f}" y="{y0 + 16}" font-size="10" text-anchor="middle">{n}</text>')
o.append(f'<text class="q" x="{(x0 + x1) / 2}" y="{y0 + 34}" font-size="11" text-anchor="middle">calls so far</text>')
o.append(f'<text class="q" x="16" y="{(y0 + y1) / 2}" font-size="11" transform="rotate(-90 16 {(y0 + y1) / 2})" text-anchor="middle">mean round trip, us per call</text>')
for fit, cls in ((A, "a"), (B, "b")):
    for inst, marker in (("koblitz-n23", "c"), ("char2-n25", "s")):
        runs = [v for k, v in fit.items() if inst in k]
        b = st.fmean(v["b_us_per_call"] for v in runs); F = st.fmean(v["F_ms_one_time"] for v in runs) * 1e3
        pts = " ".join(f"{X(n):.1f},{Y(b + F / n):.1f}" for n in range(300, 2401, 50))
        o.append(f'<polyline class="{cls} ln" points="{pts}"/>')
        for v in runs:
            for n, ms in v["points_calls_ms"]:
                cx, cy = X(n), Y(ms * 1e3)
                o.append(f'<circle class="{cls}" cx="{cx:.1f}" cy="{cy:.1f}" r="3"/>' if marker == "c" else f'<rect class="{cls}" x="{cx - 3:.1f}" y="{cy - 3:.1f}" width="6" height="6"/>')
        o.append(f'<line class="{cls}" stroke-dasharray="3 3" x1="{x0}" y1="{Y(b):.1f}" x2="{x1}" y2="{Y(b):.1f}"/>')
        o.append(f'<text class="q" x="{x1 + 4}" y="{Y(b) + 4:.1f}" font-size="10">b={b:.0f}</text>')
o.append(f'<text class="q" x="{x0 + 8}" y="{y0 + 52}" font-size="10">orange = A (before), blue = B (main); circles 24 unknowns, squares 26; dashed = steady-state b</text>')
# right: steady-state stack for B
bx = 600; sc = 240 / 220
o.append('<text class="p" x="600" y="64" font-size="12" font-weight="600">steady-state call, arm B (us)</text>')
yy = 80
for inst, tag in (("koblitz-n23", "24 unknowns"), ("char2-n25", "26 unknowns")):
    b = R[inst]["B"]; C = R[inst]["C"]
    rt = st.fmean(v["b_us_per_call"] for k, v in B.items() if inst in k)
    busy, kern = b["busy_us"]["mean"], b["kernel_gpu_us"]["mean"]
    floor = C["round_trip_ms"]["mean"] * 1e3; rust = (b["pack_ms"]["mean"] + b["verify_ms"]["mean"]) * 1e3
    segs = [("kernel", kern, "c1"), ("worker copies + reply", busy - kern, "c2"), ("pipe floor", floor, "c3"), ("Rust pack + verify", rust, "c4"), ("remainder", rt + rust - busy - floor - rust, "c5")]
    tot = sum(v for _, v, _ in segs)
    o.append(f'<text class="q" x="{bx}" y="{yy + 12}" font-size="11">{tag}: {tot:.0f} us</text>')
    x = bx
    for name, v, c in segs:
        w = max(v, 0) * sc
        o.append(f'<rect class="{c}" x="{x:.1f}" y="{yy + 20}" width="{w:.1f}" height="24"><title>{name}: {v:.1f} us</title></rect>')
        if w > 26: o.append(f'<text x="{x + w / 2:.1f}" y="{yy + 36}" font-size="10" text-anchor="middle" style="fill:#fff">{v:.0f}</text>')
        x += w
    yy += 64
yy += 6
for i, (n, c) in enumerate((("kernel (GPU)", "c1"), ("worker copies + reply", "c2"), ("pipe floor (null worker)", "c3"), ("Rust pack + verify", "c4"), ("remainder", "c5"))):
    o.append(f'<rect class="{c}" x="{bx}" y="{yy + i * 18}" width="10" height="10"/><text class="q" x="{bx + 16}" y="{yy + i * 18 + 9}" font-size="10">{n}</text>')
o.append(f'<text class="q" x="{bx}" y="{yy + 5 * 18 + 14}" font-size="10">one-time start-up F is about 150 ms per process, not per call</text>')
o.append("</svg>")
open(D + "fig_startup.svg", "w").write("\n".join(o))
