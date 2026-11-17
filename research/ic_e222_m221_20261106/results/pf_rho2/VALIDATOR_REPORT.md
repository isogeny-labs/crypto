# Validator report: IC vs multi-target DP rho vs tuned BSGS (`pf_rho2`, extension to 33 and 35 bits)

Status: [pcc:unverified] validator measurement; not registered as a pcc claim; no `VALIDATION.json` is created by this report.

- Validated commit: `eabeeab918855057de663f21b3597ea264749787` (PR isogeny-labs/crypto#3897, branch `claude/prime-field-factor-base-calculus-wy2own`). `git status` was clean before and after the validation, and the PR head was still `eabeeab918` at the end. Nothing under `/home/user/crypto` was modified except this file.
- Frozen artifact set: `RESULTS_prime_fast_multi_rho.md`, `results/pf_rho2/{frozen.json,provenance.txt,PREDICTION_33_35.txt,eval_*.jsonl,ext_rho_*.jsonl}`, `results/pf_rho/{PREDICTION.txt,rho_27_T1000.jsonl,rho2_27_T1000.jsonl}`, `examples/prime_fast_{rho_multi,multi,bsgs_multi}.rs`, `results/pf_size/{ic_c1_33,ic_c1_35,bsgs_33,bsgs_35}.jsonl`.
- Host: 4 vCPU Intel Xeon @ 2.10 GHz, 16 GB, Linux 6.18.44-fc, rustc/cargo 1.97.0, 2026-11-11 07:2x to 07:53 UTC. The same sandbox VM as the producer's working tree (its binaries and file mtimes are on this filesystem), so not an independent host class; no L2 isolation; not run under `ecbench`.
- Validator scratch (outside the repo, not committed): `/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/val_rho_20261111/` (scripts, rebuilt-binary outputs, `out/runlog.jsonl` with the load average per run, sweeps, mutants, 64 000 dumped logs). SHA-256 prefixes (16 hex) of the decisive files are listed at the end.

## Disposition

**`validated` for the measurement; interpretation qualified.** All 80 committed arm-runs and the 33/35-bit extension are arithmetically reproduced; every returned discrete log that I sampled (52 000 rho logs, 12 000 IC/BSGS logs) passes an independent big-integer check; an independent four-repetition rerun (three from rebuilt sources, one from the hash-verified producer binaries, arm orders different from the committed one) reproduces the direction and, for 25/29/33/35 bits, the magnitude of the ratios. No failed certificate.

The headline reading "IC faster than a multi-target rho up to about 29 bits, slower from 31 bits" is **qualified**, and the 31-bit, T = 10 000 sign is **not validated**: the rho arm spends 36% to 56% of its time (25 to 31 bits, T = 1000) in an un-batched restart step. Simple tuning of the same algorithm makes it 1.9x to 4.2x faster at 25 to 31 bits (T = 1000), and the IC margin at 29 bits vanishes (IC / tuned rho = 1.13 to 1.21). "No sustained improvement over a rho-type attack" is **validated**, and is strengthened, not weakened, by the tuning.

Counts: requirements R1 to R10: 6 PASS (R1 to R5, R7), 2 PARTIAL (R6, R8), 1 FAIL (R9), 1 NOT MET (R10). Claims C1 to C11: 6 validated (C2, C3, C7, C9, C10, C11), 4 qualified (C1, C4, C5, C8), 1 split (C6: qualified at T = 1000, not validated at T = 10 000).

## 1. Requirement checklist

| # | Requirement | Result | Evidence |
|---|---|---|---|
| R1 | Frozen committed set, sha recorded, no moving tree | PASS | `git rev-parse HEAD` = `eabeeab918...`, `git status --short` empty at start and end; `git diff 4a20ce1da4 eabeeab918 -- examples src Cargo.toml Cargo.lock` is empty (sources unchanged since before the freeze). |
| R2 | Freeze commit precedes the eval rows and states the prediction and abandon rule | PASS (with a note) | `4a20ce1da4` contains only `frozen.json` (with `prediction`, `abandon_if`, `order`, `basis`) and `pf_rho/rho2_27_T1000.jsonl`; every `eval_*` row first appears in `161f820323`, `68355f1923`, `7613f8f25f` (children, in DAG order). GitHub marks `4a20ce1da4` "verified". Note: the DAG proves commit order, not run order. File mtimes recorded in the working tree give `frozen.json` 07:17:47.8, commit 07:17:49, `provenance.txt` 07:17:52.7, first eval row 07:17:53.1: the first row exists about 4 s after the commit. "Pushed and checked on the remote before the evaluation" (RESULTS line 7) cannot be confirmed from the artifacts (no push-event timestamps available through the API for this branch). |
| R3 | Prediction text unedited after the fact | PASS | `git log -p` on `pf_rho/PREDICTION.txt`: the original two lines are intact; the result and the second prediction are appended in `9fe19b5742`. `PREDICTION_33_35.txt` committed (`76ce991534`, 07:20:05) before `ext_rho_*.jsonl` (`464fbe6a2c`); mtimes 07:20:03.97 / 07:20:09.2. |
| R4 | Recompute tables, ratios, ranks, counts | PASS | `scratchpad/.../recompute.py`: 20 rows per arm, same key set in all four arms, same `n` per row, `ok == T` for IC/BSGS and `ok == resolved == T` for every rho row. Medians reproduce exactly: T = 1000: best IC / rho 0.5295 (0.19 to 1.48), rho / BSGS 3.383, best IC / BSGS 2.023; T = 10 000: 0.5848, 2.693, 1.940; per-size values as in RESULTS lines 17 to 18; 33/35-bit table (IC / rho 1.91, 1.96, 2.65, 2.33; rho / BSGS 1.69, 1.65, 1.52, 1.71) reproduces. |
| R5 | Boundary, table, ratio: one table, one unit, ratios | PASS | Wall-clock ms for all arms, same instances, ratios given. |
| R6 | Reproducibility bindings | PARTIAL | Present: commit sha; SHA-256 prefixes of the three binaries in `provenance.txt` (I verified: `multi 08a0895838559fc3`, `bsgs 984f1584cba097ec`, `rho ce9262419febe3cc` match `target/release/examples/*`, and the binaries pre-date the freeze); seeds are fixed in code. Missing: command lines, dirty-tree state, rustc/CPU/kernel/load, hashes of the row files, any A/A run. |
| R7 | Certificate discipline (independent re-verification) | PASS (I produced the certificates) | The producer's rows carry counts only (`ok`, `resolved`); no certificate file and no stored logs. I re-ran the arms with a log dump and verified in separate code (section 3). |
| R8 | Controls: null, reference arm, cheaper baseline, frozen benchmark, external fastest | PARTIAL | Tuned BSGS is present and priced in the same unit (the strongest arm: per row, over the committed and all validator rows, the IC is 1.35x to 5.0x behind it and the committed rho 1.1x to 12.7x). No A/A null (I measured it: per-arm per-row CV 11% to 15%, max/min up to 1.7x). No external or tuned rho: the only rho is the in-house untuned arm (section 5). |
| R9 | Reporting contract: phases separated, all arms priced (AGENTS.md section 5a), ecbench (section 12) | FAIL / not applicable as declared | No section 5a band (0, 0.1, 1 GAE per native op) for any arm, rho references included; no `ecbench` spec/session/replay; wall-clock only on a shared VM. RESULTS declares these. Rankings are therefore wall-clock observations, not section 5a rankings. m = 83 / ECC2K-130 disclosure rules: not applicable (prime-field toy curves, no such claim). |
| R10 | Gates (`VALIDATION.json`, replay certificate, matched-rho band) | NOT MET | Declared absent by the producer. |

## 2. Correctness of the rho arm (`examples/prime_fast_rho_multi.rs`)

### 2.1 Re-derivation (read against the code, line numbers of the committed file)

- Walk: `P = a G + Q_t` for `t = 0..T`, with `Q_0 = G`, `klog[0] = 1` (lines 143 to 148, 194). The step table holds multiples of `G` only, so the invariant `P = aG + Q_t` holds under steps, `off` tracking `a`.
- Collision relation (lines 221 to 240): DP key is the x-coordinate; `y0 == p.y` means same point, otherwise `P0 = -P` (valid: n is an odd prime, no 2-torsion, `find_a3_curve` requires prime order). Same point: `a0 + k0 = a + k`, so `k0 = k + (a - a0)`, code `relate(t0, t, +1, aoff - a0)`. Negated: `a0 + k0 = -(a + k)`, so `k0 = -k - (a + a0)`, code `relate(t0, t, -1, -(aoff + a0))`. Both match.
- Offset bookkeeping: `off` is advanced by `s` before the DP test (line 220), so the offset of the current point is `aoff = off - s`; the code undoes it (lines 228, 238) in both the lookup and the insert. The guard `a0 != off[l]` (line 226) compares the stored `aoff` with the already advanced `off`, so it almost never skips; harmless because the trivial same-trail case is rejected inside `relate` (`lhs == 0`).
- Signed union-find (lines 15 to 127): `find` composes `(alpha, beta)` correctly under path compression (`k_v = a_v (acc_a k_root + acc_b) + b_v`). `relate`: different roots, `k_rx = a_x (eps a_y k_ry + eps b_y + d - b_x)`, code `a_new = ax*eps*ay`, `b_new = ax*c`; known propagation `k_ry = a_new (k_rx - b_new)`; same root: `(a_x - eps a_y) r = eps b_y + d - b_x`, `lhs in {0, +-2}`, `r = rhs/2` negated for `lhs = -2` (n odd). All agree with my derivation.
- Restart (lines 244 to 254): new `t`, `off = sa + sb`, start `Q_t + A_i + B_j`; consistent with `P = aG + Q_t`.
- Final check (lines 272 to 287): `ok` increments only if `c.mul(g, k) == Q_t` and `k == klog[t]` (`k < n`, `klog in [2, n)`), so a wrong log cannot be counted as ok; `resolved` counts roots with `known`, so `ok < resolved` exposes wrong logs and `resolved < T` exposes the 400 x sqrt(nT) cap. The solver never reads `ks`/`klog` apart from this check.

### 2.2 Mutation tests (committed source, one change per mutant; 25 bits, T = 200, instance 0, both residues; `out/mut/summary_T200_25.txt`)

Unmutated: `ok = resolved = 200` in the 2 runs of this table and in 12 of 12 runs over 6 instances x 2 residues.

| mutant | ok / 200 (res 3, res 1) | detected |
|---|---|---|
| m01 sign decision flipped (`y0 != p.y`) | 0, 0 | yes |
| m02a / m02b / m02c undo of the pre-DP offset dropped (lookup / insert / both) | 1, 0 / 1, 0 / 54, 0 | yes |
| m03a `b_new` without the `ax` sign | 99, 0 | yes |
| m03b `a_new` without `ay` | 151, 0 | yes |
| m03c known propagation without the `a_new` sign | 0, 0 | yes |
| m03d `+bx` instead of `-bx` in the attach constant | 1, 0 (12 of 12 runs fail over 6 instances) | yes |
| m03e `eps` dropped on `by` in the attach constant | 190, 0 | yes |
| m04a / m04b / m04c cycle case: no `lhs < 0` negation / `+bx` / no halving | 200, 0 each; over 6 instances x 2 residues 4, 5, 5 of 12 runs fail | yes (see note) |
| m05 anchor value 2 instead of 1 | 0, 200; 7 of 12 runs fail | yes (see note) |
| m06a restart `off = sa` only | 0, 0 | yes |
| m06b restart drops table B | 0, 0 | yes |
| m06c restart does not set `tgt` | resolved 176, 172 (cap hit, 5.8 s), ok 0, 0 | yes |
| m08 `find` compression sign | 0, 0 | yes |
| m09 offset never advanced | 0, 0 | yes |
| m10 returned log perturbed by +1 (verification control) | 0, 0 | yes: all 400 wrong logs rejected |

Note: m04 and m05 survive in some runs because the mutated branch is not reached or not used there (the giant component usually gets its value from the anchor or from an earlier cycle; `known` is only set when empty). In every surviving run `ok = 200`, i.e. all logs are correct, so the verification is not bypassed. Side finding: the anchor `Q_0 = G` is largely redundant, since a negative cycle in the signed graph already determines a component.

### 2.3 Defect found: infinity-point branch (lines 204 to 215)

When a lane's point is the point at infinity (`p + step = O`, or a restart start `= O`), the lane is re-seeded with offset `u`, but `sel[l] = steps[0].1` is added to it by the next batched add without `off[l] += steps[0].0`. The lane then carries an offset that is `steps[0].0` too small until its next DP, which can enter the DP table or a relation with a wrong offset and yield a wrong log.

- Demonstration (committed source plus a counter, `out/inf/rho_cnt.all.jsonl`): 640 toy runs (10 to 16 bits, T = 30 and 100, 40 instances x 2 residues): 370 runs had at least one infinity event; **10 runs ended with `ok < T` while `resolved = T`** (all 10 had events). The one-line fix (`off[l] = addm(off[l], steps[0].0, n)`) gives 0 of 640 failing (`out/inf/rho_cnt_fix.all.jsonl`).
- Effect on the committed rows: expected number of infinity events over the 24 committed rho rows is 0.51 (largest single row 0.079 at 25 bits, T = 10 000); every committed row has `ok = resolved = T`, so no row is affected, and a corrupted run would have been visible in `ok`. This is a latent correctness defect of the arm, not a defect of the reported numbers. The fix does not change `steps`/`dps` of runs without events.

## 3. Independent certificates (`indep.py`, pure Python big ints, no solver code)

`indep.py` (a) regenerates all planted targets from the seed formula `0xC0DE ^ inst ^ n ^ T` with its own ChaCha12 `StdRng` (rand 0.8 `seed_from_u64` via PCG32, `gen_range(2..n)` zone rejection) and asserts equality with the targets the Rust code used, (b) checks `p` prime, `a = -3`, `G` on the curve, `n` prime, `|n - (p+1)| <= 2 sqrt(p)`, `nG = O` (so the curve has prime order n), (c) checks `k_returned * G = Q_t` and `k_returned = k_planted` for every target. Negative controls (one log perturbed by +1, one log missing) are rejected.

| arm | runs verified | logs verified | result |
|---|---|---|---|
| multi-target rho | 25 bits T = 1000 (4 runs), 25 bits T = 10 000 (2), 29 bits (2), 31 bits (2), 31 bits T = 10 000 (2), 33 bits (2), 35 bits (2) | 52 000 | all pass; planted targets identical to the independent regeneration |
| IC (batched, c = 1) | 25, 29, 31 bits x 2 residues, T = 1000 | 6 000 | all pass |
| tuned BSGS | same | 6 000 | all pass |

The dumped rho runs are the committed algorithm with a log dump after the timed region (`rho_dump.rs`, generated by `gen_variants.py` from `git show HEAD:examples/prime_fast_rho_multi.rs`; the only changes are the vector of returned logs and the dump).

## 4. Rebuild and replication

Build: the full `crypto` lib plus its AWS/tokio dependency tree did not fit the free disk (my first attempt filled the volume; I deleted my own partial build directory). I therefore compiled `src/cryptanalysis/prime_fast.rs` **byte-for-byte via `#[path]`** with stubs for the four external items it names (`CurveParams`, `FieldElement`, `Point`, and `is_prime_u64`/`pow_mod`/`mul_mod` copied verbatim minus an instrumentation hook, used only in unmeasured curve setup), plus the three committed examples verbatim (git blob ids equal to HEAD), release profile, rand 0.8.8 from the repo `Cargo.lock`. Cross-check: one additional repetition used the producer's binaries after verifying their SHA-256 against `provenance.txt`. Equivalence: the rho `steps` and `dps` of all 24 rho rows are identical to the committed rows in each of the four repetitions; `fb`, `determined`, `B`, `rounds`, `n` of 240 IC/BSGS rows are identical. This is not a rebuild of the full library crate.

Protocol: all 20 frozen rows x 4 arms plus the 33/35-bit cells (IC c = 1, BSGS tuned, rho; one instance per residue), arm orders different from the committed `ic_c1, ic_c2, bsgs, rho`:
rep1 `rho, bsgs, ic_c2, ic_c1` (cells forward); rep2 `bsgs, ic_c1, rho, ic_c2` (cells reversed); rep3 `ic_c2, rho, ic_c1, bsgs`; rep4 (producer binaries) `ic_c1, rho, ic_c2, bsgs`. Every run logged `/proc/loadavg` and the top three CPU consumers before and after (`out/runlog.jsonl`, 120 runs, all exit 0): the 1-minute load average before a run was 0.24 to 1.06 (median 0.88), i.e. my own single-threaded runner (it was 0.08 when I started, with 1.08 and 1.47 for the 5- and 15-minute averages from the producer's earlier runs); the only process at or above 5% CPU in the pre- and post-run `ps` snapshots was my own `python3` runner (3 snapshots); runnable count 1 to 2 after each run. The host was quiet. Every IC/BSGS row had `ok = T` and every rho row `ok = resolved = T` in every repetition.

Best IC (min of c = 1, 2; c = 1 only at 33/35) over rho, committed vs validator (median of per-row ratios):

| bits | T | rows | committed | validator reps 1 to 4 | per-rep cell medians (rep1, rep2, rep3, rep4) | rho / BSGS committed | rho / BSGS validator |
|---|---|---|---|---|---|---|---|
| 25 | 1000 | 4 | 0.19 | 0.23 (0.19 to 0.31) | 0.22, 0.23, 0.22, 0.23 | 9.95 | 8.77 |
| 29 | 1000 | 4 | 0.61 | 0.53 (0.41 to 0.62) | 0.48, 0.51, 0.59, 0.51 | 3.06 | 4.30 |
| 31 | 1000 | 2 | 1.31 | 1.10 (0.89 to 1.22) | 0.99, 1.17, 1.07, 1.12 | 2.30 | 2.36 |
| 33 | 1000 | 2 | 1.93 | 2.27 (1.85 to 2.65) | 2.29, 2.15, 2.27, 2.31 | 1.67 | 1.63 |
| 35 | 1000 | 2 | 2.49 | 2.95 (2.73 to 3.25) | 2.82, 3.10, 3.20, 2.81 | 1.62 | 1.45 |
| 25 | 10000 | 4 | 0.37 | 0.45 (0.35 to 0.69) | 0.56, 0.44, 0.53, 0.37 | 5.64 | 4.70 |
| 29 | 10000 | 4 | 0.70 | 0.75 (0.58 to 1.02) | 0.87, 0.67, 0.82, 0.69 | 2.45 | 2.84 |
| 31 | 10000 | 2 | 1.10 | 0.92 (0.72 to 1.07) | 0.83, 0.91, 0.86, 1.05 | 2.10 | 2.47 |

Row-level signs at 31 bits (committed, rep1..rep4; + means IC slower): T = 1000 `+-++`/`++++` (two rows; 1.15/0.89/1.22/1.02 and 1.48/1.09/1.12/1.11); T = 10 000 `+---`/`----` (1.28/0.95/0.87/0.82 and 0.92/0.72/0.95/0.90). Run-to-run noise on an unchanged row (five observations: committed plus four reps): median CV 11% (rho), 12.5% (IC c = 1), 15% (IC c = 2), 11% (BSGS); max/min up to 1.48, 1.57, 1.67, 1.68.

Reading: 25 and 29 bits reproduce (IC faster than the committed rho arm). 33 and 35 bits reproduce and the gap is larger than reported (2.3x and 2.9x against 1.9x to 2.0x and 2.3x to 2.7x). 31 bits is a tie within noise: ratio 1.10 at T = 1000, 0.92 at T = 10 000 (the committed 1.31 and 1.10 are at the upper end of the noise band).

## 5. Cost accounting and fairness of the rho arm

Inside the timed region (`t0` at line 150, `ms` after the final verification): the 32 step multiples, both start tables (2 x 4096 scalar multiplications), the 2002 (or 2048) lane starts, the DP hash map, the union-find, the every-64-rounds completeness scan, and the T final `k G = Q` verifications. Outside, identically for all three arms: curve search and generation of the T target points. IC and BSGS verify inside their own timers as well.

Phase breakdown of the committed algorithm (probe with timers only, one run per row, ms): 

| bits, T | setup (tables, lane starts) | scan (hash, DP test, map) | batched add (the walk) | **restart** (2 un-batched adds, 2 inversions per DP) | done-scan | verify | total |
|---|---|---|---|---|---|---|---|
| 25, 1000 | 17 | 27 to 28 | 12 | **73 to 77 (56%)** | 0.0 | 1.7 | 131 to 136 |
| 29, 1000 | 22 | 49 to 50 | 61 | **113 to 116 (46%)** | 0.1 | 2.2 | 247 to 252 |
| 31, 1000 | 23 to 26 | 58 to 67 | 112 to 115 | **110 to 115 (36%)** | 0.1 | 2.6 | 305 to 326 |
| 25, 10 000 | 16 | 66 to 83 | 53 to 60 | **165 to 205 (50%)** | 0.5 | 15 | 315 to 380 |
| 29, 10 000 | 19 | 102 to 163 | 194 to 261 | **184 to 252 (35%)** | 1 to 2 | 17 to 21 | 518 to 719 |
| 31, 10 000 | 19 to 22 | 201 to 243 | 479 to 511 | **250 to 275 (26%)** | 4 to 6 | 19 to 27 | 977 to 1082 |

At 25 bits the actual walk is 9% of the arm's time. The rho time grows by only 13% from 25 to 29 bits while its steps grow 4.3x: the arm is overhead-dominated at the small sizes where the IC "wins". (Steps per ms: 3.0e3 at 25 bits, 11e3 at 29 bits.)

Sensitivity (my own variants of the same algorithm: `rho_probe.rs` = committed code + env knobs + the one-line infinity fix; `rho_neg.rs` = negation-map walk, canonical y, r = 256, 2-cycle escape, trail cap, batched restarts; every probe run verified `ok = resolved = T`). T = 1000, median of 5 repetitions x 2 rows, ms (`out/sweep2.jsonl`, `out/sweep1.jsonl`):

| bits | committed algorithm | + batched restarts, tables 256, dp bits +1 (C1) | + lanes 1024 (C2) | no-negation reference in the new code (N0) | negation map, r = 256 (N1: dp +1; N2: dp +0) | best of the above |
|---|---|---|---|---|---|---|
| 25 | 165 | **39** | 41 | 45 | 64 / 64 | 39 (4.2x) |
| 29 | 249 | 105 | 108 | 126 | **98** / 110 | 98 (2.5x) |
| 31 | 350 | 224 | **186** | 230 | 199 / 230 | 186 (1.9x) |
| 33 | 402 | 416 | 382 | 456 | **322** / 379 | 322 (1.25x) |
| 35 | 847 | 717 | 787 | 725 | **638** / 686 | 638 (1.33x) |

T = 10 000: 25 bits 405 to 187 (2.2x, N2), 29 bits 719 to 483 (1.5x, N2), 31 bits 1207 to 879 (1.37x, N2). One-knob sweeps with batched restarts (3 repetitions, T = 1000): lanes 512 / 1024 / 2002 / 4096 at 25 bits 54 / 62 / 74 / 122 ms, at 29 bits 135 / 111 / 132 / 175, at 31 bits 258 / 204 / 248 / 214; dp offset -1 / 0 / +1 / +2 at 25 bits 107 / 74 / 59 / 54, at 29 bits 167 / 132 / 134 / 126, at 31 bits 246 / 248 / 260 / 202; start tables 256 / 1024 / 4096 at 25 bits 61 / 78 / 74, at 29 bits 124 / 111 / 132, at 31 bits 212 / 210 / 248. Reading: the lane count and DP formula of the committed arm are within about 25% of their optimum (fair), the un-batched restart is the one large inefficiency (1.3x to 2.2x on its own), smaller start tables save 12 to 20 ms, the negation map adds a further 1.0x to 1.4x (no gain at 25 bits, where its per-step overhead dominates; 1.15x to 1.4x at 29 bits and above; step count falls 22% to 26%, short of the ideal 29%).

Best IC against the committed rho and against the tuned variants (IC time = median over the four validator repetitions of min(c = 1, 2); `out/final_tuned_table.txt`):

| bits, T | IC best ms | rho committed ms | best tuned, no negation ms | best tuned or negation ms | IC / committed | IC / tuned (no neg) | IC / best tuned |
|---|---|---|---|---|---|---|---|
| 25, 1000 | 33 | 149 | 39 | 39 | 0.22 | 0.85 | 0.85 |
| 29, 1000 | 119 | 249 | 105 | 98 | 0.48 | 1.13 | 1.21 |
| 31, 1000 | 363 | 325 | 186 | 186 | 1.12 | 1.95 | 1.95 |
| 33, 1000 | 937 | 396 | 382 | 322 | 2.36 | 2.45 | 2.91 |
| 35, 1000 | 2269 | 750 | 717 | 638 | 3.03 | 3.17 | 3.56 |
| 25, 10 000 | 162 | 338 | 193 | 187 | 0.48 | 0.84 | 0.87 |
| 29, 10 000 | 558 | 753 | 573 | 483 | 0.74 | 0.97 | 1.16 |
| 31, 10 000 | 1041 | 1177 | 1002 | 879 | 0.88 | 1.04 | 1.18 |

Caveats on the probes: my code, not frozen, quick (5 repetitions); selecting the minimum over five configurations flatters the tuned rho by up to about 10%; the noise is 11% to 15% per arm-run, so 0.85 at 25 bits is "IC about equal to or slightly faster than a tuned rho", not a clear win; the IC arm was not retuned. Cross-method rankings are wall-clock only (no section 5a band), so none of these orders is a section 5a ranking.

## 6. Claim verdicts

| # | Claim (RESULTS_prime_fast_multi_rho.md) | Verdict | Basis |
|---|---|---|---|
| C1 | Frozen at `4a20ce1da4` with prediction and abandon rule, before the evaluation | **qualified** | Order in the DAG and content verified (R2); the push-before-run statement is not provable from the artifacts (rows appear 3 to 4 s after the commit); the provenance record lacks commands and environment. |
| C2 | Identical planted targets for all arms; every target of every run verified; the rho resolved all T targets in every run; no failed run | **validated** | Same seed expression in all three sources; independent ChaCha12 regeneration equals the Rust targets; `ok = resolved = T` in all 24 rho rows and in every validator rho run (rows of 4 repetitions); 52 000 rho + 12 000 IC/BSGS logs pass the independent check. A latent infinity-branch defect exists (2.3) but did not affect any committed row. |
| C3 | Tables: pooled medians, per-size values, 33/35-bit rows, ratios | **validated** | Exact recomputation (R4). The "(171; 35; 17; 5.1e5)" style figures all reproduce. |
| C4 | Prediction check: holds at 25 and 29 bits, fails at 31 bits; abandon rule not triggered | **validated** (arithmetic) / **qualified** (preregistration) | Arithmetic reproduces. But the second-version prediction ("1x to 5x of the tuned BSGS at 27 bits") is never evaluated: the two `rho2_27_T1000.jsonl` rows are 4.1x and 7.3x of the matching `pf_bsgs_tuned/tuned_27.jsonl` rows (34.6 and 21.6 ms), one outside the stated range; and `pf_rho/PREDICTION.txt` states "about 0.2 s for the tuned BSGS" at 27 bits, which `tuned_27.jsonl` does not support (22 to 35 ms). The first prediction's own weakness rule (rho slower than 3x the BSGS means "the rho reference is weak") is exceeded at the frozen sizes (pooled 3.4x; 9.95x at 25 bits). |
| C5 | IC faster than this rho at 25 and 29 bits, "by 5x down to 1.6x at T = 1000" | **qualified** | Reproduced against the committed arm (25 bits 0.22 to 0.23; 29 bits 0.48 to 0.59). Not robust to simple tuning of the same algorithm: against the tuned rho 0.85 at 25 bits and 1.13 to 1.21 at 29 bits (T = 1000); 0.84 to 0.87 and 0.97 to 1.16 at T = 10 000. The 5x at 25 bits is mostly the rho's un-batched restart (56% of its time). |
| C6 | IC slower from 31 bits (1.3x at T = 1000, 1.1x at T = 10 000) | **qualified at T = 1000, not validated at T = 10 000** | 31 bits is a tie within noise: 1.10 at T = 1000 (per-rep 0.99 to 1.17), 0.92 at T = 10 000 (IC faster in 3 of 4 repetitions). Slower at 33 bits (2.3x) and 35 bits (2.9x) is validated. Against the tuned rho the IC is slower from about 29 bits at T = 1000. |
| C7 | Gap grows with the size | **validated** | Monotone at T = 1000 in every repetition (validator medians 0.23, 0.53, 1.10, 2.27, 2.95 at 25, 29, 31, 33, 35 bits for the committed rho; 0.85, 1.21, 1.95, 2.91, 3.56 for the best tuned one); the 33/35-bit gaps are larger than reported. |
| C8 | Advance shrinking "in line with the exponent argument (IC setup about p^(2/3), rho about sqrt(nT))" | **qualified** | The untuned 25-to-31-bit growth (about 6x, 1.38x per bit) is steeper than n^(1/6) (1.12x per bit) because the rho's fixed overhead amortises; against the tuned rho 29 to 35 bits grows 1.13 to 3.2 (1.19x per bit), closer to the argument. Four to six sizes do not fit exponents. |
| C9 | No sustained improvement over a rho-type multi-target attack | **validated** (scope-limited) | Holds against both the committed and the tuned rho for 29 to 35 bits; stronger with tuning. Scope: in-house rho, 25 to 35 bits, one thread, wall-clock on a shared VM, no section 5a band, no external fastest implementation, no `ecbench` replay. The tuned BSGS beats the IC at every size (per row 1.35x to 5.0x) and the committed rho (per row 1.1x to 12.7x); the "IC beats a rho" reading is against the weakest of the three arms. |
| C10 | "Limits and caveats all favour the rho being faster than measured" (no negation map, no tuning, simple restart) | **validated** and quantified | Section 5: 4.2x (25 bits), 2.5x (29), 1.9x (31), 1.25x (33), 1.33x (35) at T = 1000 for batching restarts, smaller tables, DP offset and a negation map together. |
| C11 | "Not independently replayed; no certificate file, bench record, `VALIDATION.json`" | **validated** (accurate disclosure) | This report is the first independent replay; the gates remain unmet. |

## 7. Recommendations (not done here)

1. Fix the infinity branch (`off[l] = addm(off[l], steps[0].0, n)`), batch the restart additions, and rerun the frozen cells; then report the IC / rho crossing against the tuned arm (expected near 27 to 29 bits at T = 1000 on this harness).
2. Add an A/A pair and the load average per run to the provenance, record commands and tool versions, store the returned logs or a verifier output as the certificate, and run under `ecbench` with the section 5a band before any ranking is cited.
3. Do not cite "IC faster than rho up to 29 bits"; cite "IC faster than the untuned multi-target DP rho at 25 and 29 bits (0.2x to 0.6x), tie at 31 bits, slower by 2.3x and 2.9x at 33 and 35 bits; against a simply tuned rho the small-size margin is gone (0.85 at 25 bits, 1.1 to 1.2 at 29 bits)".
4. Correct `pf_rho/PREDICTION.txt`'s "about 0.2 s for the tuned BSGS" (22 to 35 ms at 27 bits) in a dated note without rewriting the frozen file.

## 8. Operational notes and deviations

- My first dependency build wrote several gigabytes to the scratchpad volume and filled it; I killed the build and deleted my own partial output (`scratchpad/target`). No repo file was affected. I then built in tmpfs (`/dev/shm/val_target`).
- The scratchpad directory is shared with earlier sessions. I overwrote `scratchpad/mini/Cargo.toml` and `scratchpad/mini/src/lib.rs` (and possibly `mini/Cargo.lock`, `mini/examples/prime_fast_*.rs`) belonging to an earlier session before noticing; I cannot restore them. I also moved four older files out of `scratchpad/py/` and put them back with their original mtimes (the empty `py/__pycache__` was removed). All later work is under `scratchpad/val_rho_20261111/`.
- Not done: full-library rebuild; section 5a band; `ecbench`/L2; an external reference rho; T = 10 000 at 33/35 bits; independent certification of the probe variants' logs (they were checked by the solver-side verification only).

## 9. Artifact hashes (SHA-256, first 16 hex)

Validator scripts and outputs under `scratchpad/val_rho_20261111/`: `indep.py d0205bce24765d1b`, `gen_variants.py b636d94fe5e73cbb`, `gen_probe.py b50caed902bd0f54`, `gen_dump_others.py 8b3d6f3be07dc939`, `run_arms.py 54877c75b05be949`, `analyze_rep.py 4ca82c219177f99f`, `sweep.py cc5cf343c428b92f`, `recompute.py da8b1c61653874d2`, `out/runlog.jsonl 6e6bf19a79c7a72f`, `out/sweep1.jsonl 4808f65a24120171`, `out/sweep2.jsonl a335c3cb1aefd3c8`, `out/mut/summary_T200_25.txt 370fdd5bc0418420`, `out/final_tuned_table.txt 19be446c888fb86c`, `out/final_replication_table.md 5a0bfdd1cf3cd461`, `out/inf/rho_cnt.all.jsonl 72f6ded11e22de54`, `out/inf/rho_cnt_fix.all.jsonl 7c86e54d4f9aaa1f`. Rebuilt binaries: `prime_fast_multi 7568399895e309d7`, `prime_fast_bsgs_multi e6bcbf914cd72449`, `prime_fast_rho_multi 1c5e3670506371cc` (differ from the producer's hashes because of build path and harness; behaviour identical as stated in section 4). Producer binaries (verified against `provenance.txt`): `multi 08a0895838559fc3`, `bsgs 984f1584cba097ec`, `rho ce9262419febe3cc`.
