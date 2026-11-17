# Twist index calculus with endomorphism-image factors

Frozen 23 October 2026 before implementing or executing this experiment.
Baseline: `c7cecbd07f437b600b5893be61ad0eedf38d2368`, the exact S4/F4
collector and its independently checked original-prefix relation sets.
The objective remains an improvement in twist-based index calculus;
relation counts alone are not a complete-cost or asymptotic result.

## Hypothesis and exact scope

The existing two-line base B is saturated by its cubic and transported
Frobenius automorphisms. Test the additional physical factors
`phi(P)=P-psi(P)`, where `psi(x,y)=(omega*x,y)` and its verified subgroup
eigenvalue is `lambda`. Their logarithms are `(1-lambda)` times the
original factor logarithms, so these points need no new unknown columns.
They can nevertheless have different geometric components and supply
different mixed relations. Charge additional construction and peel work;
do not credit automorphism images as independent equations.

Use every two-line record of the existing frozen seven-prime, three-seed
panel: p=37,73,97,157,229,241,337, seeds 0,1,2 (21 instances). Read the
complete original residual streams, not only successful old prefixes.
The immutable source is `../quartic_twist_mixed_v1.ndjson`, SHA-256
`d50c500961344a1d5813a02dbba9019aef3309cb74180e99715ca693f5b8094c`.
Pin exact curve, subgroup, generator, target, signs and physical coordinates
from each record. Only public synthetic targets are used. Known answers
and reference factor logs are confined to reference generation and checks.

## Six arms on every instance

All arms retain the original three distinct L0 factors in the S4 solve.
They differ only in the permitted fourth, peeled physical factor:

| Arm | Peeled points |
| --- | --- |
| original | Original L2 |
| psi-control | psi(L2), canonically deduplicated and transported back to the original physical points |
| phi-l2 | phi(L2) |
| phi-l0 | phi(L0) |
| union-l2 | Original L2 union phi(L2) |
| union-all | Original L2 union phi(L2) union phi(L0) |

Canonicalize physical points up to sign, retaining signed coefficients
relative to the original orbit representatives. Record infinity, kernel,
duplicate abscissae, coincidences with L0 and coefficient conflicts.
Never silently drop them. An unsupported exceptional case is retained
and stops that case with an explicit incomplete verdict. A complete orbit
fold still has the original columns; new point count is separate.

## Stage 0: geometry and action checks

Construct phi by actual group subtraction and independently verify
`phi(P)=[1-lambda]P` for every original factor. Validate each new point
on the curve and under the original cubic/Frobenius action. The public
eigenvalues must agree with explicit maps on the generator and factors.
The psi control must reproduce the exact original physical peel set and
coefficients. A deliberately corrupted eigenvalue must be rejected.

Check the coordinate formula
`3*x^2*x(phi(P)) = -omega^2*(x^3+4*B)` against the group computation.
It is interpreted only where x!=0, omega!=1 and characteristic!=3.
Issue an identity certificate for its polynomial derivation, including
an explicit multiple of `omega^2+omega+1` and the curve equation; prove
the corresponding faithful conditional identity in pinned Lean where
feasible. No novelty follows from this standard degree-three map.

## Stage 1: full-stream exact relation and rank census

For all 126 arm/instance pairs, obtain complete signed four-distinct-point
relations on the whole frozen residual stream with a native reference
oracle. Compute reference orbit-representative logarithms by BSGS, verify
each against the group, and derive physical logs only with the separately
checked endomorphism coefficients. Use signed L0 pair sums and the peeled
factor to enumerate tuples, and re-add every accepted relation in the
original curve group. The public known target scalar may be used by this
reference path only, after its group check.

Record complete signed sets or canonical set digests, occupancy, row
dependence, rank progress, first full-rank query and cumulative peel counts
through that query. Count every finite signed peel, including misses.
Retain no-rank-at-cap cases as censored. Use the same anchored column
system and sorted relation insertion rule for all arms. Compare source
factor counts, unique physical factors and column counts separately.

For uniformly sampled residuals and a fixed disjoint peeled set, linearity
of expectation fixes the first moment at `16*C(|L0|,3)*|peels|/n`.
Thus additional factors do not establish better yield per peel by count
alone. Compare the actual frozen streams and matrix rank; do not turn
finite-stream deviations into a universal density advantage.

## Stage 2: native polynomial collection

Implement each arm without reading reference hits or factor logs. Use the
same exact S4 coefficient derivation and original F4 implementation:
grevlex, degree bound 12, no staircase stop, one Rayon thread per process.
Generate residuals from public G,Q,a,b. Split every returned cubic,
resolve signs and verify each sum in the curve group. Process complete
queries until full anchored rank or the original stream cap. Verify the
target and every original and image-factor logarithm in the group.

Only after candidate output is sealed, compare every processed signed
set, matrix rank and recovery with stage 1 in a separate audit mode.
Replay the original arm against the c7cecbd07f baseline on the seven
previously recorded prefixes. Run all arms on all 21 instances, including
losses and sparse cases; no selection by observed gain. A/A must reproduce
complete non-timing output. An omitted-reference-relation mutation and
an input-hash mutation must be rejected by the relevant checks.

## Execution, accounting and reporting

Native Rust algorithms, harnesses and verifiers only. Preserve the
baseline source; this round uses a separate native crate initialized
from its exact implementation. Independent processes may run different
instances with a fixed host allocation, one Rayon thread each. Use an
8 GiB process address-space cap and 1,200 s watchdog per instance/mode;
record timeout, OOM and partial query records. Bound cloud jobs by the
repository ephemeral CLI's TTL and retain terminal receipts. A cap is
not a runtime estimate or evidence of impossibility.

Record group-field arithmetic, subgroup matrix arithmetic and F4's
row-reduction counter with their exact boundaries where implemented.
Full F4 normalization/interreduction/root arithmetic, allocation/hash
CPU, cold base/model construction and memory must be closed before a
whole-method price claim. First/A/A checks deterministic counts, not
timing. Isolated five-round ABAB, strongest same-target twist24 rho,
calibrated 0/0.1/1 GAE and identical-input external solvers remain required
for speed claims. Do not pool shared setup across targets as the headline.

Retain all source snapshots, inputs, seeds, exits, failures and negative
controls. Pack new raw trees deterministically, keep cited summaries and
certificates loose, and produce source-linked graphs, reports and inspected
PDFs for every research run. Update the existing PR and problem record.
