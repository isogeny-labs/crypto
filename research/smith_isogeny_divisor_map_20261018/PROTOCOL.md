# Protocol: Smith's explicit isogeny psi on divisor classes (W3b)

- **Status:** frozen before any measurement data of this study was produced
  (2026-10-18). Changes after this commit are dated amendments below the last
  section; the original text stays.
- **Program:** `research/plane_quartic_program_20261018/PROGRAM.md`, workstream W3b.
- **Related records:** `problems/smith-kernel-isogeny-class-walk.md`,
  `problems/ghs-to-smith-pipeline-characteristic-mismatch.md`,
  `research/smith_prop24_native_20261014/` (the tests of Props. 2 and 4 this
  study builds on).
- **Source:** B. Smith, "Isogenies and the discrete logarithm problem in
  Jacobians of genus 3 hyperelliptic curves", J. Cryptology 22 (2009),
  arXiv:0806.2995, Sections 4 to 7 (Theorem 1, Props. 2 to 4, Lemma 4, eq. (11)).
- **Instrument:** `src/cryptanalysis/smith_isogeny.rs`, driven by
  `examples/smith_isogeny.rs` (native Rust; Python only lays out recorded
  values for figures and the PDF).
- **Scope:** synthetic toy fields only. No deployed-curve claim, no sub-rho
  claim, no key-recovery claim. Nothing here touches prime-field curves or the
  binary standard curves.

## 1. Question

Smith's paper gives the isogeny `phi = (pi_X)_* (pi_H)^* : J_H -> J_X` as
formulas on ideals, but the repository has only the curve `X` and its point
count. Question: **can psi be computed on F_q-rational divisor classes of H
(Mumford form, imaginary model) as an F_q-rational effective divisor on a
smooth plane quartic C ~ X, and does it have, as measured, the properties of
Theorem 1** (homomorphism on classes, kernel exactly S, order-preserving on
the odd part, `#J_X = #J_H`)?

Licensed claims: "psi, as implemented, passes the checks listed in Section 4 on
the instances of Section 3", or the exact list of checks that fail with
counts. Not licensed: anything about instances outside the sampled
distribution, cost beyond the measured `q` range, security of any curve.

## 2. Definition under test (derived from Smith, fixed before data)

Notation: `H: y^2 = F(x)`, `deg F in {7, 8}`, odd prime `q >= 7` for
the tractable-subgroup machinery (`q = 5` is run where the library allows it
and reported separately, otherwise reported as not applicable). `S` a rational
tractable subgroup with a rational trigonal map `g = N/D` in Smith's normal
form (Props. 2 and 4 pass: `lc(s)` a square). `G(t, x) = N(x) - t D(x)`.

* A point of `X` over `t0` is a pair of ideals `{(G(t0, x), y -+ b(x))}`.
* `rho = (d4 w^2 + d2 w + d0) / (8 sigma(t))`, `w = b2^2`, `sigma^2 = s`,
  `lc(sigma)` the square root of `lc(s)` in `[0, (q-1)/2]`. `R` is the
  component `y = b(x)` with `b2 = rho` (eq. (11)); `R'` is the other.
* For an affine point `P` of `H` over `t0 = g(x(P))` (not over `t = infinity`,
  not Weierstrass, fibre unramified): `psi(P) = Q1 + Q2`, the two points of `X`
  over `t0` whose R-triple contains `P`. Extended additively to effective
  affine divisors.
* On classes: for `x` in `J_H(F_q)` and a fixed generic base divisor `A`
  (effective, affine, degree 3, rational), let `D'` be the reduced Mumford
  divisor with `D' ~ x + A` (degree 3, generic). Then
  `Psi_A(x) := [psi(D')] - [psi(A)]` in `Pic^0(X)(F_q)`. This is `phi(x)` with
  no translation and no factor 2. Swapping `R` and `R'` replaces `Psi` by `-Psi`.
* `X` is represented by the plane quartic `C` = image of `Q -> (eta_0 : eta_1 :
  eta_2)`, `eta_i(Q) = sum_k x_k^i D(x_k) / (y_k G'(x_k))` over the fibre
  triple (dual correspondence on differentials). Its 15 coefficients are
  found by F_q-linear algebra on images of points; a nullspace of dimension 1
  is required.
* Output: effective F_q-rational divisors `plus`, `minus` on `C`, stored as
  multisets of closed points (Frobenius orbits, canonical key = minimum over
  Frobenius twists of the normalised coordinates in a fixed `F_{q^d}`), with
  `Psi_A(x) = [plus] - [minus]`, `deg plus = deg minus = 6`.

## 3. Instances and seeds (fixed now)

* Master seed **20261018**; all randomness is `SplitMix64`.
* `q in {7, 11, 13, 17, 19, 23}` for the verification suite (Section 4,
  T1 to T7). For each `q`, scan random squarefree `F` of degree 7 (imaginary
  model, Cantor available) with `smith_prop24::random_curve`-style sampling
  (seed 20261018, curve index `i = 0, 1, ...`) and take the first **6**
  curves that have at least one `(S, g)` with `iso_rational` true. The number of
  curves scanned is recorded (it is the denominator of the pass rate at that
  `q`). For T1 only, the same is done for degree 8 models (first 6 per `q`).
* The `(S, g)` used is the first rational one in the library's order.
* End-to-end (T8): the `m = 3, r = 5` split arm of
  `research/ghs_cubic_hyperelliptic_genus3_20261013/` (imaginary model,
  `deg h = 7`), `p in {7, 11, 13}`, plus `p in {17, 19, 23}` where cheap
  (a 1 h cap per run). All instances of that construction at each `p` (in the
  construction's own enumeration order, seed 20261013) with `ell | #E0(K)`
  prime, `ell >= 5` are tried; for each, the descended `H` is tested by
  Props. 2 and 4. **Every Prop. 2/4 failure is recorded**, and the rate is
  reported against the 18.6% prediction of Smith's Theorem 2. A DLP is run only
  where the test passes.

## 4. Checks, each with its pass criterion (a single failure is reported)

* **T1 Curve.** (a) nullspace of the quartic fit has dimension 1 (more means
  `X` hyperelliptic or too few points: reported separately); (b) hold-out:
  the quartic fitted on the first 40 image points vanishes on all the others;
  (c) `C` smooth (no singular point over the algebraic closure; method:
  common-root test of `Res_y(F, F_x)`, `Res_y(F, F_y)` after a random linear
  change, then an explicit check of any common root); (d) `#C(F_{q^k})` for
  `k = 1, 2, 3` equals `#H(F_{q^k})`; (e) the L-polynomials built from those
  counts coincide, so `#J_X(F_q) = #J_H(F_q)`; (f) for the quadratic twist
  case (`lc(s)` a non-square, Prop. 4 fails) `smith_prop24::x_check` is re-run
  and the twisted relation `N_k(X) = q^k + 1 - (-1)^k s_k(H)` is verified with
  the same counts.
* **T2 Well-definedness.** `Psi_A(x) ~ Psi_{A'}(x)` for 30 random `x` and two
  random base divisors `A, A'` per curve. Equivalent: the class does not
  depend on the choice of `D'`.
* **T3 Homomorphism.** `Psi(x + y) ~ Psi(x) + Psi(y)` for 30 random pairs per
  curve; the right side is formed in `Pic^0(C)` with the checker's own
  addition (T9), the left by Cantor on `H`.
* **T4 Kernel.** For every nonzero `d` in `S(F_q)`: `Psi(d) ~ 0`. For 100
  random `x` per curve outside `S(F_q)`: `Psi(x) !~ 0`. (Kernel as expected:
  `ker psi (F_q) = S(F_q)`.)
* **T5 Order.** For every odd prime `ell | #J_H(F_q)`: a `G` of order `ell`
  (random element times the cofactor); `Psi(G) !~ 0` and `ell * Psi(G) ~ 0`,
  the latter by repeated addition **on `C`**, not by `Psi(ell G)`.
* **T6 Order on J_X.** `#J_X(F_q)` equals `#J_H(F_q)` (from T1) and the
  group order annihilates every tested `Psi(x)` on `C`.
* **T7 Cost.** `F_q` multiplications (an inversion priced at its measured
  multiplication equivalent, calibrated per `q`) and wall time (pinned cores)
  of `psi` per divisor, for `q` in the verification set and
  `q in {101, 1009, 10007, 65537, 1000003, 2^31 - 1}` (200 random divisors
  per `q`, `psi` only, no checker); a linear fit of `log(cost)` on `log(q)`
  reported with its interval. The model expected: polylog in `q`.
* **T8 End-to-end toy.** Planted ECDLP `Q = k G` on `E0(F_{p^3})`, `G` of
  prime order `ell`; transfer `phi` (Cantor, from the construction above), then
  `Psi`, then a DLP in `J_X(F_q)` on canonical keys (brute force or BSGS), then
  `k` verified on `E` by scalar multiplication. Every Prop. 2/4 failure and
  every instance skipped, with the reason, is counted. Success needs `kG = Q`
  checked on `E`.
* **T9 Equivalence engine (the verifier).** Linear equivalence on `C` is
  decided by adjoint (cubic) reduction of degree-6 divisors: for `E` effective
  of degree 6, a cubic `g` through `E` leaves the residual `R` (degree 6); a
  cubic `h` through `R + Omega` (`Omega` a fixed rational degree-3 base
  divisor) leaves `A` of degree 3 with `E ~ A + Omega`. `A` is unique when
  `h^0(A) = 1`; otherwise the class is **exceptional** and the test is skipped
  and counted. The engine itself is validated on `C` before use: (i)
  `canon(E1 + div(f)) = canon(E1)` for known principal divisors (line
  sections `E + E' ~ E'' + E'''` built from rational lines), (ii) class counts
  at `q = 7` by enumeration against `#J_X`, (iii) associativity and
  commutativity of its addition on random triples, (iv) `#J * a ~ 0`.
  If the engine fails (i) to (iv), T2 to T6 are reported as unverified.
* If W1's canonical Pic module (`claude/quartic-pic-canonical-20261018`) has a
  working module near the end, T2 to T6 are repeated with it and the results
  of both verifiers are reported side by side; otherwise this is stated.

## 5. Records and reporting

* JSONL, one row per check instance, in
  `research/smith_isogeny_divisor_map_20261018/data/`, written by the example.
* README + report + graph + PDF per `.claude/skills/research-visuals/SKILL.md`;
  copy to `benchmarks/20261018-smith-isogeny-psi/` for T7.
* Missing capability lines in `problems/smith-kernel-isogeny-class-walk.md` and
  `problems/ghs-to-smith-pipeline-characteristic-mismatch.md` are updated to
  match exactly what passed.
* Unpriced counters of the end-to-end arm (canonicalisation, root finding,
  sorting, hashing) are listed; no IC-versus-rho ranking is made by this
  study (that is W2b).

## Amendments

(none yet)
