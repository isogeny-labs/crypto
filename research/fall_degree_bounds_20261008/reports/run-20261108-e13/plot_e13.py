"""Lay out e13/score.json as SVG (no computation). Usage: plot_e13.py score.json OUT.svg"""
import json, sys
s = json.load(open(sys.argv[1])); W, H = 1100, 470
o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica,Arial,sans-serif"><rect width="{W}" height="{H}" fill="#fff"/>']
# panel 1: pe deg-4 closure rank vs hky_par deg-4 dim, censored draws
x0, y0, pw, ph = 70, 60, 420, 330; lo, hi = 100000, 200000
sx = lambda v: x0 + (v - lo) / (hi - lo) * pw; sy = lambda v: y0 + ph - (v - lo) / (hi - lo) * ph
o.append(f'<text x="{x0}" y="35" font-size="14" font-weight="bold">E13b: degree-4 rank, pe vs hky_par (12 censored draws)</text>')
o.append(f'<rect x="{x0}" y="{y0}" width="{pw}" height="{ph}" fill="none" stroke="#999"/>')
o.append(f'<line x1="{sx(lo)}" y1="{sy(lo)}" x2="{sx(hi)}" y2="{sy(hi)}" stroke="#aaa" stroke-dasharray="4,3"/>')
for v in range(lo, hi + 1, 25000):
    o.append(f'<text x="{sx(v)}" y="{y0+ph+16}" font-size="10" text-anchor="middle">{v//1000}k</text>')
    o.append(f'<text x="{x0-6}" y="{sy(v)+4}" font-size="10" text-anchor="end">{v//1000}k</text>')
o.append(f'<text x="{x0+pw/2}" y="{y0+ph+36}" font-size="12" text-anchor="middle">hky_par degree-4 dim (E10 record)</text>')
o.append(f'<text x="18" y="{y0+ph/2}" font-size="12" text-anchor="middle" transform="rotate(-90 18 {y0+ph/2})">pe degree-4 closure_rank (E13b)</text>')
col = {'3-21-7': '#2b6cb0', '3-24-8': '#2f855a', '3-27-9': '#c05621'}
for r in s['E13b']['censored']:
    if 'pe_rank4' in r:
        o.append(f'<circle cx="{sx(r["hky_par_dim4"])}" cy="{sy(r["pe_rank4"])}" r="5" fill="{col[r["cell"]]}" fill-opacity="0.7"/>')
for i, (c, k) in enumerate(col.items()):
    o.append(f'<circle cx="{x0+14}" cy="{y0+18+i*18}" r="5" fill="{k}"/><text x="{x0+24}" y="{y0+22+i*18}" font-size="11">{c.replace("-",":")} (all on y = x, unrefuted, saturated)</text>')
# panel 2: status grid
gx, gy = 590, 60
o.append(f'<text x="{gx}" y="35" font-size="14" font-weight="bold">Every registered E13 unit (13 GiB cap, 4 vCPU)</text>')
C = {'agree': '#2f855a', 'refutes at recorded d_F': '#2f855a', 'DIFFERS': '#c53030'}
rows = []
for cell in ['3-21-7', '3-24-8', '3-27-9']:
    rows.append((f'E13b {cell.replace("-",":")} censored', [(r['draw'], r['status']) for r in s['E13b']['censored'] if r['cell'] == cell]))
for cell in ['4-12-3', '6-10-3']:
    rows.append((f'E13b {cell.replace("-",":")} control', [(r['draw'], r['status']) for r in s['E13b']['controls'] if r['cell'] == cell]))
rows.append(('E13a 3:21:7 degree 5', [(r['draw'], str(r['status'])) for r in s['E13a']['draws']]))
rows.append(('E13c 4:28:7 (gate 0)', [('-', 'not completed')]))
for i, (lab, cells) in enumerate(rows):
    y = gy + 20 + i * 40
    o.append(f'<text x="{gx}" y="{y+18}" font-size="12">{lab}</text>')
    for j, (d, st) in enumerate(cells):
        x = gx + 200 + j * 72; c = C.get(st, '#a0aec0')
        o.append(f'<rect x="{x}" y="{y}" width="64" height="28" rx="4" fill="{c}"/><text x="{x+32}" y="{y+18}" font-size="11" fill="#fff" text-anchor="middle">d{d}</text>')
ly = gy + 20 + len(rows) * 40 + 10
for j, (lab, c) in enumerate([('matches hky_par or recorded d_F', '#2f855a'), ('differs', '#c53030'), ('not completed (memory)', '#a0aec0')]):
    xx=[0,215,300][j]; o.append(f'<rect x="{gx+xx}" y="{ly}" width="14" height="14" fill="{c}"/><text x="{gx+xx+20}" y="{ly+12}" font-size="11">{lab}</text>')
o.append(f'<text x="{gx}" y="{ly+40}" font-size="12">E13b registered reading: <tspan font-weight="bold">{s["E13b"]["reading"]}</tspan> (0 draws differ; 3 controls unmeasured)</text>')
o.append(f'<text x="{gx}" y="{ly+58}" font-size="12">E13a: <tspan font-weight="bold">{s["E13a"]["reading"]}</tspan>.  E13c: <tspan font-weight="bold">no reading</tspan> (gate build OOM).</text>')
o.append('</svg>')
open(sys.argv[2], 'w').write('\n'.join(o))
