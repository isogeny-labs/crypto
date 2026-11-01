"""Evidence-linked figures and report; metadata/rendering only, no producer replay."""
from pathlib import Path
import json,re,html
import matplotlib
matplotlib.use('Agg')
matplotlib.rcParams['svg.hashsalt']='koblitz-r56-fixed'
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text())
accepted={x['attempt']:x for x in r['runs'] if x['status']=='accepted'}
assert set(accepted)=={'infinity56-02','infinity-lean56-01','refine56-01','unit56-01','check56-01'}
line=next(s for s in accepted['check56-01']['native_results'] if s.startswith('CHECK56_RESULT '))
assert 'A_poles=28,4,7,1' in line and 'pole_degree=80' in line
source=(h/'runs/check56-01/closed-poles56.sing').read_text()
rows=[[0]*6 for _ in range(4)]
for i,j,v in re.findall(r'points56\[(\d+),(\d+)\]=(-?\d+);',source):rows[int(i)-1][int(j)-1]=int(v)
assert [x[3]for x in rows]==[28,4,7,1] and all(x[2]==2 for x in rows)
# Arithmetic products below are derived degree contributions, labelled as such.
weighted=[x[2]*x[3]for x in rows]
newton={}
for branch in [0,1]:
 text=(h/f'runs/refine56-01/newton-b{branch}c0step156.sing').read_text()
 vals={int(i):int(v)for i,v in re.findall(r'int val(\d+)56=(\d+);',text)}
 newton[str(branch)]=[[i,vals[i]]for i in [0,1,8,16]]
data={'checker_result':line,'point_table':rows,'derived_pole_degrees':weighted,'newton_vertices':newton,'derivation':'pole degree contribution = recorded multiplicity times residue degree;Newton vertices copied from native records'}
(h/'plot-data56.json').write_text(json.dumps(data,indent=2)+'\n')
fig,axes=plt.subplots(1,3,figsize=(10,3.1),layout='constrained')
labels=['R07','R01','R17','R11']
axes[0].bar(labels,[x[3]for x in rows],color=['#315c89','#7197b8','#2c7756','#79ab91']);axes[0].set(title='Exact A pole orders',ylabel='Pole multiplicity',ylim=(0,33))
axes[1].bar(labels,weighted,color=['#315c89','#7197b8','#2c7756','#79ab91']);axes[1].set(title='Derived degree contributions',ylabel='Multiplicity × residue degree',ylim=(0,65))
for branch,color in [(0,'#315c89'),(1,'#2c7756')]:
 points=newton[str(branch)];axes[2].plot([x[0]for x in points],[x[1]for x in points],'o-',label=f'Q{branch}',color=color)
axes[2].set(title='Final lower Newton hulls',xlabel='Polynomial exponent',ylabel='h-adic coefficient order',xticks=[0,1,8,16]);axes[2].legend()
for ax in axes[:2]:
 for b in ax.patches:ax.annotate(str(int(b.get_height())),(b.get_x()+b.get_width()/2,b.get_height()),xytext=(0,4),textcoords='offset points',ha='center')
for ax in axes:ax.grid(axis='y',alpha=.15)
fig.savefig(h/'poles56.svg',metadata={'Date':None});plt.close(fig)
p=h/'poles56.svg';p.write_text('\n'.join(s.rstrip()for s in p.read_text().splitlines())+'\n')
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1040" height="310" viewBox="0 0 1040 310"><rect width="1040" height="310" fill="white"/>'
svg+='<g font-family="DejaVu Sans,sans-serif" fill="#18384a">'
for x,base in [(10,0),(535,1)]:
 svg+=f'<rect x="{x}" y="8" width="495" height="60" rx="8" fill="#deeee3"/><text x="{x+15}" y="34" font-size="20">Base Q{base}: residue F2, uniformizer h</text><text x="{x+15}" y="56" font-size="16">ord(A) = {-4 if base==0 else -1}</text>'
 for dx,e,order in [(0,7,28 if base==0 else 7),(250,1,4 if base==0 else 1)]:
  xx=x+dx
  svg+=f'<path d="M{x+247},68 L{xx+122},110" stroke="#6c8796" stroke-width="2"/><rect x="{xx}" y="112" width="240" height="180" rx="8" fill="#e7eef5"/><text x="{xx+12}" y="143" font-size="20">R{base}{e}: residue F4</text><text x="{xx+12}" y="179" font-size="17">e = {e}; residue degree = 2</text><text x="{xx+12}" y="214" font-size="17">A pole multiplicity = {order}</text><text x="{xx+12}" y="250" font-size="16">Uniformizer: {"alpha"+str(base)+"^2 / h^3" if e==7 else "h"}</text>'
svg+='</g></svg>'; (h/'infinity56.svg').write_text(svg+'\n')
source='''#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= R56: complete divisor, infinity points and rational uniformizers
The actual source now has a complete A=1+c principal divisor. Both infinity
fibers split into one ramified degree-7 and one unramified branch, each with
residue field F4. Explicit rational uniformizers bind the normalization to
source coordinates. The function lambda/A has certified simple poles at all
four finite A zeros, giving a concrete prescribed-pole RR construction.

== Exact base chart and pole orders
For the actual source T=lambda, put h=1/z, W=h²V and s=h⁴lambda. Full
rational chart identities transform the base to W²+W=h+h³ and bind
R=(1+z)V+z³, b=zR(R+1), c=z(R+1), A=1+c. The two simple residue roots
W=0,1 lift uniquely. Exact telescoping and 128-jets give:
#table(columns:6,inset:5pt,
[Base point],[R],[b],[c],[A],[Delta],
[Q0],[-3],[-7],[-4],[-4],[-37],
[Q1],[2],[1],[-1],[-1],[-1])
All entries are exact orders. Source: infinity56-02. In particular b has a
boundary zero at Q1, which must be included when evaluating b denominators.

== Complete source fibers at infinity
#image("infinity56.svg",width:100%)
Each fiber has degree 2×7+2×1=16. These are closed points over F2, with
residue degree 2. Both F4 conjugate residue charts are checked explicitly.

The first Newton faces were repeated eighth powers. One further source-bound
translation resolves each cluster: positive-root segments have lengths 1 and
7, with root orders 2 and 4/7 over Q0, 1 and 4/7 over Q1. The degree-7
segment has full reduced denominator 7; the separable complete-local-field
Newton theorem proves irreducibility and total ramification. The other segment
is linear. Original-polynomial reconstruction and tracked precision pass at
all stages: precision 120 over Q0, 72 over Q1. Sources: refine56-01/check56-01.
#pagebreak()
= Actual uniformizers and the complete principal divisor

== Source formulas for normalized local parameters
Define the following F2 rational coordinate functions:
#raw("alpha0=h^8*lambda^2+(h^4+h^5)*lambda+1+h+h^2\nalpha1=h^2*lambda^2+(h+h^2)*lambda+1+h^2",block:true)
In each ramified chart the exact source identity is
#raw("alpha_i=h*t*(1+h+h*t); ord(h)=7; ord(t)=4",block:true)
The last factor has residue 1. Hence alpha_i has order 11 and
sigma_i=alpha_i²/h³ has order 22-21=1. These are rational uniformizers at
R07 and R17. At R01 and R11, h is a uniformizer. The normalized completed
local rings are F4 power-series rings in those parameters. This does not
identify a smaller coordinate-image subring with a monomial ring.

== All zeros and poles
R55 gives four reduced degree-20 zeros P1,...,P4. Their local maximal ideals
are generated by A, so the Noetherian local domains are DVRs: normalization
adds no branch or multiplicity there. The complete divisor is
#raw("div(A)=P1+P2+P3+P4 -28*R07-4*R01-7*R17-R11",block:true)
Both zero and pole degrees are 80. Lambda has exactly the same pole divisor.
Unit56-01 checks the actual lambda and V values on the entire degree-80 zero
quotient, the border equation, and full Bezout products for lambda and b.
Both are units there; a changed inverse is detected. Therefore lambda/A has
exactly simple poles at the four Pi and no poles at infinity.
#image("poles56.svg",width:100%)
Multiplicities and Newton vertices are exact native records; degree bars are
labelled derived products with residue degree 2. No sampling uncertainty.

Multiplication by A identifies L(P1+...+P4) with L((A)_infinity). Next match
the retained complete 68-section ambient RR divisor to these actual points,
then impose all local constraints to construct the full prescribed space.
#pagebreak()
= Proof fidelity, failures and reproducible evidence

== Four audited Lean implications
All four authored Infinity56 declarations compile and pass separate axiom
audit, zero failing and zero trusted. They cover jet-order stability, exact
chart scaling, pole pullback, and the full-denominator ramification/residue
conclusion. Actual arrays, Hensel error bounds, local-field Newton facts and
normalization/DVR interpretation remain explicit external premises. See
PROOF_MAP.md. No sorry, native_decide or new axiom is used.

== Full terminal workflow ledger
'''
items=['Workflow','Status','Receipt']
for row in r['runs']:
 detail=f"exit {row['native_exit']}; {row['error_count']} error lines" if row['kind']=='Singular' else 'compile 0; audit 0; four declarations'
 items += [row['attempt'],row['status'],detail]
source+='#table(columns:(1.2fr,.7fr,1.8fr),inset:5pt,'+','.join('[#text('+json.dumps(s)+')]'for s in items)+')\n'
source+='''
Five workflows accepted, one rejected. The first infinity attempt failed an
indexed-list mapping after its base prefix; its late marker is excluded.
The corrected whole-list mapping passes. Integer-division and redefinition
warnings are non-error diagnostics. All 35 executed historical byte pins
match Git. The fresh local checker independently reconstructs every final
chart and checks all Newton secant inequalities, coordinate identities and
degree accounting, rather than relying only on the original hull algorithm.

== Limits, preservation and sources
Native: fixed seed 56, 4 GiB virtual, 1,200 seconds, one Singular/FLINT thread.
Lean 4.34.1 / mathlib d13f23b, -j1/-M4096, separate 1,200-second compile and
audit caps, no virtual cap. Producer hostname and CPU times were not separately
recorded; native UTC wall stamps are retained. No comparative speed claim.

All six terminal run directories survived the host transition. Only the
missing metadata index was regenerated. No accepted producer was repeated.
The original executed checkpoint helper is retained separately; a published
main merge changed relay packaging outside its gate function. No managed
Diem/freezer owner, research EKS job or new Conductor claim was started.

References checked: retained R50/R51 RR algebra, R52 source field, R53 base
coordinates, R55 quotient; Stacks04GM Hensel, Stacks00PD DVR criterion,
Stacks0C1B valuation/ramification; Garrett, Newton polygons, pages 1-2.
The source-specific geometry is derived from the certificates above.

== Next construction and preserved lanes
The retained source already has genus 61 and a complete 68-section RR space
of degree 128. Match its exact R27 cusp divisor to the new points; after
certifying inclusion, impose all degree-48 local constraints. Do not assume
that ambient divisor equals four times the z-pole divisor. Expand all source
basis functions and nu in the actual uniformizers, then test modular maps
with complete denominator, descent and intended order-11 action checks.
Preserve proper polarization, separate incidence saturation, node187 coupled
divisors, arbitrary reverse classes, and the larger degree-112/991 lane.
'''
(h/'report.typ').write_text(source);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print(json.dumps({'pdf_bytes':(h/'report.pdf').stat().st_size,'workflows':len(r['runs'])}))
