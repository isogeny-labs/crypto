# How the Joux-Vitse cover cost S/rho scales with p (2026-10-24)

**Scope.** This curve family only: the `jv_cover` genus-3 cover of E(F_{p^6}) over F_{p^2} with Nagao decompositions (full rational 2-torsion, weak class, `generate_spec(p, seed)`), sparse-FGLM linear stage. It is not a statement about prime-order curves. This is a scaling measurement; it makes no speedup claim and no claim to beat rho. Every S/rho below is a lower-bound comparison (AGENTS.md section 5a, see below), reported as the distance remaining.

Labels: **measured** = read from a run record; **derived** = computed from measured values; **model** = cost-model prediction or extrapolation.

Files: [`PROTOCOL.md`](PROTOCOL.md) (frozen before the main run; sha256 `81b2a474...` in `results/FROZEN.sha`, unchanged), [`REPORT.pdf`](REPORT.pdf), graph [`scaling.svg`](scaling.svg) / [`scaling.png`](scaling.png), raw records and logs in [`results/`](results/), fit tool in [`fit/`](fit/) (standalone Rust), `run.sh` (first execution; its 1009 loop was killed in the incident) and `run_1009.sh` (the 1009 runs, thin orchestration).

## Result

Pre-registered hypothesis H1: log(S/rho) falls roughly linearly in log p. Cost model: S/rho ~ 720 C_cov / (c_e S_rho p^2), slope **-2** (derived in PROTOCOL.md).

| quantity | value | label |
| --- | --- | --- |
| fitted slope of ln(S/rho) on ln p, 17 per-seed points, p = 53..1009 | **-2.05** | derived from measured |
| 95% interval, t (15 df) | [-2.28, -1.83] | derived |
| 95% interval, cluster bootstrap over seeds (10000 resamples) | [-2.29, -1.84] | derived |
| half slopes: p <= 151 / p >= 251 | -1.60 (se 0.43) / -1.91 (se 0.13); difference -0.31 | derived |
| extrapolated p* where the fitted line meets S/rho = 1 | **about 1330**, bootstrap 95% [1170, 1540] (slope-interval ends: [1110, 1660]) | **model** (extrapolation) |
| p* with slope fixed at -2 (cost model, fitted constant) | about 1400 | **model** |

Decision rules from PROTOCOL.md: the 95% interval lies below 0 and the half-slopes differ by 0.31 < 0.7, so **H1 is supported** on this family; the point estimate -2.05 lies in [-2.4, -1.6] and the interval lies inside [-2.5, -1.5], so the cost model's slope **-2 is consistent and not falsified**; the earlier "about p^-1.2" reading (two seeds at 53 and 101) is **outside** the interval. Caveat: the rule's falsification test only rejects slopes whose interval lies entirely outside [-2.5, -1.5]; here the interval [-2.28, -1.83] lies inside it, so the data neither falsify nor sharply confirm values between.

The largest measured prime, p = 1009, has S/rho between 1.59 and 1.98 (three seeds, **measured**, with rho S taken from the pooled smaller-p runs): the line has not met rho; the measured points do not cross S/rho = 1, so p* is an extrapolation, not an interpolation. Distance remaining at p = 1009: a factor of about 1.7 (geometric mean 1.73).

### Per-seed S/rho

S/rho = total counted multiplications / (rho_ref sqrt(l) c_add_E) with the pooled rho reference (**derived**, rho_ref = 1.344, below). "own rho" is the binary's own `s_over_rho` from that run's 16 rho runs (p <= 251 only).

| p | seed | S/rho, pooled rho | S/rho, own rho |
| --- | --- | --- | --- |
| 53 | 1 | 544.46 | 495.8 |
| 53 | 2 | 244.69 | 305.66 |
| 53 | 3 | 1786.8 | 1476.58 |
| 101 | 1 | 167.01 | 156.31 |
| 101 | 2 | 271.98 | 243.55 |
| 101 | 3 | 241.94 | 244.64 |
| 151 | 1 | 119.75 | 108.17 |
| 151 | 2 | 122.18 | 146.34 |
| 151 | 3 | 105.45 | 117.94 |
| 251 | 1 | 22.76 | 26.52 |
| 251 | 2 | 26.34 | 25.75 |
| 503 | 1 | 8.8 | n/a |
| 503 | 2 | 4.97 | n/a |
| 503 | 3 | 8.08 | n/a |
| 1009 | 1 | 1.98 | n/a |
| 1009 | 2 | 1.59 | n/a |
| 1009 | 3 | 1.66 | n/a |

Per-prime geometric means (pooled rho): p = 53: 620 (3 seeds, range 245-1787); 101: 222 (167-272); 151: 116 (105-122); 251: 24.5 (22.8-26.3, 2 seeds, see failure below); 503: 7.07 (4.97-8.80); 1009: 1.73 (1.59-1.98). The seed spread is large at small p (a factor of 7 at p = 53) because relation yield depends on the curve: residuals needed per relation ranged from 8486/33 to 62880/26 at p = 53 (residual / 720|F| from 0.36 to 3.36), and narrows with p (0.98-1.29 at p = 1009).

### Run details (measured)

| p | seed | \|F\| | residuals | relations | residual / (720\|F\|) | incomplete tests | C_cov (F_p mul) | S | LA share (r) | wall s |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 53 | 1 | 30 | 19020 | 31 | 0.85 | 7 | 945536 | 731.6 | 0.0111 | 78 |
| 53 | 2 | 32 | 8486 | 33 | 0.36 | 5 | 952378 | 328.8 | 0.0173 | 39 |
| 53 | 3 | 25 | 62880 | 26 | 3.36 | 20 | 938649 | 2400.9 | 0.0071 | 255 |
| 101 | 1 | 52 | 40100 | 53 | 1.05 | 4 | 952058 | 224.4 | 0.0046 | 114 |
| 101 | 2 | 49 | 65422 | 50 | 1.82 | 3 | 950383 | 365.5 | 0.0041 | 160 |
| 101 | 3 | 49 | 58201 | 50 | 1.62 | 3 | 950313 | 325.1 | 0.0046 | 111 |
| 151 | 1 | 74 | 95654 | 75 | 1.77 | 4 | 956408 | 160.9 | 0.0027 | 176 |
| 151 | 2 | 70 | 97696 | 71 | 1.91 | 2 | 955355 | 164.2 | 0.0032 | 246 |
| 151 | 3 | 70 | 84292 | 71 | 1.65 | 3 | 955662 | 141.7 | 0.0031 | 210 |
| 251 | 1 | 129 | 82263 | 130 | 0.88 | 1 | 970514 | 30.58 | 0.0023 | 191 |
| 251 | 2 | 123 | 95284 | 124 | 1.07 | 1 | 969913 | 35.39 | 0.0017 | 375 |
| 503 | 1 | 238 | 252255 | 239 | 1.47 | 1 | 985463 | 11.83 | 0.0008 | 551 |
| 503 | 2 | 262 | 142150 | 263 | 0.75 | 1 | 986447 | 6.67 | 0.0010 | 313 |
| 503 | 3 | 249 | 231408 | 250 | 1.29 | 2 | 985735 | 10.85 | 0.0009 | 512 |
| 1009 | 1 | 484 | 450841 | 485 | 1.29 | 1 | 1000926 | 2.66 | 0.0004 | 620 |
| 1009 | 2 | 511 | 360835 | 511 | 0.98 | 0 | 1001264 | 2.13 | 0.0005 | 494 |
| 1009 | 3 | 497 | 377514 | 498 | 1.05 | 0 | 1001006 | 2.23 | 0.0004 | 558 |

C_cov is nearly flat in p (9.4e5 to 1.0e6, +6% over a factor 19 in p) and |F| is about p/2, as the model assumes; so the slope is dominated by the p^-2 from sqrt(l) ~ p^3/2 against relations ~ p. Linear algebra is at most 1.7% of S (consistent with "relation collection ~99.99%" only at large p; at p = 53 it is up to 1.7%). Wall times are practicality notes on a shared 4-vCPU host, not the metric.

### Rho reference

Pooled mean of 176 rho S samples at p <= 251 (the `rho_e` walk, every group operation counted): **1.344** (single-run sd 0.62, se 0.047; **measured**). Per prime: 53: 1.39, 101: 1.42, 151: 1.27, 251: 1.26 (48, 48, 48, 32 runs). The trend of rho S against ln p is -0.10 per unit ln p, 95% interval [-0.27, 0.07]: no significant trend, so pooling is not rejected, but the interval is wide and the point estimate is a decrease of 0.1 in rho S per unit ln p. **The 503 and 1009 rows use this pooled value as a model assumption**: rho cannot be run there (l ~ 2^52 and 2^58, above the 2^48 memory limit of the stored-table rho). If rho S really kept falling at the point-estimate trend (not significant), rho S at 1009 would be about 1.12 and S/rho there about 20% higher than shown. The fit with each point's own rho (p <= 251 only, 11 points) gives slope -1.90, 95% [-2.48, -1.32], p* about 1640 (**model**); it is consistent with the primary fit and less precise.

### Controls (measured)

- Every `dlp` run that finished: `solved = true`, `correct = true` (logarithm equals the planted secret), `mismatches = 0` (17 of 17 runs; oracle cross-check ran on 1414 residuals at p = 53 only, because the oracle table needs |F| <= 40), every relation verified by `verify_dec` before use.
- `ccov` control per prime (60 tests each): planted decompositions found 20/20 at every p in {53, 101, 151, 251, 503, 1009}; oracle mismatches 0; unverified 0; incomplete 0. The oracle itself checked 60 residuals at p = 53 and none at larger p (|F| > 40 in this binary's setting), so the oracle control only covers p = 53.
- Determinism control: a second full execution (after the incident below) reproduced `total_muls` bit-for-bit in 11 of 11 runs that it completed (p = 53 to 251, [`results/replicate/`](results/replicate/)).
- Incomplete tests (a Nagao test whose Groebner stage did not finish) occurred in every run (0 to 20 per run); they lower the relation yield, are counted in S, and cannot produce wrong relations.

### Section 5a pricing band

Not applicable as a ranking, and stated precisely: the binary's counted unit is F_p multiplications priced into group additions. For rho the unpriced native work is a hash, a table lookup and a table insert per step (3 per step, derived from `rho_e`); for the cover arm the unpriced F4 additions and reductions, memory traffic and Wiedemann vector work are **not emitted** by the binary. An arm that does not emit its counts cannot be ranked under section 5a, so this is recorded as an implementation gap and every S/rho here is a lower-bound comparison at weight 0 on both arms. Sensitivity only (rho side repriced at w = 0, 0.1, 1 GAE per native operation, rho cost x (1 + 3w), cover arm left at 0, **not a valid band**): the p* extrapolation would move from about 1330 to 1170 and 680. The slope is unchanged by any p-independent w. No cover-versus-rho ranking is made.

## Failures and deviations (recorded, not worked around)

1. **p = 251, seed 3: killed by the out-of-memory killer** (exit 137, cgroup memory limit, `jv_cover` RSS 13.9 GB) in the 16-run stored-table rho phase, after 324 s. No JSON was written; the log is empty. Per PROTOCOL.md failures are reported and not silently replaced, so p = 251 has 2 seeds, not 3. The measurement itself is unaffected (rho memory, not the cover).
2. **Working-tree incident (caused by the parent session, not by the experiment).** At about 05:50Z the study files disappeared when the parent session's git checkout switched branches after snapshotting the study to PR #2484's branch. The parent restored every file as untracked afterwards. Meanwhile this session, not knowing that, re-executed the study from a scratchpad copy (frozen PROTOCOL.md recreated byte-identically: sha256 matched) and, on the restoration notice, stopped that second execution. Its 11 completed runs are the determinism control above; the first-execution records are the data. The p = 1009 runs were made after the restoration with `run_1009.sh` in the study's `results/` (seeds 1, 2, 3, all finished, 494-620 s each; the 25-minute gate passed). `results/walls.txt` has the first execution's lines; the 1009 lines were appended afterwards. The interim snapshot in `reports/interim-20261024T0550Z/` was produced by the parent session before the 1009 runs, and uses Python for plotting; it is not used here.
3. `convert` could not render the SVG (no `rsvg-convert`); the PNG was rendered with LibreOffice headless from the Rust-written SVG. The PDF was built with pandoc to ODT and LibreOffice headless; no TeX or typst engine is installed.
4. The rho reference at p = 503 and 1009 is a pooled model value, not a measurement (above).
5. The binary was run with `--rho-ref 1.3` at 503 and 1009 only so it would finish; the `s_over_rho` printed there is not used.

## Reproduce

```text
cargo build --release --example jv_cover
research/jv_cover_p_scaling_20261024/run.sh; research/jv_cover_p_scaling_20261024/run_1009.sh
cd research/jv_cover_p_scaling_20261024/fit && cargo build --release --offline && ./target/release/jv_p_scaling_fit ../results
```

Binary sha256 `a045cb80...e34d` (`results/versions.txt`, `results/probe/bin.sha`).
