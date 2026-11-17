# Validator report: batched per-target multi-target IC (PR #3874)

Validated artifact set: commit `e5ea7fbf99148356ac8b323b8d186a70c16fc671` (branch `claude/prime-field-factor-base-calculus-wy2own`), freeze commit `82a6b30f9611dbb79dc71abcb2ef2ff3efc0872a`.
Validator: Sonnet 5.5 validator subagent, 2026-11-11, independent of the producer's summary. Status of this report: [pcc:unverified] validator assertion; same filesystem snapshot and host class as the producer, so it is a rebuild-and-replay, not an independent-host audit.

## Disposition

**`validated`, scope-limited, with required corrections to the RESULTS wording** (section "Required corrections"). The measurement reproduces, every returned log passes an independent big-int check, no unverified log can leave `ic_multi_solve_ex` in batched mode, and all costs are inside the timed region. The result stays a toy-size (25 to 31 bit), single-thread, same-process wall-clock comparison of an engineering change. It does **not** satisfy the validation gate for the leaderboard or `benchmarks/README.md` (no artifact-carried certificate, no matched rho arm, no section 5a band, no L2/A-A wall-time admission, no `VALIDATION.json`); the producer already says so.

## Binding of this validation (reproducibility)

- Local HEAD **moved during the session**: it read `e5ea7fbf99` when I started and `c17b8f3ffc` ("Merge origin/main into the PR branch", 03:04:46Z) later, and the PR merged at 03:12:38Z (`c7c4c2874f`), before this report. I therefore bound everything to the named commit: source was read with `git show e5ea7fbf99:<path>`, and I re-checked that all 20 files in `results/pf_batched/`, all 10 in `results/pf_tscale/`, `RESULTS_prime_fast_multi_batched.md`, `src/cryptanalysis/prime_fast.rs`, `examples/prime_fast_multi.rs`, `examples/prime_fast_bsgs_multi.rs`, `Cargo.toml` and `Cargo.lock` are byte-identical at `e5ea7fbf99` and at `c17b8f3ffc` (`git diff --stat` empty for those paths; per-file SHA-256 equal). Working tree clean (`git status --porcelain` empty) at the end.
- `prime_fast.rs` SHA-256 `7c0e11595a6b612f21fc25502cf0e3af356f6e8ef51ee6aaef4f717b1911f1d6` is the same at `82a6b30f96`, `e5ea7fbf99` and in the working tree. `prime_fast_multi.rs` `7f2acb588bcc2a73...`, `prime_fast_bsgs_multi.rs` `d41cf219a67650a7...`. Source did not change between freeze and the end of the evaluation.
- Producer binaries: `target/release/examples/prime_fast_multi` (built 02:54:47) and `prime_fast_bsgs_multi` (built 02:06:40) hash to `d2ec994d3c72c242...` and `984f1584cba097ec...`, matching `provenance.txt`. I did not use them and did not build into the repo `target/` (it holds those bound binaries and the disk had 1.9 GB free).
- My build: `rustc 1.97.0`, `-C opt-level=3 -C codegen-units=16` (Cargo release default; `Cargo.toml` has no `[profile.release]`), against the repo's own third-party rlibs (`rand 0.8.8`, `num-bigint`, `serde_json 1.0.151`). `prime_fast.rs` is byte-identical except the four `use crate::...` lines, redirected to a stub module (`is_prime_u64`, `CurveParams`, `FieldElement`, `Point`); the two examples are byte-identical except the single `use crypto_lib::...` line. The stubbed `is_prime_u64` yields the same curves: n, fb, determined and ok match the committed rows in 1380 of 1380 field comparisons (below). This is not a full `cargo build` of the crate.
- Host: Intel Xeon (Sapphire Rapids, family 6 model 207) 2.1 GHz, 4 vCPU KVM, 16 GiB, kernel 6.18.44, load average 0.08 at start; all timing runs pinned with `taskset -c 2`.
- Provenance gaps (producer side): `provenance.txt` records only the commit and two 16-hex binary-hash prefixes. Not recorded: the command lines (reconstructable from the example `usage:` lines and the file names), host and CPU, rustc version, pinning, load, dirty-tree state. The multi binary was built at 02:54:47 from the working tree, 1 m 41 s before the freeze commit.

Evidence manifest (SHA-256, first 16 hex): `frozen.json` 79d0e1c664863fe1, `provenance.txt` 6c004a26754cbb3f, `eval_batched_25_T1000` 85201a5a88eee3f7, `eval_batched_25_T10000` f96a4aa3724d4f70, `eval_batched_29_T1000` f02380ae6bb9dd4e, `eval_batched_29_T10000` 170a9f0cd0494c5c, `eval_batched_31_T1000` a1cbbfc2881fa985, `eval_batched_31_T10000` b31f1a0dc371b631, `eval_bsgs_25_T1000` 4a1796580fcb1f2b, `eval_bsgs_25_T10000` 63aedf10ec7a91e6, `eval_bsgs_29_T1000` a9afce4709b7bcfc, `eval_bsgs_29_T10000` 9b8f4348851b4877, `eval_bsgs_31_T1000` cdbedba53322f01e, `eval_bsgs_31_T10000` 1630cb1ad4ccdee0, `eval_table_25_T1000` 721a236e017c43d5, `eval_table_25_T10000` 5be04d949cae6eb9, `eval_table_29_T1000` fe19915a9cac5dfb, `eval_table_29_T10000` 82a7cb9cdfe8ce90, `eval_table_31_T1000` 344dea152a0d28b5, `eval_table_31_T10000` dd26149321e0ca5b, RESULTS file 709de379b1ad25f0. Full manifest: `/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/val/manifest.sha256`.

## Claim-by-claim verdicts

| # | Claim (source) | Verdict | Evidence |
|---|---|---|---|
| 1a | Freeze commit precedes every eval row; states prediction and abandon rule | **validated** | commit order below |
| 1b | Freeze was "pushed and checked on the remote before the evaluation" | **not validated** (unverifiable) | below |
| 1c | Prediction and abandon rule unambiguous | **qualified** | statistic not specified |
| 1d | 25/29/31-bit curves disjoint from curves used to choose c or explore the batched arm | **validated** (by construction and by data) | below |
| 1e | "evaluation curves were not used for any tuning of this arm"; session log says "fresh curves" | **qualified** | curves and T=1000 targets re-used from earlier arms |
| 2 | Table in RESULTS recomputes from the rows | **validated**, except one miscount | R1 |
| 2b | Rebuild reproduces counts and ratios | **validated** | 1380/1380 exact, ratios within noise |
| 3a | All 60 arm-runs verified, no failed run | **validated** by replay (the artifact itself carries no certificate) | 148 000 logs, Python |
| 3b | `ic_multi_solve_ex` batched cannot return an unverified log; restart, infinity, b = 0 paths correct | **validated** | 11 mutants |
| 4a | All costs inside the timed region, pair-table build timed | **validated** | code reading |
| 4b | BSGS arm is a fair "tuned" reference | **qualified** | not the fastest; favors IC by 1.07x to 1.31x |
| 4c | Table arm fair | **validated** | same code, same c, same setup |
| 5 | Memory unpriced / one thread: conclusion unchanged | **qualified** (memory measured, threads untested) | below |
| 6a | "reproducible improvement" (1.8x at T = 10 000) | **validated at T = 10 000 only**; **qualified** wording | 4 runs |
| 6b | "about 1.2x at T = 1000 and not reliable there" | **validated** | 4 runs |
| 6c | "narrows the distance ... from about 8x to about 5x" | **qualified** | T = 10 000 only; BSGS understated |
| 6d | "no crossover is in sight" | **qualified** (interpretation, extrapolation) | T up to 1e5 probe |
| 6e | T-scaling statement (33x, 21x, 7.5x, 10.4x) | numbers **validated**; reading **qualified** | below |
| 6f | "one of ten rows above 1.0, 1.09" | **not validated** | two rows |
| 6g | Prediction batched/BSGS 2x to 5x "holds at the median" | **qualified** | borderline |
| 6h | Reporting contract (classification, rho arm, phases, unit) | **partial** | below |

## 1. Freeze and disjointness

Order (git, file mtimes with sub-second resolution, `git log` author dates and the GitHub PR timeline committer dates agree):

| time (UTC) | item |
|---|---|
| 02:46:53 | `c191d8da36` T-scaling probe rows (table and BSGS at 27 bits) |
| 02:48:16 | `c60ee6297f` batched code in `prime_fast.rs` and `prime_fast_multi.rs` |
| 02:54:47 | multi binary built; 02:54:55 and 02:54:58 `ic_batched_27_T1000/T10000.jsonl` written (exploration, 27 bits) |
| 02:56:27.6 | `frozen.json` mtime |
| 02:56:28 | **freeze commit `82a6b30f96`** (adds `frozen.json` and the two 27-bit batched probe files only) |
| 02:56:32.9 | `provenance.txt` |
| 02:56:33.4 | first eval row (`eval_batched_25_T1000`) |
| 02:57:48.9 | last eval row (`eval_bsgs_31_T10000`) |

- The eval rows enter git only in `db19cb7546` (02:56:41) and `e672bd3806` (02:57:54), both after the freeze. **Validated.**
- `frozen.json` states the prediction ("batched/table total time ratio <= 0.7 at T = 10000 ...; batched/BSGS ratio between 2x and 5x at T = 10000") and the abandon rule ("batched/table median >= 1.0 at T = 10000"). **Validated.** The abandon rule is not triggered (all four runs: median 0.55 to 0.61).
- **Qualified (1c):** the freeze does not say whether the prediction is a median of per-row ratios or a ratio of totals, and does not pre-state a T = 1000 prediction. RESULTS uses the median of per-row ratios; the ratio of totals gives 0.58 (batched/table, inside) and **5.29** (batched/BSGS, outside 2x to 5x).
- **Not validated (1b):** "pushed and checked on the remote before the evaluation". The interval from freeze commit to first row is 5 s. The PR timeline shows the commit's committer date (02:56:28Z), not a push time, and the repository events API returned no push events for the branch. The PR timeline records one force-push at 02:46:59Z, before the freeze; the session log says it dropped two commits, which were restored by cherry-pick at 02:57:24 and 02:57:30 (between the freeze and the end of the evaluation). `82a6b30f96` is in the PR commit list under the same sha. Commit order proves only that the freeze was committed before the rows were written, not that it was published.
- **Disjointness (1d), validated.** `find_a3_curve(bits, res)` is deterministic: p is the largest prime below 2^bits in the residue class. Curves at different bit lengths are therefore distinct. c = 2.0 was chosen on `pf_multi2/explore_table_26.jsonl` (bits {26}, c {1,2,3}) and `explore_table_30.jsonl` (bits {30}); the tune rows `pf_table/tune_rows.json` are 20 and 22 bits; the 24/28-bit probes and the 27-bit `pf_tscale` rows are not 25, 29 or 31. No committed pre-freeze batched row exists at 25, 29 or 31 bits.
- **Qualified (1e).** (i) The 25- and 29-bit curves, with the same instance seeds and, at T = 1000, the **same planted targets** (seed `0xC0DE ^ ins ^ n ^ T`), were already evaluated for the table arm in `pf_multi2` (I confirmed n, inst and `determined` are identical for the table arm at 25 and 29 bits), and for tuned BSGS in `pf_bsgs_tuned/tuned_25,29`; 31-bit curves were used in `pf_multi/eval_31` and `pf_bsgs/bsgs_31`. "Fresh curves" (session log) is therefore wrong; "not used to choose c or explore the batched arm" is right. (ii) The tuned BSGS mode was written after seeing the untuned arm's results on those curves; no parameter is curve-specific (1024 lanes, multiplicative hash), but "tuned BSGS not frozen with the same rigor" (RESULTS, Limits) is accurate and stays. (iii) Uncommitted smoke runs of the batched arm between 02:48 and 02:54 cannot be excluded; none is recorded.

## 2. Rebuild and rerun

Reruns: three full passes of all three arms at 25 (2 instances), 29 (2), 31 (1) bits and T in {1000, 10 000} (the whole eval grid, a superset of the requested sample): protocol order (batched, table, BSGS) twice, reversed order once; `taskset -c 2`. Output: `/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/pfv/rr_fwd1`, `rr_rev1`, `rr_fwd2`; comparison script `.../scratchpad/val/cmp_rerun.py`.

**Deterministic fields:** n, inst, res, T, ok, fb, determined, c (IC arms) and B, rounds (BSGS arm) equal the committed rows in **1380 of 1380** comparisons across the three reruns.

Ratios (median of per-row ratios; committed, then my three reruns):

| quantity | committed | fwd1 | rev1 | fwd2 |
|---|---|---|---|---|
| T = 10 000 batched/table | 0.550 (0.446 to 0.630) | 0.550 (0.457 to 0.731) | 0.614 (0.479 to 0.724) | 0.572 (0.367 to 0.739) |
| T = 10 000 batched/BSGS | 4.91 (2.46 to 6.20) | 4.85 | 4.28 | 5.51 |
| T = 10 000 table/BSGS | 8.30 | 8.46 | 6.92 | 8.52 |
| T = 1000 batched/table | 0.856 (0.728 to 1.092) | 0.794 | 0.802 | 0.834 |
| T = 1000 batched/BSGS | 6.86 (3.62 to 10.26) | 7.56 | 7.70 | 7.64 |
| T = 10 000 batched/BSGS by size (25, 29, 31 b) | 2.70, 5.28, 6.14 | 3.37, 5.11, 6.00 | 3.22, 4.38, 6.29 | 3.06, 5.71, 6.98 |

- The headline reproduces: batched/table at T = 10 000 is 0.55 to 0.61 in all four passes; the growth of batched/BSGS with size reproduces (monotone in every pass, 2.7 to 3.4 at 25 bits, 6.0 to 7.0 at 31 bits).
- **Row-level noise is large:** the same row varies by a median of 1.2x to 1.3x and up to 1.86x between the four passes (max/min of `total_ms`). Within-run differences between arms at T = 1000 are of this size (e.g. identical-code setup, 25-bit res 3: batched 47.0 ms vs table 33.8 ms in the committed run).
- Order effect: reversed order (BSGS first, batched last) gave batched/table 0.614 against 0.55 and 0.57 in forward order. The fixed forward order (batched first) does not explain the gain; if anything it slightly favours the producer's ratio by noise.
- "Every row below 0.7" at T = 10 000 holds in the committed rows (max 0.630) but **not in the reruns** (1 to 2 of 10 rows reach 0.72 to 0.74). It is a property of that run, not a robust one; the median claim is robust.
- T = 1000: rows with batched/table >= 1.0: committed **2** (1.092, 1.030), reruns 1, 0, 1.

## 3. Correctness and certificates

Certificate discipline. The committed rows carry only `ok` (an in-process comparison with the planted k); there is **no certificate file**, and the planted logs are not stored (the producer says so). The run is deterministic (fixed seeds), and the replay reproduces `determined` and `ok` exactly, so I re-derived the certificates by replay:

- Driver `.../scratchpad/pfv/driver.rs` (built against the committed `prime_fast.rs`) generates the same targets as the example, calls `ic_multi_solve_ex`, and dumps p, a, b, n, G, every Q_t, every planted k and every returned log. `.../scratchpad/val/certify.py` is independent big-int Python: checks p and n prime (Miller-Rabin), curve nonsingular, G and Q_t on the curve, n*G = O, n inside the Hasse bound, then **`l_t * G == Q_t` for each returned log** and `k_t * G == Q_t` for each planted target.
- Result: **148 000 returned logs verified, 0 wrong, 0 unsolved, 0 planted mismatches, 0 planted k*G != Q**: batched mode at 25 bits (T = 1000 and 10 000, 2 instances, both residue classes), 29 bits (T = 1000 and 10 000), 31 bits (T = 1000 and 10 000), and table (unbatched) mode at 25 and 31 bits, T = 10 000. A further 190 000 logs on 9- to 14-bit curves (below) also verify.
- Targets are identical across arms (same seed formula in both examples; n, inst, res asserted equal row by row), and since planted Q_t = k_t G is Python-verified, the BSGS arm's `ok` (found k == planted k) is a valid transitive certificate for the BSGS arm. I did not dump BSGS-found values separately.
- The 60 arm-run claim: 3 arms x 20 rows (4+4+2 per T, two T), every row `ok == T`, no `failed` row: **validated** from the rows.

**Mutation tests of the batched branch** (`.../scratchpad/pfv/make_variants.py`; each patch asserted to match exactly once; instrumented copy `instr` adds counters only and gives results identical to the original: same `determined`, same `walked_max`). Run on 25 bits, T = 1000, instance 0, `max_walk` 4000, both residue classes, verified in Python:

| variant | change | result |
|---|---|---|
| `instr` | counters only | 1000/1000 and 1000/1000 verified; restart-on-unusable fired 524 and 498 times; **341 and 329 lanes (34%) finished after at least one restart**; verify failures 0 |
| M1 | restart path forgets the `+1` on `a` (guard on) | 659 and 671 verified, **341 and 329 unsolved**, 0 wrong: the unsolved lanes are exactly the restarted lanes |
| M2 | M1 with the `k G == Q_t` guard removed | **341 and 329 wrong logs returned**, caught by the Python checker |
| M4 | non-restart advance forgets the `+1` (guard on) | 781 verified, 219 unsolved, 0 wrong |
| M5 | k off by one (guard on) | 0 verified, 1000 unsolved, 0 wrong |
| M6 | M5 with the guard removed | 1000 and 1000 wrong logs, caught |
| M7 | accept relations with undetermined FB logs (as 0), guard on | 1000 verified, 0 wrong (524 and 498 verify-failure restarts instead of unusable restarts) |
| M8 | M7 with the guard removed | 345 and 333 wrong logs, caught |
| M10 | b forced to 0 mod n, all guards on (T = 200) | 0 verified, **0 wrong**, 200 unsolved (fails closed) |
| M11 | M10 plus the `b % n != 0` check removed | the process **never terminates**: `inv_mod_u64(0, n)` loops forever in release builds (`debug_assert` only). Killed. |

Infinity and exceptional paths (the 25-bit run never reaches them: counters 0). On 9- to 14-bit curves with 3000 to 40 000 targets, the infinity branch fired 10 times in total (T = 40 000 runs) and the R = +-G exceptional addition up to 11 times per curve: all 190 000 logs verify with **0 verify failures** (so the `+1` bump after an infinity restart is right). Mutant M3 (infinity restart without the `+1`, guard on): 11 verify-failure restarts, 0 wrong logs, all lanes still verified; M9 (M3 without the guard): **8 wrong logs**, caught.

Conclusions: (i) the only assignment to `res[t]` in batched mode is behind `curve.mul(Some(g), k) == Some(qs[t])`; removing the guard produces wrong logs that the checker catches, so the guard is the thing that makes the output fail-closed and the checker can detect a failure; (ii) restarted lanes use the right `a` (a third of lanes restart at 25 bits and all finish with verified logs; the mutants that break the bump lose or corrupt exactly those lanes); (iii) the infinity case restarts with the right `a`; (iv) b = 0 is unreachable (`gen_range(1..n)`) and fails closed through the `usable` test, which is load-bearing for termination if b = 0 ever became reachable. Equivalent-mutant note: M7 with the guard on is indistinguishable in output from the original, so the `usable` filter is a performance filter, not a correctness one, while the final guard is the correctness one.

## 4. Cost accounting

- `ic_multi_solve_ex` starts its clock before `FastFactorBase::build`; the timed region covers factor-base build, `PairTable::build` (including the sort), shared relation collection (`collect_relations_table`), `solve_fb_logs`, the per-target phase, and per-target verification (`curve.mul` before a log is accepted). The example's `total_ms` is the wall time around that call. **Validated.** Curve construction (`find_a3_curve`, which counts points), planted-target generation and the rho/S3 reference loops are outside every arm's timer, identically for all arms.
- BSGS arm: timer covers baby-step table, giant steps, sign resolution and per-hit verification (two scalar multiplications per target). **Validated** as the same kind of work.
- Table arm: same setup code, same c = 2.0, same seed, hence identical `determined` and identical relation sets (verified from rows); the difference is only the per-target phase (one field inversion per step vs one per round). **Validated as a fair unbatched baseline.**
- **BSGS fairness, qualified (4b).** The tuned BSGS keeps stepping lanes of targets that are already found (the loop adds `-M G` to all T lanes until the last target is found, about n/M rounds instead of about n/(2M) on average), and its B = sqrt(T n / 4) is optimal for a compacted loop, not for this one. A validator variant (`ex_bsgs_compact.rs`, scratchpad, outside the frozen protocol; found lanes dropped, same table and batching, all rows `ok == T`; median of 3 repetitions, one core):

  | T, bits | tuned arm (committed code) ms | compacted, B x 1.0 | compacted, B x 0.7 |
  |---|---|---|---|
  | 10 000, 25 | 63.8 | 59.7 (1.07x faster) | 56.0 (1.14x) |
  | 10 000, 29 | 216.1 | 181.7 (1.19x) | 167.0 (1.29x) |
  | 10 000, 31 | 426.7 | 353.5 (1.21x) | 331.3 (1.29x) |
  | 1000, 25 | 14.0 | 11.3 (1.24x) | 11.4 (1.23x) |
  | 1000, 29 | 52.4 | 41.0 (1.28x) | 43.6 (1.20x) |
  | 1000, 31 | 110.1 | 83.9 (1.31x) | 83.7 (1.32x) |

  So the tuned BSGS is not the strongest simple BSGS and **all batched/BSGS ratios in RESULTS are understated by 1.07x to 1.32x**. This favors the IC arm; it does not weaken the conclusion that IC is behind, and it makes "narrows the distance" less impressive in absolute terms. batched/table is unaffected.
- Reference completeness: arms present are batched, unbatched table (the unmodified baseline) and tuned BSGS. **No Pollard rho arm** and no external implementation; the `rho_*` columns in the rows are produced by `rho_parallel` with its 1024-entry table that the earlier validator (`pf_multi2/VALIDATOR_REPORT.md`) said must not be cited; RESULTS does not cite them, but they remain unlabelled in the rows. No A/A interval, no null control, no section 5a band (declared by the producer). For the scoped claim (a component engineering gain over the table-mode IC baseline) these are not blockers; for any statement about distance to rho or to the best attack, the missing rho arm is a blocker.

## 5. Memory and the single-thread setting

Measured peak RSS (`wait4` `ru_maxrss`, MiB, one process per arm; IC runs via the driver without the rho reference loop, BSGS via the committed example):

| bits, T | batched IC | table IC | tuned BSGS | IC/BSGS |
|---|---|---|---|---|
| 25, 1000 | 13.0 | 13.0 | 9.3 | 1.4x |
| 25, 10 000 | 39.6 | 39.7 | 19.4 | 2.0x |
| 29, 1000 | 64.1 | 64.0 | 18.4 | 3.5x |
| 29, 10 000 | 72.6 | 72.6 | 67.2 | 1.1x |
| 31, 1000 | 156.1 | 156.1 | 34.3 | 4.5x |
| 31, 10 000 | 164.7 | 164.6 | 131.2 | 1.3x |

Analytic tables: IC pair table fb^2 x 16 B = 6.3, 40.3, 101.6 MiB at 25, 29, 31 bits (416 025, 2 640 625, 6 656 400 entries; 10.6 million at 32 bits as the earlier report corrected); BSGS open table cap x 16 B = 4 to 16, 16 to 64, 32 to 128 MiB at T = 1000 to 10 000.

- RESULTS' "memory not priced (fb^2 against sqrt(T n / 4))" is a disclosed limit; measured, the IC footprint is 1.4x to 4.5x BSGS at T = 1000 and 1.1x to 2.0x at T = 10 000. The BSGS table exceeds the IC pair table at 29 and 31 bits for T = 10 000 (64 vs 40 MiB, 128 vs 102 MiB), so the direction of "memory costs IC" does not hold there.
- **Does it change the conclusion?** No. At 31 bits and T = 10 000, B x 0.7 shrinks the BSGS open table to 64 MiB, below the IC pair table (101.6 MiB). By the cost model B + T n/(2B) that would cost the uncompacted BSGS about 19%; in the measured compacted variant B x 0.7 was in fact no slower than B x 1.0 (section 4). Scaling the committed batched/BSGS medians by those factors leaves IC about 4x to 8x slower at T = 10 000. Memory is not what puts IC behind.
- Single thread: **not tested** (no multi-thread arm exists). Reasoning only, labelled as such: the per-target phase and the giant steps both parallelize over targets; IC's setup (pair-table sort, dense elimination, 18% to 86% of total time depending on size and T, see below) is largely serial, so more threads would be expected to widen, not close, the gap, most at T = 1000. Not a measured result.

## 6. Wording review (RESULTS_prime_fast_multi_batched.md)

Supported numbers (recomputed): batched/table 0.86 (0.73 to 1.09) at T = 1000, 0.55 (0.45 to 0.63) at T = 10 000; batched/BSGS 6.9 (3.6 to 10.3) and 4.9 (2.5 to 6.2); table/BSGS 7.8 and 8.3; by size 2.7, 5.3, 6.1 (T = 10 000) and 4.1, 7.2, 9.6 (T = 1000); 1.8x and about 1.2x (1/0.55, 1/0.856); "about 110 ms setup at 27 bits" (batched rows 104 to 126, median 111 to 115; the table arm 123 to 140); "about 0.04 ms per target at 27 bits against BSGS's 0.012 ms" (0.043 and 0.0125, computed as (total - setup)/T). Setup share of batched total time: 69%, 82%, 86% at T = 1000 and 18%, 32%, 39% at T = 10 000 for 25, 29, 31 bits.

**Required corrections:**

- **R1 (not validated).** "one of ten rows above 1.0, 1.09": two rows are above 1.0 (1.092 and 1.030) at T = 1000.
- **R2 (qualified).** "batched/BSGS prediction (2x to 5x) holds at the median (4.9)": the median is the mean of one row at 4.78 and one at 5.04; five of ten rows are above 5; the ratio of totals is 5.29; the median over my reruns is 4.85, 4.28, 5.51. Report it as borderline, not as met, and say the statistic was not pre-specified.
- **R3 (qualified).** "every row below 0.7" (batched/table, T = 10 000) is not reproduced by reruns (rows up to 0.74); state the median and the noise (row-level 1.2x to 1.3x, up to 1.9x).
- **R4 (qualified).** "narrows the distance to the tuned BSGS from about 8x to about 5x at the median": true at T = 10 000 (8.3 to 4.9) only; at T = 1000 it is 7.8 to 6.9. The BSGS reference is 1.07x to 1.32x slower than a compacted BSGS, so the distances are understated.
- **R5 (qualified).** "reproducible improvement": supported at T = 10 000 (0.55 to 0.61 in four passes of the same grid, same host class, rebuilt from source, all rows below 1.0 in all four passes), and the file already hedges T = 1000. It is an **engineering** improvement of the table-mode IC (one shared field inversion per round replaces one per step; the gain does not touch the relation yield or the floor); RESULTS does not classify the change as AGENTS.md section 3 requires, and should say so. It is a component improvement over the producer's own table-mode IC, not over BSGS or rho. Provenance is bound for a replay by this validator on a rebuilt copy, not by an independent host; the file's own status line says "not independently replayed" and should be updated to point to this report.
- **R6 (qualified).** "no crossover is in sight": an interpretation. Data: batched/BSGS grows with size at fixed T (25 to 31 bits) and, in a validator probe at T = 100 000 (unfrozen, one instance per curve, 6 verified rows), is 3.5 to 4.3 at 25 bits, 6.8 to 7.0 at 27 bits and 8.4 to 9.5 at 29 bits, above the T = 10 000 value at each size, so larger T does not help either. This follows from IC total time growing linearly in T while BSGS grows as sqrt(T). It does not cover other sizes, a different c per size, or other IC levers. The proposed cause of the faster-than-n^(1/3) growth of the per-target cost with size (the pair table leaving cache) is untested; label it a hypothesis if mentioned.
- **R7 (T-scaling statement).** Numbers validated from `pf_tscale`: table-IC/BSGS = 33.2, 20.7, 7.5, 10.4 at T = 10, 100, 1000, 10 000 (four rows each at 27 bits, per-row ranges 27.5 to 39.7, 18.1 to 26.2, 7.1 to 8.2, 9.0 to 11.1; the ranges at T = 1000 and T = 10 000 do not overlap). Qualified reading: (i) one size, one mode (unbatched), four T values on a factor-10 grid, so "the minimum is near T = 1000" is only a bracket between 100 and 10 000; (ii) the same probe for the batched arm gives 5.5 at T = 1000 and 4.3 at T = 10 000 (27 bits, 4 rows each, written before the freeze), so the batched minimum is not at T = 1000, and my T = 100 000 probe shows it rises again beyond about 10^4; (iii) the shape is what a model S + c_IC T versus c_B sqrt(T n) predicts, the prediction "smallest at small T" was falsified by the setup cost, which the file says.
- **R8.** "Evaluated on fresh 25/29/31-bit curves" (session log `docs/session-logs/2026-11-11.md`) should read "not used to choose c or explore the batched arm; reused from earlier table and BSGS evaluations".
- **R9.** In batched mode `per_target_median_ms` is `(total - setup)/T` for every target (a mean, not a median); the field is mislabelled. RESULTS does not use it.
- **R10.** Report phases per row in the table (setup_ms is in the rows): setup is 69% to 86% of the T = 1000 total, which is what makes the T = 1000 gain unreliable.
- **R11.** Provenance: add the command lines, host and CPU, rustc version, pinning, dirty-tree state and the build time of the binary; label or drop the `rho_*` columns.
- **R12.** Unit: wall-clock milliseconds only; no S = operations / sqrt(n) conversion, no matched rho arm, no section 5a band, no ecbench L2/A-A admission. The file's status line and Limits already say this; keep them. m = 83 and ECC2K-130 disclosure rules (AGENTS.md 8a, 8b) do not apply: prime-field toy curves, no transfer claim.

## What was not checked

- An independent-host replay (this is the same snapshot and host class), a full `cargo build` of the crate, the producer's exact binaries (not executed), multi-thread behaviour, section 5a pricing, a rho arm, other c values per size, sizes outside 25 to 31 bits for the frozen protocol.
- Whether the freeze was on the remote before the evaluation (no push times retrievable).
- Uncommitted exploratory runs by the producer before the freeze.

## Reproduction commands (scratchpad, not committed)

Build: `rustc --edition 2021 -C opt-level=3 -C codegen-units=16 --crate-type rlib --crate-name pf ... lib_orig.rs` then the two examples (all under `/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/pfv/`). Rerun: `run_rerun.sh <outdir> fwd|rev`. Certificates: `driver.rs` plus `../val/certify.py`. Mutants: `make_variants.py`. Memory: `rss.py`. BSGS probe: `ex_bsgs_compact.rs`. Recompute: `../val/recompute.py`, `../val/cmp_rerun.py`, `../val/tscale.py`.
