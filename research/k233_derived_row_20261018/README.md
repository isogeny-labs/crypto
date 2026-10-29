# K-233 derived row of the admissible-degree inequality (lead L8)

**Class: derived calculation; no `S`, no measured cost.** Every number below
is arithmetic on the formulas of Lemma 2
(`research/ic_leads_20261018/LEMMAS.md`); none is a run. It records where an
algebraic decomposition oracle of Macaulay degree `D` could first fit under
matched rho on K-233. It is not an improvement over the index-calculus
baseline and not a plan.

Calculation: `examples/k233_admissible.rs` (native Rust;
`cargo run --release --example k233_admissible > output.txt`). Layout only:
`plot.py` (matplotlib reads `rows.jsonl`, recorded output; computes nothing).

## Inputs

- Curve: sect233k1 (K-233), `y² + xy = x³ + 1` over `F_{2^233}`, cofactor 4.
- `r = 0x8000000000000000000000000000069d5bb915bcd46efb1ad5f173abdf`, from
  the committed `BinaryCurve::sect233k1` in `src/binary_ecc/curve.rs`
  (SEC 2 value); `log₂ r = 231.0000`.
- Matched rho with Frobenius: `T_ρ = √(π r/(8n))`, `log₂ T_ρ = 110.894`.

## Formulas

Same as the derived table of `dreg_score slope` (`examples/dreg_score.rs`):
`R = 2^ℓ/(2n)`, `A = max(1, m!·2^{n−mℓ})`, `B = T_ρ/(R·A)`; admissible `D` is
the largest `D ∈ 2..40` with `w·log₂ C(N, ≤D) ≤ log₂ B`, for `N_chained =
mℓ + (m−2)n` and `N_elim = mℓ`, `w ∈ {1, 2.2}`. Binomial sums are in the
log domain. One deliberate difference: `dreg_score` takes `r = 2^n/4`; here
`r` is the real order (`log₂ r = 231.0000` against `n − 2 = 231`: the two agree to the
printed digits, so the test reproduces its rows). The unit test
`reproduces_dreg_score_slope_rows` checks the budget and both admissible
degrees on the rows `(n, m, ℓ) = (233, 3, 78)` (40.2; 5; 6) and
`(233, 5, 47)` (67.9; 8; 12) of `dreg_score slope`.
The fold by `2n` assumes a Frobenius-stable base (the point of this row).

## Frobenius-stable bases (computed natively)

`ord_233(2) = 29`. The 2-cyclotomic cosets mod 233: 9 in total, one of size 1
and eight of size 29 (check PASS), so `x^233 − 1` has one linear and eight
degree-29 factors over `F₂`. Stable dimensions: `29a + c`, `a ∈ 0..8`,
`c ∈ {0,1}`: 0 1 29 30 58 59 87 88 116 117 145 146 174 175 203 204 232 233.
Rows below keep `0.6n ≤ mℓ ≤ 1.5n`.

## Table (derived)

See `table.md`; rows as JSON in `rows.jsonl`; full output `output.txt`.

| m | ℓ | m·ℓ | log₂ B | D (chained, w=1) | D (chained, w=2.2) | D (elim, w=1) | D (elim, w=2.2) | D=4 fits (any) |
|--:|--:|--:|--:|--:|--:|--:|--:|---|
| 3 | 58 | 174 | 0.2 | none | none | none | none | no |
| 3 | 59 | 177 | 2.2 | none | none | none | none | no |
| 3 | 87 | 261 | 32.8 | 4 | none | 4 | none | yes |
| 3 | 88 | 264 | 31.8 | 4 | none | 4 | none | yes |
| 3 | 116 | 348 | 3.8 | none | none | none | none | no |
| 4 | 58 | 232 | 56.2 | 7 | 2 | 9 | 3 | yes |
| 4 | 59 | 236 | 59.2 | 7 | 3 | 10 | 3 | yes |
| 4 | 87 | 348 | 32.8 | 3 | none | 4 | none | yes |
| 5 | 29 | 145 | -4.1 | none | none | none | none | no |
| 5 | 30 | 150 | -0.1 | none | none | none | none | no |
| 5 | 58 | 290 | 61.8 | 7 | 3 | 10 | 3 | yes |
| 5 | 59 | 295 | 60.8 | 7 | 3 | 10 | 3 | yes |
| 6 | 29 | 174 | 22.3 | 2 | none | 3 | none | no |
| 6 | 30 | 180 | 27.3 | 2 | none | 4 | none | yes |
| 6 | 58 | 348 | 61.8 | 7 | 2 | 9 | 3 | yes |

`none` = even `D = 2` does not fit. `log₂ B ≤ 0` means the attempts per
relation alone exceed rho.

## Conclusion (derived)

- First `(m, ℓ)` with `D ≥ 4` admissible, ordered by `m·ℓ`:
  - chained `N`, `w = 1`: `m = 4, ℓ = 58` (`mℓ = 232`, `D_adm = 7`);
  - `N = mℓ`, `w = 1`: `m = 6, ℓ = 30` (`mℓ = 180`, `D_adm = 4`);
  - `w = 2.2`, both `N`: none; the largest admissible degree is 3.
- Per m, first stable ℓ with `D ≥ 4` (chained / eliminated, `w = 1`): m = 3:
  87 / 87; m = 4: 58 / 58; m = 5: 58 / 58; m = 6: 58 / 30.
- The stable ℓ do not land on the budget's best point `mℓ ≈ n` for `m = 3`
  (78 is not stable), which costs `m = 3` its fit at `ℓ = 58, 59`.
- Against measurement (`research/dreg_closure_slope_20261018/RESULTS.md`):
  the closure degree is 4 at `mℓ ≤ 18` and above 4 at `mℓ = 21`; every
  listed row has `mℓ ≥ 145`, far beyond any measured cell. Admissibility of
  `D = 4` is therefore not sufficient: the route needs a closure degree at
  most 7 (chained `w = 1`) to 10 (eliminated `w = 1`) at `mℓ ≈ 232–295`,
  and at most 3 under `w = 2.2`, while the closure degree there is
  unmeasured and known only to exceed 4 from `mℓ = 21`.
- Reading (derived): the stable-base fold lifts K-233's budget (`log₂ B`
  up to 61.8 at `m = 5, 6`), but whether a degree-7 oracle exists at
  `mℓ ≈ 232` is unmeasured. The gap to matched rho stays a distance, not
  a measurement. The inequality counts the relation phase only; linear
  algebra and descent are extra, so it is a necessary condition.
- Next: measure the closure degree at the smallest row, `m = 4, ℓ = 58`
  (`mℓ = 232`), against `D ≤ 7`, or find a smaller stable-base `(m, ℓ)`.

Class reminder: relabelling and accounting are not improvements; this row
adds no measured improvement. Unpriced native work (AGENTS.md §5a, 0 / 0.1 /
1 GAE) is not priced here: no cross-method ranking is made.

Figure: `admissible_vs_ell.svg` / `.png` (derived values from `rows.jsonl`;
hollow marker at 1 = none). Report: `report.pdf` (source `report.typ`).
