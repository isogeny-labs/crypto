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
