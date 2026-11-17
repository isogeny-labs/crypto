# Prospective finite-cost continuation review

This packet is a concrete, unfrozen amendment draft. It is written with the p739
results known. The original first-gate protocol, failed timing admission, raw
records, earlier PLAN.md/status.json and resource records remain unchanged.
The historical design documents' missing-budget statements are superseded only
by resource_authorization_20261030.json; they are retained as dated history.

The user supplied a three-hour continuation allowance. The recorded conservative
execution interpretation is 10,800 cumulative active worker seconds, absolute
expiry 2026-10-30T05:05:50Z, one active payload, 25 million coefficients, 8 GiB,
and 3,600 seconds per execution. Stop at the earliest effective limit. Preparation
consumes the remaining wall-clock window; no first-gate budget reset, transfer,
refund or claim of measured infrastructure cost follows from this authorization.

## Scientific sequence and complete denominators

1. Independently review and admit the known p739 finite observations for reuse
   under their actual original protocol/session/record identities. The disjoint
   enrollment and cross-session evidence are supporting proofs of specific
   identities and non-wall observations, not retrospective clock qualification.
2. Execute the unchanged p186379 target0 gate: all three arms, five measured
   streams and one warmup, 18 base executions. Independently replay all 15
   measured outcomes/counts and review the complete gate. Any cap, exhausted
   solver, wrong answer, missing row, invalid category coverage or failed replay
   stops automatic extension. The gate and its replay consume this same budget.
3. Only after the gate review, execute the exact remaining 1,200 workloads:
   cohort-16 remainder120 and unchanged cohorts20,24,28,32,36,40 (1,080).
   Preserve five streams, one warmup, all three arms, exact seeded ordering and
   frozen methods. This adds 21,600 base rows and 6,000 measured pairs.
4. Enroll prior p73960/1,080rows/300pairs exactly once. Total enrollment is
   1,260 workloads, 22,680 base rows and 6,300 measured pairs. The separately
   charged upper gate is not enrolled even though its target can overlap a
   full cohort. Independent replay and failed/retried attempts never increase
   target coverage. Preserve every missing slot; full sums are null when any
   required paired observation is unadmitted.

The 36 original finite pricing cells and the source-reviewed disjoint category
mapping remain fixed. Report method-only, calibration-only and combined costs;
retain actual harness, replay, reporting, memory, controller and allocation
observations separately. Do not invent a conversion of those resources into GAE.
No cost threshold or observed ratio selects the cohort, admission, or seeds.

## Timing is a separate incomplete requirement

The unchanged instrument cannot deliver the originally specified qualified
p739 timing comparison: grading uses solve_wall_ns, the old cold metric uses
method_call_wall_ns, the actual latter endpoint includes instrumentation closing
and methods::complete, and all retained reference and A/A calls are below 1 ms
under both fields. Generic rho uses rho_solve/recovery_check rather than the
candidate's five target phases. Changing only a classifier field does not fix
the short-call issue. These facts are bound in the retained timing audit.

The minimal prospective scope uses the unchanged instrument for exact counted
work and keeps every clock diagnostic, with wall_valid false. It changes no
scientific workload and makes no new timing claim. The exact original timing
requirement remains unfulfilled even if finite-cost collection finishes. A later
fixed repeated-call-block design would need an independent pre-data review,
fresh per-call state, correct method-specific phase boundaries, matched controls,
resource permission and explicit block estimand; it cannot be called individual
call latency. No repetition factor, padding, lowered floor or adaptive batching
is authorized by this packet.

## Provenance and replay contract

The complete protocol draft is protocol.json. It retains the original instrument
source543a54b4 and binarya1c8ca89 for every arm. Existing evidence is SHA-bound in
evidence-bindings.json. The native eight-plan preparation certificate retains
original and enrolled specification identities, plan hashes, indices, sequence
numbers, exact unsigned seeds, workloads, methods and within-round order.

The consumer scope is full1260-disjoint-p739/v1. Runtime lineage is a separate
pencil-disjoint-protocol-lineage/v1 record binding the full frozen continuation
protocol, prior v5 gate protocol and each plan's actual data origin. Do not pin
that runtime lineage hash inside the protocol: doing so would create a circular
digest. Prior records/history/session must be bound; prospective ones stay null
until observed. The gate's known-data amendment alone is explicitly post-data.
All changed analysis and exporter code must be reviewed and pinned before the
new observations. The enrollment certificate does not itself grant scientific
reuse admission.

## Deadline enforcement and remaining prerequisites

A request deadline equal to the outer expiry is insufficient for the retained
controller. Relative Pod deadlines, scheduling delay and synchronous replay/CRIU
can exceed that requested instant. Root's draft uses C0=02:05:50Z,
G=03:34:20Z, R=180 seconds, D=05:05:50Z, so 2G-C0+R=D. The draft request has
SHA b0c0fb5b059aca391cfd2e84e983de2e44ab7e4b8ae4ee383d74cbd0b2d46b31.
An independent review must confirm these bounds against the actual unchanged
package, serial computation/fresh/replay ownership and current infrastructure.
The guard is earlier than the authorized expiry and can leave missing targets.
Only the approved package client is applicable; the old installed client is not
interchangeable. No request was submitted by this reviewer.

Before freezing: finish and independently review the exact native disjoint
consumer and its canonical exporter; bind their source/build/binary identities;
provide the separate fixed-upper-gate analysis/export route; complete prospective
scientific reuse and source-coverage binding; validate the actual schema and full
applicable lint; bind reviewed operational requests and guards; commit the frozen
records before new affected data. Retained 11-test preparation evidence is not
validation of these still-developing consumers.

The first draft validation used the existing native validator and exited1 with
`amendment after data`. Its stdout/stderr are preserved as draft-native-validation.*.
That failure is expected until the narrowly reviewed known-data continuation
opt-in exists; it is not waived or reported as a successful validation. No freeze,
new experiment, cloud action, or launch approval is performed by this packet.

## Subsequent conditional guard correction

The R180 draft above was independently found incomplete for cumulative fresh-Job
creation call delays. The initial review is preserved. The new immutable request
19929f222a46d13f02fa50738e31ef7c86e3a0854c4f2041a58f9f6c05c46e28
uses R900 and G03:28:20Z, unchanged C0 and outerD. Four kube operations are
bounded by180-second timeout plus5-second kill-after each; adding60-second
termination grace totals800 seconds. Reserve900 leaves100 seconds for ordinary
local/control scheduling, without using the fresh120-second subtraction as slack.
operational-source-review-v2.json accepts this conditional resource guard only.
Normal OS/Kubernetes enforcement and serial across-run reconciliation remain
explicit assumptions, not a universal host-failure theorem. No scientific or
launch admission follows from this resource guard. Both drafts bind v2.
