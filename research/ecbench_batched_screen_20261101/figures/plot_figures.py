#!/usr/bin/env python3
"""fig1-ratio.svg: counted GAE of each batch_trials arm relative to the baseline
batch 64, by cell, with the paired 95% intervals.  fig2-phases.svg: where the
counted GAE sits, per phase, for batch 1, 16, 64 and 256 on the smallest and
largest cell.  Standard library only; reads summary.json."""
import json, math, os
HERE = os.path.dirname(os.path.abspath(__file__))
S = json.load(open(f"{HERE}/summary.json"))
CELLS = ["n13a0", "n17a1", "n19a0", "n23a0"]
COL = {"n13a0": "#0072B2", "n17a1": "#D55E00", "n19a0": "#009E73", "n23a0": "#CC79A7"}
BAT = [1, 2, 4, 8, 16, 32, 64, 128, 256]
esc = lambda s: s.replace("&", "&amp;").replace("<", "&lt;")
FONT = "font-family='DejaVu Sans, Arial, sans-serif'"

def fig1():
    W, H, L, R, T, B = 760, 470, 70, 150, 50, 62
    x = lambda b: L + (math.log2(b) / 8) * (W - L - R)
    ymin, ymax = 0.5, 2.6
    y = lambda v: T + (math.log(ymax / v) / math.log(ymax / ymin)) * (H - T - B)
    o = [f"<svg xmlns='http://www.w3.org/2000/svg' width='{W}' height='{H}' viewBox='0 0 {W} {H}' {FONT} font-size='12'>",
         f"<rect width='{W}' height='{H}' fill='white'/>",
         f"<text x='{L}' y='24' font-size='15' font-weight='bold'>Counted cost of ic.koblitz_batched relative to the incumbent batch_trials = 64</text>",
         f"<text x='{L}' y='41' fill='#444'>mean total GAE per run, paired by target; bars are 95% cluster-bootstrap intervals; local L0 development screen</text>"]
    for v in (0.5, 0.6, 0.8, 1.0, 1.25, 1.6, 2.0, 2.5):
        o.append(f"<line x1='{L}' x2='{W-R}' y1='{y(v):.1f}' y2='{y(v):.1f}' stroke='{'#222' if v==1.0 else '#ddd'}' stroke-width='{1.4 if v==1.0 else 1}'/>")
        o.append(f"<text x='{L-8}' y='{y(v)+4:.1f}' text-anchor='end'>{v:g}</text>")
    for b in BAT:
        o.append(f"<line x1='{x(b):.1f}' x2='{x(b):.1f}' y1='{T}' y2='{H-B}' stroke='#f0f0f0'/>")
        o.append(f"<text x='{x(b):.1f}' y='{H-B+18}' text-anchor='middle'>{b}</text>")
    o.append(f"<text x='{(L+W-R)/2}' y='{H-14}' text-anchor='middle'>batch_trials (queries collected before each rank check)</text>")
    o.append(f"<text transform='translate(18,{(T+H-B)/2}) rotate(-90)' text-anchor='middle'>total GAE / total GAE at batch 64</text>")
    for c in CELLS:
        pts = []
        for b in BAT:
            r = 1.0 if b == 64 else S["cells"][c]["arms"][f"ic-b{b}"]["ratio_to_b64"]["ratio"]
            lo, hi = (1.0, 1.0) if b == 64 else S["cells"][c]["arms"][f"ic-b{b}"]["ratio_to_b64"]["ci95"]
            pts.append((x(b), y(r), y(lo), y(hi)))
        o.append(f"<polyline fill='none' stroke='{COL[c]}' stroke-width='2' points='{' '.join(f'{px:.1f},{py:.1f}' for px,py,_,_ in pts)}'/>")
        for px, py, ylo, yhi in pts:
            o.append(f"<line x1='{px:.1f}' x2='{px:.1f}' y1='{ylo:.1f}' y2='{yhi:.1f}' stroke='{COL[c]}' stroke-width='1.5'/>")
            o.append(f"<circle cx='{px:.1f}' cy='{py:.1f}' r='3.2' fill='{COL[c]}'/>")
    for i, c in enumerate(CELLS):
        yy = T + 20 + i * 20
        o.append(f"<line x1='{W-R+16}' x2='{W-R+40}' y1='{yy}' y2='{yy}' stroke='{COL[c]}' stroke-width='2'/><circle cx='{W-R+28}' cy='{yy}' r='3.2' fill='{COL[c]}'/>")
        o.append(f"<text x='{W-R+46}' y='{yy+4}'>{c}</text>")
    o.append("</svg>")
    open(f"{HERE}/fig1-ratio.svg", "w").write("\n".join(o))

def fig2():
    PH = ["factor_base", "oracle_setup", "relations", "linear_algebra", "descent_setup", "descent", "verify"]
    PC = {"factor_base": "#999999", "oracle_setup": "#E69F00", "relations": "#0072B2", "linear_algebra": "#56B4E9",
          "descent_setup": "#009E73", "descent": "#CC79A7", "verify": "#D55E00"}
    cells = ["n13a0", "n23a0"]; sel = [1, 16, 64, 256]
    W, H, L, T, B = 820, 470, 80, 60, 70
    maxv = max(S["cells"][c]["arms"][f"ic-b{b}"]["mean_gae_total"] for c in cells for b in sel)
    y = lambda v: H - B - (v / (maxv * 1.05)) * (H - T - B)
    o = [f"<svg xmlns='http://www.w3.org/2000/svg' width='{W}' height='{H}' viewBox='0 0 {W} {H}' {FONT} font-size='12'>",
         f"<rect width='{W}' height='{H}' fill='white'/>",
         f"<text x='{L}' y='24' font-size='15' font-weight='bold'>Where the counted GAE sits (mean per run)</text>",
         f"<text x='{L}' y='41' fill='#444'>the pair table (oracle_setup) does not depend on batch_trials; only relations and linear_algebra move</text>"]
    for v in range(0, int(maxv * 1.05), 50000):
        o.append(f"<line x1='{L}' x2='{W-170}' y1='{y(v):.1f}' y2='{y(v):.1f}' stroke='#ddd'/><text x='{L-8}' y='{y(v)+4:.1f}' text-anchor='end'>{v//1000}k</text>")
    bw, gap, x0 = 44, 18, L + 20
    xs = x0
    for c in cells:
        for b in sel:
            base = 0.0
            for p in PH:
                v = S["cells"][c]["arms"][f"ic-b{b}"]["phase_gae"][p]
                if v <= 0: continue
                o.append(f"<rect x='{xs}' y='{y(base+v):.1f}' width='{bw}' height='{max(y(base)-y(base+v),0.4):.1f}' fill='{PC[p]}'/>")
                base += v
            o.append(f"<text x='{xs+bw/2}' y='{y(base)-5:.1f}' text-anchor='middle'>{base/1000:.0f}k</text>")
            o.append(f"<text x='{xs+bw/2}' y='{H-B+16}' text-anchor='middle'>b{b}</text>")
            xs += bw + gap
        o.append(f"<text x='{xs-(bw+gap)*len(sel)/2-gap/2}' y='{H-B+38}' text-anchor='middle' font-weight='bold'>{c}</text>")
        xs += 34
    o.append(f"<text transform='translate(18,{(T+H-B)/2}) rotate(-90)' text-anchor='middle'>total GAE per run</text>")
    for i, p in enumerate(PH):
        yy = T + 18 + i * 20
        o.append(f"<rect x='{W-150}' y='{yy-10}' width='14' height='14' fill='{PC[p]}'/><text x='{W-130}' y='{yy+2}'>{p}</text>")
    o.append("</svg>")
    open(f"{HERE}/fig2-phases.svg", "w").write("\n".join(o))

fig1(); fig2()
print("wrote fig1-ratio.svg fig2-phases.svg")
