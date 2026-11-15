# Closure depth in Rust: frozen prediction P1 at t = 10, 11, 12 (2026-11-15)

Status: research note. There is no `pcc` claim, so every number below is `[pcc:unverified]`. One machine, one core.

**Engine.** `examples/hky_depth.rs` (lab E) is a self-contained round-indexed HKY closure with its own sparse echelon. It does not use `hky_fp_tower::closure`, `pkm_tower_pilot`'s builders or the Python code.
- **Instances:** random curve, random coset `g·μ_{2^t}`, and a random `x_R` that is a curve abscissa and not on the grid. A draw is kept only if `S_3 ≠ 0` on all `K²` grid points.
- **Check:** the normal form of `F` is checked against `S_3` at 64 random grid points.
- **Prime:** `p = 2013265921`. A prime this large is what makes inconsistent draws exist at t ≥ 10; the Python closure in `../depth/` needs `p < 2^16`.
- **Definitions:** as in `../depth/README.md`. r* is the round in which 1 enters, G the number of growth rounds.

**Build and run.**
- `cargo build --release -j1 --example hky_depth` (about 21 min on one core; needs network for crates).
- `RAYON_NUM_THREADS=1 target/release/examples/hky_depth --p 2013265921 --t T --c C --idx I [--planted] --out f.jsonl`
- Driver: `run.sh`. Raw rows are in `raw/raw_snapshot.jsonl` and `raw/out_t7_c5_i0.json`.

## Results, m = 2 Kummer

| c | t | r* | G | final dim | draws |
|---|---|---|---|---|---|
| 4 | 2, 3, 4, 5 | 4, 6, 8, 10 (= 2t) | 4, 6, 8, 10 | 16, 57, 163, 386 | 2 each, identical |
| 4 | 6, 7, 8 | — | 11 | 605, 829, 1061 | 2 each, identical |
| 4 | **9, 10, 11, 12** | — | **11** | 1301, 1549, 1805, 2069 | 2 each, identical |
| 5 | 7 | 14 (= 2t) | 14 | 3473 | 1 |
| 4 | 5 (planted) | — | 10 | 384 | 1 |
| 4 | 8 (planted) | — | 11 | 1061 | 1 |
| 5 | 8 (planted) | — | 15 | 6883 | 1 |

## Checks

- **Positive control** `[pcc:unverified]`: `r* = 2t` at c = 4 for t = 2…5, and G = 11 at t = 6…9. The closure dimensions 605, 829, 1061 and 1301 equal the Python values in `../depth/` at t = 6–9. The two codes and their instance generators are independent; the dimensions agree across instances because the closure is instance-independent at these sizes.
- **Negative control:** no planted (consistent) draw ever reaches 1.
- **Frozen prediction P1** (`../depth/PREDICTIONS.md`, committed before these runs: G_t(4) = 11 at t = 8, 9, 10): **held**, and still held at t = 11 and 12. G = 11 on 2/2 draws at every t from 6 to 12, while the dimension grows by about 250 per level.
- **P3** (G_10(5) = 19): queued in the driver (`run.sh`) and not finished when this was written. It will follow in a later commit.

## Not done

- A same-instance comparison with `depth.py`. `crosscheck.py` is written but was not run; it needs Rust rows drawn at p < 2^16.
- c = 5 at t ≥ 9.
- m = 3.
