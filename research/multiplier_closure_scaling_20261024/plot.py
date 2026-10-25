#!/usr/bin/env python3
"""Gain (S_inc/S_mult, geometric mean over seeds) vs log2 r; reads analysis.json; writes gain_vs_log2r.svg (stdlib only)."""
import json, math, collections
d=json.load(open('analysis.json'))['instances']
ser=collections.defaultdict(list)
for tag in ('all','adm'):
    g=collections.defaultdict(list)
    for r in d:
        if 'R_'+tag in r: g[(r['kind'],r['family'],r['size'])].append((r['log2_r'],r['R_'+tag]))
    for k,v in g.items():
        ser[(k[0] if k[0]=='binary' else k[1],tag)].append((sum(a for a,_ in v)/len(v), 1/math.exp(sum(math.log(b) for _,b in v)/len(v))))
W,H,L,R,T,B=720,460,70,20,30,60
xs=[p[0] for v in ser.values() for p in v]; x0,x1=min(xs)-1,max(xs)+1
y0,y1=math.log10(0.2),math.log10(20)
X=lambda x:L+(x-x0)/(x1-x0)*(W-L-R); Y=lambda y:T+(y1-math.log10(y))/(y1-y0)*(H-T-B)
DASH='stroke-dasharray="6 4"'
col={'j0':'#1f77b4','j1728':'#d62728','binary':'#2ca02c'}
o=[f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="12"><rect width="100%" height="100%" fill="white"/>']
for t in (0.25,0.5,1,2,5,10):
    o.append(f'<line x1="{L}" x2="{W-R}" y1="{Y(t)}" y2="{Y(t)}" stroke="{"#000" if t==1 else "#ddd"}"/><text x="{L-6}" y="{Y(t)+4}" text-anchor="end">{t}x</text>')
for x in range(int(x0)+1,int(x1)+1,2): o.append(f'<text x="{X(x)}" y="{H-B+18}" text-anchor="middle">{x}</text>')
o.append(f'<text x="{(L+W-R)/2}" y="{H-B+38}" text-anchor="middle">log2 r</text><text x="14" y="{H/2}" transform="rotate(-90 14 {H/2})" text-anchor="middle">gain S_inc / S_mult (geo-mean of 3 seeds)</text>')
o.append(f'<text x="{L}" y="18" font-weight="bold">Multiplier-closure gain vs r (solid: all rows; dashed: admissible, &gt;=256 columns)</text>')
for (name,tag),pts in sorted(ser.items()):
    pts.sort(); c=col[name]
    o.append(f'<polyline fill="none" stroke="{c}" stroke-width="2" {DASH if tag=="adm" else ""} points="'+" ".join(f"{X(a):.1f},{Y(b):.1f}" for a,b in pts)+'"/>')
    for a,b in pts: o.append(f'<circle cx="{X(a):.1f}" cy="{Y(b):.1f}" r="3" fill="{c}"/>')
for i,n in enumerate(col): o.append(f'<rect x="{L+10+i*90}" y="{H-14}" width="10" height="10" fill="{col[n]}"/><text x="{L+24+i*90}" y="{H-5}">{n}</text>')
o.append('</svg>')
open('gain_vs_log2r.svg','w').write("\n".join(o))
