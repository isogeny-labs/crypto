# Layout only: plots recorded values from results/table-m*.tsv (no computation of results).
import csv, glob
from reportlab.graphics.shapes import Drawing, Rect, String, Line
from reportlab.graphics import renderSVG
from reportlab.lib import colors
rows=[]
for f in sorted(glob.glob('results/table-m*.tsv')):
    for r in csv.reader(open(f),delimiter='\t'):
        rows.append(dict(m=r[0],t=r[1],arm=r[2],s=float(r[3]),rho0=float(r[6]),rho1=float(r[8])))
groups=[]
for m in ('12','14'):
    for t in sorted({r['t'] for r in rows if r['m']==m}):
        g={r['arm']:r for r in rows if r['m']==m and r['t']==t}
        groups.append((m,t,g))
W,H=720,380; L,B,T=60,60,40
d=Drawing(W,H)
d.add(String(W/2,H-18,'S = GAE / sqrt(r): unmodified oracle vs frob_roots, same targets (counted, L0)',textAnchor='middle',fontSize=11))
ymax=max(g['ic-base']['s'] for _,_,g in groups)*1.1
# two panels share the axis only by value: m=12 values (~40) dwarf m=14 (~5); use a log-free split
def panel(sub,x0,w):
    ym=max(g['ic-base']['s'] for _,_,g in sub)*1.15
    d.add(Line(x0,B,x0,H-T)); d.add(Line(x0,B,x0+w,B))
    for k in range(0,5):
        v=ym*k/4; y=B+(H-T-B)*k/4
        d.add(Line(x0-3,y,x0,y)); d.add(String(x0-6,y-3,'%.1f'%v,textAnchor='end',fontSize=8))
    n=len(sub); bw=w/(n*3.2)
    for i,(m,t,g) in enumerate(sub):
        cx=x0+w*(i+0.5)/n
        for j,(arm,col) in enumerate((('ic-base',colors.HexColor('#6b7280')),('ic-lever',colors.HexColor('#2563eb')))):
            h=(H-T-B)*g[arm]['s']/ym
            d.add(Rect(cx+(j-1)*bw,B,bw,h,fillColor=col,strokeColor=None))
            d.add(String(cx+(j-0.5)*bw,B+h+3,'%.2f'%g[arm]['s'],textAnchor='middle',fontSize=8))
        for key,dash,lab in (('rho0',None,'rho w=0'),('rho1',[3,2],'rho w=1')):
            y=B+(H-T-B)*g['ic-base'][key]/ym
            ln=Line(cx-1.3*bw,y,cx+1.3*bw,y,strokeColor=colors.HexColor('#dc2626'))
            if dash: ln.strokeDashArray=dash
            d.add(ln)
        d.add(String(cx,B-14,'m=%s target %s'%(m,t),textAnchor='middle',fontSize=8))
        d.add(String(cx,B-26,'lever/base %.4f'%(g['ic-lever']['s']/g['ic-base']['s']),textAnchor='middle',fontSize=8))
panel([x for x in groups if x[0]=='12'],60,420)
panel([x for x in groups if x[0]=='14'],540,150)
d.add(Rect(70,H-38,9,9,fillColor=colors.HexColor('#6b7280'),strokeColor=None)); d.add(String(83,H-37,'ic-base (and ic-base-aa, identical)',fontSize=8))
d.add(Rect(260,H-38,9,9,fillColor=colors.HexColor('#2563eb'),strokeColor=None)); d.add(String(273,H-37,'ic-lever (frob_roots=1)',fontSize=8))
d.add(Line(400,H-34,420,H-34,strokeColor=colors.HexColor('#dc2626'))); d.add(String(423,H-37,'matched rho-neg S at 0 / 1 GAE per canonicalisation (solid / dashed)',fontSize=8))
renderSVG.drawToFile(d,'figures/s_lever.svg')
