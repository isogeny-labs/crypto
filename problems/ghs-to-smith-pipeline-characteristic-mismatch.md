# GHS (characteristic 2) cannot feed Smith's genus-3 trigonal construction

- **Status:** `open`. The characteristic-2 pipeline stays blocked. The
  odd-characteristic alternative now runs end to end at toy `q` (W3b,
  2026-10-25): descent, Props. 2 and 4, Smith's `psi` to a plane quartic and
  a discrete log there, verified on `E0`. Cost at scale is not measured.
- **Kind:**
  - proved obstruction for the pipeline as stated (characteristic 2),
    unchanged;
  - for the odd-characteristic alternative, no characteristic obstruction
    remains and the implementation gap is closed at toy size (Props. 2 and 4,
    the trigonal construction as an explicit `psi`, end-to-end solves; W3b,
    2026-10-25). What is open is empirical: the pipeline's cost at growing
    `q` against matched rho.
- **Affected work:**
  - [weak isogeny classes study, 2026-10-13](../research/weak_isogeny_classes_20261013/REPORT.md),
    section "Where the leads meet";
  - [odd-characteristic descent study, 2026-10-14](../research/odd_char_descent_genus3_20261014/REPORT.md).
- **Next action owner:** unassigned
- **Last updated:** 2026-10-25

## Problem

The proposed end-to-end pipeline does not compose. It reads: walk
`E/F_{2^N}` to a curve whose GHS descent is a genus-3 hyperelliptic curve,
apply Smith's isogeny to a plane quartic, run Diem's attack. The GHS curve
`C` lives over `F_{2^l}`, and Smith's construction is stated and proved only
away from characteristics 2 and 3.

Since 2026-10-14 the odd-characteristic replacement for the first step is
known to exist (see the evidence below). What remains open is whether
Smith's isogeny is `F_q`-rational, and non-degenerate, on its output.

## Evidence and uncertainty

- **Observed (source):** Smith, *Isogenies and the discrete logarithm problem
  in Jacobians of genus 3 hyperelliptic curves*, J. Cryptology 22 (2009),
  arXiv:0806.2995. The introduction says "the trigonal construction (and hence
  our formulae) does not apply in characteristics 2 and 3". All its
  algorithms assume characteristic at least 5.
- **Derived (labeled argument, not machine-checked):** for an ordinary
  genus-3 Jacobian `J` over a field of characteristic 2, `J[2](k̄)` has order
  `2^3`, the 2-rank. It is not `2^6`, so Smith's `(Z/2)^3` kernels inside a
  6-dimensional `J[2]` of Weierstrass-point differences do not exist. The
  étale part of `J[2]` is the kernel of Verschiebung, and its quotient is a
  Frobenius twist of `J`, not a plane-quartic Jacobian. GHS curves are
  hyperelliptic of the form `y^2 + h(x) y = f(x)`, so their Weierstrass
  structure differs from that of `y^2 = f(x)`.
- **Observed (sources, 2026-10-14):** an odd-characteristic descent onto
  genus-3 hyperelliptic curves exists for `n = 3`:
  - Momose–Chao (ECC 2007, slide 24): `E/F_{q^3}: y^2 = e g(x)(x−α)(x−α^q)`,
    `α ∈ F_{q^3} \ F_q`, `g ∈ F_q[x]` of degree 1 or 2. The covering has
    degree 8 and there are `Θ(q^2)` classes.
  - Joux–Vitse (Eurocrypt 2012, §4.1 eq. (3), App. A.1) give the explicit
    `H` over `F_q` and call these "the only" such curves (citing Diem 2003,
    Momose–Chao 2005, Thériault 2003).
  - This holds for any odd `q`, so in particular for `p >= 5`.
  - Diem 2003 itself was **not read**: no copy is reachable from the
    session host. Its content is known here only through the papers that
    cite it.
- **Observed (repository, before this update, not previously linked):**
  - [`research/ghs_cubic_cover_census_20261012`](../research/ghs_cubic_cover_census_20261012/README.md)
    (PR #1759) and
    [`research/ghs_cubic_hyperelliptic_genus3_20261013`](../research/ghs_cubic_hyperelliptic_genus3_20261013/README.md)
    count and build this `m = 3, r = 5` type over `F_q`, `q ∈ {5,7,11,13}`.
  - They check `L(T) = 1 − tT^3 + q^3T^6`.
  - They execute the transfer `E(F_{q^3}) → Jac(C)(F_q)` with Cantor
    arithmetic.
- **Exact computation (2026-10-14, [study](../research/odd_char_descent_genus3_20261014/REPORT.md)):**
  - Joux–Vitse's model passes the L-polynomial check on 81 of 81 instances
    (`p ≤ 31`).
  - The Frobenius cycle type on the eight Weierstrass points of `H` is
    `(3,3,1,1)` for split or linear `h`, giving `#S(H) = 3`
    `F_q`-rational tractable subgroups (Smith, Lemma 1). It is `(6,2)` for
    irreducible `h`, giving `#S(H) = 1`.
  - This is the "third arm" screen. It used the study's own
    distinct-degree factorisation, equivalent to
    `smith_kernel_coverage::cycle_type`, and was checked against Smith's
    Lemma 1 table.
- **Derived (with proof in the study, not machine-checked):**
  - The GHS genus is `1 + 2^{m−2}(r−4)` for every degree-2 `x`,
    independent of `q`.
  - Genus 3 needs `(m, r) ∈ {(2,6), (3,5)}` and a branch-set period of 2
    or 3. So for `F_q(j(E)) = F_{q^n}` with `n >= 4` no odd-characteristic
    GHS cover has genus 3.
- **Exact computation (2026-10-25, [W3b study](../research/smith_isogeny_divisor_map_20261018/README.md)):**
  - Smith's `psi` is computed on Mumford divisors as effective divisors on
    the plane quartic `C ~ X`; on random degree-7 curves at
    `q = 7, ..., 23` it passes well-definedness, homomorphism, kernel `S`,
    order and `#J` checks on two independent `Pic^0` engines with zero
    failures, and at `q <= 13` the enumerated image is exactly `J_H/S`.
  - On the Joux–Vitse split arm, Props. 2 and 4 were run on every descended
    `H` reached (counts per `q` in the study), and where they pass and `X`
    is a plane quartic a planted ECDLP on `E0/F_{q^3}` was solved in
    `J_X(F_q)` and the scalar verified on `E0`.
- **Missing capability (before 2026-10-25):** a native test of Smith's Prop. 2 (rational
  trigonal map) and Prop. 4 (leading coefficient of `s(t)` a square) on this
  family. Also an executed trigonal construction showing that `J_H/S` is a
  smooth plane-quartic Jacobian. The family has three bielliptic
  involutions, so Smith's random-curve hypotheses (1/2 each) do not
  obviously apply. See also
  [`smith-kernel-isogeny-class-walk.md`](smith-kernel-isogeny-class-walk.md),
  which needs the same Prop. 2/4 tests.
- **What this does not establish:**
  - It shows nothing about characteristic 2 or 3.
  - It gives no cost, and it does not show that the pipeline beats rho.
  - It does not show that a plane quartic results.
  - The non-hyperelliptic genus-3 covers (Momose–Chao Types 1 and 2; the
    repository's `m = 2` build) already give plane quartics without Smith.
    Smith matters only for curves whose sole genus-3 cover is hyperelliptic
    (20 at `q = 5` and 42 at `q = 7` in the census).

## What would unblock it

- [ ] A cited construction of a rational isogeny from a genus-3 hyperelliptic
  Jacobian in characteristic 2 to a plane-quartic Jacobian, with a native
  implementation checked on toy curves (point counts equal on both sides).
- [x] Or an odd-characteristic descent producing genus-3 hyperelliptic
  curves (sources above; built in the repository; re-checked 81/81),
  - [x] screened by Frobenius cycle type (`#S(H) = 3` or `1`),
  - [x] and by Smith's Props. 2 and 4, with the trigonal construction
    executed on toy instances and the quartic's `L`-polynomial equal to
    `L_H` (W3b, 2026-10-25: `q = 7, 11, 13` in full and `17, 19, 23` under a
    1 h cap; `#J_X = #J_H = #E0(F_{q^3})` on every quartic reached);
  - [x] Smith's `psi` on divisor classes, verified (T1–T7 of the W3b
    protocol), and a toy end-to-end solve through it (T8), recovered scalar
    checked on `E0`;
  - [ ] the pipeline's cost at growing `q` against matched rho (the DLP
    stage here is brute force; an index calculus on the quartic is W2b).

Either completed route gives an executable pipeline. Neither shows that the
pipeline beats rho on the source curve.

## Next discriminating step

- **Done (2026-10-25):** the step recorded on 2026-10-14 (native Props. 2
  and 4 on the Joux–Vitse family, quotient a smooth plane quartic with
  `L_X = L_H`) ran as part of W3b, together with an explicit `psi` and
  end-to-end toy solves.
- **Action now:** W2b: replace the brute-force dlog in `J_X(F_q)` by an
  index calculus on the quartic (W1 canonical keys, W2a relations) and
  measure the pipeline's end-to-end cost at growing `q` against matched rho
  on `E0(F_{q^3})`.
- **Outcomes:** a crossover estimate with an interval, or "no crossover in
  range"; either is recorded here. The instances where `X` is hyperelliptic
  (19% of the Prop. 4 passes at q = 7, falling to 4% at q = 23, in W3b) are outside this route.
- **Recheck trigger:** a cited char-2 or char-3 trigonal analogue; access to
  Diem 2003.

## Resolution and history

- 2026-10-13: opened from the weak isogeny classes study. Unresolved.
- 2026-10-14: proposal 5 run ([study](../research/odd_char_descent_genus3_20261014/REPORT.md)).
  An odd-characteristic genus-3 hyperelliptic family exists (`n = 3`), so
  the record is **not** closed as a proved obstruction. The cycle-type
  screen is done; Smith's Props. 2 and 4 remain. The characteristic-2
  statement is unchanged.
- 2026-10-25: W3b ([study](../research/smith_isogeny_divisor_map_20261018/README.md)).
  Smith's `psi` implemented on divisor classes and verified on two engines;
  Route S (Joux–Vitse `m = 3, r = 5` descent, Props. 2 and 4, `psi`, dlog in
  `J_X(F_q)`) solved planted ECDLPs end to end at toy `q`, each verified on
  `E0`. Instances where `X` is hyperelliptic are skipped with that reason.
  Open: cost at growing `q` (W2b). The characteristic-2 statement is
  unchanged.
