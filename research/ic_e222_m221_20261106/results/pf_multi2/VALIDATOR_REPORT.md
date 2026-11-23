# Validator report: table-mode multi-target IC evaluation (`pf_multi2`)

Validator: independent session, role `validator` (Sonnet 5.5). Date of the final checks: 2026-11-11 (first part of the replay was run on 2026-11-07, see 0).
Dispatch: validate `research/ic_e222_m221_20261106/RESULTS_prime_fast_multi_table.md` and `results/pf_multi2/` on the PR #3637 branch.

## 0. What was validated, and from where

| item | value |
|---|---|
| validated commit (branch head) | `58fbe717d71dc16b4cde7072deda4439f66602d3` (merge of `origin/main`, 2026-11-11T01:33:55Z), tree `ee110ede0be39ea4985e36b66ac086abfb4fbaea`. `git status --porcelain` was empty at the start and at the end; HEAD did not move. |
| evaluated commits | freeze `671cec184f` (2026-11-07T10:08:37Z), partial rows `5089230ecf` (10:08:58Z), final `2092befb7d` (10:10:26Z, tree `b6f01716ee8e…`) |
| evidence unchanged by the merge | `git diff 2092befb7d HEAD -- research/ic_e222_m221_20261106/results/pf_multi2 research/ic_e222_m221_20261106/RESULTS_prime_fast_multi_table.md` is empty |
| solver sources unchanged by the merge | blobs identical at `671cec184f`, `2092befb7d` and HEAD: `src/cryptanalysis/prime_fast.rs` `52d407a2a8`, `examples/prime_fast_multi.rs` `3146edf5b8`, `examples/prime_fast_bsgs_multi.rs` `bd09efd894`, `Cargo.lock` `426a5848aa`, and the dependency closure `residual_walk.rs`, `ecc/{curve,field,point}.rs`. The merge changed 94 other `src/` files and added only `[[bin]]` entries to `Cargo.toml` (no `[profile]` at either commit). The coordinator's statement "no source changes to prime_fast.rs or the examples" is confirmed, not assumed. |
| evidence hashes (sha256, first 16 hex) | eval_table2_{25,27,29,32} `3c693fed…`, `c7648f4c…`, `76b72ebe…`, `b279c69b…`; eval_sweep3_{25,27,29,32} `cae7e6ac…`, `ba9cb8c6…`, `741b01f0…`, `4fbd7f3b…`; eval_bsgs_{25,27,29,32} `45c24662…`, `d3414c32…`, `bcfa6fc4…`, `f156127f…`; explore_table_26 `568fce24…`, explore_table_30 `b7dde1d3…`; frozen.json `ced5f0ff…` (identical to the blob in the freeze commit); provenance.txt `d7df3acb…`; RESULTS_prime_fast_multi_table.md `8c0b4f77…` |
| hosts | (a) 2026-11-07 replays: the producer's sandbox (4 vCPU Intel Xeon @ 2.10 GHz, KVM, 16 GiB), minutes after the producer's run. (b) 2026-11-11 checks: the sandbox had been restarted (uptime 9 min at 01:40Z); same CPU model, kernel 6.18.44, rustc/cargo 1.97.0 (2d8144b78, 2026-07-07), Python 3.13.18. Absolute times differ between (a) and (b); only same-session ratios are compared across them. Neither is an independent host class. |
| writes | The only tracked file written is this report. Side effect to disclose: on 2026-11-07 10:15:22 my `cargo build --release --offline --locked --example prime_fast_multi --example prime_fast_bsgs_multi` in `/home/user/crypto` relinked the git-ignored `target/release/examples/prime_fast_bsgs_multi`. I cannot establish whether the producer's BSGS binary was on disk before that, so the BSGS hash in `provenance.txt` could not be matched (see 2.2). The HEAD rebuild on 2026-11-11 used `CARGO_TARGET_DIR` in the scratchpad and touched nothing under the repository. |

## 1. Disposition

Overall: **`validated` for the measurement** (every returned log of every arm re-verified by independent code; the deterministic fields reproduce on three builds), with the headline **magnitudes `qualified`**, **one stated number not validated** and one displayed digit wrong. The result is **not eligible** for the IC leaderboard or `benchmarks/README.md` (no `VALIDATION.json`, no matched rho arm, no §5a band, wall-clock unit, no certificate file, no bench record).

| # | claim in `RESULTS_prime_fast_multi_table.md` | verdict | one-line reason |
|---|---|---|---|
| T1 | protocol frozen in `frozen.json`, commit `671cec184f`, abandon condition stated before the run | **validated** (qualified) | commit order and mtimes agree; "pushed and checked on the remote before the run" cannot be shown to the second (2.1) |
| T2 | c = 2.0 chosen on the 26- and 30-bit curves, best of {1, 2, 3} in 4 of 4 groups | **validated** | recomputed from the committed explore rows: 4 of 4 group means, 5 of 6 single rows |
| T3 | evaluation curves 25/27/29/32 are disjoint from the exploratory curves | **validated** (qualified) | disjoint from 26/30 (and from 24/28); the two 32-bit curves are the same curves the earlier campaign used for its 32-bit exploratory runs (2.4) |
| T4 | T = 1000 targets, identical across the three arms | **validated** | identical (Q_t, planted k) on 14 of 14 rows in all three arms |
| T5 | 14 rows per arm, all 14 000 targets of every arm verified, no failed run | **validated** | 42 000 of 42 000 re-verified `kG = Q` in independent Python; 0 wrong, 0 unsolved |
| T6 | every returned log verified `kG = Q_t` (fail-closed) | **validated** | code read plus mutation tests on the table and sweep paths (2.3) |
| T7 | timed region contents; planted targets outside | **validated** | code read, all three arms (2.5) |
| T8 | result table (rows, medians, ratios) | **validated** except one digit | all entries recompute; sweep3/BSGS at 27 bits is 8.548, which rounds to 8.5, not 8.6 |
| T9 | pooled table2/sweep3 median 0.58, range 0.36 to 0.67, every row below 1.0 | **validated** as arithmetic of the committed single run; **qualified** as a property | replicates: pooled medians 0.50 to 0.65 in seven sets, but 1 of 92 row-level measurements was 1.035 (2.2, 2.5) |
| T10 | table collector "1.5x to 2.8x faster ... a reproducible improvement" | **qualified** | direction reproduces everywhere (every size, every set); the 1.5x to 2.8x range is the single-run row range; typical pooled value is 1.5x to 2.0x (0.50 to 0.65), single rows span 0.35 to 1.04 |
| T11 | "over repeated single-target `ic_solve` (134x to 570x)" | **qualified** | cross-reference to the earlier, separately validated evaluation of the sweep variant; this evaluation's own rows give 211x to 838x for table2 against the extrapolated reference T x median-of-5 `ic_solve` |
| T12 | "not an improvement over BSGS: measured BSGS is 2.9x to 5.0x faster" | direction **validated**; magnitude **qualified** | single-run per-size medians reproduce (pooled over sets 2.4, 3.5, 3.9, 5.4); but the BSGS arm is untuned and a lightly tuned BSGS is 2.6x to 3.9x faster still, so table2 is 7.6x to 16.8x behind it in one session (2.5) |
| T13 | gap to BSGS is smaller than the sweep variant's 4.8x to 8.6x | **validated** (8.5, not 8.6) | table2/sweep3 < 1 in every set |
| T14 | "the BSGS arm is simple (no tuning) so its measured speed is a floor" | **validated**, and quantified | a batched-baby-step, cheap-hash BSGS with the same B and targets is 2.6x to 3.9x faster in the same session |
| T15 | "the table is fb^2 entries (about 2 million at 32 bits)" | **not validated** | the formula is right; the number is the sweep arm's. Table arm at 32 bits: fb = 3251, 10 569 001 entries, 169.1 MB (2.5) |
| T16 | limits: no §5a band, one thread, T = 1000 only, 2 rows at 32 bits, no `VALIDATION.json`, bench record, S3 checkpoint, null control or certificate file | **validated** | every item confirmed absent or true (1 thread: grep of the IC and BSGS paths finds no threading) |
| T17 | status line "not yet independently replayed" | **superseded** | this report is the replay |

## 2. Requirement checks

### 2.1 Freeze order and constants: PASS (qualified)

- `git show 671cec184f --stat`: the freeze commit holds exactly `frozen.json`, `explore_table_26.jsonl`, `explore_table_30.jsonl` (committer time 10:08:37Z). `frozen.json` names the three arms, `c = 1.0` (sweep3), `c = 2.0` + table (table2), `B = ceil(sqrt(T n / 4))` (BSGS), `T = 1000`, `extra = 6`, sizes 25/27/29/32, instances 2/2/2/1, the primary metric and the abandon rule (`table2/sweep3 median >= 1.0`). This answers the earlier validator's complaint that `frozen.json` stated no abandon condition.
- File mtimes on the producer machine: `frozen.json` 10:08:36.086, explore files 10:08:36.079, `provenance.txt` 10:08:40.836, rows from `eval_sweep3_25` 10:08:46.083 to `eval_bsgs_32` 10:09:55.951; `RESULTS` 10:10:25.2. Commits: partial rows `5089230ecf` 10:08:58Z, final `2092befb7d` 10:10:26Z. The solver and example blobs are identical at the freeze commit and at HEAD.
- Rows exist for exactly the declared sizes and instance counts (14 per arm; no `failed` row in any of the 42 rows); `fb = round(c * p^(1/3))` in every row (645, 1024, 1625, 3251 for table2; 323, 512, 813, 1625 for sweep3).
- Qualifiers. (a) The CI check run for `671cec184f` started 10:08:46Z (GitHub API), which bounds the push time from above; the first row file was last written at 10:08:46.08. Push before first run cannot be shown at one-second resolution, and the remote push time is not recorded, so "pushed and checked on the remote before the run" is not independently verifiable. The local freeze commit (10:08:37Z) does precede `provenance.txt` and every row. (b) The sweep arm's `c = 1.0` rests on "earlier 24/28-bit probes" that are not committed. Replaced by evidence in 2.5 (the c-grid).
- Explore rows (`explore_table_*.jsonl`, 18 rows, all `ok = 1000`, all `table = true`): group means in ms, (26,res1) c1 150.3, c2 108.1, c3 138.1; (26,res3) 218.4, 139.0, 165.4; (30,res1) 1471.3, 963.7, 1172.0; (30,res3) 1652.4, 959.2, 1373.7: c = 2.0 best in 4 of 4 groups; by single row 5 of 6 (the (26,res3,inst0) row has c = 3 best, 145.1 against 157.6).

### 2.2 Rebuild and re-run: PASS

- Provenance binding. `provenance.txt` is `commit 671cec184f9a… multi 25e0e9351b633fc6 bsgs 4cf79fb2bb6cc4b6`. The on-disk `target/release/examples/prime_fast_multi` (mtime 2026-11-07 10:06:47) hashes to `25e0e9351b633fc6`: it is the producer's binary and I replayed from it, unmodified. For BSGS the on-disk binary hashes to `a2fb6933d47ec837` (linked by my build at 10:15:22), not `4cf79fb2…`; the producer's BSGS binary cannot be re-hashed. The BSGS code path is unchanged by the lib edits between the BSGS commit `43e497d804` and the freeze: `git diff 43e497d804 671cec184f -- src/cryptanalysis/prime_fast.rs` touches only `ic_multi_solve` (the `table` parameter and the collector closure); nothing in `add`, `add_many_batched`, `double`, `neg`, `mul`, `batch_inv` or the field code.
- Replay A (evidence binary, 7 Nov, 12 rows x 3 arms = 36 runs): `prime_fast_multi <out> <bits> 1000 2.0 <inst> table`, `... 1.0 <inst>`, `prime_fast_bsgs_multi <out> <bits> 1000 <inst>` for 25 (2 inst), 27 (2), 29 (1), 32 (1). Deterministic fields identical to the committed rows on all 36: `n`, `ok = 1000`, `determined`, `fb`, `B`, `rounds`. Timing rerun/original: median 0.89, range 0.43 to 1.30 (table2 0.88, sweep3 0.85, BSGS 0.92).
- Replay B (clean rebuild of HEAD, 11 Nov). `CARGO_TARGET_DIR=<scratchpad> cargo build --release --offline --locked --example prime_fast_multi --example prime_fast_bsgs_multi`: exit 0, 10 min 22 s. Rebuilt binaries: multi `2f7900eaa25da7c0`, bsgs `e94e878391a0d8f0`; **not bit-identical** to the producer's (multi: 4 438 067 differing bytes; `.text` 470 647 against 470 231 bytes). Cause: the same sources and profile compile in a crate that now holds many more modules (codegen unit partitioning), not flags; `Cargo.toml` and `Cargo.lock` carry no profile or dependency change. So the provenance hash is not reproducible from source; reproducibility rests on the deterministic fields. Replay of the same 36 runs with the rebuilt binaries: deterministic fields identical on 36 of 36; timing rerun/original median 0.89 (table2 0.86, sweep3 0.66, BSGS 1.11).
- Timing replicates of the 12-row sample (evidence binary, arm order alternated between repeats, all `ok = 1000`), pooled median of per-row ratios per set:

| set | table2/sweep3 | range | table2/BSGS | range |
|---|---|---|---|---|
| committed (12 common rows) | 0.578 | 0.360 to 0.672 | 3.69 | 2.27 to 5.72 |
| replay A | 0.550 | 0.368 to 0.835 | 3.65 | 1.67 to 5.32 |
| repeat 1 | 0.629 | 0.388 to 0.764 | 3.31 | 2.20 to 5.51 |
| repeat 2 | 0.535 | 0.352 to 0.773 | 2.96 | 1.82 to 7.25 |
| repeat 3 | 0.501 | 0.396 to 0.650 | 3.78 | 1.75 to 5.33 |
| repeat 4 | 0.632 | 0.422 to **1.035** | 3.36 | 2.71 to 5.89 |
| replay B (HEAD rebuild, new session) | 0.649 | 0.448 to 0.860 | 2.88 | 1.58 to 5.08 |

  Per-arm run-to-run variation of `total_ms` across the six sets on the original host: median CV 0.12 (table2), 0.15 (sweep3), 0.10 (BSGS), maximum 0.30, 0.37, 0.21. The frozen evaluation is a single pass per row with fixed arm order (sweep3, table2, BSGS per size); no A/A interval is recorded.
  Pooled over committed + replay A + repeats 1 to 4 (per size, all rows and sets): table2/sweep3 25 bits 0.64 (n = 24, range 0.35 to 1.04), 27 bits 0.46 (24, 0.36 to 0.65), 29 bits 0.62 (12, 0.38 to 0.75), 32 bits 0.60 (12, 0.46 to 0.77); table2/BSGS 2.37, 3.49, 3.87, 5.36.

### 2.3 Independent verification of returned logs: PASS

- Harness: `harness/dump_multi.rs` calls the committed `find_a3_curve` and `ic_multi_solve` (evidence rlib, built 2026-11-07 10:06:46 from the committed blob) with the producer's seeds (`0xC0DE ^ ins ^ n ^ T`, `fb = round(c * p^(1/3))`, `extra 6`, `max_walk 5e6`, `seed ins + 1`) and dumps `p, a, b, n, G`, every `Q_t`, the planted `k` and the returned log. `harness/dump_bsgs.rs` is a verbatim copy of the committed BSGS algorithm plus a dump (so the BSGS logs are verified from a copy of the algorithm, not from the shipped binary).
- Checker: `verify_logs.py`, Python big integers only, imports nothing from the solver, own affine arithmetic. Per instance: `p` and `n` prime (Miller-Rabin, 12 bases), `a = p - 3`, non-singular, `G` on the curve, `n G = O`, Hasse bound, `n > 4 sqrt(p)` (so `n` is the only multiple of the order in the Hasse interval). Per target: `Q_t` on the curve, `planted * G = Q_t`, `0 < k < n`, `k G == Q_t`.
- Result, 14 instance-rows per arm: **table2 14 000 / 14 000 verified, sweep3 14 000 / 14 000, BSGS 14 000 / 14 000; 0 unsolved, 0 wrong, 0 verified-but-different-from-planted.** The solver-side `determined`, `n`, `fb`, `B`, `rounds` of the dumps equal the committed rows on 14 of 14 rows per arm. Dump hashes (first 12 hex): table2_{25,27,29,32} `3a2af3bddb64`, `1deb02651813`, `44545f3623ab`, `04198c245428`; sweep3 `f8a2569db400`, `c26ac5f29dc2`, `e0b25937b1cb`, `da6805e9c254`; bsgs `f3b1161f1b93`, `a3c6c7b29b8b`, `2bc9b96164f7`, `6bce889fc0b5`.
- Null controls for the verifier (corrupted copies of `table2_25`): one log +1, flagged WRONG 1 (exit 1); one `Q` negated, assertion `planted*G != Q`; one log set to -1, counted unsolved 1; `n + 1`, assertion `n not prime`; all logs shifted by `n`, flagged WRONG 1000. The checker is not vacuous.
- Fail-closed. `ic_multi_solve` assigns `res = Some(k)` only inside `if curve.mul(Some(g), k) == Some(q)` (prime_fast.rs lines 1783 to 1786); the relation collector also rejects unverified relations (`relation_holds`, line 1429). To test the guard, stand-alone copies of the committed `prime_fast.rs` (tests cut, four crate-internal imports stubbed; blob `52d407a2a8`, sha256 `f9f80a6e…`) with mutants. The unmutated copy reproduces the committed `determined` counts (table 518, sweep 305 on the 25-bit residue-3 instance 0) and 1000 / 1000 correct. 25-bit, residue 3, T = 1000, table c = 2.0 and sweep c = 1.0:

| variant | table: right / WRONG / none | sweep: right / WRONG / none |
|---|---|---|
| unmutated copy | 1000 / 0 / 0 | 1000 / 0 / 0 |
| m1: extracted `k` off by +1, guard on | 0 / **0** / 1000 | 0 / **0** / 1000 |
| m2: same, guard off (shows the mutation bites) | 0 / 1000 / 0 | 0 / 1000 / 0 |
| m3: sign `tau` flipped for `j % 3 == 0` in `collect_relations_table`, `relation_holds` disabled, guard on | 13 / **0** / 987 | not applicable (table collector only) |
| m4: same, guard off | 13 / 987 / 0 | not applicable |
| m5: factor-base log +1 on every 5th column, guard on | 680 / **0** / 320 | 603 / **0** / 397 |

  No wrong log was ever returned with the guard present. (An earlier m3/m4 that flipped `tau` for every relation was an equivalent mutant and returned 1000 correct even with the guard off: negating every sign negates every factor-base log consistently and the extraction cancels it. I replaced it with the partial flip above.)
- Certificate-discipline gap (T16): the committed rows carry counts (`ok`, `determined`), not (Q_t, k) pairs; the logs cannot be re-verified from the artifacts without re-running. My replay closes this for all 14 rows of each arm; the producer should commit a certificate file or store reference.

### 2.4 Disjoint curves: PASS (qualified)

`find_a3_curve(bits, res)` is deterministic (largest prime `p < 2^bits` with the residue, smallest `b` giving prime order), enumerated with the committed code (`curves.jsonl`):

| set | shares (p, a, b, n) with the evaluation |
|---|---|
| this exploration, 26 and 30 bits | none |
| sweep probes, 24 and 28 bits | none |
| previous evaluation (23, 26, 30, 31, 33 bits) | none |
| previous campaign's exploratory 24/28/32 bits | **(32, res 3) and (32, res 1) are the same curves** |

Evaluation curves: (25,3) p 33554383 b 1 n 33561557; (25,1) 33554393, 56, 33549773; (27,3) 134217487, 6, 134219009; (27,1) 134217689, 58, 134204683; (29,3) 536870879, 64, 536867189; (29,1) 536870909, 6, 536864621; (32,3) 4294967291, 31, 4294959973; (32,1) 4294967197, 26, 4294887283. The committed rows' `n` equals the enumerated `n` on every row. T3 as written (disjoint from the 26/30 exploratory curves) holds. The 32-bit curve pair was also used by the earlier campaign (single-target `prime_fast_vs_rho`, and the uncommitted 32-bit multi-target exploration cited in `RESULTS_prime_fast_multi.md`); the record does not show that either arm's constant was chosen on it, but `frozen.json` lists the disjointness only for 24, 26, 28, 30.

### 2.5 Costs, timed region, arms, controls

**Timed region: PASS.** `ic_multi_solve` (prime_fast.rs 1704 to 1803) times, inside `t0`, `FastFactorBase::build`, `PairTable::build`, the G-only relation collection, `solve_fb_logs`, then per target up to 32 collector calls, the extraction and the `kG == Q` check; discarded attempts are timed. In `prime_fast_multi.rs` the clock (lines 59 to 61) brackets only `ic_multi_solve`; planted `ks`/`qs` (lines 56 to 58) are built before it. In `prime_fast_bsgs_multi.rs` the clock (line 28 to 88) covers the baby table, the giant steps, the lookups and the `c.mul(g,k) == Q` verification; `ks`/`qs` are built before. The `table = false` path is the unchanged pre-existing call `collect_relations(..., Some(&pairs), ...)` (diff `43e497d804..671cec184f`): the sweep arm is the previously evaluated solver, and the two IC arms share factor-base build, pair table, elimination, extraction and verification. A/B design is sound.

**Arms and unit.** Three arms on identical targets, wall clock only (one thread everywhere; the only `threads` in the examples is the rho reference configuration). The matched reference named in AGENTS.md (Pollard rho) is not an arm. The rows still carry `rho_single_ms`, `ratio_vs_rho_Tx`, `ratio_vs_rho_ideal`, computed with `rho_parallel`'s default 1024-entry table, which the merged validator report for the earlier evaluation showed dominates rho's small-size cost (here `rho_single_ms` 2.49 ms at 25 bits to 5.40 ms at 32 bits while sqrt(n) grows 11x). RESULTS does not cite them; they must not be cited.

**Sweep baseline tuning (c-grid probe, not frozen protocol).** Evidence binary, curves outside the evaluation (28 bits is one of the sizes the sweep constant was originally probed on; 31 bits was an earlier evaluation size), `T = 1000`, mean `total_ms` over rows, all `ok = 1000`:

| arm | 28 bits (4 rows per c) | 31 bits (2 rows per c) |
|---|---|---|
| sweep, c = 0.5 / 0.75 / 1.0 / 1.5 / 2.0 | 476 / 406 / **381** / 613 / 1227 | 2167 / - / 2207 / 3674 / 5922 |
| table, c = 1.0 / 1.5 / 2.0 / 2.5 / 3.0 | 411 / 414 / **340** / 350 / 447 | - / 1376 / **1368** / 1504 / 1841 |

The frozen constants are at or next to the optimum of each arm (sweep c = 1.0 is within 2% of the best at 31 bits; table c = 2.0 is best at both sizes), so the sweep baseline is not under-tuned. At 28 bits the table/sweep ratio at the optima is 0.89 (rows 0.88, 0.81, 1.05, 0.85); the 1.05 is a row where the table arm is slower.

**BSGS baseline (T12, T14).** The committed arm does one affine addition, with its own inversion, per baby step, and a SipHash `HashMap`. Control `harness/bsgs_tuned.rs` (a probe outside the frozen protocol): same B and targets, baby steps batched across 512 lanes with one inversion per batch, a multiplicative hasher, giant-step phase copied from the committed code, internal `kG == Q` guard, 1000 / 1000 correct on every run (not re-verified by the Python checker). Same session (11 Nov), instance 0, ms:

| bits, res | table2 | sweep3 | BSGS committed algorithm (mean of 2) | BSGS tuned | table2 / BSGS committed | table2 / BSGS tuned | BSGS committed / tuned |
|---|---|---|---|---|---|---|---|
| 27, 3 | 260 | 450 | 75.8 | 28.7 | 3.43 | 9.0 | 2.6 |
| 27, 1 | 217 | 394 | 82.8 | 28.4 | 2.62 | 7.6 | 2.9 |
| 29, 3 | 525 | 648 | 162.5 | 59.2 | 3.23 | 8.9 | 2.7 |
| 29, 1 | 495 | 667 | 171.4 | 53.8 | 2.89 | 9.2 | 3.2 |
| 32, 3 | 2186 | 5483 | 491.7 | 141.6 | 4.45 | 15.4 | 3.5 |
| 32, 1 | 2275 | 4226 | 526.1 | 135.3 | 4.32 | 16.8 | 3.9 |

B multipliers 0.7 and 1.4 stay within about +-20% of 1.0 (32 bits: 125 to 142 ms), so `B = ceil(sqrt(T n / 4))` is near its optimum. The distance remaining to a competent BSGS is therefore several times what the report states and grows with size. No external fastest implementation (an independent BSGS or multi-target rho library) was available in this sandbox; none was run.

**Memory (unpriced; byte counters are listed, not charged, per §5a).** Pair-table entries (16 B each) from the committed `PairTable::build`, BSGS table entries `B`, and peak resident set of the dump process (`VmHWM`, includes the targets and the relation matrix):

| bits | table2: fb, pair entries, MB | sweep3: fb, pair entries | BSGS: B entries | peak RSS MB table2 / sweep3 / BSGS |
|---|---|---|---|---|
| 25 | 645, 416 025, 6.7 | 323, 104 329 | 91 583 to 91 600 | 11-12 / 4 / 4 |
| 27 | 1024, 1 048 576, 16.8 | 512, 262 144 | 183 170 to 183 180 | 26 / 8 / 6 |
| 29 | 1625, 2 640 625, 42.2 | 813, 660 969 | 366 356 to 366 357 | 63 / 17 / 11 |
| 32 | 3251, **10 569 001, 169.1** | 1625, 2 640 625, 42.2 | 1 036 206 to 1 036 215 | 244 / 63 / 36 |

T15 states "about 2 million at 32 bits"; that is the sweep arm's 2.64 M. The table arm holds 10.57 M entries (169 MB, plus an 84.7 MB relation matrix) and peaks at 244 MB against 36 MB for BSGS (6.8x; 3x at 25 bits, growing).

### 2.6 Wording that overclaims

| line | text | problem | suggested replacement |
|---|---|---|---|
| 3 | "not yet independently replayed" | superseded | cite this report; keep `[pcc:unverified]` (no pcc claim id exists) |
| 7 | "pushed and checked on the remote before the run" | push time not recorded; cannot be separated from the first run at 1 s resolution | "committed locally 10:08:37Z, before the first row (10:08:46); remote push time not recorded" |
| 7 | "disjoint from them" | true for 26/30; 32-bit curves were used earlier | add "the 32-bit curves were used in earlier exploratory runs" |
| 13 to 16 | 27-bit sweep3/BSGS "8.6" | 8.548 | "8.5" |
| 18 | "every row below 1.0 ... a reproducible improvement" | the row range is one pass; 1 of 92 row-level measurements at the evaluation sizes (14 committed, 72 in six replicate sets, 6 same-session) reached 1.035 | "table2/sweep3 median below 1.0 in all seven measurement sets (0.50 to 0.65); single rows 0.35 to 1.04" |
| 18 | "1.5x to 2.8x faster" | extremes of one pass | "about 1.5x to 2x (pooled medians), up to 2.8x on single rows" |
| 18 | "2.9x to 5.0x faster [BSGS]" | per-size medians of one pass against an untuned BSGS | "2.4x to 5.4x (pooled over replicates) against the repository's untuned BSGS arm; about 8x to 17x against a batched-baby-step BSGS in the same session [validator probe, not frozen]" |
| 20 | "about 2 million at 32 bits" | 10.57 million for the table arm | "about 10.6 million (169 MB) at 32 bits for the table arm; the sweep arm's 2.6 million" |

## 3. Reporting contract

| requirement | verdict | evidence |
|---|---|---|
| boundary and reference named before measuring | partial | `frozen.json` names the arms and BSGS `B`, an abandon rule for table/sweep, no floor, no rho arm; AGENTS.md names rho as the reference |
| one table, one unit, ratio columns | **fail** | wall clock only; the program's `S = operations / sqrt(n)` is absent; ratios to sweep3 and BSGS are given |
| all arms priced including the reference, §5a band | **fail** (acknowledged) | no 0 / 0.1 / 1 band for any arm; byte counters (pair table, BSGS table) not listed with counts in RESULTS (supplied in 2.5); no ranking is valid under §5a |
| phases separated | partial | `setup_ms` vs total per row; no per-phase split inside setup or per target |
| progress as a ratio | pass | ratios to sweep3 and BSGS |
| certificate discipline | partial | counts only in the artifacts; independent replay passes (2.3) |
| controls | partial | no null control in the artifacts (supplied in 2.3); sweep arm tuned as well as the table arm (2.5); BSGS arm untuned (2.5); no matched rho arm; no external implementation |
| single-pass timing | partial | one pass per row, fixed arm order, no A/A; replicates in 2.2 |
| m = 83 / ECC2K-130 disclosure | not applicable | prime-field toy curves only; no such claim |
| execution route (CLAUDE.md: isolab-eks, bench.record, resume) | **deviates** | no bench record, no `.resume` evidence, no S3 reference, local sandbox; not eligible for `benchmarks/README.md` |
| ecbench (AGENTS.md section 12) | not used | wall-clock figures need L2 isolation and an A/A interval there; none exists here; no `vs_rho` claim is made, and none should be |
| validation gate (`docs/VALIDATION_GATE.md`) | not met | no `VALIDATION.json`, no matched rho with §5a band; this report is one required piece |

## 4. Open items for the producer

1. Amend `RESULTS_prime_fast_multi_table.md` per 2.6: the 32-bit table size, 8.5, the qualifiers on "every row" and "reproducible", the BSGS gap, the status line.
2. Tune the BSGS arm (batched baby steps, cheap hash) before using it as the reference, and restate the distance remaining (about 8x to 17x on 27 to 32 bits in the validator's probe). `examples/prime_fast_bsgs_multi.rs` is the arm to fix; the probe is `harness/bsgs_tuned.rs`.
3. Add the matched rho arm (right-sized table, not `rho_parallel`'s 1024-entry default), the §5a band, the memory counters (pair-table entries, BSGS-table entries) and a counted unit; run the repeats with alternating arm order and an A/A interval.
4. Commit a certificate file (or store reference) of the (Q_t, k) pairs for every arm and a null control; commit the 24/28-bit probes behind sweep `c = 1.0`, or cite the c-grid in 2.5.
5. Record argv, host, core count, rustc version and dirty-tree state in `provenance.txt`; hash both example binaries after the run and keep them (the BSGS hash could not be matched).
6. Drop the unreported `rho_*` columns from the rows or label them as measured with the 1024-entry table.

## 5. What this validation did not do

No different-host-class or isolab-eks replay; no ecbench session or A/A interval; no pcc canary or ledger check; no research-kb queries (no kb tools in this session); no external fastest BSGS or multi-target rho implementation; no run at T other than 1000; no sizes beyond 32 bits. The tuned-BSGS and c-grid numbers are exploratory probes outside the frozen protocol on the restarted VM, one instance each (c-grid 28/31 bits: 4 and 2 rows per c), and do not replace it. The Python checker verifies the dumps of the committed IC solver and of a copy of the BSGS algorithm; the shipped BSGS binary and the tuned control were checked only by their own group-guard and the planted `k`.

## 6. Reproduction (scratch, not committed)

All under `/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/val/`:
- Producer binary copy `evidence_bin/prime_fast_multi` (`25e0e935…`); `evidence_bin/prime_fast_bsgs_multi_rebuilt_by_validator` (`a2fb6933…`); HEAD rebuild `target_head/release/examples/` (`2f7900ea…`, `e94e8783…`), log `build_head.log`.
- Scripts (sha256 first 12): `verify_logs.py` `8529dfb94eae`, `recompute.py` `2d004f0f9cef`, `cmp_replay.py` `66544e0ee134`, `make_mutants.py` `ec07cb9d324a`, `harness/dump_multi.rs` `1e3786e4285c`, `harness/dump_bsgs.rs` `ea67ac8a0fdd`, `harness/bsgs_tuned.rs` `c5ac96b68dc8`, `harness/mem_probe.rs` `57d221bd6160`, `harness/curves.rs` `267a36e49fe5`; mutants `mut/pf_{unmut,m1,m2,m3,m4,m5}*.rs` (`ef2484dc18eb`, `f4c2ad665704`, `c3e4cbf11d87`, `a3eaf4734f16`, `885f21f2aa06`, `961c65d002db`).
- Data: `dump/*.jsonl`, `rerun1/` (replay A), `reps/` (repeats 1 to 4), `rerun_head/` (replay B), `cgrid/`, `bsgs_ctl/`, `sess/`, `nullctl/`, `curves.jsonl`.
- Commands: `prime_fast_multi <out> <bits> 1000 2.0 <inst> table`, `... 1.0 <inst>`, `prime_fast_bsgs_multi <out> <bits> 1000 <inst>`; `dump_multi_real <out> <bits> 1000 <c> <inst> table|sweep`; `DUMP=1 dump_bsgs <out> <bits> 1000 <inst>`; `python3 -I verify_logs.py dump/<arm>_{25,27,29,32}.jsonl`; `bsgs_tuned <bits> 1000 1 <bmult> 512`; `mem_probe <bits> <c> 3`; harnesses compiled with `rustc --edition 2021 -O` against `target/release/deps/libcrypto_lib-42993a31e73be122.rlib` (built 2026-11-07 10:06:46); mutants compiled stand-alone from `mut/*.rs`.
