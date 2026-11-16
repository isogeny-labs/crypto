"""Lay out e14/score.json as an SVG timeline (no computation). Usage: plot_e14.py score.json OUT.svg"""
import json, sys
s = json.load(open(sys.argv[1])); W, H = 1100, 420
x0, pw = 200, 840; t0, t1 = 4.0, 29.2  # hours UTC on 2026-11-09; 29.12 = deadline 05:07 next day
sx = lambda h: x0 + (h - t0) / (t1 - t0) * pw
hm = lambda a, b: a + b / 60
o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica,Arial,sans-serif"><rect width="{W}" height="{H}" fill="#fff"/>',
     f'<text x="20" y="30" font-size="15" font-weight="bold">E14 on RunPod pod {s["pod"]["id"]} (64 vCPU, 504 GiB): what ran before the provider terminated the pod</text>']
for h in range(4, 30, 2):
    o.append(f'<line x1="{sx(h)}" y1="55" x2="{sx(h)}" y2="330" stroke="#eee"/><text x="{sx(h)}" y="348" font-size="10" text-anchor="middle">{h%24:02d}:00</text>')
o.append(f'<text x="{x0+pw/2}" y="368" font-size="12" text-anchor="middle">UTC, 2026-11-09 to 2026-11-10</text>')
rows = [('pod idle (entrypoint fix)', hm(4,2), hm(5,7), '#cbd5e0', ''),
        ('build + gates G0, G1', hm(5,7), hm(5,43), '#2f855a', 'passed'),
        ('E14b 6:10:3 d1 (pe --to 4)', hm(5,43), hm(10,17), '#a0aec0', '≥ 4.56 h, no output (hky_par: 190 s)'),
        ('E14b d3, d4', None, None, None, 'not started'),
        ('E14a 3:21:7 degree 5 (4 draws)', None, None, None, 'not started'),
        ('E14c 4:28:7 hky_window', None, None, None, 'not started')]
for i, (lab, a, b, c, note) in enumerate(rows):
    y = 70 + i * 40
    o.append(f'<text x="{x0-8}" y="{y+18}" font-size="12" text-anchor="end">{lab}</text>')
    if a is not None:
        o.append(f'<rect x="{sx(a)}" y="{y+4}" width="{max(sx(b)-sx(a),2)}" height="22" fill="{c}"/>')
        o.append(f'<text x="{sx(b)+6}" y="{y+19}" font-size="11">{note}</text>')
    else:
        o.append(f'<text x="{sx(t0)+6}" y="{y+19}" font-size="11" fill="#c53030">{note}</text>')
for h, lab, col in [(hm(10,22), 'pod terminated (balance 0)', '#c53030'), (hm(29,7), 'registered deadline', '#555')]:
    o.append(f'<line x1="{sx(h)}" y1="55" x2="{sx(h)}" y2="330" stroke="{col}" stroke-dasharray="5,3"/><text x="{sx(h)-4}" y="52" font-size="11" fill="{col}" text-anchor="end">{lab}</text>')
o.append(f'<text x="20" y="398" font-size="12">Readings: E14b <tspan font-weight="bold">{s["E14b"]["reading"]}</tspan> · E14a <tspan font-weight="bold">{s["E14a"]["reading"]}</tspan> · E14c <tspan font-weight="bold">{s["E14c"]["reading"]}</tspan>.  Terminated between the 10:17 upload and the missing 10:27 upload.</text>')
o.append('</svg>')
open(sys.argv[2], 'w').write('\n'.join(o))
