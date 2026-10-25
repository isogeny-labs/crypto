# Phase demand of the certified F43 S3 route

Preregistered 2026-10-25, starting head
`fda3018794b3e840fc7656a17fa9195cfaf13652`, run `phase-demand-20261025`.
This is the next bounded step in PR2381, using existing local resources.

## Hypothesis and frozen scope

The first-target useful-rank and matrix-degree gain need not lower setup plus
recovery demand for every target. Test every one of the ten frozen F43
size-two base policies, all four roles (`main`, `twist`, `matched`,
`pulled_back_twist_base`), and every nonidentity main target nP, n=1..10.
This gives40 independent base setups and400 separate source-point recoveries.
The actual models, subgroups, points and target map remain exactly those in
`native/reduced-s3-20261025/cases/p43-*.json`. Each query's reduced solver
certificate is bound to its existing counterpart in
`native/adaptive-s3-20261025/cases/p43-*.json`. Keep all policies, failures,
ties and transported-base controls; do not select a favorable target after
execution. Scalar labels generate/check public known-answer inputs only and
are never passed to the recovery routine.

This is a phase-demand diagnostic within one IC method, not a cross-method
timing experiment or a calibrated total-cost claim. An exact request vector
does not price hashing, allocation, map traversal, sorting or memory. No
wall time, GAE, speed ratio, asymptotic claim or leaderboard promotion follows
from it. Actual speed comparisons still need the repository's ecbench,
matched-reference and external-fastest gates.

## Exact pipeline

Use the unchanged adaptive solver's order, row witnesses, D=0..8 schedule,
positive input-activation requirement,64-round and32-expanded-degree caps.
Snapshot its helper prefix in a new file; do not edit the frozen source.
Root candidate enumeration and polynomial witness maintenance remain charged
parts of that same solver contract.

For each role/policy construct its two membership polynomials. In setup query
public multiples mG, m=1..10 in order, until the accumulated two-column
coefficient matrix first has full rank. Each query executes the certified
reduced solver, tries all four sign pairs on each returned ordered root,
retains the distinct actual relation rows with RHS m, and inserts them in
lexicographic order. Solve the augmented matrix and verify both base logs on
actual points. No brute-force log search is used to initialize these logs.
Retain all earlier empty queries, dependencies and rank progression.

For a separate source point Q, first apply the existing actual F43 map when
the role is `twist` or `matched`; main and pullback roles use the identity.
Try shifts s=0..10 in order. Compute mapped(Q)+sG with the declared binary
scalar chain. Skip an identity image explicitly. Execute the full certified
reduced solver at every other attempted shift. Traverse ordered roots in
the solver's order, then base indices and signs (-1,+1). Charge all sign
lifts even after finding the first valid answer in that query. Recover
sum(sign*base_log)-s mod11 from the first valid lift, stop after that query,
and verify the scalar on the original main curve. Keep every failed query.
Return an explicit failure if no recovery is found; never fabricate a cost.

## Demand vector and independent replay

Record setup and each target's online/recovery verification separately:

- solver calls, root membership polynomial evaluations and reduced-S3
  evaluations, plus their polynomial term visits;
- every main matrix build and every lower-degree promotion test: basis calls,
  generated monomial rows, dense column slots represented, polynomial and
  witness monomial copies, pivot normalizations, subtraction steps, and
  polynomial/witness term updates;
- actual point-add requests (including identity/negation/vertical branches),
  binary scalar-chain iterations, point negations, signed lifts tried,
  subgroup linear-system modular row updates, and explicit coordinate-map
  multiplications;
- query/rank/degree/expanded-peak journals, retained separately from totals.

These are precisely named algorithm requests. Dense column slots are a
structural matrix-demand metric, not a claim that sparse code allocates them.
Polynomial term updates, inversion requests and point-add requests have
different costs and must not be summed into a fictional scalar time. Count
all lower-degree membership basis builds and original-input witnesses so
reinsertion cannot hide their work. Metadata and independent audit are outside
the algorithm-request ledger and are named as unpriced overhead.

C++ records the actual executed counters. Independent Rust uses its own
field/point/polynomial arithmetic, reconstructs the stated incremental
algorithm and phase boundaries, and checks the entire request vector, query
coverage, base-log solve, answer and source/certificate binding. Retain the
older batch-RREF semantic check as an additional distinct verification.
Controls must reject rehashed false setup totals, false matrix demand,
omitted failed query, false base log, false recovered scalar and false source
binding, without issuing a receipt. A matched-base difference is reported and
explained rather than suppressed.

## Formal bridge and outcomes

Attempt a faithful Lean recovery theorem for an injective additive point map,
explicit base-log equations and a signed relation at a shifted target. Expose
the source annihilator needed for reducing an integer answer modulo r.
Connect it to the existing actual F43 map rather than assume a generic
base-field twist map. A new polynomial identity, if needed, gets a source-bound
IDC1 record; existing14 identities and sources remain unchanged.

For every policy report exact setup and individual-target vectors. A lower
degree/rank observation with a larger request component narrows the remaining
pricing question; it does not erase the algebraic result. Componentwise
domination is only in this declared request model. No intrinsic regularity
comparison is performed. A non-isomorphic twist needs an explicit transferable
subgroup map or a checked pairing route before any source-DLP benefit claim.

Each native command has a600-second cap; the single iteration has a3600-second
native envelope, two build workers, and uses current local CPU/memory/disk.
No paid provisioning, additional upload or credential reuse. Preserve failed
attempts, raw changed controls, manifests, exact replay, graph/PDF, proof-gate
and mandatory Cargo receipts. Extend PR2381 normally with no force/rebase.
Atlas match and relevant leads were read at crypto-autoresearcher
`5998553341fb29d3b9ca1a4c9ac6e58d96ca3ffa`; this extends the existing
charged-toy-IC, closure-degree and curve-presentation leads, not a novelty claim.
Actual Conductor check exited127; no reservation/replacement coordinator or
completion-gate exception is claimed.
