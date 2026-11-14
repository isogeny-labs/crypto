# DRAFT Amendment R4 (proposed number 16): the 2-isogeny arm at `P1`

**Status: draft, NOT frozen.** Written 2026-11-14 by the main session, on the owner's instruction of that day: "do the isogeny
arm at P1 first". Target when frozen: appended to `research/prime_fall_degree_20261027/PREREGISTRATION.md` as Amendment 16,
**before any isogeny row at `P1` exists**. No such row exists today. Every number is `[pcc:unverified]`.

Disclosure: when this was written, the main session had seen stage A's admitted medians (`results/R9/analysis/pfdA_summary.json`):
Kummer `P1` `t = 1..14`: 3 4 4 4 4 5 5 5 5 6 6 ≥6 ≥6 ≥6. Kummer `P0` `t = 1..8`: 3 4 4 4 4 5 5 5. 2-isogeny `P0` `t = 1..8`:
3 5 5 5 5 5 5 5. The predictions below are written knowing these values. No isogeny row at `P1` has been seen, because none exists.

## 1. Why

- Amendment 1 moved the isogeny arm to `P0 = 786433`, because the builder counts `#E'` with an `O(p)` Legendre loop, and
  `P1 ≈ 2·10⁹` needs many curves. So every Kummer–isogeny comparison in this study is also a comparison between primes.
- At `P0` the isogeny ladder is flat at 5 for `t = 2..8`. Kummer reaches 5 only at `t = 6` and 6 at `t = 10` (at `P1`). Whether
  the isogeny systems stay at 5 beyond `t = 8` is the open question for the isogeny work stream (slide 12 of Petit's PKC 2016
  talk: low-degree rational maps `L_j` with `V = {L(x) = 0}`). `P0` cannot answer it, because `λ ≥ 0.1` beyond `t = 8` there.

## 2. Instrument change (code, `examples/pkm_tower_pilot.rs`)

- New flag `--aux-order naive|mestre`, default `naive`. `naive` is the frozen code path, unchanged byte for byte in its output.
- `mestre` computes `#E'` exactly by Mestre's method on `E'` and its quadratic twist (`#E + #E^d = 2p + 2`). Point orders come
  from baby-step giant-step over the Hasse interval, and the method stops when a single `n` remains. It is unique for `p > 229`.
  It draws its points from a private stream seeded by `(p, a, b)`, never from the cell's stream. So for the same seed the
  builder tries the same curves and selects the same `E'`, `Q`, `R'`, `V` and chain as under `naive`. Rows built this way
  carry `"aux_order_method": "mestre"` in the tower info. No other field changes.
- Nothing else changes: system construction, engines, limits, seeds and the scorer are as frozen.

## 3. Gate G-AUX (new; must pass before any `P1` isogeny row is run)

1. **Exactness.** The unit test `mestre_exact_order_equals_point_count` passes: Mestre equals the naive count on 772 random
   curves at `p ∈ {233, 1009, 40961, 65519, 786433}`.
2. **Same tower.** The unit test `mestre_and_naive_build_the_same_isogeny_tower` passes: for `P0`, `t = 1..5`, seeds 0..2, both
   methods give the same `V`, `c`, `aux_a`, `aux_b` and `aux_order`.
3. **Positive control, row level.** The new binary with `--aux-order mestre` reruns stage A's `P0` isogeny cells
   (`A-m2-tower-p0` and `DA-m2-tower-p0` restricted to `--kinds isogeny`, `t = 1..8`, same flags as `run.sh`). On each of the
   16 draws these must equal the admitted values in `pfdA_summary.json` `draws`: `x_r`, `closure_dF`, `falls`, `ffd`,
   `closure_muladds`, `closure_max_cols`, `D`, `f4_muladds`.
   - Any mismatch stops this amendment's runs. It is reported, not explained away.
   - The rerun is a check, not new data. Its rows enter no reading.

## 4. Cells (Kummer `P1`'s stage A cell set, restricted to `λ < 0.1`)

`λ = |V|^m·2^{m−1}/(m!·p)` at `m = 2` is `4^t / P1`. That gives 0.033 at `t = 13` and 0.133 at `t = 14`. The isogeny arm has
no decomposability certificate (`pfd_grid_zeros` is for Kummer grids), so it runs only where `λ < 0.1`, as in Amendment 1:
`t = 1..13`. Every command adds `--aux-order mestre` and uses `run.sh`'s flags:

| name | engine and limits | cells |
|:--|:--|:--|
| `A-m2-iso-p1` | `hky`, `--hky-cmax 7`, budget 1800 | `--kinds isogeny --m 2 --controls tower --t-min 1 --max-t 10` |
| `A-m2-iso-p1-big` | `hky`, `--hky-cmax 5`, budget 1800 | `--t-min 11 --max-t 13` |
| `DA-m2-iso-p1` | F4 (`--engine tower`), budget 1800, `--max-nnz 2e9` | `--t-min 1 --max-t 13` |
| `CAP-iso-m2-t{11..13}-k{0,1}` | capped F4 at `D − 1` (Amendment 3/5 rule of `cap-A`) | where `DA` gave `D` |
| `X-iso-m2-t{12,13}-k{0,1}-c{5,6,…}` | Amendment 7 B bracket, budget 86,400, `--max-dense` per Amendment 14 | only at a `t` where the steps above leave `d_F` undecided |

Draws, seeds and targets are as frozen: `--planted 0 --random 2`, the pilot's default seed. `p = 2013265921`.

## 5. Readings and predictions (fixed now)

- **§6 as written.** H-LFD, H-FFD and H-gap are applied to the isogeny `P1` ladder exactly as they are to Kummer `P1`.
- **P-ISO-1 (transfer).** At each `t = 1..8`, the isogeny `P1` median `d_F` equals the isogeny `P0` median: 3, 5, 5, 5, 5, 5, 5, 5.
  Refuted if any exact median differs. Kummer showed this kind of prime independence (Kummer `P0` = Kummer `P1` at `t ≤ 8`).
- **P-ISO-2 (the question).** At `t = 9..13` the isogeny `P1` median `d_F` stays at 5 ("flat at 5"). Readings, per `t` and then
  over the range:
  - "flat at 5" if every exact median at `t = 9..13` is 5;
  - "grows" if some exact median exceeds 5, or a censored median is `≥ 6`;
  - "undecided" otherwise.
- **Comparison with Kummer at `P1` (descriptive).** Per `t`, the isogeny median minus the Kummer median, with both draws listed.
  Two draws per cell give no test. A "below Kummer" reading needs an exact isogeny median strictly below an exact or
  lower-bounded Kummer median at the same `t`. It is a description, not an improvement claim. An improvement over the
  index-calculus baseline also needs the relation cost, the factor-base search (`aux_curves_tried`) and the matched rho
  reference (AGENTS.md §5a), and none of those is measured here.

## 6. Route, cost, records

- Execution: the RunPod reserved-pod route of draft R3 (`AMENDMENT_R3_RUNPOD.md`).
- **Binary.** `pkm_tower_pilot` is built from the frozen source f13a690df016d0a457bd40579fbb9d054f486adf plus `gaux/aux_order.patch`
  only (musl static, SHA-256 `705b151b6f35b3fdfd7a6b2ae80630edaf5fb1b7f0b75bf3c81f8e3882aa4888`). It is not built from `main`,
  whose F4 engine has changed since. The other three study binaries are the frozen ones (`runpod/REBUILD.md`).
- **Threads.** `RAYON_NUM_THREADS=12` (Amendment 9). F4's `muladds` counter depends on the thread count (`gaux/GAUX.md`).
- Gate G-AUX result: `gaux/GAUX.md` (pass in `gaux/v3/`, after two failures that are kept on record).
- Cost: owner decision 4 ceilings (US$40 per job, US$400 study).
- Records: resume checkpoint, `bench.record/v1`, director card and receipt, as for every job of this study.
