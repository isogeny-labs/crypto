# Results: lead #1 ladder: Diem 2LP IC on prime-order plane quartics, sampled factor base, against matched rho

Every number here is `[pcc:unverified]`. It comes from one session on a shared host, with
planted targets, no replay certificate and no VALIDATION.json. Nothing here is a leaderboard,
`benchmarks/` or tier row.

- Protocol: `PROTOCOL.md`, frozen at 2026-11-21 05:30 UTC before any measured run.
  **sha256(PROTOCOL.md) = `96311c59cb1504dd5f20343e65be719b452badcabbf2089977ac892063b52ec2`**.
  `frozen.sha256` also binds `analyze.py` and the three run scripts; `sha256sum -c` passes.
- Source: `source.sha256`. Binary: `binary.sha256` (`genus_ladder.qsfb`, sha256 `f0243cf8…e38b`).
- Raw data: `raw/`, hashed in `data.sha256`.
  - The 131101 runs wrote their live output to
    `/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/qladder_live/raw/`,
    as the main session asked, so that each append did not trigger a commit hook. They were
    copied into `raw/` when they finished, and the copies are byte-identical (same sha256).
  - `raw/run.jsonl` was written in place. The request came after that run had started.
- Analysis: `analyze.py` writes `analysis.json`. Figure: `plot.py` writes `fig_ladder.svg`
  (46 KB) and `fig_ladder.png` (14 KB). Run report and PDF: `reports/run-20261121T0531Z/`.
- Development data (not in any fit): `dev/` (curves at 4093 and 8191, the C0 check below, and
  the test log).

## Verdicts under the frozen rules

| rule | result |
|---|---|
| Per (arm, prime), primary band (all native operations, w = 0, 0.1, 1) | **UNRESOLVED at every prime in both arms.** At every prime, IC is far below rho at w = 0 and above it at w = 1. §5a forbids a ranking. There is no BELOW-RHO CANDIDATE and no ABOVE RHO. |
| Gap closing (upper 95% bound of the ln(IC/rho) slope < 0) | **Yes, at every w in both arms.** |
| Step A accepted | **Yes.** All 80 `2lp-sfb` runs certified with the right logarithm, the 14 `quartic_ladder` tests pass, and the pooled paired FPM mid sfb/new is **0.683 [0.640, 0.729]** (upper bound below 1). |
| Lead #1 STOP (upper 95% bound of the FPM-mid exponent >= 1.40) | **No.** sfb 0.863 [0.834, 0.891]; new 0.924 [0.893, 0.953]. |

## Grid as run

- 8 primes: 1031, 2053, 4099, 8209, 16411, 32771, 65537 and 131101.
- 5 prime-order curves per prime (`h = 1`), 2 solver seeds, both arms: 160 IC runs. All are
  certified and `ok`, and every row records the recovered logarithm (validator F3).
- Measured negation rho (`Q_rho_neg`) on all 50 (curve, seed) pairs at p <= 16411, all
  certified.
- Fifth curves (curve seed): 1031: 317; 2053: 174; 4099: 295; 8209: 218; 16411: 382;
  32771: 229; 65537: 369 (156 seeds tried after 57, 52 min of search).
- 131101: odd process 53, 177, 193 (97 seeds, 94 min); even process 228, 288 (144 seeds,
  139 min).
- `decomp_fallbacks` = 0 everywhere. No run needed more than 2 attempts.

## IC / rho per prime

Primary metric: GAE_w = `group_ops` + w · (all native operations), against the ideal negation
rho `sqrt(pi r / 4)`. Each entry is the geometric mean over 5 curves with its cluster-bootstrap
95% interval.

| p | new w=0 | new w=0.1 | new w=1 | **sfb w=0** | **sfb w=0.1** | **sfb w=1** |
|--:|--|--|--|--|--|--|
| 1031 | 0.0125 [0.0091, 0.0166] | 7.37 [6.82, 8.17] | 73.5 [67.9, 81.6] | 0.0089 [0.0078, 0.0103] | 4.58 [4.35, 4.83] | 45.7 [43.1, 48.2] |
| 2053 | 0.0058 | 5.37 | 53.6 | 0.0057 | 3.41 | 34.1 |
| 4099 | 0.0028 | 3.97 | 39.7 | 0.0027 | 2.38 | 23.8 |
| 8209 | 0.0016 | 3.26 | 32.6 | 0.0014 | 1.83 | 18.2 |
| 16411 | 0.0010 | 2.63 | 26.3 | 0.0009 | 1.76 | 17.6 |
| 32771 | 0.0006 | 2.15 | 21.5 | 0.0004 | 1.18 [1.02, 1.55] | 11.8 |
| 65537 | 0.0003 [0.0003, 0.0004] | 1.53 [1.33, 1.81] | 15.3 [13.3, 18.1] | 0.0003 [0.0002, 0.0003] | 0.90 [0.83, 1.005] | 8.96 [8.31, 10.05] |
| 131101 | 0.0002 [0.0002, 0.0003] | 1.23 [1.12, 1.35] | 12.3 [11.2, 13.5] | 0.0002 [0.0002, 0.0002] | **0.77 [0.68, 0.88]** | 7.75 [6.75, 8.85] |

- The intervals for the middle primes are in `analysis.json` (`ic_over_rho.<arm>.all_<w>`).
- The w = 0 reading counts group operations only. At 131101, IC uses about 10^4 Pic operations
  per solve; rho uses about 4·10^7. Per validator F9, the w = 0 reading is given beside every
  pooled ratio below.
- **What drives w = 1.** `la_add` (graph, DSU and `AᵀDA` additions mod r) is 85% of the native
  operations of sfb at 131101 (2.78e8 of 3.26e8). Pricing each of those additions at a full Pic
  addition, which is about 3,900 field multiplications, is what puts IC above rho at w = 1.

### Sensitivity readings (do not change the verdict)

| reading | sfb at 65537 | sfb at 131101 | new at 131101 |
|---|---|---|---|
| mul-only band, w = 0.1 | 0.138 | 0.092 [0.084, 0.100] | 0.319 |
| mul-only band, w = 1 | 1.38 [1.33, 1.43] | **0.913 [0.834, 0.999]** | 3.18 |
| FPM mid against the ideal rho in FPM | 0.0013 | **0.0010 [0.0008, 0.0011]** | 0.0016 |
| count-calibrated, w_cnt = 1/A | 0.0025 | 0.0021 | 0.0033 |
| count-calibrated, 10 w_cnt | 0.0225 | 0.0195 [0.017, 0.022] | 0.0309 |

- **Mul-only pricing** is the pre-check definition. Under it, sfb at 131101 has its upper 95%
  bound below 1 at w = 0, 0.1 and 1; at w = 1 the bound is 0.999, a hair below. Under the frozen
  primary it is UNRESOLVED. This is a sensitivity reading only.
- **Count-calibrated weight.** One native operation is priced as one field multiplication, and
  one Pic operation as A, about 3,600 to 4,000 FPM, measured in the same runs. At w = 0, w_cnt
  and 10 w_cnt, IC is below rho at **every prime in both arms**. That makes the pair "ranked at
  count-calibrated price". This is not a timing calibration: §5a's timing `w_cal` was not
  measured on this host.
- In FPM, IC at 131101 costs about 1/1000 of the ideal negation rho.

## Exponents

OLS slope of ln metric on ln p: 8 primes, 40 curve points, cluster-bootstrap 95% interval.

| metric | 2lp-new | 2lp-sfb |
|---|---|---|
| FPM mid | 0.924 [0.893, 0.953] | 0.863 [0.834, 0.891] |
| FPM mid, without 131101 | 0.909 [0.865, 0.953] | 0.842 [0.807, 0.879] |
| `rel_mul` | 1.086 [1.080, 1.092] | 0.971 [0.957, 0.985] |
| `rel_enum_mul` (set-up) | 1.113 [1.112, 1.115] | **0.616 [0.607, 0.624]** |
| `la_mul` | 0.975 [0.936, 1.015] | 0.968 [0.939, 0.998] |
| all native operations (the slope of GAE_0.1 and GAE_1) | 1.142 [1.118, 1.166] | 1.135 [1.112, 1.159] |
| GAE_w, mul-only | 1.074 [1.065, 1.083] | 0.973 [0.957, 0.990] |
| `group_ops` (= GAE_0) | 0.687 [0.626, 0.746] | 0.672 [0.622, 0.721] |
| `decomp_trials` | 0.791 [0.716, 0.878] | 0.793 [0.730, 0.855] |
| `group_ops` / `decomp_trials` | -0.104 | -0.121 |
| `relations` | 0.941 | 0.942 |
| `attempts` | 0.050 | 0.043 |
| `idx_points` | 1.001 | 0.964 |

Per-component reading (validator F11):

- Fixed per-solve Pic work produces the sub-1 total slopes. Each decomposition start costs two
  scalar multiplications, and the Pic cost per trial decreases with p.
- `rel_mul` is still `O~(p)` in the line phase.
- The total of native operations grows as p^1.14.
- **No sub-1 slope is extrapolated.** The rho reference has slope 1.5 by construction.

The ln(IC/rho) slope is negative at every w, so the gap closes; every upper bound is below 0.

| | w = 0 | w = 0.1 | w = 1 | mul-only, w = 1 | FPM |
|---|---|---|---|---|---|
| new | -0.813 [-0.872, -0.753] | -0.359 [-0.383, -0.335] | -0.359 | -0.426 | -0.551 |
| sfb | -0.829 [-0.877, -0.778] | -0.366 [-0.389, -0.341] | -0.365 [-0.390, -0.342] | -0.527 | -0.613 |

Extrapolated crossovers of the primary band are predictions only. They are reported only where
the IC exponent is at least 1.

| | w = 0.1 (log2 p*) | w = 1 (log2 p*) |
|---|---|---|
| sfb | 15.8 [15.5, 16.1] | **24.9 [24.0, 25.8]** |
| new | 17.8 [17.4, 18.3] | 27.1 [26.1, 28.2] |

- For sfb at w = 0.1, the data already show 0.90 at 2^16 and 0.77 at 2^17.
- Withheld (sub-1 IC slope): sfb mul-only, FPM, and w = 0.
- For the count-calibrated rows, the script prints "crossover" values below p = 2. The ratio is
  below 1 at every prime, so those values are meaningless and are ignored here.

## Step A: paired sfb / new

Same curve and solver seed. Pooled geometric mean with cluster-bootstrap 95% interval; the
w = 0 reading is the `group_ops` row.

| metric | pooled | 1031 | 8209 | 65537 | 131101 | slope in ln p |
|---|---|---|---|---|---|---|
| FPM mid | **0.683 [0.640, 0.729]** | 0.722 | 0.676 | 0.588 | 0.607 | -0.061 [-0.098, -0.026] |
| `rel_mul` | 0.287 [0.280, 0.294] | 0.369 | 0.303 | 0.269 | 0.221 | -0.115 |
| `rel_enum_mul` | 0.028 [0.027, 0.028] | 0.091 | 0.033 | 0.012 | 0.008 | -0.497 |
| `la_mul` | 0.918 [0.840, 0.996] | 0.935 | 0.868 | 0.857 | 0.976 | -0.007 |
| all native operations (= GAE_0.1 and GAE_1) | 0.606 [0.574, 0.639] | 0.622 | 0.560 | 0.587 | 0.631 | -0.007 |
| GAE, mul-only, w = 1 | 0.363 [0.352, 0.375] | 0.453 | 0.369 | 0.325 | 0.287 | -0.101 |
| `group_ops` (= GAE_0) | 0.846 [0.749, 0.963] | 0.713 | 0.885 | 0.801 | 0.832 | -0.016 |
| `relations` | 0.995 [0.990, 0.999] | | | | | |
| `attempts` | 0.933 [0.863, 1.000] | | | | | |

- **Set-up.** The sampled base samples about `3 sqrt p` fibres instead of all p (1,097 at
  131101). That removes 97% of the set-up multiplications and 71% of `rel_mul`.
- **What is left of `rel_mul`** is the `O~(p)` line phase plus the counted insertions of large
  primes, about 36 multiplications per new point. At 131101 the index ends with 74% of the
  rational points.
- **Unchanged within noise:** relations, attempts and LA. LA is slightly lower (0.918, interval
  just below 1).
- **The ratio of all native operations stays at about 0.6 at every p**, because `la_add` is not
  touched.

Predictions (§7):

- P1 held: 0.012 at 65537, 0.008 at 131101.
- P2 held: 0.269.
- P3 held: 0.918, inside [0.67, 1.5].
- P4 held: 0.863 < 0.924, with intervals that overlap slightly.
- P5 held at w = 1 at every prime. At w = 0 it failed in one cell: new at 1031 is 0.0125 > 0.01.
- P6 held.

## Checks

- **C0 (instrument unchanged for `2lp-new`).** On the 4 development curves × 2 seeds at 4093 and
  8191, the new binary's `2lp-new` rows equal the pre-check-2 binary's (`genus_ladder.qdiem2`)
  on every compared counter. Files: `dev/dev_c0_qdiem2_4093_8191.jsonl` and
  `dev/dev_smoke_4093_8191.jsonl`.
- **Tests:** 14 passed (`dev/tests_quartic_ladder.log`; `cargo test --release --lib
  quartic_ladder`, with the crate at opt-level 1). The three new tests are:
  - identical relations when the sampled arm is given the enumerated base, at q = 101, 251, 1031
    and 2053;
  - exhaustive agreement of membership, lines and descents at q = 31, 37, 53, 101 and 131;
  - the sampled arm certifies with Pic-verified relations.
- **All 210 measured runs certified** (160 IC, 50 rho), with `recovered` and `t_secret`
  recorded.

## Rho calibration

Measured negation rho at p <= 16411, 10 runs per prime.

| p | steps / sqrt(pi r/4) | rho FPM / ideal FPM | FPM per rho step / A | IC_sfb / measured rho at w = 0, 0.1, 1 |
|--:|--|--|--|--|
| 1031 | 0.877 | 1.86 | 2.12 | 0.010, 5.23, 52.2 |
| 2053 | 0.970 | 2.15 | 2.21 | 0.006, 3.52, 35.1 |
| 4099 | 1.188 | 2.67 | 2.25 | 0.002, 2.00, 20.0 |
| 8209 | 0.770 | 1.77 | 2.30 | 0.002, 2.37, 23.7 |
| 16411 | 0.702 | 1.64 | 2.34 | 0.001, 2.51, 25.1 |

- Per step, the executed walk costs 2.1 to 2.3 times a plain Pic operation. The overhead is the
  canonicalisation of the negation map. This is in line with L10's (A+N)/A ≈ 2.45.
- The step counts scatter around the ideal (10 runs per prime).
- Using the ideal reference with no overhead favours rho by about 2x in FPM.

## Deviations and interruptions

1. **Container restart before freeze** (2026-11-21, about 04:30 UTC). The tmpfs build directory
   was lost, and the binary was rebuilt from the same tree. No measured data existed
   (PROTOCOL §6c).
2. **Disk.** `/` had 1.1 GB free, so the cargo target directory was placed on tmpfs
   (`/dev/shm/qsfb-target`, linked from `scratchpad/target-qsfb`). On 2026-11-23 the disk was
   full (shared host). To free space, the build-1 binary was deleted; it had been used for
   development only.
   After the main session freed space, every file was re-verified: `sha256sum -c` passes on
   `data.sha256`, `frozen.sha256`, `source.sha256` and `binary.sha256`, every JSONL file parses,
   and rerunning `analyze.py` and `plot.py` reproduces `analysis.json` and `fig_ladder.svg`
   byte for byte. No truncation was found.
3. **Binary replaced before freeze.** The first build lacked the F3 fields. The frozen binary is
   the second build, and only development data used build 1.
4. **The 110-minute search cap was not enforced** for the 131101 even process. The session was
   paused (API limit) while it ran. The process finished its search at 139 min and completed its
   runs normally. A rerun under the cap rule would have used the same two admitted curves (228
   and 288), so the data are unaffected; the budget was exceeded.
5. **Budget.** Total: 3 h 34 min of wall time and about 6 core-hours, over the "about 3 hours"
   of PROTOCOL §9.
   - Main session: 05:31-06:42 UTC, 71 min of wall time on 2 threads. Most of it was the 52-min
     search at 65537 on one thread.
   - 131101 processes: 06:42-08:23 and 06:42-09:05 UTC.
   - Almost all of the time is curve admission (uncharged set-up). Every solve took under 4 s;
     a rho run at 16411 took about 100 s.
   - Builds (about 45 min each time, `-j 1`) are not counted.
6. `raw/run.jsonl` was written inside the study directory, because the live-output request
   arrived after that run had started.
7. Conductor is not installed on this host, so no task was reserved.

## Caveats

- **Planted targets, warm, one session.** These are not cold public targets, and there is no
  replay certificate.
- **`la_add` dominates the band verdict.** It counts LA structure: DSU and `AᵀDA` additions
  mod r. A cheaper LA (block Lanczos, or structured Gaussian elimination) would move the w = 1
  reading most.
- **Reference.** The ideal negation rho with nothing unpriced is a floor for rho. The executed
  lab walk costs about 2x more in FPM. No `<rho>` class walk applies to generic quartics.
- **Range.** 2^10 to 2^17 (a factor of 128). The exponents are local.
- **Admission limits the ladder, not the solver.** At 262147, each seed tried would take about
  70 s.

## Next steps

1. Pass the 131101 rows through the validation gate (replay certificate, validator report).
   That is needed before any "below rho" statement at w = 0.1.
2. Cut `la_add`, which drives w = 1: block Lanczos or SGE on the 2LP core, counted. Add a
   rank-aware stop for the residual second attempts.
3. Make prime-order admission cheaper, or allow a small cofactor, to reach 2^18 to 2^20. The
   primary w = 1 crossover is predicted near 2^25 for sfb.
