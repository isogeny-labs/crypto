# Fixed-model cold construction and verified IC solve

The pipeline constructs Weil data, both quotient kernels, normalization and
quotient geometry from the public source input, then computes calibration and
alignment, solves both inverse maps, collects point logs and verifies k=183049
on source AND destination. Both separately frozen versions pass once. V2
replaces source extension-field counting with the admitted coefficient
recurrence and exact integer lift. Classification: engineering integration.

Scope is the registered F127/genus3 model and preconfigured target chart.
No wall-time, 10x, generic complexity or novelty claim follows from these runs.

## Identity and the actual construction boundary

PUBLIC.txt is the SAME COLD_INPUT as the earlier cold comparison:
y²=f(x), f ascending [31,89,73,48,91,41,123,1], p=127, genus3,
ell=1028893, D1.u=[0,0,1], D1.v=[44,14], B.u=[9,121,1], B.v=[114,96].
It contains no kernel, normalization, matrix, point logs, images or recovered
scalar. Input/executable fingerprints are committed. Host: Darwin arm64,
Apple M4 Pro, local L0/unisolated. No independent-host replay or timing cohort.

Each version starts with a fresh output/setup directory. The study root has
NO supplied KERNELS.txt or ALPHA.txt. The loader requires HEC_SETUP_DIR and
reads source/generator/target/kernel/normalization data from newly emitted
files. A deliberately absent setup fails specifically at that loader,
exit101: an EXPECTED negative control before scientific field work.
All five positive stages and the three solver subcells exit0.

The section routines still encode a registered quartic model and preconfigured
anchors/chart. They validate that the constructed QF matches that model.
Bases and alignment are recomputed; general chart discovery is NOT implemented.
Thus this is cold construction within a fixed configuration, not arbitrary-curve
cold solving or a claim that all model-selection work has been eliminated.

## V1: direct counts and freshly constructed quotient data

Source222ed0a121f3/checkpoint20261115T232606Z precedes one run in run/.
Driver stdout creation20261115T232624Z is filesystem metadata, not a duration.
V1 counts over Fp/Fp² and recovers N1=129,N2=16043,#J=2057786 and
L=[1,1,-43,-1224,-5461,16129,2048383].

Both27pointkernels are constructed in degree13 and checked for validity,
order3 and Frobenius stability. Normalization is rebuilt (samplemultiple2,
no root extraction), then quotient geometry, smoothness and target
pointcount/Serresign. Derived HC=[18,39,55,66,2,33,55,1]; target quartic
descending homogeneous coefficients are
[1,58,103,47,66,88,51,80,84,123,34,83,109,1,111].
No earlier kernel/alpha or cyclic generator/target map scan is used.

## V2: recurrence and exact order-constrained lifting

Sourcee694bfedd11a/checkpoint20261115T233622Z precedes a separate run in run-v2/.
Stdout creation20261115T233701Z. V1 source/records remain unchanged.
V2 receives ACTUAL f,p,ell; no archived coefficients or order are supplied.
The input validator establishes primeell and a nonzero ell-torsion D1, supplying
the order-divisibility premise. It uses380coefficient steps and obtains
Hasse-Witt matrix [[35,96,124],[83,90,111],[52,11,1]], residues(1,84,46).

Exact coefficient bounds, one order multiple and congruences reduce30a2lifts
to3coarse survivors and1endpoint survivor. Disjoint unit intervals
(-20,-19),(-4,-3),(21,22) certify three real roots. Recovered
(a1,a2,a3,N)=(1,-43,-1224,2057786); implied counts129/16043 match V1's
independent counts. No source extension-field enumeration is called in V2.

The source coefficient component uses O(p)recurrence steps with bounded
products and extended-Euclidean unit inversions, plus exact lift/certificate
work. This is NOT an end-to-end complexity/runtime bound: current independent
point-list/control checks still include exhaustive enumerators, and final LA
is dense. Ambiguity, nonpositive enclosures or missing sufficient root
certificates stop unsupported, not silently reject a possible polynomial.
Finite arithmetic/domain restrictions remain; no generic Schoof replacement.

Fresh kernels, alpha, HC/QF, sourcePUBLIC, actual Bpolynomials, matrix and
point-log outputs match byte-for-byte between versions. POSTCHECK-HASHES.txt
records these post-run comparisons, not V2 inputs. The two versions are not
pooled timing samples or independent curves.

## Shared fresh alignment, inverse and scalar verification

Each run recomputes the degree18fiber and uses first supported P=(54,56),
doubling source/image to clear cofactor2. Source base:
u=[14,25,16,1],v=[103,116,48]; target2*((9,13)-(89,44)).
No scalar relative to D1 is supplied. Twelve matched multiples fit rank63;
matrixrank8, four withheld pairs and corruption rejection pass. Both original
inputs use offset1 and fresh primary/adjacent vectors and equations.

Positive/D1/Bmsolve cells give dimension0/degree12 and exact12equation
identities. Positive recovers its known control. For each actual input, two
cubic and three quadratic factors reconstruct two distinct divisor
representations and ONE original image. Public adjacency with the NEW base
orients signs; all ten corrupted-adjacency checks reject. No old original-image
oracle, expectedmatrix, targetlog or scalar is loaded.

Fresh48pointbase collection examines1128pairs/878unique lines and314split
relations (14LP0/86LP1/214LP2). Sparse elimination removes81LPcolumns; the
232×47 DENSE core has rank46/nullity1. Reverse tracing recovers all129pointlogs;
all314relation residuals pass. Sparse counters14455operations/9480entries retain
their native meanings, not wall-time equivalents.

| Original input | Conic pairs | Accepted | Nonsplit | Other exclusion | Original log |
|---|---:|---:|---:|---:|---:|
| D1 | 20 | 3 | 16 | 1 projection/infinity | 34511 |
| B | 20 | 3 | 17 | 0 | 829912 |

Accepted conics pass exact division/root checks,127resultant evaluations and
independent Jacobian divisor identities. Centered formula, public sign and
new calibration log137451 give agreeing logs. Ratio183049 verifies on source
AND destination; k+1 rejects on source. Six accepted descents are repeated
checks of ONE public pair, not independent curve samples.

## Evidence, proof boundary and next cost case

[First run report](reports/run-20261115T2326Z/REPORT.md),
[V2 run report](reports/run-20261115T2337Z/REPORT.md), [results PDF](REPORT.pdf),
[diagram](flow.svg), [V1 driver](DRIVER.txt), [V2 driver](DRIVER-V2.txt),
[V2 setup](run-v2/COLDPREP.stdout), [V2 final verification](run-v2/DESCENT.stdout),
[protocol](PROTOCOL.md), [recurrence amendment](PROTOCOL-V2.md).
Each version executed once after source/binary/publicinput freeze and
S3checkpoint/gate. No unexpected scientific failure or rerun. The loader
exception is documented; positive stage stderr is empty. The private official
msolve0.11 executable dependency is reused after hashes, not scientific data;
its rebuilt71test receipt is linked in BUILD.md.

Prior conditional Lean scalar-centering and degree-bound proofs retain their
explicit geometric hypotheses; they do not formalize the whole isogeny,
loader, recurrence or implementation. This integration adds no new theorem.
Primary comparator: [Tian2020sections4.2.2–4.2.3](https://arxiv.org/html/2007.03172).

Next develop the charged cost profile and larger-field portability case:
field/exponent representation, general chart construction and scalable final
LA. The fixed-model F127correctness run establishes no numerical crossover.
Existing performance graphs and the prior failed coldcost record are unchanged
because there is no comparable timing cohort. Genus2/4 and full10xremain open.
