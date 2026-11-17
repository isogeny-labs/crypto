# Validator report: research/genus4_rho_refs_20261116

Disposition: **validated** for the measurement and for the frozen readings as defined: freeze, replay (24 runs identical), independent certificates, gates G1 and G2, every reading Q1–Q4 and Qcal, the cheapest-rho choice and every interval (exact replication). **Headline "arm w below the cheapest rho at w = 0, 0.1 and 1 under Q3": insufficient_evidence at w = 1** (B13: the margin is the reference walk's own per-step ring scan, 89.6% of its Q3 cost, and the same plain walk without that scan reverses it; B12: the owner attribution of Q3 could not be checked). At w = 0 and w = 0.1 the claim holds under every reading tried. Corrections: B13 caveat, B14 qualifier, B16 Qcal description, B17 presentation.

Every number below was recomputed by the validator from the committed raw
rows; the producer's numbers are quoted only to be compared. Every producer
number remains `[pcc:unverified]`; this report is the validator's record, not
a pcc verification.

## Scope, bindings and method (common to both genus-4 studies)

- Validator: `validator` agent (Claude Opus 5.5), dispatched 2026-11-16 by the
  main session; validation run 17:55–20:59 UTC on the shared 4-vCPU VM the
  producer used (Intel Xeon @ 2.30 GHz, AVX2 and AVX-512F present), while
  three `genus_ladder` processes of another study and other sessions'
  release builds and test builds of `crypto_lib` (workflow agents, per the
  coordinator) were running.
- Validated artifact set: the study directories, bench records and the
  r-bound baseline as committed at `62307fead312100cc2e1b133a74bf20ecf70c968`
  (branch `claude/hyperelliptic-index-calculus-td1fdn`, HEAD at dispatch).
  The branch moved during validation (to `a23c1c4ef9`, then further: the main
  session's genus_ladder work and the scale study); `git diff 62307fead3
  HEAD` is empty for every validated path, and all 28 tracked files of the two
  study directories in the working tree equal their blobs at `62307fead3`
  (`out/freeze_check.txt`). Uncommitted edits by another session in `src/`
  were present; no validator build used the working tree.
- Frozen binaries rebuilt from `git archive` of each freeze commit
  (`scripts/paths.py`: manifest, lock, src, path dependencies, files src
  includes; 4,885 paths) into validator-owned trees, with `cargo build
  --release --locked --offline -j2 --bin genus_ladder`, rustc 1.97.0
  (2d8144b78 2026-07-07), cargo 1.97.0, one validator-owned
  `CARGO_TARGET_DIR`, sources touched before each build and the log checked
  for `Compiling crypto v0.1.0 (<that tree>)` (lesson L-ca1840e60dc0). The
  build driver (`scripts/build2.sh`) waits while any other session compiles
  `crypto_lib` and kills and retries its own compile if one starts
  (lesson L-b0626ec2a4c0): 6 yields between 18:21 and 20:57 UTC, because other sessions' release builds and test builds of `crypto_lib` (workflow agents, per the coordinator; up to about 10 GB RSS each) ran almost continuously. Builds that completed: `bce0eb2148` 19:29:47–19:42:44, `a99e7267f1` 20:30:47–20:43:32, the dumper 20:43:32–20:43:37 (same tree, library reused), and the second-path `bce0eb2148` 20:44:04–20:57:00. Deviation: from 19:07 to 20:09 the driver (`scripts/build3.sh`) also allowed starting beside another session's *library* compile and yielded only to test compiles or MemAvailable < 3 GiB. The completed `bce0eb2148` build ran under that rule; no OOM occurred and no foreign compile was seen at the 19:39 check. A `-j1` variant that would have run beside test compiles was refused by the permission system and never ran. From 20:10 the strict rule (`scripts/build2.sh`) applied, as the dispatch requires.
- The producer's binaries in the shared scratchpad were hashed at dispatch,
  before any validator build (lesson L-3f39e1d8b3a9): `genus_ladder.g4`
  `f560318c…` and `genus_ladder.rhorefs` `4b2574a3…`, equal to the two
  `binary.sha256` files.
- Independent arithmetic: `scripts/jac.py` is a validator-written Cantor
  implementation (composition and reduction from Cantor 1987 / Koblitz, NAF
  scalar multiplication, a different addition chain from the producer's two
  ladders) plus a re-implementation of the producer's documented seeding
  (`mix64`, `random_f`, `random_divisor`, `instance_from`). It shares no code
  with `genus_ladder.rs`. Self-test (`scripts/selftest.py`): on nine genus-2
  curves over F_31, F_43, F_101 the group order from independent point counts
  (`#J = (N1² + N2)/2 − p`) kills 90 random divisors; commutativity,
  associativity, inverses and `[a]D + [b]D = [a+b]D` hold, also at genus 4
  over F_2053: PASS.
- `val_g4dump` (`scripts/val_g4dump.rs`) is a validator-written binary built
  from the `a99e7267f1` tree after its frozen `genus_ladder` was built and
  hashed. It prints the producer's instance (f, G, T, t) and, per arm and seed,
  the recovered logarithm with the counted fields, so each recovered log is
  tied to a committed row by equal counts. It also runs a sensitivity walk
  (`Q`: `RhoWalk::plain()` with `cycle_window = 0`), which is the validator's
  and not a registered arm.
- Every script reads record keys with `[]` (lesson L-99ee952350b2). None of
  them imports or calls the producers' `analyze.py`, `redteam/check.rs` or
  certification code. Scripts are in `scripts/`, outputs in `out/`; paths
  below are relative to this `validation/` directory unless they start with
  `research/`, `src/` or `benchmarks/`.
- Not applicable: m = 83 and ECC2K-130 disclosure rules (no binary-field or
  Koblitz target). Not checked: `figure.png`, `figure.svg`, `REPORT.pdf` (renderings of
  numbers checked from the raw rows); the S3 copy of study 1's `run.jsonl`
  (no AWS identity here; the local file hashes to the committed manifest and
  `tools/artifacts/artifacts.py verify` reports 2 hydrated, 0 problems).

## Findings

| # | check | verdict | evidence (command or script → output) |
|--:|---|---|---|
| B1 | Freeze order. The draft protocol and the rho compares counter landed together at `1a3f5a3d46` (16:27:07, after study 1's results and red-team review). The freeze `a99e7267f1` (16:50:51: `frozen.sha256`, `binary.sha256`, `test.sh`, open record) precedes the run start (bench record 16:50:54), `analyze.py` (`4d41fff858`, 16:51:38, 44 s after launch, as RESULTS discloses) and every data commit. The partial data commit `dc3c684916` is a byte prefix of the final file. `run.jsonl` is unchanged from `c490f7ad60` to `62307fead3`. | consistent (the 44 s gap is disclosed; the validator cannot see whether data was read first) | `scripts/freeze_check.sh` → `out/freeze_check.txt` |
| B2 | `frozen.sha256`: the four entries equal `git show a99e7267f1:<path> \| sha256sum`; PROTOCOL, `frozen.sha256`, `binary.sha256`, `test.sh` unchanged from the freeze to `62307fead3`. As in study 1, `ic_boundary.rs` (the rho walk and the compares counter) is not pinned by name; the binary hash (B3) covers it. | consistent | `out/freeze_check.txt` |
| B3 | Binary. Rebuilt from `git archive a99e7267f1`: sha256 **`05bf9177…10a8`**, which is **not** the `binary.sha256` value `4b2574a3…8858`. Same size (6,870,296 bytes); `.text`, `.rodata`, `.data`, `.data.rel.ro`, `.eh_frame` and `.gcc_except_table` are byte-identical. The differences are the build-id and one LLVM-promoted local symbol's hash suffix (`…core4hash3sip…Hasher5write…10crypto_lib.llvm.2985085050382894772` in the rebuild, `….llvm.13893336826588970312` in the producer's). That name is one character shorter, which shifts every later `.strtab` entry and the `.symtab` name offsets (213,035 differing bytes, all in `.note.gnu.build-id`, `.symtab`, `.strtab` and 1 byte outside any section). This is the same mechanism as study 1 A3, where a second-path rebuild of identical sources reproduces it: the code is the frozen code, and the recorded hash is not reproducible bit for bit from another build path. The validator-built binary replays 13 runs identically (B5); the dumper built from the same tree ties instances and logs (B6). | consistent (code identical); the hash binding is not reproducible across build paths | `scripts/build2.sh`, `scripts/build3.sh`, `scripts/binary_diff.sh` → `out/binary_diff_*.txt`, `out/build_evidence.txt` |
| B4 | Data binding. `run.jsonl` (63 lines, 28,052 bytes) hashes to `10c186ca…6cf4`, equal to the bench record's raw entry (`uploaded: false`); `run.log` empty. Bench record: commit `1a3f5a3d46` (the freeze commit's parent, same `src/` and `Cargo.*`), `commit_dirty: false`, `isolation: unstated`, wall 1437.8 s (equal to the `done` row). The config row does not record `--targets 3` or `--calibrate` (the command is in `test.sh` and PROTOCOL §2). | consistent | `out/freeze_check.txt` |
| B5 | Replay. With hash-verified copies of the producer's binary (`4b2574a3…`, equal to `binary.sha256`): arms w and R, 3 curves × 2 targets (`cs` and `cs + 1000003`) × seeds 1–2: **24 runs replayed** (12 w, 12 R; the second target is new data not covered by G1). Every row equals its committed row in every field except `wall_ns`; the 3 instance rows and the cell row are identical. Not replayed: the third targets, the P rows except one (B6, dumper) and the timing-only calibration records. A subset was replayed again with the **validator-built** binary (B3): the 12 w runs of the first two targets and one R run (curve 363, seed 1): 13/13 run rows identical. The only difference is the admission cell row (tried/admitted 1 against 3), because that command admitted one curve instead of three (`out/replay_s2_valbin.out`). | consistent | `scripts/replay_s2.sh`, `scripts/compare_runs.py` → `out/replay_s2_prodbin.out` (RESULT: IDENTICAL, 0 mismatches), `out/replay_s2_valbin.out`; rows in `out/replay_s2_*.jsonl` |
| B6 | Independent certificates for all 9 targets (3 curves × target seeds `cs`, `cs + 1000003`, `cs + 2000006`, the rule in PROTOCOL §2 and the bin): as study 1's A7 (curve from `random_f`, squarefree genus 4, `r` prime, `#J = h·r` proved by the Hasse–Weil uniqueness argument with `[r]G = 0`, G and t re-derived from `instance_from`'s seeding, `[t]G = T` by two chains). All 9 pass. Tie to the producer's instances: `val_g4dump` printed the producer's f, G, T and t for all 9 targets, equal to the validator's derivation (9/9). For 12 w runs on the first targets and 6 on the second targets, recovered = t, `[recovered]G = T` in the validator's arithmetic and every counted field equals the committed row. One P run (curve 363, first target, seed 1) equals its committed row in every counted field, the 683,384,181 compares included. | consistent | `scripts/certs.py` → `out/certs.json`; `scripts/verify_dump.py` → `out/verify_dump.txt`, `out/verify_dump.json` |
| B7 | G1 recomputed: the 6 w rows and 6 R rows for target `cs`, seeds 1–2, equal study 1's rows in **every** shared field except `wall_ns` (34 fields for w, 17 for R; the protocol names 5 and 3). Instance rows are identical across the two studies. | consistent | `scripts/recompute.py` → `out/recompute.json` (`s2_G1`) |
| B8 | G2: 54/54 runs `certified: true`, `ok: true`; w: 0 fallbacks, 0 exhausted, 1 attempt each; no `skip` rows. | consistent | `out/recompute.json` |
| B9 | Readings Q1–Q4 at w = 0, 0.1, 1, Qcal and leave-one-curve-out: every point estimate recomputes from the PROTOCOL §3 definitions; the cheapest rho is R in every cell; every interval in `analysis.json` / `tables.md` reproduces **digit for digit** with the validator's own implementation of the declared bootstrap and seed (`random.Random(20261120)`, targets then seeds, paired). P/R = 1.303 at w = 0, 1.303 (Q1, Q2) and 1.351 (Q3, Q4) at w = 1; compares per step 17.87 (negation), 17.98 (plain); each calibration record's `w_cal` equals its ns ratio; the means are 0.00279, 0.00093, 0.00461. Scorecard P1–P4 as stated. | consistent | `out/claims_check.txt` (113 claims of both studies), `out/exact_vs_analysis.txt` (82 cells, 0 differences) |
| B10 | Display rounding: LOO Q1 "0.40–0.66" (0.398–0.6545; 0.6545 is printed 0.655 in `tables.md` and 0.66 in RESULTS). | consistent (rounding) | `out/recompute.json` |
| B11 | Q2 at w = 1 is borderline between "unresolved" and "above": its lower end is 0.995 with the declared seed but ranges 0.971–1.007 over 30 other seeds at B = 2000. | consistent as computed; the label depends on the RNG seed | `scripts/mc_spread.py` → `out/mc_spread.txt` |
| B12 | **Primary reading chosen after the data.** PROTOCOL §7: "each reading's verdict is reported, none is singled out. Which reading is the lab's is the owner's decision". RESULTS at `c490f7ad60` (17:16) followed that. At `540259a316` (17:45) the producing session added the AGENTS.md §5a bullet "(owner, 2026-11-16)" and rewrote RESULTS to make Q3 the headline. The session log records the owner decision in the session's own words; the validator found no owner-authored artifact and cannot verify the attribution. The timing is disclosed ("The decision followed the genus-4 positive control"). | could not check (owner attribution); disclosed (timing) | `git show 540259a316`; `docs/session-logs/2026-11-16.md` lines 1057–1074 at `62307fead3` |
| B13 | **The Q3 margin at w = 1 is the reference walk's ring scan.** Window compares are 89.6% of negation rho's Q3 cost at w = 1 (K = 9.56e8 of 1.066e9 per run). Arm w is below negation rho at w = 1 exactly when a ring compare is priced above 0.0425 of a modular multiplication. The 16-key scan runs as four 4-lane AVX2 compares on this host (`ic_boundary.rs` 1917–1949). The plain walk carries the same 16-entry window although an r-adding walk on points has no fruitless cycles; the dumper shows that the window never fires on the plain walk (0 short cycles, 0 escapes, 0 capped or abandoned walks in 6 runs at p = 2053 and 1 at p = 8209). With the window off (`cycle_window = 0`) the walk is the same walk: identical recovered logarithm, group operations and steps in all 7 runs, with 0 compares instead of 2.7e7–6.8e8. The window-free readings below are therefore exact for that walk, not estimates. Priced under Q3, the plain walk without the window gives arm w / cheapest rho at w = 1 of **1.049 [0.780, 1.427]** (C_P floored) or **2.098 [1.560, 2.854]** (C_P = 0): not below rho. RESULTS' caveat "a leaner rho walk would narrow the w = 1 margin" understates this: the margin disappears. The window-free walk is not a registered method (`rho.plain` is `RhoWalk::plain()`), so the frozen readings are computed as declared; the w = 1 claim holds only against the lab's registered walks. | **inconsistent** (caveat understated); the w = 1 Q3 claim lacks the cheaper-rho arm | `scripts/sensitivity.py` → `out/sensitivity.txt`; `out/verify_dump.txt` (Q vs P, P walk counters) |
| B14 | **Plain rho's floored canonicalisation.** C for the plain walk is `steps` by the floor in `run_rho_walk` (`genus_ladder.rs` 2030–2035); the walk performs no canonicalisation (`ic_boundary.rs` 2430). The floor is declared in PROTOCOL §3 and P1, so the frozen readings follow it. Without it the plain walk is the cheaper rho at w = 1 in Q1 and Q2: Q1 0.719 [0.535, 0.967] (still below), Q2 2.098 [1.561, 2.821] (above, not "unresolved"). Q3 and Q4 are unchanged (negation stays cheaper: 0.142, 0.123). "Negation rho is the cheapest registered rho everywhere" holds only under the floor. | consistent with the protocol; the statement needs the qualifier | `out/recompute.json` (`… plain unfloored`) |
| B15 | **IC word compares not emitted.** Q3 is called "every emitted native operation on both sides", but emission is not symmetric in granularity: rho emits its 64-bit ring compares, the IC emits none of its own (the `v == 1` and `v == r − 1` tests per LA non-zero in `mat_vec` / `mat_t_vec`, index loads, the walk's per-trial hash and coefficient additions, `col[]` and `HashMap` probes). Estimated LA compares 5.2e7 (`la_add − la_mul`) to 1.2e8 (`2·la_add`) per run. Priced in Q3: w = 1 moves from 0.142 to 0.191 [0.136, 0.294] or 0.255 [0.182, 0.393]; w = 0.1 to 0.131 or 0.175. The verdict at the registered rho is unchanged. Conversely `la_add` includes non-arithmetic bookkeeping (DSU steps, peel weights, adjacency pushes), which charges the IC for work rho is not charged for. | consistent in verdict; asymmetry to disclose | `out/recompute.json` (`Q3+ICcmp_lo/hi`, `Q4+ICwalk`) |
| B16 | Qcal: rho is priced at `G` because "rho's native work [is] timed as part of the Cantor step", but `calibrate()` (bin 653–718) times isolated Cantor additions only, so rho's per-step canonicalisation, hashing, ring scan and coefficient additions are untimed (rho's wall time per group operation, 1,754 ns for R against 1,680 ns per isolated Cantor addition, suggests about 4% per step; informational, shared host). The LA's additions are priced at the mod-p addition rate; PROTOCOL §3 names mod-r rates for the LA and no mod-r addition was timed. The add loop also computes `i % p` inside the timed loop. Both deviations are small and conservative or minor next to 0.008 [0.006, 0.012]. | consistent in direction; deviations to disclose | bin `calibrate()`; `out/recompute.json` (`rho_wall_ns_per_group_op`, `ns_cantor_add_mean`) |
| B17 | Presentation: the Qcal cell is printed in the w = 0.1 column of the readings table (RESULTS and `tables.md`); it is a single calibrated reading, not a w = 0.1 value. "window/Brent compares": Brent detection is off (`WalkPolicy::legacy()`), all are window compares. | inconsistent (presentation; no number changes) | `src/cryptanalysis/ic_boundary.rs` 1776–1782 |
| B18 | Rho reference strength: steps / √(πr/2A) = 1.074 (R) and 1.039 (P) over the 18 runs each, as expected; no curve here has automorphisms beyond the hyperelliptic involution, so negation is the strongest symmetry-based rho. Informational wall time (shared host, 3 threads, no isolation level, not a verdict): arm w / R = 0.0091, P / R = 1.27. | consistent | `scripts/rho_efficiency.py` → `out/rho_efficiency.txt` |

## Accounting: what the readings price and what the code executes

Read from the frozen `a99e7267f1` sources (`src/cryptanalysis/genus_ladder.rs`,
`src/cryptanalysis/ic_boundary.rs`, `src/cryptanalysis/chained_transfer/ic_control.rs`,
`src/bin/genus_ladder.rs`; line numbers at that commit). Study 1's binary
(`bce0eb2148`) differs only in the rho reporting (13 lines of `genus_ladder.rs`)
and the bin; its IC and rho walks are the same code.

**The fields the readings use are what the code counts.**

| field | counted where | what it is |
|---|---|---|
| IC `group_ops` | `CountedGroup::add/double` on `JCurve` | every Cantor step: jump table, walk, restarts, `[t]G` check (the second, independent `mul_rtl` check is uncounted on both sides) |
| IC `rel_mul`, `rel_add` | `Nat` in `run_ic_ext` (1300–1621), `ModU`, `disc_character`, `roots_of`, `vrem` | `mod p` products and sums of the factor base, prefilter, `x^p mod u`, root finding, `v(x)`; plus 1 `mod r` product per relation |
| IC `la_mul`, `la_add` | `build_rows`, `peel`, `wiedemann_coord`/`mat_vec`, `berlekamp_massey`, `res.la.add += dsu.ops` (1609) | `mod r` products and sums of the LA; **`la_add` also counts non-arithmetic bookkeeping**: DSU find steps and unions (`ic_control.rs` 1009, 1022), adjacency pushes, peel weight updates (1113–1157) |
| rho `group_ops` | `rho_walk_with` | steps, look-ahead rejects, escapes, starts, table, verification |
| rho `native` (C) | `canonicalisations_uncharged`, **floored at `steps`** (`genus_ladder.rs` 2030–2035) | negation: one per `rho_canon` call (4 modular negations, 2 key computations of 4 `mix64` each, one compare, selects); plain: the walk does **no** canonicalisation (`ic_boundary.rs` 2430 returns early) and `native = steps` comes from the floor only |
| rho `compares_uncharged` (K) | `work_native` = `cycle_window_word_ops` (`ic_boundary.rs` 2895, 2901, 2947) | 16 key compares for the ring scan + 2 for the push per step (Brent is off: `WalkPolicy::legacy()`); on x86-64 with AVX2 the 16-key scan runs as 4 `vpcmpeqq` + 4 `vpor` + 1 `vptest` (`any_key_equals_avx2`, 1930) |
| rho `hash_uncharged` (H), `coef_adds_uncharged` (D) | not counted by the walk: written by the bin as `steps` and `2·steps` (`src/bin/genus_ladder.rs` 441–443) | closed-form estimates, not counters; the walk computes a key and two coefficient `addmod`s per tried jump (look-ahead rejects and escapes included), so for negation rho both are about 3.5% low (`walk_ops / steps` = 1.0345) |

**Executed native work that no counter captures.**

Rho (both walks):
- the per-step key `g.key(&y)` (`key_of`: 4 `mix64` at genus 4) and `mix(ky)`
  (2864–2865); under negation 2 more `key_of` per canonicalisation (2444):
  about 5 (plain) or 13 (negation) 64-bit mixes per step, priced as 1 (H) in Q4
  and 0 in Q1–Q3;
- the two coefficient `addmod`s per step (2858–2859), D in Q4 only;
- the canonicalisation's 4 modular negations and 2 coefficient negations,
  inside the single counted `native` unit;
- the look-ahead test, the distinguished-point test, the identity test, DP
  table inserts and look-ups (rare).

Index calculus (arm w):
- per trial: the walk's jump index `mix64(key_of(cur))` (5 mixes; 1406) and
  two `addm` on the coefficients (1409–1410), the analogues of rho's H and D,
  which Q4 charges rho but not the IC; ~1.7e5 trials per run, so
  `Q4+ICwalk` moves w = 1 only from 0.123 to 0.124;
- per LA non-zero visit in `mat_vec` / `mat_t_vec`: the coefficient tests
  `v == 1` and `v == r − 1` (`ic_control.rs` 1191, 1193, 1214, 1216) and the index
  load. These are word compares of the kind Q3 prices on rho's side. The
  visits are not emitted; the validator estimates them from `la_add − la_mul`
  (lower, 5.2e7 per run) to `2·la_add` (upper, 1.2e8 per run) against rho's
  K = 9.6e8 per run;
- per root `col[x]` look-ups, per large prime a `HashMap` probe (1459, 1492),
  per candidate the `u`/`v` coefficient copies.

**Readings, recomputed with these additions (validator sensitivities,
`scripts/recompute.py`, `scripts/sensitivity.py`; bootstrap as the protocol,
B = 4000, own seeds):**

| reading | w = 0 | w = 0.1 | w = 1 |
|---|---|---|---|
| Q3 as frozen (validator recompute) | 0.003 [0.002, 0.005] | 0.098 [0.070, 0.151] | 0.142 [0.101, 0.219] |
| Q3 + IC LA compares, low estimate (`la_add − la_mul`) | 0.003 | 0.131 [0.094, 0.202] | 0.191 [0.136, 0.294] |
| Q3 + IC LA compares, high estimate (`2·la_add`) | 0.003 | 0.175 [0.125, 0.269] | 0.255 [0.182, 0.393] |
| Q4 + IC walk hash and coefficient adds | 0.003 | 0.089 [0.064, 0.137] | 0.124 [0.088, 0.191] |
| Q3 with each ring compare at 1/4 (one op per 4-lane vector compare) | 0.003 | 0.180 [0.129, 0.282] | 0.433 [0.307, 0.677] |
| Q3, plain walk without its window (K_P = 0), C_P floored | 0.003 | 0.193 [0.143, 0.262] (P) | **1.049 [0.780, 1.427] (P)** |
| Q3, plain walk without its window, C_P = 0 (unfloored) | 0.003 | 0.212 [0.158, 0.288] (P) | **2.098 [1.560, 2.854] (P)** |
| Q1 with C_P = 0 (unfloored) | 0.003 | 0.088 [0.066, 0.135] (R) | 0.719 [0.535, 0.967] (P) |
| Q2 with C_P = 0 (unfloored) | 0.003 | 0.251 [0.188, 0.388] (R) | 2.098 [1.561, 2.821] (P) |

- Window compares are 89.6% of negation rho's Q3 cost at w = 1 (K = 9.56e8
  of 1.066e9 per run). The break-even weight per compare is 0.0425: Q3 at
  w = 1 has arm w below negation rho exactly when a ring compare is priced
  above 0.0425 of a modular multiplication.
- The plain walk does the same 16-key scan every step although an r-adding
  walk on points has no fruitless cycles; the dumper confirms that the window never fires on the plain walk and that turning it off leaves group operations, steps and the recovered log unchanged (7 runs at p = 2053 and 8209, `out/verify_dump.txt`). The two window-free rows above are exact for that walk. A negation walk needs cycle handling, but the per-step 16-key scan is a design choice: the literature checks for fruitless cycles periodically (Bernstein, Lange, Schwabe 2011; Bos, Kleinjung, Lenstra 2010). That is the validator's reading, not a measurement here.

## Why there is no VALIDATION.json

`docs/VALIDATION_GATE.md` defines `VALIDATION.json` for a row that is to reach
the IC leaderboard or `benchmarks/README.md`. It does not fit here, so none is
written:

- Location: the manifest must sit in the row's evidence directory (the
  directory of the row's `source`, or `benchmarks/<dir>/`), and its paths are
  relative to that directory. This validator may write only under
  `validation/`, which is not an evidence directory of any row.
- Status: the pinned validator report must say `status: "pass"`, with every
  headline's `claimed` equal to `recomputed` within 1e-9. The measurement
  passes, but study 1 carries an inconsistent figure (A14) and study 2's
  w = 1 headline is `insufficient_evidence` (B13). The printed headlines are
  also rounded to three decimals; the gate's own advice is to claim the
  recomputed numbers, which would be the validator restating the producer.
- `rho_reference.gae_band` holds one IC/rho pair per weight. Study 2's verdict
  depends on the reading (Q1–Q4, Qcal) and, at w = 1, on the rho
  implementation (B13, B14). One pair would hide that, and
  `order_holds_across_band` would be true or false depending on which
  reading is chosen.
- `independent: true` is, under `docs/SABOTAGE_RESISTANCE.md`, backed by a
  keyring signature that this validator does not have.

A main session that admits a row can build the manifest from this report:
the replay outputs in `out/` and the recomputation in `out/recompute.json`.

## What may be cited

- The measurement: 54 runs and 3 calibration records, every logarithm the
  planted one, 24 of them replayed identically (13 again with the validator-built binary), the 9 targets certified by independent
  arithmetic, gates G1 and G2 as stated, and every number of the readings
  table, the LOO table, the P/R ratios and the scorecard as printed (B1–B10).
- Arm w is below the cheapest registered rho at w = 0 and w = 0.1 under every
  reading, the validator's sensitivities included (largest upper end 0.393
  with the IC's LA compares priced; 0.288 against a window-free plain walk).
- At w = 1 the verdict depends on what the band prices. Under the frozen Q3
  definition against the registered walks: 0.142 [0.102, 0.215], recomputed
  exactly. That margin is the reference walk's per-step 16-key ring scan
  (89.6% of negation rho's Q3 cost; break-even weight 0.0425 per compare).
  Against the same plain walk without that scan (B13) the reading is 1.049
  [0.780, 1.427] or 2.098 [1.560, 2.854]. Until a rho reference without the
  per-step window (and a negation walk with periodic, not per-step, cycle
  checks) is registered and priced, "below rho at w = 1 under Q3" is a
  statement about the lab's rho implementation, not about rho, and the
  validator does not validate it as a band verdict.
- Q2 at w = 1 is "unresolved" with the declared seed and "above" with some
  other seeds (B11); unfloored it is above (B14).
- The owner attribution of the Q3 primary reading (B12) is the producing
  session's record; the validator could not check it.
