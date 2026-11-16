#!/usr/bin/env python3
"""SVG: DL1 cost ratio c=1.60 / c=2.00 against the M-eq price of one counted addition (F_q add and mod-ell add),
with the measured CPU ratios (validator replay) as horizontal markers. Pure stdlib; data from evidence/recompute.json.
usage: make_figure.py <item dir>"""
import json, os, sys, statistics as st
D = sys.argv[1]; RC = json.load(open(os.path.join(D, 'evidence', 'recompute.json')))
W, H, L, R, T, B = 760, 470, 70, 30, 55, 70
xs = [0.0, 0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.0]
def X(x): return L + (x / 2.0) ** 0.5 * (W - L - R)          # square-root axis: resolves 0..0.1 and 0..2
ymin, ymax = 0.60, 1.30
def Y(y): return H - B - (y - ymin) / (ymax - ymin) * (H - T - B)
out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif" font-size="12">',
       f'<rect width="{W}" height="{H}" fill="white"/>',
       f'<text x="{L}" y="22" font-size="15" font-weight="bold">J2 validation: the c = 1.60 gain depends on what an addition costs</text>',
       f'<text x="{L}" y="40" fill="#444">DL1 cost ratio c=1.60 / c=2.00 (paired, held-out seeds 4-8) vs M-eq price of one counted F_q or mod-ell addition [pcc:unverified]</text>']
for y in (0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3):
    out.append(f'<line x1="{L}" y1="{Y(y):.1f}" x2="{W - R}" y2="{Y(y):.1f}" stroke="{"#000" if y == 1.0 else "#ddd"}" stroke-width="{1.2 if y == 1.0 else 0.8}"/>')
    out.append(f'<text x="{L - 8}" y="{Y(y) + 4:.1f}" text-anchor="end">{y:.1f}</text>')
for x in xs:
    out.append(f'<line x1="{X(x):.1f}" y1="{H - B}" x2="{X(x):.1f}" y2="{H - B + 5}" stroke="#000"/>')
    out.append(f'<text x="{X(x):.1f}" y="{H - B + 19}" text-anchor="middle">{x:g}</text>')
out.append(f'<text x="{(L + W - R) / 2}" y="{H - 28}" text-anchor="middle">price of one counted addition, M-eq (sqrt axis); frozen N4v2 unit = 0; 5a band: 0.1 GAE = 8.2 M-eq, 1 GAE = 82 M-eq (off scale)</text>')
out.append(f'<text x="16" y="{(T + H - B) / 2}" transform="rotate(-90 16 {(T + H - B) / 2})" text-anchor="middle">cost ratio to c = 2.00</text>')
cols = {'21': '#1f77b4', '22': '#d62728'}
for k, q in (('21', '2097091|held'), ('22', '4194271|held')):
    sw = RC['add_ladd_weight_sweep'][q]
    pts = ' '.join(f"{X(x):.1f},{Y(sw[f'wx={x}|c=1.60'][2]):.1f}" for x in xs)
    out.append(f'<polyline points="{pts}" fill="none" stroke="{cols[k]}" stroke-width="2.2"/>')
    for x in xs: out.append(f"<circle cx=\"{X(x):.1f}\" cy=\"{Y(sw[f'wx={x}|c=1.60'][2]):.1f}\" r=\"3\" fill=\"{cols[k]}\"/>")
    be = sw['breakeven_wx_c1.60_M-eq_per_add_or_ladd']
    out.append(f'<line x1="{X(be):.1f}" y1="{Y(1.0) - 6:.1f}" x2="{X(be):.1f}" y2="{Y(1.0) + 6:.1f}" stroke="{cols[k]}" stroke-width="2"/>')
    out.append(f'<text x="{X(be) + 5:.1f}" y="{Y(1.0) - 9 - (12 if k == "22" else 0):.1f}" fill="{cols[k]}">k={k} break-even {be:.2f}</text>')
    cpu = st.median(RC['cpu_wall_ratios_to_c2.00'][f'validator_replay|{q}|c=1.60|cpu_total'])
    out.append(f'<line x1="{X(0.6) if k == "21" else X(0.0):.1f}" y1="{Y(cpu):.1f}" x2="{W - R if k == "22" else X(0.6):.1f}" y2="{Y(cpu):.1f}" stroke="{cols[k]}" stroke-dasharray="5,4" stroke-width="1.6"/>')
ins = RC['in_situ_LA_regression']['ladd_in_M_eq']
out.append(f'<line x1="{X(ins):.1f}" y1="{T}" x2="{X(ins):.1f}" y2="{H - B}" stroke="#2ca02c" stroke-dasharray="2,3" stroke-width="1.6"/>')
out.append(f'<text x="{X(ins) - 6:.1f}" y="{T + 12}" text-anchor="end" fill="#2ca02c">in-situ fit: {ins:.2f} M-eq per counted mod-ell add</text>')
ly = H - B - 72
out.append(f'<rect x="{L + 10}" y="{ly - 14}" width="330" height="66" fill="white" stroke="#bbb"/>')
out.append(f'<line x1="{L + 18}" y1="{ly}" x2="{L + 48}" y2="{ly}" stroke="#1f77b4" stroke-width="2.2"/><text x="{L + 54}" y="{ly + 4}">k = 21 (q = 2,097,091): M-eq with additions priced</text>')
out.append(f'<line x1="{L + 18}" y1="{ly + 16}" x2="{L + 48}" y2="{ly + 16}" stroke="#d62728" stroke-width="2.2"/><text x="{L + 54}" y="{ly + 20}">k = 22 (q = 4,194,271): same</text>')
out.append(f'<line x1="{L + 18}" y1="{ly + 32}" x2="{L + 48}" y2="{ly + 32}" stroke="#444" stroke-dasharray="5,4" stroke-width="1.6"/><text x="{L + 54}" y="{ly + 36}">measured total CPU ratio, validator replay (medians)</text>')
out.append('</svg>')
open(os.path.join(D, 'evidence', 'ratio_vs_addition_price.svg'), 'w').write('\n'.join(out))
print('wrote evidence/ratio_vs_addition_price.svg')
