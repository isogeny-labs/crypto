# Validator report: multi-target IC4 vs the strongest rho, fresh curves (v2, 2026-11-11)

Validator: independent `validator` subagent (Opus 5.5), 2026-11-11.

**Disposition: `validated`, with amendments.**

- `replay-certificate.json` has status `verified`; `validator-report.json` has status `pass`.
- All 2112 rows replay exactly, and every number in `summary.json`, `analysis.txt` and the RESULTS.md table recomputes from `runs.json`.
- The pre-registered section-5a verdicts reproduce: cheaper at T = 16384, unranked at T = 4096, dearer at T = 1024, and dearer than bsgs2 everywhere.
- Recovered logs were verified independently on a sample that covers every arm and every cell, and the negative control was caught.
- No uncounted work was found in the new arms or in the ic4 change. One avoidable overhead in the rho arms is measured in §5 (F1). The headline survives it, but the margin narrows, and the T = 4096 "unranked" depends on it.
- RESULTS.md wording needs the amendments in §8 before the row is cited.

## 0. What was validated, and from where

- **Frozen artifact set:** commit `dbbc139ef5b2d3c5660d4ca9b47f242fe59ea3fb` (branch `aburan28/eloquent-bell-2jerrf`). I extracted it with `git archive` into the validator scratchpad. Every check reads that copy.
- **The working tree moved during validation.** HEAD became `2ad87dd980` (01:49:53Z, "IC leaderboard: multi-target IC4 vs strongest rho section ...").
  - That commit touches `benchmarks/20261111-mt-fieldops-ic4/`, `docs/ic*` and `scripts/build_ic_leaderboard.py`. It does not touch the study directory or `examples/mt_fieldops.rs`.
  - It is outside the validated set. Its table reads `summary.json` at I = 10M, which reproduces exactly (§7), but I did not validate the builder change or the rendered pages.
  - The study directory in the working tree was diffed against the archive copy: identical.
- **Measured binary:** `target/release/examples/mt_fieldops`.
  - sha256 `bbcc68cc69617d74fab25327d08e6f0a0d9a954c7ec3e6bcf68d2aff68189ca6`, equal to `bin_sha256` in all 2112 rows.
  - I copied it to scratch and did not rebuild it in place.
- **Validator environment:**
  - Local host, 4 cores, Linux 6.18.44.
  - rustc 1.97.0 / cargo 1.97.0, Python 3.13.18, numpy 2.5.3.
  - Counts are deterministic, and no wall-clock figure is used.
- **Files and scope:**
  - The only files I wrote in the repository are the three in this `validation/` directory. Nothing was committed or pushed.
  - I wrote no `VALIDATION.json`; that is the producer's step.
  - **The scratchpad directory is shared with the producing session.** I used only files I created under its `v2val/` subdirectory. Producer files sitting next to it (`r2.jsonl`, `s2.json`, `metrics.sh`, `lb/`) were not read.

## 1. Dispositions

| # | requirement | disposition | evidence |
|---|---|---|---|
| 1 | Freeze order: protocol, code, instances and runner committed before the rows existed; rows carry that commit and the binary SHA | **pass** | §2 |
| 2 | Instances fresh (disjoint from every curve JSON under `research/`) and valid (p and n prime, [n]G = O) | **pass** | §3 |
| 3 | Replay ALL 2112 rows from their argv; counters match exactly (ms ignored); `runs.json` is an exact projection of `rows.jsonl` | **pass**: 2112/2112, 0 mismatches, for the measured binary, a scratch source build and an instrumented build; projection exact | §4 |
| 4 | Correctness: independently verified recovered logs on a sample covering every arm | **pass**: 108 runs, 18 per arm, all 6 cells; 774,144 targets, 0 failures; negative control caught in all 6 arms | §5 |
| 5 | Fairness: rho2x, rhoseq and the ic4 change; uncounted work or asymmetry | **pass with amendments** (non-blocking). No uncounted field or hash work. Asymmetry F1 (per-target rho walk start) measured; it does not flip the headline. Verification constant F2 undisclosed in RESULTS. | §6 |
| 6 | Own recompute from `runs.json`: medians, bootstrap CIs, section-5a verdicts for 6 cells x I/M {3, 10, 30} x h {0, 0.1, 1}, compared with summary.json and RESULTS.md | **pass (exact)**: 324/324 summary medians bit-identical; frozen CI procedure reproduced exactly; own bootstrap within 0.0028; 0 verdict disagreements; RESULTS table 84/84; analysis.txt 324/324 | §7 |
| - | Reproducibility bindings | **partial** (F6, F7) | §4, §8 |
| - | Reporting contract (one table, one unit, ratio, phases, all arms priced, m = 83 / ECC2K-130) | **partial**: phases not separated in RESULTS (F3), verification constant undisclosed (F2), no section-3 class (F10), ecbench deviation unnamed (F8), ECC2K-130 not engaged (F13) | §8 |

The findings list, with severities, is in `validator-report.json` under `findings` (F1 to F13). None is blocking.

## 2. Freeze order (check 1)

| commit | time (Z) | content |
|---|---|---|
| `6b0ac6f192` | 01:38:02 | `PROTOCOL.md`, `run.py`, `analyze.py`, `frozen_ic.json`, `instances/` (33 curves + MANIFEST), the `examples/mt_fieldops.rs` change (rho2x, rhoseq, ic4 dedupe hash), `RESUME.md`. No results. |
| (binary) | 01:41:49 | mtime of `target/release/examples/mt_fieldops`; source mtime 01:33:53, blob-identical to the commit |
| (run) | 01:42:12 | bench record: `run.py holdout` started, 144 s, `commit_dirty: false` at init (01:42:08) |
| `6c5c849d1d` | 01:42:43 | bench record only |
| `706e8d3eb5` | 01:46:10 | rows.jsonl, summary.json, analysis.txt, RESULTS.md, figures |
| `dbbc139ef5` | 01:47:00 | runs.json |

- **Blobs unchanged across all four commits:**
  - `examples/mt_fieldops.rs` `7a21a968`
  - `PROTOCOL.md` `d2b62a9d`
  - `run.py` `9503965f`
  - `analyze.py` `65a38539`
  - `frozen_ic.json` `ca02e950`
  - `instances/` tree `c5f3c9a2`
- **Recorded commit and binary:**
  - All 2112 rows record `commit = 6b0ac6f192...`, which is HEAD when `run.py` ran. The bench-record commit came 31 s after the run started.
  - All rows record `bin_sha256 = bbcc68cc...`.
- **History:**
  - The reflog is linear: commits only after the branch reset to `origin/main` at 01:32:39, with no amend, rebase or reset.
  - `frozen_ic.json` (mu = 1.0, g = 2.0) equals v1 `research/mt_fieldops_20261107/frozen.json`, so the parameters were carried over and not re-tuned.
- **Caveat:**
  - Commit times, mtimes and the reflog are producer-controlled. They are consistent, but they are not third-party timestamps.
  - I cannot exclude private runs before the protocol commit, and I found no evidence of any.

## 3. Instances (check 2)

- **Validity:** all 33 instances pass, using my own code (`instances_check.py`):
  - p and n prime, by Miller-Rabin and independently by trial division;
  - 4a^3 + 27b^2 != 0;
  - G on the curve;
  - [n]G = O by a Montgomery ladder;
  - n in the Hasse interval, whose width 4 sqrt(p) is below n, so #E = n and cofactor 1;
  - three random points killed by n;
  - fixture point = [known_log]G;
  - MANIFEST sha256 and names match.
- **No extra automorphisms:** no instance has a = 0 or b = 0 (j != 0, 1728). Negation is the only automorphism, so the rho arms' x-only lookups are the strongest symmetry available.
- **Determinism:** the 33 files and `MANIFEST.json` regenerate **byte-identically** from the declared command. The generator `research/ic_e222_m221_20261106/gen_instances.py` (blob `5a04d358`) is unchanged since 2026-11-06, and was run as `--families W --bits 22,24,25 --per 11 --seed mtfo-v2-20261111`.
- **Freshness scan:**
  - I recursively extracted every curve-like record (p, a, b), including nested ones, from 119,742 `.json`/`.jsonl` files under `research/`, excluding this study. That gave 30,652 distinct (p, a, b). I also scanned 20,886 records in `docs/curves/`.
  - **0 exact (p, a, b) matches, 0 isomorphic matches (same p, j and n), and 0 same-j curves at the same p.**
  - This is a broader scan than RESULTS' "1031 curves", and it reaches the same conclusion.
- **Notes (not overlaps):**
  - W22-9 has the same (p, n) as `research/ic_e222_m221_20261106/results/instances_scale/W22-6.json`, with a different j. The two curves are isogenous, not the same curve.
  - W22-5 and W22-10 share (p, n) within the set, also with different j.

## 4. Replays and projection (check 3)

| replay | binary | rows | mismatches |
|---|---|---|---|
| measured | `bbcc68cc...` (the measured binary) | 2112 | **0** |
| source build | `d556df7c...`: `6b0ac6f192:examples/mt_fieldops.rs` copied verbatim into a standalone scratch crate, deps pinned to Cargo.lock (rand 0.8.8, rand_chacha 0.3.1, serde_json 1.0.151 with `arbitrary_precision`, `raw_value`) | 2112 | **0** |
| instrumented | scratch copy with the §5/§6 instrumentation (no counted operation touched) | 2112 | **0** |

- **Fields compared:** every field the binary emits except `ms`:
  - arm, n, p, targets, seed, param, ok;
  - I, M, S, H, L;
  - every phase snapshot;
  - every arm extra (m, table, relations, p1_trials, determined, trials, wasted_hits, w_len, db, db_steps, target_steps, u, v, mx, baby, giants).
- **Replay inputs:** instances were read from the archive copy, blob-identical to the recorded argv paths.
- **Phase snapshots:** they are monotone in every row, and the last snapshot equals the totals.
- **runs.json:**
  - Its `rows` equal the projection of `rows.jsonl` onto its 16 fields, in order and type-exact.
  - Its `source_sha256` equals sha256(`rows.jsonl`) = `cafb8c71...4997a7a`.
  - Its commit and `bin_sha256` equal the rows'.
- **Grid:** exactly as in PROTOCOL.md:
  - 6 cells x 11 instances x seeds {11, 12} = 132 configurations;
  - each configuration has 1 ic4 (1.0, 2.0), 12 rho runs (4 arms x kappa {0.5, 0.7, 1.0}) and 3 bsgs2 runs (mf {0.7, 1.0, 1.4});
  - no duplicates or omissions;
  - every argv names `W{pb}-{idx}.json`, and its n and p match the instance file;
  - 0 errors, `ok == targets` in all 2112 rows (15,138,816 targets).

## 5. Correctness (check 4)

- **Instrumentation:** the scratch copy adds an env-gated dump of (t, planted x_t, recovered log, Q_t), written after verification.
- **Sample:**
  - One run per (arm, cell, parameter); for ic4, three per cell.
  - idx and seed were drawn with my RNG (seed string `validator-v2-20261111`).
  - That gives **108 runs: 18 per arm, covering all 6 cells and every kappa and mf value, with 774,144 targets.**
- **My checker (`eccheck.py`)** shares no code with the harness. It uses Jacobian coordinates, right-to-left binary scalar multiplication and a Fermat inversion. For every target it checks:
  - Q_t is on the curve;
  - 0 <= log < n;
  - log == planted x_t;
  - [log]G == Q_t.
- **Result: 0 failures.** The dump headers (p, a, b, n, G) match the instance files.
- **Checker sanity:** before use, the Jacobian code was cross-checked against an affine reference on 900 random scalars over three instances (0 mismatches, [n]G = O).
- **Negative control:** with `MTFO_CORRUPT=1`, found[0] is incremented before verification, on W22-3, T = 1024, seed 11, for each of the 6 arms. The binary reported `ok = 1023/1024` in all six, and my checker flagged exactly target 0 in all six.
- **No leakage:**
  - `xs` (the planted logs), `plant` and the uncounted `f0` are used only to plant targets (lines 270-285) and in the final check (line 972).
  - No arm reads them.
  - The solver RNG is seeded from `seed` only.
- **Unsampled runs:** for the other 2004 runs the certificate records the binary's own counted check (`ok == targets`), which the replay reproduces. Those runs are marked `scalar_independent: false`.

## 6. Fairness (check 5)

**6.1 The new code.** The v1-to-v2 diff of `mt_fieldops.rs` (`3434954e47..6b0ac6f192`) has exactly four hunks:

- header comments;
- one `f.hash()` in ic4;
- the `rho2x` arm;
- the `rhoseq` arm.

`rho`, `rho2` and `bsgs2` are byte-identical to the v1 harness, which the v1 validator checked line by line.

- **`rho2x`:**
  - `step_x` returns the full P + A and x(P - A) for one inversion, at a cost of I + 3M + 2S. The sibling slope (-y2 - y1)/(x2 - x1) is correct.
  - Phase 1 stores sibling entries as (log, NOY, (prev_x, prev_y, arm)). On a phase-2 hit, `full_y` recomputes such an entry with a counted `padd`.
  - A hit on the target-side sibling recomputes its y with a counted `padd`.
  - The logs are right: the sibling is (pa - a_A)G + Q, giving Q = eps a2 - (pa - a_A).
  - Every insert and lookup calls `f.hash()`.
- **`rhoseq`:**
  - Phase 1 does one counted `padd` and one counted hash per entry, over {G, 2G, ..., NG}. N < n/2, so the x-classes are distinct, and the database yields exactly 1.000 entries per step.
  - Phase 2 is identical to `rho`.
- **ic4 change:** one `f.hash()` per table hit in phase 1, before the `seen_pairs` dedupe, as the v1 validator asked. It raises IC4's cost.
- **Uncounted in every arm, as in v1:**
  - F_p additions, subtractions and negations;
  - mod-n log arithmetic;
  - at most one mod-n inversion per component in IC4's solve.

  The source header discloses this; RESULTS.md does not. The rho arms do more mod-n work per step than IC4 does per trial, so omitting it is conservative for the IC4-cheaper claim.

**6.2 F1: per-target walk start in every rho arm (asymmetry, measured).**

- **The asymmetry:**
  - Each rho arm starts every target walk at Q + bank[k1] + bank[k2], which costs two counted affine additions. It repeats this at every 32-step restart.
  - IC4's trials x(Q +- W_k) and bsgs2's giants use progressions shared by all targets, so they pay no per-target start.
  - An anchored walk can start at Q itself. Q is uniformly random, the 32-arm walk does not cycle within the 12 to 92 steps per target observed here, and each step is an independent database lookup.
- **Measurement:** the instrumented build accumulates the counters of those additions. At kappa 0.7 they amount to:
  - 1.07 to 2.07 starts per target;
  - 2.1 to 4.1 GAE/target;
  - **2.5% to 3.2% of each rho run's total cost** (I = 10M, h = 0).
- **Counterfactual (validator estimate, not a measurement):**
  - I subtracted the measured start/restart counters from every rho run, recomputed rhobest, and applied the pre-registered rule.
  - IC4 stays **cheaper at T = 16384 for every I/M and h (medians 0.932 to 0.978, largest upper bound 0.982)**.
  - T = 4096 becomes **dearer at I = 10M and 30M** and stays unranked at 3M.
  - T = 1024 stays dearer.
- **Other overheads, not quantified:**
  - Phase-1 walk waste: rho2/rho2x store 1.64 to 1.87 x-classes per step, against an ideal of 2.
  - Phase-1 restarts: 1.2% to 2.5% of cost.
  - Sibling self-duplicates: when the step's arm equals the previous step's arm, x(prev - A) is the previous walk point. That happens on about 4.6% of steps by the arm-draw probability; this is estimated, not measured.

  All three can only make a stronger rho cheaper, so the measured margin is an upper bound on IC4's lead over rho variants in this family.

**6.3 F2: the verification constant (undisclosed).**

- Every arm's I/M/S includes the counted double-and-add check of every recovered log.
- It is identical across the 16 runs of each configuration (132/132) and costs 31.6 (22 b) to 36.3 (25 b) GAE/target at I = 10M. For IC4 at 24 b, T = 16384 that is 34.7 of 63.8 GAE/target (54%).
- No attack pays this cost, and including it in both arms pulls every ratio toward 1.
- Without it:
  - IC4/rhobest is 0.840 to 0.929 at T = 16384, 0.941 to 1.045 at T = 4096 and 1.078 to 1.209 at T = 1024;
  - IC4/bsgs2 is **1.49 to 2.04** (measured: 1.18 to 1.80);
  - all section-5a verdicts are unchanged.

**6.4 Sensitivity table, I = 10M** (median IC4/rhobest at h = 0 / h = 1, and the pre-registered verdict over h in {0, 0.1, 1}; own bootstrap, B = 20000):

| cell | as measured | verdict | verify removed | verdict | rho start removed | verdict | both | verdict |
|---|---|---|---|---|---|---|---|---|
| 22 b, T = 1024 | 1.053 / 1.120 | dearer | 1.080 / 1.143 | dearer | 1.084 / 1.136 | dearer | 1.128 / 1.163 | dearer |
| 24 b, T = 1024 | 1.106 / 1.182 | dearer | 1.136 / 1.202 | dearer | 1.137 / 1.197 | dearer | 1.177 / 1.218 | dearer |
| 24 b, T = 4096 | 0.976 / 1.016 | unranked | 0.962 / 1.019 | unranked | 1.004 / 1.030 | dearer | 1.006 / 1.037 | dearer |
| 24 b, T = 16384 | 0.933 / 0.942 | cheaper | 0.865 / 0.919 | cheaper | 0.964 / 0.960 | cheaper | 0.924 / 0.943 | cheaper |
| 25 b, T = 4096 | 0.987 / 1.033 | unranked | 0.982 / 1.038 | unranked | 1.014 / 1.046 | dearer | 1.021 / 1.053 | dearer |
| 25 b, T = 16384 | 0.926 / 0.938 | cheaper | 0.868 / 0.920 | cheaper | 0.953 / 0.953 | cheaper | 0.914 / 0.938 | cheaper |

Verdict counts over all 18 (cell, I/M) vs rhobest:

| scenario | cheaper / unranked / dearer |
|---|---|
| as measured | 6 / 6 / 6 |
| verify removed | 6 / 6 / 6 |
| rho start removed | 6 / 2 / 10 |
| both | 6 / 2 / 10 |

**6.5 Other controls:**

- The rhobest selection (minimum over 12 runs per configuration and price) favours rho, as declared. That is conservative for "cheaper" and slightly optimistic for rho in the "dearer" cells.
- bsgs2 is priced in the same layer and is cheaper than IC4 everywhere.
- No external implementation is compared, and none is claimed.

## 7. Recompute (check 6)

**Method.** My own script reads `runs.json` only:

- Cost per target = [(c_I I + M + S)/(c_I + 3) + h (H + L)]/T, as in PROTOCOL.md.
- rhobest is the per-configuration minimum over the 12 rho runs at each (c_I, h); bsgs2 is the minimum over its 3 runs.
- The statistic is the median of the 22 per-configuration ratios.
- CIs come from:
  - my numpy bootstrap: B = 20000, seeds 0 and 12345, 2.5/97.5 percentiles;
  - a re-implementation of the frozen procedure, to check the reported bounds: B = 4000, Python `random.Random(1)`, sorted medians at indices 100 and 3899.

**Agreement:**

- 324/324 `summary.json` medians agree with a maximum relative difference of **0.0**.
- The reported CI bounds agree exactly with the frozen procedure (maximum difference 0.0).
- My bootstrap bounds are within 0.0028 of the reported ones.
- `ic`, `ref`, `wins` and `n` agree in every entry.
- **0 section-5a verdict disagreements** across all 108 (ref, c_I, cell) combinations, under the reported bounds, my two bootstrap seeds and the reproduced procedure.
- `analysis.txt` agrees on 324/324 entries.
- The RESULTS.md headline table agrees on 84/84 numbers and verdicts at their stated rounding.
- All 10 bench-record metrics agree.

**Headline, I = 10M** (median, my 95% bootstrap; 5a verdict claimed / mine):

| cell | IC4/rhobest h = 0 | h = 0.1 | h = 1 | 5a vs rhobest | IC4/bsgs2 h = 0 | h = 0.1 | h = 1 | 5a vs bsgs2 |
|---|---|---|---|---|---|---|---|---|
| 22 b, T = 1024 | 1.0534 [1.0405, 1.0585] | 1.0656 [1.0515, 1.0709] | 1.1199 [1.1007, 1.1268] | dearer / dearer | 1.4532 [1.4482, 1.4615] | 1.4788 [1.4734, 1.4880] | 1.5970 [1.5904, 1.6116] | dearer / dearer |
| 24 b, T = 1024 | 1.1063 [1.0990, 1.1184] | 1.1215 [1.1139, 1.1342] | 1.1825 [1.1737, 1.1981] | dearer / dearer | 1.6449 [1.6305, 1.6544] | 1.6703 [1.6552, 1.6804] | 1.7734 [1.7560, 1.7863] | dearer / dearer |
| 24 b, T = 4096 | 0.9757 [0.9724, 0.9803] | 0.9830 [0.9793, 0.9878] | 1.0157 [1.0104, 1.0221] | unranked / unranked | 1.3296 [1.3238, 1.3362] | 1.3516 [1.3454, 1.3589] | 1.4557 [1.4479, 1.4657] | dearer / dearer |
| 24 b, T = 16384 | 0.9334 [0.9316, 0.9371] | 0.9349 [0.9328, 0.9389] | 0.9424 [0.9392, 0.9482] | cheaper / cheaper | 1.1849 [1.1831, 1.1885] | 1.2024 [1.2004, 1.2064] | 1.3006 [1.2973, 1.3072] | dearer / dearer |
| 25 b, T = 4096 | 0.9875 [0.9832, 0.9913] | 0.9961 [0.9915, 1.0001] | 1.0327 [1.0270, 1.0378] | unranked / unranked | 1.3966 [1.3903, 1.3982] | 1.4182 [1.4115, 1.4200] | 1.5130 [1.5050, 1.5160] | dearer / dearer |
| 25 b, T = 16384 | 0.9256 [0.9234, 0.9281] | 0.9278 [0.9254, 0.9305] | 0.9384 [0.9352, 0.9418] | cheaper / cheaper | 1.2268 [1.2242, 1.2281] | 1.2459 [1.2430, 1.2474] | 1.3455 [1.3411, 1.3477] | dearer / dearer |

At I = 3M and 30M the verdicts are identical: vs rhobest, 6 cheaper / 6 unranked / 6 dearer over all three I/M, and vs bsgs2, 18 dearer.

**The proposed row's numbers, recomputed:**

| claim | recomputed |
|---|---|
| T = 16384 range | 0.9083 to 0.9494, i.e. "0.91 to 0.95" |
| T = 1024 range | 1.0533 to 1.1881, i.e. "1.05x to 1.19x" |
| T = 4096 at h = 1 | 1.0058 to 1.0385 |
| bsgs2 range | 1.1754 to 1.7971, i.e. "1.18x to 1.80x" |
| vs the anchored `rho` | 17 cheaper, 1 unranked ("cheaper in most cells") |

**Which rho arm is cheapest:** rho2x in 51 of the 54 (cell, I/M, h) settings, in all 22 configurations of each. rho2 is cheapest in the other 3 (24 b, T = 16384, I = 30M). rho and rhoseq are never cheapest.

**Phases, I = 10M, h = 0** (GAE/target, medians over the 22 configurations; parameters fixed at kappa = 0.7 and mf = 1.0):

| cell | IC4 fb / table / relations / targets / verify = total | rho2x db / targets / verify = total (start/restart part of targets; starts per target) | rhoseq db / targets (+ verify) = total | bsgs2 baby / targets (+ verify) = total |
|---|---|---|---|---|
| 22 b, T = 1024 | 3.7 / 24.6 / 11.7 / 27.7 / 31.6 = 99.0 | 31.4 / 31.9 / 31.6 = 94.8 (2.7; 1.36) | 46.8 / 48.8 = 127.2 | 17.5 / 19.2 = 68.2 |
| 24 b, T = 1024 | 5.7 / 49.1 / 31.6 / 53.2 / 34.7 = 174.4 | 60.3 / 61.7 / 34.7 = 156.6 (4.1; 2.07) | 91.8 / 97.0 = 223.5 | 34.8 / 36.5 = 106.0 |
| 24 b, T = 4096 | 2.0 / 24.5 / 5.7 / 27.5 / 34.7 = 94.6 | 30.8 / 31.4 / 34.7 = 96.9 (2.7; 1.35) | 45.3 / 48.7 = 128.7 | 17.4 / 19.1 = 71.1 |
| 24 b, T = 16384 | 0.7 / 12.2 / 1.1 / 15.2 / 34.7 = 63.8 | 16.7 / 17.1 / 34.7 = 68.4 (2.1; 1.07) | 22.5 / 25.0 = 82.2 | 8.7 / 10.6 = 53.9 |
| 25 b, T = 4096 | 3.0 / 34.7 / 9.3 / 38.3 / 36.3 = 121.6 | 43.4 / 43.7 / 36.3 = 123.3 (3.3; 1.64) | 63.9 / 68.4 = 168.6 | 24.5 / 26.3 = 87.1 |
| 25 b, T = 16384 | 1.1 / 17.3 / 1.8 / 20.5 / 36.3 = 76.8 | 23.6 / 23.1 / 36.3 = 83.0 (2.4; 1.17) | 31.8 / 34.9 = 103.0 | 12.2 / 14.1 = 62.6 |

IC4's solve phase costs no field operations; its L count enters only through h. These totals use fixed parameters, so they differ slightly from the per-configuration-best totals behind the ratios.

## 8. Required amendments (RESULTS.md and the row text)

1. **Scope "strongest rho" (F4, F1).** Write "the cheapest of the four pre-registered rho arms in this harness (rho, rho2, rho2x, rhoseq x kappa 0.5/0.7/1.0, per configuration and price)". Add:
   - "These arms start each target walk at Q plus two bank points and restart every 32 steps (2.5% to 3.2% of their cost). In a validator sensitivity with that start removed, IC4 is still cheaper at T = 16384 (0.93x to 0.98x) and is dearer at T = 4096 for I = 10M and 30M."
   - "Phase-1 walk waste and sibling self-duplicates are further unexploited rho savings, not quantified."

   Replace the spot-check sentence with the full count: rho2x is cheapest in 51/54 settings and rho2 in 3/54 (24 b, T = 16384, I = 30M).
2. **Disclose the verification constant (F2).** "Every arm includes the counted verification of every recovered log, identical across arms (31.6 to 36.3 GAE/target). Without it, IC4/strongest rho is 0.84x to 0.93x at T = 16384, and IC4/bsgs2 is 1.49x to 2.04x; the verdicts are unchanged."
3. **Add the phase table (F3)** from §7.
4. **T = 4096 wording (F5).** "Cheaper at h = 0 and 0.1, dearer at h = 1" holds in medians for every I/M at 24 b and for I = 3M at 25 b. At 25 b:
   - at I = 30M, the h = 0.1 median is 1.005 and the h = 0 interval includes 1;
   - at I = 10M, the h = 0.1 interval includes 1.

   Say "unranked: the order changes inside the band".
5. **Label "Where IC4 wins and why" as interpretation (F10)**, and add the section-3 class: the v2 IC4 change is an accounting correction that raises IC4's cost, and the row is a cross-method comparison, not an improvement over an IC baseline.
6. **Limits:**
   - say that this is not an ecbench session, with no spec or `verify --replay` receipt (F8);
   - say that F_p additions and mod-n arithmetic are uncounted in every arm (F12);
   - add "prime-field toy curves; no ECC2K-130 / m = 83 evidence; no transfer claimed" (F13).
7. **Process (F6):**
   - Move `results/holdout/rows.jsonl` (1,446,974 bytes) to the artifact store under AGENTS.md section 13. Upload it and verify sha256 `cafb8c71...4997a7a` before removing it from Git: the phases and argv that this validation replayed exist only there.
   - Close the bench record (`uploaded: false`, `status: open`, `validation: null` at the validated commit).
8. **Bindings (F7):** record the dirty-tree state and the toolchain with the rows, as the v1 validator asked.

**Suggested row text:**

> IC4 is cheaper per target than the cheapest of the 12 pre-registered rho runs (rho/rho2/rho2x/rhoseq x kappa 0.5/0.7/1.0, per configuration and per price) at T = 16384 on 24- and 25-bit prime-order curves, at I/M in {3, 10, 30} and h in {0, 0.1, 1}: median ratios 0.91 to 0.95 including the verification constant common to all arms, 0.84 to 0.93 without it. It is unranked at T = 4096 and dearer at T = 1024 (1.05 to 1.19). It is dearer than bsgs2 everywhere (1.18 to 1.80; 1.49 to 2.04 without verification). Validator sensitivity: with the rho arms' per-target walk start removed, the T = 16384 ratios are 0.93 to 0.98 (still cheaper), and T = 4096 is dearer at I = 10M and 30M. [pcc: cite the claim id once recorded]

## 9. Gate notes

- **This directory holds** the replay certificate (sha256 `e17fa9d7431b324dacc44dae56a9f9c19daa7307f1526f55b0b772fd66343452`) and the validator report (sha256 `67597495448abb3e67819a2d44f4e0fafee24eedd5db65ff5a644e5075e14d87`).
  - Both pass the structural checks in `scripts/validation_gate.py` (`check_replay_certificate`, `check_validator_report`).
  - The report's `raw_data` pins `../results/holdout/runs.json` (sha256 `54583863e525805c79825b84c36e98e6b2f155674300332d2c4a0ec2349f5a3a`).
- **Rho reference, for `VALIDATION.json`:**
  - It is the protocol-named rhobest, in the same binary and the same GAE unit, with the band at h in {0, 0.1, 1}; ranks hold only where the order holds across the band.
  - The symmetry is negation (`kind: negation`), the only automorphism on these curves (§3).
  - "strongest_available" is true in the gate's sense (symmetry), subject to the F1 caveat on implementation overheads.
- **Not written by me:** `VALIDATION.json`, the pcc claim, the benchmarks row and the leaderboard regeneration.

## 10. Not checked

- Wall-clock behaviour: no claim uses it.
- A distinguished-point rho, other walk families, and arm-count or bank-size sweeps.
- A rho variant with the start removed, *measured* rather than estimated (§6.2 is a counterfactual from measured counters).
- Rho variants with the phase-1 waste or the sibling duplicates removed.
- IC4 at g > 2.0, outside the frozen grid.
- Montgomery batching of inversions: it is assumed, not implemented, for c_I = 3 in every arm.
- isolab-eks, ecdb, research-kb and the pcc ledger: none was engaged by this validation.
- Commit `2ad87dd980` (leaderboard builder and pages).

## 11. Reproduction (validator scratch, not committed)

The scripts are under `/tmp/claude-0/-home-user/aeefaead-5838-5329-9e0a-13b7d93fe240/scratchpad/v2val/tools/`. That directory is ephemeral; the descriptions above are enough to rebuild them.

| script | purpose |
|---|---|
| `instances_check.py` | validity and freshness scan |
| `projection_grid.py` | runs.json projection and grid |
| `replay.py` | full replays with any binary |
| `patch_dbg.py` | instrumentation: dump, corruption, start/restart counters |
| `replay_dbg.py` | instrumented replay and the sample |
| `eccheck.py` | independent discrete-log verification and negative control |
| `recompute.py` | medians, CIs, verdicts, RESULTS/summary/analysis comparison |
| `sens.py` | phases, verification constant, rho-start sensitivity |
| `make_artifacts.py` | assembles the two JSON artifacts |

Outputs are in `out/`, dumps in `dumps/`, and the scratch builds in `build/exact` and `build/dbg`.
