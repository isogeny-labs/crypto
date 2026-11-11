# Validator report: post-merge re-measurement of Eh / D / Dw (results/walk_postmerge)

Validator: independent session, 2026-11-07 (Sonnet 5.5). Scratch evidence is NOT committed; it is re-derivable and lives in
`/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/val2/` (`stats.py`, `replay.py`, `cmp_replay.py`,
`verify_inst.py`, `probe.py`, `probe_sum.py`, `replay_v4.py`, `cgsum.py`, `cgcalls.py`, result JSONs `passA/B/C/D`, `pass_s1/s2`,
`probe_tune.json`, `probe_eval_posthoc.json`, `probe_eval_D16.json`, `probe_A_new.json`, `probe_A_s0.json`, callgrind outputs in `cg/`).
I wrote all of these myself; none imports producer code. No file under `results/` other than this report was written. I did NOT run
`cargo build`, `cargo check` or `cargo test`, and I did not use a separate `CARGO_TARGET_DIR`: the binary under test was never rebuilt
or replaced, and the merged tree's unit tests were not run.

## Binding: what was validated

| item | value |
|---|---|
| Branch / HEAD | `claude/prime-field-factor-base-calculus-wy2own`, `9f66489f2c` ("Re-measure the frozen v5 evaluation against the merged baseline"), working tree clean at start and at end. The branch did not move during validation. |
| Merge | `2ee1e6c494` (05:15:21Z), merge-base of its parents `e1826419b7`. `git diff 58b0a26c12 HEAD` over `src/`, `Cargo.toml`, `Cargo.lock`, `src/bin`: only `src/cryptanalysis/ec_index_calculus.rs` (+107), `src/ecc/point.rs` (+52, one new method), and three `src/zk/*` files. No change to `Point::add`, `Point::scalar_mul`, `add_vartime`, `scalar_mul_vartime`. |
| Binary used (not rebuilt) | `target/release/ic`, 59,148,688 bytes, mtime 05:23:26.71Z, sha256 `d5e24e8db00a3c48aea9f38baa8667afb1d01dfcd266786d088a355d639a13f1`, self-reported `binary_blake3` `ea312aff993e31f2645af6382628590d01c5596ec50985da7dc39b15a0c1dd45`, `git_commit: null`, `git_dirty: null`. No file under `src/`, `Cargo.toml` or `Cargo.lock` is newer than the binary (`ec_index_calculus.rs` mtime 05:14:31Z, `point.rs` 05:12:02Z). Symbols `relation_point`, `Point::double_scalar_mul_vartime`, `SampleWalk::step` are present; the `IC_SHAMIR` knob behaves as the merged source says (this report, section 5). The binary cannot be bound to the commit by embedded metadata (see F9). |
| Order of events (mtimes) | binary built 05:23:26; `frozen.json` copied 05:23:38; `rows.jsonl` finished 05:23:53; `analysis.txt` 05:24:02; commit 05:24:26. The run used a binary built before it. |
| Artifact sha256 | `results/walk_postmerge/rows.jsonl` `20063a093770abaf5a6ab0e3b9fe9bee325b790d5a3eddc44fe53b1500577735`; `results/walk_postmerge/frozen.json` `25bfcc300bacf7cd046e43f7c0b7061a8a34c0108334c739feace057b9f1cc16` (byte-identical to `results/walk/frozen.json`, committed in `0b345b6c73` at 02:39:17Z, before any eval); `results/walk_postmerge/analysis.txt` `2c8c0509b8c81be1dbe48e71afa030413d983bbfe5062024a043d6d86db0f555`; `results/walk/rows.jsonl` (pre-merge) `ff9a65b72d8ca5b20ee9fd54160a27f819b196adf335a32d1e860d97ca1dcf5f` (same hash the earlier validator recorded; unchanged since `58b0a26c12`); `run_walk.py` `7cc306d647b5a1e2b3a0f49545b341e9851231aadcc744e7a9a5494066709839` (unchanged); `results/instances_v5/MANIFEST.json` `001437bfb91b58f5146574c611c5a30bd5d0681c35165baed9bf925c8587a646` (90 of 90 instance files hash to it; unchanged since `91e77b589e`, 02:15:06Z). |
| Host | 4-core VM, 16 GB, load average 0.2 to 0.4 during my runs; all runs serial; not isolated (no L2 / replay / A/A admission). VM up since about 03:28Z (uptime at 05:25:46Z was 1:57), so the pre-merge panel (02:39Z) very likely ran on a different VM instance than the post-merge panel (05:23Z) and my runs. |
| Environment of replays | all `IC_*` variables removed from the ambient environment (none were set), then the arm's own variables added, exactly as `run_walk.py`; command line `--json solve-prime-file --input T --seed S --fb-size FB --extra-relations 2 --rho-steps 0 --max-trials-per-relation 200000 --max-relation-attempts 512`. |
| Not done | no isolab-eks run, no `bench.record/v1`, no pcc claim, no `VALIDATION.json`, no matched rho with the 5a band. The post-merge panel remains "unvalidated (pre-gate)" in the repository's sense. m = 83 / ECC2K-130 disclosure rules: not applicable (prime-field toy sizes). |

## Disposition

| claim | disposition |
|---|---|
| The 360-row post-merge panel as frozen (rows, ratios, absolute medians, counts) | **validated**, scoped to: solver stage timer and process wall time, one local unisolated host, 14 to 16 bit fields (group order up to 2^17), pre-gate. All numbers recompute; every row replays deterministically. |
| "The merge did not change which relations are found or the correctness of the default path" | **validated** (360/360 pre vs post rows; 360/360 under all four `IC_SHAMIR` modes; 640/640 pre-walk v4 rows including 40 shipped-default rows). |
| "The walk is about 6.6x faster in solver time and 1.9x end to end than the best hoisted sweep on the merged baseline" | The literal ratio is **supported**: 1 / 0.152 = 6.6x (by-instance bootstrap 5.8x to 7.6x) and 1 / 0.537 = 1.86x (1.63x to 2.10x), and the hoisted sweep is within 2% of its own best fb. As a statement that the walk is an improvement on the merged baseline it is **overstated** (F1, F2): on the merged tree a table retuned to fb 16 with fresh random sampling is 1.45x faster than the walk in solver time and level end to end, and the walk arm does not use the merged fast arithmetic. The "200x vs shipped default" figure is **stale**: about 76x on the merged binary (F4). |
| New prose in `RESULTS_walk.md` and `CAMPAIGN_SUMMARY.md` | **partly misleading** (F3 to F9, of which F7 is a framing problem). Numbers quoted from the panel are correct. |

## 1. All 360 rows recovered; no timeouts, no missing timings

| id | requirement | result | evidence |
|---|---|---|---|
| 1a | 360 rows, 360 distinct (name, seed, arm), 60 instances (idx 10 to 29 of W14, W15, W16) x seeds 1, 2 x 3 arms, 120 rows per arm, fb 256 / 48 / 16 | **PASS** | `stats.py` on `rows.jsonl` |
| 1b | `ok` true in 360/360; no missing `total_ms`, `trials`, `wall_ms`; no timeout | **PASS** | max `wall_ms` 118.2 ms against the runner's 120 s timeout; a timeout would store `ok: false` and a null `total_ms` |
| 1c | `ok` is only the runner's comparison with the planted log: re-verified independently | **PASS** | (i) my replays: scalar string equals `fixture.known_log` in 360/360 rows on each of six full passes (default x2, `IC_SHAMIR`=0 x2, =1, =2), every run `status: complete`, `verified: true`; (ii) `verify_inst.py`, own affine EC arithmetic: for all 60 evaluation instances p and n prime, curve nonsingular, G and Q on the curve, nG = O, 0 < known_log < n, known_log * G = Q, and #E (Legendre sum) = n * cofactor = `group_order`; 60/60 pass, 60 distinct (p, a, b, G, Q); p from 16381 to 65521 |

## 2. Recomputed medians and ratios (from `rows.jsonl` alone)

Absolute medians over 120 rows per arm: Eh 24.85 / 51.07 ms (solver / process), D 6.23 / 30.36, Dw 3.71 / 28.61. Median trials 262.0 / 377.5 / 1113.5; median relations 258 / 50 / 18. All six figures equal the document's (24.8 / 51.1, 6.2 / 30.4, 3.7 / 28.6): **PASS**.

Median of 120 paired ratios, with 95% bootstrap (20,000 resamples) by pair and by instance (60 clusters, both seeds of an instance resampled together):

| ratio | solver median | solver by pair | solver by instance | process median | process by pair | process by instance | wins solver / process |
|---|---|---|---|---|---|---|---|
| Dw / Eh | 0.152 | [0.132, 0.170] | [0.131, 0.173] | 0.537 | [0.483, 0.600] | [0.477, 0.613] | 120/120, 119/120 |
| D / Eh | 0.260 | [0.220, 0.300] | [0.211, 0.317] | 0.593 | [0.534, 0.649] | [0.512, 0.665] | 120/120, 113/120 |
| Dw / D | 0.587 | [0.541, 0.659] | [0.538, 0.666] | 0.937 | [0.908, 0.959] | [0.908, 0.959] | 107/120, 84/120 |

**PASS** for all claimed medians (0.152 / 0.537, 0.260 / 0.593, 0.587 / 0.937) and for the producer's intervals (differences of at most 0.005 are bootstrap-seed noise). The by-instance intervals are slightly wider (Dw/Eh solver upper end 0.173 against 0.170) and change no conclusion. Other aggregates for context: ratio of medians Dw/Eh 0.149 / 0.560, ratio of sums 0.135 / 0.510, geometric mean of ratios 0.152 / 0.539. Pre-merge column recomputed from `results/walk/rows.jsonl`: Eh 26.90 / 50.53, D 13.91 / 37.82, Dw 3.34 / 27.11 ms; Dw/Eh 0.124 / 0.520, Dw/D 0.249 / 0.720, D/Eh 0.494 / 0.718, all equal to the "before" column of the new table: **PASS**.

Per group-order size (pairs 18 / 42 / 44 / 16 for r_bits 14 / 15 / 16 / 17), solver / process:

| r_bits | Dw/Eh pre-merge | Dw/Eh post-merge | Dw/D pre-merge | Dw/D post-merge |
|---|---|---|---|---|
| 14 | 0.134 / 0.626 | 0.221 / 0.700 | 0.250 / 0.840 | 0.470 / 0.902 |
| 15 | 0.138 / 0.627 | 0.192 / 0.691 | 0.254 / 0.754 | 0.483 / 0.922 |
| 16 | 0.107 / 0.426 | 0.129 / 0.458 | 0.251 / 0.682 | 0.723 / 0.968 |
| 17 | 0.104 / 0.366 | 0.105 / 0.402 | 0.200 / 0.594 | 0.790 / 0.963 |

The pooled 0.152 hides a size dependence: the post-merge walk advantage over the hoisted sweep ranges from 4.5x (r_bits 14) to 9.5x (r_bits 17) in solver time and from 1.4x to 2.5x end to end, and the advantage over the table falls toward parity as size grows (F6). Process minus solver (constant overhead) medians: Eh 25.4, D 24.3, Dw 24.0 ms, arm-independent, so the process ratios are compressed by a constant unrelated to the algorithm (callgrind: `compute_binary_hash` is 72% of all instructions in a D run).

## 3. Determinism and timing replay

I replayed **all 360 rows** with the exact command line and environment, in six passes (shuffled with my own seeds), plus 640 rows of the pre-walk v4 panel (section 4).

| id | requirement | result | evidence |
|---|---|---|---|
| 3a | scalar, `trials_total`, `relations_collected` equal the stored values | **PASS** | pass A (default), pass B (default, other order): 360/360 identical in scalar, trials and rels; scalar equals `known_log` 360/360. Medians: trials Eh 262.0, D 377.5, Dw 1113.5, identical to the pre-merge medians. |
| 3b | pre-merge trial counts reproduced | **PASS** | per-row `trials` and `rels` of `walk/rows.jsonl` and `walk_postmerge/rows.jsonl` are equal in 360/360 (sum of trials Eh 32,726, D 52,170, Dw 159,044 in both). No difference to report. |
| 3c | timing within about 30% (median over rows) | **PASS** | replay / recorded solver median 1.001 (pass A: Eh 1.022, D 0.995, Dw 0.995) and 0.967 (pass B: 0.956, 0.969, 0.967); process 1.004 and 0.961. Replay paired medians, pass A / pass B: Dw/Eh 0.145 / 0.155 solver, 0.548 / 0.593 process; D/Eh 0.242 / 0.267, 0.600 / 0.644; Dw/D 0.594 / 0.584, 0.924 / 0.923. All inside the producer's intervals. Pass-to-pass noise of a median is about 4%. |

Run-to-run process-level ratios move more than solver ratios (Dw/Eh process 0.548 to 0.593 across my two default passes, 0.561 to 0.584 in later probes); 1.9x end to end is the favourable end of a 1.7x to 1.9x range.

## 4. The merge did not change which relations are found or the default path

| id | requirement | result | evidence |
|---|---|---|---|
| 4a | pre-merge vs post-merge rows for the same (name, seed, arm): ok and trials agree | **PASS** | 360 keys identical, `ok` equal and true in 360/360, `trials` equal 360/360, `rels` equal 360/360, `fb` and `r_bits` equal |
| 4b | all four relation-point modes give the same relations | **PASS** | `IC_SHAMIR` unset (mode 3, joint word-Jacobian), 0 (two `scalar_mul` + `add`, the pre-merge path), 1 (affine interleave), 2 (two `scalar_mul_vartime` + `add_vartime`): scalar, trials, rels identical in 360/360 rows for each pair of modes, scalar equals `known_log` 360/360 in each |
| 4c | default path of the shipped solver (no env, fb 8, extra 64) and the earlier arms unchanged | **PASS** | pre-walk v4 rows recovered from git (`1ec774fa45:.../results/final/rows.jsonl`, sha256 `1fb8a766d504...ee04d`) replayed on the merged binary: A shipped default 40/40, B 120/120, C 120/120, Bh 120/120, Eh 120/120, D 120/120 identical in scalar-vs-known_log, trials and rels. Also A on 60 v5 instances: trials identical under `IC_SHAMIR`=0 and default, 60/60 recovered |
| 4d | exactness of `Point::double_scalar_mul_vartime` | **PASS (by code reading and end-to-end equality; unit tests not run)** | `src/ecc/point.rs` lines 201 to 252 and the `Word` helpers (lines 597 to 820): `jacobian_add_affine` handles `h == 0` (doubling when `r == 0`, identity otherwise), `jacobian_double` handles `y == 0`, an infinite `G + Q` is skipped, a missing accumulator is initialised from the first addend. Together with 4b (about 244,000 trials per pass, four passes), I find no defect. The two new unit tests were not executed (no build allowed). |

## 5. Which arms use the faster arithmetic, and why D got faster and Eh did not

**Code reading** (`src/cryptanalysis/ec_index_calculus.rs` at HEAD):

* `relation_point` (line 931) is called at exactly two sites: `find_one_relation_sweep_with_index` (line 1043; the Eh arm and the shipped default) and `find_one_relation_table_with_rng` (line 1827; the D arm). Default mode 3 is `Point::double_scalar_mul_vartime` (one word-Jacobian pass, `Word` arithmetic with Euclid inversion).
* `find_one_relation_table_walk` / `SampleWalk` (lines 1859 to 1978; the Dw arm) never call `relation_point`. `SampleWalk::new` and `restart` compute `g.scalar_mul(a).add(&q.scalar_mul(b))`, i.e. exactly the pre-merge mode-0 expression; `step` does `self.r.add(pt)`. These are the constant-time BigUint `Point::scalar_mul` / `Point::add` (Fermat inversion). The pre-existing fast `add_vartime` / `scalar_mul_vartime` are not used by the walk.
* `build_pair_sum_table` (line 1777) also uses `Point::add` (constant-time). So in the D and Dw arms the table build is on the slow path in both arms and is untouched by the merge.
* `MODE` is a process-wide `OnceLock` read once. Nothing in `IC_SUM_TABLE`, `IC_WALK` or `IC_HOIST_SWEEP` reads `IC_SHAMIR`; the interaction is only through which function each arm calls. **The arms are therefore unequal in arithmetic cost**: Eh and D sample with the fast path, Dw samples with the slow path, and D / Dw build their tables on the slow path.

**Same-binary control** (all 360 rows, two passes of `IC_SHAMIR`=0 and two of default, same binary, same host, same hour). Per-pair median solver time, mode 0 to default:

| arm | mode 0 (pre-merge relation path) | default | per-pair speedup | trials |
|---|---|---|---|---|
| Eh | 27.84 ms | 23.18 ms | 1.23x | identical |
| D | 15.45 ms | 6.33 ms | 2.52x | identical |
| Dw | 3.71 ms | 3.63 ms | 0.99x (no effect) | identical |

The control reproduces the pre-merge ratios with the merged binary: with `IC_SHAMIR=0` Dw/Eh = 0.119 to 0.121 (pre-merge recorded 0.124), D/Eh = 0.478 to 0.519 (0.494), Dw/D = 0.232 to 0.251 (0.249). So the whole change in the ratios is the `relation_point` switch; the host change cancels in within-run ratios. In absolute terms every arm, including Dw (whose code did not change), is 1.105 to 1.114x slower with `IC_SHAMIR=0` than the pre-merge recorded (per-pair median), i.e. the merged-binary / new-VM environment is about 11% slower for identical work. That is why the document's Dw column goes from 3.3 to 3.7 ms and why D's measured 2.2x (13.9 to 6.2 ms) is a 2.5x arithmetic effect divided by an 11% environment drift (F5).

**Instruction counts** (callgrind on the real binary, one instance W15-17, seed 2; instruction counts are deterministic; `cgsum.py`, `cgcalls.py`). Inclusive Ir of `ec_index_calculus_dlp_staged_with_rng`:

| arm (fb, trials) | merged default | `IC_SHAMIR=0` | relation_point calls, Ir (default) | relation_point Ir (mode 0) | other dominant costs (default) |
|---|---|---|---|---|---|
| Eh (256, 262) | 178.6 M | 242.6 M (1.36x) | 262 calls, 2.63 M = 1.5% of solver | 66.6 M = 27.4% | sweep in `find_one_relation_sweep_with_index` 161.2 M = 90.2%: 9,347 `semaev_s3_in_x3` evaluations (35.7 per trial), 9,882 `sqrt_mod_p`, 9,965 field inversions, 18,556 `try_finalise`; `FieldElement::inv` 39.8%, `semaev` 16.9%, `sqrt_mod_p` 14.2%, `try_finalise` 6.5%, elimination 8.0% |
| D (48, 353) | 53.0 M | 138.9 M (2.62x) | 353 calls, 3.53 M = 6.7% | 89.5 M = 64.4% | `build_pair_sum_table` 47.67 M = **89.9%** (4,704 `Point::add`, identical Ir in both modes: 47,670,373 vs 47,669,660); `FieldElement::inv` 62.3% |
| Dw (16, 1391) | 39.68 M | 39.68 M (identical to 0.001%) | 0 calls | 0 | `SampleWalk::step` 13.75 M = 34.6% (1,391 `Point::add`); `Point::scalar_mul` 19.41 M = **48.9%** (157 calls: 66 arm points + 2 start + 43 restarts x 2 + a few); table build 5.33 M = 13.4% |
| D (16, 971 trials), control, not a producer arm | 16.87 M | not run | 971 calls, 9.67 M = 57% | | `build_pair_sum_table` 5.33 M = 31.6% |

Per trial: a fresh sample costs 10.0k Ir with the merged `relation_point` (253k Ir with mode 0, 25x more). A walk trial costs 9.9k Ir for the step plus 13.9k Ir amortised for the 154 old-path scalar multiplications (restart every 32 steps and the 33 arm points), about 24k Ir. **On the merged arithmetic a walk trial is about 2.4x more expensive than a fresh random trial; before the merge it was about 10x cheaper.** This is the mechanism behind the shrinking Dw/D and behind F1.

**Why D sped up and Eh did not:** each Eh trial is dominated by the factor-base sweep (about 36 semaev + square-root + inversion evaluations per trial, about 610k Ir) and the relation point was only 27% of Eh even on the old path, 1.5% after; the switch removes about 244k of about 926k Ir per trial in the solver (26%; 1.36x fewer instructions on that instance, 1.23x in time). In D the relation point was 64% of solver instructions on the old path (the table build the other 34%), so making it 25x cheaper leaves a solver that is 90% table build (2.6x fewer instructions, 2.5x in time). The document's "the sweep (whose cost is square roots)" is not accurate: square roots are 14% of Eh; inversions are 40% (F5).

## 6. Findings on the claims and the prose

Severity: major = changes the reading of the headline; minor = wording or disclosure.

| id | severity | finding |
|---|---|---|
| F1 | **major** | **The hoisted sweep is no longer the strongest implemented baseline; a retuned table beats the walk on the merged tree.** Exploratory probe by me, clearly not a producer panel and not a new frozen holdout. On the tune instances idx 0 to 9 (30 instances, seeds 1, 2, 60 runs per cell, merged default arithmetic, 960/960 recovered), median solver ms: D fb 8: 3.70, 12: 2.75, **16: 2.54, 20: 2.50**, 24: 2.63, 32: 3.53, **48 (frozen): 5.85**; Dw fb 8: 6.09, 12: 4.10, **16: 3.53**, 24: 3.89, 32: 4.56; Eh fb 128: 27.2, **192: 21.7, 256: 22.2**, 384: 25.8. By the producer's own rule and grid [16, 24, 32, 48, 64, 128], D would be re-frozen at fb 16 (not 48; fb 64 and 128 were not probed, the medians rise monotonically from fb 24 up). Applying that on the 60 evaluation instances post hoc (seeds 1, 2, interleaved, 360/360 recovered): D16 2.58 ms, Dw16 3.73 ms, Eh256 24.06 ms (solver medians); **Dw/D16 = 1.450 [1.387, 1.579] (by-instance bootstrap) solver, 1.009 [0.987, 1.074] process; Dw faster than D16 in 19/120 pairs (solver), 54/120 (process)**; D16/Eh = 0.106 [0.092, 0.122] (9.4x) solver, 0.560 [0.483, 0.604] (1.8x) process. A separate run with fb 20 and 24 gives the same picture (Dw/D20 = 1.472 [1.393, 1.597], Dw/D24 = 1.426). Per size Dw/D16 solver: 1.58, 1.67, 1.34, 1.25 (r_bits 14 to 17), D16 faster at every size. Trial counts are equal (1112 vs 1113.5), so the walk adds no trial saving, only a more expensive trial (section 5). On the merged tree the walk is **not** an improvement over the best implemented variant; the improvement over Eh that the headline reports comes from the merged `relation_point` and a smaller table. |
| F2 | **major** | **The walk arm does not use the merged arithmetic; the arms are unequal** (section 5). Dw's setup and restarts are 48.9% of its instructions on the old constant-time path, identical with or without the merge. So the post-merge Dw is not the best walk the merged code allows (a rough instruction-count estimate: routing the 77 `(a, b)` starts (33 arm points, 1 initial, 43 restarts) through `relation_point` would cut about 18.7 M of 39.7 M Ir, but a walk trial would then cost about the same as a fresh trial, so no advantage over D16 is expected; this was NOT measured). Conversely D / Dw table builds (90% of D) still use constant-time `Point::add`; the pre-existing `add_vartime` is not used. The post-merge ranking of D vs Dw is therefore an artefact of which call sites main's session happened to switch. |
| F3 | minor/major | **Frozen fb not re-tuned; magnitude not stated.** `RESULTS_walk.md` says the settings "were not re-tuned"; `CAMPAIGN_SUMMARY.md` omits it. The staleness is large for D: the frozen fb 48 gives 5.85 ms against 2.50 ms at fb 20 on the tune set (2.3x), so the reported D/Eh = 0.260 and Dw/D = 0.587 are against a D mistuned by 2.3x. Eh (192 to 256 within 2%) and Dw (fb 16 is the optimum of 8, 12, 16, 24, 32) are not stale. |
| F4 | **major (stale figure)** | **"About 200x lower than the shipped default" is a pre-merge number and is wrong for the merged baseline.** The shipped default (A: fb 8, extra 64, sweep with per-call index) also calls `relation_point`. I ran A on the 60 evaluation instances, seed 1, merged binary: solver median **307.6 ms** with the default relation point and 751.8 ms with `IC_SHAMIR=0`, trials identical in 60/60, 60/60 recovered. Per-pair A / Dw (recorded post-merge Dw, seed 1): **76x solver** (180x with `IC_SHAMIR=0`, consistent with the pre-merge 215x = 708.5 / 3.3), **11x process**. Against D16 (2.58 ms) the shipped default is 119x. The document states the 200x is pre-merge but does not give the merged value; it is about 2.6x lower. |
| F5 | minor | **Wording in the new `RESULTS_walk.md` section.** (a) "evidently, the point arithmetic had become faster": only the relation-point call path changed; `Point::add` and `Point::scalar_mul` are unchanged (table build 47.67 M Ir in both modes, Dw 39.68 M Ir in both modes). (b) "the sweep (whose cost is square roots)": square roots are 14.2% of Eh's instructions, inversions 39.8% (section 5). (c) "in counted group operations nothing changed" is unverified: the trial counts are unchanged (360/360), but the relation point now executes a different number of group operations (one interleaved pass, about 1.75 b operations and one inversion against about 3 b + 1 and three inversions); it is "unchanged" only for a counter that prices R as two scalar multiplications plus an addition. (d) The Dw column rising from 3.3 to 3.7 ms is not explained; it is about 11% environment drift (section 5), and the before / after absolute columns mix that drift with the arithmetic change, so only the within-run ratios are comparable across the two columns. (e) The table header "ratio (120 pairs)" covers three absolute-time rows. (f) The merge time is given as 05:12Z; the merge commit is 05:15:21Z (05:12Z is `point.rs`'s mtime). |
| F6 | minor | **Pooled median over an unstratified size mix; pseudo-replication.** The 120 "pairs" are 60 instances x 2 seeds (pairs by size 18 / 42 / 44 / 16). Per-size Dw/Eh runs 0.221 to 0.105 and Dw/D 0.47 to 0.79 (section 2); the old per-size breakdown was in the pre-merge file and is missing from the new section, as are the win counts (Dw < D in 107/120 solver and only 84/120 process pairs, down from 120/120 and 114/120). By-instance intervals are only slightly wider than by-pair ones (Dw/Eh solver [0.131, 0.173] vs [0.132, 0.170]), so this does not change the ratio, but the extrapolation direction matters: Dw/D rises with size (0.47 to 0.79), so the walk's remaining margin over the unretuned table is shrinking, and it is already negative against the retuned one. |
| F7 | **major (framing)** | **`CAMPAIGN_SUMMARY.md` conclusion mixes pre- and post-merge figures and keeps the pre-merge result as the headline.** The one sentence reads "about 8x ... about 1.9x ... about 200x ... (both measured before the merge ...; after the merge ... 0.152x ... about 6.6x ... 0.537x ...)". The leading figures (8x, 200x) are the superseded ones; "about 1.9x" is the pre-merge 1.92x and the post-merge 1.86x is within rounding, so that one carries over, but "8x" does not (6.6x) and "200x" does not (76x, F4). The table above the paragraph (rows "pair-sum table" and "restarted r-adding walk") still gives only pre-merge numbers (0.494x / 0.718x; 0.124 / 0.520) with "independent check: PASS" that applies to the pre-merge binary; the section heading still says "same arithmetic, same binary", which holds within each panel but not across the section. The reason given for the smaller walk gain ("because scalar multiplications became cheap") is incomplete: it omits that the frozen D is mistuned by 2.3x and that a retuned D is faster than the walk (F1, F3). "not independently replayed" is out of date after this report, but the retuning result should travel with it. |
| F8 | minor | **`RESULTS_walk.md` top sections are unmarked as superseded.** "Mechanism" still says the cost of a trial is the two scalar multiplications (7% of D's instructions after the merge) and "Reading" still says "4x over the table and about 8x over the best hoisted sweep"; only the last section updates. The pre-merge validator's own "walk fb 16 is the lowest grid value" caveat was refuted by that validator and is still in "Caveats". |
| F9 | minor | **Provenance.** `rows.jsonl` stores no scalar, binary hash, commit, host, env or tool version; the binary reports `git_commit: null`. The row-to-binary binding rests on mtimes (binary before run, newer than every source) plus exact trial-count reproduction. The `frozen.json` in `walk_postmerge` was committed together with the rows (`9f66489f2c`), so git alone does not show it was fixed first; it is byte-identical to the pre-merge file committed at 02:39:17Z, which closes that. `run_walk.py` removes only `IC_SUM_TABLE` and `IC_HOIST_SWEEP` from the inherited environment, not `IC_WALK` or `IC_SHAMIR`: a stray `IC_SHAMIR` or `IC_WALK` in the producer's shell would silently change arms and is not recorded. The data show it was not set (D 6.23 ms equals my default replay 6.25 ms, not the `IC_SHAMIR=0` 14.9 ms). `analysis.txt` comes from an uncommitted inline script; I reproduced every point estimate and interval (section 2). No `bench.record/v1`, pcc claim or `VALIDATION.json`. |
| F10 | note | **What does carry over.** "The decomposition changed" and the direction of the explanation (table gained, sweep did not, walk's gain over the table shrank to 1.7x / 6%) are correct and are supported by sections 3 and 5. Nothing was found wrong in the panel itself. The table remains the main structural gain over the sweep (frozen D 3.85x, retuned D 9.4x in solver time). Under the program's success criterion ("verified, measured improvement over the current baseline"), the merged `relation_point` plus a retuned table is a verified improvement over Eh; the walk adds none beyond it on this tree, and the unfixed table-build and walk arithmetic (F2) is the largest remaining lever. |

## Statements that hold up

"All 360 recovered, no timeouts"; the absolute medians; the six ratio medians and their intervals; "trials identical" implicitly (Eh 262, D 377.5, Dw 1113.5); "table alone is now 0.26x of the sweep"; "the walk's extra gain over the table fell from 4x to 1.7x in solver time and to 6% end to end" (1/0.587 = 1.70, 1 - 0.937 = 6.3%); "the frozen settings were not re-tuned"; "post-merge panel not independently replayed" (true when written); "counted-ops conclusions vs rho / BSGS are unaffected" was not examined.

## Reproduction

```
cd /tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/val2
R=/home/user/crypto/research/ic_e222_m221_20261106/results
python3 -I stats.py $R/walk_postmerge/rows.jsonl                      # sections 1, 2 (and $R/walk/rows.jsonl for the pre-merge column)
python3 -I verify_inst.py                                              # 60 instances, own EC arithmetic
python3 -I replay.py $R/walk_postmerge/rows.jsonl passA.json valA '{}'                    # all 360 rows, default
python3 -I replay.py $R/walk_postmerge/rows.jsonl passC_shamir0.json valC '{"IC_SHAMIR":"0"}'
python3 -I cmp_replay.py                                               # sections 3, 5 control
python3 -I replay_v4.py                                                # section 4c (needs final_rows_v4.jsonl from git 1ec774fa45)
python3 -I probe.py probe_tune.json 0 10 1,2 '[["D",16],["D",20],["D",48],["Dw",16],["Eh",256]]'     # F1 tune-set probe
python3 -I probe.py probe_eval_D16.json 10 30 1,2 '[["Eh",256],["D",16],["Dw",16]]'                 # F1 post-hoc
valgrind --tool=callgrind --callgrind-out-file=cg/D.out env IC_SUM_TABLE=1 /home/user/crypto/target/release/ic --json solve-prime-file --input w1517.json --seed 2 --fb-size 48 --extra-relations 2 --rho-steps 0 --max-trials-per-relation 200000 --max-relation-attempts 512   # section 5
python3 -I cgsum.py D D_s0 Eh Eh_s0 Dw Dw_s0 D16; python3 -I cgcalls.py D D_s0 Eh Dw D16
```

Caveats on my own probes (F1, F4): one host, unisolated, serial; the F1 post-hoc comparison reuses the evaluation instances after they were evaluated, so it is a robustness check of the headline, not a new frozen holdout (the fb for D came from the tune instances by the producer's rule); the callgrind counts are for one instance and seed; the A run (F4) is seed 1 only on the v5 evaluation instances, not the v4 panel behind 708.5 ms.
