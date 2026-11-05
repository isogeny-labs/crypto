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
assert all(any(x['attempt']==a and x['status']=='accepted' for x in r['runs']) for a in ['supersingular53-01','compactfiber53-01'])
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
 if 'interruption' in row:detail='interrupted; intermediate custody invalid'
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
= Full integral fiber versus the localized compact equations
At c=1 (A=0),the source equations give
#raw("Q=lambda^4+b*lambda+b^2; G=Q^4; N=b^3*Q^2",block:true)
The prediction N=0 failed. A fresh native check verifies both full ideal
products identifying (G,N) with (Q^2). This compact specialization leaves nu
free:its two-variable quotient has infinite dimension (vdim=-1).

The full three QR equations instead give a finite etale algebra of dimension
16,with a Jacobian-minor unit witness and complete ideal products. This holds
over F2(b) and at the actual F32 base point
#raw("a^5+a^2+1=0; V=(1+a^4+a)/(a+a^2); c=1; b=1+1/a",block:true)
The original base equation and b,c formulas are checked directly. The lambda
quartic factors with degrees 1,1,2 over F32,so it has four distinct geometric
roots,two of which are F32 rational.

== The actual twisted fiber
Ordinary nu is descended nu+b*zeta at A=0,where zeta^2+zeta+1=0.
Elimination and both full augmented-ideal products certify descent. The
descended quotient has dimension16,Jacobian unit,and leading monomials
lambda^4 and nu^4. Its standard monomials are lambda^i*nu^j for i,j = 0,1,2,3.
Thus nu separates four points over each geometric lambda root. The ordinary
and twisted nu equations differ;both are retained,without identifying them.

This identifies the missing integral coordinate data at the projection
fiber. It is a finite-fiber result;direct binding to the normal RR fiber,
the remaining completed local rings and proper correspondence are next.
Sources:supersingular53-01 and compactfiber53-01,including every full witness.

== Normalized map prefix and its cap
packet53-02 retains the composed inverse,the image line,normalized coordinates
and a native target-Delta unit check. The coordinate export is 78KiB;the
Delta unit lift was not separately serialized. Direct multivariate modular
powers reached the original 1200second cap(exit124) before the five target
equations completed. This is a computational cap,not a failed identity.
The next checker can use Frobenius squares and the retained QR/G,Nsyzygies,
or the actual degree8level11base to reduce coefficient growth.
#pagebreak()
= Validation and next construction
The generic marked Tate change uses h=A*r+b,
s=(r^2+a4+A*q)/h, D2=b+r+s^2+A*s and u=h/D2.
All t,h,D2,u units require full Bezout products. The complete curve change is
oldX=u^2*x+r and oldY=u^3*y+s*u^2*x+q, with
bnew=D2/u^2=h/u^3 and Anew=A/u. See CM.md for actual terminal acceptance.

== Complete terminal attempt ledger
'''
source+='#table(columns:(1.2fr,.65fr,2fr),inset:5pt,'+','.join('[#'+t(s)+']' for s in ['Attempt','Result','Native / Lean receipt']+rows)+')\n'
source+='''
Every failed implementation attempt is retained, including late markers after
errors. Exact native limits: seed53,one thread,4GiBvirtual,1200s. No accepted
producer rerun for publication. Lean uses4.34.1,pinned mathlibd13f23b,-j1,
-M4096,1200s separately for compile/audit,without a virtual-memory cap. Native wall times are recorded in results.json;
complete CPU times were not recorded. These are exact identity checks,not
statistical or performance comparisons.

packet53-01 has invalid intermediate custody:a live Bash-runner edit caused
interpreter re-entry into a second native invocation. The owned processes
were stopped,all retained files are rejected,and this exception is excluded
from valid historical executed-byte pins. A fresh workflow requires completed
Lean receipts before admission. All12workflows are terminal,6accepted and
6rejected;24valid completed-workflow input pins are verified separately.

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
for old,new in {
 'dimension16':'dimension 16','dimension64':'dimension 64','All12':'All 12',
 '6accepted':'6 accepted','6rejected':'6 rejected','24valid':'24 valid',
 '1200second':'1200 second','78KiB':'78 KiB','seed53,one':'seed 53, one',
 'thread,4GiBvirtual,1200s':'thread, 4 GiB virtual, 1200 s',
 'uses4.34.1,pinned':'uses 4.34.1, pinned','mathlibd13f23b':'mathlib d13f23b',
 'for i,j':'for i, j','4x4basis':'4 by 4 basis','the5':'the five',
 'the3':'the three','all3':'all three','all5':'all five',
 'F2(b)':'F2(b)','actualF32':'actual F32','degree8level11base':'degree-8 level-11 base',
 'degree8level11':'degree-8 level-11','R50complete':'R50 complete',
 'R51line':'R51 line','R52complete':'R52 complete','QR/G,Nsyzygies':'QR/G,N syzygies'
}.items():source=source.replace(old,new)
# Plain prose punctuation; protect Typst commands and raw formulas.
source='\n'.join(line if line.startswith('#') else line.replace(',', ', ').replace(';', '; ').replace(':', ': ').replace(',  ', ', ').replace(';  ', '; ').replace(':  ', ': ') for line in source.splitlines())+'\n'
(h/'report.typ').write_text(source);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print(json.dumps({'pdf_bytes':(h/'report.pdf').stat().st_size,'terminal_attempts':len(r['runs'])}))
