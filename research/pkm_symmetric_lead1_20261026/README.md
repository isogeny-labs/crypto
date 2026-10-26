# Lead 1 (bounded D?) on the symmetric PKM tower, and the first m = 4 cells

2026-10-26 · follow-up to [`../pkm_symmetric_tower_20261026/`](../pkm_symmetric_tower_20261026/README.md)
(PR #2632) · instrument [`examples/pkm_symmetric_tower.rs`](../../examples/pkm_symmetric_tower.rs) ·
records `rows.jsonl` (m = 3, sweep t ≤ 7), `rows_n18.jsonl` (m = 3, N = 18 rerun), `rows_m4.jsonl`
(m = 4), `run.log` · graph [`d_vs_n.svg`](d_vs_n.svg) (`plot.py`, recorded values only) · PDF `report.pdf`

**Class: stage diagnostic of the solver axis (AGENTS.md §2). No pipeline is priced, no `S`, no
leaderboard row.** Same prime `p = 786433`, curve, `g`, `ζ`, engine (`f4_fp`, grevlex) and
construction as PR #2632.

## Verdict

__VERDICT__

## m = 3: does D stay at 6?

| N | symmetric D | last productive | widest matrix | time per system |
|--:|--:|--:|--:|--:|
| 6 | 4 | 2 | 35 | < 1 ms |
| 9 | 5 | 3 | 248 | 4 ms |
| 12 | 5 | 3 | 1,224 | 0.4 s |
| 15 | 6 | 4 | 10,532 | 50 s |
__N18ROW__

Rows N = 6–15 reproduce PR #2632 exactly: same D, widths and field-operation counts.

## m = 4: S₅ in the elementary symmetric functions

`S₅(x₁,…,x₄, x_R) = Res_u(S₄(x₁,x₂,x₃,u), S₃(u,x₄,x_R))`, where `S₄ = Res_v(S₃(x₁,x₂,v), S₃(v,x₃,u))`.
It has degree ≤ 8 in each variable. With `x_R` fixed, the instrument evaluates it on a
9⁴ grid (6×6 Sylvester determinants mod p) and recovers it exactly by tensor Lagrange
interpolation. It then rewrites it in `e₁..e₄` with the leading-term algorithm.
**Check:** every cell first evaluates every equation at a planted decomposition. The planted
decomposition uses four distinct summands from `V` and `x_R` computed by chained point
additions. The check passes at N = 12 and N = 16.

| N | arm | D | last productive | widest matrix | time per system |
|--:|--|--:|--:|--:|--:|
| 12 | plain chain | 7 | 6 | 21,619 | 157–159 s |
| 12 | **symmetric** | 8 | **5** | **3,663** | **13 ms** |
| 16 | **symmetric** | 8 | **6** | **13,926** | **18 s** |
| 20 | symmetric | — | — | — | aborted under its 6 GB cap |

Both random targets give identical D, widths and field-operation counts in every cell.

- **D is not the right comparison at m = 4.** The symmetric system's input `S₅(e)` already
  has degree 8, so `D ≥ 8` holds trivially. The plain chain's inputs have degree 4, and its
  D = 7 matches the recorded tower-engine value at N = 12 (encyclopedia §0.1:
  6, 7, 7, ≥ 8 at N = 8, 12, 16, 20).
- **The cost is the comparison.** At N = 12 the symmetric system is 5.9× narrower and about
  12,000× faster per system (158 s against 13 ms). It uses about 2.7·10⁵× fewer field
  operations (1.1·10¹¹ against 4.0·10⁵). Its last productive
  degree is one lower (5 against 6).
- From N = 12 to 16 the symmetric arm's last productive degree rises from 5 to 6, and its cost
  grows by about 2.7·10⁴×.

## What did not run, and why

- **m = 3, N = 18, first attempt.** The kernel's OOM killer stopped it (exit 137) after
  92 CPU-minutes, at about 8.5 GB or more. The operator had started the m = 4 cells
  alongside it, and together they exceeded the container's 15 GB. It was rerun alone
  (`rows_n18.jsonl`) with a 13 GB cap and one random target. Because it starts at
  `--t-min 6`, that target differs from the one a full sweep would draw.
- **m = 3, N = 21** was not attempted after that. It would need more than 13 GB.
- **m = 4, N = 8** cannot be planted. `V` has 4 points, and 200,000 draws found no ordered
  4-subset whose chained sum gives an `x_R`. The instrument skips such a cell instead of
  measuring it without its check.
- **m = 4, N = 16, plain chain** was not run. At N = 12 it already takes 158 s with 21,619
  columns.
- **m = 4, N = 20, symmetric:** allocation failure under the 6 GB cap (exit 134).
- The m = 4 times were measured while the m = 3 N = 18 cell held all four cores. They are
  upper bounds. D, widths and field-operation counts do not depend on load.
- The resume checkpoint could not be pushed: the container has no S3 token.

## Reading

__READING__
