# Validator report: pair-sum table confirmation panel (`results/confirm`)

Validator: independent session, 2026-11-07. Scratch evidence (not committed, must be re-derivable): `/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/` (scripts `item1.py`, `item2.py`, `item2_neg.py`, `replay.py`, `cmp.py`, `equiv.py`, `model.py`, `hist_check.py`, `cg.py`; outputs `replay_full1.json`, `replay_full2.json`, `cg_summary.json`, `hist/*.jsonl`, `cargo_test.log`).

## Binding: what was validated

| item | value |
|---|---|
| Results commit validated | `f7fae7d87b` (adds `results/confirm/rows.jsonl`, `analysis.txt`); runner and instances from `b6d5b525f1` |
| HEAD when I started | `1a259b4b40` (clean tree) |
| HEAD at the end | `c90013978b`. **The branch moved during validation**: another session committed `0d02c0b860` (p cap 4e7, 56 larger instances) and merge `c90013978b` in the shared checkout. I did not validate the moving tree; I pinned the claim artifacts by path. `git diff f7fae7d87b HEAD` is empty for `results/confirm`, `results/instances_confirm`, `run_confirm.py` and `src/cryptanalysis/ec_index_calculus.rs`. The only `src/` change is `src/bin/icx/prime_external.rs` (p/n range cap), which does not affect p <= 65521 |
| Binary | `target/release/ic`, mtime 2026-11-07 00:42:40 (before the run at 00:44:23 to 00:45:30), sha256 `5378fc35eb60e08aaa48f72e7f00495233a34018c028b60779a3156158c075ea`, self-reported `binary_blake3` `abcc7ee9112c724954c4fd991f9c534e9e9c3ee9e1d5e44982fecc243d447df0`; unchanged (hash and mtime) at the start and the end of my work. Its string table has the old p cap `5..=1000000`, so it predates `0d02c0b860`. Not rebuilt |
| Host | 4 vCPU Intel Xeon 2.1 GHz VM, loadavg about 0.2 to 1.2 during my runs, shared checkout, no isolation (not isolab-eks, no L2 admission) |

## Disposition

**`validated`, scoped, with four required corrections to the wording of the claim (C1 to C4 below).** The numbers reproduce: every run recovers the planted log (480 of 480 replayed runs, 240 of 240 recorded) and D / B is 0.411 recorded, 0.410 and 0.386 in my two full replays. Three parts of the claim as written do not hold as stated: "never used in earlier panels" is false (C1), the `0.41x` is a solver-stage-timer ratio that includes about 16% of avoidable baseline work unrelated to the table (C2, C3), and "same relations" is verified only as identical relation and trial counts, not row-for-row (C4). This report does not satisfy the validation gate: there is no `VALIDATION.json`, no matched Pollard rho reference, no 0 / 0.1 / 1 GAE pricing and no L2 run, so the row stays "unvalidated (pre-gate)".

## Requirement table

| # | requirement | result | key numbers | evidence |
|---|---|---|---|---|
| 1a | 30 instances x 2 seeds x 4 arms, no duplicates, no dropped or failed rows | PASS | 240 rows, 240 distinct (name, seed, arm) cells, 0 missing, 240 `complete`, 240 `ok`, 0 null metrics; relations per arm constant 72 / 320 / 130 / 34 = fb + extra; row order equals the runner's seeded shuffle `Random("confirm-v1")`, so nothing was reordered or removed | `item1.py` |
| 1b | medians and paired ratios recomputed | PASS | see table below; all of `analysis.txt` reproduces | `item1.py` |
| 1c | instances unused by any other panel | **FAIL (literal), PASS for tuning** | see C1 | `item1.py`, `hist_check.py` |
| 2 | instance validity | PASS | 5 random (seed `validator-item2-2026-11-07`: W15-30, W16-39, W15-38, W16-34, W15-36) and all 30 pass | `item2.py`, `item2_neg.py` |
| 3 | replay | PASS (timing flag) | 240 rows replayed twice, 480 of 480 identical scalar, trials and relations | `replay.py`, `cmp.py` |
| 4 | table path soundness, default path unchanged | PASS (no unit test covers the table path) | reading of the diff plus 600 sweep-vs-table equality checks | below |
| 5 | fairness and misleading-claim review | 2 FAIL findings, others noted | C1 to C4, F1 to F8 | below |

### Item 1: recomputation (all from `rows.jsonl`, own code)

| arm | median total_ms (mine / `analysis.txt`) | median trials | trials per relation |
|---|---|---|---|
| A fb 8, extra 64 | 688.71 / 688.7 | 17546 | 243.7 |
| B fb 256, extra 64 | 38.92 / 38.9 | 326 | 1.02 |
| C fb 128, extra 2 | 29.88 / 29.9 | 197 | 1.52 |
| D fb 32, extra 2, table | 16.33 / 16.3 | 567.5 | 16.7 |

| paired ratio (60 instance-seed pairs) | median | bootstrap 95% (per pair) | bootstrap 95% (cluster by instance) | min to max | wins |
|---|---|---|---|---|---|
| D / B | 0.4112 | [0.392, 0.459] | [0.396, 0.458] | 0.209 to 0.695 | 60 / 60 |
| D / C | 0.5211 | [0.491, 0.583] | [0.490, 0.583] | 0.257 to 0.948 | 60 / 60 |
| C / B | 0.7428 | [0.690, 0.818] | [0.690, 0.818] | 0.437 to 1.442 | 49 / 60 |
| D / A | 0.0234 | [0.022, 0.025] | [0.022, 0.025] | 0.011 to 0.044 | 60 / 60 |

Other views of the same rows: ratio of medians D / B 0.4197, ratio of sums 0.4200; per size D / B 0.424 (14 bits, n 14), 0.409 (15, n 16), 0.445 (16, n 22), 0.443 (17, n 8); by seed 0.400 and 0.456; by run-position quartile 0.432, 0.441, 0.422, 0.396 (no drift). Arm C's median is unstable within the run (first half 26.4 ms, second half 44.9 ms, different instance mixes), so the D / C and C / B figures carry more noise than D / B.

**C1 (freshness, FAIL as worded).** The 30 files in `instances_confirm` are byte-identical to `instances_holdout_v3/W{14,15,16}-{30..39}.json`. They do not overlap by name or content with `instances`, `instances_holdout`, `instances_holdout_W`, `instances_scale`, `instances_table`, `instances_timed` or `instances_v3_sub`, and the rows of the timing and fb-selection panels (`timed` 600 rows, `timed_ext` 360 rows, `table` 600 rows, recovered from git history, sha256 equal to their `.artifacts.json` manifests) contain 0 of them. But the `extra_v3` panel ran the whole v3 set: `e64` and `e2` rows (840 each, hashes match their manifests) contain all 30 confirm instances, 90 rows each, at fb 8, metric `trials_total` only. So `RESULTS_sum_table.md` ("never used in a panel", "three disjoint fresh instance sets") is wrong. The reuse touched the extra-relations failure-rate evidence for extra = 2, not any timing or fb/table-size choice. Impact is small, but the sentence must be corrected.

### Item 2: instance verification (own Jacobian-coordinate arithmetic, no repo code)

For all 30 instances, including the 5 sampled: p prime (Miller-Rabin, 12 bases); discriminant nonzero; G and Q on the curve with reduced coordinates; n prime; n*G = O; known_log in [2, n-1]; known_log*G = Q; n*Q = O; |n - (p+1)| <= 2 sqrt(p), n^2 > 16p, so n is the only multiple of n in the Hasse interval and the group order equals n (cofactor 1, `group_order` field agrees); `r_bits` = bit length of n. p in {16381, 32749, 65521}, 10 instances each; n from 16183 to 65899; 30 distinct curves, 30 distinct (G, Q). The sha256 of each of the 30 files equals its entry in the generator's `instances_holdout_v3/MANIFEST.json` (0 mismatches), so they are unmodified generator output. Negative control: known_log +1, -1 and n - known_log are all rejected, and an off-curve Q is rejected. Because n is prime and G has order n, the log is unique, so "scalar equals known_log" is equivalent to "scalar*G = Q".

### Item 3: replay (existing binary, same argv and env as `run_confirm.py`, input file without the fixture)

I replayed all 240 rows, twice, in two different shuffled serial orders, not only 10. Both passes: status `complete` 240 / 240, scalar equals `known_log` 240 / 240, `trials_total` identical to the recorded value 240 / 240, `relations_collected` identical 240 / 240, and both passes agree with each other row for row. Sample of 14 (seeded, 3 each of A, B, C and 5 of D):

| row | instance | seed | arm | scalar | trials rec / replay | rels | total_ms rec / replay 1 / replay 2 |
|---|---|---|---|---|---|---|---|
| 66 | W16-37 | 2 | A | 4890 ok | 38932 / 38932 | 72 / 72 | 1748.6 / 1663.0 / 1733.5 |
| 26 | W16-32 | 1 | A | 46787 ok | 29817 / 29817 | 72 / 72 | 1102.1 / 1225.5 / 1343.4 |
| 162 | W15-31 | 1 | A | 8547 ok | 23542 / 23542 | 72 / 72 | 1048.7 / 833.5 / 1009.1 |
| 219 | W16-36 | 1 | B | 12916 ok | 358 / 358 | 320 / 320 | 68.3 / 84.0 / 88.4 |
| 236 | W14-35 | 2 | B | 10332 ok | 321 / 321 | 320 / 320 | 24.8 / 23.5 / 24.6 |
| 94 | W16-30 | 1 | B | 22435 ok | 370 / 370 | 320 / 320 | 86.4 / 72.6 / 69.8 |
| 28 | W14-33 | 2 | C | 14672 ok | 144 / 144 | 130 / 130 | 13.3 / 19.6 / 13.0 |
| 6 | W15-33 | 2 | C | 27470 ok | 193 / 193 | 130 / 130 | 28.4 / 44.2 / 28.5 |
| 194 | W16-35 | 1 | C | 209 ok | 299 / 299 | 130 / 130 | 57.9 / 84.7 / 57.2 |
| 93 | W16-36 | 2 | D | 12916 ok | 1023 / 1023 | 34 / 34 | 39.3 / 28.7 / 42.7 |
| 117 | W15-32 | 2 | D | 20430 ok | 526 / 526 | 34 / 34 | 13.4 / 21.7 / 20.4 |
| 165 | W14-35 | 1 | D | 10332 ok | 423 / 423 | 34 / 34 | 11.9 / 13.7 / 13.1 |
| 99 | W16-34 | 2 | D | 39722 ok | 1317 / 1317 | 34 / 34 | 35.1 / 32.4 / 34.2 |
| 195 | W16-32 | 2 | D | 46787 ok | 1172 / 1172 | 34 / 34 | 37.2 / 36.1 / 32.0 |

**Timing flag (the 30% rule).** Per-arm median replay / recorded total_ms is 1.04 to 1.09 in both passes (all-row median 1.053, so my replays ran about 5% slower). But single-row timing is not reproducible to 30%: 59 of 240 rows (pass 1) and 66 of 240 (pass 2) differ from the recorded value by more than 30%, and two replays of the same row differ by more than 30% in 50 of 240 rows (same-row ratio p5 to p95 about 0.63 to 1.6 for B, C and D; A is steadier, 0.83 to 1.18). Aggregates are stable: replay D / B 0.410 and 0.386 (recorded 0.411), D / C 0.536 and 0.528, C / B 0.776 and 0.776, D / A 0.0240 and 0.0232; D beat B in 60 / 60 pairs in both passes (largest D / B 0.77 and 0.79). The run-to-run spread (about 3%) is as wide as the stated CI, so quote D / B as "about 0.4", not as 0.411 to three digits.

### Item 4: diff of `src/cryptanalysis/ec_index_calculus.rs` against `origin/main`

Fetched `origin/main` (`a98b62d70a`); the two-dot diff of this file is identical (same md5) to the diff against the old merge base `ff60182b66`. +111 / -8 lines, three hunks:

1. Type `PairSumTable = HashMap<(BigUint, BigUint), (i, s_i, j, s_j)>`, `build_pair_sum_table`, `find_one_relation_table_with_rng` (new, only reachable through the switch).
2. In `ec_index_calculus_dlp_staged_with_rng`: `let sum_table = if std::env::var("IC_SUM_TABLE") == Ok("1") { Some(build_pair_sum_table(..)) } else { None }`, placed after `relations_started`, so the build is inside `relations_ms` and `total_ms`.
3. The relation call becomes `match &sum_table { Some(t) => table path, None => find_one_relation_counted_with_rng(..same arguments..) }`.

**Can the table path return a relation that is not a true decomposition R = +-F_i +-F_j of the sampled R = aG + bQ? No, by construction.** Every table key is the affine point returned by `Point::add` on `+-F_i` and `+-F_j` for the stored `(i, s_i, j, s_j)`; the lookup requires equality of both x and y with R; sums equal to infinity (`F + (-F)`) never enter the table; `Point::add` handles doubling and P + (-P). The row it builds uses the same convention as `try_finalise` (entries multiply log F; i == j gives +-2; coef_a, coef_b copied from the sample). RNG consumption per trial is identical (two `sample_nonzero` draws), and the sampled R is the same expression. Second line of defence: the solver only reports success if `[x]G == Q` (`verify` stage, plus a second check in the CLI); status `complete` implies a correct scalar. The default path is unchanged: `find_one_relation_counted_with_rng`, `try_finalise` and `build_factor_base` have no hunks; the only default-path additions are one `std::env::var` read per solve and the `match`. Factor-base `idx` equals the list position (`idx: out.len()`), so the table's positions and the sweep's `idx` agree.

What the table path can do differently: when R has two or more distinct representations, the sweep returns the first `i` in factor-base order and the table returns the first-inserted one (outer loop `i`, `j >= i`). My Python model (independent factor base and pair sums) finds that at fb 32 a mean 3.5% (max 8.4%) of decomposable points have more than one distinct representation (fb 256: 87%, fb 128: 44%). So row content can differ from the sweep for those relations, though each is still a true relation.

Equivalence evidence, independent of the producer's summary:
- Earlier panel (`table/rows.jsonl`, recovered from `5db8af0dee~1`, sha256 `ee9e3c88...` = manifest): 300 of 300 (instance, seed, fb) pairs have identical `trials` and `rels` for sweep and table.
- My own on the confirm instances (status, scalar, trials, relations, per-relation trials min / median / max, exhausted attempts): identical in 300 of 300 sweep-versus-table pairs (fb 8 / extra 64, 16 / 2, 32 / 2, 64 / 2, 32 / 64; 30 instances x seeds 1, 2).
- Independent count model (Python factor base and pair sums; expected trials = (fb + extra) * n / T where T is the number of distinct decomposable points): sum of recorded trials over the 30 instances divided by model is A 1.012, B 0.998, C 0.978, D 1.004. A table that held false or missing keys would show here.
- Extra = 2 safety (the speedup depends on it): seeds 3 to 12 on all 30 instances, 300 runs each, D config and C config: 300 / 300 and 300 / 300 recovered.

Gaps: no unit test exercises `IC_SUM_TABLE` or `build_pair_sum_table` (grep over `src` and `tests`); the JSON report does not record which path ran (`config` has no table flag), so an arm is identified only by the runner's environment.

`cargo test --release --lib --offline ec_index_calculus` (run in the shared `target/`, 10 min 25 s wall including the test-profile compile, so under the 15 minute limit; the `ic` binary was untouched, hash and mtime unchanged afterwards): **43 passed, 0 failed** (20 in `ec_index_calculus::tests`, the rest in `ec_index_calculus_curves` and `ec_index_calculus_j0`), log `cargo_test.log`. This shows the default-path tests pass at HEAD sources; none of the 43 exercises the table path, so it adds nothing to the table-soundness argument above.

### Item 5: what could make the comparison unfair or misleading

**C2 (scope of "0.41x").** `total_ms` is the solver's stage timer (factor base + relations including table build + matrix assembly + elimination + verify). The process also pays about 25 ms per run outside the timer in every arm (median wall minus total_ms 25.1 to 26.2 ms for B, C and D; callgrind: `compute_binary_hash`, a blake3 over the 59 MB executable, is about 150 M instructions, plus input validation and curve counting). It is arm-independent, so it does not bias the stage ratio, but process-level D / B is 0.637 and 0.630 and D / C 0.731 and 0.712 (replays 1 and 2). State "solver stage" next to every 0.41.

**C3 (attribution; F1).** The sweep path in `find_one_relation_counted_with_rng` rebuilds the `x_to_idx` HashMap<BigUint, usize> (a `BigUint` clone and insert per factor-base entry) on every call, i.e. once per relation: fb x (fb + extra) inserts per solve. Callgrind on the unmodified binary (no rebuild): in a B run (W15-30, seed 1) `HashMap::insert` is called 81,920 times and costs 64.2 M of 382.7 M solver instructions (16.8%); over 10 sampled instance-seed pairs the share is 16.1% median for B, 4.3% for C and 0 for D and A, and this excludes the clone, allocation and free. The table path builds one table per solve and does not pay this. A competent sweep would hoist the map, so part of the D / B gain is a baseline inefficiency, not the pair-sum lookup. Deterministic instruction-count ratios on the same 10 pairs (medians of paired ratios): D / B 0.441, D / C 0.536, C / B 0.785 (so the speedup is real and not a timer artifact); and with only the map-insert instructions removed from B or C: D / B' 0.502, D / C' 0.548. No hoisted-sweep arm exists, so the cheaper baseline is missing. Class by the program's table: engineering on the baseline implementation (a valid improvement over the current code), not an advance, and part of it would be obtained by a one-line hoist.

**C4 ("same relations").** Verified as identical counts of trials and relations (above), not as identical rows; the CLI does not expose relations. About 3.5% of decomposable points at fb 32 can legitimately map to a different true decomposition.

Further findings:
- **F2 Confounded arms.** D differs from B in three settings at once (algorithm, fb 32 vs 256, extra 2 vs 64). No confirm arm isolates "sweep, fb 32, extra 2" or "table at fb 128 / 256". C / B (0.743) mixes the extra-relations retune and fb 128 vs 256. The earlier table panel covers same-fb pairs, but it chose the best fb of each arm on its own data. Frozen settings in the confirm panel remove that optimism: fb 256 (B) from `timed` and `timed_ext`, fb 128 (C) and fb 32 (D) from `table`; all three panels share no instance with the confirm set. The best extra = 2 sweep fb is nearly tied between 128 and 256 (32.9 vs 32.5 ms in `timed` / `timed_ext`), and C used 128, which is not unfair.
- **F3 Freeze order not demonstrable.** `run_confirm.py` at HEAD is byte-identical to the version in `b6d5b525f1` ("runs pending", 00:44:35), so the content was frozen. But the instance files were created at 00:44:23, `rows.jsonl` was written at 00:45:30, and the recorded solver time alone sums to 60.5 s (more than the 55 s between commit and last write), so the run began before the commit.
- **F4 Reproducibility bindings missing in the artifacts.** `rows.jsonl` has no scalar (only the runner's `ok` boolean), no commit, no binary hash (the solver's own `software.binary_blake3` was discarded; `git_commit` and `git_dirty` are null in the binary), no host, no load and no env. `analysis.txt` was produced by an inline script that is not committed (I reproduced every point estimate to three decimals and every interval to within 0.005, so its content is correct). No `bench.record/v1`, no `VALIDATION.json` in `results/confirm`.
- **F5 Host and timing method.** Local, shared, unisolated; no A/A interval in the panel (my two replays give one: arm medians within about +-4% of each other, row level +-30 to 60%). Serial, interleaved, one process at a time is sound, and there is no drift by run position.
- **F6 Regime.** Toy sizes (p in {16381, 32749, 65521}, n in 16183 to 65899). Arm B at fb 256 is degenerate here: its pair-sum space (about 131 k sums) exceeds n, so almost every trial succeeds (1.02 trials per relation) and the optimum fb for the sweep is "cover the group"; D works at about 6% decomposable (16.7 trials per relation). Nothing here extrapolates to cryptographic p, as `RESULTS_sum_table.md` already says.
- **F7 No rho or priced reference.** The panel compares IC variants only; there is no Pollard rho arm, no S = operations / sqrt(n) unit and no 0 / 0.1 / 1 GAE pricing, so no distance to rho can be read from it. This is consistent with the "Not established" paragraph of `RESULTS_sum_table.md`. The m = 83 and ECC2K-130 disclosure rules do not apply (prime field toy curves, no such claim).
- **F8 Moving tree.** The branch advanced twice during validation; see the binding table.

## Required corrections before this result is cited

1. Replace "never used in a panel / three disjoint fresh sets" with: disjoint from every timing and factor-base-selection panel; reused from the `extra_v3` trials-only failure-rate panel (fb 8).
2. Write "0.41x the solver-stage `total_ms`" and give the process-level 0.63.
3. Add a sweep arm with the `x_to_idx` map hoisted (or report the instruction-count bound D / B' about 0.50) and label the gain engineering.
4. Replace "identical relations" with "identical trial and relation counts per seed; row content can differ for about 3.5% of decomposable points".

Things I did not do: rebuild the binary, edit any evidence, push or open a PR; I did not write the session log (the dispatch limits writes to this file).
