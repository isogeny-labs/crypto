# Validator report: research/ic_positive_control_lucas_20261116

Disposition: **validated (measurement, gates and ratios); RESULTS.md needs corrections L4, L11, L13; X2 and X3 must be disclosed**

Every number below was recomputed by the validator from the committed raw
rows; producer numbers are quoted only to be compared. Paths are relative to
this `validation/` directory unless they start with `research/` or
`benchmarks/`.

## Validation scope and bindings (common to the four positive-control studies)

- Validator: `validator` agent (Claude Opus 5.5), dispatched 2026-11-16 by the main
  session; validation run 14:50–16:35 UTC on the same shared 4-vCPU VM the
  producer used.
- Validated commit: `79c91dd9136662ed4ee007852ca5bc6abe415c6d` (branch
  `claude/hyperelliptic-index-calculus-td1fdn`, HEAD at dispatch, `git status`
  clean). The main session committed `616f0eb5c4` and `2ca7ece552` (genus_ladder
  only) during validation; neither touches a study directory, the instrument
  or the bench records. `scripts/pin_check.sh` hashed all 81 tracked files of the
  five study directories and the four bench records in the working tree against
  their blobs at `79c91dd913`: all equal (`out/pinned_sha256_79c91dd913.txt`).
- Every script named below is in this directory's `scripts/` (copied from the
  validator scratchpad); their outputs are in `out/`. None of them imports or
  calls the producer's `analyze.py`, `train_score.py` or certification code.
  Every row key the scripts read is indexed with `[]` (lesson L-99ee952350b2).
- Rebuild of the frozen binaries: each freeze commit's build inputs were
  extracted with `git archive` (`scripts/build_paths.py`: manifest, lock, src,
  path dependencies, files src includes) into a validator-owned tree and built
  with `cargo build --release --locked --offline`, rustc 1.97.0 (2d8144b78
  2026-07-07), cargo 1.97.0, in one validator-owned `CARGO_TARGET_DIR`
  (`scripts/build_pair.sh`: the frozen `ic_positive_control` and the validator's
  `val_dump` from one compilation of that tree's library). Rebuilt hashes:
  `81ddcdc450` → `6a97ea07…`, `551590e3b7` → `cd574bde…`, `caae5eca9f` →
  `d611851a…` (each **bit-identical** to the study's `binary.sha256`);
  `4feb188f47` → `6f822439…` (no recorded hash). Errors handled: a first
  build at 15:05 was killed by the cgroup OOM killer while another session's
  `cargo test --release` compiled the same crate (builds then waited for the
  host to be free, `scripts/build_when_free.sh`); at 15:50 two "builds" took
  0.17 s and copied another tree's binary, because the trees share cargo's
  location-independent metadata hash and `git archive` sets old mtimes. Those
  two copies were quarantined in the validator scratchpad and not used
  (hashes in `out/build_evidence.txt`); every binary
  used below comes from a build whose log shows `Compiling crypto v0.1.0
  (<that tree>)`, after touching that tree's sources.
- `val_dump` (`scripts/make_dumper.py`) is the frozen commit's own
  `src/bin/ic_positive_control.rs` with extra JSON fields only: the recovered
  log, the instance's secret, f, G and T, and for rho rows a second direct
  call of `rho_reference_walk` exposing its counters and `work_native`. Its
  counted fields must equal the frozen binary's (checked row by row), which
  ties each recovered log to the committed row.
- A second replay used the producer's own hashed binaries from the shared
  scratchpad (copied before anything rebuilt them; lesson L-3f39e1d8b3a9) for
  the LA, unit-T and capped studies (the Lucas study recorded no binary):
  `out/producer_bin_replay/` (commands in `commands.txt`), same results.
- Independent arithmetic: `scripts/jac3.py` is a validator-written Cantor
  implementation (textbook composition and reduction for `y² = f(x)`,
  `deg f = 7`) with a self-test (commutativity, associativity, inverse,
  `[5]A`) and an independent re-implementation of the documented curve
  generator (`random_curve`, `mix64`). It shares no code with `hyper64.rs`.
- Not in scope / not applicable: m = 83 and ECC2K-130 disclosure rules (no
  binary-field or Koblitz target here); `figure.png` (all four studies and
  their `benchmarks/` copies) is store-only (`.artifacts.json`) and was not
  hydrated in this container, so the PNGs were not checked; the PDFs and SVGs
  are present but were not checked (they are renderings of numbers checked
  from the raw rows).


## Findings

| # | check | verdict | evidence (command / script → output) |
|--:|---|---|---|
| L1 | Freeze order: `4feb188f47` (01:22:10, PROTOCOL + frozen.sha256 + instrument) precedes every data commit (`82b3d37a39` analysis script 01:24:26, `effac24c61`…`0867916552` partial logs, `ba0c9c1233` results 01:34:02). Run start = run.json mtime − `wall_s` = 01:22:42.2, 32 s after the freeze commit; the bench record's `started_utc` is 01:22:42. | consistent | `scripts/freeze_check.sh` → `out/freeze_check.txt` (commit log; run start = mtime − `wall_s`) |
| L2 | `frozen.sha256` entries (PROTOCOL.md, ic_control.rs, ic_positive_control.rs) equal `git show 4feb188f47:<path> \| sha256sum`; the instrument blobs are unchanged from the freeze commit to `79c91dd913`'s ancestor `ba0c9c1233`. | consistent | `scripts/freeze_check.sh` → `out/freeze_check.txt` |
| L3 | Binary binding: no `binary.sha256`; the run used `target/release/ic_positive_control` from the shared target dir, and the bench record says `commit_dirty: true` at 01:22:34. The binding of the measured binary to the frozen commit therefore rests on the replay (L9), not on a recorded hash. | partially consistent (binding established only by replay) | `benchmarks/records/20261116T0122Z-ic-positive-control-lucas.json` (`commit`, `commit_dirty`, `commands[0].argv`) |
| L4 | Undisclosed removal of a frozen-commit file: `4feb188f47` added `research/ic_positive_control_lucas_20261116/uncertainty.py` (byte-identical to the 11-12 control's: a **one-stage** run-level bootstrap); the results commit `ba0c9c1233` deleted it. PROTOCOL §5 calls it "the 11-12 control's two-stage bootstrap (`uncertainty.py`, curves then seeds…)"; the intervals in RESULTS come from `analyze.py`'s two-stage bootstrap. Point estimates are unaffected; the two-stage intervals are the wider (more conservative) choice. | inconsistent (protocol text vs frozen file; deletion not disclosed) | `git show 4feb188f47 --stat=200`; `git show ba0c9c1233 --stat=200 \| grep uncert`; `diff <(git show 4feb188f47:research/ic_positive_control_lucas_20261116/uncertainty.py) research/ic_positive_control_20261112/uncertainty.py` |
| L5 | Raw files: `run.json` (576 rows) and `run.log` hash to the bench record's `raw[].sha256`; `uploaded: false` (no S3 copy, disclosed in PROTOCOL §8). `figure.png` is store-only, not hydrated, not checked. | consistent | `out/bench_records_check.txt` |
| L6 | Accounting, every row: `total_w{0,0.1,1} = group_ops + w·native_mul`; IC `native_mul = rel_mul + la_mul`, `native_all = rel_mul + rel_add + la_mul + la_add`, `la_mul = la_graph_mul + la_wied_mul`, `group_ops = setup + walk + verify`; rho `group_ops = walk_ops + setup_and_other_ops`, `native_mul ≥ steps`; no row exhausted. 0 violations in 576 rows. | consistent | `python3 -I scripts/validate_lucas.py /home/user/crypto out/lucas_recompute.json` → `accounting_violations: []` |
| L7 | G1 (C and R rows equal the 11-12 control's): 288 rows compared (144 C, 144 R), 0 differences in the protocol fields **and** in every other deterministic field. G2 (L equals C in trials, splits, relations, group ops, LA mul and add, with lower rel_mul): 144 pairs, 0 failures. G3: 576/576 rows certified (`all_certified: true`). | consistent | `out/lucas_recompute.json` → `G1`, `G2`, `G3` |
| L8 | G2's "and the recovered log" clause cannot be checked from `run.json` (rows carry no recovered value; `analyze.py` does not check it). Checked instead on the replayed cells (L9): on all 100 replayed (p, curve, seed) at p ≤ 2053, L's recovered log equals C's, and both equal M's, R's and the instance's secret mod r. Not checked at p ≥ 4099 (not replayed). | consistent at p ≤ 2053; could not check at p ≥ 4099 | `out/certs_lucas_CLMR.json` |
| L9 | Replay certificate (p ≤ 2053, all 5 curves × 4 seeds, arms C L M R): the binary rebuilt from `4feb188f47` (sha256 `6f822439…`; no recorded binary hash to compare) reproduces all 400 rows of run.json at p ≤ 2053 in every deterministic field (all but `wall_ns`, `total_cal`): 400/400 identical. This binds the dirty-tree run to the frozen sources at these primes. At p ≥ 4099 the C and R rows equal the 11-12 control's and the M rows equal the LA study's (whose binary is bound to its freeze commit by hash, A3/A16), so only the L rows at p ≥ 4099 rest on this run alone. | consistent (p ≤ 2053, 400/400) | `scripts/replay_all.sh lucas` → `out/replay_lucas_CLMR.json`, `out/certs_lucas_CLMR.json` |
| L10 | Independent log check on the same cells: 400/400 rows pass: f is the documented generator's output for curve seeds 5, 6, 16, 19, 20 (p = 131) … (all 25 curves found in order), squarefree; G and T reduced divisors on the curve; G ≠ 0, [r]G = 0, r prime, r > h, h·r = order inside the Hasse–Weil interval, [order]D = 0 for 4 random divisors; [t]G = T for the recovered t of every C, L, M and R row, and t ≡ t_secret (mod r). The validator's val_dump rows equal the frozen binary's rows (400/400), so each checked log belongs to the committed row. Negative control: corrupting a log, T, f, G or a counted field is caught (`out/negctl_verify.txt`). The 43 test and training instances at every prime up to 131101 were also re-derived from the generator with the validator's arithmetic (curve seed, [order]D = 0, h smallest cofactor; `out/instances_check.json`, negative control `out/instances_negative_control.txt`). | consistent (p ≤ 2053); p ≥ 4099 rows carry no recovered log and were not re-verified | `scripts/verify_certs.py` (uses `scripts/jac3.py`) → `out/certs_lucas_CLMR.json` |
| L11 | RESULTS "G3 … 696 runs": run.json has **576** rows (4 arms × 144: 5+5+5+5+5 primes × 20, 2 × 12, 2 × 8, 1 × 4). | **inconsistent** (count misstated; every row is certified) | `out/lucas_recompute.json` → `n_rows`, `n_per_arm` |
| L12 | Per-trial table (C, L, M at 1031, 4099, 16411, 32771, 65537), L/C and M/C point estimates (ratio of mean rel_mul), C/R, M/R and L/R at w = 0.1 and 1, C/R at w = 0 (0.11 at 65537), P1–P5 values: 48 of 49 transcribed numbers match at their printed precision. | consistent (48/49) | `out/lucas_recompute.json` → `claim_checks` |
| L13 | RESULTS "LA share … C 0.36, L 0.54, M 0.74 (p = 32771)": L is **0.556** at 32771 (0.538 at 65537); the producer's own `tables.md` prints 0.56. | **inconsistent** (transcription; C 0.362 and M 0.743 match) | `out/lucas_recompute.json` → `per_p[p=32771].la_share_L` |
| L14 | "M … 0.20–0.22× where a single-exponent cube root exists": holds for p ≥ 1031 (0.196–0.223); at the cube-root primes 131 and 257 M/C is 0.304 and 0.295. "L is 0.45–0.52× at every p ≥ 257": 0.454–0.524. M equals L row for row at 521, 2053, 8209, 65537 (fallback primes), as stated. | consistent (range statement holds for p ≥ 1031 only) | `out/lucas_recompute.json` → `per_p[*]` |
| L15 | 95% intervals: own two-stage bootstrap (4000 resamples, different RNG) against 27 transcribed intervals: max relative endpoint gap 0.016, none on a different side of 1. | consistent | `python3 -I scripts/interval_check.py out` → `out/interval_check.json` |
| L16 | Decision rule (L/C ≤ 0.8 at every p ≥ 1031): holds (max 0.490). §5a band: no arm below R at all three prices at any p; every arm below R at w = 0 from p = 257 and above at w = 0.1 and 1 at every p. | consistent | `out/lucas_recompute.json` → `component_rule…`, `verdict_below_R_all_prices_point`, `below_R_w0_from`, `above_R_w01_and_w1_point` |
| X1 | **Accounting definition.** `IcResult::total(w) = group_ops + w·native_mul`, `native_mul = rel.mul + la.mul` (`ic_control.rs`); `RhoOut::total(w) = group_ops + w·native`, `native = max(canonicalisations_uncharged, steps)`. Every committed row satisfies these identities (the accounting row above). Both arms are in the same group (the order-r subgroup of the same genus-3 Jacobian, same G, T and seeds), so one group operation is the same unit on both sides. | consistent | `ic_control.rs` `IcResult::total`, `run_rho`; `scripts/vlib.py` `accounting_check` |
| X2 | **Counted native work left out of the priced totals.** (a) IC: every relation-phase and LA addition/subtraction (`rel_add`, `la_add`, including the DSU operations) is counted and reported as `native_all` but priced at 0 in `total(w)`, by the repository's field-operation convention ("additions are free", `ecbench/hyper_ic.rs`; `ic_control.rs` header); rho counts no additions. (b) Rho: `run_rho` keeps only `canonicalisations_uncharged` (or steps) and drops the walk's counted `work_native`, the short-cycle window's word compares (`cycle_window = 16` in `RhoWalk::negation()`). The validator's dumper (the frozen bin plus a direct `rho_reference_walk` call per rho row, which reproduces each committed rho row's recovered log and group operations) measured 11.5 (p = 131), 14.7 (p = 257), 15.6 (p = 521), 16.5 (p = 1031), 17.2 (p = 2053), 17.3 (p = 4099) window word compares per rho step, against 1.04–1.13 canonicalisations per step. In ecbench these compares are priced in the bitop unit and not in the GAE band (`band.rs` charges `cost.unpriced` plus `table_insert_attempts`), so leaving them out matches `band.rs`'s rule (lesson L-546d37ca7750). (c) Uncounted in both arms: the per-step partition hash, rho's distinguished-point table inserts and lookups, the IC large-prime hash map, IC's per-step coefficient additions. **Sensitivity** (arm/R; base = the studies' rule; "IC adds" prices `native_all`; "rho window" adds the window compares to rho's native work, at the p = 4099 rate above p = 4099, an estimate the rising trend makes conservative; "both" does both): arm M: p = 16411: w = 0.1 3.69 → IC adds 15.76, rho window 1.47, both 6.26; w = 1 19.54 → 85.95, 2.09, 9.20; p = 32771: w = 0.1 3.69 → IC adds 17.31, rho window 1.46, both 6.87; w = 1 19.57 → 94.53, 2.09, 10.11; p = 65537: w = 0.1 3.97 → IC adds 16.49, rho window 1.57, both 6.54; w = 1 21.30 → 90.16, 2.28, 9.64. No arm of this study is below R at both w = 0.1 and w = 1 under any of the four readings; the verdict "unresolved" holds under each. | consistent with `band.rs`; **disclosure gap**: §5a's first bullet asks for every unpriced counter of every arm with its count, and neither rho's window compares nor the uncounted hash/table work is listed | `ic_control.rs` `run_rho`; `ic_boundary.rs` `WalkTally::work_native`, `RhoWalk::negation`; `ecbench/band.rs` module doc; `python3 -I scripts/pricing_sensitivity.py /home/user/crypto out out/pricing_sensitivity.json` |
| X3 | **§5a calibrated reading (owner rule 2026-11-12) is not reported.** Each run records a best-of-N `w_cal` per prime (`calibration_ns_informational`: `w_rel` ≈ 0.003–0.005, `w_la` ≈ 0.009–0.014) and a `total_cal` per row (recomputed: 0 mismatches), but RESULTS gives no order at w = 0, w_cal and 10·w_cal. Recomputed (arm/R at 0 / w_cal / 10·w_cal): arm M p = 16411: 0.166 / 0.572 / 4.063; p = 32771: 0.156 / 0.575 / 4.178; p = 65537: 0.114 / 0.480 / 3.638. So no arm is "ranked at calibrated price" below rho (10·w_cal > 1 everywhere); the omission hides no ranking. | **inconsistent** (required reading omitted; no effect on the band verdict) | `python3 -I scripts/calibrated_reading.py /home/user/crypto out/calibrated_reading.json` |
| X4 | **Solver independence from the secret.** `Instance::t_secret` is read only in instance construction and in the `#[cfg(test)]` module at every freeze commit; no solver path reads it. | consistent | `git show <freeze>:src/cryptanalysis/chained_transfer/ic_control.rs \| grep -n t_secret` vs the line of `mod tests` |
| X5 | **Reproducibility bindings.** Recorded: commit, dirty flag, argv, start time, wall time, host CPU and platform (bench record). Not recorded: the Rust toolchain version, `--threads`/`--curves-map` in `run.json`'s `cfg` (they are in the bench record's argv). Counts are deterministic in (p, curve seed, seed) and do not depend on the thread count (replay, above). | partially consistent (toolchain not recorded) | `out/bench_records_check.txt` |
| X6 | **Unit tests named by the protocol** (the oracle, LA-mode, unit-T and capped-walk tests at the freeze commit): not re-run. A `cargo test --release` build of this crate took another session about 35 minutes and 10 GB on this host during the validation, and the dispatch asked for modest compute. The test names exist at each freeze commit (6, 8, 9 and 10 tests in `ic_control.rs`), matching the counts RESULTS gives where it gives one. | could not check | `git show <freeze>:src/cryptanalysis/chained_transfer/ic_control.rs \| grep -A1 "#\[test\]"` |


## Disposition

`validated` for the measurement: the frozen binary rebuilt from `4feb188f47` replays every p ≤ 2053 row exactly, every replayed log passes an independent certificate, the gates recompute, and 48 of 49 transcribed numbers match. Scope limits: rows at p ≥ 4099 (the headline primes) are accepted on determinism, on their identity with the 11-12 control and the LA study, and on the instrument's own `[t]G = T` check, which agreed with independent arithmetic on all 400 replayed rows; run.json records no recovered logs, so they could not be re-verified directly. Corrections needed in RESULTS.md (as an ERRATA file, not in place): 576 runs, not 696 (L11); L's LA share at 32771 is 0.56, not 0.54 (L13); the frozen `uncertainty.py` was a one-stage bootstrap and was deleted at the results commit (L4). The §5a calibrated reading (X3) and rho's dropped window compares with their sensitivity (X2) should be disclosed. No row may be cited beyond `[pcc:unverified]` until a pcc claim records this report.
