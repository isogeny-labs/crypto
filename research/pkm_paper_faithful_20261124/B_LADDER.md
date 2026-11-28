# The smoothness bound `B` at fixed composed degree

Date: 2026-11-24. Driver: `examples/pkm_mixed_chain_f4.rs`. Raw rows:
`b_ladder.jsonl`. All `[pcc:unverified]`.

## Question

PKM §3.4 asks for a chain `L = L_n0 ∘ … ∘ L_1` with `∏ deg L_j ≈ p^{1/m}` and
every `deg L_j` small. The paper's Table 5, and every run in this directory,
take every step of degree 2, which costs `n0 = log2(deg L)` tower levels per
summand — at the density condition, `m·n0 ≈ log2 p` variables. A larger
smoothness bound `B` should buy fewer levels, `n0 ≈ log_B(deg L)`, at a higher
degree per equation. This measures the trade instead of arguing it.

## Setup

`p = 1000003` (20-bit), `deg L = S ≈ 1000 ≈ p^{1/2}`, `m = 2`. For each `B`:
`pkm_smooth_tower::search_aux_exact(p, S)` finds an auxiliary curve with a
point of order `S`; `isogeny_tower_on` builds the mixed-degree Vélu chain;
`relation_system_polys` builds the same relation system the paper defines
(`S₃` plus the chain, denominators cleared); `f4_fp::f4` solves it with a
300 s budget. The search count is recorded, not hidden.

| B | degrees | n0 | vars | max eq degree | D | max cols | wall | verdict | aux curves tried |
|--:|:--|--:|--:|--:|--:|--:|--:|:--|--:|
| 2 | 2×10 | 10 | 20 | 4 | 6 | 61 680 | **>300 s** (timed out) | — | 2 561 |
| 3 | 2,2,3,3,3,3,3 | 7 | 14 | 4 | 7 | 28 853 | 71.6 s | refuted | 967 |
| 5 | 2,2,2,5,5,5 | 6 | 12 | 5 | 8 | 20 395 | 9.3–10.0 s | solved | 908 |
| 7 | 3,7,7,7 | 4 | 8 | 7 | 10 | 13 514 | 2.4–3.1 s | solved | 2 756 |
| 11 | 3,3,11,11 | 4 | 8 | 11 | 13 | 36 591 | **1.1–1.7 s** | solved | 556 |
| 13 | 7,11,13 | 3 | 6 | 13 | 18 | 52 030 | 8.2 s | solved | **57** |

`S` is within 9 % of `p^{1/2}` in every row, so the composed degree is fixed.

## Findings

1. **The variable count follows `log_B`, as predicted.** 20 → 14 → 12 → 8 → 8
   → 6 tower variables for `B = 2, 3, 5, 7, 11, 13`. This is the `m·n0 ≈
   log2 p / log2 B` relation, measured rather than assumed.
2. **The solver cost is U-shaped in `B`, with two orders of magnitude at
   stake.** At `B = 2` the F4 does not finish one system in 300 s; at `B = 11`
   the same composed degree is decided in ~1.3 s; at `B = 13` it climbs again
   to 8.2 s because the solving degree has reached 18. The optimum here is
   interior, around `B = 7–11`.
3. **Precomputation improves too, and faster than the paper's Table 1
   suggests.** The 2-power chain needed 2 561 auxiliary curves; `7·11·13`
   needed 57. Bigger smooth primes are both easier to find and cheaper to
   solve with.
4. `X_R` changes only the coefficients of `S₃`, never its support, so
   `max_cols`, `D` and the wall time are target-independent (verified across
   three targets per row for `B = 5, 7, 11`). One number per configuration is
   the right summary.

## Caveats

* One field size (`p = 2^20`) and one composed degree (`S ≈ 1000`). The
  U-shape's minimum will move with both.
* The solver is the generic `f4_fp` on the `pkm_smooth_tower` system, not the
  `ic bench` harness: this is a solver-cost measurement on the paper's system,
  not an end-to-end ECDLP run.
* The `B = 3` row refuted that target (deriving `1`), which costs more than
  finding univariate elimination polynomials; its wall time is not directly
  comparable to the "solved" rows.
* `pairs_above_bound` is non-zero for some rows (867 at `B = 5`, 494 at
  `B = 7`, 240 at `B = 11`), so those bases are degree-truncated. They still
  reached a 0-dimensional basis (`zero_dim: true`), but the truncation is part
  of the cost picture, not a footnote.

## Extrapolation, stated as extrapolation

`n0 ≈ log2(p^{1/m})/log2 B = log2 p /(m log2 B)`, so at P-192 (`log2 p =
192`) and `m = 2`:

| B | n0 per summand | total tower vars |
|--:|--:|--:|
| 2 | 96 | 192 |
| 7 | 34 | 68 |
| 11 | 28 | 56 |
| 13 | 26 | 52 |
| 2^20 | 5 | 10 |

The engine's measured ceiling is ~14 variables at `D ≤ 7` and 8 variables at
`D = 13`. `B = 11` would put P-192 at 56 variables, with a solving degree
well above the 18 seen at `B = 13` and `n0 = 3`. So `B` narrows the gap by a
factor of ~3.5 in variables and roughly two orders of magnitude in measured
solver time at `p = 2^20` — it does not close it. It is nonetheless the only
knob in the construction that moves in the right direction, and it moves both
the precomputation and the solve.
