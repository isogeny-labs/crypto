"""Lay out e14/score.json (r2 section) as an SVG timeline (no computation). Usage: plot_e14r2.py score.json OUT.svg"""
import json, sys, datetime as dt
s = json.load(open(sys.argv[1]))['r2']; W, H = 1100, 470
T0 = dt.datetime(2026, 11, 12, 4, 0)
hrs = lambda iso: (dt.datetime.strptime(iso, '%Y-%m-%dT%H:%M:%SZ') - T0).total_seconds() / 3600
x0, pw, t1 = 230, 820, 38.0
sx = lambda h: x0 + h / t1 * pw
o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="Helvetica,Arial,sans-serif"><rect width="{W}" height="{H}" fill="#fff"/>',
     f'<text x="20" y="30" font-size="15" font-weight="bold">E14 r2 on RunPod pod {s["pod"]["id"]} (32 vCPU AMD EPYC 9575F, 503 GiB): what ran and what was recorded</text>']
for h in range(0, 39, 3):
    lab = (T0 + dt.timedelta(hours=h)).strftime('%H:%M')
    o.append(f'<line x1="{sx(h)}" y1="55" x2="{sx(h)}" y2="380" stroke="#eee"/><text x="{sx(h)}" y="398" font-size="10" text-anchor="middle">{lab}</text>')
o.append(f'<text x="{x0+pw/2}" y="418" font-size="12" text-anchor="middle">UTC, 2026-11-12 04:00 to 2026-11-13 18:00</text>')
a = s['E14a']; d0, d1 = a['draws'][0], a['draws'][1]
g0, gA = hrs('2026-11-12T04:57:05Z'), hrs(a['started'] + ('' if a['started'].endswith('Z') else 'Z'))
f0, f1 = hrs(d0['finished_approx_utc']), hrs(d1['finished_approx_utc'])
last, dl = hrs(s['pod']['last_pull']), hrs(s['pod']['deadline'])
rows = [('boot, build, gates G0-G3', g0, gA, '#2f855a', 'all passed', None),
        ('E14a draw 0 (degree 5)', gA, f0, '#2b6cb0', f'd_F = 5, {d0["secs"]:,.0f} s, valid', None),
        ('E14a draw 1 (degree 5)', f0, f1, '#2b6cb0', f'd_F = 5, {d1["secs"]:,.0f} s, valid', None),
        ('E14a draw 3 (degree 5)', f1, f1 + 10.6, '#cbd5e0', 'running at last pull; record lost with the pod', '5,3'),
        ('E14a draw 4', f1 + 10.6, dl, '#edf2f7', 'cut off by deadline', '2,2'),
        ('E14b 6:10:3 controls (pe)', None, None, None, 'not started (E14a held the pod)', None),
        ('E14c 4:28:7 hky_window', None, None, None, 'not started', None)]
for i, (lab, a_, b_, c, note, dash) in enumerate(rows):
    y = 65 + i * 42
    o.append(f'<text x="{x0-8}" y="{y+18}" font-size="12" text-anchor="end">{lab}</text>')
    if a_ is not None:
        st = f' stroke="#4a5568" stroke-dasharray="{dash}"' if dash else ''
        o.append(f'<rect x="{sx(a_)}" y="{y+4}" width="{max(sx(b_)-sx(a_),2)}" height="22" fill="{c}"{st}/>')
        tx = sx(b_) + 6 if sx(b_) < 820 else sx(a_) + 6
        o.append(f'<text x="{tx}" y="{y+19}" font-size="11">{note}</text>')
    else:
        o.append(f'<text x="{sx(0)+6}" y="{y+19}" font-size="11" fill="#c53030">{note}</text>')
for h, lab, col in [(last, 'last pull 02:50', '#c05621'), (dl, 'deadline 16:57', '#555')]:
    o.append(f'<line x1="{sx(h)}" y1="55" x2="{sx(h)}" y2="380" stroke="{col}" stroke-dasharray="5,3"/><text x="{sx(h)-4}" y="52" font-size="11" fill="{col}" text-anchor="end">{lab}</text>')
o.append(f'<text x="20" y="446" font-size="12">Readings: E14a <tspan font-weight="bold">{a["reading"]}</tspan> (2 measured, both d_F = 5; the cell rule needs 3 of 4) · E14b <tspan font-weight="bold">{s["E14b"]["reading"]}</tspan> · E14c <tspan font-weight="bold">{s["E14c"]["reading"]}</tspan>.</text>')
o.append('<text x="20" y="464" font-size="11" fill="#555">Draw 3 and draw 4 bars are expected spans (about 10.6 h per draw), not observations.</text>')
o.append('</svg>')
open(sys.argv[2], 'w').write('\n'.join(o))
