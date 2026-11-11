# Validator report: prime_fast multi-target IC frozen evaluation

Validator: independent session, role `validator` (Sonnet 5.5). Date: 2026-11-07.
Dispatch: validate the frozen evaluation of PR isogeny-labs/crypto#3636.

## 0. What was validated, and from where

| item | value |
|---|---|
| validated commit | `054aa11952f052681ee8856d1aa394e0d4f722c1` (PR branch tip), tree `77cfa2809fe0bf5903e382d10c456da1e2db37fc` |
| working tree | clean at the start (`git status` empty). During validation the PR was merged: HEAD became merge commit `98ddd0f81c3e` (2026-11-07T09:36:47Z, parents `aecd4d3cc0` and `054aa11952`). Its tree is byte-identical to the validated tree (`git rev-parse 054aa11952^{tree} HEAD^{tree}` both `77cfa2809f…`; `git diff 054aa11952 HEAD` empty). The tree did not move under the validation. |
| evidence set hashes (sha256) | eval_23 `116e5a6e…`, eval_26 `c41ced0e…`, eval_30 `38118b45…`, eval_31 `1f3694bc…`, eval_33 `da4d1a2f…`, frozen.json `583ab2f3…`, provenance.txt `8b5ddad7…`, RESULTS_prime_fast_multi.md `f0dfea5d…`, examples/prime_fast_multi.rs `5380c482…`, src/cryptanalysis/prime_fast.rs `019a3304…` |
| source between freeze and head | `git diff 3211c092c9 HEAD -- src examples Cargo.toml Cargo.lock` is empty; blobs `prime_fast.rs` `95e5266721`, `prime_fast_multi.rs` `3fb097ea00` |
| validation host | 4 vCPU Intel Xeon (family 6 model 207, KVM) at 2.1 GHz, 15 GiB, rustc/cargo 1.97.0, Python 3.13.18. The producer's binary and rows are on this same machine, so this is a same-host replay, not a different-host-class audit. |

I did not edit evidence, results, source or status. The only repository file I wrote is this report. A clean rebuild
altered the git-ignored `target/` directory (see 2.1). I did not append to `docs/session-logs/` because the dispatch
restricts me to this file.

## 1. Disposition

Overall: **`validated` for the measurement** (every returned log re-verified by independent code; the rows reproduce)
**and for Finding 1; Findings 2 and 3 are `qualified`** and must be re-worded before they are cited. The result is
**not eligible** for the IC leaderboard or `benchmarks/README.md` (no `VALIDATION.json`, no §5a pricing band, wall-clock
unit; see 6).

| # | claim in `RESULTS_prime_fast_multi.md` | verdict | one-line reason |
|---|---|---|---|
| C1 | constants frozen (c = 1.0, T = 1000) before the evaluation rows | **validated** (qualified) | commit order and file mtimes agree; the exploratory basis for c is not committed |
| C2 | all 22 000 targets verified, no failed run | **validated** | 22 000 / 22 000 re-verified with Python big-int arithmetic, 0 failures |
| C3 | `ic_multi_solve` returns only verified logs | **validated** | code path + mutation test: 0 wrong logs returned under corrupted factor-base logs |
| C4 | all costs inside the timed region, planted targets outside | **validated** | code read; phases listed in 5.1 |
| C5 | table values (medians, ratios) | **validated** | all 22 rows recomputed, 0 arithmetic mismatches; one text rounding (570x vs "550x") |
| C6 | Finding 1: 130x to 550x less total time than ic_solve x T | **validated** (qualified) | range is per-size medians (134x to 570x; rows 105x to 580x); the reference is an extrapolation and a naive T-fold repeat |
| C7 | Finding 2: advantage over rho x T is 48x, 13x, 2.5x, 2.1x, none at 33 bits | **qualified** | numbers reproduce, but the rho arm spends 1.3 to 2.9 ms per solve building a 1024-entry table; right-sized rho gives 9.5x, 4.4x, 1.14x, 1.44x, 0.72x |
| C8 | Finding 3: IC wins only at 23 bits against idealised multi-target rho, loses 2.5x to 33x from 26 bits | **qualified**; the 23-bit win is **not validated** | the idealised arm multiplies rho's fixed table cost by sqrt(T); with fixed cost paid once IC loses 3.8x at 23 bits and 7.7x to 44x above |
| C9 | "no improvement over the best known rho strategy" | **not validated as a measured statement** (direction supported) | no multi-target rho was run; see 5.4 |
| C10 | "favourable to rho" label on the idealised arm | **not validated** | unfavourable to rho at 23 and 26 bits, see 5.3 |
| C11 | "earlier exploratory 32-bit runs: 0.46 at T = 10 000" | **not validated** | no committed artifact |
| C12 | "c chosen on 24- and 28-bit curves, which the evaluation excludes" | disjointness **validated**; "c = 1.0 best or tied" **not validated** | curves disjoint; probe output not committed |
| C13 | per-target sweep is O(fb), "which is what grows" | **not validated** | interpretation; no phase profile of the per-target cost |

## 2. Requirement checks

### 2.1 (1) Freeze order and constants: PASS (qualified)

- `git show 3211c092c9:research/ic_e222_m221_20261106/results/pf_multi/frozen.json`: `"c": 1.0`, `"T": [1000]`, `"extra": 6`,
  `eval_bits [23,26,30,31,33]`, instances `{23:3, 26:3, 30:2, 31:2, 33:1}`. File unchanged to HEAD.
- Commit order (git log): `3211c092c9` freeze 09:26:41Z, `4dc49381af` rows 09:27:30Z, `054aa11952` final rows and RESULTS 09:28:52Z.
- File mtimes on the producer machine (consistent with the commits): `frozen.json` 09:26:35.7, binary
  `target/release/examples/prime_fast_multi` 09:26:36.8, `provenance.txt` 09:26:48.98, `eval_23` 09:26:49.4, `eval_26` 09:26:51.0,
  `eval_30` 09:27:06.3, `eval_31` 09:27:23.6, `eval_33` 09:28:28.1. Segment durations match the work (33-bit segment 65 s against an
  estimated 5 single solves x 2 residues of ic_solve at ~4.7 s plus two multi runs of 8.3 s).
- Rows exist for exactly the declared sizes and instance counts (22 rows: 6, 6, 4, 4, 2); no `failed` rows; `fb = round(1.0 * p^(1/3))` in every row.
- Qualifiers. (a) Git and mtimes show the freeze preceded the rows' commit and the file writes; they cannot prove when the run started, and "verified
  on the remote before the run" cannot be checked from here (origin now contains `3211c092c9`, nothing records the push time).
  (b) `frozen.json` states no numeric success or abandonment condition (AGENTS.md "Declare the falsification target in advance").
  (c) The "m24/m28 probes" that fixed c = 1.0, and the exploratory 32-bit T = 10 000 figure, are not in the repository
  (no probe output under `pf_multi/` or elsewhere in `research/ic_e222_m221_20261106/`; the only `m24` paths in `git ls-files` are unrelated
  gb_cas_comparison inputs). The text also says a first version failed ~15% of
  targets and was fixed by the retry loop; that is recorded only in `docs/session-logs/2026-11-07.md`.
  (d) The command line, host CPU and core count, rustc version and dirty-tree state are not recorded; `provenance.txt` holds only
  `commit 3211c092… bin 0fee411c421f1510`.

### 2.2 (2) Clean rebuild and re-run: PASS

- The producer's binary (sha256 `0fee411c421f1510…`, matches `provenance.txt`) was copied aside, then
  `cargo clean -p crypto --release --offline` and `cargo build --release --example prime_fast_multi --offline` (6 min 54 s, exit 0).
  The git-ignored `target/` of `/home/user/crypto` was rewritten by this; tracked files were untouched.
- Rebuilt binary sha256 `1da1d65d0679e3f994c1542e2d3ef811f14fe6bcab4145ae0df6c18ba983e26a`: **not bit-identical** to the producer's
  binary, same size (5 279 512 B). `cmp -l` shows 37 differing bytes: the 20-byte GNU build-id and 17 bytes of one private
  LLVM symbol suffix (`FOREIGN_SEEN.0.llvm.<hash>`) in `.strtab`. Everything else, including `.text`, `.rodata` and `.data`, is identical.
  So the provenance hash is not reproducible from source, but the executable code is the same; the evaluation ran the committed code.
- Re-run with the rebuilt binary, same arguments as `frozen.json` (`prime_fast_multi <out> <bits> 1000 1.0 <instances>`): 23 and 26 bits, 3 instances each
  (12 rows); 30 and 33 bits, instance 0 each (4 rows). Instances are seeded, so rows pair one to one.

| bits | res | inst | total ms orig | total ms rerun | rerun/orig | ok orig/rerun | determined orig/rerun |
|---|---|---|---|---|---|---|---|
| 23 | 3 | 0,1,2 | 47.3, 48.3, 47.7 | 49.8, 69.7, 45.6 | 1.05, 1.44, 0.95 | 1000/1000 each | 192, 189, 192 identical |
| 23 | 1 | 0,1,2 | 48.9, 50.7, 48.6 | 46.5, 49.1, 48.5 | 0.95, 0.97, 1.00 | 1000/1000 each | 190, 185, 188 identical |
| 26 | 3 | 0,1,2 | 208.8, 168.3, 176.6 | 218.7, 218.8, 202.1 | 1.05, 1.30, 1.14 | 1000/1000 each | 370, 382, 373 identical |
| 26 | 1 | 0,1,2 | 289.4, 211.1, 213.5 | 160.5, 181.1, 204.4 | 0.55, 0.86, 0.96 | 1000/1000 each | 365, 367, 363 identical |
| 30 | 3 | 0 | 2176.0 | 1903.4 | 0.87 | 1000/1000 | 942 identical |
| 30 | 1 | 0 | 1757.5 | 2192.1 | 1.25 | 1000/1000 | 948 identical |
| 33 | 3 | 0 | 8415.0 | 8480.1 | 1.01 | 1000/1000 | 1882 identical |
| 33 | 1 | 0 | 8238.6 | 8851.4 | 1.07 | 1000/1000 | 1886 identical |

  Verification counts and `determined` counts are identical on all 16 rows. Timing ratio rerun/orig: median 1.00, range 0.55 to 1.44
  (the 0.55 is an outlier in the original: setup 68.8 ms against 29.8 ms in the re-run). Same-subset medians of the three headline
  ratios reproduce within 10%: vs ic_solve x T 0.0075/0.0081 (23), 0.0070/0.0075 (26), 0.0027/0.0028 (30), 0.0018/0.0019 (33);
  vs rho x T 0.0205/0.0233, 0.0785/0.0677, 0.4068/0.4398, 1.049/1.112. Run-to-run noise on this shared VM is therefore about +-30% per row
  and there is no A/A interval or isolation level (see 6).
- I additionally replayed all 22 instance-rows (without the reference arms) through a harness that uses the producer's seeds
  (3.1); its `determined`, `n` and `fb` equal the committed rows on 22 / 22.
- Reference arms re-sampled: ic_solve summands 3, targets disjoint from the producer's, mean of N solves (ms): 23 bits 5.85 / 6.76 (N = 100, residues 3 / 1; producer
  medians of 5: 6.95 / 6.09), 26 bits 31.2 / 26.8 (N = 40; 29.6 / 30.2), 30 bits 740 (N = 12; 714.7). The median-of-5 reference is representative within about 15%.

### 2.3 (3) Independent verification of returned logs: PASS

- Harness (`/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/harness/dump_multi.rs`, sha256 `30e4a441…`) calls the committed
  `find_a3_curve` and `ic_multi_solve` with the producer's seeds (`0xC0DE ^ ins ^ n ^ T`, fb = round(1.0 p^(1/3)), extra 6, max_walk 5e6, seed ins + 1),
  and dumps p, a, b, n, G, every Q_t and the returned log as JSON.
- Checker (`…/scratchpad/py/verify_logs.py`, sha256 `392f9a9f…`) is Python only. It uses its own affine short-Weierstrass arithmetic on Python integers and
  imports nothing from prime_fast. Per instance it checks: p and n prime (Miller-Rabin, 12 bases), a = p - 3, non-singular, G on the curve,
  n * G = O, G of order > 3, |p + 1 - n| <= 2 sqrt(p), n > 4 sqrt(p) (so n is the only multiple of the order in the Hasse interval). Per target it checks:
  Q_t on the curve, 0 < k < n, and `k * G == Q_t`.
- Result on all 22 instance-rows: **22 000 targets, 22 000 verified `kG == Q_t`, 0 unsolved, 0 wrong, 0 verified-but-different-from-planted.**
  Dump hashes: `d_23` `b745ed44…`, `d_26` `53b59e32…`, `d_30` `f19d5d3d…`, `d_31` `1511a589…`, `d_33` `a0208c5c…`.
- **Fail-closed.** In `ic_multi_solve` (prime_fast.rs lines 1757 to 1784) the only assignment `res = Some(k)` is guarded by
  `curve.mul(Some(g), k) == Some(q)`; there is no other write to `out`. To test the guard rather than trust the reading, I compiled
  stand-alone copies of `prime_fast.rs` (stubs for the four crate-internal imports; `mut/pf_unmut.rs` reproduces the committed results: 23-bit
  determined 192 / 190, 26-bit 370 / 365, 1000 correct each) and two mutants of `solve_fb_logs`:
  A: every determined log is off by +1; B: every column with index c % 5 == 0 is off by +1.

  | variant | bits 23 res 3 | 23 res 1 | 26 res 3 | 26 res 1 |
  |---|---|---|---|---|
  | unmutated copy: correct / WRONG / None | 1000 / 0 / 0 | 1000 / 0 / 0 | 1000 / 0 / 0 | 1000 / 0 / 0 |
  | mutant A | 0 / **0** / 1000 | 0 / **0** / 1000 | 2 / **0** / 998 | 1 / **0** / 999 |
  | mutant B | 565 / **0** / 435 | 574 / **0** / 426 | 551 / **0** / 449 | 567 / **0** / 433 |

  No wrong log was ever returned. The few correct returns under mutant A are relations whose signed coefficients cancel the +1 offset; they
  verified. The producer's artifact set contains no such null control; this supplies one.
- Certificate-discipline gap: the committed rows carry only counts (`ok`, `determined`), not (Q_t, k) pairs, so the logs cannot be re-verified from the
  artifacts without re-running. My replay closes this for the 22 rows; the producer should commit a certificate file (or the store reference) next time.

### 2.4 (4) Disjoint curves: PASS

`find_a3_curve(bits, res)` takes the largest prime p < 2^bits with p mod 4 = res and the smallest b giving prime order, so curves of different bit lengths
cannot coincide. Enumerated with the committed code (`…/scratchpad/dump/curves.jsonl`):

| set | (bits, res, p, b) |
|---|---|
| evaluation | (23,3,8388587,1) (23,1,8388593,3) (26,3,67108859,30) (26,1,67108837,42) (30,3,1073741783,36) (30,1,1073741789,34) (31,3,2147483647,21) (31,1,2147483629,9) (33,3,8589934583,28) (33,1,8589934513,206) |
| exploratory (24, 28, 32) | (24,3,16777199,31) (24,1,16777213,13) (28,3,268435399,3) (28,1,268435361,10) (32,3,4294967291,31) (32,1,4294967197,26) |

No shared p, no shared (p, a, b), no shared n. The disjointness of curves is validated; whether c = 1.0 really was "best or tied" there is not (2.1 c).
The rows within one (bits, res) share one curve and one reference measurement (`rho_single_ms`, `s3_single_ms` are identical across the instance rows),
so the 22 rows contain only 10 independent reference measurements; row medians are pseudo-replicated against the references.

### 2.5 (5) Costs, timed region, arms

**5.1 Timed region: PASS.** In `examples/prime_fast_multi.rs` the clock (`t0 = Instant::now()` at line 58, `wall` at line 60) brackets only `ic_multi_solve`;
the planted `ks` and `qs` (lines 55 to 57) are built before it. Inside `ic_multi_solve` (prime_fast.rs 1704 to 1799): factor base build, `PairTable::build`,
G-only relation collection (`collect_relations(..., g, g, ...)`), `solve_fb_logs`, then per target: the fresh `combine`, up to 32 relation searches
(`collect_relations`), the extraction arithmetic and the group verification. Discarded unusable attempts are timed. `total_ms` is `wall`; `setup_ms` is the
shared phase (12% of total at 23 bits rising to 50% at 33). Not priced: memory (pair table B^2 x 16 B, about 67 MB at 33 bits), a counted-operation ledger.

**5.2 Reference arms as implemented.** `ic_solve` summands 3, c = 0.5, extra 6, timed end to end, median of 5, times T (an extrapolation, disclosed). `rho_parallel`
via `RhoConfig::for_bits(bits, true); cfg.threads = 1; cfg.seed = t + 1` (example lines 27 to 29), median of 10 single solves, times T.

**5.3 The rho arm is not right-sized, and this moves Findings 2 and 3.** `rho_parallel` builds a table of `1 << cfg.table_bits` = 1024 entries, each a
`combine(g, q, u, v)`, and `for_bits` leaves `table_bits = 10` at every size. A validator probe (`…/scratchpad/harness/rho_probe.rs`, sha256 `6edea889…`;
same call as the example, 1 thread, 100 solves per cell, both residues) separates the table cost:

| bits | rho producer (median of 10, rows) | rho table_bits 10 (mean of 100) | table build alone | rho table_bits 5 (mean of 100) | steps / sqrt(n) (tb 10 / tb 5) |
|---|---|---|---|---|---|
| 23 | 2.37 ms | 2.47 ms | 1.3 to 2.0 ms | **0.46 ms** | 0.95-1.05 / 0.97-1.01 |
| 26 | 2.68 | 2.71 | 1.9 to 2.1 | **0.91** | 0.94-0.96 / 0.88-1.03 |
| 30 | 4.82 | 4.36 | 1.9 to 2.3 | **2.18** | 0.88-0.95 / 0.89 |
| 31 | 4.79 | 5.38 | 2.2 | **3.28** | 0.92-0.95 / 0.96-1.06 |
| 33 | 7.94 | 7.72 | 2.2 to 2.9 | **6.02** | 0.88 / 0.92-0.99 |

The producer's rho measurement is reproducible (median of 10 equals the mean of 100 within 15%), and the walk itself is on the expected curve (about 0.9 to 1.0 sqrt(n)
steps, 59 to 70 ns per step at 30 to 33 bits). But the rho time is flat from 23 to 26 bits (2.48 ms and 2.67 ms, while sqrt(n) grows 2.8x) because roughly 80% of the
23-bit solve and 60 to 70% of the 26-bit solve is the 1024-entry table build (difference of the table_bits-10 and table_bits-5 means). A 32-entry table gives the same step counts. The `ratio_vs_rho_Tx` column therefore overstates the
IC advantage at the small sizes. Recomputed with the validator's table_bits-5 rho (a probe, not a frozen protocol; the IC rows are the committed ones, per-size median total):

| bits | IC total ms | reported speedup vs rho x T | speedup vs right-sized rho x T | reported IC / idealised multi-rho | IC / idealised, fixed cost paid once |
|---|---|---|---|---|---|
| 23 | 48.5 | 48.9x | **9.5x** | 0.6 (IC wins) | **3.8** (IC loses) |
| 26 | 209.9 | 12.7x | **4.4x** | 2.5 | **7.7** |
| 30 | 1909.9 | 2.5x | **1.14x** | 12.9 | **28.6** |
| 31 | 2282.0 | 2.1x | **1.44x** | 15.2 | **22.6** |
| 33 | 8326.8 | 1.0x (0.95) | **0.72x** (IC slower by 1.4x) | 33.2 | **44.3** |

"Idealised, fixed cost paid once" = table once + sqrt(1000) x (walk time), with walk time = right-sized mean solve minus table build (`…/py/corrected.py`).
It is a second idealisation of the same kind as the producer's, not a run. It shows the producer's arm `rho_single x sqrt(T)` charges a sqrt(T) = 31.6-fold
repeat of a fixed setup that a shared-table multi-target rho pays once; at 23 and 26 bits that makes the arm unfavourable to rho, contradicting its label.
The break-even against independent rho moves from about 33 bits to about 30 to 32 bits.

**5.4 Would a real distinguished-point multi-target rho change the conclusion?** Not run: I did not implement one, so this is an assessment.
- Direction: the correction above makes the idealised arm cheaper, so Finding 3's negative conclusion (IC does not beat multi-target rho) strengthens at every size
  except the 23-bit exception, which reverses (IC 0.65x becomes 3.8x worse).
- Margin: a real multi-target rho would have to cost more than the corrected idealised arm by a factor of 3.8 (23 bits), 7.7 (26), 28.6 (30), 22.6 (31) or 44.3 (33)
  for IC to tie. The usual overheads (distinguished-point storage and lookups, walk restarts, the constant and any log factor for recovering all T rather than one log,
  no free negation map when targets are tagged) are plausibly a small multiple, not 8x to 44x. I cannot exclude a tie at 23 bits, and I would not expect one at 26 bits or above.
  This is judgment, not measurement. I could not consult the research-kb in this session (no kb tools), so the constant for recovering all T logs is not checked against the literature.
- Existing evidence in the repository, not re-validated here: `examples/ic_batch_rho.rs`, `examples/ic_multi_bsgs.rs`, `results/batch_rho/`, `results/bsgs_large_T/` and
  `docs/session-logs/2026-11-07.md` (lines 546 to 645) record real counted multi-target rho and BSGS runs at 20 to 25 bits in which multi-target IC cost 1.0x to 2.2x
  the generic arm. They are in counted additions on another engine, so they do not transfer to wall-clock on `prime_fast`, but they say a real multi-target arm exists in the repo and
  points the same way. `RESULTS_prime_fast_multi.md` neither uses nor cites them. It also does not mention the earlier multi-target IC v3, so Finding 1 is not compared against the
  repo's previous multi-target baseline.
- A tied or lost row cannot be ranked from wall-clock anyway: AGENTS.md §5a requires the 0 / 0.1 / 1 GAE band on every arm. The document concedes the absence.

**5.5 Single-thread and parallel handling (quoted).**
- Example: `let mut cfg = RhoConfig::for_bits(bits, true); cfg.threads = 1; cfg.seed = t + 1;` (lines 27 to 29). `for_bits` sizes walkers, threads and dp_bits from
  `std::thread::available_parallelism()`; the example then overrides `threads` to 1 without recomputing, so the per-thread walker count depends on the host's core count.
  On the 4-vCPU host the probe prints walkers = 40, 113, 113, 160, 320 and dp_bits = 3 for 23, 26, 30, 31, 33 bits. The core count is not in `provenance.txt`.
- The IC path has no threads (`grep thread|rayon|spawn` over prime_fast.rs lines 1040 to 1860: no matches); the multi-target phase and the per-target phase are single-threaded.
- Hence every arm is one thread and "1 thread everywhere" in the Limits is correct. Neither side's parallel scaling is measured. The per-target phase of IC is embarrassingly parallel and so is a
  DP multi-target rho, so a single-thread comparison is an equal-footing per-core comparison, but it says nothing about a parallel crossover.

### 2.6 (6) Wording that overclaims

| line | text | problem | suggested replacement |
|---|---|---|---|
| 9 | "idealised multi-target rho ... (not measured, favourable to rho)" | unfavourable to rho at 23 and 26 bits (5.3) | "idealised multi-target rho = single-solve time x sqrt(T), not measured; it repeats rho's fixed 1024-entry table build sqrt(T) times and is therefore pessimistic for rho at small sizes" |
| 22 | "130x to 550x less total time" | range of per-size medians; the largest is 570x; rows span 105x to 580x | "134x to 570x per size (median of rows), 105x to 580x over rows; against T x median of 5 single ic_solve solves" |
| 22 | "a reproducible, verified improvement" | true for the measurement; the baseline is a naive T-fold repeat of a single-target solver | add "over repeating single-target ic_solve; not compared with the repository's earlier multi-target IC v3" |
| 23 | "48x at 23 bits, 13x at 26, 2.5x at 30, 2.1x at 31, and none at 33" | the rho arm carries ~2 ms of table build per solve; right-sized rho gives 9.5x, 4.4x, 1.14x, 1.44x, 0.72x | report both, label the first set "against rho_parallel with its default 1024-entry table" |
| 23 | "earlier exploratory 32-bit runs: 0.46 at T = 10 000" | no artifact | delete or mark `[unverified: no committed artifact]` |
| 24 | "IC wins only at 23 bits" | artifact of the sqrt(T) x fixed-cost arm; IC loses 3.8x with fixed costs paid once | "IC does not beat the idealised multi-target arm at any size tested" |
| 24 | "no improvement over the best known rho strategy" | no multi-target rho was run; the "best known" is not established | "no improvement over an idealised, unrun multi-target rho estimate" |
| 24 | "the per-target sweep is O(fb) with fb ~ p^(1/3), which is what grows" | no per-phase profile of the per-target cost | mark as hypothesis; at 33 bits the per-target mean is 4.1 ms (total minus setup, over 1000) against a median of 2.8 to 3.5 ms, so retries and the tail are part of the cost |
| 7 | "c was chosen from exploratory runs on the 24- and 28-bit curves" | uncommitted | "(probe output not committed)" |
| 7 | "Provenance: commit and binary hash" | binary hash prefix only, not reproducible (build-id differs); no host, argv, rustc, dirty state | list the missing fields under Limits |
| 26 | Limits | omits: run on a shared local VM, not isolab-eks, no bench.record found; rho table not right-sized; shared reference measurements across rows; no certificate file; wall-clock only | add |

## 3. Reporting contract

| requirement | verdict | evidence |
|---|---|---|
| boundary named before measuring | partial | rho named; no floor stated; no falsification target in `frozen.json` |
| one table, one unit, ratio columns | **fail** | wall-clock only; the program's S = operations / sqrt(n) is not reported; §6 lists wall-clock as a practicality note, not a headline |
| every arm priced, reference included, §5a band | **fail** (acknowledged in Limits) | no 0 / 0.1 / 1 GAE band for any arm; unpriced counters not listed; no ranking is valid |
| phases separated | partial | setup vs per-target given per row; no per-phase split (factor base, pair table, collection, elimination; combine, sweep, verify) |
| ratio form of progress | pass | ratios to each arm |
| certificate discipline | partial | counts only in the artifacts; independent replay passes (2.3) |
| controls | partial | null/fail-closed control absent from the artifacts (supplied by 2.3); rho arm mis-sized (5.3); cheaper baselines (multi-target BSGS, earlier IC v3, shared-setup ic_solve) not arms |
| m = 83 / ECC2K-130 disclosure | not applicable | prime-field toy curves only; no such claim made |
| execution route (CLAUDE.md: isolab-eks, bench.record, resume) | **deviates** | no record under `benchmarks/records`, no benchmarks/README row, ran on the local sandbox (session log: isolab/S3 unavailable); not stated in RESULTS |
| validation gate (`docs/VALIDATION_GATE.md`) | not met | no `VALIDATION.json`, no §5a band; this report is one required piece |

## 4. Open items for the producer

1. Amend `RESULTS_prime_fast_multi.md` per 2.6 (PR #3636 merged before this report existed; main currently carries the unamended text).
2. Commit the exploratory probe output that chose c, or drop the claim; commit a certificate file or store reference for the 22 000 (Q_t, k) pairs.
3. Re-run the rho arm with a size-appropriate `table_bits` (or report both), and re-state Findings 2 and 3 with the fixed cost paid once.
4. Implement the real DP multi-target rho (and the multi-target BSGS already in `examples/ic_multi_bsgs.rs` on this engine) as measured arms; add the §5a band and an operation-count column.
5. Record argv, host, core count, rustc version and dirty state in `provenance.txt`.
6. Adjacent, not examined here: `RESULTS_prime_fast_vs_rho.md` (single-target ic_solve against rho at 24 to 32 bits) uses the same `rho_parallel` defaults, so the ~2 ms table cost
   inflates its rho time at 24 bits and the quoted small-size gap is probably understated. Check before citing.

## 5. What this validation did not do

No different-host-class or isolab-eks replay; no A/A interval; no ecbench or pcc claim check (`tools/pcc` canary and ledger not run); no timed re-run of the second 30-bit instance or of 31 bits
(correctness of all 22 rows was replayed untimed); no real multi-target rho; no literature lookup. The rho and ic_solve re-samples (5.3, 2.2) are exploratory probes outside the frozen
protocol and do not replace it.

## 6. Reproduction (scratch, not committed)

All under `/tmp/claude-0/-home-user/ecc58476-b907-5471-a536-d3549b06e740/scratchpad/`:
- `bin/prime_fast_multi_producer` (sha256 `0fee411c…`), `bin/prime_fast_multi_rebuilt` (`1da1d65d…`), `build.log`, `build.time`.
- `py/recompute.py` (table arithmetic, 0 mismatches), `py/cmp2.py` (rerun comparison), `py/corrected.py` (5.3), `py/verify_logs.py` (2.3).
- `harness/{dump_multi,curves,rho_probe,s3_probe,mut_check}.rs` compiled with `rustc --edition 2021 -O` against the rebuilt `libcrypto_lib-42993a31e73be122.rlib` in
  `/home/user/crypto/target/release/deps` (`mut/` builds use stand-alone copies `pf_unmut.rs`, `pf_mutA.rs`, `pf_mutB.rs`; sha256 `71eee87f…`, `f5195b26…`, `0f70ab03…`).
- Data: `dump/d_*.jsonl`, `dump/curves.jsonl`, `rerun/rr_*.jsonl` (hashes in 2.3; `rr_23` `0bc88018…`, `rr_26` `24a18bfc…`, `rr_30` `bb9befaf…`, `rr_33` `0f41239c…`).
- Commands: `prime_fast_multi_rebuilt <out> 23|26 1000 1.0 3`, `… 30|33 1000 1.0 1`; `dump_multi <out> <bits> 1000 1.0 <inst>`; `python3 -I py/verify_logs.py dump/`;
  `rho_probe <bits> 100 <res>`; `s3_probe <bits> <N> <res>`; `mut_check_{unmut,mutA,mutB} <bits> 1000 1.0 0`.
