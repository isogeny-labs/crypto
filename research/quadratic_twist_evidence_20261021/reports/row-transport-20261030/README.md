# Actual F13 relation rows, coefficient spans and source recovery

2026-10-30 · same PR2381 · [inspected report PDF](report.pdf) · [diagram](bridge.svg) ·
[actual Lean source](../../lean-row-yield/ActualRowYield.lean) · [raw proof evidence](../../validation/row-transport-20261030/README.md).

The new unmodified scoped proof gate passes **20 named theorems /24 audited
declarations**,including4 generated helpers,0failing/extra trusted/native axioms.
Only propext,Classical.choice,Quot.sound. Lean4.34.1/Mathlib
d13f23b723b8a846827a245b89c10fc7d3f11612. Imported historical projects are not a
fresh combined/global audit. All18 earlier polynomial identities remain unchanged.
No new field-polynomial identity is claimed or fake IDC substituted for group proofs.

## Actual models and theorem

Curve field F13; source E:y²=x³+x+4,historical `icv1-fp4-t0-6bee67e0`;
target actual d2 monic twist T:y²=x³+4x+6,`icv1-fp4-t0-097da6b6`.
These historical registrations are still absent from the active merged registry;
no row/alias is fabricated or promoted. P=(0,2),U=(8,11),order7. Phi and I=[4]D
are the previously checked actual coordinate functions on Mathlib nonsingular
points,not log tables. PhiP=U;I(PhiQ)=Q when7Q=0.

For any finite INTEGER coefficient universe V,source bases B_i killed by7,
and arbitrary unknown Q killed by7,Lean proves

    sum_i v_i Phi(B_i)=Phi(Q) iff sum_i v_i B_i=Q.

Thus the complete matched coefficient-row finsets and cardinalities equal.
Reducing integer coefficients modulo7 gives equal spans over the RELATION field
F7 (different from curve fieldF13). For any supplied coefficient subspace L,the
increment dim(L+span(rows))-dim(L) is equal. This is the coefficient-rank metric
of the study,not an assertion that an arbitrary unknown-RHS system is solved.

A supplied state-dependent query rule q(L) must return source7torsion points;
with complete matched oracles the collection-step functions,all finite iterates
and rank progress are equal. This **deterministic adaptive state coupling** does
not prove iid stopping-time semantics,absorption/integrability,the native RREF
implementation or the separate general PairRank independence/count theorem.
Different bases,encodings,costs and degree-admitted subsets remain distinct.

Any admitted target subset is valid on the original source under the written
matched-domain hypotheses. The actual inverse hom also sends a valid target
AGGREGATE relation to Q under7Q=0,without claiming individual summands vanish.
Global injectivity would be false:the actual source K=(10,0) is nonzero,
PhiK=0 and7K!=0. These three domain facts are kernel checked.

## Original main-DLP scalar, not only a target-point relation

For target bases A_i with EXPLICIT PUBLIC setup logs A_i=l_i U,and an actual
accepted relation sum(v_i A_i)=Phi(Q)+s U,Lean proves,for signed integers,

    Q=(sum_i v_i*l_i-s) P
    Q=((sum_i v_i*l_i-s) mod7) P.

The inverse coordinate hom and the source annihilator justify the recovery and
modulo step. Unknown Q is an arbitrary actual source point with7Q=0. The base-log
hypotheses must come from legitimate public setup;no target label is returned
to a solver. This formal bridge does not show how many relations an algorithm
finds,or grant free setup/inversion/linear algebra work.

## Degree-qualified scope and actual progress

The module defines an ABSTRACT numerical admission profile d(v) and cap.
An explicit compression hypothesis says every source-admitted valid row also
meets the target cap;only then is source qualified span contained in target
qualified span. There is **no proof that these abstract profiles are the native
solver's degree,degree of regularity or measured work**,and no actual strict
lower-cap/higher-yield instance is inferred. The unchanged RelationCost conditional
cost theorem also requires verified rates/work/overheads before any speed claim.

Full bounded-native experiment was considered at the start,then this actual
domain/row bridge was selected before any implementation/execution of that
experiment. All16F13arms,original/reduced,caps0..8,all6source targets,blocked
setups,strongest-source/matched/direct-enumeration controls are a separate
[validated DRAFT](../../../../backlog/XP-20261030-degree-qualified-f13/README.md).
Producer/auditor/runner/scientific JSON protocol DO NOT EXIST YET;zero capped
solver runs,zero new native scalar recoveries or degree/yield/speed results.
Do not treat this as a completed or partially executed favorable subset.

## Proof flow, preservation and open gates

Six scoped proof builds:01failed;02passed15named/19audited;03passed18/22;
04and05failed recovery-sum rewrites;06passes20/24. Corrected statements and
hypotheses are unchanged. Exact failed sources (ending .lean.txt),passed-stage
sources,build/audit/status logs and official checkpoint receipts are retained.
First failure was zero-action/forward-hom/finrank coercion;last failures were
sum-lemma orientation. Explicit finite-set induction fixes variable integer
scalar summation. Failed sorryAx outputs are unaccepted,not0axiom proofs.
All code was committed before affected builds. Every proof launch follows an
actual successful official narrow-branch checkpoint;no gate patch/skip/fake marker.

Source-linked report/diagram/PDF pages rendered and inspected. Proof-only result
has no new native comparable measurements;diagram explains what was checked.
Prior uniform/dual/pair-rank numeric reports and canonical scoreboards/graphs
remain unchanged:no curve/catalog/performance row is promoted. No native
producer/replay/control/benchmark/Lean-independent checker run is claimed.
Existing root library latest5140case attempt remains incomplete600s/124,no
fresh rerun/pass for unchanged root. Native backlog binary was rebuilt through
Cargo freshness and the new draft validates;no Cairn lease/task/funding.
Conductor127/no replacement. Naming/1921missing study registrations,required
exact-head CI/review/integration and broader geometry/probability/regularity/
complete-work obligations remain. Keep same PR open/non-draft,auto-mergeoff.

Sources:existing actual CM8/dual and phase-recovery modules,the old conditional
RelationCost theorem,lead-atlas isogeny-transported-yield/decomposition/degree
leads and prior protocols. No novelty claim for transported group relations.
Native/proof caps600sec;bounded iteration03:08–04:08UTC,two localworkers,no paid
provisioning/arbitrary result upload. Private narrow checkpoint token is outside
Git;the supplied broad AWS pair was not stored or included in evidence.
