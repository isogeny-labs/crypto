# Validator report: multi-target IC (v1/v2/v3) and counted batch rho

Validator: independent session (Sonnet 5.5), 2026-11-07. Scope: the dispatch for the multi-target
index-calculus and batch-rho evidence in `research/ic_e222_m221_20261106/` and `examples/`.
Nothing in the repository was edited except this file. No `cargo` run touched `target/`.

## 0. Binding

| item | value |
|---|---|
| validated commit | `bf3310aaa3800262c6c3821aac250b44fc99aa2a` (branch `claude/prime-field-factor-base-calculus-wy2own`) |
| working tree | clean before and after (`git status --porcelain` = 0 lines); `target/release/ic` untouched (mtime 2026-11-07 02:38:27) |
| tools | rustc/cargo 1.97.0; Python 3 + numpy 2.5.3 (no sympy) |
| build used by me | separate dir: `cd /tmp/val_build && CARGO_TARGET_DIR=/tmp/val_target cargo build --release --offline --locked --example ic_multi_target --example ic_batch_rho --example ic_multi_target_val --example ic_batch_rho_val` (12 min 12 s). `/tmp/val_build` is a symlink tree of the repo with copies of `src/`, `examples/`, `Cargo.toml`, `Cargo.lock`. The only source differences from HEAD are (a) three global atomic call counters in `src/ecc/point.rs` (`Point::add`, `Point::double`, `Point::scalar_mul`) and (b) two example copies `*_val.rs` that print the planted log, the recovered log, the target point and the factor-base logs to stderr. `ic_multi_target` and `ic_batch_rho` in that tree are unmodified copies of HEAD. |
| my scripts | not in the repo (the dispatch limits writes to this report). They are in `/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/`: `common.py lt.py tables.py indep.py kap.py comp.py sens.py ledger.py ledger2.py replay.py replay2.py cert.py ec.py inst.py bsgs.py bsgs2.py unanch.py fbsens.py fbsens2.py`. The same directory also holds older files from the producer's own work; I did not use them. |
| not done | no session-log entry (write scope); no pcc claim, no `VALIDATION.json` exists or was created |

## 1. Disposition

* **Row-level measurements: validated.** Every stored row replays bit-for-bit, every recovered log verifies with code independent of the solver, and the group-addition counters equal the true number of `Point::add` calls.
* **Headline claim as worded: `insufficient_evidence` (over-stated in IC's favour).**
  * "Within about 10% of a counted batch rho at large T": **supported at price 0 and 0.1** for this specific anchored random-walk rho (IC/rho = 1.027 to 1.072 at 0; 1.012 to 1.028 at 0.1), 24 and 25-bit groups, T/sqrt(n) = 0.7 to 4. It is not a statement about the strongest generic multi-target attack (F2: a sequential-database rho is 7 to 12% cheaper, a measured multi-target BSGS 25 to 32% cheaper, so IC v3 is 1.38 to 1.55 times BSGS at price 0).
  * "Unranked under the pricing band / the order flips / IC wins 22 of 22 at price 1": **not supported.** The flip exists only because IC's own ledger omits about 2 m^2 hash inserts (the pair-sum table), which is 0.9x to 6.7x the per-trial hash operations that are counted. With the code's own complete ledger the order does not flip: IC/rho = 1.03 to 1.12 at every price in every cell, 0 of 22 configurations cheaper (section 2, F1). Under AGENTS.md section 5a an arm that does not emit a counter cannot be ranked, so the artifacts as committed can support neither "rho ahead" nor "unranked".

## 2. Findings (ordered by effect on the claim)

### F1 (major, changes the headline): IC's unpriced ledger omits its table-build hash inserts; "order flips at price 1" is a ledger artifact

`ic_multi_target.rs` lines 93-132 call `table.entry(..).or_insert(..)` up to four times per pair, 2 m^2 times in total (v2/v3 build; `HashMap` stores both signs). Neither the example nor the "symmetric pricing" (`IC unpriced = la_ops + trials_g + trials_targets + 4 m pbits`) counts them, although the same ledger charges rho one hash operation per walk step, and AGENTS.md section 5a names "table inserts and lookups" explicitly. (The analysis scripts that produced the committed `analysis*.txt` are not in the repo; I reproduced their numbers from the formulas in the dispatch.)

Median IC v3 / rho (best kappa per config at each price), 22 paired configs per cell, `results/large_T_holdout/rows.json`:

| ledger | 24 b, T=4096 (0 / 0.1 / 1) | 24 b, T=16384 | 25 b, T=4096 | 25 b, T=16384 |
|---|---|---|---|---|
| A. as committed | 1.057 / 1.023 / 0.847 | 1.027 / 1.012 / 0.923 | 1.047 / 1.021 / 0.887 | 1.072 / 1.028 / 0.788 |
| B. A + IC 2 m^2 table inserts | 1.057 / 1.107 / 1.370 | 1.027 / 1.044 / 1.141 | 1.047 / 1.084 / 1.271 | 1.072 / 1.130 / 1.445 |
| C. B + exact rho hash count | 1.057 / 1.110 / 1.391 | 1.027 / 1.046 / 1.151 | 1.047 / 1.087 / 1.291 | 1.072 / 1.131 / 1.459 |
| D. C + one modular exponent update per step on both arms | 1.057 / 1.068 / 1.113 | 1.027 / 1.028 / 1.030 | 1.047 / 1.055 / 1.084 | 1.072 / 1.083 / 1.124 |
| E. hypothetical x-only IC table (m^2 inserts) + per-step update, both arms | 1.057 / 1.029 / 0.924 | 1.027 / 1.013 / 0.950 | 1.047 / 1.026 / 0.947 | 1.072 / 1.035 / 0.885 |

* A recomputes the committed headline exactly (section 4). B, C, D count the code as written; in B, C, D IC is cheaper in 0 of 22 configurations at every price in every cell.
* E is the only ledger on which the flip survives. It credits IC with an optimization it does not implement (x-only table) and charges both arms a second per-step operation, which favours IC because IC takes about 0.42x as many steps as rho (24 b, T=4096: 168.6k trials vs about 402k rho steps).
* Size of the omission, median IC counted hash ops (trials) vs omitted inserts: 171,661 vs 524,288; 592,481 vs 524,288; 343,027 vs 524,288; 314,506 vs 2,097,152. In the 25-bit, T=16384 cell the omitted inserts equal 101% of IC's counted additions. The RESULTS sentence "the IC performs one [hash op] per trial plus a small amount of other work" is therefore wrong.
* Related minor ledger errors: rho's hash proxy `adds_db + target_steps` includes the 64 bank scalar multiplications and the restart additions, and overstates rho's phase-1 hash inserts by 6.4 to 7.9% (median 6.9%; exact count solved from `adds_db = 64*ceil(1.5 bits) + 2*ceil(s/32) + s`, which holds in 264 of 264 rho rows). The factor-base-build charge `4 m pbits` is a hand estimate, not an emitted counter. IC's table memory is about 2 m^2 entries (524k to 2.1M) against a rho database of about 183k to 520k (kappa 0.7); memory is not charged by the band, but the "memory-unbounded rho" framing hides that IC is not the smaller table.

### F2 (major, scope of "batch rho"): the rho reference is correct but 8 to 14% above a sequential-database anchored rho (computed) and 34 to 48% above a measured multi-target BSGS (BSGS is 25 to 32% cheaper)

All numbers from the raw rows or my independent harness.

* Correct: 12 instrumented rho runs verify 17,024 of 17,024 targets with independent code; phase-2 steps per target match theory (n/2)/N to 0.990 to 1.023 (observed/theory), so the walk space behaves like a random function and lookups are right. x-only lookup with the sign test (`same` y) gives `x = a2 - ca` or `-a2 - ca` correctly.
* Phase-1 waste (the database is built by restarted 32-step walks): steps per distinct database entry at kappa = 0.7 are 1.161 (24 b, T=4096), 1.383 (24 b, 16384), 1.171 (25 b, 4096), 1.429 (25 b, 16384); at kappa 0.5 1.11 to 1.27; at kappa 1.0 1.25 to 1.80. Per target (kappa 0.7, median): wasted steps 7.2 / 8.6 / 10.9 / 13.6 and phase-1 restart additions 3.3 / 1.9 / 4.6 / 2.8, against rho totals 142.4 / 94.8 / 188.7 / 122.2. A database of sequential multiples (G, 2G, ..., NG; N additions, N x-only inserts, same memory) has neither.
* **Computed (not run)** with the recorded phase-2 counters kept: IC v3 / rho, rho with a sequential database, at price 0: 1.139, 1.134, 1.140, 1.205 (vs 1.057, 1.027, 1.047, 1.072); with also one-add phase-2 starts: 1.157, 1.152, 1.158, 1.223.
* **Measured**, my own Python implementation with its own counter (x-only baby table of N = round(sqrt(nT)/2) multiples, deterministic giant steps, every target verified with my EC code; instances idx 1 to 3, 24 and 25 bits; `bsgs_large.json`): additions per target at price 0 = 101.3 / 69.5 / 127.8 / 82.6 (rho best kappa 142.4 / 92.9 / 188.7 / 118.5; IC v3 151.2 / 95.7 / 197.8 / 127.1). BSGS / rho = 0.71, 0.75, 0.68, 0.70; IC v3 / BSGS = 1.49, 1.38, 1.55, 1.54 at price 0; at price 1 with IC's committed ledger 1.25, 1.33, 1.36, 1.20, with its table inserts 2.03, 1.64, 1.94, 2.20 (BSGS charged one hash op per baby insert and per giant step). Reason: a deterministic giant-step search has mean n/(4N) steps per target against rho's n/(2N), so its optimum is about 1.0 sqrt(nT) vs 1.41 sqrt(nT). BSGS is a different algorithm from rho; the claim compares to "batch rho", but "within 10%" then measures one implementation, not the generic multi-target baseline at this T.
* A collision-based (distinguished-point style) multi-target rho is **not** shown cheaper: a crude idealized random-mapping simulation (M = 2^20 classes, T = 256 / 1024 / 4096, adaptive trails, no DP overhead) gives 1.19 / 1.25 / 1.31 times the anchored ideal. This is a model, six trials per point, not an implementation. T here (T/sqrt(n) = 0.7 to 4) is far above n^(1/4), where the sqrt(T) DP-rho analysis applies.
* Parameter slack, how far each could move the IC/rho ratio:

| parameter | evidence | effect on ratio (price 0) |
|---|---|---|
| kappa grid (0.5, 0.7, 1.0) | adding 0.7 to {0.5, 1.0}: per-config rho cost -0.00% median, -0.70% mean, -3.9% max; 0.7 wins in 44% of configs; cell medians +0.000 to +0.012. The RESULTS note "up to 5%" is an overstatement (max 3.9%, cells 1.2%). Cost is flat near the optimum (x + 1/x)/2: a finer grid gains at most about 1 to 1.5% | rho already near its optimum for this family; +0.01 at most |
| best kappa per config (oracle) | fixed kappa 0.7 gives 1.057 / 1.009 / 1.047 / 1.041 vs best-per-config 1.057 / 1.027 / 1.047 / 1.072 | generous to rho by 0 to 0.03 |
| restart length 32, start = 2 bank points + Q | restart additions 4.5 to 9.8 per target across both phases (2.6 to 5.2 in phase 2, 1.9 to 4.6 in phase 1; about 5% of rho's cost); one-add phase-2 starts alone move the ratio by +0.018 in every cell (R2 - R1) | about +0.02 |
| phase-1 random-walk database | see above | +0.07 to +0.13 (computed) |
| arm count 32, bank 64 | NOT RUN: constants are compile-time; I did not rebuild a modified example. Phase-2 behaviour matches theory, so no inefficiency is visible there | not measured |
| memory unbounded, no DP, x-only (negation) | all favour rho or are neutral; hash cost is priced by the band | not separable |
| database share of x-classes | kappa 0.7: 2.2 / 4.4 / 1.5 / 3.1%; kappa 1.0: 3.1 / 6.3 / 2.2 / 4.4% (kappa 1.0 at 24 b, T=16384 exceeds the declared 5% bound; it is never the best kappa) | none |

### F3 (minor): IC is also untuned and wasteful in small ways, all conservative for IC

* Redundant first restart per target: v3 target start calls `new_walk2(.., &[])` (which already runs `do_restart`: 2 counted additions, performed) and then `do_restart` again with `qm = [Q]`. Two additions per target are performed, counted and discarded. Removing them lowers IC/rho at price 0 by 0.014 / 0.021 / 0.010 / 0.017 (to 1.043 / 1.006 / 1.037 / 1.055). `w.b` is also updated every step in v3 although it is constantly 1. IC restarts cost 3 additions (3 bank points + Q) against rho's 2.
* The frozen rule snaps to powers of two. Post-hoc sensitivity (v3 binary, idx 1 to 3, seed 1, **outside the pre-registered rule**): at 25 b, T=16384 the frozen fb = 1024 costs 127.3 per target vs 114.0 at fb = 768 (-10.4%); elsewhere the frozen choice is within 0.6% of the best of {256, 384, 512, 768, 1024, 1536}. With fb = 768 at 25 b, T=16384, IC/rho on those three instances is 0.959 at all three prices under ledger D (frozen: 1.071 / 1.081 / 1.123), and 0.959 / 0.931 / 0.779 under ledger A. This is tuning on the test instances and is not evidence of an improvement; it shows that the 25 b, T=16384 cell is conservative for IC and that the sign of IC - rho is within the noise of tuning choices at the 5% level. Against the sequential-database rho (F2) and BSGS the sign does not change.
* Shared per-target verification (36 to 38 additions on both arms) dilutes the ratio: excluding it from both arms, IC/rho at price 0 = 1.076 / 1.044 / 1.059 / 1.106.

### F4 (minor, reporting): provenance and scope gaps

* Rows carry no commit sha, binary hash, argv or environment. The v1, v2 and v3 holdout runner scripts are byte-identical except for the output directory (`run_multi_holdout*.py` all call whatever `target/release/examples/ic_multi_target` was at run time); v1 and v2 are reproducible only with `MT_V2=1 [MT_FULL_TABLE=1]`, which the runners do not set. I reproduced them with the flags (section 5).
* No script for any `analysis*.txt` is committed; the raw data of the kappa scan that justified kappa = 0.7 is not in the repo (the only 0.7 rows are the holdout itself).
* The v1 table still uses S = 1.28 at 22 bits, which `RESULTS_vs_rho.md` line 46 already calls an interpolation that is too high (measured 0.925 at 16 runs, 1.112 at 64). Using S = 1.112 the 22-bit IC / independent-rho ratios at price 0 become 0.339 (T=128) and 0.114 (T=1024) instead of 0.295 and 0.099; at 24 bits with the measured S = 1.082 0.408 and 0.096 instead of 0.396 and 0.093. The conclusion (far below 1) stands. Using each instance's actual n instead of 2^(pbits/2) changes nothing (n is within a few percent of 2^pbits for all 48 instances).
* Phases are not separated in the final tables. From the rows, median additions per target (table | bank | G relations | target phase | total): (24 b, 4096, m=512) 64.0 | 0.56 | 8.7 | 77.7 | 151.2; (24 b, 16384) 16.0 | 0.14 | 2.2 | 77.3 | 95.7; (25 b, 4096) 64.0 | 0.59 | 17.0 | 116.3 | 197.8; (25 b, 16384, m=1024) 64.0 | 0.15 | 2.2 | 60.7 | 127.1. Rho (kappa 0.7): bank 0.6 | phase-1 44.8 distinct entries + 7.2 wasted + 3.3 restart | phase-2 steps 46.2 + restarts 4.0 | verify 36 (24 b, 4096).
* Outside the ecbench/isolab-eks protocol (AGENTS.md section 12): the producer says so. No pcc claim, `VALIDATION.json`, benchmark record or leaderboard row exists for these panels, consistent with the "unvalidated (pre-gate)" label. The m = 83 and ECC2K-130 rules are not engaged (prime-field toy curves).
* Toy regime: one prime per size, 12 curves per prime, groups 2^20 to 2^25, near-parity only where T/sqrt(n) = 0.7 to 4 (the IC/rho ratio at 24 b is 1.99 / 1.28 / 1.057 / 1.027 for T = 128 / 1024 / 4096 / 16384). At fixed T the v3 ratio grows with n (T=1024: 1.12, 1.12, 1.28 at 20, 22, 24 b). Extrapolation to cryptographic n with T << n^(1/2) is not supported by these data. The narrow bootstrap intervals are spread over a nearly deterministic count on 22 non-independent configurations (11 curves x 2 seeds), not a sampling confidence.

## 3. Requirement table

| # | requirement (dispatch) | result | evidence |
|---|---|---|---|
| 1a | Algorithm flaw that makes a reported log wrong | **PASS.** `x = b^-1 (s_i y_i + s_j y_j - a)` follows from `a + b x_t = log(s_i F_i + s_j F_j)`; `xk` is used only to build Q (separate RNG stream); a wrong log would fail the check | `ic_multi_target.rs` 286-327 |
| 1b | Verification vacuous | **PASS.** `FieldElement::eq` compares reduced values; check is `smul(g,x) == q`. My checker confirms `x == xk` and `x*G == Q` with Q on curve for 80,000 targets (59,520 default v3, 1,280 `MT_V2`, 2,176 `MT_V2+MT_FULL_TABLE`, 17,024 rho), 0 failures. Negative control: the checker rejects x+1, -x, a wrong Q, y+1 and a sign-flipped y | `cert.py`, `cert_results.json` (scratchpad) |
| 1c | `sparse_solve` signs | **PASS.** Derived independently: `alpha_v = s_v(a - s_u alpha_u)`, `beta_v = -s_v s_u beta_u`; a non-tree edge with opposite betas gives `t = (alpha_v - alpha_v')/(beta_v' - beta_v)`, which the code implements including the `!bv_plus` sign flip; self-loop rows give `y = +-a/2`. Empirical: 6,753 determined factor-base logs satisfy `y_i G = F_i` exactly (0 failures). Omission, not error: a node fixed by a self-loop inside a component whose `t` is otherwise unfixed is not used to fix `t` (conservative; 130 of 132 v3 holdout configs and 88 of 88 large-T configs leave columns undetermined, 1 to 6% (median about 2%); skipped relations are counted as trials) | `sparse_solve` 341-428 |
| 1d | Group addition performed but not counted / counted but not performed | **PASS.** Static: every `Point::add` in IC goes through `Counter::add`; rho pairs each `.add` with `adds += ..`; the only uncounted group operation is the planted `g.scalar_mul(&xk)` (IC line 287, rho line 120), the same in both arms; `neg()` is a field negation. Dynamic, independent counter in `Point::add/scalar_mul`: `Point::add` calls = `adds_total - ceil(1.5 bits)*(64+T)` and `scalar_mul` calls = `64 + 2T` in 36 of 36 runs (18 default v3 over 20 to 25 bits and T = 128 to 16384, 3 `MT_V2`, 3 `MT_V2+MT_FULL_TABLE`, 12 rho with kappa 0.5/0.7/1/2). Two redundant but counted additions per target: F3 | `cert.py` |
| 2a | rho correct | **PASS** (F2) | |
| 2b | rho not deliberately weak | **PARTIAL.** No deliberate weakening found and the kappa grid is within about 1% of optimal, but phase 1 wastes 5 to 11% of rho's cost and a sequential database is cheaper by 7 to 12% (computed from counters) and a multi-target BSGS by 25 to 32% (measured) | F2 |
| 2c | unfairly generous to rho | **PARTIAL.** Memory unbounded, no DP overhead, per-config best kappa (0 to 0.03). None of these is priced; hash cost is covered only by the band | F2 |
| 3 | recompute v1/v2/v3, batch rho, T_ext, T_ext_v3, large-T tables | **PASS (arithmetic).** Every median, count and "cheaper in k of 22" reproduces; only the last digit of some bootstrap bounds differs (e.g. 1.72 vs 1.73) from resampling noise. Headline: 1.057, 1.027, 1.047, 1.072 at 0 and 0.847, 0.923, 0.887, 0.788 at 1, 22 configs per cell, 352 of 352 runs verified, cheaper counts 0/0/22 per cell. **FAIL (ledger)** for the "order flips" interpretation: F1 | `lt.py tables.py kap.py` |
| 4a | replay >= 12 IC and >= 12 rho | **PASS.** 88 sampled configs with the producer's binaries (seed 20261107) and 88 different configs with a fresh build of HEAD (seed 777): 52 + 52 IC (v3 holdout 16+16, v2 8+8, v1 8+8, large-T 12+12, T_ext v1 4+4, T_ext_v3 4+4) and 36 + 36 rho (batch_rho 16+16, large-T 16+16, T_ext 4+4). All 176: `ok == T`, and every compared counter (`adds_total, adds_table, adds_bank, adds_grel, adds_targets, trials_g, trials_targets, la_ops, determined, skipped_undetermined`; rho `adds_db, db, target_steps`) identical. Also T_ext rho rows replay inside `large_T_holdout` (32 of 32 identical). The binary dated 04:13 predates the 04:14:04 commit, but it matches HEAD behaviourally on all replayed configs; byte hashes differ from my build (different paths), so I do not claim byte-identity | `replay.py replay2.py` |
| 4b | instance arithmetic | **PASS, all 48 not 5.** `W{20,22,24,25}-{0..11}`: SHA-256 equals `MANIFEST.json`; p and n prime; nonsingular; G on curve; n*G = O with my affine arithmetic; Hasse holds; full Legendre-sum point count gives #E = n (prime order, cofactor 1). 12 distinct (a, b) per size, all over the same prime p per size | `inst.py` |
| 5a | rule and runner committed before rows | **PASS.** v1: `run_multi_holdout.py` + fb rule `ce1ebd4` 03:32:29Z, rows mtime 03:33:10Z (commit 03:34:00Z). batch rho: `5ebf6a2` 03:37:36Z, rows 03:38:50Z (commit 03:39:28Z). v2: `4e62822` 04:12:07Z, rows 04:12:43Z. v3: `6731caa` 04:14:04Z, rows 04:14:38Z. large-T: `1c7e5d7` 04:18:38Z, rows 04:30:14Z (commit 04:30:49Z). Commit time proves order only because file mtimes agree | `git log`, `ls --time-style=full-iso` |
| 5b | holdout disjoint from the exploratory sweep | **PASS.** `run_multi.py` and its rows use `W{20,22,24}-0` only (sizes 20/22/24, T <= 4096); holdouts use idx 1 to 11. The frozen fb rule is an extrapolation at 25 b and T = 16384 | `run_multi.py`, `results/multi/` |
| 5c | independence of the "holdouts" | **PARTIAL.** v2 and v3 were designed after seeing v1 and batch-rho results on the same idx 1 to 11 and re-run on them, so they are post-selected variants, not holdouts for the code changes. The large-T "holdout" overlaps idx 1 to 4, seed 1 (4 of 22 configs per cell) with `T_ext_v3` (seen at 04:16:15, before the 04:18:38 freeze) and with the kappa scan. Effect: scanned vs non-scanned subsets differ by <= 0.007 in IC/rho (e.g. 1.055 vs 1.057, 1.022 vs 1.029, 1.043 vs 1.048, 1.073 vs 1.072); the kappa = 0.7 addition raises rho's strength (conservative for IC) by the amounts in F2. Estimated contamination of the headline: below 0.01, direction against IC | `kap.py` |
| 6a | validity regime | **PASS with note.** Database 1.1 to 4.4% of x-classes at the winning kappas (kappa 1.0 reaches 6.3% at 24 b, T=16384; never the best). The 20/22-bit exclusion is correctly stated | `comp.py` |
| 6b | S interpolation | see F4 (conclusion unaffected) | `indep.py` |
| 6c | independent counting harness | **PASS (supplied by this report):** F1 and requirement 1d | |
| 6d | near-parity vs improvement wording | **OVER-STATED.** Price-0 numbers say "3 to 7% above", never below. "wins in every configuration at price 1" is a ledger artifact (F1). "Reduced from 1.7 to 2.8x to 1.0 to 1.1x" mixes regimes (v1 at large T was 1.56 to 1.93 in `T_ext`) and is relative to a reference that is itself slack (F2) | |
| 6e | pricing symmetry | **FAIL** (F1): IC lacks table inserts; rho's hash count is a proxy; per-step exponent updates, finalization (a few modular operations and one `mod_inverse` per target) and the fb-build estimate are not emitted counters on either arm | |
| 6f | one table, one unit, phases separated, all arms priced | **PARTIAL.** One unit (GAE) yes; phases not separated in the final tables (given in F4); the stronger generic arms (sequential database, BSGS) are missing; the T independent single-rho column (v1) is superseded | |

## 4. Recomputed headline (large-T holdout, symmetric pricing as specified)

Median of per-configuration IC v3 / best-kappa rho, bootstrap 95% (2000 resamples, seed 7) at price 0:

| cell | IC @0 | rho @0 | ratio @0 [boot95] | @0.1 | @1 | cheaper (0 / 0.1 / 1) |
|---|---|---|---|---|---|---|
| 24 b, T=4096 | 151.2 | 142.4 | 1.057 [1.052, 1.063] | 1.023 | 0.847 | 0 / 0 / 22 |
| 24 b, T=16384 | 95.7 | 92.9 | 1.027 [1.022, 1.031] | 1.012 | 0.923 | 0 / 0 / 22 |
| 25 b, T=4096 | 197.8 | 188.7 | 1.047 [1.045, 1.049] | 1.021 | 0.887 | 0 / 0 / 22 |
| 25 b, T=16384 | 127.1 | 118.5 | 1.072 [1.071, 1.074] | 1.028 | 0.788 | 0 / 0 / 22 |

Earlier tables recompute exactly under both the original (asymmetric) and the symmetric pricing, for example symmetric v3: 20 b/128 1.47 [1.42, 1.50], 20 b/1024 1.12, 22 b/128 1.81, 22 b/1024 1.12 (price 1: 0.97, 21 of 22), 24 b/128 1.99, 24 b/1024 1.28; v1 vs batch rho 2.34 / 1.75 / 2.28 / 1.84 / 2.84 / 1.70; T_ext (IC v1) 1.75 / 1.60 / 1.56 / 1.93; T_ext_v3 1.05 / 1.02 / 1.04 / 1.07. Paired configs share their planted targets (both programs draw them from the same stream `seed ^ 0x9e3779b97f4a7c15`), so the pairing is valid.

## 5. Replay and audit detail

* `replay.py` (producer binaries) and `replay2.py` (fresh HEAD build): arguments exactly as the runners build them (`m = 2^round(log2((p*T)^(1/4)))`, seed, kappa); v1 rows with `MT_V2=1 MT_FULL_TABLE=1`, v2 rows with `MT_V2=1`. 176 of 176 identical, 176 of 176 `ok == T`.
* `cert.py` over `/tmp/val_target/release/examples/*_val`: 36 runs, 80,000 independent target certificates, 6,753 factor-base logs, 36 of 36 add-count and 36 of 36 scalar-multiplication-count equalities.
* `inst.py`: 48 of 48 instances.
* `bsgs.py`, `bsgs2.py`: 18 runs (3 instances x 4 large-T cells, plus 3 instances at 24 b, T=1024 not tabulated), all targets verified.
* `unanch.py`: idealized collision-model comparison; `fbsens.py`, `fbsens2.py`: post-hoc fb sensitivity.

## 6. Not run

Arm-count and bank-size sweeps for rho (compile-time constants; I did not build a modified example); an implemented distinguished-point rho (only the idealized model above); IC at fb values other than those listed (F3); an independent count of the field multiplications in `build_factor_base` (the `4 m pbits` charge stays an estimate); ecbench replay, isolab-eks, pcc, the validation gate and the m = 83 / ECC2K-130 disclosure rules (not engaged by this work).

## 7. Required corrections before the claim is cited

1. Emit and price IC's table-insert counter (and per-step exponent updates, finalization operations, an actual field-multiplication count for the factor-base build) and exact rho hash counts, regenerate every symmetric table, and remove "unranked", "order flips" and "wins 22 of 22 at price 1" until they hold under the complete ledger.
2. Scope "within about 10%" to "the anchored random-walk rho as implemented, price 0 and 0.1", and add the sequential-database rho and a multi-target BSGS arm (same memory) before any statement about parity with generic multi-target attacks.
3. Correct "a small amount of other work", the "up to 5%" kappa note (max 3.9% per config, cell medians <= 1.2%), the stale 22-bit S in the v1 table, and the cross-regime "1.7 to 2.8x to 1.0 to 1.1x".
4. Record commit sha, binary sha-256, argv and environment in the rows; commit the analysis scripts and the kappa-scan raw data; commit the `MT_V2` / `MT_FULL_TABLE` settings used for v1 and v2.
5. Disclose that the large-T panel overlaps the earlier T_ext_v3 / scan instances, that v2 and v3 are post-selected variants, and that the fb rule is not optimal at 25 b, T=16384.
6. Engineering (not accounting): remove the redundant first restart and the dead `b` update in v3 (moves IC by 0.010 to 0.021 in ratio), which would be a real, small improvement if re-measured under the frozen rule.
