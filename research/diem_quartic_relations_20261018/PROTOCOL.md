# Protocol W2a: Diem line relations on GHS plane quartics, linear algebra mod ell, and a priced rho arm

Frozen 2026-10-18, before any relation, linear-algebra or rho cost was measured.
Conductor scope: `research/diem_quartic_relations_20261018`,
`src/cryptanalysis/diem_quartic.rs`. Workstream W2a of
`research/plane_quartic_program_20261018/PROGRAM.md`. **Synthetic instances
only. No target descent (that is W2b). No key recovery on any real curve, no
asymptotic or sub-rho claim, no deployed-curve claim, no fastest-implementation
claim (no external tool is installed on the host; every speed statement is "not
compared with an external implementation").** A crossover statement, if any, is a
measured fit over the stated `q` range.

## Question

For `E0/F_{q^3}` with a genus-3 `m = 2` GHS descent to a smooth plane quartic
`C/F_q`, how do the cost of Diem's line-relation phase plus linear algebra mod
`ell` (three variants) and the cost of a matched, priced Pollard rho on the
order-`ell` subgroup of `E0(F_{q^3})` scale with `q`, over `q ~ 2^8 .. 2^21`?
The IC arm here is a **lower bound on a complete attack** (descent excluded), so
it can say nothing about which full method is cheaper; it measures the two
phases that W2b will need and the rho reference it will be compared to.

## Instances

Field sizes: `q_k` = the smallest prime `>= 2^k` with `q_k = 3 (mod 4)`, for
`k = 8, 9, ..., 21` (the restriction to `q = 3 mod 4` makes square roots a single
exponentiation; no claim is made for other `q`). `k <= 21` keeps `#E0 < 2^64`.

`K = F_q[g]/(g^3 + a g + b)`, `(a, b)` lexicographically first with no root in
`F_q` (so cubic irreducible). `u = t^2 + B t + Cc` monic quadratic over `K`.

Per `(k, i)`, `i = 0, 1, 2` (3 instances per `q`; 5 for `k = 8..12`, `i = 0..4`):

- PRNG: SplitMix64 (defined in the module), seed
  `S(k,i) = 0x5732414C_00000000 ^ (q_k << 8) ^ i`, the stream is consumed in
  triples of `K` elements... precisely: candidate `j` draws the 6 values
  `(B0,B1,B2,C0,C1,C2)` uniformly in `[0, q)` in that order.
- Accept the first candidate satisfying, in this order: `u, u', u''` have
  pairwise non-zero resultants and `det[u,u',u''] != 0` (independence); the
  Ciani smoothness conditions of the existing build study on the `K` model;
  the quartic in `W` coordinates has all coefficients in `F_q` (descent);
  `alpha != 0` in the Weierstrass map below; `N = #E0(K)` computed by
  baby-step giant-step on the Hasse interval (two independent points must agree
  on the unique multiple in the interval) is `N = h * ell` with `ell` prime,
  `h <= 16`, `ell > q^{5/2}`.
- Record the number of candidates examined.
- Frozen list: `instances.jsonl` in this directory, produced by
  `cargo run --release --example diem_quartic_study -- gen`, committed and
  sha256-pinned (in `FROZEN.sha256`) before any cost cell is run. The list is a
  deterministic function of this section; a regeneration that differs is a
  protocol violation.

`E0 : y^2 = u u''` with a monic quartic right side; the exact map to a short
Weierstrass model is derived in the module (`(y - w)(y + w) = alpha t + beta`),
and verified by checking that the images of all rational points of `C` lie on the
curve and are annihilated by `N`.

### Instance verification (all recorded)

1. Smoothness (Ciani, on `K` model) and descent (coefficients in `F_q`).
2. `N1 = #C(F_q) = q + 1` by exhaustive enumeration for every instance with
   `k <= 21` that the enumeration budget allows; `N2 = q^2 + 1` by exhaustion
   over `F_{q^2}` for `k <= 11`; `N3 = q^3 + 1 - 3 t_E` by exhaustion over `F_{q^3}`
   for `k = 8` only (cost-limited). These give the `L`-polynomial
   `1 - t_E T^3 + q^3 T^6` and `#Jac(C)(F_q) = #E0(K)` on the checked range; for
   larger `q` the L-polynomial is inherited from the construction, with `N1`
   and the pushforward checks below as the live evidence. Stated as a limit.
3. Pushforward check (independent of the line geometry): every rational point
   `P` of `C` maps to `pi0(P) = (t, y0)` in `E0(K)` with `y0^2 = f(t)`, and
   `N * pi0(P) = O`.

## Relations (Diem)

Factor base `F` is a set of rational points of `C`; classes `[P - P0]` with `P0`
a fixed rational point (the first point of `F`), plus one extra unknown `h` for
the class `[H - 4 P0]` of a line section `H`. A line through `P1, P2 in F`
(distinct, chosen uniformly at random without repetition from the unordered pairs)
meets `C` in `P1 + P2 + R1 + R2` (multiplicities counted; the residual binary
quadratic is computed from the polarisation of `G` at `P1`, `P2`). When it splits
over `F_q`, `[P1]+[P2]+[R1]+[R2] = H` gives the relation
`sum_i x_{P_i} - h = 0` with `x_{P0} = 0`.

Arms (variants):

- **FULL**: `F = C(F_q)` (`q + 1` points). A relation needs `R1, R2` rational (all
  rational points are in `F`).
- **SLP** (reduced FB, single large primes): `|F| = ceil(kappa * q^{2/3})`, `F`
  a uniformly random subset of `C(F_q)` of that size (drawn by random-slice root
  finding, no enumeration); relations with at most one residual point outside `F`
  (the large prime). `kappa in {1, 2, 4}`.
- **DLP** (double large primes, graph/cycle recombination): `|F| =
  ceil(kappa * q^{1/2})`, relations with up to two large primes; large primes are
  vertices, relations are edges, FB-only relations come from cycles
  (structured elimination of large-prime columns). `kappa in {2, 4, 8}`.

Collection stops when the number of FB-only relations obtainable (full
relations + cyclomatic number of the large-prime graph, counted with a
union-find) reaches `(1 + 0.1) * (|F| + 1) + 20`, then structured elimination
produces the matrix. If the cell then fails the consistency test below, the
target is raised by 25% and collection continues (the number of rounds is
recorded; cost includes all rounds).

**Independent verification of every relation, at collection time.** The line
section is recomputed from scratch by evaluating `G` at five points of the line
and interpolating the binary quartic (not the polarisation formulas used to find
it); the four points (with multiplicity) must be exactly its roots; each point
satisfies `G(P) = 0`. Failures are counted and any failure fails the cell.
Pushforward verification (`sum pi0(P_i)` equal across lines in `E0(K)`) on every
relation for `k <= 12`, and on a seeded random sample of 2000 relations
otherwise, including the final rows' constituent lines.

## Linear algebra mod ell

Unknown vector `(x_P for P in F \ P0, h)`; matrix `A` (rows = FB-only relations)
over `F_ell`. Solve with Wiedemann (Berlekamp-Massey, Montgomery arithmetic mod
`ell < 2^64`) on `M = A^T D A` restricted to all columns but one fixed column
(the `h` column normalised to 1, falling back to the next column if `h = 0`),
`D` a random diagonal; the solution is accepted only if `A v = 0` holds on **all**
rows including those not used for the solve. For `|columns| <= 1500` the rank is
also computed by dense Gaussian elimination mod `ell` as a cross-check.
**`ell`-rank finding**: reported as `rank(A)` vs `ncols - 1`, i.e. nullity, as a
function of the number of relations (checkpoints at `1.0, 1.05, 1.1, 1.25, 1.5 x
ncols` where affordable).

**ell-part coverage (rigorous test).** Since `#Jac(C)(F_q) = h * ell` and
`pi0_* phi = [2]` on `E0`, `Jac[ell]` is cyclic and `pi0_*` is injective on it.
The FB classes generate `Jac[ell]` iff for some `P in F`, `h * (pi0(P) - pi0(P0))
!= O` in `E0(K)` (then `h * [P - P0]` is a non-zero element of `Jac[ell]`,
hence a generator). Recorded per instance. Cross-check of the LA solution: for
20 random pairs `(a, b)` in `F`, `v_a * (h(pi0(b)-pi0(P0))) = v_b * (h(pi0(a)-pi0(P0)))`
in `E0(K)` (a solution that is a genuine homomorphism proportional to the
`E0` logarithm passes; this also validates the relations).

## Cost accounting (shared across arms)

- **Cost unit**: `F_q` multiplications (squaring counted as a multiplication),
  every `F_q` inversion priced at its measured multiplication equivalent
  `rho_inv(q)` (microbenchmark on the pinned core, 10^6 samples, recorded per
  `q`). `F_q` additions are not counted (all arms). `K`-multiplication is 9
  products + 4 reduction products and is counted that way.
- **mod-ell operation** (multiply-add in Wiedemann, elimination, rho exponent
  bookkeeping): weighted `w_ell`. Primary `w_ell = 9` (word-model scaling
  `(log ell / log q)^2` at `ell ~ q^3`, schoolbook); sensitivity `w_ell = 1`
  (one hardware multiplication, what the implementation does). Both are
  reported; no result is stated for only one of them.
- **Unpriced counters (AGENTS.md 5a), emitted by both arms**, priced at
  `w in {0, 0.1, 1}` group-addition equivalents (GAE = one measured `E0(K)`
  affine addition inside a batched walk, in `F_q`-mult equivalents, measured
  per `q`):
  - IC arm: hash lookups and inserts (FB index, large-prime table, relation
    dedup), RNG draws, union-find operations, sparse-row entries touched in
    elimination, root-finding calls (also priced inside via counted mults),
    point canonicalisations (also priced inside via counted inversions),
    sorting (none), memory allocation (not counted, stated).
  - Rho arm: DP checks (one per step), DP table inserts/lookups,
    canonicalisations (negation map), partition-hash computations (one per step),
    RNG draws, mod-ell bookkeeping adds (also priced at `w_ell`).
  Ranking between arms is stated only where the order holds across the whole
  band (and across both `w_ell` values); otherwise "unresolved at the boundary".
- Wall time on the pinned cores is recorded as a practicality note.
- Normalised cost `S = cost / sqrt(ell)`.

## Rho arm

Pollard rho on `E0(K)` restricted to the order-`ell` subgroup: generator
`G0 = h * P`, target `Q = s G0` with `s` a seeded uniform in `[1, ell)`. R-adding
walk, `r = 32` multipliers `m_j G0 + n_j Q` (random mod `ell`), partition by a hash
of the `x`-coordinate, distinguished points with `2^{-d}`,
`d = max(0, floor(log2 sqrt(ell)) - 10)`, `m = min(64, ...)` walkers advanced in
lock-step with Montgomery batch inversion in `K` (counted). Collision of two
distinct DPs gives `s`; the recovered `s` is **verified** by `s G0 == Q`. Runs per
instance: 9 for `k <= 12`, 5 for `k = 13..16`, 3 for `k = 17, 18`, 1 for `k >= 19`,
each with an independent seeded `s` and walk seed. A negation-map variant is
reported as a separate arm only if it is implemented and verified; otherwise the
rho figures are stated without it (conservative **against** rho, i.e. in favour
of IC, by at most about `sqrt 2`).

## Budget and stopping

Per measurement cell `<= 1 h` wall on 3 pinned cores (instances run one per core).
Whole study `<= ~6 h` wall. A cell whose predicted cost (from smaller cells of the
same arm, fitted power law) exceeds 1 h is not run; its value is a labelled
extrapolation and the first skipped `k` is recorded. Memory `<= 12 GB`.
Hosts: `taskset -c 26,27,28 nice -n 10`, `cargo -j 3`.

## Fits and uncertainty

Per arm and per `q`: mean of the per-instance (and per-run) cost, with a
bootstrap 95% interval over instances and runs. Exponent: ordinary least squares
of `log2(cost)` on `k` (equivalently `log2 q`) over the cells with `k >= 10`,
reported with the OLS standard error and a 95% bootstrap interval resampling
instances. For each IC arm the exponent is reported for the relation phase, the
linear-algebra phase, and the total (best `kappa` per `q`, chosen post hoc by
minimum measured total, and also with `kappa` fixed at each grid value).
Extrapolation of a phase beyond its measured range uses that fit, stated with its
exponent.

## Predictions (stated in advance, falsifiable)

- P1: rho cost scales as `q^{1.5 +- 0.1}` (steps `~ sqrt(pi ell / 2)`, `ell ~ q^3 / h`).
- P2: FULL total is dominated by linear algebra, exponent `2 +- 0.2`.
- P3: DLP relation-phase exponent `1 +- 0.2` and total exponent `<= 1.3`.
- P4: at every instance `k >= 10` the FB classes generate `Jac[ell]` (the toy
  failure at `q <= 11` is a small-`q` effect of `q + 1` points being too few).
- P5: the LA solution passes the cross-multiplication check on every instance.
A falsified prediction is reported as a result.

## Not measured

Target descent (W2b), a class-group arithmetic on `C` (W1), any comparison of a
complete attack, behaviour for `q = 1 (mod 4)`, any `q > 2^21`, the `m = 3, r = 5`
(Smith) route.

## Language and layout

Computation: native Rust, `src/cryptanalysis/diem_quartic.rs` and
`examples/diem_quartic_study.rs`. Python only plots recorded values and lays out
the report. Records: JSON lines under `results/`; reports under `reports/`.
