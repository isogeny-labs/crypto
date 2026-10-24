# Protocol: genus-4 curves and `Res_{F_{q^4}/F_q} E` at `q = 3, 5, 7` (lead 3, `n = 4`), frozen before the first run

Study directory: `research/genus4_n4_census_20261024/`. Branch
`claude/edlyn-teske-trapdoor-variants-eghwfy`. Frozen 2026-10-24 (UTC), repository
HEAD `0fb870d0ee76ba6a1b059e9e34c869518fb412b0`. Follows round 3 item 4
(`research/weil_descent_angles_20261018/`, Theorem 5 and the `q = 3` census) and
round 2 (`research/weil_restriction_isogeny_census_20261018/`, Theorem 4 and the
`n = 4` Tian-kernel counts, Schottky question left open).

Conductor: not run in this agent session; the parent session integrates. This
agent touches only this directory and `examples/genus4_n4_census.rs`.

Tool: `examples/genus4_n4_census.rs`, a new std-only Rust file
(`rustc --edition 2021 -O`; `rustfmt --edition 2021`; `clippy-driver --edition 2021
-D warnings`; unit tests with `--test`). The tool of round 3,
`examples/genus4_res_census.rs`, is not modified.

## 1. Questions

* **Q1 (census).** For `q = 3, 5, 7` and every prime-order `E/F_{q^4}`
  (classified by the trace `t`), is there a smooth non-hyperelliptic genus-4
  curve `C/F_q` with `Jac(C) ~ Res_{F_{q^4}/F_q} E`? How many (isomorphism
  classes, weighted by `1/|Aut|`; hit rates)?
* **Q2 (condition).** Is existence of such a curve (or its abundance) predicted
  by a congruence on `(q mod l, t mod l)` and the Frobenius class on `E[l]`,
  through the existence of a Tian kernel (`F_q`-rational maximal isotropic
  subgroup of `(Res E)[l]` meeting no conjugate factor) for `l in {2, 3, 5, 7}`,
  `n = 4`? Or by an arithmetic function of `t` (class numbers)?
* **Q3 (plane model).** Do the curves found have a degree-5 plane model over
  `F_q`, and what are its singularities?
* **Q4 (controls).** Does the instrument reproduce the round-3 `q = 3`
  results, and does it find nothing where a proved theorem says nothing exists?

## 2. Definitions and the exact reduction

* `Res E` has Weil polynomial `chi_E(T^4) = T^8 - t T^4 + q^4` over `F_q`.
  A genus-4 curve `C/F_q` has `L_C(T) = 1 - t T^4 + q^4 T^8` iff
  `N_k := #C(F_{q^k}) = q^k + 1` for `k = 1, 2, 3` and
  `N_4 = q^4 + 1 - 4t` (Newton's identities: the power sums `p_1 = p_2 = p_3 = 0`,
  `p_4 = 4t`). Equal L-polynomials give isogenous Jacobians (Tate). Call such a
  curve **Res-shaped with trace `t`**. It is a **hit** if moreover an
  elliptic curve `E/F_{q^4}` with trace `t` exists (Waterhouse, exponent `4f`
  even: `p ∤ t`, or `t = ±2q^2`, or `t = ±q^2` with `p ≢ 1 mod 3`, or `t = 0`
  with `p ≢ 1 mod 4`) and `#E = q^4 + 1 - t` is prime. (`t` is then odd.)
* **Parametrisation.** A smooth non-hyperelliptic genus-4 curve is its canonical
  model `V(Q, F) ⊂ P^3`, a complete intersection of a quadric `Q` (unique, hence
  over `F_q`; rank 3 or 4 for a smooth curve) and a cubic `F` modulo
  `Q·(linear forms)` (`20 - 4 = 16` dimensional). For `q` odd there are three
  `PGL_4(F_q)`-classes of such quadrics: split (`x0x3 = x1x2`, `P^1×P^1`),
  non-split (`x0x3 = x1^2 - ν x2^2`, `Res_{F_{q^2}/F_q}P^1`), cone (`x0x2 = x1^2`,
  `P(1,1,2)`). Normal forms of `F`:
  split: `(3,3)`-forms `Σ a_ij x^i y^j`, `a_ij ∈ F_q` (`q^16` forms);
  non-split: the same with `a_ij ∈ F_{q^2}` and `a_ji = conj(a_ij)` (`q^16`
  forms; Hilbert 90 lets one normalise the Frobenius-stability scalar to 1);
  cone: `u^3 + a2 u^2 + a4 u + a6`, `a_d` a binary form of degree `d` (`q^15`
  monic forms; smoothness forces the `u^3` coefficient to be non-zero, scalars
  are absorbed). The mass of a set `S` of forms is `|S| / (s · |G|)` with
  `s = q - 1` (split, non-split) or `1` (cone) and
  `|G| = 2|PGL_2(q)|^2`, `2|PGL_2(q^2)|`, `|GL_2(q)| q^3`; at `q = 3` these
  reproduce the round-3 masses 9475, 9479, 6561 and the total
  `q^9 + q^8 - q^6` of non-hyperelliptic classes (literature count, [L]).
* **Point counts** (exact): split and cone by summing over `P^1(F_{q^k})` the
  number of distinct roots of a binary cubic (table lookup; `u^3 + pu + r`
  tables for `p ≠ 3`, full tables for `p = 3`); non-split for even `k` as split
  over `F_{q^k}`; non-split for odd `k`: points `X ∈ P^1(F_{q^{2k}})`,
  `Y = X^{q^k}`, `F(X, Y) = 0`. Fields are log/Zech tables.
* **Smoothness** (exact, over `F̄_q`): `V(Q,F)` is smooth iff
  `J = (Q, F, 2×2 minors of the Jacobian of (Q,F))` satisfies `J_8 = S_8` (all
  forms of degree 8). Proof: if `V(J) = ∅`, `J` contains a regular sequence of
  degrees 2,3,3,3 (generic elements over `F̄_q`), whose quotient has socle degree
  `1+2+2+2 = 7`, so `S_8 ⊂ J`; conversely a point of `V(J)` makes `J_8 ≠ S_8`.
  The test is a rank computation mod `p` of a 476×165 matrix.
* **Hyperelliptic control curves.** `y^2 = f(x,z)`, `f` a squarefree binary form
  of degree 10 over `F_q` (all hyperelliptic genus-4 models, as in round 3).

## 3. Design

* **Sieve order.** `N_1`, then `N_2`, then (optionally) `N_4` against the set of
  traces for which a prime-order `E` exists, then `N_3`, then `N_4`. Only
  sieve survivors with an allowed trace are tested for smoothness (hits); a
  random sub-sample of at most 2000 other Res-shaped survivors per run is
  tested to measure the smooth fraction.
* **Sampling.** Forms are drawn uniformly (code `splitmix64` stream, seed
  `20261024·10 + q` mixed with the quadric type). The estimator of the mass of
  hits with trace `t` is `H_t / M · q^16 / (s |G|)` (`q^15` for the cone,
  `M` forms sampled), standard error `sqrt(H_t) / M · …`. Exhaustive
  enumeration at `q = 3`. Coverage: a class with `|Aut| = a` has orbit
  fraction `s|G| / (a q^16)`; the expected number of sampled forms per class is
  `M · s |G| / (a q^16)`, reported per `q`.
* **Sizes (fixed now).** `q = 3`: exhaustive, all three quadric types.
  `q = 5`: `M = 5·10^7` forms per type. `q = 7`: `M = 3·10^7` per type (the
  non-split type with the `N_4` pre-filter, which selects the same hits).
  Hyperelliptic control: `q = 3` exhaustive (`3^11` forms), `q = 5, 7`:
  `5·10^7` random forms each. Stopping rule: a run is stopped and reported as
  partial if it exceeds 90 minutes wall clock; sizes are never increased after
  seeing counts without an amendment.
* **Tian kernels.** For each prime-order `t` with `E` existing and each
  `l ∈ {2,3,5,7}`, `l ≠ p`: the `F_l[π]`-module `W[l]`, `π(v_0,…,v_3) =
  (φ v_3, v_0, v_1, v_2)`, `φ` the Frobenius class on `E[l]` with
  `det φ = q^4`, `tr φ = t`, the form `⊕ q̄^i ω`; kernels are `π`-stable
  isotropic 4-subspaces and Tian kernels those meeting `V_0` trivially
  (round 2 definitions; unit-tested against round 2's `n = 3` counts). The
  Frobenius class is determined by `(t, q) mod l` except for a repeated root,
  where scalar versus Jordan depends on `E`: the scalar class is possible iff
  `l^2 | t^2 - 4q^4` (ordinary), both for supersingular `t`; the result is
  `yes`, `no` or `depends`.
* **Plane quintics.** For a hit curve `C` and a rational point `P ∈ C(F_q)`,
  put `P = (0:0:0:1)`, `Q' = y3·L + Q2`, `F' = y3^2 C1 + y3 C2 + C3`; the
  projection from `P` is the plane quintic
  `G = C1 Q2^2 − C2 Q2 L + C3 L^2` with singular points at `L = Q2 = 0`.
  Checked per `(C, P, k)`: `N_k(C)` equals the point count of `G = 0` with the
  node and tangent-direction corrections; the singular points of `G` over
  `F_{q^k}` are exactly `{L = Q2 = 0}`.

## 4. Controls (what each tests)

* **C1 engine.** Unit tests: field axioms; cubic root tables against brute
  force; point counts of all three types against enumeration of
  `V(Q,F) ⊂ P^3(F_{q^k})` (`q = 3`, `k ≤ 3`; `q = 5`, `k ≤ 2`, covers the
  odd-`k` non-split map); the Macaulay smoothness test against a
  singular-point search over `F_9`; orbit sizes divide `|G|`; the Tian code
  against round 2's `n = 3` cells; the plane identity. *Tests the
  implementation, not the mathematics.*
* **C2 hyperelliptic, `q = 3` exhaustive (positive control for the sieve).**
  Must give 104,976 squarefree forms, 1,100 Res-shaped models, trace histogram
  `{-18:18, -10:96, -8:144, -6:96, -2:192, 0:204, 2:180, 4:48, 8:48, 10:48,
  16:24, 18:2}` (round 3, Part H4) and no odd `t`.
* **C3 non-hyperelliptic, `q = 3` exhaustive (positive control for the whole
  pipeline).** Must reproduce round 3's nine prime-order classes:
  split `t=11` (1), non-split `t=-1` (1), cone `t=-7` (3), cone `t=9` (3, one
  with `|Aut| = 3`), cone `t=-1` (1), i.e. mass `8 + 1/3`; the classes are
  recovered by orbit enumeration under the generators of the group.
* **C4 negative control, hyperelliptic (Theorem 5, proved).** For `q` odd, `n ≥ 3`
  a hyperelliptic genus-`n` curve with Jacobian isogenous to `Res E` has `t`
  even. The same sieve on hyperelliptic samples at `q = 5, 7` must give zero
  odd-`t` Res-shaped curves, while even `t` must occur (detector sees the
  effect). Theorem 5 says nothing about non-hyperelliptic curves, and this
  control does not test any statement about them.
* **C5 Waterhouse exclusion.** Res-shaped non-hyperelliptic curves whose trace
  admits no elliptic curve (e.g. `p | t`, `t ∉ {0, ±q², ±2q²}`) exist (round 3:
  32 classes at `q = 3`) but must never be counted as hits; their number is
  reported. Tests that matching L-polynomials is necessary, not sufficient,
  for "isogenous to `Res E`".
* **C6 composite order.** Res-shaped curves with `#E` composite are counted
  separately and are not hits.

## 5. Predictions (before any run)

* **P1** C2 and C3 reproduce exactly (any difference is a tool bug to be
  explained before other results are used).
* **P2** (universal realisation) at `q = 5` and `q = 7` every `t` with an
  elliptic curve of prime order over `F_{q^4}` has at least one smooth hit
  (the expected number of sampled hits per `t` is in the tens to hundreds; a
  `t` with no hit despite that is a negative finding to be reported with its
  coverage). At `q = 3` the round-3 data already show it for all four valid `t`.
* **P3** C4: zero odd-`t` hyperelliptic Res-shaped curves.
* **P4** (no Tian predictor) existence of a Tian kernel at `l ≤ 7` does not
  separate realised from unrealised `t`, because P2 predicts no unrealised
  `t`. The informative response is the mass `μ(t)` (weighted class count) of
  hits per `t`, compared with (a) the number of `l` with a Tian kernel,
  (b) the Hurwitz class number `H(4q^4 − t^2)` (the number of elliptic
  curves over `F_{q^4}` with trace `t`, up to isomorphism, weighted). No
  quantitative model is asserted before the `q = 3, 5` data.
* **P5** (plane model) a hit curve has `N_1 = q+1 ≥ 1` rational points, so
  every one has `q + 1` degree-5 plane models over `F_q` (`L = K − P`); the
  nodes are rational for the split quadric, conjugate over `F_{q^2}` for the
  non-split one, and coalesce into a tacnode for the cone. The identity of
  point counts holds with zero mismatches.

## 6. Holdout

`q = 7` is held out. The census, the Tian table and the mass-versus-class-number
analysis are run for `q = 3, 5` first. Any conjectured rule is written into an
**Amendment** dated and committed to this file before the `q = 7` census is
started, and then tested on `q = 7` with the metric stated in the amendment
(default: Spearman correlation of `μ(t)` with the predictor, and the
`P2` existence check). A rule mined from data is a conjecture until it
passes the holdout, and a pass at one `q` is not a proof.

## 7. What is not claimed

No attack. Isogenous Jacobians exist by Tate, but the map between `Jac(C)` and
`Res E` is not computed, its degree is unknown, and nothing here lowers the
cost of any discrete logarithm. Complexities of index calculus on plane curves
(`Õ(q^{2−2/(d−2)})`, Diem; `Õ(q^{2−2/g})`, Gaudry) are quoted as literature,
not verified. `q = 9` (not prime) is not covered: the tool needs `q` prime.

## 8. Outputs

`results/<item>-run-<UTC>/` with `env.txt` (git HEAD, tool sha256, command,
protocol sha256, rustc, host), CSVs and `summary.txt`; `REPORT.md`,
`REPORT.pdf`, `figures/` (SVG, PNG), `scripts/` (plot, build).

## Amendment 1 (2026-10-24, before the q = 5 census and every Tian/join run)

Runs `ctrlH-q3`, `ctrlK-q3-{cone,split,nonsplit}`, `ctrlH-q5`, `ctrlH-q7` were made
with the tool as frozen. After them, and before any other run, the tool gained,
without touching existing code paths: `hurwitz` (Hurwitz class number),
`haar_density` (Katz-Sarnak density at `T^8 - tT^4 + q^4`, [L]), `spearman`,
parts `JOIN` and `TIANTHEORY`, and the unit tests for them. `TIANTHEORY`
checks Theorem 6 (a closed form for the number of Tian kernels at `n = 4` when
`T^8 - tT^4 + qbar^4` is squarefree mod odd `l`, derived in the report) against
the linear algebra on every cell, `l = 3, 5, 7`. Predictors for P4, fixed now:
(A) `H(4q^4 - t^2)`; (B) Haar density; (C) `B·H/sqrt(4q^4 - t^2)`; (D) number of
`l` with a Tian kernel. Held-out metric: Spearman correlation of the mass `mu(t)`
with each predictor over the ordinary prime-order traces. `q = 7` stays held out
until a conjecture is written as Amendment 2.

Amendment 1b (2026-10-24): `JOIN` accepts a comma-separated list of `mass_by_t.csv`
files (one per quadric type); no other change.

## Amendment 2 (2026-10-24, after the q = 3 and q = 5 data, before any q = 7 census run)

Data seen so far (runs `ctrlK-q3-*`, `classes-q3-*`, `cen-q5-*`, `tian-q3/5`,
`join-q3/5`, `tiantheory`): all four valid prime-order traces at q = 3 and 13 of
14 at q = 5 have hits; the exception is t = 49 (#E = 577, one below the
Hasse bound 50), Haar density 0.0005. A Tian kernel at l = 2 exists for every
prime-order trace (Frobenius class T^2+T+1, unique kernel), so the Tian
predictor (D) has no discriminating power at l = 2. Spearman(mass, predictor) at
q = 5: Haar density 0.775, Haar·H/sqrt 0.775, H 0.565, Tian count 0.29.
(The `join-q7pre` and `tian-q7` runs above used empty census files: they list the
q = 7 traces and predictors only and contain no q = 7 census data.)

**Conjecture (mined from q = 3, 5; a conjecture, not a theorem).**
(R1) A prime-order trace t is realised by a smooth non-hyperelliptic genus-4
curve iff its Haar density D(t) = prod_{j<k}|cos φ_j − cos φ_k| prod_j sin φ_j,
φ_k = (θ+2πk)/4, cos θ = t/(2q²), is not tiny: realised for D >= 0.0143 (the
smallest realised value at q = 5), unrealised at D <= 0.0005 (the unrealised
value at q = 5); in between no claim. (R2) The mass of hits per trace is
rank-correlated with D(t). (R3) Existence of Tian kernels at l <= 7 does not predict
either.
**Predictions for the held-out q = 7 census** (frozen now): the 20 prime-order
traces are t ∈ {-75,-71,-65,-57,-45,-39,-15,-9,3,9,13,19,25,31,45,51,55,61,69,93}.
(P-a) the 19 traces with D >= 0.0143 (all but t = 93, D = 0.00199) each have at
least one smooth hit; t = 93 is in the no-claim band, with a lean towards no hit.
(P-b) Spearman(mass, D) >= 0.5 and |Spearman(mass, Tian count)| < 0.5.
(P-c) the total mass lies within a factor 1.5 of N_t · 11.7 · (7/5)^3.4 with
N_t = 20 (mean mass per trace 2.1 at q = 3 and 11.7 at q = 5 gives the exponent
3.4; a two-point extrapolation, registered as such): 20·11.7·3.14 ≈ 735, i.e.
within [490, 1100]. Sizes: 3·10^7 per type as in section 3 (split and cone without the N_4
pre-filter, non-split with it, seed 20261024·10 + 7).
