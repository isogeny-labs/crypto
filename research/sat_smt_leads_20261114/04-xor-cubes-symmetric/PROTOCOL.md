# PROTOCOL — idea 4: XOR cubes on symmetric functions (pilot)

Written 2026-11-14 before any measured run. Pilot only, toy sizes, one core,
not claim data. Every number that results is `[pcc:unverified]`.

## Claim under test

For the m=3 point decomposition problem on a binary Koblitz-type curve
`y^2+xy = x^3+a x^2+1` over F_{2^n} with factor base `x(P) in V`, `dim V = l`:
fixing k of the l coordinates (in a basis of V) of `e1 = x1+x2+x3` ("XOR
cube") simplifies the residual Weil-descent system more than fixing k
literal bits of x1 ("literal cube"), in two measurable senses:

- (A) algebraic: the residual bit-system becomes F2-affine (decidable by
  Gaussian elimination alone) for some k < l;
- (B) SAT: total conflicts summed over all 2^k XOR cubes is below the total
  summed over all 2^k literal cubes, on the same CNF, same instances.

## Prediction (from the idea card)

Residual becomes affine for some k < l.

## Kill criterion (fixed now)

Kill-criterion-met iff BOTH:
1. the non-affine rank of the residual (defined below) does not reach 0 for
   any k <= l-1 (we also report k = l) in any measured cell, AND
2. XOR-cube total conflicts >= literal-cube total conflicts, judged as the
   geometric-mean ratio XOR/literal over instances >= 1.0 at the largest l,
   at k = l (full e1 cube vs full x1 cube) and at the best k of each arm.

If (1) holds and (2) fails (ratio < 1) the verdict is "alive (SAT constant
only)" or "inconclusive" depending on whether the fitted slope in bits per l
differs by more than 0.15 between arms; a constant-only ratio is reported as
a ratio, not as a scaling change. If (1) fails the verdict is "alive".

## Instance generator

- Field F_{2^n}, n odd prime, modulus = first irreducible polynomial (in
  increasing integer order) for which x is primitive. Curve a=1, b=1.
- Cells (l, n): (3,11), (4,13), (5,17), (6,19); (7,23) only if budget allows
  (algebra part only is acceptable there).
- V = span of l random F2-independent field elements, `random.Random(seed)`,
  seed = 1000*l + instance index. Basis stored as generated (no echelon).
- Per cell 8 targets: 4 planted (R = P1+P2+P3, Pi random curve points with
  x(Pi) in V, x != 0, pairwise distinct x) and 4 random (x of a random curve
  point). Ground truth for every target = brute-force zero set of
  S4(x1,x2,x3,xR) over V^3 (full 2^{3l} table). Results are reported
  separately for truth-SAT and truth-UNSAT targets.
- S4 from the resultant in X of S3(x1,x2,X), S3(x3,x4,X), with
  S3(u,v,w) = (uv+uw+vw)^2 + uvw + b.

## Validation (before measured runs)

- planted tuple is a zero of S4; SAT model of the CNF decodes to a zero of
  S4; when the model's x's are curve x-coordinates, some sign choice of real
  points sums to +-R (reported as a count, spurious quadratic-twist-type
  models are possible because S4 only sees x);
- CNF SAT/UNSAT == brute-force truth on every instance, every arm, every
  cube set (sum over cubes: at least one cube SAT iff truth-SAT);
- symbolic e-form of S4 is checked numerically against S4 at random points.

## Part A — algebraic measurement

View X (x-bit variables): variables are the 3l coordinates of x1,x2,x3.
XOR cube with value c in F2^k: substitute x3_j = c_j + x1_j + x2_j for
j < k, leaving 3l-k variables. Literal cube: x1_j = c_j, j < k. Compute the
exact ANF (Moebius transform of the full evaluation table) of the n bit
equations. Metrics per (instance, k, cube type), over all cubes if 2^k <= 16
else 16 seeded-random cubes:
- `rank_nonlin` = F2-rank of the span of ANF coefficients (in F2^n) of
  monomials of degree >= 2 ("non-affine rank"; 0 means affine);
- `rank_deg3` = same for degree >= 3; number of nonlinear monomials; max
  degree.
k ranges over 0..l.

View E (e-variables), exact symbolic: write S4 in (e1,e2,e3,xR,b) by
symmetric reduction; for e1 fixed list the monomials e2^i e3^j and their
bit-degree wt(i)+wt(j) when e2,e3 are treated as free field elements. The
residual is affine in the bits of (e2,e3) iff the maximum is <= 1.

## Part B — SAT measurement

One CNF per instance, identical for all arms: ANF encoding of the k=0 system
(one Tseitin AND variable per monomial of degree >= 2, n XOR rows chopped
into 3-literal chunks) plus l definitional variables y_j <-> x1_j^x2_j^x3_j.
Solver: CaDiCaL 1.5.3 via python-sat, default options. Arms, differing only
in the assumptions passed:
- `plain`  : one solve, no assumptions (untreated SAT arm);
- `lit-k`  : all 2^k assignments of x1_0..x1_{k-1} as assumptions;
- `xor-k`  : all 2^k assignments of y_0..y_{k-1} as assumptions.
k in {1, 2, ..., l} (subset {2, l/2 rounded up, l} if the cell is too slow,
decided on timing of the first instance only, never on outcomes). One fresh
solver per (instance, arm, k), cubes in numeric order, incremental (learned
clauses kept across cubes of the same arm); all cubes are solved (no early
exit on SAT). Arms are interleaved per instance. A fresh-solver-per-cube
variant is run at the two smallest cells as a robustness check.
Secondary arm: CryptoMiniSat (pycryptosat) with the XOR rows native, same
cube sets; pycryptosat exposes no counters, so wall time only.

Primary metric: total conflicts summed over cubes (also decisions,
propagations). Secondary: wall time.

Non-SAT baselines reported as counts alongside (not run as solvers):
enumeration 2^{3l}/6 tuples, MITM 2^{2l}/2 pair-table entries, linear 2+1
scan 2^l leaf solves (repository baseline 2^{(m-2)l}*poly).

Slope: least-squares fit of log2(geometric-mean total conflicts) against l
per arm, truth-UNSAT targets (full refutation) as the primary population.

## Budget

About 40 minutes wall in total, any single run <= 5 minutes, one core.
