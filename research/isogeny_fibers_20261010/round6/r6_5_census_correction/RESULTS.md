# R6.5 — the finite-size A-pair correction applied back to the §15.2 j = 0 census

## Question
Round-4 lead 16.6 derived the correction `1 + (h−1)(|A|−2)/(2|F|)` to the
m = 2 class factor of an A-closed base when A fixes the cofactor classes, and
predicted +5.6 % for the j = 0 census of §15.2. Does it explain the recorded
ζ-closed c(m = 2) = 1.02 (predicted 1) in `round3/census_j0_*.json`?

## Hypothesis and falsifier (PROGRAM.md R6.5, verbatim in substance)
*Hypothesis.* The recorded ζ-closed c(m = 2) values lie within exact-count
error of `1 + (h−1)(|A|−2)/(2|F|)` with |A| = 3 (or 6), h = 3, |F| = 72, and
the residual after correction is within ±1 %.
*Falsifier.* Residual after correction larger than before, or sign-wrong.

## Which |A| (derived)
For a signed base and h = 3 with ζ trivial on E[3](F_p) (checked on all three
curves: `zeta_fixes_t = true`), negation pairs the two nonzero cofactor
classes, so the number of class-sum-0 pairs is a function of n₀ (base points
in G) alone: `g₂(n₀) = n₀(n₀+1)/2 + ((|F|−n₀)/2)²`. A plain signed base has
n₀ = 2·Bin(|F|/2, 1/3); a ζ-closed one n₀ = 6·Bin(|F|/6, 1/3): same mean,
three times the variance. g₂ is convex with g₂'' = 3/2, so the ζ-closed base
gains `(3/4)·ΔVar = (3/4)(8|F|/9) = 2|F|/3` hits on a baseline of |F|²/6:
ratio `1 + 4/|F|`, i.e. 16.6's formula with **|A| = 6** (negation counts).
Pair-wise: the deterministic hits are {ζᵃP, −ζᵇP}, a ≠ b, whose class
(χ(ζᵃ) − χ(ζᵇ))·cls P vanishes for every P; they need both signs. At
|F| = 72 the prediction is 1.0556 (|A| = 6) or 1.0139 (|A| = 3). This is an
*expectation over bases*: one base's ratio has derived sd 0.121 at |F| = 72
(P(ratio < 1) = 0.315) because it is fixed by that base's n₀.

## Commands
```
export CARGO_TARGET_DIR=<shared r6target> CARGO_BUILD_JOBS=1
cargo build --release --example r6_5_census_correction
R=research/isogeny_fibers_20261010/round3
r6_5_census_correction reanalyse --json $R/census_j0_p18.json,$R/census_j0_p19_s7.json,$R/census_j0_p20_s11.json --out reanalysis.json
r6_5_census_correction census --p 195709 --b 3 --sizes 36,72,144 --reps 200 --seed 1 --out census_p18_sizes.json   # 2m20s contended
venv/bin/python plot_excess.py
```

## Arm 1 — re-analysis of the recorded censuses (no legacy rerun)
The Rust census rebuilt each recorded base from (p, b, ω) and **reproduced
every recorded D₂, D₃ and column count exactly** (6/6 bases, `reproduced =
true`; an earlier draft was off by one on D₃ until the m = 3 hit set took all
splits as the legacy census does).

| curve p | n₀ plain / ζ (reconstructed) | measured c ratio m = 2 | collision-free H₂ ratio from n₀ | measured m = 3 | residual before | after, \|A\| = 3 | after, \|A\| = 6 | z after (\|A\|=6, sd 0.121) |
|--:|:--|--:|--:|--:|--:|--:|--:|--:|
| 195709 (2^18) | 12 / 18 | 0.931 | 0.917 | 1.016 | −0.069 | −0.083 | −0.125 | −1.03 |
| 493159 (2^19) | 30 / 24 | 0.959 | 0.966 | 0.969 | −0.041 | −0.055 | −0.097 | −0.80 |
| 903163 (2^20) | 28 / 30 | 1.021 | 1.019 | 0.966 | +0.021 | +0.007 | −0.034 | −0.28 |
| mean of 3 | | 0.970 | | | −0.030 | −0.044 | −0.085 | SEM 0.070 |

All measured (counts) except the predictions and the sd (derived). The
recorded "1.02" of §15.2 is the p ≈ 2^20 census alone; at 2^18 and 2^19 the
ratio is below 1. Each recorded ratio is reproduced to ≤ 1.5 % by the
collision-free pair count fixed by that base's n₀ — the base's class
composition, not the A-pair term, sets it.

**Falsifier fires on the recorded data:** the mean residual after the |A| = 6
correction (−0.085) is larger than before (−0.030); the ±1 % target is missed
on every curve. Both residuals are within 1.3 SEM of zero under the
composition model: three single bases cannot resolve a 5.6 % effect whose
per-base sd is 12 %.

## Arm 2 — confirmatory exact census, p = 195709, b = 3 (r = 64951)
Same census definition (D_m exact over all of G, signed bases, columns =
orbits of [h]P under ⟨−1, ζ⟩); 200 random plain and 200 random ζ-closed bases
per size plus the deterministic small-x pair; ratios of ensemble means with
95 % bootstrap intervals; the identities `hits + O-sums = g_m(n₀)` held on
all 1206 censuses.

| \|F\| | 1/\|F\| | predicted \|A\| = 6 | predicted \|A\| = 3 | H₂ ratio (measured) | D₂ ratio (measured, §15.2 c) | small-x single base D₂ | H₃ ratio | composition E[g₃] ratio (derived) |
|--:|--:|--:|--:|:--|:--|--:|:--|--:|
| 36 | 0.0278 | 1.111 | 1.028 | 1.130 [1.095, 1.170] | 1.130 [1.094, 1.166] | 1.065 | 1.026 [1.004, 1.052] | 1.016 |
| 72 | 0.0139 | 1.056 | 1.014 | 1.059 [1.043, 1.077] | 1.060 [1.043, 1.077] | 0.931 | 1.001 [0.994, 1.009] | 1.004 |
| 144 | 0.0069 | 1.028 | 1.007 | 1.033 [1.024, 1.044] | 1.032 [1.023, 1.042] | 0.987 | 1.002 [0.998, 1.006] | 1.001 |

The excess scales as 1/|F| with slope ≈ 4: every interval contains the
|A| = 6 prediction and excludes |A| = 3 at |F| = 36, 72, 144. At m = 3 the
raw triple count follows the composition model (excess ≈ 0); distinct D₃ is
saturated at these r (hit rates 0.04 / 0.27 / 0.92) and is not a clean test.
The m = 2 cost factor (U·r/D₂ ratio) is 0.305 / 0.317 / 0.325 against the
1/3 of Prop. 15.2: the correction is a constant-level gain that vanishes as
|F| grows.

Graph: `excess_vs_inv_size.svg` (data `plot_data.json`, script `plot_excess.py`).

## Verdict
**Hypothesis refuted for the recorded census; the correction itself
confirmed.** The A-pair correction with |A| = 6 (derived here; negation
counts) predicts the *ensemble* ζ-closed/plain m = 2 ratio at |F| = 36, 72,
144 within the 95 % intervals and the 1/|F| scaling is seen directly. It
does **not** explain the recorded 1.02: the recorded single-base ratios
(0.931, 0.959, 1.021) are set by each base's cofactor-class composition
(per-base sd 0.12 at |F| = 72), the mean residual grows from −0.030 to
−0.085 after correction (falsifier fires), and both are within noise.

## Proposed erratum for §15.2 of the note
> *Erratum (R6.5).* The ζ-closed m = 2 entry "measured c = 1.02" is one base
> at p ≈ 2^20; the three recorded censuses give 0.93 / 0.96 / 1.02 (mean
> 0.97). A single base's ratio is fixed by how many of its points lie in G
> (sd ≈ 0.12 at |F| = 72) and is not a test of Prop. 15.2. Over 200 random
> bases per size (p = 195709) the ζ-closed/plain m = 2 ratio is 1.130,
> 1.059, 1.033 at |F| = 36, 72, 144 (95 % intervals contain
> 1 + 4/|F|): the finite-size A-pair correction of §17 (16.6) with |A| = 6,
> since the pairs {ζᵃP, −ζᵇP} need negation. "c unchanged by A" in Prop. 15.2
> holds as |F| → ∞; at finite |F| the ζ-closed m = 2 cost factor is
> (1/3)(1 + 4/|F|)⁻¹.

## Limits
Toy curves (p ≤ 2^20) of the recorded census only; one curve for arm 2; no
claim about any deployed curve and no sub-rho claim. Wall time contended.
The correction is a constant; it moves no §15.4 row.

## Next step
Replace the single-base §15.2 rows by ensemble means (all four arms), and
state Prop. 15.2's A-clause with its 1/|F| term.
