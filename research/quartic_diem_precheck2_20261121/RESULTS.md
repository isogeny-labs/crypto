# Results: pre-check 2 for lead #1 (Diem line IC on prime-order plane quartics, fixed instrument)

Every number here is `[pcc:unverified]`. It comes from one session on a shared host, with
no replay certificate and no VALIDATION.json.

- Protocol: `PROTOCOL.md`, frozen before any measured run; `frozen.sha256` holds the
  hashes of `PROTOCOL.md` and `analyze.py`.
- Source: `source.sha256`. Binary: `binary.sha256` (`genus_ladder.qdiem2`).
- Raw data: `raw/run.jsonl` and `raw/run65537_{odd,even}.jsonl`, hashed in `data.sha256`.
- Analysis: `analyze.py`, which writes `analysis.json`.
- Figure: `plot.py`, which writes `fig_scaling.svg` and `fig_scaling.png`.
- Post-hoc exploration, not in any fit: `explore/`.

## Verdict

**PROCEED-TO-LADDER under the frozen rule. Neither K1 nor K2 fires.** This verdict is not
a success claim and contains no rho comparison.

| rule | statistic (arm `Q_diem_2lp_new`, 7 primes, 28 curves, 56 runs) | value | fires? |
|---|---|---|---|
| K1 | `la_mul` exponent, point estimate > 1.40 | **0.971** (boot95 [0.919, 1.024]) | no |
| K2 | FPM mid exponent, upper boot95 >= 1.40 | **0.897** (boot95 [0.858, **0.934**]) | no |

No sensitivity reading would flip the verdict. The six-prime fit gives `la_mul` 1.018
[0.933, 1.099] and FPM mid 0.898 [0.833, 0.957]. The t-intervals on the per-prime means
are [0.854, 1.088] and [0.792, 1.002]. `la_mul + la_add` gives 1.181 [1.130, 1.230]. FPM
low gives 0.870 and FPM high 0.882. GAE gives 0.638 at w = 0, 1.063 at w = 0.1 and 1.064
at w = 1.

The pre-check-1 instrument (`Q_diem_2lp`) on the same 7 primes would stop again. Its FPM
mid exponent is 1.329 [1.236, **1.419**], and it reproduces pre-check 1 exactly (check C0).
The stop in pre-check 1 therefore came from the instrument: the linear-scan descent and
the structurally singular first solve.

## Grid as run

- Primes 1031 ... 32771 used the 24 prime-order curves of pre-check 1 (`--quartic-seeds`,
  same admission path). At 65537, the two search processes admitted curve seeds 13 and 57
  (odd list, 29 seeds tried, 496 s) and 156 and 166 (even list, 83 seeds tried, 1418 s).
  `h = 1` everywhere. Each curve ran with 2 solver seeds. All 224 runs (4 arms x 56) are
  certified (`[t]G = T`, `ok`). No prime was dropped. `decomp_fallbacks` = 0 and
  `la_skipped` = 0 in every run.
- Compute on at most 2 threads. The main run took 156 s wall on 2 threads. The 65537
  processes took 1551 s and 623 s wall on 1 thread each. That is about 41 core-minutes for
  the measured runs. The development runs (4093, 8191) and the post-hoc diagnosis were not
  timed precisely; they took roughly 30 to 45 core-minutes more. Builds are
  not counted. `research/genus_ladder_scaling_20261119` was not touched.

## Checks

- **C0 (OLD reproduces pre-check 1):** all 48 of 48 (curve, seed) pairs are identical on
  every compared field (`group_ops`, `cantor_*`, `rel_*`, `la_*`, relations,
  `decomp_trials`, `attempts`, core size, `fb_cols`, certified, ok).
- **C1 (the algebraic descent changes nothing but the descent count):** all 56 of 56
  pairs `Q_diem_2lp_alg` vs `Q_diem_2lp` are identical on every field except `rel_*`.
  There were 0 fallbacks.
- **C2:** 224 of 224 runs are certified.

## Fitted exponents (OLS of ln metric on ln p, curve level, 28 points; boot95 = cluster bootstrap, 4000 replicates)

| arm | `la_mul` | FPM mid | `rel_mul` | GAE w=1 |
|---|---|---|---|---|
| `Q_diem_2lp` (OLD) | 1.194 [1.065, 1.316] | 1.329 [1.236, 1.419] | 1.573 [1.496, 1.653] | 1.551 [1.472, 1.631] |
| `Q_diem_2lp_alg` | 1.194 [1.057, 1.321] | 1.041 [0.947, 1.135] | 1.082 [1.077, 1.087] | 1.113 [1.081, 1.148] |
| `Q_diem_2lp_kick` | 1.233 [1.149, 1.327] | 1.051 [0.986, 1.122] | 1.077 [1.071, 1.084] | 1.117 [1.091, 1.147] |
| **`Q_diem_2lp_new`** | **0.971 [0.919, 1.024]** | **0.897 [0.858, 0.934]** | 1.075 [1.068, 1.082] | 1.064 [1.052, 1.074] |

Local slopes of `Q_diem_2lp_new` from 1031 to 65537:

- FPM mid: 0.54, 0.74, 0.91, 1.13, 1.10, 0.67.
- `la_mul`: 0.82, 1.02, 0.72, 1.31, 1.28, 0.35.

The slope below 1 comes from fixed costs at small p. The starts of the decomposition walk
cost two scalar multiplications each, so GAE at w = 0 scales as 0.64. The local slopes are
noisy because the number of attempts (1 or 2) moves `la_mul` by about 2x.

Per-prime geometric means, `Q_diem_2lp_new`:

| p | `la_mul` | FPM mid | `rel_mul` | of which `rel_enum` | LA share | attempts mean/max | `decomp_trials` |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 1031 | 1.16e5 | 3.62e6 | 6.5e5 | 4.5e5 | 0.29 | 1.12 / 2 | 308 |
| 2053 | 2.04e5 | 5.26e6 | 1.30e6 | 9.5e5 | 0.35 | 1.00 / 1 | 348 |
| 4099 | 4.13e5 | 8.76e6 | 2.45e6 | 1.96e6 | 0.43 | 1.12 / 2 | 436 |
| 8209 | 6.81e5 | 1.65e7 | 5.84e6 | 4.53e6 | 0.37 | 1.00 / 1 | 955 |
| 16411 | 1.69e6 | 3.63e7 | 1.26e7 | 1.04e7 | 0.43 | 1.38 / 2 | 1826 |
| 32771 | 4.09e6 | 7.77e7 | 2.36e7 | 1.95e7 | 0.48 | 1.62 / 2 | 3856 |
| 65537 | 5.21e6 | 1.23e8 | 5.66e7 | 4.43e7 | 0.38 | 1.12 / 2 | 4750 |

For comparison, the OLD arm averaged 1.25, 1.62, 1.75, 1.75, 3.75, 3.88 and 4.50 attempts
(maximum 9). Its FPM mid at 65537 is 8.54e8, of which the scan descent accounts for
`rel_decomp_mul` = 6.9e8.

## Paired comparison (same curve and solver seed; ratio arm/reference; curve geo-mean, cluster boot95)

| contrast | FPM mid | `la_mul` | `rel_mul` | GAE w=1 | attempts | `decomp_trials` |
|---|---|---|---|---|---|---|
| **NEW / OLD** (pooled) | **0.391 [0.349, 0.437]** | 0.560 [0.486, 0.644] | 0.221 [0.201, 0.243] | 0.239 [0.217, 0.263] | 0.552 [0.487, 0.627] | 1.185 [0.989, 1.393] |
| ALG / OLD (descent) | 0.526 [0.511, 0.541] | 1 (identical) | 0.218 [0.199, 0.240] | 0.264 [0.248, 0.281] | 1 | 1 |
| KICK / ALG (kick, frozen order) | 0.899 [0.788, 1.016] | 0.846 [0.713, 1.006] | 0.994 | 0.963 | 0.870 [0.737, 1.028] | 0.886 |
| NEW / KICK (descent last, linked) | 0.827 [0.747, 0.910] | 0.663 [0.581, 0.751] | 1.022 | 0.939 | 0.635 [0.555, 0.719] | 1.338 [1.132, 1.581] |

NEW / OLD by prime, FPM mid:

| p | 1031 | 2053 | 4099 | 8209 | 16411 | 32771 | 65537 |
|---|---|---|---|---|---|---|---|
| ratio | 1.002 | 0.609 | 0.525 | 0.481 | 0.261 | 0.241 | 0.144 |
| 95% interval | [0.924, 1.074] | [0.495, 0.749] | [0.437, 0.670] | [0.316, 0.733] | [0.207, 0.329] | [0.156, 0.342] | [0.107, 0.215] |

The slope of the ln ratio on ln p is -0.432 [-0.502, -0.359] for FPM mid and -0.224
[-0.321, -0.115] for `la_mul`.

- The descent fix is exact and grows with p. ALG / OLD FPM mid goes from 0.881 at 1031 to
  0.276 at 65537.
- The T kick alone, under the frozen order, is inconclusive. Its pooled interval includes
  1, and at 65537 it is 1.275 [0.84, 1.94]. With decompositions taken before the
  large-prime graph exists, most rows never reach the cycle matrix.
- Descent last with linked rows removes most of the retries, at the price of about 1.3x
  more decomposition trials.

## Wiedemann retries: diagnosis and status

- **Pre-check 1 (frozen instrument).** The retries were structural, not random. This is
  shown by the rank diagnosis in `PROTOCOL.md` §2 (development curves, 39 of 39 failed
  attempts) and the kernel argument there. Line relations leave the base-point shift and
  the scale free. One decomposition walk `x + iG` puts every row on one line in the
  `(a, b)` plane, and rows collected before the graph exists mostly dangle. So `t` is not
  determined. Wiedemann then returns a consistent but wrong solution (residual 0, wrong
  `t`), and the run retries.
- **NEW.** The two fixes remove most retries: mean attempts are 1.0 to 1.62 against 1.25
  to 4.50 for OLD.
- **Residual retries.** About 1 run in 5 still needs a second attempt (11 of 56 runs, none
  needs more than 2), and each one is charged. A post-hoc rank diagnosis of the 16 NEW runs
  at 16411 and 32771 (`explore/diag_new_post_hoc.jsonl`, exploratory, uncounted rank,
  not in any fit) shows the same pattern in 8 of 8 failed attempts: `rank = rank_no_t`.
  In those attempts, the 4 rows that were linked when found leave as few as 1 or 2 core
  rows with a `t` entry. This residual is still an instrument choice (the row target and
  the linked test at insertion), not something inherent to Diem's algorithm. A
  rank-aware stop (for example, three `t` rows in general position in the core) would
  remove it.

## Caveats

- **The point enumeration is now the main relation-phase term.** `rel_enum`
  (`O(p log p)`, root finding of `F(1, y, z)` for every `y`) is 78% of `rel_mul` at 65537
  in the NEW arm. It is needed here only for the factor-base draw and the point index.
  Diem's algorithm does not need it: a factor base of `3 sqrt p` points can be drawn by
  sampling. It was left unchanged by design, as one-time set-up and not a per-trial step.
  It is the next sublinear fix.
- **The development data chose the fix and the 4-row target.** These were curves at 4093
  and 8191, which are not grid primes. The KICK arm's mixed result shows that the choice
  matters.
- **LA solver.** LA is scalar Wiedemann on `AᵀDA`, not block Lanczos.
- **Small range.** The prime range is small (1031 to 65537, a factor of 64), and fixed
  costs keep the fitted exponent below 1. The exponent is local and is not an asymptotic
  claim.
- **No rho, no gate.** The L10 corrections were not applied because no rho was run. No
  validation gate, no leaderboard row and no `benchmarks/` row.
- **Host.** Shared host. Wall time is not used.

## Next step (as preregistered)

PROCEED-TO-LADDER means the full destination measurement for lead #1 with the
L10-corrected matched rho (negation rho / 1.73, `<rho>` class walk 0.447), all four phases
charged, and the §5a GAE band. Before that, it should replace the `O(p log p)` point
enumeration with a sampled factor base, and add a rank-aware stop for the linked rows.
