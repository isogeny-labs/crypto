"""Render the checked two-place infinity valuation profile."""
from pathlib import Path
import json
import typst

h=Path(__file__).resolve().parent
r=json.loads((h/'results.json').read_text())
assert [x['status'] for x in r['runs']]==['accepted','accepted']
assert 'norm_B=t5' in r['runs'][0]['result_line']
assert r['runs'][1]['theorems']==3 and r['runs'][1]['failing']==r['runs'][1]['trusted']==0
rows=[{'place':'U=0','v_B':0,'v_B_plus_t3':0,'v_b':-7,'v_c':-4,'weight':16},
      {'place':'U=1','v_B':5,'v_B_plus_t3':3,'v_b':1,'v_c':-1,'weight':16}]
assert sum(-x['v_b']*x['weight'] for x in rows if x['v_b']<0)==112
assert sum(-x['v_c']*x['weight'] for x in rows if x['v_c']<0)==80
assert sum(x['v_b']*x['weight'] for x in rows if x['v_b']>0)==16
(h/'plot-data77.json').write_text(json.dumps(rows,indent=2)+'\n')
bars=[]
for i,row in enumerate(rows):
    y=95+i*92
    bars.append(f'<text x="20" y="{y+20}" font-family="sans-serif" font-size="20" fill="#17384c">{row["place"]}</text>')
    for j,(name,color) in enumerate([('b','#205984'),('c','#b87337')]):
        val=row['v_'+name];yy=y+j*30
        x=344+min(val,0)*36;w=abs(val)*36
        bars.append(f'<rect x="{x}" y="{yy}" width="{w}" height="21" fill="{color}"/>')
        bars.append(f'<text x="470" y="{yy+17}" font-family="sans-serif" font-size="17" fill="#17384c">{name}: {val:+d}</text>')
svg='<svg xmlns="http://www.w3.org/2000/svg" width="740" height="340" viewBox="0 0 740 340">'+\
    '<rect width="740" height="340" fill="white"/><text x="20" y="33" font-family="sans-serif" font-size="21" fill="#17384c">Quadratic infinity orders; negative means pole</text>'+\
    '<line x1="344" y1="62" x2="344" y2="274" stroke="#657582" stroke-width="2"/>'+''.join(bars)+\
    '<text x="20" y="316" font-family="sans-serif" font-size="15" fill="#435569">Each quadratic place contributes weighted degree 16 in the full source.</text></svg>'
(h/'infinity77.svg').write_text(svg+'\n')
report='''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R77: the missing infinity zero of b
The quadratic source equation becomes $U^2+U=t+t^3$ at $t=1/z$, with two etale infinity places $U=0,1$. Exact native quotient checks give $N(B)=t^5$ and $N(B+t^3)=t^3(1+t)^3$ for $B=1+(1+t)U$. Three pinned Lean polynomial and degree theorems compiled and passed an axiom audit with zero failing and zero trusted.

#image("infinity77.svg", width: 100%)

At $U=0$, both factors are units; the norms force orders five and three at $U=1$. Since $b=t^-7 B(B+t^3)$ and $c=t^-4(B+t^3)$, the proper pole degrees in the degree-32 source are 112 for $b$ and 80 for $c$. The second infinity place also contributes a zero divisor of degree 16 for $b$, correcting R76's inference from finite zeros alone. Individual degree-32 infinity branches remain to be split.

One native and one Lean run passed. Eleven historical source-byte pins and five donor hashes match. The additive R76 erratum preserves its sealed original evidence.
'''
(h/'report.typ').write_text(report)
typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print(json.dumps({'quadratic_infinity_places':2,'b_poles':112,'b_infinity_zeros':16,'c_poles':80}))
