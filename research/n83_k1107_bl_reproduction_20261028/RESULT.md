# N83 K1107 baseline against matched precomputed rho: result

**Decision (PROTOCOL.md §4, frozen): precomputed rho is faster online in all three
matched arms.** For each arm, the 95% interval of the median paired ratio
IC online / precomputed-rho online lies entirely above 1. This matches the prediction
frozen in §5. It describes this L0 host only. It is not an improvement, a §8a gate
result, a validation-gate rho reference or a leaderboard row.

Run: `records/run-20261028T1511Z` (run 4, decision-bearing per Amendment 3), source
`de31b7c99`, binaries hashed in `host/binaries.sha256`. The analyzer exited 0 with
`all_scalars_verified_and_agreeing: true`: every arm recovered every one of the 16
scalars, each replayed with the generic library curve, all arms agree, and all match
the hosted reference answers.
Report: [reports/run-20261028T1511Z/report.pdf](reports/run-20261028T1511Z/report.pdf).
Graph: [figures/online-and-ratio.svg](figures/online-and-ratio.svg) (laid out by
[plot.py](plot.py) from `analysis.json` only).

## Online (measured; 16 targets; medians with 10,000-resample 95% bootstrap intervals)

| arm | threads | online median ms [95% CI] | median paired IC / arm [95% CI] | targets where IC slower | §4 reading |
|:--|--:|:--|:--|--:|:--|
| IC K1107/root11 dynamic | 4 (rank), 1 per target | 115.0 [44.6, 379.7] | | | |
| precomputed rho (a), 650 s table walks | 1 | 3.69 [1.99, 4.87] | **40.9** [7.58, 141.9] | 15 / 16 | precomputed rho faster |
| precomputed rho (a′), walks = IC setup wall − 2 s = 149.6 s | 1 | 15.2 [11.9, 30.7] | **4.10** [1.58, 24.8] | 12 / 16 | precomputed rho faster |
| precomputed rho (b), table = IC max RSS 2.03 GB | 4 | 4.21 [2.16, 9.39] | **33.1** [6.60, 74.4] | 15 / 16 | precomputed rho faster |
| plain rho, empty table (positive control only) | 1 | 2,226 [1,561, 2,654] | 0.093 (rho/IC = 10.7 [6.33, 88.8]) | 1 / 16 | does not count (§6) |

The (a′) arm is the closest match, since its table walks used the same wall as IC's
target-independent setup, on one thread against IC's four. It is also the tightest
call: its interval's lower end is 1.58.

## Cold cost (descriptive; derived as setup plus the sum of the 16 recorded online intervals)

| arm | setup wall s | online total s | cold total s | peak RSS |
|:--|--:|--:|--:|--:|
| IC (whole process, 4 threads, 314 s user CPU) | 151.6 | 3.87 | 155.5 | 2.03 GB |
| precomputed rho (a) | 651.0 | 0.07 | 651.0 | 0.32 GB |
| precomputed rho (a′) | 149.8 | 0.42 | 150.2 | 0.08 GB |
| precomputed rho (b) (4 threads, 2,277 s CPU) | 586.2 | 0.11 | 586.3 | 2.04 GB |
| plain rho, empty table | 0 | 36.96 | 36.96 | |

Cold cost is reported beside the decision, not ranked (§4). On this host plain
empty-table rho solved all 16 targets in about a quarter of IC's whole-process wall.

## Operation counts (§5a band)

The median paired IC / precomputed-rho operation-count ratio is below 1 at w = 0 and
above 1 at w = 1 for every arm ((a) 0.0005 / 2.04 / 8.18; (a′) 0.0001 / 0.41 / 1.64;
(b) 0.0009 / 3.76 / 15.0 at w = 0 / 0.1 / 1 GAE). The order does not hold across the
band, so no arm is ranked on operation counts. IC's group-addition count is a lower
bound (4 per relation check plus 1 recovery check), so its w = 0 figure undercounts.

## Context

The 2026-10-16 control found the same direction for K3300 (median paired IC/rho
3.74–40.6) and K600 (81.5–396) on GitHub runners. K1107 here sits at 4.10–40.9.
Hosts differ, so the cross-study numbers are not a paired comparison.
IC online median here is 115 ms against about 63 ms on the 2026-10-21 Tier 1 host.

## Runs 1–3 (kept, not combined)

- `run-20261028T1156Z`: no `/usr/bin/time`; only plain rho ran (Amendment 1).
- `run-20261028T1159Z`: IC and plain rho done; killed by a container restart in arm (a) (Amendment 2).
- `run-20261028T1354Z`: IC and plain rho done; killed by idle container reclaim in arm (a) (Amendment 3).

## Limits

- L0: shared 4-vCPU cloud VM (Intel Xeon @ 2.10 GHz), no PMU, no isolation. The load
  gate passed at start (1-min load 0.21); the end reading (3.54) is the run's own
  4-thread arm (b).
- The 16 points are already public (reproduction panel, not a fresh holdout).
- One run. No replay certificate or validator report, so nothing here enters the
  leaderboard or `benchmarks/README.md` (validation gate).
