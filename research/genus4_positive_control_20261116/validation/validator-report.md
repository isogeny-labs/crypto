# Validator report: research/genus4_positive_control_20261116

Disposition: **validated** for the measurement: freeze, replay (132 runs identical, with the producer's and with the validator-built binary), independent certificates, gates G1 and G2, and every table number and interval. **Required corrections:** A14 (the "1.057" figure is mislabelled as additions priced on both arms) and A15 (relation share is of multiplications only). **Binding note:** A3: `binary.sha256` does not reproduce bit for bit, because the build is path-dependent in one LLVM symbol hash; the code sections are identical. The w = 1 band statement is a sensitivity under the current §5a rule and is superseded by study 2 (A17).

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
| A1 | Freeze order. `bce0eb2148` (15:41:00 UTC: PROTOCOL, `frozen.sha256`, `binary.sha256`, `test.sh`, open bench record) precedes the run start (bench record `started_utc` 15:41:14), `analyze.py` (`16b409c747`, 15:41:54, 40 s after launch, as RESULTS discloses) and every data commit. `analyze.py` has one commit and is unchanged since. The five partial `run.jsonl` commits (15:42:12 to 15:53:12) are byte prefixes of the final file. | consistent (the 40 s gap is disclosed; the validator cannot see whether data was read before `analyze.py` was written) | `scripts/freeze_check.sh` → `out/freeze_check.txt` |
| A2 | `frozen.sha256`: the four entries equal `git show bce0eb2148:<path> \| sha256sum`; PROTOCOL, `frozen.sha256`, `binary.sha256` and `test.sh` are unchanged from the freeze to `62307fead3`. `frozen.sha256` does not pin `ic_boundary.rs` or `residual_walk.rs` (the rho walk and `mix64`); the binary hash (A3) covers them. | consistent | `out/freeze_check.txt` |
| A3 | Binary. Rebuilt from `git archive bce0eb2148`: sha256 **`2b58ab47…37f2`**, which is **not** the `binary.sha256` value `f560318c…0b4d` (the producer's binary, hashed at dispatch). The two files have the same size (6,858,320 bytes) and differ in 36 bytes: the 20-byte GNU build-id and a 16-byte hash suffix of one LLVM-promoted local symbol in `.strtab` (`_RNvMs4_…RawVecInner11finish_grow…10crypto_lib.llvm.11676721725037927419` in the rebuild, `….llvm.17669595450908921892` in the producer's). `.text`, `.rodata`, `.data`, `.data.rel.ro`, `.eh_frame` and `.gcc_except_table` are byte-identical, and the rebuilt binary replays the same 132 runs identically (A6). So the code the producer ran is the code of the frozen sources, but `binary.sha256` does not reproduce bit for bit. The suffix is LLVM's hash of the `crypto_lib` codegen unit holding that function. It changes when anything in that unit changes, including code this binary does not link and absolute paths (the rebuilt `crypto_lib` rlib carries the tree's absolute path 1,822 times; the linked binary carries none). Either the producer's library was compiled from a tree that differed from `bce0eb2148` outside the linked code (the worktree is shared with other sessions), or the hash depends on the build path. **Settled:** the same `bce0eb2148` tree copied byte for byte to a second path and rebuilt gives a third hash, `81408d2f…09ef`. It differs from the first rebuild in exactly the same way: the build-id plus the same symbol's suffix, `….llvm.13384214426060252674`; code sections identical (`out/binary_diff_bce0eb2148_path2.txt`). The build is not reproducible across build paths, or across builds, in this one ThinLTO symbol hash. A same-path rebuild would separate the two; it was not run. A `binary.sha256` taken at one path therefore cannot be reproduced bit for bit from another, and the mismatch is not evidence of different sources. | consistent (code identical); the hash binding is not reproducible across build paths | `scripts/build2.sh`, `scripts/build3.sh`, `scripts/binary_diff.sh` → `out/binary_diff_*.txt`, `out/build_evidence.txt` |
| A4 | Data binding. `run.jsonl` is store-managed (not in git): the local file (158 lines, 78,049 bytes) hashes to `e0f7050d…a2efa32f`, equal to `.artifacts.json` at `62307fead3` and to the bench record's raw entry (`uploaded: false`). `run.log` is empty (`e3b0c442…`), as the record says. | consistent; S3 copy not checked | `out/freeze_check.txt` |
| A5 | Bench record `benchmarks/records/20261116T1540Z-genus4-positive-control.json`: commit `2ca7ece552` (the freeze commit's parent; no `src/` or `Cargo.*` difference to `bce0eb2148`), `commit_dirty: false`, `isolation: unstated`, wall 717.7 s (equal to the `done` row). Its metric `n = 12` counts runs, not the 3 independent targets, and its problem id is the genus-3 control's (both disclosed in RESULTS, F6). | consistent (disclosed) | `out/freeze_check.txt` |
| A6 | Replay. With hash-verified copies of the producer's binary (`f560318c…`, equal to `binary.sha256`; copied before any validator build): **132 runs replayed** — p = 2053 and 4099 completely (arms c, v, w, R: 96 runs, 24 of them rho) and p = 8209 arms c, v, w (36 runs). Every replayed run row equals its committed row in **every field except `wall_ns`**, and the 9 instance rows and 3 cell rows (except `search_wall_s`) are identical. The same 132 runs replayed a second time with the **validator-built** binary (`2b58ab47…`, A3): identical again. Not replayed: the 12 R rows at p = 8209 (about 100 s each); 6 of them (seeds 1–2) are reproduced by study 2's replay of the same walk on the same targets (study 2 B5 and G1). | consistent | `scripts/replay_s1.sh`, `scripts/compare_runs.py` → `out/replay_s1_prodbin.out`, `out/replay_s1_valbin.out` (RESULT: IDENTICAL, 0 mismatches, in all four comparisons); rows in `out/replay_s1_*.jsonl` |
| A7 | Independent certificates for the 9 instances (target seed = curve seed): `f = random_f(p, 4, cs)` equals the instance row and the r-bound row; f squarefree of degree 9 (genus 4); `r` prime (deterministic Miller–Rabin); `order = h·r`, `h ≤ 2`; `h·r` in the exact Hasse–Weil interval `[(√p−1)^8, (√p+1)^8]` and `r` larger than the interval's width, so with `G ≠ 0`, `[r]G = 0` and `r` prime, `#J = h·r` is proved; `[order]D = 0` for 4 further random divisors; G and t re-derived from `instance_from`'s seeding, `[t]G` by two addition chains. All 9 pass. Tie to the producer's instances: `val_g4dump` (built from the `a99e7267f1` tree, whose instance and IC code equal `bce0eb2148`'s) printed the producer's f, G, T and t for all 9 targets; every coefficient equals the validator's derivation (9/9). For 72 IC runs at p = 2053, 4099 (arms c, v, w, seeds 1–4), 12 w runs at p = 8209 and 6 R runs at p = 2053 (seeds 1–2), the dumped recovered logarithm equals t, `[recovered]G = T` holds in the validator's arithmetic, and every counted field equals the committed row (90/90). That ties the rows' `ok: true` to a logarithm the validator verified independently. | consistent | `scripts/jac.py`, `scripts/certs.py` → `out/certs.json`; `scripts/verify_dump.py` → `out/verify_dump.txt`, `out/verify_dump.json` (19 instance lines, 110 runs, 0 failures over both studies) |
| A8 | G1 recomputed: the 9 instances equal the r-bound study's (order, h, r, f), and the 18 c rows and 18 R rows with seeds 1–2 equal the r-bound rows in **every** shared field except `wall_ns` (33 fields for c, 17 for R; the protocol names 5 and 2). The match set is non-empty (18 + 18), so the gate cannot pass vacuously. | consistent | `scripts/recompute.py` → `out/recompute.json` (`s1_G1`) |
| A9 | G2: 144/144 runs `certified: true` and `ok: true`; no `exhausted`, no `skip`, no second attempt, 0 fallbacks. The gate row says "`[t]G = T` twice" for all four arms; the rho arm checks once (`run_rho`, `genus_ladder.rs` 2006–2010 at `bce0eb2148`), as RESULTS discloses lower down (F6). | consistent (wording of the gate row imprecise for R) | `out/recompute.json` (`s1_G2_fail` empty) |
| A10 | G3 (two unit tests at the frozen commit). Re-executed through the validator's dumper rather than `cargo test`: `val_g4dump` mode `G3` makes the same calls and checks the same assertions as `ext_default_is_run_ic_count_for_count` and `ext_arms_certify_at_genus_3_and_4` (`genus_ladder.rs` 2036–2125 at `bce0eb2148`) on the `a99e7267f1` library, whose IC code equals `bce0eb2148`'s: **12/12 pass**. Default `IcExt` equals `run_ic` count for count at (131, 3), (131, 4), (257, 3); coordinate Wiedemann, unit-T and both certify at genus 3 and 4 with 0 bad relations (`verify_relations: true`); the coordinate solve keeps the relations and spends fewer LA multiplications. Not run as `cargo test` at the frozen commit: a release test build of `crypto_lib` peaks near 10 GB on this host, and other sessions ran such builds back to back. | consistent (same assertions, not the test harness) | `scripts/val_g4dump.rs` (`fn g3`) → `out/dump_g3.jsonl` |
| A11 | Components table: all 15 numbers (LA v/c, LA w/c, relation w/c, relation share, fallbacks at 3 primes) recompute; the 3 printed LA w/c intervals and every interval in `tables.md` and `analysis.json` reproduce **digit for digit** when the validator's own bootstrap uses the producer's declared design and seed (`random.Random(20261119)`, curves then seeds). | consistent | `scripts/recompute.py`, `scripts/exact_intervals.py` → `scripts/compare_analysis.py` → `out/exact_vs_analysis.txt` (82 cells of both studies, 0 differences) |
| A12 | Band table (21 cells and FPM): every point estimate recomputes to the printed digits; every interval reproduces exactly with the declared seed. With the validator's own seed (B = 4000) the ends move by at most 0.03 and no end crosses 1. | consistent | `out/claims_check.txt`, `out/exact_intervals.txt`, `out/mc_spread.txt` |
| A13 | Prose numbers: v/c total 0.705–0.754 ("0.71–0.75"), w/c 0.661–0.705 ("0.66–0.70"); w's modular additions 9.91e7 per run at 8209 ("9.9e7"), 1.926× its priced multiplications (printed "1.92×", truncated); the estimated symmetric reading 0.630 ("about 0.63"); leave-one-curve-out upper bounds 0.73–0.88 with the validator's seed ("0.74–0.88"); v's w = 1 upper end 0.809 ("0.805"), c's 1.146 ("1.142"); w at w = 1 falls 1.018 → 0.708 → 0.528; FPM 0.011; P1–P3, P5 outcomes; LA w/v 0.705 in multiplications, 0.933 with additions; the unit-T cap is at least 187,590 > 4096 ("≥ 1.9e5"). | consistent (rounding only) | `out/claims_check.txt`, `out/recompute.json` |
| A14 | **Mislabelled sensitivity.** RESULTS ("Additions unpriced (F2)" bullet and the P4 scorecard row) says "Pricing multiplications **and** additions on both arms as emitted: w/R at w = 1 is **1.057 [0.765, 1.499]**". Recomputed, 1.057 is `IC (G + M) / rho (G)`: multiplications only, with rho's canonicalisations priced 0 (the red team's row "multiplications only on both arms (rho native = 0)"). IC multiplications and additions against rho as emitted is **1.543 [1.125, 2.218]** (red team: 1.543 [1.117, 2.192]); adding rho's estimated hash and two coefficient additions per step gives 0.630. The conclusion drawn (w = 1 unresolved without symmetric pricing) still holds, but the number cited for "additions on both arms" is the wrong reading. | **inconsistent** | `scripts/f2_check.py` → `out/f2_check.txt` (with the declared seed: IC(G+M)/rho(G) = 1.0568 [0.764, 1.508]; IC(G+M+A)/rho(G+C) = 1.5426 [1.116, 2.200]; with rho + 3 per step 0.6296) |
| A15 | "The relation phase is now 84–88% of w's native work" ("What is next"): 0.844–0.879 is the share of w's native **multiplications** (as the components table header says). Counting additions, which AGENTS.md §5a now prices (owner bullet of 2026-11-16), it is 0.548–0.655. | inconsistent (wording) | `scripts/f2_check.py` → `out/f2_check.txt` |
| A16 | Bootstrap design: `analyze.py` draws numerator and denominator seeds independently within each resampled curve, while PROTOCOL §5 says "paired on (curve, seed)". The paired bootstrap gives the same verdicts (e.g. w/R at w = 1, p = 8209: paired 0.528 [0.381, 0.746], unpaired 0.528 [0.384, 0.759]). | consistent in effect (minor deviation from protocol wording) | `out/recompute.json` (`… paired` keys) |
| A17 | Pricing status. This protocol priced IC multiplications against rho's floored canonicalisations (`total()` in `analyze.py`), a reading that AGENTS.md §5a (owner bullet dated 2026-11-16, added after this study) now allows only as a sensitivity. This binary's rho rows do not emit the window compares, so the study cannot give the primary reading; its w = 1 statement at p = 8209 is superseded by `research/genus4_rho_refs_20261116` (see that report). RESULTS already reports w = 1 as unresolved under symmetric pricing; w = 0 and w = 0.1 are below rho under every reading tried here and in study 2. | consistent (disclosed) | `research/genus4_rho_refs_20261116/validation/validator-report.md` |
| A18 | Accounting (counters vs executed work): see the shared section below and study 2's report, which covers the same code. | see below | `scripts/sensitivity.py`, code references |

## Accounting: what the readings price and what the code executes

The IC and rho code of `bce0eb2148` is the code of `a99e7267f1` except the
rho reporting (`genus_ladder.rs` 1971–2030 at `bce0eb2148`: this binary emits `native` but not
the walk's `work_native`), so the full counter-by-counter list is in
`research/genus4_rho_refs_20261116/validation/validator-report.md`
("Accounting"). For this study:

- The protocol prices IC `rel_mul + la_mul` and rho `native` (canonicalisations,
  floored at one per step). Both are what the code counts (`run_ic_ext`
  `Nat` tallies; `canonicalisations_uncharged` with `.max(rr.steps)`, line 2025 at `bce0eb2148`).
- Unpriced and counted: IC `rel_add + la_add`, 9.91e7 per run at p = 8209
  (1.926× the priced multiplications; `la_add` includes DSU, peel and
  adjacency bookkeeping, not only modular additions).
- Executed, not emitted by this binary: rho's 16 + 2 ring compares per step
  (17.9 per step in study 2's emission of the same walk; 89.6% of negation
  rho's cost at w = 1 when priced, study 2 B13), its per-step key hash
  (4–5 `mix64`, 13 under negation) and two coefficient `addmod`s; the IC's
  per-LA-non-zero coefficient tests, per-trial walk hash and coefficient
  `addm`, `col[]` and `HashMap` probes.
- FPM prices rho at `cantor_mul + 10·cantor_inv` only (rho's native work at 0)
  and the IC's LA multiplications at g² = 16 field multiplications each, as
  PROTOCOL §5 defines; recomputed as defined (A12).
- The certificate's second check (`mul_rtl`) is outside `group_ops` on both
  sides; rho certifies once (A9).

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

- The measurement: 144 runs, every logarithm the planted one, 132 of them replayed identically with both the producer's and the validator-built binary,
  the instances certified by independent arithmetic, gates G1 and G2 as
  stated, and every table number and interval as printed (A6–A13).
- The components: coordinate Wiedemann cuts w's and v's LA multiplications to
  0.330–0.331× of arm c, and with the unit-T walk to 0.222–0.233×, without a
  fallback; arm w's total is 0.661–0.705× of c at w = 0.1 and 1 (all
  intervals below 1). Under the 2026-11-16 §5a reading the LA change is
  mostly multiplications becoming additions (w/v LA 0.705 in multiplications,
  0.933 with additions; red-team F5), so the component claim should be
  quoted in multiplications with that qualifier.
- Against negation rho: below at w = 0 and w = 0.1 at every prime, under
  every pricing tried. At w = 1 the protocol's multiplications-only reading
  (0.528 [0.382, 0.754] at p = 8209) is a sensitivity under the current §5a
  rule, not a verdict; RESULTS already calls w = 1 unresolved.
- Before citing RESULTS: replace "1.057 [0.765, 1.499]" as the "additions on
  both arms" figure (A14; it is the multiplications-only, rho-native-0
  reading; IC additions against rho as emitted is 1.543 [1.125, 2.218]), and
  say "84–88% of w's native multiplications" (A15). Per lesson L-8d86e8fc352c
  the correction belongs in a sibling ERRATA.md, leaving RESULTS byte-equal.
