# R6.9: GHS phase two (isogeny walk cost, relation rate on binary Jacobians)

**Date.** 2026-10-19. **Lead.** R6.9 (round 6, depends on R5.2). **Example.** `examples/r6_9_ghs_phase2.rs`
(R5.2's `r5_2_ghs_class.rs` copied, plus per-search walk records and a `relrate` mode).
**Labels.** *measured*: an exact count printed by the example. *derived*: arithmetic on measured
counts. *recalled*: a literature statement not re-checked here. *model*: a cost or rate model.
Toy fields only (N ≤ 18, q ≤ 32). This lead makes no claim about any deployed curve, and no
sub-rho claim. Conductor was not installed in this container, so no task was reserved.

## 0. Verdict

* **Walk: the hypothesis holds on average but fails for individual searches; the falsifier did not fire.**
  * The mean cost of reaching an LG5 curve (g ≤ 2n, n ≥ 5, R5.2's target) is 0.28–0.67 × 2·(class
    size)^{1/2} isogeny evaluations (measured).
  * The share of searches that exceed 2·√(class size) rises with N: 0 % at N = 12, then 6 %, 25 %,
    14 % and 24 % at N = 14, 15, 16, 18. The worst search costs 4.07× the bound.
  * A log-log fit of evaluations against class size has slope **0.84** (derived, 4 338 searches),
    not 0.5. So the 2√ bound is expected to fail at larger N.
  * Evaluations per curve of class size stay flat or fall: 0.063, 0.057, 0.068, 0.048, 0.048 at
    N = 12, 14, 15, 16, 18. The falsifier "cost grows with N beyond the class size" therefore does
    **not** fire. The walk cost behaves like a near-constant fraction of the class (about 1/(LG5 share)),
    not like √(class size).
  * 18 %–36 % of random strong starts cannot reach any target with ℓ ∈ {3, 5, 7}. The exception is
    N = 15, where every start reached one (0 of 110 failed). These failed starts are excluded from
    all cost figures above.
* **Relation rate: the falsifier fires against the 1/g! model at q = 4 and q = 8; the exact model holds.**
  * The measured smooth (fully split) rate stays within **0.88–3.22×** of the exact split-divisor
    fraction (#split reduced divisors / #J) in all 34 runs. That is within 4×, so the hypothesis
    holds against this model.
  * Against Gaudry's heuristic 1/g! (recalled), the rate is off by more than 10× in 6 of 34 runs:
    q = 4 with g ∈ {6, 7, 8}, and q = 8 with g = 6. That is the stated falsifier.
  * At the **smallest weak instance** the gap is **585× and 858×** (two curves). That instance is
    N = 12, (l, n) = (2, 6), with descended genus g = 8 over F_4: every one of the 53 N = 12
    searches ended at a curve with (g, l, n, m) = (8, 2, 6, 4) (measured).
  * The reason is that 1/g! assumes q ≫ g. With q ≤ g there are too few degree-1 places for that
    limit to apply, and the split fraction is set by the actual point count.
* **This is a stated library limit, so it is not a measurement on the descended Jacobian.** The library has Cantor/Mumford
  arithmetic on imaginary curves y² + h(x)y = f(x) over F_{2^m} (`binary_ecc::hyperelliptic`).
  It cannot construct the GHS descended curve for m ≥ 2:
  * `ghs_descent::descend_m2_affine` gives a singular affine model, and its smooth-model step is a
    documented TODO.
  * `descend_m2_abstract` is symbolic only, and `smooth_model_status()` says so.
  * `prime_hyperelliptic` and `hyperelliptic_index_calculus` (the Gaudry IC) work over F_p only,
    and `jv_cover` is prime-field as well.

  Task 2 therefore measures **random smooth imaginary genus-g curves over the same F_{2^l}** as a
  stand-in for the descended Jacobian. Whether a descended curve with m ≥ 3 is hyperelliptic at all
  is not checked here; it is not hyperelliptic in general (recalled).

## 1. Question, hypothesis, falsifier (PROGRAM.md R6.9)

* **Hypothesis.** The walk costs at most 2·(class size)^{1/2} isogeny evaluations, and the measured
  relation rate at the smallest weak instance is within 4× of the model.
* **Falsifier.** Walk cost grows with N beyond the class size, or the relation rate is off the model
  by more than 10×.

## 2. Task 1: walk from random strong curves to the nearest LG5 curve

**Method.** The isogeny graph, kernels (ψ_ℓ factorisation over F_{2^N}), Vélu codomain, Galois-orbit
representatives and the LG5 target are R5.2's, unchanged. The graph validation is clean at every N
(measured): 0 codomain trace mismatches, 0 asymmetric edges and 0 kernel-orbit errors.

Each search runs BFS from a random strong orbit representative in a mixed class, using at most 50
samples per class. "Isogeny evaluations" counts the Vélu codomains computed at the expanded nodes,
one for each rational kernel found, over every ℓ ∈ {3, 5, 7} at the node. "Kernel searches" is
3 × expanded. Class size counts curves (with multiplicity over the Galois orbit), as R5.2 does.

| N | starts | reached | unreachable | median class size | mean evals | max evals | mean evals / 2√s | median | p90 | max | share > 2√s | mean evals / s |
|--|--|--|--|--|--|--|--|--|--|--|--|--|
| 12 | 65 | 53 | 12 | 84 | 4.9 | 15 | 0.28 | 0.22 | 0.61 | 0.77 | 0.0 % | 0.063 |
| 14 | 616 | 475 | 141 | 168 | 11.0 | 74 | 0.39 | 0.31 | 0.81 | 1.98 | 6.3 % | 0.057 |
| 15 | 110 | 110 | 0 | 255 | 17.2 | 44 | 0.54 | 0.38 | 1.25 | 1.38 | 24.5 % | 0.068 |
| 16 | 701 | 472 | 229 | 448 | 19.7 | 116 | 0.48 | 0.31 | 1.14 | 2.42 | 14.4 % | 0.048 |
| 18 | 5075 | 3228 | 1847 | 756 | 38.5 | 280 | 0.67 | 0.43 | 1.63 | 4.07 | 23.9 % | 0.048 |

All values are measured, except the ratios, which are derived. The log-log slope of evaluations
against class size is 0.84, over all reached searches pooled. Figure: `fig1_walk_cost_vs_class_size.svg`.

**Reached targets** (smallest genus over factorisations with n ≥ 5; measured counts of searches):

| N | (g, l, n, m) |
|--|--|
| 12 | (8, 2, 6, 4) × 53 |
| 14 | (8, 2, 7, 4) × 296, (7, 2, 7, 4) × 141, (7, 1, 14, 4) × 23, (8, 1, 14, 4) × 15 |
| 15 | (15, 1, 15, 5) × 68, (16, 1, 15, 5) × 42 |
| 16 | (16, 2, 8, 5) × 472 |
| 18 | (8, 3, 6, 4) × 3228 |

**Caveat.** The search walks orbit representatives, so it uses each curve's neighbours exactly
once. An attacker working on a concrete curve sees the same neighbour count, because Galois
conjugates are isomorphic as graphs. That attacker also needs Galois-orbit canonicalisation to
avoid revisiting conjugates. Without it, cost can grow by up to the orbit length (≤ N). This
factor is not measured here.

## 3. Task 2: relation rate on binary hyperelliptic Jacobians

**Method.** Each run takes a random smooth imaginary model: f monic of degree 2g + 1 and h of
degree ≤ g over F_q, q = 2^l. It is built with `HyperellipticCurve::try_new`, which checks
smoothness. Curves with fewer than g affine F_q-points are rejected, because random divisors are
built from points.

* **Point counts and #J.** #C(F_{q^k}) is counted exhaustively for k = 1..g, in an extension built
  in the example. The base field is embedded through a root of its modulus. #J = L(1) then follows
  from Newton's identities.
* **Order check.** #J is validated by checking [#J]D = 0 with the library's `scalar_mul` on
  4 random D per curve: 136 checks in total, 0 failures (measured).
* **Walk.** The walk takes 40 000 steps of W ← W + R_j, with j uniform among 20 random divisors R_j.
  Every step is a library `MumfordDivisor::add`. A step is "smooth" when u splits into linear
  factors over F_q, found by exhaustive trial division. This is the Gaudry factor-base relation
  test with the full degree-1 factor base.
* **Exact model.** The number of split reduced divisors is
  [t^{≤g}] (1 + t)^{#ramified} · (1 + 2t/(1 − t))^{#pairs}, divided by #J.
* **Heuristic model.** 1/g! (Gaudry 2000, recalled).

A first version used a hash-driven r-adding walk. It fell into a rho cycle of length about √#J and
gave meaningless rates, so it was replaced with the random step choice and nothing from it is reported.

| l | g | seed | #J | visited / #J | measured | exact model | meas / exact | 1/g! | meas / (1/g!) |
|--|--|--|--|--|--|--|--|--|--|
| 2 | 2 | 1 | 40 | 1.000 | 0.7992 | 0.8000 | 1.00 | 5.00e-01 | 1.6 |
| 2 | 2 | 2 | 31 | 1.000 | 0.8014 | 0.8065 | 0.99 | 5.00e-01 | 1.6 |
| 2 | 3 | 1 | 46 | 1.000 | 0.2557 | 0.2609 | 0.98 | 1.67e-01 | 1.5 |
| 2 | 3 | 2 | 74 | 1.000 | 0.3389 | 0.3378 | 1.00 | 1.67e-01 | 2.0 |
| 2 | 4 | 1 | 516 | 1.000 | 0.3793 | 0.3721 | 1.02 | 4.17e-02 | 9.1 |
| 2 | 4 | 2 | 663 | 1.000 | 0.2010 | 0.1946 | 1.03 | 4.17e-02 | 4.8 |
| 2 | 6 | 1 | 7770 | 0.993 | 0.0459 | 0.0485 | 0.95 | 1.39e-03 | 33.1 |
| 2 | 6 | 2 | 7032 | 0.333 | 0.1135 | 0.0353 | 3.22 | 1.39e-03 | 81.8 |
| 2 | 7 | 1 | 43480 | 0.595 | 0.0192 | 0.0219 | 0.88 | 1.98e-04 | 97.0 |
| 2 | 7 | 2 | 59364 | 0.369 | 0.0707 | 0.0378 | 1.87 | 1.98e-04 | 356.2 |
| 2 | 8 | 1 | 248079 | 0.141 | 0.0145 | 0.0147 | 0.99 | 2.48e-05 | 584.6 |
| 2 | 8 | 2 | 214672 | 0.154 | 0.0213 | 0.0170 | 1.25 | 2.48e-05 | 857.8 |
| 3 | 2 | 1 | 65 | 1.000 | 0.6348 | 0.6308 | 1.01 | 5.00e-01 | 1.3 |
| 3 | 2 | 2 | 64 | 1.000 | 0.6223 | 0.6250 | 1.00 | 5.00e-01 | 1.2 |
| 3 | 3 | 1 | 456 | 1.000 | 0.1718 | 0.1754 | 0.98 | 1.67e-01 | 1.0 |
| 3 | 3 | 2 | 395 | 1.000 | 0.1587 | 0.1595 | 1.00 | 1.67e-01 | 1.0 |
| 3 | 4 | 1 | 4628 | 0.999 | 0.1019 | 0.0972 | 1.05 | 4.17e-02 | 2.4 |
| 3 | 4 | 2 | 4760 | 1.000 | 0.0688 | 0.0674 | 1.02 | 4.17e-02 | 1.7 |
| 3 | 6 | 1 | 419344 | 0.091 | 0.0168 | 0.0184 | 0.92 | 1.39e-03 | 12.1 |
| 3 | 6 | 2 | 275450 | 0.135 | 0.0049 | 0.0047 | 1.04 | 1.39e-03 | 3.5 |
| 4 | 2 | 1 | 205 | 1.000 | 0.4152 | 0.4146 | 1.00 | 5.00e-01 | 0.8 |
| 4 | 2 | 2 | 206 | 1.000 | 0.4766 | 0.4757 | 1.00 | 5.00e-01 | 1.0 |
| 4 | 3 | 1 | 4450 | 1.000 | 0.2154 | 0.2198 | 0.98 | 1.67e-01 | 1.3 |
| 4 | 3 | 2 | 5120 | 1.000 | 0.3038 | 0.3008 | 1.01 | 1.67e-01 | 1.8 |
| 4 | 4 | 1 | 53600 | 0.527 | 0.0238 | 0.0240 | 0.99 | 4.17e-02 | 0.6 |
| 4 | 4 | 2 | 56858 | 0.486 | 0.0314 | 0.0293 | 1.07 | 4.17e-02 | 0.8 |
| 4 | 5 | 1 | 890154 | 0.044 | 0.0056 | 0.0056 | 1.01 | 8.33e-03 | 0.7 |
| 4 | 5 | 2 | 996994 | 0.039 | 0.0070 | 0.0072 | 0.98 | 8.33e-03 | 0.8 |
| 5 | 2 | 1 | 1082 | 1.000 | 0.5358 | 0.5342 | 1.00 | 5.00e-01 | 1.1 |
| 5 | 2 | 2 | 1057 | 1.000 | 0.5158 | 0.5156 | 1.00 | 5.00e-01 | 1.0 |
| 5 | 3 | 1 | 31436 | 0.719 | 0.1746 | 0.1741 | 1.00 | 1.67e-01 | 1.0 |
| 5 | 3 | 2 | 37493 | 0.658 | 0.2268 | 0.2260 | 1.00 | 1.67e-01 | 1.4 |
| 5 | 4 | 1 | 1220560 | 0.032 | 0.0658 | 0.0641 | 1.03 | 4.17e-02 | 1.6 |
| 5 | 4 | 2 | 1078721 | 0.036 | 0.0466 | 0.0464 | 1.01 | 4.17e-02 | 1.1 |

The binomial standard error is ≤ 0.0025 in every run (`relrate.json`).

**Reading.**
* Most of the larger deviations from the exact model come with low coverage (visited/#J ≤ 0.6):
  (2,6) seed 2 at 3.22× with 0.33 coverage, (2,7) seed 2 at 1.87× with 0.37, and (2,8) seed 2 at
  1.25× with 0.15. These are probably steps generated by points spanning a proper subgroup, or slow
  mixing. This was not resolved.
* The 1/g! heuristic is within 0.6–2.4× for q ≥ 8 with g ≤ 4 and for q ≥ 16 (all g run here). It fails badly
  at q = 4 for g ≥ 6.

Figure: `fig2_relation_rate_measured_vs_model.svg`.

## 4. ec_trapdoor::audit_curve

This run did not call `audit_curve`; genus is computed from the magic number, as in R5.2. So no
further discrepancies were tested. R5.2's genus-rule bug still stands.

## 5. Commands

```sh
export CARGO_TARGET_DIR=<scratch>/r6target CARGO_BUILD_JOBS=1
cargo build --release --example r6_9_ghs_phase2
D=research/isogeny_fibers_20261010/round6/r6_9_ghs_2
$CARGO_TARGET_DIR/release/examples/r6_9_ghs_phase2 relrate --samples 40000 --out $D/relrate.json        # 65 s contended
$CARGO_TARGET_DIR/release/examples/r6_9_ghs_phase2 walk 12 14 15 16 18 --samples 50 --out $D/walk.json  # 21 s contended
$VENV/bin/python $D/plot.py   # figures, figure_data.tsv, summary.json from the JSON only
$VENV/bin/python -c "import typst; typst.compile('$D/report.typ', output='$D/report.pdf')"
```

## 6. Limits and next step

* **Limits.**
  * N ≤ 18 and ℓ ≤ 7.
  * The stand-in curves are not the descended Jacobians.
  * The relation rate only covers the decomposition-phase smoothness, not the full index-calculus
    cost (factor base, linear algebra, large primes).
  * All wall times are contended.
* **Next step.**
  * Implement the smooth model of the GHS cover in `src/` (`descend_m2_affine`'s TODO), starting
    with (l, n) = (2, 6) at N = 12. Then repeat task 2 on the real Jacobian.
  * Add ℓ = 9, 11 to test whether the unreachable starts are an ℓ-coverage effect.
