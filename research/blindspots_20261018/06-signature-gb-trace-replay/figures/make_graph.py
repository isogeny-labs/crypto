# Lays out recorded values from raw/card-14-summarize.stdout.log (no computation of results).
data = {  # cell: (sig bound*, f4r, f4n, labels n_refuted sig/f4r/f4n)
 "c13-3-4": (0.494, 0.885, 1.120, "sig 0/16; f4r,f4n 16/16"),
 "c15-3-4": (1.677, None, None, "sig 0/16 (15 capped); no replay rows"),
 "c15-5-3": (0.498, 0.727, 1.405, "sig 0/16 (8 capped); f4r,f4n 8/16"),
 "c17-4-3": (0.492, 0.587, 1.160, "sig 0/16; f4r,f4n 16/16"),
}
cols = {"sig":"#b2182b","f4r":"#2166ac","f4n":"#4d9221"}
W,H=760,430; x0,y0,pw,ph=70,60,660,270
ymax=2.0
def Y(v): return y0+ph-ph*v/ymax
o=[f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="sans-serif" font-size="12">',
 '<rect width="100%" height="100%" fill="#ffffff"/>',
 '<text x="70" y="22" font-size="15" font-weight="bold">Item 06: R = touches(f4) / cost(X), geometric mean by cell</text>',
 '<text x="70" y="40" fill="#444">Recorded values, raw/card-14-summarize.stdout.log. sig bars are upper bounds (stopped jobs), not measurements of R.</text>']
for v in (0,0.5,1,1.5,2):
    o.append(f'<line x1="{x0}" x2="{x0+pw}" y1="{Y(v)}" y2="{Y(v)}" stroke="#ddd"/><text x="{x0-8}" y="{Y(v)+4}" text-anchor="end">{v}</text>')
o.append(f'<line x1="{x0}" x2="{x0+pw}" y1="{Y(1.5)}" y2="{Y(1.5)}" stroke="#000" stroke-dasharray="6 4"/><text x="{x0+pw-4}" y="{Y(1.5)-4}" text-anchor="end">qualify threshold 1.5</text>')
o.append(f'<line x1="{x0}" x2="{x0+pw}" y1="{Y(1/1.5)}" y2="{Y(1/1.5)}" stroke="#888" stroke-dasharray="2 3"/><text x="{x0+4}" y="{Y(1/1.5)-4}" text-anchor="start" fill="#555">regress line 1/1.5</text>')
gw=pw/4
for i,(c,(s,r,n,lab)) in enumerate(data.items()):
    gx=x0+i*gw+20
    for j,(k,v) in enumerate((("sig",s),("f4r",r),("f4n",n))):
        bx=gx+j*48
        if v is None:
            o.append(f'<text x="{bx+20}" y="{Y(0)-6}" text-anchor="middle" fill="#777">n/a</text>')
        else:
            o.append(f'<rect x="{bx}" y="{Y(v)}" width="40" height="{Y(0)-Y(v)}" fill="{cols[k]}"/>')
            o.append(f'<text x="{bx+20}" y="{Y(v)-4}" text-anchor="middle">{"&#8804;" if k=="sig" else ""}{v:.2f}</text>')
    o.append(f'<text x="{gx+70}" y="{y0+ph+18}" text-anchor="middle" font-weight="bold">{c}</text>')
    o.append(f'<text x="{gx+70}" y="{y0+ph+34}" text-anchor="middle" font-size="9" fill="#444">{lab}</text>')
for k,(t,x) in enumerate((("sig (upper bound)",70),("f4r exact replay",230),("f4n negative-filter replay",390))):
    kk=["sig","f4r","f4n"][k]
    o.append(f'<rect x="{x}" y="395" width="12" height="12" fill="{cols[kk]}"/><text x="{x+18}" y="405">{t}</text>')
o.append('<text x="70" y="424" font-size="10" fill="#444">Paired by target against f4; replay cost includes the held-out trace amortised over 16 targets. Frozen rule: PROTOCOL.md. Verdict INCONCLUSIVE.</text>')
o.append('</svg>')
open('figures/ratio-by-cell.svg','w').write("\n".join(o))
