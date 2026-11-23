# Validator report: merged-baseline evaluation (table and walk vs the hoisted sweep), v6 panel

**Validated commit:** `c60036e1f9319c2e1af837eeb388b7da0a9d2ab6` (branch `claude/prime-field-factor-base-calculus-wy2own`), working tree clean before and after (`git status --short` empty; `target/release/ic` untouched, sha256 `3b3786f80d29f0254020511be3165c13b42bb42ab9e1b05f876fbe1b71ebf4b2`, mtime 05:53:01Z).
**Date:** 2026-11-07, validator session on the same unisolated 4-vCPU host (rustc 1.97.0, Linux 6.18). Artifact hashes (first 16 hex): `rows.jsonl` 430dab211051859f, `frozen.json` e5136b148bb3c6d2, `tune_rows.json` 92179942995a04d8, `analysis.txt` d504d74aeab31d1c, `run_merged.py` e5d22d10ce996236, `analyze_merged.py` 632521d047771dca, `instances_v6/MANIFEST.json` acb14851d51d68cc.
**Builds:** the only builds were made with `CARGO_TARGET_DIR=/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/target_val` (`cargo build --release --bin ic --locked --offline`); nothing was built in `/home/user/crypto/target`.

## Disposition

**`validated`** as a measurement: the numbers in `RESULTS_merged_baseline.md` are reproduced from the raw rows, every run carries an independently re-verified certificate, the evidence binary is bit-identical to a clean rebuild of the committed source, and 960/960 replays reproduce scalar, trials and relations exactly with timings within 2% (median).

**Not validated as worded** for "the table is about 15x and the walk about 38x lower than the *best* hoisted sweep in solver time, and about 1.9x lower end to end". The arithmetic is right (1/0.0658 = 15.2, 1/0.0262 = 38.2, 1/0.534 = 1.87), but the label "best hoisted sweep" is not supported, and the factors need four qualifiers before they can be quoted:

1. The sweep arm (Eh) is the repository's own sweep inner loop on constant-time `BigUint` `FieldElement` arithmetic. This PR moved the table build and the walk onto word/vartime arithmetic and did not touch the sweep. A same-semantics word-arithmetic sweep (validator probe, section 5; identical relations, trials and scalars) is 2.1x faster in solver time at its own tuned fb, which turns 15x into about 7.6x (table) and 38x into about 20x (walk) in solver time, and 1.9x into about 1.46x / 1.54x end to end.
2. 3.9x (table) and 7.5x (walk) of the factors come from this PR's code-path change at identical trials (section 4), i.e. engineering catch-up of the new arms, as the producer partly says.
3. The end-to-end 1.9x is at 95 to 97% of the ceiling set by a fixed 23 ms per-process overhead (ceiling 1.93x at the median) and varies with size (1.4x, 1.8x, 3.1x at p = 16381, 32749, 65521).
4. "Counted group operations are unchanged" is false for the table build (additions halved, section 6); the rho/BSGS conclusions survive.

## Summary of requirements

| # | Requirement | Result |
|---|---|---|
| 1a | Recompute all headline numbers from `rows.jsonl` | PASS (all reproduced) |
| 1b | Order of events (freeze before eval) | PASS, with a 4 s local margin and a 13 s remote-visibility gap that does not matter |
| 1c | Eval instances disjoint from tune set and all earlier panels | PASS |
| 2 | Instance validity by own arithmetic | PASS (90/90, not only 10) |
| 3 | Replay with stored argv/env, sha256 equals stored | PASS (960/960 exact; timing median within 2%) |
| 3b | Binary bound to committed source | PASS (clean rebuild is bit-identical) |
| 4 | Source review of the `ec_index_calculus.rs` diff | PASS (one wording error in the producer text; no unit test on the gated paths) |
| 5 | Fairness of the arms | PARTIAL: per-process overhead and timer scope PASS; sweep arithmetic FAIL (avoidable asymmetry, quantified) |
| 6 | Claim strength | PARTIAL: see the list of required corrections |

## 1. Recomputation and order of events

**1a. Headline numbers** (own code, `scratchpad/recompute.py`; 10,000-resample cluster bootstrap by instance, both seeds of an instance resampled together; `analyze_merged.py` itself was also run and its output equals the committed `analysis.txt` except for set-iteration order on the `IC_ env seen` line).

- 480 rows; 480 recovered; independently `scalar == fixture.known_log` from the instance file in 480/480; no missing timings; 120 (instance, seed) pairs, each with all four arms; `fb` equals `frozen.json` on every row; argv identical across rows except `--seed`, `--fb-size`, `--extra-relations`, `--input`; `rels` = fb + extra on every row (A 72, Eh 258, D 34, Dw 26); row order equals the seeded shuffle `Random("eval-merged-v1")` regenerated from `run_merged.py`, so the file is in execution order and nothing was dropped or reordered.
- Medians (solver ms / process ms / trials): A 304.96 / 327.34 / 17,909; Eh 22.21 / 45.13 / 263; D 1.495 / 26.16 / 543.5; Dw 0.590 / 23.96 / 661. All match the table.
- Paired ratios, median [95% by-instance bootstrap], solver | process:

| ratio | solver | process | wins (solver) |
|---|---|---|---|
| D / Eh | 0.0658 [0.0582, 0.0717] | 0.546 [0.469, 0.596] | 120/120 |
| Dw / Eh | 0.0262 [0.0222, 0.0291] | 0.534 [0.459, 0.577] | 120/120 |
| Dw / D | 0.374 [0.345, 0.429] | 0.939 [0.920, 0.975] | 120/120 (process: 81/120) |
| Eh / A | 0.0769 [0.0739, 0.0832] | 0.147 [0.132, 0.162] | 120/120 |
| D / A | 0.0051 [0.0046, 0.0058] | 0.082 | 120/120 |
| Dw / A | 0.0020 [0.0017, 0.0024] | 0.080 | 120/120 |

  Geometric means agree (D/Eh 0.0644, Dw/Eh 0.0243). Averaging the two seeds per instance and bootstrapping 60 instances gives D/Eh 0.0685 [0.060, 0.075] and Dw/Eh 0.0266 [0.024, 0.030], so pseudo-replication does not change the picture. By `r_bits` 14, 15, 16, 17 (16, 52, 26, 26 pairs) Dw/Eh = 0.0405, 0.0309, 0.0186, 0.0145 and D/Eh = 0.0817, 0.0725, 0.0583, 0.0457, as in the producer's file. The `r_bits` buckets mix fields (r_bits 15 contains both p = 16381 and p = 32749); by field (40 pairs each) Dw/Eh = 0.039, 0.026, 0.0145 and D/Eh = 0.081, 0.066, 0.048 for p = 16381, 32749, 65521.
- The frozen values are the argmin of the committed tune medians (own recomputation of `tune_rows.json`, 600 rows, 600 recovered, 30 instances idx 0 to 9, seed 1, 30 runs per cell): Eh {64: 28.6, 128: 25.6, 192: 23.6, **256: 21.0**, 384: 22.6, 512: 27.4} ms; D {8: 4.25, 12: 2.40, 16: 1.85, 24: 1.41, **32: 1.27**, 48: 1.47, 64: 2.28}; Dw {6: 1.05, 8: 0.76, 12: 0.55, 16: 0.49, **24: 0.469**, 32: 0.62, 48: 1.02}. Optima are interior for Eh, D, Dw; Dw is flat between 16 and 24 (4%). Arm A (fb 8) has no grid; the sentence "all four frozen values are interior to their grids" is inexact for A, which is the shipped default by definition.

**1b. Order of events.** Three independent time sources agree:

- `fdf5fc3ab0` 05:46:18Z (source change, v6 instances, runner). `target/release/ic` mtime 05:53:01Z. Tune rows carry commit `fdf5fc3ab0` and the same binary sha256. Tune wall time sums to 20.5 s and `frozen.json` / `tune_rows.json` mtime is 05:53:33.30Z (tune ran about 05:53:12 to 05:53:33).
- Freeze commit `0ea02a380e` 05:53:44Z (reflog agrees; the committed `frozen.json` blob equals the file; `tune_rows.json` is byte-identical to the committed one).
- Every one of the 480 eval rows records `commit = 0ea02a380e...` (taken by `git rev-parse HEAD` when `run_merged.py` started), so the freeze commit existed locally before the eval process started. Eval start estimated from `rows.jsonl` mtime 05:54:49.87Z minus the sum of row wall times (60.3 s) minus Python overhead (about 1 s) is about 05:53:48Z, i.e. about 4 s after the freeze commit. `rows.jsonl` is written only after the last run (`run_merged.py` lines 43 to 45), results commit `c60036e1f9` 05:55:35Z.
- Judgement: the 13 s between eval start and the remote receiving the commit (05:54:01Z, the producer's statement; I could not verify remote receipt time from the artifacts) does not matter for validity: the frozen values are a mechanical function of the committed tune rows (I recomputed them), the eval cannot influence them, and the rows file did not exist until the end. What it costs is third-party tamper evidence for those 13 s, and the margin between freeze and eval start is only about 4 s. PASS.
- Dirty-tree state is not recorded in the rows; mitigated by 3b (the binary equals a clean rebuild from HEAD, and `git diff fdf5fc3ab0 HEAD -- src Cargo.toml Cargo.lock` is empty).

**1c. Disjointness** (own scan of every JSON with the instance schema under `results/`): eval = 60 instances idx 10 to 29 of W14/W15/W16; tune = 30 instances idx 0 to 9. Zero collisions of the curve key (p, a, b) or of the full tuple (p, a, b, G, Q) between eval and tune, within eval, or between eval and the 1,060 instances in the 12 earlier directories (`instances`, `_confirm`, `_holdout`, `_holdout_W`, `_holdout_v3`, `_scale`, `_table`, `_timed`, `_v3_sub`, `_v4`, `_v5`, `_vsrho`). The seed string `e222m221-holdout-v6` is unique to `instances_v6`, and no script other than the results note refers to it. Caveat: all 90 instances use only three primes (16381, 32749, 65521); "60 instances" are 60 curves over 3 fields (20 per field).

## 2. Instance validity

Own affine arithmetic in Python (`scratchpad/val_instances.py`, `count_pts.py`), all 90 v6 instances (60 eval and 30 tune, not just 10): p prime, discriminant nonzero, G and Q on the curve, `n*G = O` with `G != O`, `n` prime, `n` equals `group_order` and `subgroup_order`, `cofactor = 1`, Hasse bound holds, `n > 4 sqrt(p)` (so the order of G is exactly n and it is the group order), `known_log * G = target` and `0 < known_log < n`. An independent Legendre-sum point count (sum over x of 1 + chi(x^3 + ax + b)) equals `group_order` on all 90. SHA-256 of all 90 files equals `MANIFEST.json`. 90/90 valid, 0 hash mismatches.

Note (not a validity failure): `n == p` (anomalous curves, Frobenius trace 1) for W14-0 (tune), **W14-11 and W15-26 (eval)**. The generator excludes only trace 0. The index-calculus solver treats them like any other (their rows are unremarkable), but they are special curves and should be excluded or flagged in any statement about generic curves.

## 3. Replay

Binary: `target/release/ic` sha256 equals the `binary_sha256` stored in all 480 rows (`3b3786f8...bf4b2`).

All 480 rows replayed twice (960 replays; `scratchpad/replay.py`) in seeded random order with the stored argv (input path replaced by a freshly written copy of the same eight integers) and the stored `IC_*` environment exactly, all other `IC_*` variables scrubbed:

- scalar, `trials_total`, `relations_collected` identical to the stored values in **960/960**; all 960 scalars equal the planted log.
- Timing, replay / stored median: solver A 0.999, Eh 1.021, D 1.009, Dw 1.014; process 1.002, 1.017, 1.006, 1.019. Within the 30% band at the median for every arm. Per row (not required): within 30% in 92% (A), 73% (Eh), 62% (D), 66% (Dw) of replays; the same row run twice by me differs by an IQR of about 0.80 to 1.21 in the sub-millisecond arms, so single-row timings of D and Dw are noisy at the 20 to 25% level, while medians over 120 pairs are stable.
- Headline ratios in the replays (pass 1 / pass 2; stored): D/Eh 0.0625 / 0.0611 (0.0658); Dw/Eh 0.0229 / 0.0247 (0.0262); Dw/D 0.393 / 0.414 (0.374); Eh/A 0.0809 / 0.0808 (0.0769); process D/Eh 0.518 / 0.533 (0.546); Dw/Eh 0.492 / 0.516 (0.534). D and Dw beat Eh in 120/120 pairs in both passes.

**3b. Binary bound to source.** A clean build of the committed tree (`cargo build --release --bin ic --locked --offline` into a separate target dir) produced a binary with the same sha256 (`cmp` identical). So the evidence binary was built from the committed source at HEAD (whose `src/` equals `fdf5fc3ab0`), with no uncommitted changes, with this toolchain.

## 4. Source review (`src/cryptanalysis/ec_index_calculus.rs`, merge-base `76134392e5` to HEAD; the only `src/` change)

The diff touches `build_pair_sum_table` and `SampleWalk::{new, restart, step}` only. `find_one_relation_table_with_rng` already called `relation_point` at the merge base (the producer's wording "the table's relation point ... now use" is inexact; the table build and the walk changed).

- **Table entries.** Old: for each pair i <= j, all four sign combinations, each by constant-time `add`, `entry().or_insert((i, s_i, j, s_j))` in the order (+,+), (+,-), (-,+), (-,-). New: `sum = F_i + F_j` stored as (+,+) and its negation as (-,-); for i != j, `dif = F_i + (-F_j)` stored as (+,-) and its negation as (-,+). Algebra: -(F_i + F_j) = (-F_i) + (-F_j), -(F_i - F_j) = (-F_i) + F_j. For i == j: `sum = 2 F_i` (add_vartime doubles), -2 F_i, and the two infinity combinations F_i - F_i are skipped in both versions (`put` ignores `Point::Infinity`; `neg_vartime(Infinity)` is Infinity). For F_j = -F_i (i != j) the sum is infinity and skipped, and the difference is 2 F_i, correct. Every stored entry is a true decomposition.
- **Own check** (`scratchpad/table_equiv.py`, independent re-implementation of both builds): on all 90 v6 curves at fb 24, 32, 48 (270 cases) the old and new tables are identical as maps point -> (i, s_i, j, s_j) (0 mismatches), and every entry satisfies s_i F_i + s_j F_j = point. On 3,240 adversarial cases (tiny primes, all points including 2-torsion, factor bases with duplicates and negated copies) every entry is a true decomposition and the key sets are equal; in 385 of them the stored decomposition differs between old and new when two different decompositions of one point exist within a degenerate base (duplicates or negated copies), which `build_factor_base` (distinct x, canonical y) cannot produce and in any case yields a valid relation.
- **Same relations as before, on real runs.** Replaying the 360 rows of the pre-PR post-merge panel (`results/walk_postmerge/rows.jsonl`, instances v5, arms Eh fb 256, D fb 48, Dw fb 16, seeds 1 and 2, produced by the merge-base code) with the HEAD binary: `trials_total` and `relations_collected` identical in **360/360** (`scratchpad/cross_old.py`), 360/360 recovered. So the table build and the walk setup/step change neither the sampled points, the RNG consumption, nor the relations found.
- **`add_vartime` vs `add`.** `Point::add_vartime` (`src/ecc/point.rs` lines 107 to 132) has the same three branches (identity, `x1 == x2` with `y1 == y2` double, `y1 != y2` infinity); the word path (`Word::affine_add`/`affine_double`, `y == 0` doubles to infinity) is branch-for-branch the same; `neg_vartime` equals `neg` including 0. The repository test `vartime_add_double_match_reference` compares `add_vartime`/`double_vartime`/`neg_vartime` with the constant-time versions on every pair of points of F_97 (identity, doubling, P + (-P)), near 2^64, and on P-256.
- **Walk invariant.** `new`/`restart` set `r = relation_point(g, a, q, b) = aG + bQ` (`Point::double_scalar_mul_vartime`, tested equal to the two-ladder form at `ec_index_calculus.rs` lines 2420 to 2447); `step` does `r += (a_k G + b_k Q)` with `add_vartime` and `a += a_k`, `b += b_k` mod n, so `r = aG + bQ` is preserved (n is prime, Q in <G>). Infinity after a step is skipped and triggers a restart on the next trial. The solver verifies `g*x == q` before returning, so an invalid relation could not yield a wrong verified log (and all 480 + 960 outputs equal the planted log).
- **Default path.** `A` and every non-gated path call none of the changed functions (`build_pair_sum_table` runs only if `IC_SUM_TABLE=1`, `SampleWalk` only with `IC_WALK=1`); `git diff` shows no other change in `src/`. Bit-for-bit unchanged at source level; runtime evidence: A rows replay exactly, and the shipped default's relation point is the same `relation_point`.
- **Gap.** No unit test exercises `build_pair_sum_table` or `SampleWalk` (the env-gated paths are covered only by the experiments). A test pinning the table map to the four-combination definition would have caught nothing here but would protect later edits.

Instrumented counts (callgrind on the evidence binary, `scratchpad/cg/`), inclusive instructions (Ir) of `ec_index_calculus_dlp_staged_with_rng`, frozen settings:

| instance, seed | A | Eh | D | Dw | D/Eh | Dw/Eh |
|---|---|---|---|---|---|---|
| W15-17, 2 | 2,278.1 M | 177.2 M | 7.97 M | 3.03 M | 0.045 | 0.017 |
| W14-22, 1 | - | 100.4 M | 5.69 M | 2.62 M | 0.057 | 0.026 |
| W16-17, 1 | - | 494.7 M | 17.24 M | 4.46 M | 0.035 | 0.009 |

Eh/A = 0.078 and Dw/D = 0.38 on W15-17, matching the time ratios (0.077, 0.374). The time ratios of D and Dw to Eh are, if anything, larger than the instruction ratios, so the timings do not overstate the effect. Call counts at W15-17 (fb 32, D): `build_pair_sum_table` calls `Point::add_vartime` exactly 1,024 times (= 528 sums + 496 differences); in Dw (fb 24): `SampleWalk::step` 669 calls (= trials), `relation_point` 53 calls (33 set-up + 20 restarts), `build_pair_sum_table` once.

## 5. Fairness of the arms

- **Which arm uses what.** All four arms obtain the per-trial point from `relation_point` (word-Jacobian `double_scalar_mul_vartime`, default mode): A and Eh in `find_one_relation_sweep_with_index`, D in `find_one_relation_table_with_rng`, Dw in `SampleWalk::new/restart` (once per 32 trials). The vartime point additions (`add_vartime`) are used by D and Dw (table build; Dw also per step) and not by A or Eh. A and Eh then run, per factor-base element per trial, `semaev_s3_in_x3`, a discriminant, `sqrt_mod_p` (already a u64 path) and a **constant-time `FieldElement::inv`** (a Fermat exponentiation on `BigUint`) on public data.
- **Where Eh's time goes** (callgrind, Eh, W15-17, 177.2 M Ir): the sweep is 159.9 M (90%); `FieldElement::inv` 70.6 M (40%), `semaev_s3_in_x3` 30.0 M (17%), `sqrt_mod_p` 25.1 M (14%), `try_finalise` 11.9 M (7%), elimination 14.3 M (8%), `relation_point` 2.6 M (1.5%). For A (2,278 M Ir in the solver) the same functions are 40%, 18% and 15% of the solver.
- **Validator probe (exploratory, not the producer's protocol, not a frozen holdout):** a copy of the source tree in scratch with only `find_one_relation_sweep_with_index` changed to compute the same candidates in the same order on `u64` words when p < 2^40 (same RNG draws, same `try_finalise`), built into `target_val` (`probe_tree`, `scratchpad/make_probe.py`). Replaying the 120 stored Eh rows with the probe gives identical scalar, trials and relations in 120/120. fb retuned on the tune split exactly like the producer's grid logic (idx 0 to 9, seed 1, grid 32 to 384): best **192**, median 9.59 ms (vs 21.0 ms at fb 256 for the shipped sweep). Four-arm paired run on the 120 eval pairs, interleaved in seeded order, same host session, 480/480 recovered:

| arm | solver median (ms) | process median (ms) |
|---|---|---|
| Eh (shipped sweep, fb 256) | 22.05 | 44.38 |
| Ep (word-arithmetic sweep, fb 192) | 10.25 | 33.01 |
| D (fb 32) | 1.318 | 22.78 |
| Dw (fb 24) | 0.562 | 21.98 |

  Paired medians, solver | process: Ep/Eh 0.477 | 0.728; **D/Ep 0.131 | 0.687; Dw/Ep 0.049 | 0.651**; D/Eh 0.061 | 0.506; Dw/Eh 0.024 | 0.509. So against a sweep that is not handicapped by constant-time inversion, the table is about 7.6x and the walk about 20x lower in solver time, and about 1.46x and 1.54x lower end to end. The probe is a quick minimal word port, so the true best sweep is at least this fast. The table and walk arms are not fully optimised either (BigUint hash keys, BigUint coefficient updates), so the right comparison is not "probe vs shipped", but the asymmetry in effort is avoidable and large.
- **Per-process overhead** is the same for all arms: median (process minus solver) A 23.2, Eh 23.1, D 24.1, Dw 23.4 ms; by field 23.1, 23.3, 23.6 ms. It is the CLI's own work (order counting, `n*G` and `n*Q` checks, final verification on constant-time scalar multiplication), identical across arms, and it caps the end-to-end gain: the ratio with solver time taken to zero is 0.519 (1.93x) at the median; observed Dw/Eh process 0.534 (1.87x) is 97% of that ceiling (D 1.83x, 95%); by field the ceiling and observed values are 1.50x / 1.43x (p = 16381), 1.93x / 1.81x (32749), 3.15x / 3.10x (65521).
- **What the solver timer includes:** `total_ms` runs from the start of `ec_index_calculus_dlp_staged_with_rng` to its end: factor-base build, the pair-sum table build (inside `relations_started`, so charged to the relations stage), the hoisted x-index, `SampleWalk::new` (33 `relation_point` calls), relation collection, matrix build, elimination, and the verification scalar multiplication. It excludes argument parsing, point counting, the CLI's input checks, hashing and JSON output. Replay stage medians: A relations 301.5 / total 301.7 ms; Eh relations 21.2, elimination 1.7, factor base 0.27, total 24.1; D relations 1.33, total 1.47; Dw relations 0.50, total 0.62.
- The tune used the same instances, seed and statistic (solver median) for the three tuned arms, with wide grids on both sides, so tuning effort is symmetric between Eh, D and Dw. A is untuned by design; `Eh/A` mixes three changes (fb 8 to 256, extra relations 64 to 2, hoisted index).
- Budgets: all arms use `--max-trials-per-relation 200000 --max-relation-attempts 512`, not the CLI defaults (5000, 64); this affects only failure cases (none occurred), but "shipped default" for A means fb 8 / extra 64 only.

## 6. Claim strength

| Claim | Verdict |
|---|---|
| 480/480 recovered; paired ratios and CIs; "analyze_merged.py reproduces them" | PASS (section 1) |
| "Table about 15x, walk about 38x lower solver time than the best retuned and hoisted sweep" | arithmetic PASS; **"best" not supported** (section 5: a same-semantics word-arithmetic sweep makes it about 7.6x / 20x) |
| "about 1.9x lower end to end" | PASS as arithmetic (1.83x table, 1.87x walk; "about 1.9" slightly rounds the table); it is within 4% of the overhead ceiling 1.93x and size dependent 1.4x, 1.8x, 3.1x; D and Dw cannot be ranked end to end beyond 0.94 [0.92, 0.975] (walk wins 81/120) |
| "about 500 times lower than the shipped default" | Dw/A = 0.0020 [0.0017, 0.0024], i.e. 420x to 590x for the walk; the table is about 200x; the sentence reads as if it covers both |
| "A large part of the change from the previous panel is my own code catching up with the fast point path" | **understated**: at identical frozen fb, instances, seeds and trials the PR's code change alone takes D from 6.23 to 1.58 ms (3.9x; paired median 0.238) and Dw from 3.71 to 0.495 ms (7.5x; paired 0.136) (`cross_old.py`, idle host; the stored pre-PR timings come from an earlier session: Eh, which the PR did not touch, reads 24.8 ms stored vs 21.5 ms now, paired median 0.975, so host drift is between 3% and 14% depending on the statistic and the gains are 3.4x to 4.2x for the table and 6.5x to 7.5x for the walk). Dw/Eh at those settings goes from 0.149 to 0.023 on this change alone, i.e. essentially all of the movement from the previous panel's 0.152 to this panel's 0.026 |
| "the walk is 0.374x of the table in solver time and 0.94x end to end" | PASS, with the caveat that D and Dw have different fb (32 vs 24) |
| "Counted group operations are unchanged by this work (the new code paths perform the same number of additions)" | **FAIL as worded.** Trials and relations are unchanged (360/360 pre-PR rows, 480/480 here), walk additions unchanged, but the table build now does 2 additions per pair (1,024 at fb 32, callgrind) instead of 4 (2,112), i.e. 0.485x. This is an engineering change, and it is the reason the single-target ledger in `RESULTS_vs_rho.md` (which charges `2 fb^2` for the table) is now an overcount by about fb^2 additions: from the producer's own table 36,617 / 108,706 / 240,140 less 4,224 / 4,224 / 16,640 gives about 22x / 40x / 49x instead of 25x / 41x / 53x (my arithmetic on the producer's ledger inputs, not a re-run of those panels). The multi-target 1.40 to 1.53x vs BSGS is unaffected because `examples/ic_multi_target.rs` imports only `build_factor_base` and the eliminator and has its own table build (already 2 additions per pair). Qualitative conclusions against rho and BSGS (IC more expensive) stand; the exact sentence does not |
| "counted-operation conclusions against rho/BSGS ... are unchanged" | NOT RUN: I did not recompute the 25 to 53x or 1.40 to 1.53x ledgers; the statement above is a consistency check only |
| "solver-timer ratios of this size are close to what that timer can resolve" | Imprecise: the timer is `Instant` (ns); the limit is run-to-run noise (IQR about +-20 to 25% per row at 0.4 to 1.5 ms). The result rests on medians of 120 pairs, 120/120 wins, replay agreement within 2%, and instruction counts that show the same ordering and larger gaps |
| Size trend (ratio falls from 0.041 to 0.014 for r bits 14 to 17) | Numbers PASS; the r_bits buckets are confounded with field (bucket sizes 16, 52, 26, 26 pairs); by field 0.039, 0.026, 0.0145. Three fields, group order up to 2^17: no basis to extrapolate in either direction; the table build grows as fb^2 |
| "pairs share an instance" / pseudo-replication | Handled correctly (cluster bootstrap by instance; I get the same intervals); additionally only 3 fields and 2 of 60 eval curves are anomalous (n = p) |
| "all four frozen values are interior to their grids" | Inexact for A (no grid, fixed at the shipped default) |
| Status "unvalidated (pre-gate) until the validator replay" | Correct label; this report is that replay, it is not the repository gate (no `VALIDATION.json`, matched rho band, pcc claim, bench record or isolab-eks run exist for this panel, as the producer states) |

## Required corrections before the factors are quoted

1. Do not call Eh the "best" sweep. Say "tuned, hoisted shipped sweep (constant-time `BigUint` inner loop)", and either add a word-arithmetic sweep arm to the protocol (the probe shows 2.1x) or quote the corrected factors (about 7.6x table, about 20x walk, 1.46x / 1.54x end to end) next to the 15x / 38x / 1.9x.
2. Replace "Counted group operations are unchanged ... same number of additions" by: trials, relations and walk additions unchanged, table build halved (4 to 2 additions per pair), single-target ledger formula `2 fb^2` now overcounts by about fb^2; conclusions against rho and BSGS unchanged.
3. State the attribution: 3.9x (table) and 7.5x (walk) of the gain over the previous merged-baseline panel is this PR's code-path change at identical trials.
4. Quote the end-to-end figure with its ceiling (1.93x at the median, from a fixed 23 ms) and its size dependence.
5. `CAMPAIGN_SUMMARY.md` is now internally inconsistent: the section A conclusion ("the walk is not a supported improvement beyond a retuned table", "Remaining lever: `build_pair_sum_table` and `SampleWalk` still use the slow path") and the walk row ("pre-merge binary only") predate this panel's code change and are contradicted by the new paragraph (walk 0.374x of the table). Mark them superseded.
6. Process: `results/merged/rows.jsonl` (273 KiB) is a `.jsonl` above 64 KiB committed to Git; AGENTS.md section 13 puts it in the artifact store with a `.artifacts.json` manifest (earlier panels in this directory have one). `tune_rows.json` is 345 KiB. Record the dirty-tree state or the build commit in the rows, not only the binary hash.
7. Flag or exclude the two anomalous eval curves (W14-11, W15-26) in statements about generic curves; add a unit test for the gated table and walk paths.

## What was not done

No isolab-eks run, no S3/ecdb/pcc checks, no gate items (those are declared missing by the producer). The remote push time of the freeze commit was not verified (no remote query). The 25 to 53x and 1.40 to 1.53x ledgers were not recomputed. No build of the pre-PR source was made; old-versus-new equivalence rests on replaying the 360 pre-PR panel rows with the new binary, the independent table re-implementation, and the source review. No experiment was re-run to replace evidence; the probe, replays, callgrind runs and builds are validator-side checks in `/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/` (`recompute.py`, `val_instances.py`, `disjoint.py`, `count_pts.py`, `replay.py`, `replay_an.py`, `table_equiv.py`, `cross_old.py`, `make_probe.py`, `probe_eval.py`, `cg/`, outputs `replay_full.json`, `cross_old_2.json`, `probe_eval.json`).
