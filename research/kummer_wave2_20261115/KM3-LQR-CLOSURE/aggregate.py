#!/usr/bin/env python3
"""aggregate raw/*.json -> table.json, table.md, figure.svg  (usage: aggregate.py RAWDIR)"""
import sys, json, glob, os
raw = sys.argv[1]; rows = []
for f in sorted(glob.glob(os.path.join(raw, "m[23]_*.json"))):
    r = json.load(open(f)); cells = r["cells"]
    dF = r["d_F"]
    below = [c for c in cells if c["c"] < dF]
    lam_kills_all = all(c.get("lam_kills") for c in below)
    lam1_nz = all(c.get("lam_one", 0) != 0 for c in cells)
    other = [c.get("lamOther_nonzero_rows") for c in below if c["c"] == dF - 1]
    row = dict(file=os.path.basename(f), m=r["m"], p=r["p"], t=r["t"], idx=r["idx"], d_F=dF, dF_f2=r.get("dF_f2"),
               c_star=r.get("c_star"), c_star_eq_dF_minus_1=(r.get("c_star") == dF - 1), lam_kills_all_below=lam_kills_all,
               lam1_nonzero=lam1_nz, dim_V_dFminus1=below[-1]["dim"] if below else None,
               lamOther_nonzero_rows_at_dFminus1=other[0] if other else None,
               pi_rank_dFminus1=below[-1].get("pi_rank") if below else None, secs=r["secs"])
    rows.append(row)
json.dump(rows, open("table.json", "w"), indent=1)
with open("table.md", "w") as o:
    o.write("| m | p | t | draw | d_F | d_F^(f2) | c* | c*=d_F-1 | lam kills all c<d_F | lam(1)!=0 | dim V_{d_F-1} | rank pi(V_{d_F-1}) | other-draw lam nonzero rows at d_F-1 | s |\n|--|--|--|--|--|--|--|--|--|--|--|--|--|--|\n")
    for r in rows:
        o.write(f"| {r['m']} | {r['p']} | {r['t']} | {r['idx']} | {r['d_F']} | {r['dF_f2'] if r['m']==3 else '-'} | {r['c_star']} | {r['c_star_eq_dF_minus_1']} | {r['lam_kills_all_below']} | {r['lam1_nonzero']} | {r['dim_V_dFminus1']} | {r['pi_rank_dFminus1'] if r['m']==3 else '-'} | {r['lamOther_nonzero_rows_at_dFminus1']} | {r['secs']} |\n")
m3 = [r for r in rows if r["m"] == 3]; m2 = [r for r in rows if r["m"] == 2]
summ = dict(m3_cells=len(m3), m3_dFf2_eq_dF=sum(r["dF_f2"] == r["d_F"] for r in m3), m3_cstar_eq=sum(r["c_star_eq_dF_minus_1"] for r in m3),
            m3_lam1=sum(r["lam1_nonzero"] for r in m3), m2_cells=len(m2), m2_cstar_eq=sum(r["c_star_eq_dF_minus_1"] for r in m2),
            m2_lam1=sum(r["lam1_nonzero"] for r in m2), m3_dF=sorted(set((r["t"], r["d_F"]) for r in m3)), m2_dF=sorted(set((r["t"], r["d_F"]) for r in m2)))
json.dump(summ, open("summary.json", "w"), indent=1); print(json.dumps(summ))
# figure: x = cell label, bars: d_F, c*+1 ; simple dot plot
W, H = 760, 340; L, B, T = 60, 60, 40
cells = m3 + m2
n = len(cells); step = (W - L - 20) / max(n, 1)
def y(v): return H - B - v * 28
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="10">',
       f'<rect width="{W}" height="{H}" fill="#ffffff"/>',
       f'<text x="{L}" y="20" font-size="13">d_F (filled) vs c*+1 (ring) per cell; equal means lambda is a closure pseudo-solution there [pcc:unverified]</text>']
for v in range(0, 8):
    svg.append(f'<line x1="{L}" y1="{y(v)}" x2="{W-10}" y2="{y(v)}" stroke="#dddddd"/><text x="{L-18}" y="{y(v)+3}">{v}</text>')
for i, r in enumerate(cells):
    x = L + (i + 0.5) * step
    col = "#1b6ca8" if r["m"] == 3 else "#b5561b"
    svg.append(f'<circle cx="{x:.1f}" cy="{y(r["d_F"])}" r="3.5" fill="{col}"/>')
    if r["c_star"] is not None:
        svg.append(f'<circle cx="{x:.1f}" cy="{y(r["c_star"]+1)}" r="6" fill="none" stroke="#222222"/>')
    if r["m"] == 3 and r["dF_f2"] is not None:
        svg.append(f'<path d="M{x-4:.1f},{y(r["dF_f2"])-8} l4,-6 l4,6 z" fill="#2a8a3a"/>')
    lab = f'm{r["m"]} t{r["t"]} p{str(r["p"])[:2]} #{r["idx"]}'
    svg.append(f'<text transform="translate({x:.1f},{H-B+8}) rotate(60)" font-size="7">{lab}</text>')
svg.append(f'<text x="{L}" y="{H-6}" font-size="9">blue = m3 d_F, orange = m2 d_F, green triangle = d_F^(f2) (m3), ring = c*+1 (largest c with lambda(V_c)=0, plus 1)</text></svg>')
open("figure.svg", "w").write("\n".join(svg))
import xml.dom.minidom as m; m.parse("figure.svg"); print("svg ok")
