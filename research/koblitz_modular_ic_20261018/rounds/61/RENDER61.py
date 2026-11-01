"""Render recorded dimensions, cohort rows and terminal receipts; no new algebra."""
from pathlib import Path
from collections import Counter
import csv,json,re
import matplotlib
matplotlib.use('Agg');matplotlib.rcParams['svg.hashsalt']='koblitz-r61-fixed'
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text())
rows=list(csv.DictReader((h/'runs/cohort61-01/cohort61.tsv').read_text().splitlines(),delimiter='|'));assert len(rows)==818
counts=Counter(int(x['degree']) for x in rows);excluded=sum(x['kernel_dimension']=='0' for x in rows);principal=len(rows)-excluded
fig,ax=plt.subplots(1,2,figsize=(10,3),layout='constrained')
labels=['Cubic ambient','Mixed space','Fixed','Trace image'];vals=[324,76,38,38]
bars=ax[0].bar(labels,vals,color=['#76869b','#2e827a','#3f659a','#786299']);ax[0].set(ylabel='Binary dimension',ylim=(0,355),title='Certified cubic restriction and involution');ax[0].tick_params(axis='x',labelrotation=12)
for b in bars:ax[0].annotate(str(int(b.get_height())),(b.get_x()+b.get_width()/2,b.get_height()),ha='center',xytext=(0,3),textcoords='offset points')
keys=sorted(counts);ax[1].bar(keys,[counts[k] for k in keys],width=1.4,color='#377962');ax[1].set(xlabel='Conditional map degree',ylabel='Prescribed divisor patterns',title=f'{excluded} zero kernels; {principal} principal patterns')
for a in ax:a.grid(axis='y',alpha=.15)
fig.savefig(h/'cubic61.svg',metadata={'Date':None});plt.close(fig)
p=h/'cubic61.svg';p.write_text('\n'.join(x.rstrip() for x in p.read_text().splitlines())+'\n')
(h/'plot-data61.json').write_text(json.dumps({'sources':['runs/cubic61-02/native.log','runs/fix61-02/native.log','runs/cohort61-01/cohort61.tsv'],'dimensions':dict(zip(labels,vals)),'degree_counts':dict(sorted(counts.items())),'degree_counts_kind':'histogram of818 recorded rows','excluded':excluded,'principal':principal,'uncertainty':'exact prescribed matrices; no statistical sampling'},indent=2)+'\n')
attempts='\n'.join('['+x['attempt']+'],['+x['status']+'],['+(str(int(x['native_wall_seconds']))+' s' if 'native_wall_seconds' in x else 'not recorded')+'],' for x in r['runs'])
source='''#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= R61: complete cubic space and818 support certificates
The actual cubic ambient space has dimension324. All248 local constraints
leave the complete mixed space of dimension76. Its fixed space and trace
image both have dimension38. All38 actual fixed functions,38 trace
preimages and a trace-one function satisfy whole-source identities.

#image("cubic61.svg",width:100%)
Recorded dimensions: cubic61-02 and fix61-02. The histogram aggregates
all818 rows of cohort61-01. EXCLUDED patterns have zero kernel;
PRINCIPAL have one-dimensional principal-function spaces. The fresh checker
reconstructs every matrix from ambient profiles before fixed projection.

== Complete cubic construction and source binding
The324 descended cubic products are checked on all20 normalized charts:
6480 pole bounds and1360 old-function coefficient conjugations. All385
F4 regular jets give770 binary rows and an explicit left inverse. The
ambient degree384 and genus61 Riemann-Roch bridge imply completeness.
All248 binary conditions have rank248; full C K=0, R K=I76 and
K R+H C=I324 certify the degree136 mixed space, with caps34,12,20,2
at four degree2 infinity points. All324 cubic and76 mixed functions are
bound in the actual degree32 function field, including all76 involutions.

With N=sigma+I, the actual fixed/trace complex has an explicit Jordan
basis, both inverse products and a contraction Q satisfying NQ+QN=I76,
Q squared=0. All fixed local and regular profile imaginary components
vanish. Both trace parities remain available before the principal-x gate.

== Complete prescribed support cohort
Every triple in the intervals [-6,6],[-10,10],[-1,1] fixes the first
signed ramification by balancing; only the all-zero triple is erased.
All818 cases impose136 binary coefficient rows on the complete38-space.
Each zero kernel has a saved H C=I38. Any one-dimensional kernel has
C K=0, R K=1 and K R+H C=I38, with its generator retained. The global
regular positive control has rank37 and kernel1 containing actual1.
Map degrees are at most34, including degrees divisible by11.

The checker reconstructs constraints in the324-dimensional ambient
profiles, then applies the mixed and fixed inclusions. It also checks the
ambient injection, full mixed/fixed splitting, contraction, odd particular,
regular control and a changed-row negative control. Proper-source,
normalization, divisor and Riemann-Roch identifications remain explicit
geometric premises; the certified cohort is the stated pole support.

#pagebreak()
= Execution, formal implications and the next geometry
#table(columns:(2fr,1fr,1fr),[Attempt],[Outcome],[Native wall],
ATTEMPTS
)
Native wall is timestamp-derived and covers only that invocation; it is
not a performance comparison. Each raw directory preserves its exact
source revision, copied source/protocol, native binary hash and output.
Seed61; one native job/thread;1200seconds and4GiB virtual per invocation.
Lean4.34.1/mathlib d13f23b723b8a846827a245b89c10fc7d3f11612,
-j1 -M4096, separate1200second compile/audit limits and no virtual cap.

The first cubic attempt reversed the retained section/quadratic pair
indices, produced native errors and was stopped. Its exit0 does not make
its output acceptable. The repair was frozen before fresh02. The first
fixed-space launch was interrupted at the checkpoint gate during migration,
before native start; all saved inputs are retained. Fresh02 used unchanged
mathematical source. The first checker was manually stopped after an
avoidable per-cell interpreter loop was identified; it has no accepted
result and is not labelled a timeout. Checker02 then failed on unsupported
module(int) zero initialization; its late PASS and numbers are rejected.
Fresh03 removes that unnecessary reset, uses native row-vector copies,
checks all272 row bindings and retains all818 pre-projection products.
No accepted producer was repeated for publication.
Historical auxiliary protocol metadata defects and prospective repairs are
disclosed in PROTOCOL-METADATA61.md; raw versions remain immutable.

== Six audited Lean implications
The six Scaling61 declarations prove arbitrary-cap balanced first and
conditional degree bounds, a symbolic product-minus-one cohort count,
the818 corollary, inverse squaring from x to power32=x, and obstruction
to surjectivity of C-I from a nonzero invariant functional. The source
geometry and Cartier interpretation are external premises, separate from
these generic kernel-checked implications. Compile/audit receipts identify
all six declarations and their axiom status.

== Trace dual and new construction paths
#image("square-plan61.svg",width:100%)
All528 normal-product columns and1024 ordered trace products give a
rank32 trace pairing. Both inverse products and32 actual source-coordinate
dual columns are certified, with a changed-Gram negative control. This is
the affine trace-dual lattice; global regular differentials require the
complete infinity conditions described in DIFFERENTIAL-DESIGN61.md.

The next differential construction uses a certified normal-basis pole
bound to obtain a complete finite coefficient span, imposes every infinity
regularity condition, then checks the full61-dimensional holomorphic space
and source-bound Cartier/sigma operators. CARTIER-CONTROL61.md records the
Artin-Schreier quotient's nonzero joint fixed-space control and convention
checks. CUSPIDAL-UNIT-LEAD61.md gives a second path through exact generated
unit lattices and characteristic2 saturation. Neither proposed path is
reported as an executed differential or saturation result here.
SQUARE-DECOMPOSITION61.md proposes one certified32x32 square-basis inverse
for both paths, with exact rational coefficient splitting and source checks.

All original integral level77 factor/action, proper polarized embedding,
separate full quotient saturation, coupled divisors and arbitrary reverse
classes remain on the worklist. Acceptance of a surviving transfer also
requires function-field identities, coefficient descent and nonzero11-action.
'''
source=source.replace('EXCLUDED',str(excluded)).replace('PRINCIPAL',str(principal)).replace('ATTEMPTS',attempts)
(h/'report.typ').write_text(source);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'),root=str(h))
print('Rendered report.pdf from terminal receipts and818 rows')
