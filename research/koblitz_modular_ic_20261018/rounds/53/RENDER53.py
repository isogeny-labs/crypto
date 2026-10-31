"""Render only recorded native exponents and terminal receipts."""
from pathlib import Path
import json,html
import matplotlib
matplotlib.use('Agg')
matplotlib.rcParams['svg.hashsalt']='koblitz-r53-fixed'
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text())
assert all(any(x['attempt']==a and x['status']=='accepted' for x in r['runs']) for a in ['discrim53-02','alphaunit53-01','tate53-03'])
assert len(r['audited_unique_declarations'])==3
data={'source':'runs/discrim53-02/native.log','stage_labels':['M0','M1','M2','M3'],
      'factor_exponents':{'z':[82,60,60,60],'z+1':[164,120,120,120],
                          'fA = z^5+z^2+1':[24,8,4,0],'fC = z^5+z^3+z^2+z+1':[2,0,0,0]}}
(h/'plot-data53.json').write_text(json.dumps(data,indent=2)+'\n')
fig,axs=plt.subplots(1,2,figsize=(9,3),layout='constrained')
for name,values in data['factor_exponents'].items():
 ax=axs[1] if name.startswith('f') else axs[0]
 ax.plot(data['stage_labels'],values,'o-',label=name)
 for i,value in enumerate(values):ax.annotate(str(value),(i,value),xytext=(0,5),textcoords='offset points',ha='center',fontsize=8)
for ax in axs:
 ax.set(xlabel='Complete coordinate adjunction stage',ylabel='Determinant factor exponent')
 ax.grid(axis='y',alpha=.2);ax.legend(fontsize=7)
fig.savefig(h/'defect-support53.svg',metadata={'Date':None});plt.close(fig)
p=h/'defect-support53.svg';p.write_text('\n'.join(s.rstrip() for s in p.read_text().splitlines())+'\n')
flow='''<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="240" viewBox="0 0 1100 240"><rect width="1100" height="240" fill="white"/><g font-family="DejaVu Sans,sans-serif" fill="#15364a"><text x="15" y="28" font-size="24">Exact source geometry to the normalized correspondence</text>'''
boxes=[('Trace pairing',['32 generator traces; 1024 entries','All eight discriminant predictions match','Next: completed rings at z=0,1']),
       ('Actual image-line denominator',['Full witness (G,N,psi3,alpha) W = Delta^16','Nonzero quotient dimension 64','Generic registered chart']),
       ('Marked quotient normalization',['Four explicit unit products','Full curve change and corruption check','Next: target QR / F2 map / 11 action'])]
for x,(title,lines) in zip([15,380,745],boxes):
 flow+=f'<rect x="{x}" y="48" width="340" height="170" rx="8" fill="#d7eee3"/><text x="{x+12}" y="80" font-size="20">{html.escape(title)}</text>'
 for j,line in enumerate(lines):flow+=f'<text x="{x+12}" y="{115+31*j}" font-size="15">{html.escape(line)}</text>'
flow+='</g></svg>';(h/'construction53.svg').write_text(flow+'\n')
def t(s):return 'text('+json.dumps(s)+')'
rows=[]
for row in r['runs']:
 detail=('exit '+str(row['native_exit'])+'; '+str(row['error_count'])+' error lines') if row['kind']=='Singular' else 'compile '+str(row['compile_exit'])+'; audit '+str(row['audit_exit'])
 rows += [row['attempt'],row['status'],detail]
source='''#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= R53: exact defect supports and marked correspondence coordinates
== Complete source-bound trace geometry
All 32 generator traces and all 1,024 Gram entries agree with an independent
companion-power and quadratic-base trace calculation. Both full source and
discriminant-change products pass; changed entries are rejected. All eight
predictions match in discrim53-02, with
#raw("fA=z^5+z^2+1; fC=z^5+z^3+z^2+z+1",block:true)
#raw("disc(B)=z^76(z+1)^24 fC^12; Norm(Delta)=z^11(z+1)^22 fC",block:true)
#image("defect-support53.svg",width:100%)
Measured complete determinant exponents, not a sample or a timing comparison.
Source: runs/discrim53-02/native.log. The last two adjunctions remove defect
only at fA. Final coordinate defect remains at z=0,1. Completed local-ring
identification needs branch valuations and residue-field descent.

== Actual denominator and marked model
The composed membership gives the full witness
#raw("(G,N,psi3,alpha)*(Vx,Vy,U,Vz0)^t = Delta^16",block:true)
Alpha is the actual R52 norm coefficient, and the G,N,psi3 quotient has
dimension 64. The complete witness is retained in raw custody. Generic
coefficient denominators and exceptional fibers remain explicit obligations.
#image("construction53.svg",width:100%)
#pagebreak()
= Validation and next construction
The generic marked Tate change uses h=A*r+b,
s=(r^2+a4+A*q)/h, D2=b+r+s^2+A*s and u=h/D2.
All t,h,D2,u units require full Bezout products. The complete curve change is
oldX=u^2*x+r and oldY=u^3*y+s*u^2*x+q, with
bnew=D2/u^2=h/u^3 and Anew=A/u. See CM.md for actual terminal acceptance.

== Complete terminal attempt ledger
'''
source+='#table(columns:(1.2fr,.65fr,2fr),inset:5pt,'+','.join('['+t(s)+']' for s in ['Attempt','Result','Native / Lean receipt']+rows)+')\n'
source+='''
Every failed implementation attempt is retained, including late markers after
errors. Exact native limits: seed53,one thread,4GiBvirtual,1200s. No accepted
producer rerun for publication. Lean uses4.34.1,pinned mathlibd13f23b,-j1,
-M4096,1200s separately for compile/audit,without a virtual-memory cap. Native wall times are recorded in results.json;
complete CPU times were not recorded. These are exact identity checks,not
statistical or performance comparisons.

The three Lean implications cover determinant change,composed alpha product
and the resulting unit. Actual arrays,trace semantics,source specialization
and geometric hypotheses are external inputs to those statements.

== Next decisive checks
Use the normalized line coordinates (L+s)/u and (L*r+N+q)/u^3. Verify all
target QR identities,the F2 descent and registered denominator specializations.
Then specialize the level11 base and certify intended subgroup action. The
two completed local rings,proper polarization,full quotient saturation and
arbitrary reverse-divisor evaluation retain their separate obligations.

Sources: R50 complete normal RR algebra;R51 line/collision/quotient functions;
R52 complete nu closure and norm line;pinned donors.json. Established
isogeny model conventions: Sage elliptic-curve isogeny documentation.
Proper/function-field correspondence: Stacks Project tag0BXX. No novelty
claim is made for trace pairings,Bezout composition or Tate normalization.
'''
(h/'report.typ').write_text(source);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print(json.dumps({'pdf_bytes':(h/'report.pdf').stat().st_size,'terminal_attempts':len(r['runs'])}))
