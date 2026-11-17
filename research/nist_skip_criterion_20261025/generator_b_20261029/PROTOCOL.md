# Generator/coefficient comparison — prepared 29 October 2026

The user's objective remains a supported criterion explaining skipped curves,
with a comprehensive difference inventory and an explicit G–b investigation.
There are infinitely many functions of curve parameters; this protocol defines
the finite measured family instead of claiming all possible differences.
All labels and prior findings are visible. There is no blinded prediction or
historical holdout. No DLP or performance benchmark is requested or performed.

## Inputs and models

Retain every original row: 21 curve choices, ten published and eleven omitted,
including P-384 and P-521. Bind the complete existing 1,127-feature tables,
historical p,b,n,seed identities, four supplied points, and ten standardized
published generators. Unknown historical generators for all eleven omissions
remain textual NA, never zero, a numeric feature, or grounds for dropping rows.
Standardized generators come from pinned std-curves records, checked against
the exact curve tuple and by independent native group arithmetic before admission.

Use two explicit coefficient conventions: the historical table, and the same
table with P-224 b replaced by its smaller sign. These are two views of the
same 21 choices, not 42 historical samples. Other fields retain the original
exponentiation convention. For P-224 only, transport a published point under
b→−b with (x,y)→(−x,i*y), where i is the smaller square root of −1 mod p.
Retain the map's literal i and check all transported points and group orders.
Do not apply this same-order identification to p≡3 mod4 twists.

## Fixed comparisons

For each view, choose Q by the user's rule: first nonnegative x with a point,
then smaller y. Check the four supplied canonical points exactly. Evaluate
±[k]Q for k in {1,2,3,5,7,11,17}. These are controls on generator dependence,
not 14 independent curve samples and not a proof of invariance over all points.

Record 171 point/coefficient traits per point: 14 coordinate/distance/character
metrics; x,y,b residues modulo 2,3,4,5,7,8,11,13,16,17,19,23,31,32,64; and
16 fixed polynomial expressions compared with seven fixed b expressions.
The exact catalog is emitted by the native implementation before cohort use.
The polynomial expressions are x,y,x²,y²,x³,y³,x+y,x−y,xy,x²+y²,x³+y²,
x²y,xy²,2x,3x,3y. Targets are b,−b,b+1,b−1,b+2,b−2,−3.
Curve-equation consequences are marked definitional, not discoveries.

For each trait and point rule, report global/per-field tight upper/lower
published envelopes with every published control kept, and all retained skips.
Report every same-field published/omitted difference, rather than only favorable
traits. Do not fit a new conjunction after seeing results. Any exact single
separator remains exploratory; no historical/security conclusion follows.
Reusing old feature values must retain their coefficient convention and source
hash; do not silently attach historical-b features to a changed-b model.

Separately test 22 public scalar recipes against each known published generator:
b,b−1,b+1,b²,b³,seed,SHA1(seed),SHA256(seed),SHA512(seed),SHA256(field-width b),
low/high32(b),low/high160(b),p+1−n,X9.62(seed),index,run,1,2,3,7.
Reduce scalars modulo n; check G=±[s]Q and Q=±[s]G. Zero scalars are invalid
generator recipes and remain explicit. These are known-scalar equality tests,
not discrete-log recovery. Unknown historical skip generators prevent an
all-cohort historical-generator classifier; do not claim one.

## Execution and acceptance

Use native C++/PARI producer and a separate GMP affine verifier. Develop first
with fixed tiny controls, planted scalar relations, altered points/counts,
singular curves and sign-map tests. No full-cohort execution until preparation,
input/source/binary bindings and the required S3 checkpoint are complete.
Default EKS routing and its current supported contracts must be checked; local
builds/tests and analysis of existing results remain development under AGENTS9a.
Freeze actual launch route, source, binary, deadline and address limit before
the affected invocation; never substitute a deployment fixture for this work.

Primary execution requires all rows, exact input identities, on-curve and
order checks, first-x/minimum-y checks, complete trait catalogs, and independent
verification. Missing or failed checks remain failures/unknowns. Preserve every
attempt and raw file. No clocks, speedup, security break, universal curve
property or discovered historical predicate is admitted from this finite test.

## Current status

Prepared, not run. The restored runtime reports no configured AWS/checkpoint
identity and the old temporary credential wrappers were not preserved. The
existing checkpoint identity has been requested through environment settings;
no keys are to be posted or copied into source or evidence. Development and
source review can continue. This is an access dependency, not a mathematical
obstruction or an exhausted curve search. Update this status additively with
actual preparation/run receipts; preserve the frozen protocol version.
