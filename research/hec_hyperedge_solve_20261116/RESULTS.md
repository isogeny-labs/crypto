# Complete higher-arity supplied-target solves and existing controls

## Complete-solve outcome

All20preregistered cases terminate without algebraic/scalar/reference mismatch. Twelve recover and verify the supplied public target; eight lower-arity controls exhaust or reach their tuple cap. All10highest-arity cases recover113/136:fourgenus2cases,two genus3cases,fourgenus4cases. Two additional genus4three-LPcases recover. Every outcome, includingNone, agrees with the immutable sparse solve_fast implementation on exactly the inserted equations. Expected scalars are checked by the caller, outside the solver API.

The solver keeps source and target descent as catalogue-column equations instead of enlarging anchors to contain descent support. This matters at genus4:targetsupport31or20exceedsH10andH16, yet recovery succeeds. Source/target exact unprojected descent identities, raw principal identities, common catalogue-annihilator projection certificate, nonzero projected source, integer Hasse bound and final original-source scalar verification remain in the correctness boundary.

|g,p|H|maxLP|Target113 status/classes|Target136 status/classes|
|---|---:|---:|---|---|
|2,1009|12|2|verified / 813|verified / 813|
|2,1009|20|2|verified / 837|verified / 837|
|3,503|14|2|exhausted / 8008|exhausted / 8008|
|3,503|14|3|verified / 2068|verified / 1660|
|4,127|10|2|exhausted / 4032|exhausted / 4032|
|4,127|10|3|exhausted / 4032|exhausted / 4032|
|4,127|10|4|verified / 2139|verified / 2139|
|4,127|16|2|tuple cap / 16384|tuple cap / 16384|
|4,127|16|3|verified / 3304|verified / 3304|
|4,127|16|4|verified / 1504|verified / 1504|

Fresh setup and descent per call; fixed firstHcanonicalanchors; exact original tuple/sign stream; no expected scalar is passed. Reused thread-family instances and known-scalar controls, not fresh holdouts or blinded input selection. No noncompletion is replaced. CONTROL.txt retains all counts/supports/ranks/statuses; exit0, empty stderr.

## Why the prior rank screen was insufficient

At genus3H14the anchor-only screen reached rank13atclass1447. With the supplied targets, completion needs2068and1660classes. Anchor-rank completion does not imply a descent vector outside that anchor subspace is determined.

More precisely, let Rbe principal rows over F_ell, abe source descent and btarget descent. The certified projected log vector x satisfies Rx=0, ax=1andk=bx. The scalar k is uniquely determined by these equations iff

    b lies in rowspace([R;a]),

or equivalently rank([R;a;b])=rank([R;a]). If b=k*a+lambda*R, applying x proves the returned scalar. Conversely, if b is not in that rowspace, a linear functional on the quotient by rowspace(R) can hold a at1while varying b; hence the linear constraints do not fix k. The actual group may still fix it, but the admitted relation space is insufficient. This explains the observed extra work without assuming all catalogue logs must be known. At one genus3target the augmented solve rank is223below catalogue+target dimension237andk is already determined.

## Existing complete controls

A separate frozen gate runs the historical compact principal solver and12native/table variants on each of the same6unique curve/target inputs. Branches1/64, Fast/Hybrid, native reference_oracle true/false and table oracle are retained. Every one of78calls recovers and verifies; no incomplete control. Raw CONTROLS.txt includes every arm, not a selected fastest arm. Internal implementation clocks are not published or compared.

|g,p|Target index|Compact anchors|Compact constructed outputs|CandidateH/maxLP|Candidate visited classes|Native/table branches1trials|
|---|---:|---:|---:|---|---:|---:|
|2,1009|0|21|672|12 / 2|813|1038|
|2,1009|1|21|736|12 / 2|813|1045|
|3,503|0|64|2744|14 / 3|2068|1503|
|3,503|1|64|2336|14 / 3|1660|1660|
|4,127|0|38|1888|16 / 4|1504|1489|
|4,127|1|38|2112|16 / 4|1504|1657|

These are HETEROGENEOUS event counts, not arithmetic units or time ratios. Compact constructs signed batches with its optimized kernel; candidate uses fresh Newton interpolation for every visited signclass. Compactconstructedincludesunusedbatchtail; compactdrawscanalsoincludetupleretries. Native/table trials include Jacobian addition+oracle. Native/tablebranches64andallremainingarmdata stay in rawoutput; this table is not an elapsed-time ranking or strongest-arm selection. The new candidate has no tenfold generation-count premise against these complete controls. No timing campaign follows solely from the earlier contrast with incomplete small-anchor2LP.

## Decision and remaining objective

The finite mathematical result is useful: retaining3/4off-anchor rational places can produce enough independent constraints for complete supplied-target solves that the2LPrestricted family fails to determine. This is ordinary larger-matrix indexcalculus, not a novel algorithm/exponent. The control comparison weakens the10xcase: the apparent large advantage from an incomplete restricted policy disappears against established complete solvers. Source/target support, setup, construction kernels and actual elimination must be included in any later full-cost claim. Genus2doesnotgainextraarity.

Keep the verified prototype and all failures/caps. Do not launch a performance campaign without a new complete cost premise. The next family screen under consideration is osculatory/confluent rational interpolation with repeated anchor support, changing available functions rather than equivalent arithmetic. Existing rational-interpolation primary paper includes an osculatory section/theorem4.2; this is established priorart and has not been implemented here. Local notes have no explicit anchor-Hermite constructor, which is not a novelty proof. Requires primarytextreview, primitive cancellation/multiplicity handling and an independent rank/cost falsifier before execution. The mathematical tenfold objective stays active.

## Custody and retained errors

Candidatefreeze01ac15121e29/checkpoint20261116T050035Z; once-only20case execution, exit0/emptystderr. Existing-controlinitialbuildfailedonhistoricalsolve_for_lastAPIarity. A subsequent shell sequence lackedfailfast and recorded a partial hashmanifest without a binary atfa522c7811:that manifest isINVALIDfor execution custody and retained asFREEZE-CONTROLS-INITIAL.sha256. No scientific controls ran then.

AMENDMENT-CONTROLS.md records the correction. Local pipeline_compat.rsadds only&stepsargument to match currentAPI; COMPAT.diffshows the exact one-line change. Historical source unchanged. Fastignoreshotcolumns;Hybridusesitsseparateexistingpath. Successfulsource/binaryfreeze93c779a4673e/checkpoint20261116T050440Z precedes the once-only78controlgate, exit0/emptystderr. Subsequent freeze usedfailfast and checked executable existence before hashing. No scientific case or expected outcome changed.

No timing ledgerrecord was opened because these are correctness/count gates and no elapsed comparison is asserted. Shared dailylog remains reserved byT6448; SESSION.mdcontains this project's handoff. Source/lock/imported/native/binary hashes and raw outputs are committed and S3checkpointed. No PR merge authorized or performed.
