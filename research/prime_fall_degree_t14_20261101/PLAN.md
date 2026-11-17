# Plan for the t = 14 cell (frozen before any study row)

Written 2026-11-01. Rows of `runs/` do not exist yet. A pre-flight on scratch output (below) preceded it.

## 1. What is being run, under which procedure

The cell is Kummer, `m = 2`, `P1 = 2013265921`, `t = 14`, the two draws of `research/prime_fall_degree_20261027`
(`--planted 0 --random 2`, default seed), read against the registered prediction C-step `d_F = 4 + ⌊(t−2)/4⌋ = 7`
(Amendment 6). The procedure is Amendment 7 B, a two-cap bracket: cap `c* − 1 = 6` first; if it refutes,
`d_F ≤ 6` and C-step is refuted; if it does not, cap 7; a run stopped by a limit is censored and bounds nothing.

## 2. What this host can and cannot run

Pre-flight (scratch, same binary, this container: 4 vCPU, 15 GB, no AWS access), draw 0, capped F4:

| cell | outcome | wall | peak RSS | widest matrix | dense block |
|:--|:--|--:|--:|--:|--:|
| `t = 11`, cap 5 | no refutation, 30,335 pairs above the bound | 19 s | 390 MiB | 16,572 columns | 5.4·10⁷ entries |
| `t = 12`, cap 5 | identical to `t = 11` | 20 s | 390 MiB | 16,572 | 5.4·10⁷ |
| `t = 13`, cap 5 | identical | 21 s | 393 MiB | 16,572 | 5.4·10⁷ |
| `t = 11`, cap 6 | refutes, `D = 6`, 28 steps | 585 s | 1.6 GiB | 51,909 | 3.1·10⁸ |
| `t = 12`, cap 6 | **censored at the 7,200 s budget**, step 28 | 7,202 s | 8.2 GiB | 103,785 | 1.4·10⁹ |

The cap-5 cost does not depend on `t`. The cap-6 cost does: at least 12× in time, 5× in memory and 2× in matrix
width from `t = 11` to `t = 12`. If those ratios held for two more levels, `t = 14` cap 6 would need at least
about 150× the `t = 12` time (about 12 days at 4 vCPU) and a dense block of at least about 2·10¹⁰ entries (80 GB),
which matches Amendment 7 A's 112 GiB request. This is an extrapolation from one pair of points, not a measurement.

So **cap 6 at `t = 14` is not run here**. It needs the Iso Lab job of Amendment 7 A (112 GiB, 86,400 s budget per
system), which this container cannot reach (AGENTS.md §9a). `run_cells.sh T 6 <k>` is the command for that route.

## 3. Cells run here

- **R, regression** (the options of `run.sh`, as the frozen stage A): `t = 9, 10` uncapped, both draws (expect `D = 5`
  and `6`, recorded medians); `t = 11` cap 5 (no refutation) and cap 6 (refutation, `D = 6`) for both draws.
  Any miss stops the study: the instrument or host does not reproduce the recorded rows.
- **T5**: `t = 14`, cap 5, both draws, `--budget 43200 --max-dense 1600000000`.
- **G**: `pfd_grid_zeros` on the `t = 14` rows (and the `t = 11` rows). A draw with `grid_zeros > 0` is decomposable and
  excluded from every reading (Amendment 7 C).

## 4. Readings, fixed now

- T5 does not refute on a non-decomposable draw (`inconsistent = false`, not censored): `d_F ≥ 6` for that draw. This
  is a lower bound; C-step (7) is neither confirmed nor refuted by it.
- T5 refutes: `d_F ≤ 5` for that draw, C-step is refuted at `t = 14`, and the bracket steps down per Amendment 7 B.
- T5 stopped by `--budget`, `--max-dense` or the memory ceiling: censored, no reading.
- A decomposable draw is excluded and reported.
- Reading of C-step at `t = 14` for Amendment 7 D5: **undecided** unless a cap-6 row exists. This plan produces no
  cap-6 row.

## 5. Route and disclosures

Off-route (host above); `D` is a deterministic function of the system, so the host changes only censoring odds. Rows
here are reported separately from Amendment 7 scoring. `RAYON_NUM_THREADS = 4`, `ulimit -v 14500000`. The t = 12 and
t = 13 cap-5 pre-flight cells touched draw 0 only and are not repeated as study rows. Binary: `pkm_tower_pilot` and
`pfd_grid_zeros` built from the commit recorded in `commit.txt` at run time.
