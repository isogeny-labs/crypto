"""Render only recorded exact counts; no mathematical experiment."""
from pathlib import Path
import csv,json
import matplotlib
matplotlib.use('Agg');matplotlib.rcParams['svg.hashsalt']='koblitz-r59-fixed'
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent
r=json.loads((h/'results.json').read_text())
rows=list(csv.DictReader((h/'runs/cohort59-01/cohort59.tsv').open(),delimiter='\t'))
assert len(rows)==18 and all(x['rank']=='100' and x['kernel_dimension']=='0' for x in rows)
fig,ax=plt.subplots(1,2,figsize=(10,3),layout='constrained')
ax[0].plot(range(5),[320,240,160,80,0],'o-',color='#246749');ax[0].set(xticks=range(5),xlabel='Power k of A on B/A^4',ylabel='Binary rank',title='Full normal quotient filtration');ax[0].grid(alpha=.2)
ax[1].bar(range(1,19),[int(x['rank']) for x in rows],color=['#407fab']*12+['#856a9c']*6);ax[1].axhline(99,color='#b86128',linestyle='--',label='regular control: rank99');ax[1].set(xlabel='Prescribed finite-support pattern',ylabel='Constraint rank / 100',ylim=(94,101),title='All18 complete kernels are zero',xticks=[1,6,12,13,18]);ax[1].legend(fontsize=8);ax[1].grid(axis='y',alpha=.15)
fig.savefig(h/'normal-jets59.svg',metadata={'Date':None});plt.close(fig)
p=h/'normal-jets59.svg';p.write_text('\n'.join(s.rstrip() for s in p.read_text().splitlines())+'\n')
(h/'plot-data59.json').write_text(json.dumps({'source':'runs/cohort59-01/native.log and cohort59.tsv; checked by checkcohort59-01','quotient_A_power_ranks':[320,240,160,80,0],'patterns':rows,'regular_control_rank':99,'units':'exact binary dimensions; exhaustive fixed matrices; no statistical uncertainty'},indent=2)+'\n')
attempts='\n'.join('['+x['attempt']+'],['+x['kind']+'],['+x['status']+'],' for x in r['runs'])
source="""#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= R59: complete normal jets and finite-support exclusions
All18 prescribed F2-stable ordinary O/T fiber patterns have zero solution
spaces on the actual complete100-section source system. Explicit left
inverses certify every exclusion and persist after extension to F32.
A separate exact certificate invocation checks every matrix product using
row-dot-column parity rather than the producer's column-XOR method.

#image("normal-jets59.svg",width:100%)
Exact recorded counts from cohort59-01, replayed by checkcohort59-01.
Patterns1--12 are ordered degree20 component pairs;13--18 are degree40
complementary partitions. The dashed rank99 control has a known nonzero
kernel. No statistical or performance comparison is represented.

== Complete source-bound quotient
The actual normal algebra B is free of rank32 over F2[z]. Its commuting
annihilator satisfies A C=(z^5+z^2+1)I. Hence f^4 B is contained in
A^4 B. The presentation modulo f^4 has640 binary coordinates; its complete
cokernel is320-dimensional. Explicit products certify
#raw("E M4=0, E G=I320, M4 H+G E=I640",block:true)
Four source-polynomial-labelled CRT blocks each have dimension80. Their
complete Jordan models have length4. Successive A ranks are320,240,160,80,0.
The source operator nu/b and old reduced model agree on all160 retained
columns. All100 R58 numerator functions reconstruct in the full normal
source field, with3,200 coefficient denominator-unit checks.

== Complete prescribed cohort
For x=f/A squared, impose numerator order at least4 at T and at least2
outside O union T. All12 ordered one-component pairs and6 complementary
partitions are retained. Every constraint matrix C has rank100 and an exact
H C=I100. Thus all18 principal-function spaces vanish over F2 and F32.
No31-scalar Artin--Schreier branch remains within this finite cohort.
The regular-function positive control has rank99 and contains the actual
A squared. A changed cokernel projection is rejected by the checker.

The interpretation as normal fourth-order local jets uses the retained
proper-source and reduced-support/DVR premises from R55/R56. It does not
prove saturation of the separate incidence or projective ideal. Nonstable
F32 supports and other polar divisors remain separate construction spaces.

#pagebreak()
= Failures, formal implications and the next construction
#table(columns:(1.6fr,1.8fr,1fr),[Attempt],[Workflow],[Outcome],
ATTEMPTS
)
Five workflows:4 accepted,1 rejected. data59-01 omitted linalg.lib, so
unitmat was unavailable. Cascading errors and a late PASS are retained and
excluded. data59-02 repairs that import before its fresh invocation. All31
executed-source Git pins match their historical committed bytes.

== Five audited generic Lean implications
Jets59.fourth_annihilator transfers the commuting annihilator to fourth
powers. fourth_idempotent and fourth_orthogonal justify the thickened CRT
formulas under native nilpotence premises. fourth_nilpotent_model transfers
the invertible length4 model to A. left_inverse_basechange proves that each
native H C=I remains an identity after coefficient extension, including F32.

Lean4.34.1 with mathlib d13f23b723b8a846827a245b89c10fc7d3f11612,
-j1 -M4096, separate1200second compilation/audit caps. Both exit0. Five
authored declarations are audited; zero failing, zero trusted native_decide.
Actual arrays, function-field bindings and source geometry remain explicit
native premises. See PROOF_MAP.md and runs/jets-lean59-01/audit.log.

== Next: infinity-supported ordinary maps
The four actual degree2 infinity points have original pole caps56,8,14,2.
Balanced signed ramification permits404 nonzero patterns. It forces both
x and t into the tighter common bound24,8,14,2, of degree96. First construct
that complete mixed space and its actual involution. Test BOTH trace
parities for t, with a genuine mixed-space particular function when trace1
is feasible. The finite-pole lambda/A is not a valid substitute here.

For every principal x pattern, test all31 F32 scalars, both ordinary signs
and both feasible parities with one exact AS operator. Final candidates
require whole-source identities, all5 coefficient conjugates, descent and
intended order11 action. Degree22 is possible in this cohort and must not
be removed by the earlier finite-support degree-unit filter.

Preserve wider F32 supports, direct500-bit relations, L(3P), integral
level77 factor geometry/action, proper polarization, separate saturation,
node187 coupled divisors and arbitrary reverse classes including460/367.
All original native caps and existing managed owners persist. No accepted
producer is repeated for publication. NEXT.md records the full next scope.

== Sources and reproducibility
CM.md, PROTOCOL.md, RUST-PROTOCOL.md, LEAN-PROTOCOL.md, results.json,
export.json and SESSION.md bind source functions, attempted workflows,
controls and exact acceptance. Donors retain R52 normal tables, R55
annihilator/CRT, R58 complete spaces and source maps. Singular official
source documentation was checked for helper/library behavior. No novelty,
performance or independent scientific-review claim is made.
""".replace('ATTEMPTS',attempts)
(h/'report.typ').write_text(source);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print(json.dumps({'pages_source':'report.typ','graph':'normal-jets59.svg','attempts':len(r['runs'])}))
