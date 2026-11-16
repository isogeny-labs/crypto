# Checked factor-base roots preserve the actual S3 query

2026-10-25 · run `known-roots-20261025` · PR2381 ·
[protocol](../../KNOWN_ROOTS_20261025_PROTOCOL.md) ·
[pre-rerun ordering correction](../../KNOWN_ROOTS_20261025_ORDER_CORRECTION.md) ·
[PDF](report.pdf) · [graph](membership.svg) · [diagram](bridge.svg) ·
[native records](../../native/known-roots-20261025/) ·
[proof](../../lean-known-roots/KnownRoots.lean) ·
[validation, attempts and reproduction](../../validation/known-roots-20261025/README.md).

The native C++ producer and independent Rust verifier execute **80 setups and
800 separate nonidentity main-source recoveries**, comprising40 setups/400
recoveries in each of the frozen scan and new checked-root arms. All800 scalars
verify. They use all ten previous F43 size-two policies and four roles (main,
twist, matched base and inverse-transported twist base). Reference records equal
the frozen phase-demand records exactly after removing the three new zero counters.
Every candidate constrained root, ordered certificate, matrix/promotion trace,
expanded witness degree, useful row/rank, failed query, base log and source scalar
equals its reference. A second batch-RREF/witness checker audits all100 distinct
solver certificates, separately from the request replay.

## What changed and what it costs

The previous query reevaluated its two membership polynomials across F43,
requiring129 membership evaluations per solver call before four reduced-S3
constraint checks. The candidate checks the already supplied two base coordinates
once per setup, binds both expanded monic polynomials, verifies both listed roots
on both membership polynomials, and queries their four ordered Cartesian pairs.
The base's column order is preserved; only the separate query-root list is sorted.

| Prefix policy, separate source Q=P; request unit | Scan main | Checked main | Scan twist | Checked twist |
| --- | ---: | ---: | ---: | ---: |
| Setup membership evaluations |387|4|129|4|
| Online membership evaluations |258|0|129|0|
| Cold membership evaluations |645|4|258|4|
| Cold solver calls |5|5|2|2|
| Represented matrix column slots |540|540|36|36|
| Point-add requests |125|125|136|136|
| New setup root entry checks |0|2|0|2|
| New setup order checks |0|1|0|1|
| New setup polynomial bindings |0|2|0|2|

The membership request ratios for this one cold record are645/4=161.25 and
258/4=64.5. These are **component request ratios, not speedups**. Every one of
the400 candidate cold records includes four setup membership evaluations; each
reference has129 times its actual solver-call count. All other old components
except membership-related term visits are unchanged. Transported controls retain
the prior mathematical and request correspondence, including the two explicit
coordinate-map multiplications where applicable. All policies, failed queries
and later target losses remain in the canonical records and all-target graph.

This is an actual query-interface algorithm change with a verified request
reduction. A calibrated engineering improvement or complete attack-cost gain
is **unestablished**. The32 components have distinct, sometimes nested units.
Sorting, field-table setup, traversal, allocation, hashing, internal exponentiation
and independent auditing remain unpriced; do not sum the counters into a total.
Dense slots represent matrix shape, not measured sparse allocation. No timing,
GAE, S, rho ratio, intrinsic regularity or fastest-external claim is made.

## Why no roots are lost

For an integral domain, Lean proves

    product(a in roots) (x-a)=0 iff x is in roots.

For two entries it proves `(x-a)(x-b)=x²-(a+b)x+ab` and exact equivalence
between the two-polynomial membership constraints and their Cartesian root list,
with an arbitrary remaining constraint h. No distinctness assumption is needed
for that set theorem; distinct sorted entries are needed for this implementation's
ordered no-duplicate list. The domain hypothesis is essential to the proof.
Checking a few zero evaluations alone is not treated as completeness evidence.

The first actual main system uses x²−17x+9 and roots4,13; the twist uses
x²−32x+40 and roots15,17, in F43. Lean instantiates the equivalence with the
actual reduced-S3 polynomial for every target abscissa z. The generic theorem
applies to all other setup pairs, whose exact bindings Rust independently checks.

| Statement | Lean declaration in `KnownRoots` |
| --- | --- |
| Complete product-membership root set | `product_root_iff` |
| Expanded monic two-root polynomial | `expanded_two_factors` |
| Exact quadratic root set | `quadratic_roots_iff` |
| Constrained Cartesian solution equivalence | `constrained_pairs_iff` |
| Actual first-policy F43 systems, arbitrary z | `main_actual_input`, `twist_actual_input` |

The unmodified scoped gate passes **182 public named theorems /238 audited
declarations**, zero failing or extra trusted axioms. The seven new audited
declarations include six public theorems and one generated definition helper.
Only propext, Classical.choice and Quot.sound occur. Native demand code is
independently replayed, not universally formalized. The polynomial certificate
**IDC1he9f02ada0008638c** checks the expansion at32+16 native points and rejects
false-plus-one and false-source controls; completeness rests on the Lean theorem.
All fourteen earlier study identities and frozen sources remain unchanged.

## Scope, controls and next lead

The main model is `icv1-fp6-t0-74779950`, E:y²=x³+x, P=(31,18), and the
twist model is `icv1-fp6-t0-c6a8a47c`, T:Y²=X³+4X, U=(24,26), over F43,
r11. The actual source-DLP map phi(x,y)=(41x,11y) is additive/injective and
P maps to U. This exceptional pair is base-field-isomorphic. U has no registered
EC1 representation; no older generator alias is reused. The frozen annihilator-
aware shifted-recovery theorem is reused unchanged. Recovery receives Q alone;
known labels only generate/check public synthetic targets outside it.

Nine rehashed corruptions reject exit101/no receipt: omitted/extra root, false
membership polynomial, setup demand, matrix demand, base log, recovered scalar,
source binding and a removed failed query. The exact fresh CI command passes,
including byte comparisons, the source-bound certificate and22 backend tests.
The full release library suite and workflow/parser checks are recorded in validation.

The first native attempt rejected an invalid ascending-order assumption on the
transported base, before accepting any case. Its source, log and empty output
tree remain archived. The preregistered correction changes only query-root
ordering. A pre-compile helper-name error is also retained. The current-main
profile probe could not start because gawk is unavailable; it supplies no cost
evidence. Conductor remains unavailable (exit127), no reservation claimed.

Next, test a checked cache of signed base points to remove repeated scalar/negation
requests during actual root lifting, charging its setup. Freeze its protocol and
test the same per-target workload against this checked-root route before a claim.
This addresses the remaining first-policy lifting tradeoff; broader useful-yield,
intrinsic regularity, general transfer and fully calibrated work remain open.
No additional upload or credential reuse occurred. Shared coordination, review,
exact-head CI access and merge conflicts remain completion holds.

Canonical scoreboard/leaderboard/browser graphs were checked for applicability:
no calibrated whole-pipeline value, registered model, benchmark, session or claim
row changes here. Their axes cannot represent these unpriced request vectors;
they remain unchanged, and the new study graph is the quantitative source.
