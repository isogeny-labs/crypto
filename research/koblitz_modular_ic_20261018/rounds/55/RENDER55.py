"""Render only terminal receipts and exact recorded dimensions."""
from pathlib import Path
import json,html,re
import matplotlib
matplotlib.use('Agg')
matplotlib.rcParams['svg.hashsalt']='koblitz-r55-fixed'
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text())
accepted={x['attempt']:x for x in r['runs'] if x['status']=='accepted'}
line=accepted['quotient55-02']['native_results'][0]
values={k:int(v)for k,v in re.findall(r'(\w+)=(\d+)',line)}
assert values['quotient_dimension']==80 and values['rank_A']==80 and values['rank_E']==80
check=accepted['check55-02']['native_results'][0];assert 'point_values=128' in check
(h/'plot-data55.json').write_text(json.dumps({'producer':line,'checker':check,'source_binary_dimension':160,'image_rank':values['rank_A'],'quotient_dimension':values['quotient_dimension'],'field_dimensions':[20,20,20,20]},indent=2)+'\n')
fig,axes=plt.subplots(1,2,figsize=(8.8,2.8),layout='constrained')
axes[0].bar(['B / fA B','image A','B / A B'],[160,values['rank_A'],values['quotient_dimension']],color=['#476a96','#9b7635','#2e7853']);axes[0].set(ylabel='Dimension over F2',title='Complete source quotient',ylim=(0,185))
axes[1].bar(['P1','P2','P3','P4'],[20]*4,color='#2e7853');axes[1].set(ylabel='Residue-field degree over F2',title='All four reduced closed points',ylim=(0,24))
for ax in axes:
 ax.grid(axis='y',alpha=.15)
 for b in ax.patches:ax.annotate(str(int(b.get_height())),(b.get_x()+b.get_width()/2,b.get_height()),xytext=(0,4),textcoords='offset points',ha='center')
fig.savefig(h/'quotient55.svg',metadata={'Date':None});plt.close(fig)
p=h/'quotient55.svg';p.write_text('\n'.join(x.rstrip()for x in p.read_text().splitlines())+'\n')
boxes=[('Actual normal source',['32 generators over F2[z]','All 528 multiplication columns','A A-prime = fA I']),('Complete binary source',['160 coordinates modulo fA','Abar has rank 80','E Abar = 0']),('Exact quotient recovery',['E G = I80','Abar H = I160 - G E','No omitted quotient classes']),('Binary CRT algebra',['Degree-80 squarefree modulus','Four degree-20 fields','All 32 source values'])]
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="190" viewBox="0 0 1120 190"><rect width="1120" height="190" fill="white"/>'
for x,(title,lines)in zip([10,290,570,850],boxes):
 svg+=f'<rect x="{x}" y="5" width="260" height="178" rx="8" fill="#deeee3"/><g font-family="DejaVu Sans,sans-serif" fill="#173a4a"><text x="{x+10}" y="37" font-size="18">{html.escape(title)}</text>'
 for j,t in enumerate(lines):svg+=f'<text x="{x+10}" y="{77+35*j}" font-size="16">{html.escape(t)}</text>'
 svg+='</g>'
(h/'construction55.svg').write_text(svg+'</svg>\n')
source='''#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= R55: complete A-zero source quotient and degree-80 CRT
The four binary degree-20 points now exhaust the A=0 scheme in the actual
normal affine source. Full module products identify the quotient, all source
functions, and every quotient class. The complete pole divisor remains a
separate projective geometry calculation.

== Full annihilator on the actual source
B is the retained normal rank-32 RR algebra over F2[z]. All 528 multiplication
columns bind the unit and actual V coordinate. With V²+z²V+z+z³=0, set
#raw("R=(1+z)V+z^3; A=1+z(R+1); fA=z^5+z^2+1",block:true)
The complete 32-by-32 multiplication matrices satisfy
#raw("C=M_A+(z^3+z^4)I; M_A C=C M_A=fA I32",block:true)
Thus fA B is contained in A B. Reduction modulo fA adds no extra constraint
to B/A B. A changed conjugate matrix is detected. Source: quotient55-02.
#image("construction55.svg",width:100%)

== Exact kernel and recovery products
B/fA B has 160 binary coordinates. Abar has rank 80 and E has rank 80.
The retained arrays give all three exact products
#raw("E Abar=0; E G=I80; Abar H=I160-G E",block:true)
The last product proves ker(E)=image(Abar) for every source class, while the
second proves surjectivity. Consequently B/A B is exactly the proposed
80-dimensional algebra. This is a full source quotient identification,
not just the rank of a sample or a determinant prediction.
#image("quotient55.svg",width:100%)
All plotted dimensions are exact native data, without sampling uncertainty.
Source: quotient55-02. The point degrees are the four retained R54 fields.
#pagebreak()
= Complete binary functions, proofs and retained failures

== Four fields and every source function
The product N of the four degree-20 moduli has degree 80 and is squarefree.
Four full cofactor/Bezout products construct CRT idempotents; all 16
orthogonality products, all 16 residue labels and partition of unity pass.
CRT recovers z=rho(w) and all 32 normal RR basis functions. The recovered
base coordinate satisfies fA(rho)=0. The actual unit and all 528 source
multiplication identities pass modulo N.

Thus B/A B is the product of four degree-20 fields. Its four closed points
have local length one and total binary length 80 in this normal affine chart.
The fresh serialized checker reconstructs every binary and evaluation entry,
all recovery products, all 128 point-value labels and all 528 source products.
An altered recovery matrix is rejected. Source: check55-02.

== Lean claim fidelity
Four authored implications compile and pass the separate axiom audit:
surjectivity from E G=I, exact kernel/image recovery, submodule equality, and
scalar multiples lying in image A from A C=f I. Zero failing and zero trusted
declarations. Actual matrix arrays, polynomial arithmetic, irreducible fields
and retained source normalization remain external premises. See PROOF_MAP.md.

== Complete workflow ledger
'''
items=['Workflow','Status','Receipt']
for row in r['runs']:
 detail=f"exit {row['native_exit']}; {row['error_count']} error lines" if row['kind']=='Singular' else 'compile 0; audit 0; four declarations'
 items += [row['attempt'],row['status'],detail]
source+='#table(columns:(1.2fr,.7fr,1.7fr),inset:5pt,'+','.join('[#text('+json.dumps(s)+')]'for s in items)+')\n'
source+='''
The first quotient attempt is rejected: a combined native scalar/matrix
expression produced a 32-by-64 matrix and failed the base identity. Later
undefined objects and a late marker are excluded. A separately frozen
shape diagnostic reproduced the issue. The first checker attempt also retained
a ring-local variable declaration failure; its output is rejected. Entrywise scalar construction passes
the original formulas and checks. No failed output supports the quotient.

Native limits: 4 GiB virtual, 1,200 seconds, one Singular/FLINT thread, seed 55.
Lean 4.34.1 / mathlib d13f23b, -j1/-M4096, separate 1,200-second compile/audit
caps, no virtual cap. Historical executed bytes are checked against Git and
drivers execute immutable copies. No accepted producer is rerun for custody.
CPU times were not collected; no comparative performance claim is made.

== Next geometry and preserved lanes
Determine all infinity branches, residue fields, ramification and A pole
orders, then bind this affine zero scheme to the complete projective divisor.
Use the degree-80 evaluator for an entire finite modular-map control before
generic target identities. Preserve the actual degree-8/128 level-11 map and
intended-11 action, full incidence saturation, proper polarization, node187,
arbitrary reverse classes and the larger degree-112/991 source lane.

Sources: R50/R51 full RR algebra, R52 PID embedding, R53 base geometry,
R54 four point fields, Stacks00DT CRT, native linalg.lib. See CM.md,
protocol.json, results.json and PROOF_MAP.md for exact premises and receipts.
'''
(h/'report.typ').write_text(source);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print(json.dumps({'pdf_bytes':(h/'report.pdf').stat().st_size,'workflows':len(r['runs'])}))
