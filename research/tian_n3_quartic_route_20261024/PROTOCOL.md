# Protocol: the n = 3 Tian route, quartic Jacobian census and Diem index calculus

Study `research/tian_n3_quartic_route_20261024/`, written 2026-10-24 and frozen
before any measured run (the sha256 of this file is recorded in every run's
`env.txt`; amendments are appended below, dated, before the runs they govern).
Tools (std-only Rust, built with `rustc --edition 2021 -O`):
`examples/tian_quartic_census.rs` (item a), `examples/quartic_ic.rs` (item b).
Background: `research/weil_restriction_isogeny_census_20261018/REPORT.md`
(Theorem 4, Part H) and its `results/run-20261018T1605Z/curves.csv`.

Smoke runs made while writing the tools (timing and unit-test debugging at
q = 7, 11, 13, 17, 19 with 40k-300k samples) are not evidence and their output
is discarded; they fixed only the sample sizes below.

## 1. Questions

* **(a)** Round 2 took on trust that for an `E/F_{q^3}` with a Tian kernel the
  quotient `Res E / Lambda` is a Jacobian. Does the isogeny class of `Res E`
  contain the Jacobian of a smooth plane quartic over `F_q`, for the
  prime-order `E` that Theorem 4 flags, at `q in {7, 11, 13}` (extended to
  17, 19), and does the answer depend on the flag?
* **(b)** How does Diem's double-large-prime index calculus on plane-quartic
  Jacobians compare with Pollard rho in the same group, at counted cost,
  as `q` grows?

## 2. Derivations used by the tools (exact)

Let `E/F_{q^3}` have trace `t3` (`#E = q^3 + 1 - t3`) and `W = Res E`, an
abelian threefold over `F_q`.

1. The `q`-Frobenius of `W` has characteristic polynomial
   `P(T) = T^6 - t3 T^3 + q^3`. (`W(F_{q^k})` for `3 | k` contains `E(F_{q^k})`
   as a factor; on the induced module `F[T]/(chi(T^3))` of round 2 the
   characteristic polynomial of `T` is `chi(T^3)`; equivalently the roots are
   the cube roots of the two roots `alpha, alpha-bar` of
   `x^2 - t3 x + q^3`, all of absolute value `sqrt q`.)
2. For a genus-3 curve `C/F_q` with `L_C(T) = 1 + c1 T + c2 T^2 + c3 T^3 +
   q c2 T^4 + q^2 c1 T^5 + q^3 T^6`, `L_C` equals the reversal of `P` iff
   `c1 = c2 = 0` and `c3 = -t3`. In terms of point counts `N_k = #C(F_{q^k})`:
   `N_1 = q + 1`, `N_2 = q^2 + 1`, `N_3 = q^3 + 1 - 3 t3` (power sums
   `s_1 = s_2 = 0`, `s_3 = 3 t3`). Any quartic with `N_1 = q+1`, `N_2 = q^2+1`
   therefore has `L_C = 1 + c3 T^3 + q^3 T^6`, so it matches the `Res E` with
   `t3 = -c3` automatically (the tool asserts `N_3 = q^3 + 1 - 3 t3` is an
   integer solution). Equal `L`-polynomials give isogenous Jacobians (Tate).
3. `#Jac(C)(F_q) = L_C(1) = q^3 + 1 - t3 = #E(F_{q^3})`.
4. `P` is irreducible over `Q` unless `t3 = s^3 - 3 q s` for an integer
   `|s| <= 2 sqrt q` (then `alpha = beta^3` with `beta` a `q`-Weil number, `E` is
   isogenous to the base change of an `E'/F_q`, and `W ~ E' x B`). Prime
   `#E` excludes that case (`#E'` would divide `#E`); the tool tests it and
   lists those `t3`. For prime-order `E` the class is therefore a simple
   ordinary-or-supersingular threefold with irreducible `P` (supersingular
   only for `t3 = 0`, never prime order for the `q` here).
5. A curve with an elliptic curve over `F_{q^3}` of trace `t3` exists iff
   `p` does not divide `t3` or `t3 = 0` (Waterhouse, `n = 3` odd); other `t3`
   are skipped as unrealizable (checked against round 2's curve list).
6. Own Theorem 4 implementation, per `(q, t3, l)`, `l in {3, 5, 7}`, `l != p`:
   roots of `T^2 - t3 T + q^3` in `F_l`; none = nonsplit = no kernel; two
   distinct roots: kernel iff a root is an `l`-th-power-cube (`lambda^((l-1)/3) = 1`
   for `l = 1 mod 3`, always otherwise); a double root `lambda`: scalar `phi`
   (kernel always) is possible only if `l^2 | t3^2 - 4 q^3` (necessary:
   `(pi - lambda)/l` integral), otherwise Jordan (kernel iff `l = 1 mod 3`).
   `Ambiguous` marks a double root with `l^2 | disc` and `l != 1 mod 3`
   (scalar or Jordan depending on the individual curve). Cross-check: every
   odd-`l` row of round 2's `curves.csv` (class and flag) must agree with this
   code; every prime-order realizable `t3` must satisfy: own `kernel` implies
   round 2 has a curve with that trace and kernel, own `none` implies none.

## 3. What a hit or a miss means (stated before the data)

The class of `W` is determined by `t3` alone, so "a Jacobian exists in the
class" is a property of `t3`, not of an individual `E`. Theorem 4's flag, in
contrast, is a property of the individual `E` (scalar versus Jordan can differ
for equal `t3`); a `t3` is **positive** if some prime-order `E` with that trace
has a Tian kernel at some odd `l <= 7` (round 2's per-curve flag, equal to own
`kernel` or an `ambiguous` value realised by some curve), **negative**
otherwise (own `none`, no curve flagged).

* Positive `t3`: Tian's argument (quoted by round 2, not proved there)
  makes `W/Lambda` a principally polarised threefold that is geometrically
  indecomposable, hence a Jacobian over `F_q`, non-hyperelliptic for prime
  `#E`, hence a plane quartic with `L_C = P`. **Prediction H1**: every
  positive `t3` is hit (at least one smooth quartic with `N_1, N_2, N_3` as
  in section 2 among the samples). A positive `t3` with local expected count
  `>= 10` hits and zero hits is a **miss** and is evidence against the chain
  "Tian kernel implies plane-quartic Jacobian" (or against its provisos), to
  be reported as such.
* Negative `t3`: Theorem 4 says there is no `(l,l,l)`-isogeny of Tian type
  with `l <= 7`; it is silent about other `l`, about non-Tian isogenies and
  about Jacobians in the class reached some other way. A hit is **allowed** and
  is not a contradiction; a miss is also allowed. The control quantifies the
  base rate: **H2 (exploratory, no direction preregistered)**: the hit-rate
  ratio (hits / local expectation) of positive and negative `t3` is
  compared; if the negatives are hit at the same rate, the flag does not
  discriminate and a hit is no evidence of a kernel; if systematically lower,
  the flag carries information beyond the density.
* Local expectation of `t3`: mean hits of realizable, non-constant-type `t3'`
  with `|t3' - t3| <= 6`, excluding `t3` itself (the hit rate is smooth in
  `t3`; its centre is near 0).
* None of this concerns the cost of the route (item b) nor whether the
  isogeny can be computed; it is existence of a Jacobian in the class.

## 4. Item (a): design

Sampling. Draw the 15 coefficients of `F(x,y,z) = sum_{i+j+k=4} c_{ijk} x^i y^j z^k`
uniformly and independently from `F_q` (all of P^14(F_q), singular quartics
included). Per sample: `N_1` by a monomial table over `P^2(F_q)`; if `N_1 = q+1`,
`N_2` by distinct-root counting of the quartic in `y` over `F_{q^2}` at each
`x` (gcd with `y^Q - y`), summed over Frobenius orbit representatives, plus the
points at infinity; if `N_2 = q^2+1`, a smoothness certificate; if certified
smooth, `N_3` the same way over `F_{q^3}`. Smoothness certificate: the three
pairwise resultants in `y` of the partials `F_x, F_y, F_z` (chart `z = 1`,
interpolated from 10 values over `F_{q^2}`) have gcd 1 and some pair of the
binary cubics at `z = 0` has non-zero resultant; up to 4 random `GL_3`
changes of coordinates are tried. The certificate is sound (a singular
curve cannot pass); a smooth curve that no tried chart certifies is counted
as `uncertified` and excluded from hits (it can only lower hit counts).
Unit tests pin this against brute-force singular-point search over
`F_5, F_25, F_125`.

Weight and mass. The mass of `F_q`-isomorphism classes of smooth quartics
(`sum 1/|Aut|`, twists included) is `s (q^15 - 1)/(q - 1) / |PGL_3(F_q)|`
(`s` = smooth fraction, measured by the `smooth` command from 20 000 random
forms), about `q^6`. Mass of the Jacobians in an isogeny class =
`hits/N x mass`.

Samples (per q): `q=7`: 3 000 000, seed 1007; `q=11`: 3 000 000, seed 1011;
`q=13`: 3 000 000, seed 1013; extension `q=17`: 2 500 000, seed 1017;
`q=19`: 2 500 000, seed 1019. A/A control: `q=7` again with seed 2007 and
3 000 000 samples; the per-`t3` hit counts of the two runs are compared by
`sum (h1 - h2)^2/(h1 + h2)` against its chi-square reference (degrees of
freedom = number of `t3` with `h1 + h2 > 0`); the same instrument, different
seed. Smooth fraction: 20 000 forms, seeds 3000 + q.

Controls.
* **Positive control (the finder finds what exists).** A fixed-seed smooth
  quartic `C*` is constructed (seed `0xC0FFEE xor q`), its `(N_1,N_2,N_3)` are
  computed by the independent streaming brute-force counter, the fast
  counters are required to agree on `C*`, and the search is run for 1 000 000
  samples (seed 2000 + q) for that triple (any smooth quartic with the same
  three counts), `q in {7, 11, 13}`. Pass: at least one hit, and the hit count is
  within a factor of 3 of the rate predicted from the main run's density
  near that class (reported, not gated).
* **Independent recount.** 10 percent of the stored example quartics (up to
  2 per `t3`) of `q <= 13` are recounted by the brute-force counter over
  `P^2(F_{q^k})`, `k = 1, 2, 3`; all must satisfy `N_k` of section 2.
* **Negative control** is the negative group of section 3, run in the same
  samples (the search does not know the target), so the instrument is identical.
* **Counters** (`tests`): fast counters equal brute force for `q in {5, 7}`,
  `k = 1,2,3`; field inverse; smoothness known cases (Klein smooth,
  four concurrent lines and nodal quartic rejected); Theorem 4 cells.

Outputs per run: `census_q<q>.csv` (every `t3` in the Hasse range: `numE`,
primality, constant-type flag, own class, round-2 status, realizable, samples,
hits, rate per 10^6 samples, class-mass estimate, example coefficients),
`search_summary_q<q>.txt`, `summary_groups.csv`, `summary_detail.csv`,
`recount_q<q>.csv`, `control_q<q>.txt`, `smooth_q<q>.txt`, `weil_table.csv`.

Stopping rules. The sample sizes above are fixed; no run is extended
after seeing results. If a positive `t3` has zero hits, no further samples
are drawn; it is reported as not found within N, with its local expectation.

## 5. Item (b): design (parameters fixed by Amendment 1 before its runs)

Group. `Jac(C)(F_q)` of a smooth plane quartic `C : F(x,y)=0` over a prime
`q`, with a rational non-flex point `O` and `F_y(O) != 0`. Classes are
represented as `[D - 3O]` with `D` an effective degree-3 divisor, stored as
a Mumford-like pair `(a(x), b(x))`, `deg a = 3`, `deg b <= 2`, ideal
`(a(x), y - b(x))`. Addition of `[D1-3O]`, `[D2-3O]`: take a cubic `G0`
through `D1 + D2`; its residual `R` (degree 6) is obtained from
`Res_y(G0, F)`/`(a1 a2)` and the `y`-coordinates by a gcd over `F_q[x]/(r)`;
take the cubic `G1` through `R + 3O` (local expansion of `y` at `O`);
the residual of `G1` is `D3 ~ D1 + D2 - 3O` (since `D1+D2+R ~ 3H ~ R+3O+D3`).
Subtraction `D1 - D2`: cubic through `D1 + 3O`, residual `R`, cubic through
`R + D2`, residual `D3`. Negation is subtraction from the identity `3O`.
Doubling uses the Hensel-lifted ideal `(a^2, y - b~)`. Degenerate cases
(shared support, residual with repeated `x`, points at infinity, vanishing
leading coefficients, probability `O(1/q)` per operation) are detected and
handled by re-randomisation (`(a+c) + (b-c)`), and counted.
Correctness checks before any measured run: associativity and
commutativity on random triples, `(a + b) - b = a`, `N * X = 0` for the
brute-force group order `N` at `q = 61`, group order by the Mestre-type
interval search equal to the brute-force `N` at `q in {61, 83}`.

Order and instances. `c1, c2` from `N_1, N_2`; the admissible `c3` interval
from the real-root condition of the real Weil polynomial; baby-step giant-step
on random classes picks the unique `N` with `N X = 0` for 3 random `X`
(lcm check); `N` is factored by trial division; an instance requires a
prime `ell | N`, `ell^2 !| N`, cofactor `h = N/ell <= 2000`. `P = h X`
(order `ell`), `k` uniform in `[1, ell-1]`, `Q = kP` (scalar multiplication).
Curves are random quartics (15 uniform coefficients, `f_{04} != 0`),
drawn until smooth and the instance conditions hold; they are generic quartics,
not the (a) classes; the `Res E` classes are the object of item (a) only.

Index calculus (Diem 2006, "An index calculus algorithm for plane curves of
small degree"; Diem-Thome 2008, "Index calculus in class groups of
non-hyperelliptic curves of genus three", J. Cryptology; implemented from
the published idea, constants not recalled). Unknowns: `x_P = log([P]-[O])`
modulo `ell` for the rational points `P`, `x_h = log([H]-4[O])`. Factor base
`FB`: `B` rational points. Relations: for each pair `P1, P2` in `FB` the line
through them meets `C` in a residual pair `R1, R2` (the quadratic
`c1 s^2 + c2 st + c3 t^2` obtained from the gradient/Hessian of `F` at `P1`,
`P2`); if it splits over `F_q`, `x_{R1} + x_{R2} = x_h - x_{P1} - x_{P2}`.
Double large primes: `R1, R2` need not lie in `FB`; the relations are edges of
a graph on the non-`FB` rational points with `FB` contracted to one ground
vertex. Each non-tree edge of the ground component gives a relation among
`FB` unknowns via the tree potentials. Pairs are processed in a fixed
pseudo-random order until the ground component has `>= B + 1 + 20` independent
cycles. Linear algebra: dense Gaussian elimination modulo `ell` (a sparse
solver is not implemented; counted honestly as dense). Target: `xi = a P + b Q`
walked by additions; accept when the cubic `a(x)` of `xi` splits into three
rational points whose logs are known; two independent such relations give
`log P` and `log Q` up to the common scale, `k = log Q / log P`.
Every answer is verified by `k P - Q = 0` in the group.

Rho. Pollard rho in `<P>`: `r = 32` precomputed multipliers `T_j = alpha_j P + beta_j Q`,
partition by a hash of `a(x)`, distinguished points with 20 expected steps
between them, restarts; and the same with the negation map (canonical sign
by lexicographic order of `(a, b)`), the negation costing one measured
subtraction. Answer verified. Two variants are measured; the one with the
negation map is expected to lose on counted cost because negation is not free
in this group (prediction P-b4).

Counters (same process, same code path for both algorithms):
`M` (F_q multiplications and squarings), `I` (inversions), `adds` (group
additions, subtractions, doublings), native operations (hash probes and
inserts, union-find steps, comparisons, list pushes, sorting steps, modular
multiply-adds in linear algebra modulo `ell` counted separately as `la`).
`GAE` = the measured mean cost of one group addition on the same curve in
`M`-equivalents (`M + I*c_I`), taken from the run's own additions.

Pricing (AGENTS.md section 5a). Total cost per arm in GAE =
`(M + c_I I)/GAE_M + w * native` for `w in {0, 0.1, 1}`, with
`c_I in {3, 2 log2 q}` and `native` the sum of all native counters including
`la`; the same weights on both arms. Two arms are ranked only if the order is
the same at all six `(w, c_I)` cells; otherwise unresolved.

Fit. Log-log least squares of cost against `q` over the measured `q` for each
arm and each pricing cell, with a bootstrap interval over instances; the
exponent for rho is also fitted on steps. The fitted range is a handful of
primes; the exponent is a finite-range slope and is labelled as such.

Predictions (set before the runs).
* **P-b1** rho steps scale as `ell^(1/2)`: slope `1.5 +- 0.15` in `q`.
* **P-b2** index-calculus `M` scale with slope `1.0` to `1.3` in `q`
  (relation search `~ B^2/2 ~ 2 q` line evaluations).
* **P-b3** dense linear algebra grows with slope `1.5`; at 1 GAE per native
  operation it dominates the index-calculus cost, so the index calculus does
  not beat rho at weight 1 in the measured range; at weights 0 and 0.1 the
  order may flip. Stated so that a resolved ranking is not expected.
* **P-b4** rho with the negation map costs more `M` than plain rho.
* **P-b5** a stage-only reading (relation collection) is labelled a stage
  diagnostic and not a speedup in any case.

Scope limits stated in advance: generic quartics only; dense linear algebra;
no claim about the `Res E` classes of item (a), whose group order is
`#E` (prime) and whose quartic Jacobians are found only at `q <= 19`.
A constant in `q` over a few primes does not establish an asymptotic exponent.

## Amendment 1 (2026-10-24, before any measured run of items a or b)

1. Positive control, item (a): `C*` is a generic smooth quartic with a generic
   triple `(N_1, N_2, N_3)`, not a `c1 = c2 = 0` curve, and the main run gives
   no density for a generic triple; the clause "within a factor of 3 of the
   predicted rate" is dropped. Pass = the fast counters agree with the
   independent brute-force counts on `C*` and the search recovers the triple
   at least once; the hit count and the first-hit sample index are reported.
2. `summarize` reports a 95 percent bootstrap interval (4000 resamples over
   `t3`) for each group's mean ratio-to-local and, as the primary H2 contrast,
   the ratio of the mean ratio-to-local of prime-order `t3` with a kernel to
   that of prime-order `t3` without, with its bootstrap interval. The local
   expectation uses all realizable non-constant `t3`; prime-order classes may
   sit below 1 on that scale (nothing is preregistered about it), which is why
   the within-prime contrast is the primary one.
3. The tool sha256 in `env.txt` is of the working-tree file at run time; the
   tool is not edited between the run and the report.

## Amendment 2 (2026-10-24, after the item (a) run started, before any item (b) run)

Item (a)'s run recorded the sha256 of the file as of Amendment 1; this
amendment concerns item (b) only and changes nothing item (a) uses.

Calibration on separate curve seeds (9000 + i; `q = 251` and `1021`, 2 to 3
curves each, outputs discarded, not evidence) changed the index-calculus
design from section 5 as follows, before any measured run:
1. The base point `O` joins the factor base as a pinned point (value 0) in the
   pair enumeration. Reason: every pair relation has four points and `x_h`, so
   `x_P = c, x_h = 4c` solves the homogeneous system and the kernel was
   always at least 2-dimensional; a relation containing `O` removes it.
2. The pair order is a Fisher-Yates shuffle of all `(B+1) B / 2` pairs (a
   mixing-permutation first tried was visibly non-uniform).
3. Pairs are processed until the ground component has at least `2B + 30`
   independent cycles (not `B + 21`): the cycle rows are far from independent
   (rank about 0.7 of the row count in calibration). If the nullspace is still
   not one-dimensional, the attempt is repeated with `B` larger by 100 per
   cent of `sqrt q` (up to 3 times) and the costs of all attempts are summed.
4. `B = ceil(3.00 sqrt q)` initially (calibration: 2.0 and 2.5 failed at
   `q = 1021`, 3.0 worked).
5. Instance condition: cofactor `h <= 1000` (was 2000 in the text; 1000 is
   what the runs use) and `ell^2 !| N`. The rho and index-calculus costs are
   normalised by `sqrt(ell)` where shown, since `ell` varies with `h`.
6. Rho: `r = 32` multipliers `T_j = m_j P + n_j Q` with 16-bit `m_j, n_j`;
   distinguished with probability `2^-5`; after a distinguished point the walk
   continues from it plus a random multiplier (no fresh scalar
   multiplications); walks longer than `40 * 2^5` steps are restarted
   (cycle guard); the step budget is `60 sqrt(ell) + 10^5`.
7. Primes and instance counts: `q = 127, 251, 509, 1021, 2039` with
   `8, 8, 6, 6, 4` instances (curve seed `1000 q + i`, run seed `q`); plain
   rho and the index calculus at every `q`, rho with the negation map at
   `q <= 509` only (its cost is dominated by the extra operation, see P-b4).
8. `GAE` for pricing is the mean `M` and `I` of 100 successful additions of
   random classes on the instance's own curve, measured after the instance is
   built; instance construction (point count, order search, planting) is not
   charged to either arm.
9. Degenerate configurations (about 5 per cent of operations at `q = 251`,
   falling with `q`) are retried with a fresh random cubic and, if still
   degenerate, handled through a random pool element; every attempt's `M`
   and `I` is counted. An element whose own divisor cannot be written as a
   Mumford pair makes the walk restart (`dead` in the rho rows).
10. No sparse linear algebra: the dense elimination modulo `ell` is counted
    (`LA`), so the index-calculus total includes `~B^3` multiply-adds. This is
    the main reason P-b3 expects no resolved ranking at weight 1.

## Amendment 3 (2026-10-24, after the first item (b) run `b-ic-vs-rho-run-20261024T2232Z`, before the second)

The first run finished in about four minutes, far below the budget, and its
rho-with-negation rows were dominated by two-cycles of the r-adding walk on
classes modulo sign (steps per `sqrt(ell)` of 20 to 39 against about 1 to 3 for
plain rho, 4 to 200 restarts per instance): the walk had no cycle escape, so
those rows measure a defective implementation. Changes, fixed before the second run:
1. Rho with the negation map detects a two-cycle (the canonical state equals the
   state two steps earlier) and jumps with a random multiplier; escapes are counted
   (`cycle_escapes`). Plain rho and the index calculus are unchanged.
2. Scope: `q = 127, 251, 509, 1021, 2039, 4093, 8191` with `12, 12, 10, 10, 8, 6, 4`
   instances (same curve-seed rule `1000 q + i`, so the first instances share curves
   with the first run); rho with the negation map for `q <= 2039`.
3. The first run's directory stays as a record; its `ic` and `rho` rows are valid
   and agree in kind with the second run's; its `rhoneg` rows are superseded and
   are not used for P-b4.
Hypotheses and pricing are unchanged. The fitted exponents are reported with
and without the one-off rho set-up (32 multipliers, one start) because that
set-up is a constant of roughly 4 000 additions that dominates rho's total at
small `q`.

## Amendment 4 (2026-10-24, after an aborted second run, before the third)

The second run (`b-ic-vs-rho-run-20261024T2234Z`, Amendment 3's scope) was
stopped by the operator at `q = 509`, curve seed 509009: the planted `Q = kP`
was an element whose scalar multiples could not be formed (every `smul(t, Q)`
returned "not representable"; 199 of 199 probes), and both arms looped on it
(one index-calculus row of 1.4e10 `M` was recorded before the stop). Changes,
fixed before the third run:
1. Instance usability filter: a planted pair `(P, Q)` is accepted only if the
   multiples `3, 5, 7, 11, 13, 17` of `P` and `1009 t` of `Q` are computable;
   otherwise a new `k` (up to 8 times) or a new curve is drawn. The number of
   instances rejected is not recorded per instance (the generator loops), so the
   filter is stated here as a selection rule; it removes pairs on which the
   group-law implementation, not the algorithm, fails.
2. Guards: rho multipliers and starts, and the index calculus' target loop, stop
   after a fixed number of failures (an arm that stops is a failed, unverified
   row and stays in the table).
3. Each instance's rows are appended to the CSV as soon as the instance is done.
4. `summarize` can also report medians over instances (`QIC_STAT=median`); both
   the mean (primary, as preregistered) and the median are reported, because a
   single slow instance moves a mean.
Scope, parameters and hypotheses are those of Amendments 2 and 3.
