# Independent validation: negation re-measurement (2026-11-13)

Validator role, for the target `research/negation_remeasure_20261113/`. The
harness did not let the validator write this file, so its returned report is
transcribed here. Raw outputs are in `out/`, the scripts in `scripts/`, and
the resume checklist in `PROGRESS.md`. Every figure is `[pcc:unverified]`.

## Disposition at hand-back: insufficient evidence (commit binding only)

Every measurement check passed. The single failure was that the artifacts were
uncommitted: the record bound a dirty `47dc95a` that lacked the code. Those
exact bytes are now committed in `8bf77ba333`. All 25 target lines of
`out/snapshot_start.sha256` match that commit, and the record is closed in the
following commit. Admission (writing `VALIDATION.json`) is left to a validator
pass that confirms the committed hashes.

## Checks

| # | Requirement | Result |
|--|--|--|
| 1 | `frozen.sha256` | pass. Protocol ctime 03:00:01Z, before the first run at 04:20:02Z; this is filesystem evidence, not git |
| 2 | Record raw sha256 | pass, 9 of 9 |
| 3 | Certificates (independent `vlib` and validator code) | pass, 6,020 of 6,020 logs. A tamper control was rejected (28 errors) |
| 4 | Replay | pass. Every cell of v3, cover, tower p ≤ 16001 and tower p = 32003; 0 differences outside timing |
| 5 | Tables recomputed from raw JSON | pass. 0 mismatches across 162 arm-rows, 214 metrics, 478 recorded values, 1,497 RESULTS cells and 8,272 merged values |
| 6 | Strongest rho | pass. Tuned negation walk. The automorphism fold is vacuous: none of the 38 curves has `j = 0` or `1728` (checked independently) |
| 7 | Same process, native counters emitted on both sides | pass, with the lower-bound gaps below |
| 8 | §5a band | pass. 156 of 162 rows above at 0, 0.1 and 1; 6 unresolved lower bounds; 0 below |
| 9 | Calibrated reading | pass. It agrees with the band except on the same 6 rows (best-of-N timing, labelled as such) |
| 10 | Same unit | pass as a counting convention. A Jacobian step costs 4.3–22.5 elliptic additions in time, so "IC above rho" is conservative |
| 11 | Bound to a commit | was the failure; now addressed by `8bf77ba333` |

## Discrepancies (none changes a class)

1. **Binding.** The measured parts came from an earlier build whose source was
   not kept. The source files were last modified during the p = 32003 run.
   Replaying from the committed source closes this gap except at tower
   p = 64007.
2. **"Old" at p = 64007.** The "old" column at p = 64007 is a 4-seed rerun,
   not the recorded original. For example, the g3 old ratios are 141.78 /
   159,687 / 1,595,592 against the recorded 141.97 / 160,179 / 1,600,508.
3. **Arms A/B/C are lower bounds.** They are labelled complete, but they omit
   the walk's partition hash, the large-prime HashMap and the DSU
   (`ic_control.rs:1129–1155`). Reversing any order would need at least 165
   (v3), 4,585 (tower g3) or 2,587 (cover) extra native operations per arm-C
   group operation.
4. **Other lower bounds.** The v3 and cover recorded collectors emit no
   relation-phase work, and the cover first-collision arms emit no counters.
   These account for the 6 unresolved rows.
5. **`metrics[].n`.** It holds the rho run count, not the arm sample size.
6. **Disclosed deviations, confirmed in the data.** Cover rho runs on seed 0
   only. Tower p = 64007 used 4 and 2 seeds instead of 8 and 4. Rho word ops
   are priced, which is conservative.

## Not verified

- The p = 64007 runs were not replayed; only their instances were rebuilt.
- Solver operation counts were reproduced only by re-running producer code.
- S3 copies of the raw files; none were uploaded because there are no
  credentials.
- Pre-data freezing, beyond filesystem ctime.

## Follow-up pass, 2026-11-13: commit binding confirmed, admitted

This is a separate validator pass. It was not the producer and not the pass above.
It ran from HEAD `4c6db51b17e5e2242db44ccbeb4b9f83a757c22d` with a clean tracked tree on
`claude/ecdlp-chained-transfers-8cpiek`. **Disposition: validated.** `VALIDATION.json`
is written and the gate reports `valid`. Standing is `[pcc:unverified]`: there is no pcc
claim id and no keyring signature.

The paragraph under "Disposition at hand-back" that says all 25 lines match was
written into this file before any validator had checked it. This pass checks it below
and confirms it.

| # | Check | Result |
|--|--|--|
| 1 | The 25 target lines of `out/snapshot_start.sha256` against `git show 8bf77ba333:<path> \| sha256sum` | pass, 25 of 25. The 24 non-record files also pass `sha256sum -c` in the working tree |
| 2 | The other 70 lines (ecbench sources and `src/bin/ecbench.rs`) at `8bf77ba333` | pass, 70 of 70 |
| 3 | Record diff from `8bf77ba333` to `4c6db51b17` | pass. Changed: `closed_utc`, `reports.pdf`, `reports.svg`, `status` (`open` to `measured`) and `summary`. Unchanged: the 214 metrics, 9 raw entries and commands. The raw hashes equal the committed blobs |
| 4 | Remainder of `4c6db51b17` | pass, additive only. `REPORT.pdf` has the same bytes as `benchmarks/20261111-chained-prime-transfer/REPORT_negation_remeasure_20261113.pdf` (`35fa9fca…`). The benchmarks `remeasure.svg` has the same bytes as the target's (`3060a750…`). Also added: validator evidence, the session log (+30/−0), the problem addendum (+9/−0) and the regenerated progress report. Nothing in `src/` or in the 24 measured files changed |
| 5 | Replay build tree (`out/snapshot_tree.sha256`) against the `8bf77ba333` tree | 3,651 of 3,656 files equal. The 5 that differ changed on the base: `src/bin/ic/bench.rs`, `src/bin/icx/prime_external.rs`, `src/cryptanalysis/fes_gpu.rs`, `src/cryptanalysis/ic_framework/solvers.rs` and `src/cryptanalysis/ic_run.rs`. A static check finds no reference to them from `src/cryptanalysis/chained_transfer/`. No rebuild was done at the commit |
| 6 | Validator scripts re-run on the committed bytes | pass. Outputs equal the committed `out/` files; the replay diffs differ only in their path strings. `recompute_tables` gives 162 arm-rows and 214 metrics against the closed record, 0 mismatches. The merge, headline, old-value and RESULTS-cell checks pass. Replay comparisons: 840, 2,492, 2,373 and 339 values, 0 differences |
| 7 | Certificates re-verified (`check_nr_certs.py`, vlib) on the scratch dumps, whose hashes equal `out/dumps.sha256` | pass, 6,020 of 6,020 logs (364 + 784 + 3,480 + 1,392). Outputs are identical. The tamper control still fails with 28 errors. The tower 32003/64007 curve check is identical |
| 8 | Scratch copies | pass. `validator2/snap_check.sha256` equals the hash lines of `snapshot_start.sha256`. The scratch replay outputs equal `out/replay/*.replay.json` |
| 9 | `make_gate_drafts.py pass` | pass after the edit below. It reports 405 headline entries and 0 failures. `pending` mode reproduces the committed pending drafts byte for byte |
| 10 | `python3 scripts/validation_gate.py evidence research/negation_remeasure_20261113` | `valid`. In-memory controls (a perturbed headline, a bad raw pin, a pending status, a non-strongest rho) are each rejected |

**Edit to `scripts/make_gate_drafts.py` before running it.** As written, `pass` mode would
have issued a certificate marked `verified` [pcc:unverified] with `head 47dc95a`, `dirty: true` and "status open",
hashing working-tree files. `pass` mode now does the following:

- It re-checks the binding itself and stops on any failure: checks 1 and 3, and that the
  tracked tree is clean.
- It binds `source.commit` to `8bf77ba333…` with `dirty: false`, sets
  `record_close_commit` to `4c6db51b17…`, and describes the replay build tree (check 5).
- It hashes the source files and raw data from the committed blobs.
- It reads the binary list from `out/binaries.sha256`, which has the same lines as the
  scratch copy.
- It corrects the unit note from "5-6" to "4.3 to 22.5 elliptic additions" per Jacobian
  step. Recomputed from `t_add_ns`: the word-implementation and tower Jacobians give 4.29 to
  9.78, the recorded implementations 20.1 to 22.45.
- It records the findings below.

`pending` mode is unchanged apart from that unit wording.

**Files written:**

- `research/negation_remeasure_20261113/VALIDATION.json` (`eba6273e…`), pinning
  `out/gate_draft/replay-certificate.json` (`0f29135e…`) and
  `out/gate_draft/validator-report.json` (`2b205945…`)
- `out/gate_draft/VALIDATION.draft.json` (`1ea0bc9b…`)
- `scripts/make_gate_drafts.py`
- this section, and one line in `PROGRESS.md`

The pending drafts remain in `4c6db51b17`.

### Findings (none changes the evidence directory or a class)

1. **The record summary is not supported by the data.** It says "recorded gaps
   understated 5.4-9.2x", and the problem addendum repeats "5.4–9.2×". The data give a
   recorded-plain-rho to negation-rho total of 5.50–9.22 (v3), 5.21–8.00 (cover) and
   4.35–7.21 (tower), so 4.35–9.22 overall (`out/headlines.json`). At the headline cells,
   the recorded-arm distance moves by these factors at 0 / 0.1 / 1 GAE:
   - v3: 9.2 / 7.7 / 3.1
   - cover: 6.3 / 4.4 / 1.2
   - tower genus 3: 4.3 / 2.8 / 0.62

   The tower genus 3 distance at w = 1 shrinks, so the gap is not understated at every
   price. The producer should correct both texts.
2. **The record summary's verdict direction is inaccurate.** It says "16 verdict changes,
   all toward 'above'". Ten go to `above` and six go to `unresolved (lower bound)`;
   `RESULTS.md` states this correctly.
3. **The record keeps its run-time binding.** It still shows commit `47dc95a`,
   `commit_dirty: true` and `uploaded: false` on all 9 raw files. That is accurate for
   the run. The S3 copies are still missing, which is a benchmark-records requirement, not
   a gate requirement.

### Not verified, beyond the list above

- A rebuild and replay at `8bf77ba333` itself (see check 5).
- The tower p = 64007 replay.
- The S3 copies.
