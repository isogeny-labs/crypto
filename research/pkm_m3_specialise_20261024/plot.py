#!/usr/bin/env python3
"""Writes degree_vs_N.svg from the completed (exit 0) runs in runs/: max productive degree vs N per arm."""
import re, glob, os
N = {'m3_t5': 15, 'm3_t7': 21, 'm3_t8': 24}; pts = {}
for mem in glob.glob('runs/*.mem'):
    name = os.path.basename(mem)[:-4]; kv = dict(x.split('=') for x in open(mem).read().split()[1:])
    if kv['exit'] != '0': continue
    sysn, arm = name.replace('pkm_kummer_p2013265921_', '').split('__'); n = N.get(sysn[:5])
    if n is None or arm in ('H1b', 'M1b'): continue
    d = 0
    for l in open(f'runs/{name}.log', errors='replace'):
        m = re.match(r'\s*(\d+)\s+\d+\s+\d+\s+\d+ x\s*\d+\s+[\d.]+%\s+(\d+) new', l)
        if m and int(m.group(2)) > 0: d = max(d, int(m.group(1)))
    pts.setdefault(arm, {})[n] = d
col = {'F0': '#333', 'H1': '#1b9e77', 'H2': '#7570b3', 'M1': '#d95f02', 'M2': '#e7298a'}
W, H, x0, y0 = 480, 300, 50, 260
X = lambda n: x0 + (n - 13) * 35; Y = lambda d: y0 - d * 28
o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="sans-serif" font-size="11">',
     f'<rect width="{W}" height="{H}" fill="white"/>', f'<line x1="{x0}" y1="{y0}" x2="{W-20}" y2="{y0}" stroke="#000"/>', f'<line x1="{x0}" y1="20" x2="{x0}" y2="{y0}" stroke="#000"/>']
for d in range(0, 9): o.append(f'<text x="{x0-18}" y="{Y(d)+4}">{d}</text>')
for n in (15, 21, 24): o.append(f'<text x="{X(n)-8}" y="{y0+16}">N={n}</text>')
o.append(f'<line x1="{x0}" y1="{Y(8)}" x2="{W-20}" y2="{Y(8)}" stroke="#999" stroke-dasharray="4"/><text x="{W-90}" y="{Y(8)-4}" fill="#666">degree 8</text>')
for k, (arm, p) in enumerate(sorted(pts.items())):
    c = col.get(arm, '#000'); xs = sorted(p)
    o.append(f'<polyline fill="none" stroke="{c}" stroke-width="2" points="{" ".join(f"{X(n)},{Y(p[n])}" for n in xs)}"/>')
    for n in xs: o.append(f'<circle cx="{X(n)}" cy="{Y(p[n])}" r="3.5" fill="{c}"/>')
    o.append(f'<text x="{60+k*60}" y="14" fill="{c}">{arm}</text>')
o.append('</svg>'); open('degree_vs_N.svg', 'w').write('\n'.join(o))
