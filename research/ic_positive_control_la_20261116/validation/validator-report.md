# Validator report: research/ic_positive_control_la_20261116

Disposition: **validated (measurement, gates and ratios); RESULTS.md needs corrections A10, A12, A13; X2 and X3 must be disclosed**

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
| A1 | Freeze order: `caae5eca9f` (06:36:03: PROTOCOL, frozen.sha256, instrument with `LaMode`) precedes every data commit (training 06:39–06:51, TRAINING.md `2bc7249bac` 06:51:46, test log 06:52, results `fbd22482c7` 07:13:56). Training start (train_cc0.7 mtime − wall_s) 06:36:20; test start 06:51:53, 7 s after TRAINING.md was committed, as PROTOCOL §4 requires. | consistent | `scripts/freeze_check.sh` → `out/freeze_check.txt` |
| A2 | `frozen.sha256` entries equal `git show caae5eca9f:<path> \| sha256sum`. | consistent | `out/freeze_check.txt` |
| A3 | Test binary: `binary.sha256` (added in `cc2d4d50c6`, written 06:37:09) names `d611851a…`; the producer's copy in the shared scratchpad hashed to that value at validator dispatch. `test.sh` runs that copy. | consistent | `out/freeze_check.txt` (last section) |
| A4 | Training binary: `train.sh` runs `target/release/ic_positive_control` from the shared target dir, not the hashed copy; it started at 06:36:20, before `binary.sha256` existed, and `cc2d4d50c6` (06:38:32) committed a modified `ic_control.rs` (unit-T WIP, "tests running") while the sweep ran. The training rows are therefore not bound to the frozen commit by any recorded hash. Replay at p = 4099 (the training prime below 8209): the binary rebuilt from `caae5eca9f` (bit-identical to `binary.sha256`) reproduces all 32 training rows at p = 4099 for the four cc values (32/32 identical in every deterministic field). The p = 8209 and 16411 training rows were not replayed (dispatch limit), so the training score's binding to the frozen sources is established for a third of its instances. | consistent at p = 4099; could not check at 8209/16411 | `scripts/replay_all.sh la` → `out/replay_la_train_cc*.json`; `python3 -I scripts/compare_runs.py out/replay_la_train_cc<cc>.json research/ic_positive_control_la_20261116/train_cc<cc>.json` → `out/compare_la_train.txt` |
| A5 | Training score recomputed from `train_cc*.json` (geometric mean over (p, curve, seed) of S total_w01 / R total_w01): 4.073, 3.270, 2.876, 2.843 for cc = 0.7, 1.0, 1.4, 2.0; cc* = 2.0; 12 runs each, 0 uncertified; training instances (curve_seed0 = 5001) share no (p, #J) with the test instances. | consistent | `python3 -I scripts/validate_la.py /home/user/crypto out/la_recompute.json` → `training`, `training_cc_star`, `train_test_instance_overlap` |
| A6 | Analysis script `analyze.py` was first committed with the results (`fbd22482c7`), after the data; the gates and metrics it computes are the ones PROTOCOL §5–§6 name, and the validator's recomputation (A8–A12) does not depend on it. | consistent (no pre-data analysis-script commit; no effect on numbers) | `git log -- research/ic_positive_control_la_20261116/analyze.py` |
| A7 | Raw files (`run.json`, `run.log`, four `train_cc*.json`) hash to the bench record's `raw[].sha256`; `uploaded: false`. | consistent | `out/bench_records_check.txt` |
| A8 | Accounting identities on all 576 rows (as in the Lucas report, L6): 0 violations. Arms record `(oracle, la_mode)` = M (lucas_roots, baseline), N (lucas_roots, coord), S (lucas_roots, coord_square). | consistent | `out/la_recompute.json` → `accounting_violations`, `modes` |
| A9 | G1 (M reproduces the Lucas study's M rows): 144/144 identical in group ops, rel_mul, la_mul, trials, relations and in every other deterministic field; R rows are identical too (144/144). G2 (N and S have M's trials and relations; RESULTS adds rel_mul): 288/288, also equal in splits and fb_cols; N and S have M's setup and walk group ops. G3: 576/576 certified. | consistent | `out/la_recompute.json` → `G1`, `G2`, `G3`, `N_S_setup_walk_equal_M` |
| A10 | RESULTS "G3 … 696 runs" and "0 fallbacks in 348 N and S runs": run.json has **576** rows and **288** N + S runs (0 fallbacks is right). | **inconsistent** (counts misstated) | `out/la_recompute.json` → `n_rows`, `N_and_S_runs`, `fallbacks_total` |
| A11 | Component table (M LA mult., N/M, S/M, "S/M incl. additions"), end-to-end table (M and S at w = 0.1 and 1, p = 4099, 16411, 32771, 65537), C→S distance at 32771 (7.44 → 1.92, 3.9×; 40.2 → 9.85, 4.1×): 32 of 32 transcribed numbers match at printed precision; 18 transcribed intervals agree with the validator's bootstrap (max endpoint gap 0.016, none crosses 1). | consistent | `out/la_recompute.json` → `claim_checks`; `out/interval_check.json` |
| A12 | Labelling: the column "S/M incl. additions" (0.405, 0.394, 0.388, 0.453) is the ratio of **total** native work (`native_all`: relation phase plus LA, multiplications plus additions), not of the LA. The LA alone counting additions is S/M = 0.334, 0.337, 0.339, 0.339 at those primes (0.303–0.339 over all p). The prose "0.39–0.55× counting additions too" therefore describes total native work, not the linear algebra. | **inconsistent** (mislabelled; the LA result counting additions is better than stated) | `out/la_recompute.json` → `per_p[*]["S/M la incl adds"]`, `["S/M native_all"]` |
| A13 | "LA is now 0.26–0.49 of S's native multiplications": 0.256–0.486 over p ≥ 4099 (0.20 at p = 521). "S/R at w = 0.1 is 2.6–5.6 at 521, 2053, 8209, 65537 against 1.9–2.8 at the neighbouring primes": 2.57–5.61 and 1.92–2.77, but these are the **test** rows (run.json), not the training curves the paragraph heading names. | consistent (numbers) / labelling error (source) | `out/la_recompute.json` → `la_share_S range`, `S/R w0.1 by p` |
| A14 | Decision rule (S/M LA ≤ 0.6 at every p ≥ 4099): holds (0.317–0.329). S* rule: cc* = 2.0, so S* = S, as reported. §5a band: no arm below R at all three prices; S below R at w = 0 from p = 257, above at w = 0.1 and 1 at every p. | consistent | `out/la_recompute.json` → `verdict_below_all_prices_point`, `S_below_R_w0`, `S_above_R_w01_w1` |
| A15 | Pre-freeze diagnostic `dbg.json` (producer scratchpad, 06:16, before the freeze): arm M only, test curve 0 (curve_seed0 = 1) at p = 4099 and 32771 — the "diagnostic split" PROTOCOL §1 discloses. It touches only arm M, whose rows were already published by the Lucas study; no N or S test row was seen before the freeze. | consistent (disclosed) | producer file `…/scratchpad/dbg.json` (uncommitted; sha256 in `out/w_vs_u.txt`), inspected read-only |
| A16 | Replay certificate (p ≤ 2053, 5 curves × 4 seeds, arms M N S R) with the binary rebuilt from `caae5eca9f`: sha256 `d611851a…`, **bit-identical to `binary.sha256`**, so the measured binary is the frozen commit's build. It reproduces all 400 rows at p ≤ 2053 exactly (400/400). The producer's own copy of that binary gives the same 400/400 (`out/producer_bin_replay/`). | consistent (binary hash and 400/400 rows) | `scripts/replay_all.sh la` → `out/replay_la_MNSR.json`, `out/certs_la_MNSR.json` |
| A17 | Independent log check on the same cells: 400/400 rows (M, N, S, R × 100) pass the checks listed in the Lucas report (L10): instance from the generator, valid divisors, [r]G = 0, [order]D = 0, [t]G = T, t ≡ t_secret; val_dump rows equal the frozen rows 400/400. | consistent (p ≤ 2053) | `scripts/verify_certs.py` → `out/certs_la_MNSR.json` |
| X1 | **Accounting definition.** `IcResult::total(w) = group_ops + w·native_mul`, `native_mul = rel.mul + la.mul` (`ic_control.rs`); `RhoOut::total(w) = group_ops + w·native`, `native = max(canonicalisations_uncharged, steps)`. Every committed row satisfies these identities (the accounting row above). Both arms are in the same group (the order-r subgroup of the same genus-3 Jacobian, same G, T and seeds), so one group operation is the same unit on both sides. | consistent | `ic_control.rs` `IcResult::total`, `run_rho`; `scripts/vlib.py` `accounting_check` |
| X2 | **Counted native work left out of the priced totals.** (a) IC: every relation-phase and LA addition/subtraction (`rel_add`, `la_add`, including the DSU operations) is counted and reported as `native_all` but priced at 0 in `total(w)`, by the repository's field-operation convention ("additions are free", `ecbench/hyper_ic.rs`; `ic_control.rs` header); rho counts no additions. (b) Rho: `run_rho` keeps only `canonicalisations_uncharged` (or steps) and drops the walk's counted `work_native`, the short-cycle window's word compares (`cycle_window = 16` in `RhoWalk::negation()`). The validator's dumper (the frozen bin plus a direct `rho_reference_walk` call per rho row, which reproduces each committed rho row's recovered log and group operations) measured 11.5 (p = 131), 14.7 (p = 257), 15.6 (p = 521), 16.5 (p = 1031), 17.2 (p = 2053), 17.3 (p = 4099) window word compares per rho step, against 1.04–1.13 canonicalisations per step. In ecbench these compares are priced in the bitop unit and not in the GAE band (`band.rs` charges `cost.unpriced` plus `table_insert_attempts`), so leaving them out matches `band.rs`'s rule (lesson L-546d37ca7750). (c) Uncounted in both arms: the per-step partition hash, rho's distinguished-point table inserts and lookups, the IC large-prime hash map, IC's per-step coefficient additions. **Sensitivity** (arm/R; base = the studies' rule; "IC adds" prices `native_all`; "rho window" adds the window compares to rho's native work, at the p = 4099 rate above p = 4099, an estimate the rising trend makes conservative; "both" does both): arm S: p = 16411: w = 0.1 1.97 → IC adds 6.31, rho window 0.78, both 2.51; w = 1 10.07 → 33.95, 1.08, 3.63; p = 32771: w = 0.1 1.92 → IC adds 6.80, rho window 0.76, both 2.70; w = 1 9.85 → 36.69, 1.05, 3.92; p = 65537: w = 0.1 2.57 → IC adds 7.52, rho window 1.02, both 2.99; w = 1 13.62 → 40.87, 1.46, 4.37. No arm of this study is below R at both w = 0.1 and w = 1 under any of the four readings; the verdict "unresolved" holds under each. | consistent with `band.rs`; **disclosure gap**: §5a's first bullet asks for every unpriced counter of every arm with its count, and neither rho's window compares nor the uncounted hash/table work is listed | `ic_control.rs` `run_rho`; `ic_boundary.rs` `WalkTally::work_native`, `RhoWalk::negation`; `ecbench/band.rs` module doc; `python3 -I scripts/pricing_sensitivity.py /home/user/crypto out out/pricing_sensitivity.json` |
| X3 | **§5a calibrated reading (owner rule 2026-11-12) is not reported.** Each run records a best-of-N `w_cal` per prime (`calibration_ns_informational`: `w_rel` ≈ 0.003–0.005, `w_la` ≈ 0.009–0.014) and a `total_cal` per row (recomputed: 0 mismatches), but RESULTS gives no order at w = 0, w_cal and 10·w_cal. Recomputed (arm/R at 0 / w_cal / 10·w_cal): arm S p = 16411: 0.166 / 0.339 / 1.820; p = 32771: 0.156 / 0.325 / 1.777; p = 65537: 0.114 / 0.286 / 1.769. So no arm is "ranked at calibrated price" below rho (10·w_cal > 1 everywhere); the omission hides no ranking. | **inconsistent** (required reading omitted; no effect on the band verdict) | `python3 -I scripts/calibrated_reading.py /home/user/crypto out/calibrated_reading.json` |
| X4 | **Solver independence from the secret.** `Instance::t_secret` is read only in instance construction and in the `#[cfg(test)]` module at every freeze commit; no solver path reads it. | consistent | `git show <freeze>:src/cryptanalysis/chained_transfer/ic_control.rs \| grep -n t_secret` vs the line of `mod tests` |
| X5 | **Reproducibility bindings.** Recorded: commit, dirty flag, argv, start time, wall time, host CPU and platform (bench record). Not recorded: the Rust toolchain version, `--threads`/`--curves-map` in `run.json`'s `cfg` (they are in the bench record's argv). Counts are deterministic in (p, curve seed, seed) and do not depend on the thread count (replay, above). | partially consistent (toolchain not recorded) | `out/bench_records_check.txt` |
| X6 | **Unit tests named by the protocol** (the oracle, LA-mode, unit-T and capped-walk tests at the freeze commit): not re-run. A `cargo test --release` build of this crate took another session about 35 minutes and 10 GB on this host during the validation, and the dispatch asked for modest compute. The test names exist at each freeze commit (6, 8, 9 and 10 tests in `ic_control.rs`), matching the counts RESULTS gives where it gives one. | could not check | `git show <freeze>:src/cryptanalysis/chained_transfer/ic_control.rs \| grep -A1 "#\[test\]"` |


## Disposition

`validated` for the measurement: the test binary is the frozen commit's build (bit-identical rebuild), it replays every p ≤ 2053 row and the p = 4099 training rows exactly, every replayed log passes an independent certificate, the gates and the training score recompute, and all 32 transcribed numbers match. Scope limits as in the Lucas report (p ≥ 4099 test rows and p ≥ 8209 training rows not replayed; no recovered logs in run.json). Corrections needed (ERRATA): 576 runs and 288 N + S runs, not 696 and 348 (A10); the "incl. additions" column is total native work, the LA alone counting additions is 0.33–0.34× (A12); the 2.6–5.6 vs 1.9–2.8 comparison is from the test rows (A13). Disclose X2 and X3.
