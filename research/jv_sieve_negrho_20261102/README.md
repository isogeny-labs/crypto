# Sieve route vs a matched negation-map rho at p = 1511 (frozen 2026-11-02)

**Scope.** The `jv_cover` weak class only (full rational 2-torsion, `#E = 4l`, genus-3 cover over `F_{p^2}`); never prime order. Lead B (L3) of [`../jv_cover_next_leads_20261102/LEADS.md`](../jv_cover_next_leads_20261102/LEADS.md). This is accounting (a matched reference for V2's instances), not an improvement. No leaderboard or `benchmarks/` row.

Labels: **measured** = read from a run record; **derived** = computed from measured values by the Rust tool [`fit/`](fit/); **model** = extrapolation (none here).

Files: [`PROTOCOL.md`](PROTOCOL.md), [`run.sh`](run.sh), [`results/`](results/) (`runs/` 16 records and logs, `replay/`, `inputs/` V2's sparse record copy with sha256, `FROZEN.sha`, `versions.txt`, `walls.txt`, `analysis.out`), [`fit/`](fit/), [`fit.txt`](fit.txt) / [`fit.json`](fit.json) (every statistic), [`table.md`](table.md), [`graph.svg`](graph.svg) / [`graph.png`](graph.png), [`REPORT.pdf`](REPORT.pdf).

## Result (derived)

Reference: **the pooled 142 runs** (the 16 matched runs plus #3278's 126), chosen by the frozen homogeneity rule. Pooled reference means: `s_harness` 0.9180, `s_tuned` 0.3820. Intervals are 95% bootstrap (20000 resamples, jointly over cover seeds and reference runs).

| quantity | seeds 17-24 | all 16 seeds |
|:--|--:|--:|
| **S/rho, U1** (`s_harness`, group additions) | 0.00647 [0.00545, 0.00799] | 0.00655 [0.00581, 0.00746] |
| **S/rho, U2** (`s_tuned`, textbook F_p mults) | 0.01556 [0.01310, 0.01921] | 0.01573 [0.01396, 0.01793] |
| **S+/rho, U1** (S+ over the unpriced rho) | 0.02685 [0.02429, 0.02970] | 0.02679 [0.02436, 0.02954] |
| **S+/rho, U2** (S+ over the unpriced rho) | 0.06452 [0.05837, 0.07137] | 0.06439 [0.05855, 0.07098] |
| S+/(rho S+), U1 (V2's like-for-like) | 0.02261 [0.02045, 0.02501] | 0.02256 [0.02051, 0.02487] |
| S+/(rho S+), U2 | 0.04447 [0.04023, 0.04919] | 0.04438 [0.04035, 0.04892] |

Distance to rho: the sieve route is **below** rho in every cell, by x155 [125, 184] (S, U1), x64 [52, 76] (S, U2), x37 [34, 41] (S+, U1) and x15.5 [14.0, 17.1] (S+, U2) for seeds 17-24 (16 seeds: x153, x64, x37, x15.5, see `table.md`). Against the 16 matched runs only (mean 0.805 vs 0.918) the ratios are about 14% higher: S/rho U1 0.00738 [0.00563, 0.01021] (8 seeds), 0.00746 [0.00588, 0.00996] (16 seeds); all in `fit.txt`. The ratio of arithmetic means (V2's convention) differs from the geometric mean by 4% (0.00671 vs 0.00647, 8 seeds).

**Section 5a band** (primary reference, seeds 17-24; native operations at w = 0, 0.1, 1 GAE on both arms). Layer A is V2's upper bound (cover: sieve adds + lookups + enumeration adds + all unemitted F4/descent/linear-algebra mults counted again as native; rho: six native counters + `fp_adds`); layer B is what is emitted (cover: sieve lookups; rho: the six #3278 native counters).

| layer | unit | w = 0 | w = 0.1 | w = 1 |
|:--|:--|--:|--:|--:|
| A | U1 | 0.0065 [0.0054, 0.0080] | 0.107 [0.097, 0.119] | 0.121 [0.110, 0.135] |
| A | U2 | 0.0156 [0.0131, 0.0192] | 0.117 [0.105, 0.129] | 0.123 [0.111, 0.136] |
| B | U1 | 0.0065 [0.0054, 0.0080] | 0.0093 [0.0085, 0.0103] | 0.0107 [0.0097, 0.0118] |
| B | U2 | 0.0156 [0.0131, 0.0192] | 0.0119 [0.0108, 0.0132] | 0.0110 [0.0100, 0.0122] |

The order (sieve below rho) holds across the band in all four cells, for both seed sets and both references. Layer A is an upper bound on the cover side; the cover emits no F_p additions for its F4, descent and linear-algebra phases, a listed gap (layer A prices those phases' multiplications as native in their place). At w = 1 the ratio is still 0.12-0.14, about 7x below parity.

## Controls

1. **Replay (measured, before launching the 16 runs):** #3278's record `p=1451 seed=1 run=0` rerun with the frozen binary: bit-identical in every field except `wall_ms` ([`results/replay/REPLAY.txt`](results/replay/REPLAY.txt), wall 304 s vs 288 s). The tool also repeats the comparison inside the analysis (`fit.txt`).
2. **Identity (measured):** `l` and `c_add_e` of every new run equal V2's sieve row for the same seed (all 16).
3. **Homogeneity (derived, frozen rule):** steps/sqrt(l), 16 matched runs (mean 0.8056) vs 126 pooled (0.9324): permutation test of the difference of means p = 0.282; KS D = 0.278, permutation p = 0.182; both >= 0.05, so **homogeneous** (not rejected; Kruskal-Wallis permutation p = 0.169, information). The same on `s_harness` (p 0.278 / 0.183) and `s_tuned` (0.277 / 0.184). With 16 runs and a heavy-tailed per-run distribution (sd 0.48) the power is low: "homogeneous" means not rejected. The matched mean is 0.127 below the pooled mean, within its noise (se 0.121).
4. **Positive control (derived):** matched mean 0.8056, se 0.1209, theory 0.88796; z = -0.68, within 3 se: **pass**. #3278's pooled z = +1.15 (information).
5. All 16 runs correct, uncapped, negation, batch 16, r 256, dp-bits 16; none excluded. The tool's selftest on #3278's data reproduces its pooled `s_harness` 0.9322 and `s_tuned` 0.3879 (n = 126).

## Deviations and notes (all stated, none changes a number)

* **Sieve S in the sparse stage as V2 recorded it.** The cover side is V2's records unchanged (no new sieve run); V2's rho records are not used (A4 failed there); V2's rho used dp-bits 15 at p = 1511, these runs use 16 (the #3278 setting).
* **Tool freeze order.** The binary, `PROTOCOL.md` and `run.sh` were hashed before any run; the analysis tool was hashed after the replay control and before any new-run record was read (it was tested only on #3278's data). A later graph-only edit (panel A offset and labels) was re-hashed; `fit.txt` is byte-identical before and after.
* **Replay file.** The #3278 record file holds three seeds; the single seed-1 row was cut out with `jq` (`old_p1451_s1_run0.json`) for the field-by-field comparison; the full file is kept as `old_p1451_allseeds_run0.json`. `run.sh` was launched within seconds of the first replay-comparison attempt (which failed only on the tool's argument count, then passed); the replay run itself had finished before launch.
* **Cut rule not triggered:** both rounds (run 0 and run 1, 16 runs) completed in 2571 s wall (the box was shared; s18 run 0 took 1619 s).
* **Primary statistic.** Geometric mean over cover seeds (as #3278) is primary; V2's ratio of means is in `fit.txt`. Cover native counts in layer A include the `unemitted` upper bound as V2 defines it; the cover's F_p additions outside the sieve and enumeration are not emitted.
* **Pooled reference includes the new runs** (126 + 16 = 142), resampled within its 17 instance groups; the reference intervals are therefore narrow, and the cover-seed spread dominates them.
* The weak class only; the result says nothing about prime-order curves. The distances here are to a negation-map rho at p = 1511 (l = 2^61.4); no extrapolation is made.
