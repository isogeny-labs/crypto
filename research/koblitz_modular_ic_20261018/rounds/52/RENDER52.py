"""Evidence-linked figures and PDF from terminal native receipts only."""
from pathlib import Path
from collections import Counter
import json,re,html
import matplotlib
matplotlib.use('Agg')
matplotlib.rcParams['svg.hashsalt']='koblitz-r52-fixed'
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text());c=r['construction_counts']
assert c['fresh_verified_packet_fibers']==3810 and c['audited_unique_theorems']==9
lengths=[int(x) for x in re.findall(r'CLOSURE52_LENGTH step=\d+ length=(\d+)',(h/'runs/closure52-02/native.log').read_text())]
assert lengths==[376,220,200,180]
fig,ax=plt.subplots(figsize=(8,2.5),layout='constrained');ax.plot(range(len(lengths)),lengths,'o-',color='#246350',linewidth=2)
ax.set(xlabel='Complete nu-adjunction extensions',ylabel='Remaining index length',xticks=list(range(len(lengths))),ylim=(140,420))
for i,n in enumerate(lengths):ax.annotate(str(n),(i,n),xytext=(0,10),textcoords='offset points',ha='center')
ax.grid(axis='y',alpha=.2);fig.savefig(h/'closure52.svg',metadata={'Date':None});plt.close(fig)
svgpath=h/'closure52.svg';svgpath.write_text('\n'.join(line.rstrip() for line in svgpath.read_text().splitlines())+'\n')
field_rows=[];plot_rows=[]
for attempt in ['fibers23-52-02','largefibers23-52-01']:
 counts=Counter()
 for m,pattern in re.findall(r'_ROW m=(\d+) .*? pattern=(\d+)',(h/'runs'/attempt/'native.log').read_text()):counts[int(m),int(pattern)]+=1
 for m in sorted({m for m,p in counts}):
  total=sum(n for (mm,p),n in counts.items() if mm==m)
  field_rows += [str(m),str(total),str(counts[m,88]),str(counts[m,214])]
  plot_rows.append({'field_degree':m,'good_fibers':total,'pattern88':counts[m,88],'pattern214':counts[m,214]})
(h/'plot-data52.json').write_text(json.dumps({'closure_lengths':lengths,'field_rows':plot_rows},indent=2)+'\n')
def t(s):return 'text('+json.dumps(s)+')'
run_rows=[]
for row in r['runs']:
 if row['kind']=='Lean':detail=f"compile {row['compile_exit']}; audit {row['audit_exit'] if row['audit_exit'] is not None else 'not run'}"
 else:
  detail='exit '+str(row['native_exit'])
  if row['error_count']:detail+='; native errors'
  elif not row['marker_present']:detail+='; marker absent'
 run_rows += [row['attempt'],row['status'],detail]
flow='''<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="245" viewBox="0 0 1100 245"><rect width="1100" height="245" fill="white"/><g font-family="DejaVu Sans,sans-serif" fill="#18364a"><text x="15" y="29" font-size="24">Three constructive outputs and their next checks</text>'''
for x,title,lines,color in [(15,'Normal-chart closure',['32 source-bound columns','Least denominator z^5(z+1)^9','Next: two completed local rings'],'#d7eee3'),(380,'Explicit image line',['Full degree 3 norm identity','All three sign products','Next: Tate / QR / F2 descent'],'#d7eee3'),(745,'Larger marked-23 source',['3,810 fresh factor certificates','Two-pattern degree 112 field','Next: normalization / 991 action'],'#d7eee3')]:
 flow+=f'<rect x="{x}" y="50" width="340" height="167" rx="8" fill="{color}"/><text x="{x+12}" y="81" font-size="21">{html.escape(title)}</text>'
 for i,line in enumerate(lines):flow+=f'<text x="{x+12}" y="{116+30*i}" font-size="18">{html.escape(line)}</text>'
flow+='</g></svg>'; (h/'leads52.svg').write_text(flow+'\n')
source='''#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= R52: exact coordinate closure and larger source field
The complete nu adjunction stabilizes at index length 180,with least recovery
denominator z^5(z+1)^9. An explicit degree3 line norm supplies generic packet
transport. Complete good fibers certify a degree112 larger source field by
monic specialization; all3,810stored factorizations are freshly checked.
#image("leads52.svg", width: 100%)

== Complete coordinate algebra
Starting from S=F2[z][V,lambda] inside the retained rank32 normal-chart B,
all32actual nu-multiplication columns and their full source products pass.
Both matrix products at every adjunction step and final stability pass.
#image("closure52.svg", width: 100%)
The exact determinant is z^60(z+1)^120. The full32x32 recovery product uses
the least scalar z^5(z+1)^9; predecessor nonzero remainders prove minimality.
A fresh process reconstructs every closure and source-action product.
#table(columns: (1fr, 4fr, 1fr, 1fr), inset: 5pt,
 [Prime], [Nonzero elementary exponents], [Zeros], [Length],
 [z], [1x4, 2x6, 3x6, 4x4, 5x2], [10], [60],
 [z+1], [1x4, 3x4, 4x4, 6x4, 7x4, 9x4], [8], [120])
All14local levels retain both products. The complete types match the candidate
(7,11) semigroup model (30gaps,conductor60),but actual completed local-ring
identification still requires branch valuations and residue-field descent.
The four coordinates recover the normal source on z(z+1)!=0,under the retained
normal-RR bridge. Omitted fibers,infinity and proper polarization remain tasks.
#pagebreak()
= Explicit line norm and the larger marked-23 field
== Actual generic line transport
For f=y+lambda*x+nu and the degree3kernel {0,Q,-Q},the full polynomial product is
#raw("NormNumerator = alpha*Ynum*H^3 + beta*Xnum*H^4 + gamma*H^6",block:true)
Here H=x+t and alpha is the original line cubic evaluated at t. Consequently
Y=(beta/alpha)X+gamma/alpha on alpha!=0. Changed intercept is rejected. The
sign action preserves alpha,changes beta by A*alpha and gamma by b*alpha.
The fresh checker reconstructs the actual translated line functions and Velu
coordinates before checking the norm product and all3sign products.

Next certify alpha on the actual QR packet,normalize the marked point to
Tate form,check all3target QR equations and gamma/F2descent,including alternate
charts and exceptional branches. The intended11primary action needs the
actual integral/odd-adic correspondence module.

== Larger source: two reducible patterns suffice
The retained marked23 base field K has degree7 over F2(x). Its actual twisted
QR polynomial is monic degree16 and nu is linear with nonzero A²Delta. Both
fixed probes enumerate every coefficient-field element and every smooth
linear base root with registered denominators and A*Delta nonzero.
'''
source+='#table(columns:(1fr,1fr,1fr,1fr),inset:5pt,[Field degree],[Good fibers],[8+8],[2+14],'+','.join(t(x) for x in field_rows)+')\n'
source+='''The small range has 488 good fibers and no 2+14 pattern. Scaling gives 3,322 good
fibers, 462 patterns 8+8, 924 patterns 2+14 and 2,024 odd-degree factors over F2048.
Every 3,810 source specialization and each stored factor's irreducibility are
freshly checked. Full factors and source coordinates are retained.

At a checked smooth base place,the local ring is a DVR and G's coefficients
are integral. A monic global factor is integral there,so reduction preserves
its degree. The 8+8 fiber allows degrees 0, 8, 16; the 2+14 fiber allows 0, 2, 14, 16.
Only 0 or 16 survive, proving irreducibility and degree112overF2(x). A degree-dividing-10
base place rules out constant extension7. An odd-degree11packet place then
rules out a nontrivial constant field dividing16,so the full constants areF2.
The DVR/monic-specialization and place interpretations are external standard
arguments in CM.md;the polynomial implication is formalized separately.
Normalization of the112field and its selected991action are the next checks.
#pagebreak()
= Receipts, formal scope and continuation
'''
source+=f"Terminal attempts: {len(r['runs'])}; accepted: {r['status_counts']['accepted']}; rejected: {r['status_counts']['rejected']}. All failed attempts remain in raw custody.\n"
source+='#table(columns:(2fr,1fr,2fr),inset:3pt,[Attempt],[Status],[Native receipt],'+','.join(t(x) for x in run_rows)+')\n'
source+='''Nine declarations compile and pass the axiom audit with zero failing or
trusted declarations. Closure52 proves range/stability/least-closure and
unit-scalar implications plus the pattern intersection. FibreIrred52 proves
polynomial irreducibility from the degree restrictions and the7/10constant
divisor step. RectBridge52 covers the actual rectangular generator products. Actual native matrices,DVR specializations,source geometry and
selected-factor actions are external;PROOF_MAP.md gives the precise boundary.

Native runs:seed52,one Singular/FLINT thread,4GiBvirtual and1200s each.
Lean4.34.1/mathlibd13f23b,-j1,-M4096,1200s separately compile/audit,noVMcap.
A native exit 0 or a late success marker never overrides error lines or a missing
required marker. The first fiber loop was manually stopped after a documented
continue-semantics bug;the corrected complete range was run once. Other
rejections retain their source,logs and partial certificates.

Evidence:results.json,receipt-validation.json,CM.md,PROOF_MAP.md and the
per-stage frozen protocols. ARTIFACTS.md records immutable byte custody and
fresh download/repack;that is separate from the native fresh checker and
external independent semantic review. Standard sources and exact references
are recorded in CM.md. Existing sealed R27-R51records are unchanged.

Continue with actual cusp valuations/completed subrings,normalized QR/Hecke
transport,112field normalization and991action. Preserve quotient saturation,
proper polarized embeddings,node187coupled divisors,132section RR intersection
and arbitrary reverse collectors,with Klein n3/samefield n5controls.
'''
# Polish prose without touching Typst command syntax or recorded identifiers.
def prose52(line):
    line=re.sub(r'([,;:])(?=[A-Za-z])',r'\1 ',line)
    line=re.sub(r'\b(all|All|full|Full|seed|rank|degree|degrees|conductor|extension|selected|intended|with|and|has|no|Only|only|the|or|dividing)(?=\d)',r'\1 ',line)
    line=re.sub(r'(?<=\d)(?=[A-Za-z]{2,})',' ',line)
    line=line.replace('overF','over F').replace('areF2','are F2').replace('GiBvirtual','GiB virtual').replace('noVMcap','no VM cap').replace('1200s','1200 s')
    return line
source='\n'.join(line if line.lstrip().startswith('#') else prose52(line) for line in source.splitlines())+'\n'
(h/'report.typ').write_text(source)
typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print(json.dumps({'pages_expected':3,'attempts':len(r['runs']),'plot_data':plot_rows,'status_counts':r['status_counts']}))
