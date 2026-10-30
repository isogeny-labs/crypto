# Native diagnostic protocol v1

## Hypothesis and scope

Public quadratic-character patterns near original-curve x coordinates might
select factor bases that make original-curve relations cheaper. This is a
testable hypothesis; no positive outcome is assumed. Coverage, independent
matrix information and complete target-solving cost are different endpoints.

TR-01 uses p=1019,4091,16319, a=-3 and the first nonsingular b with prime order.
Each field uses seeds 0..29 inclusive by default; a runner can freeze disjoint
explicit seed intervals. B=round(n^(1/3)). Seeds feed the documented native
SplitMix64 streams, not the historical Python RNG. Random/progression/twist-run
arms use the same public target and random-walk prefix. The twist score is the
following consecutive nonsquare run, capped at eight, with shuffled ties.
No knob is tuned on evaluation targets. BSGS includes its own setup and check.

TR-02/03 inspect p=7,11,19,31,43 and every (a,b), retaining all nonsingular
models with prime original order at least seven. Enumerate every three-x base,
both signs, all score-optimal ties and every finite unordered x triple with
repetition. The distinct nonzero-point triple-base distribution is exhausted
once per cyclic group order. This includes isomorphic and special models and
is not a generic-curve performance sample. It proves completeness only inside
this declared finite search space.

## Accounting and validation

TR-01 charges selection, pair setup, relation search, joint base-log/k recovery
and verification separately. Raw operations remain in their own units.
Discovery of public curve parameters and creation of Q are excluded equally;
discovery time is recorded. Every target pays fresh tables. Memory cardinalities
are reported; complete field-operation prices and byte-level peak memory remain
unmeasured. Instrumented ns are diagnostic, not isolation-admitted timings.

Every recovered scalar is replayed. Counters must reconcile exactly across
phases. Exhaustive S3 zero sets must equal the sets from independently evaluated
signed group additions on the original curve and its twist. Audit-only oracle
logs never enter relation search. Raw source parameters, points, scalars, phase
records, hashes and failures remain in artifacts. Replaying with the same binary
is not an independent validation-gate certificate.

## Flow, stopping and acceptance

Planned TR-01 cases: fields × frozen seeds × four methods. Planned TR-02/03:
all eligible coefficient models and all bases/triples in those fields. Preserve
planned/attempted/completed/failing counts when landing an actual run; this
implementation PR contains no new run export or admitted performance finding.

Stop at 1000*n search attempts per relation solve, any algebraic/accounting
failure, or the runner's 1800-second deadline. A failure exits nonzero and keeps
available artifacts; an interrupted running manifest is incomplete, not a pass.
There is no automatic retry or mid-search resume. Do not discard seeds.

Pre-run evidence-retention amendment: each completed benchmark case is published
individually before the next starts, with synced lifecycle/progress events and
an atomic active-case snapshot. Search progress is sampled at most 256 trials
apart. Caught failures retain the exact case and current counters; hard kills
retain only the last published prefix and cannot produce a pass or final clock.
The optional per-solve limit only tightens the original attempt cap. In-solve
evidence I/O is included in `total_ns` and separated as `evidence_io_ns`; final
record/panel exports remain outside that interval. This amendment changes the
diagnostic instrumentation, not the hypothesis, target/RNG streams, relation
equations or finite audit scope. No performance result is promoted.

An exact finite coverage benefit is reported as such. A native solver improvement
requires lower complete cost on identical fresh public instances against the
frozen ordinary IC baseline, with every answer verified. A fastest or vs-rho
claim additionally needs matched strongest references and the existing admission,
0/0.1/1 GAE, external-tool, independent-validation and visual-report gates.

TR-04 through TR-07 require their own frozen executable protocol before timing;
their designs in README.md are not completed experiments. Before ready-card
promotion, pin a merged native source, fill the protocol blob, implement/check
recovery, and replace same-implementation replay with the required independent
verification/admission. No scheduler or funding state transition is implicit.

## TR-04 finite census preparation, 2026-10-29

The new user-authorized four-part implementation has a separate
[human-readable contract](../../research/twist_relations_tr04_20261029/PROTOCOL.md)
and [machine-readable record](../../research/twist_relations_tr04_20261029/protocol.json).
This supplement does not change the older TR01-03 source, streams, finite
audit fixtures or historical evidence above. The new record remains draft
until its source/analysis/runtime-calibration identities and canonical hash
are frozen before affected data.

The declared new scope is every coefficient-model disposition over p=7,11,19,
B=2,3, selector budget eight and relation-word length eight. It compares
random, progression, collision-avoiding and four fixed twist-threshold
selectors. The finite IID stream distribution is exhausted by exact weighted
subset-state DP; progression starts/steps and all 256 binary query words are
also exhausted. Every nonzero public target is included for the canonical G.
Incomplete bases and missing independent-rank milestones remain in the
unconditional distribution and paid-work vector. Exactness is confined to
that declared finite population; arbitrary fields, generators or unbounded
streams are outside it.

Execution is prepared for the existing native workflow's `tr04` suite, a
source/protocol/S3-checkpoint preflight and three parallel ARC EKS Spot field
jobs. It is not a scheduler-ready or funded-card promotion. Source/evidence
checkpointing does not establish process-memory recovery, and L0 operation
counts do not establish an admitted runtime/GAE or ECDLP-solving speedup.
No new data or cloud-execution result is asserted by this preparation note.
