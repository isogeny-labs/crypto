# Original public DLP solved through a freshly computed calibration base

The joined fiber/alignment/inverse/conic path recovers k=183049 and verifies
[k]D1=B on BOTH the source curve and the recovered destination classes. It
computes a new calibration base, fits a new matrix, solves BOTH original-input
inverse systems and collects point logs afresh. No earlier original-D1image,
Bimage, target scalar, alignment matrix or point-log table is loaded.
Classification: engineering integration of the verified algebraic components;
this is not a timing, generic complexity, novelty or complete cold-cost claim.

## Exact instance and remaining supplied setup

Source y²=f(x), f ascending [31,89,73,48,91,41,123,1], p=127, genus3,
ell=1028893. D1=2*(0,44), B=2*(3,21), #J=2057786. Target quartic descending
homogeneous coefficients [1,58,103,47,66,88,51,80,84,123,34,83,109,1,111].
The quotient kernel and Weil normalization are still supplied in copied
KERNELS.txt/ALPHA.txt. This reused setup is the remaining cold-integration
boundary. PUBLIC.txt contains the original curve/generator, not a target scalar.

## Computed calibration and fresh alignment

The degree18fiber selector is executed again with the same exact restriction,
saturation, complete regular zero-set and124direct-value controls. It computes
two supported points and uses the FIRST in fixed order, P=(54,56). The new
source base is twice(P+(0,44)+(3,21)-3infinity), with
u=[14,25,16,1], v=[103,116,48]. Its target image is
2*((9,13)-(89,44)). These data are computed during execution, not loaded from
the preceding run. Their scalar relative to original D1 is not needed.

The first16multiples are all regular: n1..12train the matrix; n13..16are held
out. The projective system has rank63/nullity1 and the8x8matrix has rank8.
All16checks and a corrupted-matrix control pass. No expected matrix is read.
Both original inputs use offset1 by the fixed first-supported offset rule.

| Input | Fresh primary Kummer vector | Fresh adjacent vector |
|---|---|---|
| D1 | [3,110,98,89,109,82,112,82] | [61,98,20,86,96,78,72,100] |
| B | [56,100,66,117,97,24,109,32] | [119,93,61,67,91,120,17,7] |

Both inverse systems are generated anew. Known positive-control identities,
independent tensor expansion and input-corruption checks pass in each producer.

## Actual inverse and orientation

A rebuilt private msolvev0.11.0 at cc044551f8e202d00691990fd3fd0b1226a88ea2
passes71upstream tests. One positive and two actual-input cells each give
dimension0/degree12. The independent verifier checks every one of12equations
identically, squarefreeness, invertible denominator and numerator convention126;
the positive cell also recovers its known control.

For EACH original input the parameter polynomial factors as two irreducible
cubics and three irreducible quadratics, totaldegree12. All five factors
reconstruct valid closed divisors. Public adjacent coordinates and the new
known calibration image select the sign; subtract offset1 of that image.
All factors agree on ONE original-image class, with two distinct effective
divisor representations per input. All ten corrupted-adjacency checks reject.
No original-image oracle is invoked.

## Fresh point logs and conic descent

The point list is computed with127affine polynomial restrictions and checked
against an independent enumerator. The48pointbase generates1128pairs,
878unique lines, 314split relations (14LP0/86LP1/214LP2). Sparse elimination
removes81large-prime columns. The remaining232-by-47core is DENSE, rank46 and
nullity1. Reverse tracing recovers all129point logs; all314relation residuals
pass. Sparse counters:14455row operations/9480entries. Full sparse scalability
is not established by this final dense core.

All distinct recovered divisors are tested with the same ten anchor pairs.

| Original input | Pairs attempted | Accepted conic descents | Nonsplit | Other exclusions | Original class log |
|---|---:|---:|---:|---:|---:|
| D1 | 20 | 3 | 16 | 1 projection/infinity | 34511 |
| B | 20 | 3 | 17 | 0 | 829912 |

Every accepted conic passes exact intersection division, residual root/y
checks, all127resultant evaluations and the independent group divisor identity.
The previously proved centered formula gives each translated log; public sign
and subtraction of the NEW calibration log137451 give the original class log.
All three descents per input agree. These checks are on one curve/input pair,
not independent curve experiments. Ratio829912/34511 mod1028893 is183049.
Actual scalar multiplication verifies this ratio on source and destination;
k+1fails on the source. No expected k is supplied to the worker.

## Frozen first executions and evidence

| Gate | Source | Pre-run S3checkpoint UTC on2026-11-15 | Outcome |
|---|---|---|---|
| Fiber/base alignment and equations | cc24c005a260 | 225944 | exit0 |
| Positive/D1/Bsolve cohort | df1c2f9a5687 | 230343 | all3exit0, verified |
| Both class recoveries | 8c030a9a4df1 | 230658 | exit0 |
| Fresh relations and conic DLP | 4aea4bb4b468 | 230937 | exit0 |

Every gate executed once after its own committed source/binaries/inputs and
checkpoint gate. No scientific rerun or dropped input; all stderr files empty.
Unused-helper compiler warnings are retained. The missing ignored solver cache
was rebuilt from the pinned official source without global library changes.
No time or RSS ratio is inferred from these local unisolated correctness runs.

[Prep raw](RUN.txt), [solver cohort](SOLVE.txt), [class recovery](RECOVER.txt),
[descent/verification](DESCENT.txt), [PDF](REPORT.pdf), [diagram](flow.svg),
[prep protocol](PROTOCOL.md), [solve protocol](PROTOCOL-SOLVE.md),
[class protocol](PROTOCOL-RECOVER.md), [descent protocol](PROTOCOL-DESCENT.md).

## Proof map and next step

The degree18selector uses the conditional theorem
CalibrationFiber.saturated_norm_degree from
../hec_calibrationfiber13_20261115/DegreeBound.lean. Conic scalar descent uses
ConicDescent.centered_conic from ../hec_conicdescent13_20261115/Centering.lean.
Their checked Lean4.19/mathlib pins and explicit geometric hypotheses are in
the adjacent prior proof maps. No new mathematical theorem is claimed by this
integration, and these scalar/degree proofs do not formalize the full code.
The [public-adjacency argument](../hec_inverse13_20261115/RESULTS.md) applies
with the new nonzero odd-order base. Geometric isogeny premises remain explicit.

Next replace copied quotient kernel/alpha with fresh cold construction in this
joined path. Then measure complete costs with fresh inputs/source identity,
including construction, inverse solving, collection/LA/descent and verification.
The original cold-cost failure is unchanged. A larger-field cost case and a
scalable final solver are required before extrapolating. Genus2/4 and the full
10xgoal remain open. Existing performance graphs are unchanged because this
run has no comparable timing cohort.

Primary comparator: [Tian2020sections4.2.2–4.2.3](https://arxiv.org/html/2007.03172).
The calibration scan removed here was specific to the earlier implementation;
Tian already provides a separate two-torsion alignment route. No novelty claim.
