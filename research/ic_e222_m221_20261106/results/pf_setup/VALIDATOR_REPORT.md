# Validator report: batched setup collection (`pf_setup`), PR isogeny-labs/crypto#3881

Validated artifact set: branch `claude/prime-field-factor-base-calculus-wy2own`, HEAD `ba4ac55912f03b78cb2f4a469b29798e54188b82` (clean tree at the start and at the end; HEAD did not move during the validation). Freeze commit `1eba8dd551b09f252dc93596f46b3379c9e91054`; code commit `7f525dda4e` (`collect_setup_batched`).
Validator: Sonnet 5.5 validator subagent, 2026-11-11, working from the artifacts and its own recomputation. Status of this report: [pcc:unverified] validator assertion. Same VM snapshot and host class as the producer (rebuild-and-replay, not an independent-host audit). The reruns were local checks: no isolab-eks run, no `.resume` checkpoint, no `bench.record/v1`; they are validator notes, not repository benchmark records.

## Disposition

**`validated`, scope-limited, with required corrections.** The measurement is sound: every deterministic field reproduces exactly (3 720 of 3 720 on rebuilds plus 1 380 of 1 380 on the producer's own hash-bound binaries), 220 000 returned logs and 25 412 setup relations pass independent big-int checks, no mutant of the lane bookkeeping can make `ic_multi_solve_ex` return a wrong log, and all costs are inside the timed region. The T = 1000 gain of the new arm over the previous arm reproduces (0.56). Several statements in `RESULTS_prime_fast_multi_setup.md` and `CAMPAIGN_SUMMARY.md` section A2 are **not reproduced or not supported** (T = 10 000 "small gain 0.94" and "down from 4.9x to 3.8x"; "none at 25 bits"; "2.9x to 9.2x"; the pre-change 27-bit timings are not in any artifact; c per T is post hoc and does not extend to T = 30 000). See "Required corrections". It is an **engineering** change to the project's own multi-target IC (AGENTS.md section 3); the files do not classify it. It does not satisfy the validation gate (no artifact-carried certificate, no matched rho arm, no section 5a band, no ecbench L2/A-A admission, no `VALIDATION.json`); the producer says so.

## Binding of this validation

- Source hashes (SHA-256, 16 hex): `src/cryptanalysis/prime_fast.rs` `bbebaf5f08f85085` identical at `7f525dda4e`, `1eba8dd551` and HEAD; `examples/prime_fast_multi.rs` `7f2acb588bcc2a73` and `examples/prime_fast_bsgs_multi.rs` `d41cf219a67650a7` unchanged since `c60ee6297f`. Previous-arm source: `c60ee6297f:prime_fast.rs` `7c0e11595a6b612f` (the file the `pf_batched` freeze `82a6b30f96` used); `94394a0473` adds only the env-gated `IC_MULTI_TIMING` output (diff read). Between the freeze and HEAD four other files changed by the main merge (`src/cryptanalysis/ec_index_calculus.rs`, `src/ecc/point.rs`, `src/bin/icx/prime_external.rs`, and the new example `examples/relation_point_native.rs`); none is on the timed path (`ecc/point.rs` is only used by `to_textbook`/`from_textbook`).
- Producer binaries in `target/release/examples/`: `prime_fast_multi` SHA-256 `562603d8397f5fd3...` (mtime 03:44:04.5) and `prime_fast_bsgs_multi` `984f1584cba097ec...` (02:06:40) equal the two prefixes in `provenance.txt`. I copied them out and ran them without rebuilding (a `cargo build` would have overwritten them).
- My build: `rustc 1.97.0 (2d8144b78 2026-07-07)`, `-C opt-level=3 -C codegen-units=16 --edition 2021` (Cargo.toml has no `[profile]`). Each `prime_fast.rs` is the committed file with exactly the four `use crate::...` lines redirected to a stub module (`is_prime_u64`, `CurveParams`, `FieldElement`, `Point`; checked by `diff` against `git show`: 8 changed lines per file); the examples differ only in the `use crypto_lib::...` line. Third-party rlibs are the repo's own (`rand 0.8.8`, `num-bigint`, `serde_json 1.0.151`). **Not a full `cargo build` of the 943 k-line crate** (1.5 GB of disk free). Fidelity evidence: producer binary (quiet host) / my stub build = 0.95 to 1.04 in time on every arm and cell, and all deterministic fields equal.
- Host: Intel Xeon @ 2.10 GHz (4 vCPU KVM, 16 GiB, kernel 6.18.44). Timing runs pinned with `taskset -c 2`; during the six main passes only brief `nice 19` analysis and file-generation scripts (seconds) ran beside them (load average about 1.0 = the pinned process). The first producer-binary, 27-bit and T-probe runs overlapped with compile and certification jobs on other cores; I discarded them (`*_loaded` directories) and reran them on the idle host. Only the idle-host results are cited.
- Evidence manifest (SHA-256 first 16 hex, taken at start and re-checked at the end, identical): `frozen.json` 72d51461fd61d7b2, `provenance.txt` 4938c5fa74f0467b, `explore_27_timing.txt` 14349014446a9629, `eval_c1_{25,29,31}_T{1000,10000}` 616b9578260d4b35 / 334f1b5ed996f6d2 / 661120acbacca46b / 59de746f4ba83f37 / 3a027e45e678abbd / fe2e8e2befe59945, `eval_c2_...` 07d305d7d90ac888 / afe8c5bb53e83cc2 / 37a47623e8b4b4c4 / e5f4f8c96a435aaa / f18582f4035ccee8 / 888be6f9f0927f85, `eval_bsgs_...` 5793598691881604 / 738815fb830210eb / 2a88196939dfd779 / b31a1843625c1ea8 / 0d5f66ef976a6711 / ac455fe0b66d8175, `RESULTS_prime_fast_multi_setup.md` `dafc93e318a4fc27`, `CAMPAIGN_SUMMARY.md` `5bad82c20eae7df9`. The reference rows `results/pf_batched/*` hash to the same values the previous validator recorded (10 of 10 spot-checked, e.g. `eval_batched_25_T1000` 85201a5a88eee3f7).
- Producer-side provenance gaps: `provenance.txt` holds only the commit and two 16-hex binary-hash prefixes. Missing: command lines, host/CPU, rustc version, pinning/load, dirty-tree state at build time. The rows do not record which setup algorithm ran: `"batched": true` means unbatched setup in `pf_batched` and batched setup in `pf_setup`; `frozen.json` names a `"batched_setup": true` parameter that does not exist in the code or the example CLI. At HEAD the previous arm (batched per-target phase with the per-walker setup) is **no longer reachable by any flag**; replaying `pf_batched` needs `c60ee6297f`.
- Scratch tooling and outputs (not committed): `/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/pfs/` (`out/analysis_6pass.txt`, `out/analysis_quiet.txt`, `out/summary_cert.txt`, `out/summary_mut.txt`, `out/cert/*.jsonl`, `out/rss.txt`, `src/`, `bin/`).

## Claim-by-claim verdicts

| # | Claim (source) | Verdict | Section |
|---|---|---|---|
| 1a | Freeze commit precedes every row; states prediction and abandon rule (RESULTS Protocol) | **validated** | 1 |
| 1b | "pushed and checked on the remote before the evaluation" | **qualified** (push supported by the local reflog, 03:44:33; "checked" unverifiable) | 1 |
| 1c | 25/29/31-bit curves not used to tune this arm; c = 1.0 chosen from the 27-bit exploration | **qualified** | 1 |
| 1d | Frozen protocol covers the headline (c chosen per T) | **not validated** | 5 |
| 2a | RESULTS table and size breakdown recompute from the committed rows | **validated** | 2 |
| 2b | Rebuild reproduces counts; every target verified, 60 arm-runs, no failed run | **validated** (artifact carries no certificate; replay certified) | 2, 3 |
| 2c | T = 1000: new c=1 / previous = 0.56; predictions hold; abandon rule not triggered | **validated** (0.56, 95% interval 0.51 to 0.76 over rows) | 2 |
| 2d | "none at 25 bits" (0.98) | **not validated** (0.76; two outlier rows) | 2 |
| 2e | T = 10 000: c=1 prediction fails (1.47) | **validated** (1.38, 1.33 to 1.62) | 2 |
| 2f | T = 10 000 with c = 2: "small gain (0.94)" | **not validated** (0.98, 0.95 to 1.04; per-pass 0.95 to 1.03) | 2 |
| 2g | Distance to tuned BSGS "3.7x to 3.8x at the median for both T" | **qualified** (T = 1000: 3.8 to 4.3, validated; T = 10 000: 4.3 to 4.5, not 3.8) | 2, 4 |
| 2h | "down from 6.9x (T = 1000) and 4.9x (T = 10 000)" | **T = 1000 validated** (7.6 to 3.8); **T = 10 000 not validated** (4.48 to 4.47) | 2 |
| 2i | Gap to BSGS grows with size (2.9 to 5.0) | **validated** (3.1, 4.3, 5.6) | 2 |
| 2j | 27-bit setup timing "before 52 to 81 ms, after 20 to 30 ms (2x to 3x lower)" | magnitude **validated by re-measurement**; the "before" figures are **not in any committed artifact** | 2 |
| 3a | All returned logs correct (independent re-check) | **validated** (220 000 logs, 0 wrong) | 3 |
| 3b | Setup relations are correct, `a` bookkeeping right incl. restart and infinity paths | **validated** (25 412 relations, 0 failing; 37 infinity events certified) | 3 |
| 3c | A wrong relation cannot yield a wrong returned log | **validated** (mutation: 25 200 guarded targets, 0 wrong) | 3 |
| 4a | All costs in the timed region | **validated** | 4 |
| 4b | Tuned BSGS is a fair reference | **qualified** (compacted BSGS is 1.18x to 1.27x faster) | 4 |
| 4c | Controls: null/A-A, rho, section 5a band, external implementation | **not met** (disclosed by the producer; A/A noise quantified here) | 4 |
| 5 | c = 1 for T <= 1000, c = 2 for T >= 10 000 (Decisions, session log) | direction **qualified**; rule beyond the two tested T **not validated** | 5 |
| 6 | Wording of RESULTS and CAMPAIGN_SUMMARY A2 | **qualified**, 9 required corrections | 6 |

## 1. Freeze, order, disjointness

Timeline (UTC, 2026-11-11; file mtimes with sub-second resolution, commit committer dates, `git reflog` of `refs/remotes/origin/<branch>`):

| time | item |
|---|---|
| 03:37:13.8 / 03:37:23 | `prime_fast.rs` last written / commit `7f525dda4e` (batched setup) |
| 03:44:04.5 | producer `prime_fast_multi` binary built |
| 03:44:29.634 | `frozen.json`, `explore_27_timing.txt` written |
| 03:44:30 | **freeze commit `1eba8dd551`** (adds those two files only; parent `7f525dda4e`) |
| 03:44:33 | reflog: remote-tracking ref set to `1eba8dd551` by "update by push" |
| 03:44:34.326 / .866 | `provenance.txt` / first row (`eval_c1_25_T1000`) |
| 03:44:45 | commit `61cc2821c7` (partial rows, child of the freeze) |
| 03:45:43.77 | last row (`eval_bsgs_31_T10000`) |
| 03:46:10 | commit `0813955089` (rest of the rows, RESULTS) |

- **1a validated.** Every row file's mtime (03:44:34.9 to 03:45:43.8) is after the freeze file (03:44:29.6) and after the push entry (03:44:33); the rows enter git only in descendants of the freeze. `frozen.json` states: new c=1 / previous (committed `pf_batched` rows, c = 2.0) <= 0.85 at T = 1000 (median over rows); new c=1 / tuned BSGS <= 5 at T = 1000 (median); new c=1 / previous <= 1.0 at T = 10 000; abandon if the T = 1000 median >= 1.0. The source did not change between `7f525dda4e` and the freeze (empty diff). Only one finished file was modified after its first commit: `eval_c1_29_T10000.jsonl` was committed empty in `61cc2821c7` while its job was still running (mtime 03:44:52.3) and filled in `0813955089`; consistent with the mtimes, not an edit of finished evidence.
- **1b qualified.** The push of the freeze is supported (reflog entry 03:44:33, about 2 s before the first row). That it was "checked on the remote" has no artifact; the reflog is a local file.
- **1c qualified.** The committed record agrees: `explore_27_timing.txt` holds 27-bit lines only (`fb 512` and `fb 1024` are p^(1/3) at 27 bits), and there is no committed row for the new arm at 25, 29 or 31 bits before the freeze. Qualifiers: (i) the curves, instance seeds and, at T = 1000, the planted targets were already evaluated by the previous arm and by tuned BSGS (`pf_batched`), and the design target (setup is 67% to 87% of the previous arm's total at T = 1000, see the `setup_ms` columns of `pf_batched`) was visible on these very rows; "not used to tune c" is right, "fresh" would be wrong (RESULTS does not say fresh). (ii) Uncommitted smoke runs between 03:37 and 03:44 cannot be excluded. (iii) The exploration does not discriminate c = 1 from c = 2 in total time: the committed totals at 27 bits, T = 1000 are 130, 143, 105, 106 ms for c = 1.0 and 160, 132, 108, 109 ms for c = 2.0 (medians 118 vs 120.5); it shows only the cheaper setup (35 to 46 vs 73 to 110 ms). The frozen choice c = 1.0 rested on that.
- **1d not validated.** `frozen.json` lists `new_c2` as an arm but gives it no prediction and states no rule for choosing c by T; nothing at T = 10 000 was explored before the freeze. See section 5.

## 2. Recomputation, rebuild and rerun

**Table arithmetic (2a).** `/.../pfs/recompute.py` over the committed rows reproduces every figure in the RESULTS table and size breakdown: T = 1000: c1/previous 0.560 (0.435 to 1.288), c2/previous 0.786, c1/BSGS 3.749, c2/BSGS 5.858; T = 10 000: 1.470 (1.051 to 1.837), 0.944, 5.785, 3.802; by size at T = 1000 c1/previous 0.98, 0.50, 0.54 and c1/BSGS 2.94, 3.79, 4.96. Ratio of totals (a second statistic): c1/previous 0.543 and c2/previous 0.768 (T = 1000), 1.380 and 0.919 (T = 10 000). The "previous" figures (6.9x, 4.9x) are previous-arm rows divided by the previous run's BSGS rows (6.858, 4.910).

**Design of the reruns.** Six passes of the whole frozen grid (25 bits x 2 instances, 29 x 2, 31 x 1, both residues, T in {1000, 10 000}), every pass running six arms in a cyclically rotated order, one core: previous arm (`c60ee6297f` build) at c = 2.0 as in the committed reference rows, previous arm at c = 1.0, new arm at c = 1.0 and c = 2.0, tuned BSGS, and a compacted BSGS. The previous-arm c = 1.0 run is an addition that separates the factor-base size from the batching. Statistic: per-row median over the six passes, then the median over the ten rows ("pooled"), with a 95% bootstrap interval over the rows (4 000 draws) and the range of the six per-pass medians.

**Deterministic fields.** n, inst, res, T, ok, fb, c, **determined** (and B, rounds for BSGS) equal the committed rows in **3 720 of 3 720** comparisons over the six passes for the new c=1, new c=2, previous c=2 (vs `pf_batched`) and BSGS arms; the three quiet-host passes with the producer's own binaries add **1 380 of 1 380**. Every row has `ok == T`.

| quantity (median of per-row ratios) | committed | reruns: pooled [95% over rows] (per-pass range) |
|---|---|---|
| T = 1000 new c1 / previous c2 | 0.560 (0.435 to 1.288; 2 rows >= 1) | **0.561** [0.507, 0.760] (0.541 to 0.623); 0 rows >= 1 |
| T = 1000 new c2 / previous c2 (batching alone, c = 2) | 0.786 | 0.821 [0.764, 0.903] (0.743 to 0.860) |
| T = 1000 previous c1 / previous c2 (c alone, no batching) | not measured | 0.974 [0.888, 1.049] |
| T = 1000 new c1 / previous c1 (batching alone, c = 1) | not measured | 0.625 [0.590, 0.697] |
| T = 1000 new c1 / new c2 | 0.735 | 0.740 [0.642, 0.829] |
| T = 1000 new c1 / tuned BSGS | 3.749 | 3.803 [3.064, 5.155] (3.42 to 4.35); producer binaries 4.32 [3.01, 5.06] |
| T = 1000 new c1 / compacted BSGS | not measured | 4.958 [3.819, 7.000] (4.81 to 5.72) |
| T = 1000 previous c2 / tuned BSGS | 6.858 | 7.581 |
| T = 10 000 new c1 / previous c2 | 1.470 (1.051 to 1.837) | 1.380 [1.331, 1.615] (1.34 to 1.58); 10 of 10 rows >= 1 |
| T = 10 000 new c2 / previous c2 | 0.944 (0.782 to 1.398; 3 rows >= 1) | **0.978** [0.948, 1.037] (0.952 to 1.032); 4 of 10 rows >= 1 |
| T = 10 000 previous c1 / previous c2 | not measured | 1.499 [1.371, 1.629] |
| T = 10 000 new c1 / previous c1 | not measured | 0.952 [0.899, 0.998] |
| T = 10 000 new c1 / new c2 | 1.441 | 1.456 [1.319, 1.554] |
| T = 10 000 new c2 / tuned BSGS | 3.802 | **4.470** [3.607, 5.174] (3.79 to 4.56); producer binaries 4.30 [3.25, 5.19] |
| T = 10 000 new c2 / compacted BSGS | not measured | 5.482 [3.888, 6.273] (4.76 to 5.64) |
| T = 10 000 previous c2 / tuned BSGS | 4.910 | 4.481 |
| tuned BSGS / compacted BSGS | | 1.272 (T = 1000), 1.183 (T = 10 000) |

By size (pooled): T = 1000 c1/previous 0.76, 0.50, 0.56 and c1/tuned BSGS 3.05, 4.28, 5.63 at 25, 29, 31 bits; T = 10 000 c2/previous 0.99, 1.00, 0.97 and c2/tuned BSGS 3.32, 4.64, 5.88.

- **2c validated.** The T = 1000 prediction (<= 0.85) and the abandon rule (>= 1.0) behave as stated in all six passes; the upper end of the row interval (0.76) is below 0.85. The tuned-BSGS bound (<= 5) holds against the frozen BSGS arm (3.8 to 4.3) and is borderline against a compacted BSGS (4.96; three of six per-pass medians exceed 5).
- **Attribution (new, from the added arms).** The factor-base constant alone does nothing at T = 1000 (0.97); batching alone gives 0.62 (c = 1) and 0.82 (c = 2); the combination is 0.56. So RESULTS' "batching the setup collection is a reproducible gain at T = 1000 for c = 1" is right, and c = 1 matters only together with the cheaper setup. Setup (pooled medians): previous c=2 319 ms, new c=2 236 ms (-26%), previous c=1 208 ms, new c=1 89 ms (-57%); setup share of the total at T = 1000 / 10 000: new c1 44% / 6%, new c2 73% / 24%, previous c2 79% / 31%.
- **2d not validated.** The committed 25-bit figure (0.98) is an artifact of four rows: new c=1 at res = 1 ran 61.0 and 53.7 ms against 37.6 and 37.2 ms (reruns, per-row median of six) while the reference rows were fast (47.3, 45.3 ms). Per-row ratios at 25 bits in the reruns are 0.73, 0.76, 0.77, 0.80 (pooled 0.76). There is a gain at 25 bits, smaller than at 29 and 31 bits (0.50, 0.56).
- **2f not validated.** With c = 2 at T = 10 000 the new setup changes the total by 0.978 (interval 0.948 to 1.037); the per-pass medians are 0.96, 1.02, 0.95, 0.96, 1.03, 0.97. The committed 0.944 is inside the run-to-run noise of the reference rows. The setup saving (median 83 ms per row) is real but small against totals of 0.2 s, 1.0 s and 2.4 s at 25, 29 and 31 bits (new c=2, T = 10 000), under the noise.
- **2g, 2h.** At T = 1000 the claim reproduces (previous 7.58 to new 3.80, same session; 6.9 to 3.7 committed). At T = 10 000 the same-session pair is **4.48 to 4.47**: no change. The committed "4.9 to 3.8" mixes runs: the new run's BSGS rows (same binary `984f1584...`) are 1.19x slower than the previous run's (median over rows, range 0.92 to 1.33; T = 1000: 1.09, 0.89 to 1.40) and 1.1x to 1.3x slower than the same binary in this session (producer binary on the idle host / committed row: 0.90 at T = 10 000, 0.79 at T = 1000; stub build 0.91 and 0.81). A slower BSGS denominator lowers every IC/BSGS ratio in `pf_setup`. Distance to the tuned BSGS, chosen c: 3.8 (stub build) to 4.3 (producer binary) at T = 1000, 4.3 to 4.5 at T = 10 000; against a compacted BSGS 5.0 and 5.5.
- **Noise (null control, A/A).** Same arm, same row, six passes: max/min of `total_ms` has median 1.19 to 1.43 and maximum 1.39 to 2.07 depending on arm and T (new c1: 1.23 and 1.35, max 1.52 and 1.62; new c2: 1.25 and 1.34, max 1.67 and 1.78; previous c1 at T = 1000: 1.43, max 2.07). A per-row ratio therefore carries 1.3x to 1.5x of noise; per-pass medians of ten rows vary by 1.08x to 1.15x peak to peak (c1/previous at T = 1000: 0.541 to 0.623; c2/previous at T = 10 000: 0.952 to 1.032). RESULTS' "1.2x to 1.3x, up to 1.9x" is consistent.
- **2j (27-bit phases, idle host, `IC_MULTI_TIMING=1`, T = 1000, 12 samples per cell, previous arm = `94394a0473` build).** Collection: previous c=1 62 to 73 ms (median 66), new c=1 21 to 32 (23) = 2.9x lower; previous c=2 47 to 59 (55), new c=2 22 to 29 (26) = 2.1x lower. Pair table 10 to 17 ms (c = 1) and 44 to 56 ms (c = 2), elimination 1.7 to 2.8 and 8 to 14 ms, setup c=1 36 ms (previous 81), c=2 87 ms (previous 114); total 113 vs 161 ms (c = 1) and 131 vs 163 ms (c = 2). The magnitudes agree with RESULTS. But `explore_27_timing.txt` contains only the "after" run (collect 20.3 to 30.2 ms); the "before" numbers (collection 52 to 81, pair table 14 to 65, elimination 2 to 14) are in no committed file.
- **Order effect.** The fixed rotation puts every arm in every position; per-pass medians show no systematic order effect larger than the pass-to-pass spread.

## 3. Correctness, certificates, mutation tests

**Certificate discipline.** The committed rows carry only `ok` (an in-process comparison with the planted k); no certificate file exists, and the planted logs are not stored. The run is deterministic, so I re-derived the certificates by replay. `src/drv_e2e.rs` calls `ic_multi_solve_ex(&c, g, &qs, fb, 6, 5_000_000, ins + 1, true, true)` exactly as the example does (same seeds, same target formula) on the committed source and dumps p, a, b, n, G, every Q_t, every planted k and every returned log. `certify.py` (Python ints only, no code shared with the solver) checks p and n prime, the curve nonsingular, G on the curve, n*G = O, n inside the Hasse bound, Q_t on the curve, planted k*G == Q_t, and **returned l*G == Q_t**.

- **3a.** Frozen grid, both arms (c = 1 and 2, 40 curve-rows): **220 000 returned logs verified, 0 wrong, 0 unsolved, 0 planted k*G != Q, 0 returned != planted**; `ok`, `determined`, n and fb equal the committed row on all 40 curve-rows. Targets are identical across the IC and BSGS arms (same seed formula `0xC0DE ^ ins ^ n ^ T` in both examples), so the BSGS arm's `ok` (found == planted) is certified transitively; I did not dump BSGS-found values. A failed certificate would have made the run `invalid_measurement`; none failed.
- **3b, setup-phase certificate.** `drv_setup` calls `collect_setup_batched` (copy with the single change `fn` to `pub fn`) and `solve_fb_logs` with the arguments `ic_multi_solve_ex` passes, and dumps every relation and every factor-base log. Checked in Python: each relation satisfies `a*G == sum s_j*F_j` with `b == 0`; each determined log satisfies `x_j*G == F_j`. 16 configurations (25, 29, 31 bits, c = 1 and 2, both residues, extra 6; and 25 bits with extra 0 or 6 and 1, 7, 256 lanes): **25 412 relations, 0 failing; 20 138 determined logs, 0 wrong; 0 duplicate entry-sets**. The determined fraction is 78% to 82% in every arm (previous c2 0.807, new c2 0.794, new c1 0.800: the batched collector is not worse by more than about 1 point); with fb + 6 relations about one factor-base log in five stays undetermined in every arm.
- **3c, mutation tests** (`make_mut.py`; every patch asserted to match exactly once; the instrumented copy `cnt` adds counters only; mutants build on it):

| mutant | change | relation certificate | end to end, guard on | end to end, guard removed |
|---|---|---|---|---|
| none (control) | | 3 921 relations, 0 failing (25/29 bits); tiny curves 3 600, 0 failing, **37 infinity restarts and 37 exceptional adds certified** | 2 400 of 2 400 verified | 2 400 of 2 400 (control) |
| `restart_nobump` / `restart_plus2` | productive-restart lane `a` off by -1 / +1 | 3 040 of 3 921 failing; 2 962 "determined" logs wrong | 0 wrong (all verified, slower) | **1 059 of 1 200 wrong** |
| `walk_nobump` | non-productive advance does not bump `a` | 3 876 failing | 376 verified, 824 unsolved, 0 wrong | 1 197 of 1 200 wrong |
| `inf_nobump` / `inf_plus2` | infinity branch `a` off by -1 / +1 | 25 bits: 0 (branch unreachable at that size); tiny curves: **17 of 3 600 failing** (4 wrong determined logs) | 0 wrong | unchanged at 25 bits (equivalent) |
| `dup_every_relation` | every relation pushed twice | 0 failing; 1 966 duplicates; determined 3 113 to 68 (25/29 bits) | 471 verified, 729 unsolved, 0 wrong | n/a |
| `no_seen_filter` | duplicate filter removed | 0 failing; 1 281 duplicates on tiny curves | all 2 400 verified | n/a |
| `rank_half` | fb/2 + 6 relations | n/a (e2e only) | 471 verified, 729 unsolved, 0 wrong | n/a |
| `wrong_sign_no_relcheck` | wrong sign on pair relations and `relation_holds` removed | 3 922 of 3 922 failing; 3 108 "determined" logs wrong | 453 verified, 747 unsolved, **0 wrong** | **1 198 of 1 200 wrong** |
| `wrong_sign_relcheck_on` | same, relation check kept, round guard 3 000 | no relation accepted | `ic_multi_solve_ex` returns `None` (4 of 4 curves fail closed) | n/a |
| `no_relcheck_only` | `relation_holds` removed | 0 failing (redundant on valid candidates) | all verified | n/a |

Totals: 25 200 targets under guarded mutants and the control, **0 wrong**, 22 171 verified, 3 029 unsolved, 4 curve-runs failed closed; 14 400 targets with the final guard removed (6 variants incl. the control), 5 555 wrong logs, all caught by the Python checker.

- **3d conclusions.** (i) The only assignment to `res[t]` in the batched branch is behind `curve.mul(Some(g), k) == Some(qs[t])` (code read lines 1963 to 1965; removing it produces wrong logs that the checker flags). A wrong relation can corrupt the factor-base logs, and `solve_fb_logs` then reports wrong logs as determined (up to 3 112 of 3 113), so the "determined" flag is not a correctness signal; the final guard is the only protection, and it holds. (ii) The `a` bookkeeping of restarted lanes, of lanes after an infinity point and of ordinary advance is correct: the relation certificate fails for every one-step mutant that is reachable and passes for the committed code (37 infinity restarts, 3 600 productive restarts on tiny curves). (iii) Dependence and rank deficiency (duplicates, half the relations) reduce the determined set and leave targets unsolved, never wrong. (iv) The `R = G` doubling branch of the setup collector was not hit (the exceptional count equals the infinity count), and the per-target phase's own infinity/exceptional paths were validated for the previous arm and not re-tested here. The 25-bit `inf_*` mutants are equivalent at that size.

## 4. Cost accounting and controls

- **4a validated.** `ic_multi_solve_ex` starts its clock before `FastFactorBase::build`; the timed region covers factor-base build, `PairTable::build` (including the sort), `collect_setup_batched`, `solve_fb_logs`, the per-target phase and the per-target `curve.mul` verification; the example's `total_ms` is the wall time around that call (drop of the pair table included). Not in any arm's timer, identically: curve construction (`find_a3_curve`, point counting), planted-target generation, and the rho and `ic_solve` reference loops that run before each cell. The BSGS timer covers baby steps, giant steps, sign resolution and two scalar multiplications per hit. Verified in the same way for the new setup collector: the 256-lane `fresh` starts (`curve.mul`) and all `relation_holds` re-additions are inside `col_ms`.
- **4b qualified.** The tuned BSGS keeps stepping found lanes; a compacted variant (found lanes dropped; same table, batching and per-hit verification; all rows `ok == T`; `src/ex_bsgs_compact.rs`, taken from the previous validator and diffed) is **1.27x faster at T = 1000 and 1.18x at T = 10 000** (pooled; intervals 1.25 to 1.34 and 1.15 to 1.23). Every IC/BSGS ratio in RESULTS is understated by that factor (stated in RESULTS Limits).
- **Memory (peak RSS, MiB; one process per arm, IC via the dump driver without the rho loop).** 25 bits: T = 1000 new c1 9.4, c2 12.4, BSGS 9.4; T = 10 000 14.0, 15.2, 19.4. 31 bits: T = 1000 41.2, 155.5, 34.3; T = 10 000 44.0, 158.5, 131.3. The chosen configurations (c1 at T = 1000, c2 at T = 10 000) use 0.8x to 1.2x the BSGS footprint (1.2x at 31 bits); memory does not change any conclusion. RESULTS lists memory as unpriced.
- **4c not met (disclosed).** No Pollard rho arm (the `rho_*`, `s3_*` columns in the rows come from the unfixed 1024-entry-table rho the earlier validator said not to cite; RESULTS does not cite them, but they remain unlabelled), no external fastest implementation, no section 5a pricing band (0, 0.1, 1 GAE per unpriced native operation; IC and BSGS both have unpriced hashing/table work), no ecbench session and no L2/A-A admission. The headline is a wall-clock ratio in one process; AGENTS.md section 6 says wall time is a practicality note, not the metric. "Behind the tuned BSGS" is therefore a wall-clock statement on one host class at 25 to 31 bits, not a section 5a ranking. The A/A noise above is the only null I could add. The m = 83 and ECC2K-130 disclosure rules do not apply (prime-field toy curves, no transfer claim).

## 5. Choice of c per T

- The frozen protocol does not justify it. `frozen.json` lists `new_c2` as an arm without a prediction, states the prediction for c = 1 only, and has no rule that picks c by T. The T = 10 000 prediction for c = 1 failed (1.47; reproduces as 1.38). Selecting c = 2 for T = 10 000 uses the same evaluation rows that produced the failure: a best-of-two choice made after seeing the result. The T = 1000 choice (c = 1) was frozen.
- The direction is robust. c1/c2 is 0.74 [0.64, 0.83] at T = 1000 and 1.46 [1.32, 1.55] at T = 10 000, below 1 in 10 of 10 rows at T = 1000 and above 1 in 10 of 10 rows at T = 10 000 (pooled); the previous arm shows the T = 10 000 effect (previous c1/c2 = 1.50) and none at T = 1000 (0.97). The selection effect is small against that spread. What is post hoc is the packaging: "c = 2 at T = 10 000 is 0.94 of the previous arm and 3.8x behind BSGS" is the better of two arms against a reference from another run, and neither number reproduces (0.98; 4.3 to 4.5).
- "c = 1 for T <= 1000, c = 2 for T >= 10 000" (session log, Decisions) extrapolates from two T values and two c values. **Exploratory probe, not frozen, 29 bits, 2 instances x 2 residues, per-row minimum of 2 repetitions, idle host, ratio to the compacted BSGS:**

| T | c = 1.0 | c = 1.5 | c = 2.0 | c = 3.0 | best c |
|---|---|---|---|---|---|
| 1000 | 5.89 | 6.80 | 8.54 | 14.27 | 1.0 |
| 2000 | 6.17 | 5.81 | 7.11 | 10.32 | 1.5 |
| 4000 | 6.19 | 5.17 | 5.67 | 6.69 | 1.5 |
| 10 000 | 7.14 | 6.04 | 5.49 | 5.17 | 3.0 |
| 30 000 | 11.73 | 9.51 | 7.43 | 5.43 | 3.0 |

The best c rises with T, c = 2 is not best at T = 10 000 (c = 3 is 6% lower, within noise) and loses by 1.37x to c = 3 at T = 30 000; the grid stops at c = 3, so the optimum may be higher. With c retuned per T the ratio to the compacted BSGS stays at 5.2 to 5.9 over T = 1000 to 30 000 (four rows per cell). This is one size and four rows per cell; it is a probe, not a result. It does contradict a reading of "the per-target cost is a floor in T" (CAMPAIGN_SUMMARY): the per-target cost falls as c rises, at the price of setup.

## 6. Wording review

Supported (recomputed): the RESULTS table, the size breakdown, "20 rows per arm, 60 arm-runs, all targets verified, no failed run", the T = 1000 predictions and the abandon rule, the failed T = 10 000 prediction for c = 1, "the smaller factor base makes the per-target phase slower" (previous c1/c2 = 1.50 at T = 10 000 with unbatched setup), "still grows with size" at T = 1000, the Limits list.

**Required corrections** (RESULTS = `RESULTS_prime_fast_multi_setup.md`, CS = `CAMPAIGN_SUMMARY.md`):

- **C1 (not validated).** RESULTS Prediction check and Finding: "with c = 2 the new setup is a small gain (0.94)"; CS A2 row "0.94 at T = 10 000 (c = 2)". Say: no measurable change (0.98, 95% interval 0.95 to 1.04, per-pass medians 0.95 to 1.03; committed 0.944 with 3 of 10 rows above 1).
- **C2 (not validated).** RESULTS "none at 25 bits" and "0.98 at 25 bits": two outlier rows; reruns give 0.76 (rows 0.73 to 0.80).
- **C3 (qualified, T = 10 000 not validated).** RESULTS "distance ... about 3.7x to 3.8x at the median for both T, down from 6.9x (T = 1000) and 4.9x (T = 10 000)", and CS "3.7x to 3.8x behind at the median (T = 1000 / 10 000)". T = 1000: 3.8 to 4.3, down from 7.6. T = 10 000: 4.3 to 4.5, not 3.8; same-session previous arm 4.48, so no reduction; the reported fall comes from the new run's BSGS rows being 1.19x slower than the earlier run's. Against a compacted BSGS the distances are 5.0 and 5.5.
- **C4 (not validated).** CS A2: "2.9x to 9.2x over 25 to 31 bits" for "the best multi-target IC variant": 9.2 is c = 1 at T = 10 000 and 31 bits, a configuration the same table rejects. For the chosen c per T the committed by-size medians are 2.9, 3.8, 5.0 (T = 1000) and 2.6, 4.1, 5.9 (T = 10 000), i.e. 2.6x to 5.9x.
- **C5 (qualified).** RESULTS "best factor-base constant depends on T: c = 1 ... c = 2": say "better of the two tested values", mark the T = 10 000 choice as made after the failed prediction on the evaluation rows (post hoc; `new_c2` had no prediction), and drop or label as extrapolation the session-log rule "T >= 10 000: c = 2" (exploratory probe: c = 3 at T = 30 000).
- **C6 (qualified).** RESULTS "before, collection 52 to 81 ms ... pair table 14 to 65 ms and elimination 2 to 14 ms": cite where the "before" run is stored or commit it; `explore_27_timing.txt` (unlabelled, no header) holds the "after" run only. Re-measured here: before 62 to 73 / 47 to 59 ms, after 21 to 32 / 22 to 29 ms (c = 1 / c = 2).
- **C7 (qualified).** RESULTS Limits and CS "not independently replayed / not yet replayed": replayed here with the qualifications above; CS "the per-target relation search costs about p^(1/3) additions (a floor in T)" is a fixed-c model statement; label it so (probe above).
- **C8 (qualified).** Classification missing (AGENTS.md section 3): engineering change of the project's own multi-target IC, not an advance over BSGS or rho. The headline is wall-clock; the table has no phase columns although `setup_ms` is in the rows (setup is 44% to 73% of the total at T = 1000, 6% to 24% at T = 10 000 for the new arm).
- **C9 (qualified).** Provenance: add command lines, host/CPU, rustc, pinning, dirty-tree state; record the setup mode in the rows (`batched` is ambiguous across `pf_batched` and `pf_setup`; `frozen.json` names a `batched_setup` field that does not exist); note that the previous arm is not reachable at HEAD; commit a certificate file (relation and log checks) instead of `ok` counts; label the `rho_*` and `s3_*` row columns.

## What was not checked

An independent-host replay; a full `cargo build` of the crate; the producer's exact earlier (previous-arm) binary, which was overwritten in `target/`; multi-thread behaviour; section 5a pricing; a rho arm; ecbench; other sizes; the per-target phase's infinity and `R = G` paths for this arm; BSGS-found values dumped separately; whether the freeze was "checked" on the remote; uncommitted producer runs before the freeze; B x 0.7 for the compacted BSGS (the previous validator measured up to 1.32x).

## Reproduction (scratch, not committed)

Sources: `git show <commit>:<path>` with the four `use` lines redirected (`src/pf_prev.rs` = `c60ee6297f`, `pf_prevt.rs` = `94394a0473`, `pf_new.rs` = `1eba8dd551`). Build: `rustc --edition 2021 -C opt-level=3 -C codegen-units=16 --crate-type rlib --crate-name pf ... lib_<v>.rs`, then the examples (`bin/ex_multi_{prev,prevt,new}`, `bin/ex_bsgs`, `bin/ex_bsgs_compact`) and drivers (`bin/drv_e2e_*`, `bin/drv_setup_*`, `bin/drv_tiny_*`). Runs: `run_pass.sh` x 6 via `run_all_passes.sh`, `run_producer_pass.sh` x 3, `run_phase27.sh`, `run_tprobe.sh`, `rss.py`, `orchestrate.py {cert_e2e,cert_setup,mut_setup,mut_e2e}`, `run_tiny.py`. Analysis: `recompute.py`, `analyze_passes.py`, `analyze_quiet.py`, `summarize_cert.py`, `summarize_mut.py`. Directory: `/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/pfs/`.
