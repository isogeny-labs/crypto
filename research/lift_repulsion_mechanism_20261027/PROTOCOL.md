# PROTOCOL: mechanism of lift-set repulsion (Kloosterman moments)

Frozen 2026-10-27, before the confirmatory run `reports/run-<UTC>/`.
`FREEZE.sha256` holds the sha256 of this file, `src/main.rs` and `Cargo.toml`
at the moment of freezing. Parent study:
`research/lift_dense_factor_base_20261027/` (`README.md`, `REPORT.typ`
§ Prime degree, repulsion; `src/main.rs`, `summarize/src/main.rs`). Class of
every claim below: **measurement** (exact counts, exact integers) unless
marked **theorem** (proved in `REPORT.typ`) or **exploratory**.

## 0. Disclosure: what was seen before freezing

1. Parent results (given in the task): normalised all-lift 4-subspace counts,
   curves 0.683 / 0.921 / 0.981 and nulls 1.021 / 1.027 / 0.994 at
   N = 9 / 11 / 13, composite N = 12 no difference.
2. `explore` (exact integer Walsh sums, 6 curves each, N = 9, 11, 12, 13):
   Σ W² = q(q−1); Σ W⁴ = 2q³ − 2q² − 3q with the same value on every curve;
   Σ W³ = ∓q(q−2) for odd N (sign (−1)^{Tr a}) and +q(q+2) for N = 12. These
   became identities I1 and I2 below, which were then **proved by hand**
   (see `REPORT.typ`), not only observed.
3. Two `--quick` runs of this crate on N = 9, 11 (6 curves, 6 paired nulls,
   40 extra nulls, a first positive-control design). They showed:
   (a) the parent's ratios are reproduced exactly on the first 6 curves and
   nulls (counts identical to 6 digits);
   (b) the first-order formula of §3 tracks the exact k = 3 and k = 4 ratios
   (R² ≈ 0.9 over all sets at N = 11);
   (c) the raw Walsh 4th moment is NOT correlated with the k = 4 ratio inside
   the nulls (Spearman ≈ −0.09, n = 40), because the 3-point term and the
   density dominate the null scatter. I therefore expect **P2 (within-null
   part) to fail** and report it as such if it does;
   (d) my first positive control (unions of cosets of a subspace H) is not a
   clean control: the counts there are non-perturbative and not monotone in
   the 4th moment. It is kept only as an exploratory, ungated block, and the
   gated positive control became the rank-one Walsh bias of §4.
   The quick-run outputs are not committed and are not scored. The
   confirmatory run uses larger populations (new curves, new nulls) and N = 13.
4. A hand estimate before the run: with the plane correlation fixed by I1 the
   first-order k = 4 deficit of curves is about 105/q, plus a line term
   ±35/q with sign set by Tr(a), against an observed 155–162/q. So I expect
   the first-order formula to explain roughly two thirds of the curve deficit,
   not all of it.

## 1. Setting, notation

E_{a,b}: y² + xy = x³ + ax² + b over F_q, q = 2^N, b ≠ 0, c = √b, u = q − 1.
Lift set L = {x ≠ 0 : Tr(x + a + b/x²) = 0}. ψ(x) = +1 on L, −1 on F_q^* \ L,
ψ(0) = 0. W(w) = Σ_x ψ(x)(−1)^{w·x} (Walsh transform with the bit-dot-product
character; the multiset of values equals that for Tr(wx)). p = |L|/u,
e(x) = 1_L(x) − p for x ≠ 0, e(0) = 0. A_k = number of k-dimensional
F_2-subspaces V with V \ {0} ⊂ L (exact, from the parent's count-mode DFS).
E_k = exact expectation for a uniform random subset of the same size (the
parent's `expect`). Ratio_k = A_k / E_k. kurt = ΣW⁴ / (q·u²) (equals 3 − O(1/q)
for a random ±1 function, 2 − O(1/q) for a Sato–Tate law); the task's
"Σ W⁴/2^{3N}" is also written (`walsh4_over_q3`).

First-order quantities, defined from e (see `REPORT.typ` §2 for the identity):
- D_line = (1/6)·(1/q)Σ_w ê(w)³ − N_lines·ε₃,  N_lines = u(u−1)/6;
- D_plane = (1/24)[(1/q)Σ_w ê(w)⁴ − 3(Σe²)² + 2Σe⁴] − N_planes·ε₄,
  N_planes = u(u−1)(u−3)/24;
- ε_t = E over a uniform |L|-subset of Π_{x∈T}(1_L(x) − p), |T| = t;
- N_k(r) = Gaussian binomial [N−r, k−r]₂ (0 if k < r);
- Δ̂_k = Σ_{t=3,4, t ≤ 2^k−1} p^{2^k−1−t}·D_t·(N_k(r_t) − N_k(t)), r₃ = 2,
  r₄ = 3; predicted ratio = 1 + Δ̂_k/E_k.
Standardised: z_line = D_line/(√N_lines·(p(1−p))^{3/2}),
z_plane = D_plane/(√N_planes·(p(1−p))²).

## 2. Hypotheses and registered predictions

- **I1 (theorem, Walsh 4th moment).** For every curve (every a, every b ≠ 0, every
  N): ΣW⁴ = 2q³ − 2q² − 3q. Checked as exact integer equality on every
  curve of the run. Prediction: 100 % of curves at N = 9, 11, 13 (and 12).
- **I2 (theorem, Walsh 3rd moment).** ΣW³ = −(−1)^{Tr a}·q(q−2) for N odd,
  (−1)^{Tr a}·q(q+2) for N even. Same check.
- **P1.** Curves have a lower normalised Walsh 4th moment than density-paired
  nulls. Per N ∈ {9, 11, 13}: pass iff one-sided permutation p < 0.01
  (100 000 label shuffles of the mean difference, curves = orig + rep curves,
  nulls = their paired nulls) AND mean(null kurt) − mean(curve kurt) ≥ 0.5.
  P1 passes iff all three N pass. Predicted: pass (curve kurt ≈ 2.00, null
  ≈ 3.0).
- **P2.** The normalised k = 4 ratio rises with the Walsh 4th moment.
  Spearman correlation of ratio₄ with kurt, with one-sided permutation p
  (100 000 shuffles, H1: ρ > 0), plus Pearson r with Fisher 95 % intervals.
  Reported separately: (i) pooled curves + all nulls; (ii) within nulls
  (paired + extra); (iii) within curves (kurt is constant by I1, so ρ is
  undefined: reported as such). Pass iff (i) ρ > 0 and p < 0.01 at all three
  N, AND (ii) ρ > 0 and p < 0.05 at at least 2 of 3 N. Arm confounding is
  explicit: (i) alone cannot establish a mechanism, it separates two groups
  whose kurt differs by construction. Predicted: (i) passes, (ii) **fails**
  (§0.3c).
- **P3.** Formula test.
  - **P3a (k = 2, exact).** A₂ = E₂ + D_line for every set, as an exact integer
    identity (the integer form 48·A₂ = u(u−1) + 3s(u−1) + 3(s²−u) + C₃ with
    s = W(0), C₃ = ΣW³/q), and |A₂ − E₂ − D_line| ≤ 10⁻⁴ in floating point.
    Pass iff both hold for every set at N = 9, 11, 13.
  - **P3b (k = 4, first order).** Per N: (α) predicted curve deficit
    (1 − mean pred₄) over observed curve deficit (1 − mean ratio₄) in
    [0.5, 2]; (β) OLS of (ratio₄ − 1) on (pred₄ − 1) over curves + all nulls:
    R² ≥ 0.5 and slope in [0.5, 2]. Pass iff (α) and (β) at all three N.
    Predicted: pass, with (α) ≈ 0.65–0.75.
  - Descriptive (no gate): the same at k = 3; the residual of the curve
    deficit scaled by q and by q²; an exploratory "A₃-aware" k = 4
    prediction that replaces the plane-only rank-3 block by the measured
    A₃ deviation; within-arm dependence of ratio₄ on ratio₃, D_plane,
    D_line, density, z_line, z_plane.
- **Positive control (gated).** A rank-one Walsh bias is planted: L = {x ≠ 0 :
  l(x) = 0} with l a random linear functional, each membership flipped with
  probability η ∈ {0.46, 0.42, 0.38, 0.34, 0.30}, 4 sets per η per N. The
  density stays 1/2 and ΣW⁴ grows with (1−2η)⁴q⁴. Predicted direction: ratio₄
  increases with kurt. Pass per N iff Spearman(kurt, ratio₄) over the 20
  planted sets is > 0 with permutation p < 0.01, AND at η = 0.30 all 4 sets
  exceed (null mean + 3 null sd) in both kurt and ratio₄. Passes iff all
  three N pass. If it fails, P2/P3 verdicts are void (detector blind).
- **Exploratory, ungated:** unions of cosets of a random ℓ-subspace (ℓ = 1..3,
  H excluded), N = 12 composite contrast incl. the count of all-lift
  F₄-subspaces of F₄-dimension 2 and F₁₆-lines (a possible cancelling
  structure), within-arm dependence tables, residual scaling.

## 3. Populations, seeds, instrument

- Field arithmetic, the lift set, the Rng, the paired-null generator and the
  count-mode DFS are copied from the parent (`SEED = 0x11F7_2026_1027_C`,
  same tags). The `orig` population therefore equals the parent's:
  N = 9, 11: 26 curves (2 Koblitz + 24 random), 26 paired nulls; N = 13: 6 + 6;
  N = 12 (descriptive): 6 + 6. Parent ratios are the replication check:
  `jq` comparison of this run's `orig` ratio₄ values with the parent's
  `summary.json` (`kmax.<N>.{curve,null}.count_ratio["4"]`); expected exact
  equality to 10 digits.
- `rep` (fresh, tag `curves_rep`): N = 9, 11: 24 random curves + 24 paired
  nulls; N = 13 and 12: 4 + 4. Curve and null labels are fixed by the
  generator, not chosen by outcome.
- `extra` nulls (tag `nullx_j`): sizes cycle through the curve sizes (orig +
  rep). Count: 3 per curve at N ≤ 11 (150 at N = 9, 11), 4 per curve at
  N = 12, 13 (40 at N = 13).
- Same instrument for every arm: one binary, one code path, exact integer
  counts and Walsh sums; arm is irrelevant to the computation (labels are not
  maskable, but nothing depends on them). Order: interleaved curve/null; the
  results are deterministic and independent of thread schedule.
- Single machine, 3 worker threads, no preemption exposure. Host recorded
  in `reports/run-*/host.txt`.
- Execution route: local, std-only Rust on the executor host, because this is
  an exact counting study of small fields (N ≤ 13), not an x86 CPU benchmark
  and not a timing claim. No isolab-eks submission was made.

## 4. Statistics

- Means, sd, 95 % bootstrap interval of the mean (10 000 resamples).
- Group differences: mean difference with 95 % bootstrap interval, Cohen's d
  (pooled sd), Cliff's delta, one-sided permutation p (100 000).
- Correlations: Spearman (average ranks) with one-sided permutation p and
  Fisher-z 95 % interval (SE × 1.06), Pearson with Fisher interval.
- Regression: OLS slope, intercept, R².
- Permutation RNG: the parent's splitmix generator, tag `stats`, per N.
- Verdicts are computed by the program with exactly the rules above and
  written to `summary.json` → `verdicts`.

## 5. What each outcome means

- P1 + I1 pass: the k = 4 deficit sits on a population with a pinned,
  lower Walsh 4th moment (Sato–Tate 2 vs 3).
- P2 pass AND P3b pass: the 4th moment is a sufficient driver of the ratio.
- P2 fail within nulls, P3b pass: the 4th-moment (4-point) term is the mean
  shift of the curve deficit, but null scatter is carried by other terms
  (3-point correlation, density); this is the outcome I expect.
- P3b (α) outside [0.5, 2]: the first-order correlation terms do not
  account for the deficit and the mechanism is not established.
- Not claimed: anything about N > 13 or k ≥ 5; any ECDLP cost.

## Addendum (post-run, 2026-10-27): execution route

§3 above records a local run with "no isolab-eks submission". AGENTS.md §9a
does not exempt exact counting studies, so the local run is local evidence,
and the §9a confirmatory EKS run of the same binary and seeds is **open**.
The blocker was that this session had no AWS identity or CLI. The text above
is unchanged. See `../lift_dense_followups_20261027/README.md`, "Execution
route".

