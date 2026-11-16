import json, math
rows = [json.loads(l) for l in open("raw/split_law.jsonl")]
W, H, L, B = 640, 400, 70, 50
ds = [1,2,3,4,5,8,9,10,11]
cols = {101: "#1f77b4", 1009: "#d62728", 10007: "#2ca02c"}
ymin, ymax = -16, 0.5
def X(d): return L + (d-1)/10*(W-L-20)
def Y(v): return 20 + (ymax-math.log10(v))/(ymax-ymin)*(H-20-B)
o = ['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" font-family="sans-serif" font-size="11">' % (W,H),
     '<rect width="100%" height="100%" fill="white"/>',
     '<text x="%d" y="14" font-size="12">L07: exact complete-splitting fraction C(p,d)/p^d vs degree d [pcc:unverified]</text>' % L]
for e in range(-16, 1, 4):
    y = Y(10.0**e); o.append('<line x1="%d" x2="%d" y1="%.1f" y2="%.1f" stroke="#ddd"/><text x="5" y="%.1f">1e%d</text>' % (L, W-20, y, y, y+4, e))
for d in ds:
    o.append('<text x="%.1f" y="%d">%d</text>' % (X(d)-4, H-B+16, d))
o.append('<text x="%d" y="%d">sieving degree d (d=g hyperelliptic; d=2g+1 Prym)</text>' % (L+120, H-8))
for i,(p,c) in enumerate(cols.items()):
    pts = [(X(r["d"]), Y(r["splitfrac_dec"])) for r in rows if r["kind"]=="exact" and r["p"]==p]
    o.append('<polyline fill="none" stroke="%s" stroke-width="1.5" points="%s"/>' % (c, " ".join("%.1f,%.1f"%q for q in pts)))
    for q in pts: o.append('<circle cx="%.1f" cy="%.1f" r="3" fill="%s"/>' % (q[0],q[1],c))
    o.append('<text x="%d" y="%d" fill="%s">p=%d</text>' % (W-90, 40+14*i, c, p))
o.append('</svg>')
open("plot.svg","w").write("\n".join(o))
