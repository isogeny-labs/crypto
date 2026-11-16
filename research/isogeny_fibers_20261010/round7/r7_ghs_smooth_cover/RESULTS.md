# R7.1: the smooth GHS descended curve for m >= 2, and R6.9's relation rate redone on it

**Date.** 2026-10-25. **Stacked on** PR #2502 (R6.9, `research/r6-ghs-2`); this branch merges it.
**Code.** `src/cryptanalysis/ghs_smooth.rs` (construction and map), `MumfordDivisor::try_from_unreduced`
in `src/binary_ecc/hyperelliptic.rs`, `examples/r7_ghs_smooth_cover.rs` (selection, verification, measurement).
**Labels.** *measured*: an exact count or a walk statistic printed by the example. *derived*: arithmetic on
measured values (shown). *recalled*: a literature statement not re-checked here. Toy fields only
(N <= 15 here, q <= 8). No claim about any deployed curve and no sub-rho claim.
Conductor was not installed in this container, so no task was reserved.

## 0. Verdict

* **The construction is built and verified on the instances measured.** For `E : y^2 + xy = x^3 + a x^2 + b`
  over `K = F_{2^N}` with `a` in `k = F_{2^l}` the library now builds the smooth imaginary model of the
  GHS descended curve `C/k` and the homomorphism `Phi : E(K) -> Jac(C)(k)` for `m >= 2`
  (R6.9 had only a singular affine model). All 13 curves measured pass every check in section 2.
* **R6.9's smallest weak instance, N = 12, (l, n) = (2, 6), g = 8 over F_4, is now measured on the real
  descended Jacobian** (6 curves with distinct `(a, beta)` orbits).
  * The measured fully-split rate is **0.00064 to 0.00261** per walk step (100000 steps each, 20 pool
    elements, burn-in 20000).
  * It agrees with the exactly enumerated split fraction of the group the walk lives in: ratio
    0.87 to 1.23 (derived), all within 2.1 binomial standard errors.
  * Against Gaudry's 1/g! (recalled) = 2.48e-5 the rate is **26x to 105x** higher (derived). The R6.9
    falsifier (off by more than 10x) therefore **still fires against 1/g!** and does not fire against the
    exact split-divisor model, as in R6.9.
  * **R6.9's stand-in overstated the rate.** The two random genus-8 F_4 curves of R6.9 gave 585x and 858x
    over 1/g! (exact fraction 0.0147 and 0.0170). On the real descended Jacobians the exact fraction is
    0.00074 to 0.00249 (J0, N = 12), 6x to 23x lower than the stand-ins' 0.0147 and 0.0170 (derived).
    The reason is measured: every N = 12 descended curve has only 5 F_4-points (4 affine,
    N_1 = 5), so there are exactly **60** fully split reduced divisors in a group of 24120 to 81200 elements.
    The N = 14 curves have 8 F_4-points (7 affine) and 1408 split divisors, a rate near 0.0108, the
    stand-ins' level.
* **Not done.** `(l, n) = (3, 4)` with `--magic 4` (g = 8 over F_8) did not finish: the run was stopped by me
  after about 25 minutes of wall time without output (a 1000-step re-run was stopped at 280 s as well). The
  cause was not isolated. A guess, unverified, is the exact enumeration of the 400000-element group cap in
  `subgroup()` at g = 8 over F_8. So there is no descended q = 8, g = 8 measurement. The q = 8 rows below
  are g = 2 and g = 4 only.

## 1. What was built

`E` has function field `K(x, s_0)`. The GHS function field is `L = K(x, s_0, ..., s_{n-1})`,
`s_i^2 + s_i = x + a + beta_i / x`, `beta_i = sigma^i(sqrt b)`. The module derives (module doc, items 1 to 6):
the genus-0 quotient by the `-1` involution with parameter `v` and `1/x = P(v)`, `P` an additive polynomial in
`k[v]` fixed by the root space of `Q_B`; the lift of `sigma` with `sigma'(v) = v`; additive Hilbert 90 for the
cover `w' ^2 + w' = 1/P(v) + a + c_1 v`; the imaginary model `Y^2 + P Y = P + P^2 (a + c_1 v)`
(genus `2^r` if `c_1 != 0`, `2^r - 1` if `c_1 = 0`; `r = magic - 1`); and
`Phi(P) = sum_i ([sigma'^i Con(P)] - [W])`, computed with Cantor arithmetic over K and moved to k.
`MumfordDivisor::try_from_unreduced` reduces a divisor of degree above g (a conorm) to its reduced form and
rejects a pair with `u` not dividing `v^2 + hv + f`.

Limits of the library (stated in the module doc, unchanged): `a` must lie in `k`; `N <= 128`; the 2-torsion
point `x_0 = 0` is not mapped; `magic = 1` is the trace map of `ghs_descent::trace_map`.
The `t`-model (`c_1 = 0`, odd genus) was exercised by one curve only (`(2,7)`, `beta = 0x4d`, g = 7); the
other 12 are `v`-models (`c_1 != 0`).

## 2. Verification (all measured; tests are in `ghs_smooth::tests`, the per-curve counts in `relrate_descended.json`)

| Check | Result |
|--|--|
| Library tests `ghs_smooth` (5: genus and smoothness over (2,3), (2,4), (3,3), (2,5) for a in {0,1}; homomorphism; order divides; point counts and Frobenius relation; the N = 12 instance with frozen values) | **5 passed, 0 failed** |
| Existing tests matching `hyperelliptic` or `ghs_descent` (71, which include `binary_ecc::hyperelliptic` 11 and `ghs_descent` 4; 2 ignored) | 71 passed, 0 failed |
| Genus: magic number and `c_1 = 0` iff type I against `ec_trapdoor`, genus `2^(m-1)` or `2^(m-1) - 1` | all curves in the tests; **13/13** measured curves match |
| Genus by point counts: L-polynomial from `N_1..N_g`, then `N_(g+1)` predicted from it and compared with the counted `N_(g+1)` | **13/13** equal. N = 12 target: `N_1..N_9 = 5, 15, 65, 319, 1025, 4761, 16385, 64639, 262145` |
| `Phi` is a homomorphism: `Phi(P) + Phi(Q) = Phi(P + Q)` on 20 random point pairs per curve | **260 ok, 0 failed, 0 skipped** |
| `[#E] Phi(P) = 0` and `#J * Phi(P) = 0` | 156 order checks, **0 failures**; `#J` annihilates the images on 13/13 |
| Order of `Phi(G_l)` for the l-part generator, l = largest prime of `#E` | equals l on 11/13; on the other two (the (3,4), g = 2 curves, `#J` = 72 and 56) l does not divide `#J`, so `Phi(G_l)` is trivial there, which is consistent with the dimension count |
| Frobenius relation: the degree-2n polynomial `x^2n - t x^n + q^n` divides the characteristic polynomial of Frobenius on `Jac(C)` (gcd degree = 2n when g >= n) | gcd degree 12 (n = 6), 14 (n = 7), 8 (n = 4, g = 4), 4 (n = 4, g = 2), mod two primes; **negative control** (trace +-1) gives gcd degree 0 on all 13 |
| Split divisors: the closed-form count of fully split reduced divisors equals the number found by enumerating the group J0 | **13/13**; every closed-form split divisor lies in J0 |

Note `#E | #J` holds when g >= n (the images are then a finite quotient of the Weil restriction) and failed
for the two g = 2 < n = 4 curves, as expected; it is not asserted as a general invariant.

## 3. Relation rate on the real descended Jacobian

**Method (unchanged from R6.9, plus exact enumeration).** `J0` is the subgroup of `Jac(C)(F_q)` generated by the
divisors of all affine `F_q`-points and of `Phi(E(K))`; it is enumerated exactly. Every fully split divisor is a
sum of `F_q`-points, so all of them lie in J0 (checked). Walk A: 20 random elements of J0 as steps, a random
`+r_i` at each step from a random start, burn-in `max(steps/5, 2000)`, then `steps` steps; a step counts if `u`
splits into distinct-or-repeated linear factors over `F_q`. Walk B is the same walk inside
`H = <Phi(P_1..P_12)>` (the subgroup the attack actually sees). `index` is `#J / #J0`.
The binomial s.e. treats steps as independent, which understates the error of a correlated walk.
A first version of walk A (steps built from point sums, no burn-in, 2000 steps) visited only 737
distinct elements and gave 27x over the exact fraction; it was a mixing artefact, was replaced, and is not
reported further.

N = 12, (l, n) = (2, 6), g = 8, q = 4, `magic = 4` (100000 steps per walk). `A` = walk in J0, `B` = walk in H.

| N (l,n) | beta | g | #J | #E | l | J0 (index) | split in J0 | exact J0 | measured A | A / exact | A / (1/g!) | #H | split in H | exact H | measured B | B / exact H |
|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
| 12 (2,6) | 0x28 | 8 | 67072 | 4192 | 131 | 33536 (2) | 60 | 0.00179 | 0.00170 ± 0.00013 | 0.95 | 69 | 2096 | 5 | 0.00239 | 0.00209 | 0.88 |
| 12 (2,6) | 0xe2 | 8 | 63744 | 3984 | 83 | 63744 (1) | 60 | 0.00094 | 0.00116 ± 0.00011 | 1.23 | 47 | 1992 | 3 | 0.00151 | 0.00141 | 0.94 |
| 12 (2,6) | 0x220 | 8 | 83440 | 4172 | 149 | 41720 (2) | 60 | 0.00144 | 0.00166 ± 0.00013 | 1.15 | 67 | 2086 | 3 | 0.00144 | 0.00139 | 0.97 |
| 12 (2,6) | 0x221 | 8 | 48240 | 4020 | 67 | 24120 (2) | 60 | 0.00249 | 0.00261 ± 0.00016 | 1.05 | 105 | 2010 | 5 | 0.00249 | 0.00194 | 0.78 |
| 12 (2,6) | 0x228 | 8 | 81200 | 4060 | 29 | 81200 (1) | 60 | 0.00074 | 0.00064 ± 0.00008 | 0.87 | 26 | 2030 | 1 | 0.00049 | 0.00039 | 0.79 |
| 12 (2,6) | 0x229 | 8 | 50352 | 4196 | 1049 | 50352 (1) | 60 | 0.00119 | 0.00116 ± 0.00011 | 0.97 | 47 | 2098 | 3 | 0.00143 | 0.00144 | 1.01 |

Other instances measured with the same code (so the q = 4, g = 8 result is not an isolated case):

| N (l,n) | beta | g | #J | #E | l | J0 (index) | split in J0 | exact J0 | measured A | A / exact | A / (1/g!) | #H | split in H | exact H | measured B | B / exact H |
|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
| 14 (2,7) | 0x4c | 8 | 130624 | 16328 | 157 | 130624 (1) | 1408 | 0.01078 | 0.01078 ± 0.00033 | 1.00 | 435 | 16328 | 181 | 0.01109 | 0.01129 | 1.02 |
| 14 (2,7) | 0x4d | 7 | 16432 | 16432 | 79 | 16432 (1) | 113 | 0.00688 | 0.00654 ± 0.00025 | 0.95 | 33 | 16432 | 113 | 0.00688 | 0.00703 | 1.02 |
| 14 (2,7) | 0x58 | 8 | 129984 | 16248 | 677 | 129984 (1) | 1408 | 0.01083 | 0.01055 ± 0.00032 | 0.97 | 425 | 16248 | 181 | 0.01114 | 0.01135 | 1.02 |
| 12 (3,4) | 0x8 | 4 | 4144 | 4144 | 37 | 4144 (1) | 280 | 0.06757 | 0.06724 ± 0.00112 | 1.00 | 2 | 1036 | 91 | 0.08784 | 0.08990 | 1.02 |
| 12 (3,4) | 0x16 | 4 | 3984 | 3984 | 83 | 3984 (1) | 280 | 0.07028 | 0.07084 ± 0.00115 | 1.01 | 2 | 996 | 70 | 0.07028 | 0.07136 | 1.02 |
| 12 (3,4) | 0x21 | 2 | 72 | 4176 | 29 | 72 (1) | 40 | 0.55556 | 0.55568 ± 0.00222 | 1.00 | 1 | 36 | 20 | 0.55556 | 0.55322 | 1.00 |
| 12 (3,4) | 0xa2 | 2 | 56 | 4144 | 37 | 56 (1) | 40 | 0.71429 | 0.71250 ± 0.00202 | 1.00 | 1 | 28 | 20 | 0.71429 | 0.71714 | 1.00 |

(3,4) steps: 50000 per walk. The (3,4) g = 4 curves are within 2x of 1/g!, g = 2 within 1.5x, as R6.9 found at
q = 8, g <= 4.

* Walk A vs exact J0: z-scores (measured - exact)/s.e. are -0.68, 2.03, 1.72, 0.76, -1.24, -0.29 for the six N = 12
  curves, and between -1.32 and 0.49 for the other seven (derived from the JSON).
* Walk B in H: the s.e. is binomial on a correlated walk with 1 to 5 split elements in 2000; the z-score of
  `0x221` is -3.94, which I attribute to that, not to the construction (the exact count in H is 5/2010; the walk saw
  fewer). I did not check this.
* The R6.9 comparison uses R6.9's recorded `relrate.json` (not re-run): two stand-in curves at q = 4, g = 8,
  measured 0.0145 and 0.0213, exact fractions 0.0147 and 0.0170.
* The exact split fraction in J (not J0) is half the J0 value where J0 has index 2. The walk and the attack both
  live in J0, so J0 is the right denominator; R6.9 used the whole J, which differs by the index.

## 4. What this does and does not settle

* **Settles (measured, toy sizes):** the library's descended Jacobian reproduces the exact split-divisor model at
  q = 4, g = 8, both for N = 12 and N = 14, and the R6.9 falsifier against 1/g! fires on the descended curve itself,
  at a smaller (26x to 105x) gap than the stand-in's 585x and 858x at N = 12.
* **Does not settle:** (i) any larger N or q (only N <= 14 at q = 4 and N = 12 at q = 8 with g <= 4); (ii) the q = 8,
  g = 8 instance; (iii) odd-genus `t`-models beyond one curve; (iv) `a` outside the base field; (v) that the
  split-divisor fraction in H is the right quantity for an actual attack (the relation also needs the factor-base
  decomposition used in the discrete-log step, which is not run here); (vi) any cost comparison with rho.
  The expected number of group additions per relation, 1 / rate, is 380 to 1560 for the six N = 12 curves
  (derived, walk A), against a rho cost near `sqrt(#H)` ~ 45 in H; this study draws no conclusion from that.
* **Why N = 12 has so few relations (measured fact, not a model):** 5 F_4-points on C, i.e. a factor base of 4 affine
  points. The split count 60 is independent of the walk.

## 5. Files

* `relrate_descended.json`: raw per-curve records of the four runs (commands inside). `census_l2_n6.json`,
  `census_l2_n7.json`, `census_l3_n4.json`: counts of `beta` by (genus, magic, type I) from the `scan` mode.
* `fig1_relrate_descended_vs_standin.{svg,png}`, `fig2_ratio_to_inverse_gfactorial.{svg,png}`, `figure_data.tsv`,
  `plot.py` (plots recorded values only; it also reads `../../round6/r6_9_ghs_2/relrate.json`).
* `report.typ`, `report.pdf`.
* Reproduce: `cargo build --release --example r7_ghs_smooth_cover`, then
  `r7_ghs_smooth_cover run 2 6 --curves 6 --samples 100000 --magic 4 --out relrate.json` (about 12 minutes),
  and likewise for the other three commands in the JSON. `cargo test --release --lib ghs_smooth` for the tests.

Graphs checked and left unchanged: R6.9's `fig2_relation_rate_measured_vs_model` stays as recorded (its curves are
the stand-ins, labelled as such); no scoreboard or `benchmarks/` entry changes, since nothing here measures an
algorithm's speed.
