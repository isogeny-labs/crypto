# Validator report: research/ic_positive_control_unit_t_20261116

Disposition: **validated (measurement, gates, D1 and F1); RESULTS.md needs correction U13; X2 and X3 must be disclosed**

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
| U1 | Freeze order: `551590e3b7` (07:51:52: PROTOCOL, frozen.sha256, binary.sha256, test.sh) precedes the analysis-script commit (07:52:46), the run (start = run.json mtime − wall_s = 07:52:04.9) and every data commit. The instrument (unit-T, `LucasRootsFast`) was committed earlier (`5d39b2f156`, 07:17:30). | consistent | `scripts/freeze_check.sh` → `out/freeze_check.txt` |
| U2 | `frozen.sha256` entries equal `git show 551590e3b7:<path> \| sha256sum`; `binary.sha256` (`cd574bde…`) equals the producer's binary hashed at validator dispatch. | consistent | `out/freeze_check.txt` |
| U3 | Bench record names commit `be5e426df1` (the freeze commit's parent), not `551590e3b7`; its instrument blobs are identical to the freeze commit's (`ic_control` 1b99e0a25bdd, bin b5b41e9fff84). | consistent | `out/bench_records_check.txt` |
| U4 | Pre-freeze smoke runs (producer scratchpad `smokeU.json` 07:12, `smokeV.json` 07:51:08) used the training curves (curve_seed0 = 5001) only. | consistent | producer files `…/scratchpad/smokeU.json`, `smokeV.json` (uncommitted; sha256 in `out/w_vs_u.txt`), inspected read-only |
| U5 | `analyze.py` changed after the data (`7d135a6a18` → `3eaf62ab17`): it adds the V/S per-trial ratio (the D1 like-for-like comparison) and rewrites the summary line; gate code and every other ratio are unchanged. The validator's numbers do not depend on it. | consistent (post-data additive change, D1-motivated) | `git diff 7d135a6a18 3eaf62ab17 -- research/ic_positive_control_unit_t_20261116/analyze.py` |
| U6 | Raw files hash to the bench record; `uploaded: false`. | consistent | `out/bench_records_check.txt` |
| U7 | Accounting identities on all 592 rows: 0 violations. Arms record `(oracle, la_mode, walk)` = S (lucas_roots, coord_square, mixed), U (lucas_roots, coord_square, unit_t), V (lucas_roots_fast, coord_square, **mixed**). | consistent | `python3 -I scripts/validate_unit_t.py /home/user/crypto out/unit_t_recompute.json` → `accounting_violations`, `modes` |
| U8 | G1 (S and R equal the LA study's S and R rows on every shared key): 144 S + 144 R compared, identical in every deterministic field (S rows differ only by the new `walk` key); 131101 is the only unshared prime. G2: 592/592 certified (RESULTS "592 runs" is right). | consistent | `out/unit_t_recompute.json` → `G1`, `G2` |
| U9 | **D1**: every one of the 148 V rows records `walk: mixed`, `oracle: lucas_roots_fast` (U rows: `unit_t`; S rows: `mixed`). V has S's trials in 112 of 148 runs; the 36 that differ are all at 2053, 8209 and 65537 (20, 12, 4), where S's Cantor–Zassenhaus fallback draws random numbers and V's deterministic one does not. | consistent | `out/unit_t_recompute.json` → `D1` |
| U10 | **F1** counts: U fallbacks 268 at p = 131 (8 of 20 runs; 280 attempts), 13 at 2053 (1 run), 2 at 4099 (1 run), 0 at every other prime, 0 from 8209 up; S and V have 0 fallbacks everywhere. Instance (131, curve 1, seed 1001): r = 281357, 76 fallbacks, 76 attempts, certified. | consistent | `out/unit_t_recompute.json` → `F1` |
| U11 | **F1** diagnosis ("first core 60 × 37 of rank 17, target column not determined"): not checkable from committed artifacts (the debug build is not committed). The producer's uncommitted dump `scratchpad/core131.json` (sha256 `57b76d7c…`) gives, by the validator's own Gaussian elimination mod r: 60 rows × 37 columns, rank 17, 18 distinct rows (12 zero), target column not in the row space. The debug build's counted row for that instance (`scratchpad/dbg131.json`) equals the committed U row. | consistent with uncommitted evidence; not checkable from the committed artifact set | `python3 -I scripts/check_f1_core.py …/scratchpad/core131.json` → `out/f1_core_check.json` |
| U12 | Component table (U/S LA, "U/S LA incl. additions", V/S per trial at 8209…131101), end-to-end table (S, U, V at w = 0.1 and 1), P1 (U/S LA 1.24 at 4099), P5 slope −0.165, U/R at 131101: 48 of 48 transcribed numbers match; 13 transcribed intervals agree with the validator's bootstrap (max endpoint gap 0.016, none crosses 1; U/R w = 0.1 at 32771 lower end 0.957 vs claimed 0.97, both < 1). P2 values are V/U (0.771–0.823 at 2053/8209/65537, 0.795 at 521; 1.000–1.006 elsewhere ≥ 1031). | consistent | `out/unit_t_recompute.json` → `claim_checks`; `out/interval_check.json` |
| U13 | Labelling: "U/S LA incl. additions" (0.985, 0.962, 0.954, 0.971, 0.965) is the `native_all` ratio (relation phase plus LA); LA alone counting additions is 0.963, 0.951, 0.944, 0.953, 0.958. The prose "0.95–0.97" omits 8209 (0.985 in the producer's own table). | **inconsistent** (mislabelled column; prose range excludes one table entry) | `out/unit_t_recompute.json` → `per_p[*]` (printed by `scripts/validate_unit_t.py`) |
| U14 | "V 12–14% closer at the fallback primes": V/S total at w = 0.1 is 0.857 (8209) and 0.876 (65537), also 0.845 at 521 and 2053. "U about 10% closer at the cube-root primes": 0.898, 0.895, 0.902 at 16411, 32771, 131101, but 1.099 at 4099 (F1). | consistent at the tabulated primes |  same |
| U15 | Decision rules: unit-T rule (U/S LA ≤ 0.85 at every p ≥ 4099) not met (1.242 at 4099), fallback rule (V/U ≤ 0.9 at 2053, 8209, 65537) met (0.773, 0.771, 0.823), as reported. §5a: no arm below R at all three prices; every arm below R at w = 0 from 257, above at 0.1 and 1 everywhere. P5 slope −0.1646. | consistent | `out/unit_t_recompute.json` |
| U16 | Replay certificate (p ≤ 2053, 5 curves × 4 seeds, arms S U V R) with the binary rebuilt from `551590e3b7`: sha256 `cd574bde…`, **bit-identical to `binary.sha256`**. It reproduces all 400 rows at p ≤ 2053 exactly (400/400), including the 9 U runs with fallbacks (8 at p = 131, 1 at 2053) and the F1 instance (131, 1, 1001). The producer's copy gives the same 400/400. | consistent (binary hash and 400/400 rows) | `scripts/replay_all.sh unit_t` → `out/replay_unit_t_SUVR.json`, `out/certs_unit_t_SUVR.json` |
| U17 | Independent log check on the same cells: 400/400 rows (S, U, V, R × 100) pass the checks of the Lucas report (L10), including every U row that fell back; val_dump rows equal the frozen rows 400/400. | consistent (p ≤ 2053) | `scripts/verify_certs.py` → `out/certs_unit_t_SUVR.json` |
| X1 | **Accounting definition.** `IcResult::total(w) = group_ops + w·native_mul`, `native_mul = rel.mul + la.mul` (`ic_control.rs`); `RhoOut::total(w) = group_ops + w·native`, `native = max(canonicalisations_uncharged, steps)`. Every committed row satisfies these identities (the accounting row above). Both arms are in the same group (the order-r subgroup of the same genus-3 Jacobian, same G, T and seeds), so one group operation is the same unit on both sides. | consistent | `ic_control.rs` `IcResult::total`, `run_rho`; `scripts/vlib.py` `accounting_check` |
| X2 | **Counted native work left out of the priced totals.** (a) IC: every relation-phase and LA addition/subtraction (`rel_add`, `la_add`, including the DSU operations) is counted and reported as `native_all` but priced at 0 in `total(w)`, by the repository's field-operation convention ("additions are free", `ecbench/hyper_ic.rs`; `ic_control.rs` header); rho counts no additions. (b) Rho: `run_rho` keeps only `canonicalisations_uncharged` (or steps) and drops the walk's counted `work_native`, the short-cycle window's word compares (`cycle_window = 16` in `RhoWalk::negation()`). The validator's dumper (the frozen bin plus a direct `rho_reference_walk` call per rho row, which reproduces each committed rho row's recovered log and group operations) measured 11.5 (p = 131), 14.7 (p = 257), 15.6 (p = 521), 16.5 (p = 1031), 17.2 (p = 2053), 17.3 (p = 4099) window word compares per rho step, against 1.04–1.13 canonicalisations per step. In ecbench these compares are priced in the bitop unit and not in the GAE band (`band.rs` charges `cost.unpriced` plus `table_insert_attempts`), so leaving them out matches `band.rs`'s rule (lesson L-546d37ca7750). (c) Uncounted in both arms: the per-step partition hash, rho's distinguished-point table inserts and lookups, the IC large-prime hash map, IC's per-step coefficient additions. **Sensitivity** (arm/R; base = the studies' rule; "IC adds" prices `native_all`; "rho window" adds the window compares to rho's native work, at the p = 4099 rate above p = 4099, an estimate the rising trend makes conservative; "both" does both): arm U: p = 16411: w = 0.1 1.77 → IC adds 6.07, rho window 0.70, both 2.41; w = 1 8.97 → 32.67, 0.96, 3.50; p = 32771: w = 0.1 1.72 → IC adds 6.49, rho window 0.68, both 2.58; w = 1 8.75 → 35.02, 0.94, 3.74; p = 65537: w = 0.1 2.42 → IC adds 7.31, rho window 0.96, both 2.90; w = 1 12.79 → 39.68, 1.37, 4.24; p = 131101: w = 0.1 1.78 → IC adds 7.66, rho window 0.71, both 3.04; w = 1 9.17 → 41.54, 0.98, 4.44. Under "rho window" alone, these arms would be below R at all three prices: C_unit_t at p = 16411, 32771, 131101. The band verdict of this study is therefore not robust to pricing rho's compares alone; under "both" every arm stays above R at w = 0.1 and 1, and under "IC adds" the distances grow 3–5×. | consistent with `band.rs`; **disclosure gap**: §5a's first bullet asks for every unpriced counter of every arm with its count, and neither rho's window compares nor the uncounted hash/table work is listed | `ic_control.rs` `run_rho`; `ic_boundary.rs` `WalkTally::work_native`, `RhoWalk::negation`; `ecbench/band.rs` module doc; `python3 -I scripts/pricing_sensitivity.py /home/user/crypto out out/pricing_sensitivity.json` |
| X3 | **§5a calibrated reading (owner rule 2026-11-12) is not reported.** Each run records a best-of-N `w_cal` per prime (`calibration_ns_informational`: `w_rel` ≈ 0.003–0.005, `w_la` ≈ 0.009–0.014) and a `total_cal` per row (recomputed: 0 mismatches), but RESULTS gives no order at w = 0, w_cal and 10·w_cal. Recomputed (arm/R at 0 / w_cal / 10·w_cal): arm U p = 16411: 0.164 / 0.300 / 1.468; p = 32771: 0.154 / 0.285 / 1.413; p = 65537: 0.112 / 0.236 / 1.317; p = 131101: 0.137 / 0.227 / 1.011. So no arm is "ranked at calibrated price" below rho (10·w_cal > 1 everywhere); the omission hides no ranking. | **inconsistent** (required reading omitted; no effect on the band verdict) | `python3 -I scripts/calibrated_reading.py /home/user/crypto out/calibrated_reading.json` |
| X4 | **Solver independence from the secret.** `Instance::t_secret` is read only in instance construction and in the `#[cfg(test)]` module at every freeze commit; no solver path reads it. | consistent | `git show <freeze>:src/cryptanalysis/chained_transfer/ic_control.rs \| grep -n t_secret` vs the line of `mod tests` |
| X5 | **Reproducibility bindings.** Recorded: commit, dirty flag, argv, start time, wall time, host CPU and platform (bench record). Not recorded: the Rust toolchain version, `--threads`/`--curves-map` in `run.json`'s `cfg` (they are in the bench record's argv). Counts are deterministic in (p, curve seed, seed) and do not depend on the thread count (replay, above). | partially consistent (toolchain not recorded) | `out/bench_records_check.txt` |
| X6 | **Unit tests named by the protocol** (the oracle, LA-mode, unit-T and capped-walk tests at the freeze commit): not re-run. A `cargo test --release` build of this crate took another session about 35 minutes and 10 GB on this host during the validation, and the dispatch asked for modest compute. The test names exist at each freeze commit (6, 8, 9 and 10 tests in `ic_control.rs`), matching the counts RESULTS gives where it gives one. | could not check | `git show <freeze>:src/cryptanalysis/chained_transfer/ic_control.rs \| grep -A1 "#\[test\]"` |


## Disposition

`validated` for the measurement: the binary is the frozen commit's build (bit-identical rebuild), it replays every p ≤ 2053 row exactly, every replayed log passes an independent certificate, the gates and all 48 transcribed numbers recompute, D1 is confirmed in every V row and F1's counts are exact (its rank diagnosis is confirmed only on the producer's uncommitted debug dump). Scope limits as in the Lucas report. Corrections needed (ERRATA): the "LA incl. additions" column is total native work, and its prose range omits 0.985 at 8209 (U13). Disclose X2, in particular that U would be below rho at all three prices at 16411, 32771 and 131101 if rho's dropped window compares alone were priced (not under the repository's band rule, and not when the IC's additions are priced too), and X3. Note for the reader: the "first interval to touch 1" (U at 32771, w = 0.1, lower end 0.957–0.97 depending on the resampling seed) is not a result below rho.
