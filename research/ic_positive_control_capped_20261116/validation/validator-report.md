# Validator report: research/ic_positive_control_capped_20261116

Disposition: **validated (measurement, gates and ratios); RESULTS.md needs corrections W8, W11, W13; X2 and X3 must be disclosed**

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
| W1 | Freeze order: `81ddcdc450` (10:01:01: PROTOCOL, frozen.sha256, binary.sha256, test.sh) precedes the analysis-script commit (10:01:52) and every data commit; W run start (run_w.json mtime − wall_s) 10:01:06.2, S/R reproduction 11:36:30–11:37:01. `analyze.py` is unchanged from 10:01:52 to the results commit. | consistent | `scripts/freeze_check.sh` → `out/freeze_check.txt`; `git diff 4a95132450 5576b7fac8 -- research/ic_positive_control_capped_20261116/analyze.py` (empty) |
| W2 | `frozen.sha256` entries equal `git show 81ddcdc450:<path> \| sha256sum`; `binary.sha256` (`6a97ea07…`) equals the producer's binary hashed at validator dispatch. | consistent | `out/freeze_check.txt` |
| W3 | Bench record names commit `fcc5fc5e87` (the freeze commit's parent); its instrument blobs equal the freeze commit's (79c5a3f8faa3, 43ddd2127db4). Raw files hash to the record; `uploaded: false`. | consistent | `out/bench_records_check.txt` |
| W4 | Pre-freeze smoke on **test** instances (producer scratchpad `smokeW.json`, 10:00:26: arms U W R, p = 131 and 2053, curves 0–2, seeds 1000–2002), disclosed in PROTOCOL §1. Its 36 rows equal the committed W rows and the unit-T study's U and R rows exactly. It touches only p ≤ 2053, outside the p ≥ 4099 decision rule. | consistent (disclosed peek; no effect on the decision primes) | `python3 -I scripts/check_w_vs_u.py /home/user/crypto …/scratchpad/smokeW.json` → `out/w_vs_u.txt` (last lines; sha256 of the producer file recorded there) |
| W5 | Accounting identities on all 148 W and 248 S/R rows: 0 violations. W rows record `(lucas_roots_fast, coord_square, unit_t_capped)`. | consistent | `python3 -I scripts/validate_capped.py /home/user/crypto out/capped_recompute.json` → `accounting_violations`, `W_modes` |
| W6 | G1 (run_sr S and R rows equal the unit-T study's at p ≤ 8209): 124 S + 124 R, identical in **every** deterministic field, and run_sr covers exactly the unit-T keys with p ≤ 8209. G2: 148/148 W certified; every W key has S and R partners; W's (order, r, h) equal the partners'. | consistent | `out/capped_recompute.json` → `G1`, `G2`, `instance_mismatch` |
| W7 | Reuse of S and R rows at p > 8209 rests on determinism plus G1 at p ≤ 8209 (disclosed, PROTOCOL §2 and §8). The unit-T and capped binaries differ (551590e3b7 vs 81ddcdc450); G1 shows they agree on every compared row. | consistent (disclosed) | as W6 |
| W8 | **W is arm U at the cube-root primes ≥ 8209.** At p = 16411, 32771 and 131101 every W row equals the unit-T study's U row in every deterministic field (8/8, 8/8, 4/4); at 4099, 4 of 12. There the cap `max(64, ⌊√r/16⌋)` is at least the default 4096 and the fallback root finder never fires, so W runs U's walk with U's roots. The W/S ratios and the W/R values 1.77, 1.72, 1.78 at those primes are the unit-T study's U measurements, re-run; the capped walk and the fast fallback act only at p ≤ 4099 and at the fallback primes (8209, 65537). RESULTS does not say this. | **inconsistent** (undisclosed; the numbers are right, the attribution "W … new operating point" at 16411/32771/131101 is U's result) | `scripts/check_w_vs_u.py` → `out/w_vs_u.txt` |
| W9 | W-vs-S table (LA, "LA incl. additions", relation per trial, total at w = 0.1 and 1), end-to-end table (original C from the 11-12 `control.json`, S, W at w = 0.1 and 1), slope −0.161, P1 fallbacks 26: 54 of 54 transcribed numbers match; 10 transcribed intervals agree with the validator's bootstrap (max endpoint gap 0.008, none crosses 1). Every W/S interval at p ≥ 4099 lies below 1 at w = 0.1 and w = 1. | consistent | `out/capped_recompute.json` → `claim_checks`, `advances_upper<1`; `out/interval_check.json` |
| W10 | Fallbacks: W 6, 3, 6, 4, 7 at p = 131, 257, 521, 1031, 2053 (26 in total; P1's ≤ 5 failed, as reported), 0 at p ≥ 4099. W/S at w = 0.1 for p ≤ 2053: 0.751–1.071, every interval straddles 1. | consistent | `out/capped_recompute.json` → `W_fallbacks_by_p`, `W/S small p w0.1` |
| W11 | Labelling: "LA incl. additions" (0.958, 0.867, 0.962, 0.954, 0.892, 0.965) is the `native_all` ratio (relation phase plus LA); LA alone counting additions is 0.941, 0.966, 0.951, 0.944, 0.959, 0.958. | **inconsistent** (mislabelled column) | `out/capped_recompute.json` (printed `W/S LA-only incl adds`) |
| W12 | "W below rho at w = 0 from p = 257 (0.11–0.16 at p ≥ 16411)": below from 257; 0.164, 0.154, 0.113, 0.137. No w = 0.1 interval lies wholly below 1 (32771: [0.957, 3.347] own, [0.95, 3.32] claimed). §5a: W never below R at all three prices. | consistent | `out/capped_recompute.json` → `W_below_R_w0`, `W/R w0 p>=16411`, `W_w01_upper_below_1`, `W_below_R_all_prices` |
| W13 | Extrapolation (labelled "not measured"): a least-squares fit of ln(W/R) on ln p over p ≥ 4099 reaches 1 at p ≈ 2^21.7 (w = 0.1) and 2^37.5 (w = 1); RESULTS says "near 2^20–2^21" and "not below about 2^35". The w = 1 statement holds; the w = 0.1 one is about 0.7 bits low against the fit. | inconsistent (minor; interpretation) | `out/capped_recompute.json` → `crossover_log2_w0.1`, `crossover_log2_w1` |
| W14 | Decision rule (W below S at w = 0.1 and w = 1 at every p ≥ 4099): holds (point and 95% upper ends < 1). | consistent (see W8 for what "W" is at 16411/32771/131101) | `out/capped_recompute.json` → `advances_every_p>=4099 (point)`, `advances_upper<1` |
| W15 | Replay certificate (p ≤ 2053, 5 curves × 4 seeds, arm W and arms S R) with the binary rebuilt from `81ddcdc450`: sha256 `6a97ea07…`, **bit-identical to `binary.sha256`** (two independent rebuilds, 15:49 and in the dumper pair build). It reproduces all 100 W rows of run_w.json and all 200 S/R rows of run_sr.json at p ≤ 2053 exactly (300/300); the producer's copy gives the same. | consistent (binary hash and 300/300 rows) | `scripts/replay_all.sh capped` → `out/replay_capped_W.json`, `out/replay_capped_SR.json`, `out/certs_capped_*.json` |
| W16 | Independent log check on the same cells: 300/300 rows (W, S, R × 100) pass the checks of the Lucas report (L10), including the 15 W runs that fell back (26 fallbacks); val_dump rows equal the frozen rows 300/300. | consistent (p ≤ 2053) | `scripts/verify_certs.py` → `out/certs_capped_W.json`, `out/certs_capped_SR.json` |
| X1 | **Accounting definition.** `IcResult::total(w) = group_ops + w·native_mul`, `native_mul = rel.mul + la.mul` (`ic_control.rs`); `RhoOut::total(w) = group_ops + w·native`, `native = max(canonicalisations_uncharged, steps)`. Every committed row satisfies these identities (the accounting row above). Both arms are in the same group (the order-r subgroup of the same genus-3 Jacobian, same G, T and seeds), so one group operation is the same unit on both sides. | consistent | `ic_control.rs` `IcResult::total`, `run_rho`; `scripts/vlib.py` `accounting_check` |
| X2 | **Counted native work left out of the priced totals.** (a) IC: every relation-phase and LA addition/subtraction (`rel_add`, `la_add`, including the DSU operations) is counted and reported as `native_all` but priced at 0 in `total(w)`, by the repository's field-operation convention ("additions are free", `ecbench/hyper_ic.rs`; `ic_control.rs` header); rho counts no additions. (b) Rho: `run_rho` keeps only `canonicalisations_uncharged` (or steps) and drops the walk's counted `work_native`, the short-cycle window's word compares (`cycle_window = 16` in `RhoWalk::negation()`). The validator's dumper (the frozen bin plus a direct `rho_reference_walk` call per rho row, which reproduces each committed rho row's recovered log and group operations) measured 11.5 (p = 131), 14.7 (p = 257), 15.6 (p = 521), 16.5 (p = 1031), 17.2 (p = 2053), 17.3 (p = 4099) window word compares per rho step, against 1.04–1.13 canonicalisations per step. In ecbench these compares are priced in the bitop unit and not in the GAE band (`band.rs` charges `cost.unpriced` plus `table_insert_attempts`), so leaving them out matches `band.rs`'s rule (lesson L-546d37ca7750). (c) Uncounted in both arms: the per-step partition hash, rho's distinguished-point table inserts and lookups, the IC large-prime hash map, IC's per-step coefficient additions. **Sensitivity** (arm/R; base = the studies' rule; "IC adds" prices `native_all`; "rho window" adds the window compares to rho's native work, at the p = 4099 rate above p = 4099, an estimate the rising trend makes conservative; "both" does both): arm W: p = 16411: w = 0.1 1.77 → IC adds 6.07, rho window 0.70, both 2.41; w = 1 8.97 → 32.67, 0.96, 3.50; p = 32771: w = 0.1 1.72 → IC adds 6.49, rho window 0.68, both 2.58; w = 1 8.75 → 35.02, 0.94, 3.74; p = 65537: w = 0.1 2.11 → IC adds 6.72, rho window 0.84, both 2.67; w = 1 11.11 → 36.47, 1.19, 3.90; p = 131101: w = 0.1 1.78 → IC adds 7.66, rho window 0.71, both 3.04; w = 1 9.17 → 41.54, 0.98, 4.44. Under "rho window" alone, these arms would be below R at all three prices: C_unit_t_capped_fast at p = 16411, 32771, 131101. The band verdict of this study is therefore not robust to pricing rho's compares alone; under "both" every arm stays above R at w = 0.1 and 1, and under "IC adds" the distances grow 3–5×. | consistent with `band.rs`; **disclosure gap**: §5a's first bullet asks for every unpriced counter of every arm with its count, and neither rho's window compares nor the uncounted hash/table work is listed | `ic_control.rs` `run_rho`; `ic_boundary.rs` `WalkTally::work_native`, `RhoWalk::negation`; `ecbench/band.rs` module doc; `python3 -I scripts/pricing_sensitivity.py /home/user/crypto out out/pricing_sensitivity.json` |
| X3 | **§5a calibrated reading (owner rule 2026-11-12) is not reported.** Each run records a best-of-N `w_cal` per prime (`calibration_ns_informational`: `w_rel` ≈ 0.003–0.005, `w_la` ≈ 0.009–0.014) and a `total_cal` per row (recomputed: 0 mismatches), but RESULTS gives no order at w = 0, w_cal and 10·w_cal. Recomputed (arm/R at 0 / w_cal / 10·w_cal): arm W p = 16411: 0.164 / 0.306 / 1.532; p = 32771: 0.154 / 0.293 / 1.497; p = 65537: 0.113 / 0.265 / 1.589; p = 131101: 0.137 / 0.238 / 1.118. W and R were priced with their own runs' w_cal; one common w_cal gives the same order (`out/capped_calibrated_same_wcal.txt`). So no arm is "ranked at calibrated price" below rho (10·w_cal > 1 everywhere); the omission hides no ranking. | **inconsistent** (required reading omitted; no effect on the band verdict) | `python3 -I scripts/calibrated_reading.py /home/user/crypto out/calibrated_reading.json` |
| X4 | **Solver independence from the secret.** `Instance::t_secret` is read only in instance construction and in the `#[cfg(test)]` module at every freeze commit; no solver path reads it. | consistent | `git show <freeze>:src/cryptanalysis/chained_transfer/ic_control.rs \| grep -n t_secret` vs the line of `mod tests` |
| X5 | **Reproducibility bindings.** Recorded: commit, dirty flag, argv, start time, wall time, host CPU and platform (bench record). Not recorded: the Rust toolchain version, `--threads`/`--curves-map` in `run.json`'s `cfg` (they are in the bench record's argv). Counts are deterministic in (p, curve seed, seed) and do not depend on the thread count (replay, above). | partially consistent (toolchain not recorded) | `out/bench_records_check.txt` |
| X6 | **Unit tests named by the protocol** (the oracle, LA-mode, unit-T and capped-walk tests at the freeze commit): not re-run. A `cargo test --release` build of this crate took another session about 35 minutes and 10 GB on this host during the validation, and the dispatch asked for modest compute. The test names exist at each freeze commit (6, 8, 9 and 10 tests in `ic_control.rs`), matching the counts RESULTS gives where it gives one. | could not check | `git show <freeze>:src/cryptanalysis/chained_transfer/ic_control.rs \| grep -A1 "#\[test\]"` |


## Disposition

`validated` for the measurement: the binary is the frozen commit's build (bit-identical rebuild), it replays every p ≤ 2053 W, S and R row exactly, every replayed log passes an independent certificate, G1 holds field for field, and all 54 transcribed numbers match. Scope limits as in the Lucas report. Corrections needed (ERRATA): state that W equals the unit-T study's U row for row at p = 16411, 32771 and 131101 (and 4 of 12 runs at 4099), so the W/S gain and the W/R values there are U's, already reported, and the capped walk and fast fallback act only at p ≤ 4099 and the fallback primes (W8); relabel the "LA incl. additions" column (W11); the w = 0.1 extrapolation is 2^21.7 by the fit, not 2^20–2^21 (W13). Disclose X2, in particular that W (= U there) would be below rho at all three prices at 16411, 32771 and 131101 if rho's dropped window compares alone were priced (not under the repository's band rule, and not when the IC's additions are priced too), and X3.
