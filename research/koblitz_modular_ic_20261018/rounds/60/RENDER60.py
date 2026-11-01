"""Render terminal exact records only, without mathematical computation."""
from pathlib import Path
from collections import Counter
import csv,json
import matplotlib
matplotlib.use('Agg');matplotlib.rcParams['svg.hashsalt']='koblitz-r60-fixed'
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text())
rows=list(csv.DictReader((h/'runs/cohort60-01/cohort60.tsv').open(),delimiter='\t'))
assert len(rows)==404 and all(x['rank']=='18' and x['kernel_dimension']=='0' for x in rows)
counts=Counter(int(x['degree']) for x in rows)
fig,ax=plt.subplots(1,2,figsize=(10,3),layout='constrained')
labels=['Original space','Mixed space','Fixed','Trace image'];vals=[100,36,18,18]
bars=ax[0].bar(labels,vals,color=['#738ca8','#387e75','#476b9a','#71619a']);ax[0].set(ylabel='Binary dimension',ylim=(0,112),title='Actual infinity restriction and involution');ax[0].tick_params(axis='x',labelrotation=14)
for b in bars:ax[0].annotate(str(int(b.get_height())),(b.get_x()+b.get_width()/2,b.get_height()),ha='center',xytext=(0,3),textcoords='offset points')
keys=sorted(counts);ax[1].bar(keys,[counts[x] for x in keys],width=1.4,color='#3f7d59');ax[1].set(xlabel='Conditional map degree',ylabel='Prescribed divisor patterns',xticks=[2,6,10,14,18,22,24],title='All404 patterns excluded; rank18 each')
for a in ax:a.grid(axis='y',alpha=.15)
fig.savefig(h/'infinity60.svg',metadata={'Date':None});plt.close(fig)
p=h/'infinity60.svg';p.write_text('\n'.join(x.rstrip() for x in p.read_text().splitlines())+'\n')
(h/'plot-data60.json').write_text(json.dumps({'sources':['mixed60-02/native.log','cohort60-01/cohort60.tsv','check60-01/native.log'],'dimensions':dict(zip(labels,vals)),'degree_counts':dict(sorted(counts.items())),'degree_counts_kind':'derived histogram of404 recorded rows','excluded':404,'principal':0,'regular_control_dimension':1,'uncertainty':'exact exhaustive matrices, no statistical intervals'},indent=2)+'\n')
attempts='\n'.join('['+x['attempt']+'],['+x['status']+'],['+(str(int(x['native_wall_seconds']))+' s' if x['kind']=='Singular' else 'not recorded')+'],' for x in r['runs'])
source='''#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= R60: complete infinity-pole space and404 exclusions
The actual complete mixed-pole space has dimension36, with18 fixed
functions and trace rank18. A certified trace-one function exists, so both
even and odd trace branches are feasible. Every one of the404 prescribed
infinity principal-divisor spaces has zero kernel. Each has an exact left
inverse, with all source rows reconstructed in a fresh certificate check.

#image("infinity60.svg",width:100%)
Counts from mixed60-02 and cohort60-01; certificates checked by check60-01.
The histogram aggregates404 recorded degree rows. Exact dimensions and
exhaustive patterns; no statistical or timing comparison is represented.

== Complete pole restriction
Four actual degree2 source points R07,R01,R17,R11 carry original caps
56,8,14,2 in the complete100-dimensional undivided R58 space. For ordinary
maps x,t=y/x in that space, signed ramification e_i balances to zero and
has minor caps4,7,1. Hence absolute e0 is at most12. Both x and t obey
sharper caps24,8,14,2. The first point's orders -56 through -25 give64
binary constraints. Their rank is64, leaving dimension36.

The evaluator uses actual descended quadratic products and all100 source
functions, with all four uniquely selected normalized charts, actual bounds
and precision checked. Full certificates establish C K=0, R K=I36 and
K R+H C=I100. The actual involution restricts and squares to I36.
Its N=sigma+I has fixed dimension18 and trace rank18, with complete fixed
kernel splitting. A retained particular t has N t=1. Fixed local profile
imaginary coefficients vanish in all four charts. The regular514-row
source evaluation has a checked left inverse.

== Exhaustive ordinary divisor patterns
All9 times15 times3 minor triples exceptzero give404 balanced patterns.
On the fixed18-space each case imposes96 binary coefficient rows. Every
matrix has rank18 and an explicit H C=I18. The global-regular positive
control has rank17 and kernel dimension1 containing the actual constant1.
Map degrees run up to24; degree22 cases were included without a coprimality
filter. No principal x survives, so there are no scalar or AS branches to
run for this cohort. Both trace parities were retained before that gate.

The proper genus61 source, complete R58 space and normalized/DVR chart
interpretations are explicit premises. This resolves the stated infinity
cohort with both functions in the original bound. Other polar divisors and
nonstable F32 finite-support fibers remain available construction spaces.

#pagebreak()
= Exact checks, formal bounds and the cubic enlargement
#table(columns:(2fr,1fr,1fr),[Attempt],[Outcome],[Native wall],
ATTEMPTS
)
Native wall is timestamp-derived, includes this invocation only and is not
a performance comparison. Host: migrated Spot workspace, Linux/x86_64;
actual source revisions, native binary hashes and UTC are retained per run.
Commands: REPLAY_CAS.sh with mixed60-01/02,cohort60-01,check60-01 and
the corresponding MIXED60/COHORT60/CHECK60 input and exact marker;
REPLAY_LEAN.sh with infinity-lean60-01/02/03. Seed60, one native job/thread,
4GiB virtual and1200seconds each; Lean -j1 -M4096, separate1200second
compile/audit and no VM cap. No warmups or statistical sampling.

The first native attempt used an unsupported indexed intvec increment.
All four chart bodies failed; its late PASS was rejected and all reported
numbers excluded. The repair was frozen before mixed60-02. Two Lean attempts also failed:
first an absent Mathlib Omega module, then missing absolute-value notation
and recursive enumeration depth. Fresh repairs use core Omega, explicit
integer intervals and symbolic finite cardinality identities. No accepted
producer was repeated to publish or validate custody.

== Fresh certificate invocation
The checker reconstructs all404 constraint matrices as selected rows of
the100-function profiles followed by the fixed-basis product. The producer
selects rows after forming fixed profiles. All404 reconstructed matrices,
all left inverses, mixed and fixed splitting identities, actual odd
particular, regular injection and positive control pass. A changed source
row fails comparison. This is exact certificate validation, not an
independent scientific review or projective-saturation proof.

== Three Lean implications
Infinity60.balanced_first_bound proves the sharper absolute e0 bound.
balanced_degree_bound proves d=2 sum positive(e) is at most24.
nonzero_minor_patterns proves the full404 count by symbolic cardinalities.
Lean4.34.1/mathlib d13f23b723b8a846827a245b89c10fc7d3f11612,
-j1 -M4096, separate1200second compile/audit limits, no VM cap.
Three authored declarations compile and audit; zero failing or trusted
native_decide. Source geometry and ramification interpretation remain
external premises. See PROOF_MAP.md and the raw audit.

== Next complete construction
The cubic bound3D permits minor caps6,10,1 and818 balanced nonzero
patterns. It forces a common mixed bound34,12,20,2 of degree136. On the
retained genus61 source, RR predicts dimension76. First certify all324
actual descended cubic products as a complete degree384 ambient space,
using385 regular F4 jets and all20 source charts. Impose all248 binary
removed-pole coefficients; require full kernel splitting and actual source
function/involution bindings. Dimensions76 and818 are next-stage derived
predictions, not R60 measurements. NEXT.md gives the complete acceptance.

Preserve level77 factor/action, full quotient and incidence saturation,
proper polarization, node187 coupled divisors, arbitrary reverse classes
and ordinary460/367 fibers, direct relations and larger112/991 sources.
All original managed owners, native caps and authorizations persist.

== Delivery and sources
PR3062 was merged externally at head404f49801. ContinuationPR3200 uses the
original stacked base without merge or automatic-merge changes. R59 source
science is sealed; its fresh archive download checks all91 files and
repacking. R60 records, report and custody follow separately.
Sources: R27 normalized charts, R56 divisor geometry, R58 full100-space,
R59NEXT, official Singular helper documentation and protocol donor hashes.
Read CM.md, results.json, export.json, PROOF_MAP.md, SESSION.md and NEXT.md.
#pagebreak()
#include("report-cartier60.typ")
'''.replace('ATTEMPTS',attempts)
(h/'report.typ').write_text(source);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print('R60 report and exact recorded-count graph rendered')
