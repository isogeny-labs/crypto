# Validator report VAL-20261119-1246f3: v5 jump-index tally (61e4022114) and the v5-vs-strong re-measurement on 8 Koblitz curves

This report independently validates two claims on branch `claude/f4-f5-algorithm-speed-bvp7qd`:

1. The source change `61e4022114`. `rho.signed_frobenius_v5` now reports `partition_hashes_uncharged` equal to its walk
   steps. This is the counter name and per-step definition `rho.signed_frobenius_strong` uses. It is listed in
   `cost.unpriced`, and nothing else in any report changes.
2. The re-measurement `research/koblitz_v5_tally_remeasure_20261119/`: protocol `9c525b01f5`, results `87c8e2f970`.

The validated snapshot is `87c8e2f970`, which HEAD `f09779a5cf` carries unchanged (the later commit touches only logs). The
validator did not design, write or run any part of the study. It edited no producer file, and it read no other validator
report on this study (none exists). Every producer number cited below is `[pcc:unverified]` until the Coordinator records
this report.

Validator nonce `6941f7b0f8715209a1220a2022757e83`, drawn with `os.urandom` at 2026-11-19T19:13:38Z. That is after the
results commit (19:06:37Z) and after the pushed HEAD (19:07:13Z). The nonce sets the session order a0-n19, a1-n29, a1-n23,
a1-n17, a0-n23, a0-n13, a0-n31, a0-n37 and picks the pre-tally sessions n29-r2 and n29-r3. Validator scratch:
`/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/val-v5t/`. The tarball was extracted there as
data. The validator's scripts are in `work/`, which shares no code with `analyze.py` or ecbench.

## Verdicts

| claim | verdict |
|:--|:--|
| (1) v5 jump-index tally `61e4022114` | **passed**, with findings F1-F3 |
| (2) v5 / strong re-measurement, 8 curves, w = 0 / 0.1 / 1 | **passed**, with findings F4-F9 |

Passing means the receipts are admissible evidence. It does not change any leaderboard row. It does not satisfy the
validation gate by itself; F7 (raw data not durable) must be closed first. It is not an IC improvement: changing the
reference is an accounting change.

## Claim 1: the tally

### Code reading (`git show 61e4022114`)

- **v5 side.** `ic_boundary::TunedWalker::walk` runs one loop iteration per step. That iteration does
  `j0 = index_of(h)` and `jump_index_selections += 1` before the candidate loop, and `steps += 1` after it. The candidate
  loop always breaks, because `tried < jump_count` bounds it, and no return separates the two increments. So the two
  counts are equal on every walk, including lost, capped and out-of-budget walks. The only walk paths that increment `steps` are `rho_reference`,
  `TunedWalker::walk`, `rho_walk_lanes` and the parallel walker. `rho.signed_frobenius_v5` goes
  `signed_frobenius_rho_v3` -> `rho_walk_v3` -> `rho_search_from` -> `TunedWalker::walk` only, so
  `RhoResult.jump_index_selections == RhoResult.steps`. Then `rho_report_v2` writes `counters["steps"] = res.steps`.
  `unpriced_of` lists every `*_uncharged` counter with a positive count, so the tally is priced whenever steps > 0.
- **Strong side** (`koblitz_strong_rho.rs`, `solve_accounted` lane loop). Each live lane in each round does one
  `partition(point)` with `partition_hashes += 1`, one group addition, one canonicalisation and `steps += 1`. So
  `partition_hashes == walk_steps`, and that held on all 384 strong records of every session. **The two counters
  measure the same thing: one jump-index selection per walk step.**
- **What is still not tallied** (finding F2, small). v5 computes `key`, `mix` and, through the non-short-circuit `&`
  test, `index_of` for each look-ahead-rejected candidate. It also computes `mix` for each escape point and each walk
  start. None of these is a selection, and none is tallied. Each rejected candidate is still charged a full addition
  (`look_ahead_additions`) and a canonicalisation. Strong, for its part, computes a `dp_hash` per live lane per round
  (about one per step) and a 5-point fruitless-cycle compare per step, and tallies neither. v5's window and Brent word
  operations go to `work_native` (bitop/v1), not to the GAE band. Neither arm's cycle checks are band-priced.
- **Other methods.** `jump_index_selections` is `#[serde(skip)]`, kept out of `counters`, set to 0 in the other
  constructors, and inserted into a report only in the `rho.signed_frobenius_v5` dispatch arm.
- **Audit.** `compare_replay` removes the counter from the replay only when the record lacks it
  (`v5_pre_tally_record`). See F3.

### Empirical checks

| check | result |
|:--|:--|
| tally == `steps` and listed in `cost.unpriced`, every v5 record in the 8 sessions, warm-ups included | 3072 / 3072 (384 per curve) |
| strong `partition_hashes_uncharged == walk_steps` | 3072 / 3072 |
| **Cross-binary re-execution.** Pre-tally binary `ecbench-sw-217638524d` (sha256 `7b182360…d315`, pinned and validated in VAL-20261118-24d204) re-ran the frozen specs of a0-n19, a1-n29, a1-n23 and a0-n37 (768 records each); field-by-field diff against the producer's records, time and id fields excluded | strong: 1536 / 1536 identical. v5: 1536 / 1536 identical once the tally is removed from the new record (counters, cost, phases with natives and `work_native`, outcome), and the tally equals steps. The **only** other difference is the record-level `work` block (F1). Between `217638524d` and `61e4022114` the source differs only in IC code (factor base, pair table, oracle counters, `ic_framework`), one IC bitop rule and the tally commit. |
| Pre-tally binary `verify --replay-all` on the new sessions (a0-n19, a1-n29, a1-n23, a0-n37) | 288 problems per session: exactly the 288 measured v5 records, none on any other record. Expected, because it does not know the counter. |
| **Other methods.** 11 other rho methods (`rho.signed_frobenius`, `_v2`, `_v3`, `_v4`, `rho.negation`, `negation_v4`, `_v5`, `_v6`, `plain_v5`, `_v6`, `signed_frobenius_strong_v2`) x 4 targets on a1-n23: `profile-input` + `exec` with both binaries | 44 / 44 solve reports identical. v5: 4 / 4 identical apart from the tally, which equals steps. |
| **Pre-tally records elsewhere.** `koblitz_lean_sib_confirm_20261118` sessions n29-r2, n29-r3 (nonce) and n37-r1 (binary `43f44fec…`, whose rho code equals the tally parent's: the only shared-path change is a new `CountedGroup::x_sibling_pair` trait method defaulting to `None`, used by IC `walk_sib`). Record sha256s match that study's `summary.json`. | `verify --replay-all` with the pinned binary: 1152 records, 864 replays, **0 problems** each. Every v5 record (576, warm-ups included) re-executed with the pinned binary: **576 / 576 report a tally equal to the old record's `steps`**, with all other counters, `total_gae`, `unpriced` and the recovered scalar identical. The DECISION rule "price a pre-tally record with `+ w · steps`" is therefore exact on these records. |
| Unit test `v5_jump_index_tally_adds_one_counter_equal_to_steps_and_changes_nothing_else` | Read. It asserts what it says on 4 curves x 6 targets, plus strong's `partition_hashes == walk_steps` on n17. **Not re-run by the validator** (no rebuild: 3.0 GB free, a 28-42 min single-job build). The producer's `test3.keep.log` reports 49 passed. Its two earlier runs (`test.log`, `test2.log`, exit 101) were compile errors from fixture files missing in the sparse worktree, not test failures. The cross-binary checks above test the same assertions on 3072 + 576 + 44 solves with a separately validated binary. |

### Findings, claim 1

- **F1. The bitop/v1 `work` block of every v5 record changes**, by +12 word operations per step (for example
  a1-n29 seq with 17 steps: `prims.word_op` gains 204, and `bitops` goes from 610461 to 623517). The cause is that
  `work.rs` prices `partition_hashes_uncharged` at `(WordOp, 12)`, the price strong's partition hashes already carry.
  The doc comment on `V5_JUMP_INDEX_COUNTER` intends this ("so `ecbench::work` and the §5a band price the two alike").
  But the README says "Nothing else changes", and the commit message, DECISION and unit test do not mention the `work`
  block. The test compares `SolveReport`s, and the `work` block is computed later, at record level. The GAE band
  is unaffected. A bitop/v1 comparison that mixes pre-tally and tallied v5 records needs `+12 · steps` word operations on
  the pre-tally ones.
- **F2. Residual untallied hash work** (see the code reading). Worst one-sided case: v5 is additionally charged `w` per
  look-ahead candidate, escape and walk start, and strong is charged nothing for its own untallied `dp_hash`. v5 / strong
  at w = 1 then moves from 0.1040 to 0.1139 (n13), 0.1435 to 0.1522 (n29) and 0.7295 to 0.7367 (n37). No order changes.
  The step-level definitions match.
- **F3. Pricing and replay of pre-tally records are manual and keyed on absence.** No tool applies the `+ w · steps`
  derivation: `ecbench band` and `compare` price a pre-tally v5 record without the tally. The DECISION allows only
  in-session v5 to set the reference, and the in-session v5 records that exist (lean_sib_confirm, k1_setup_w1, …) are
  all pre-tally. So every min(strong, v5) row must add `w · steps` by hand; it is exact (576 / 576 above). Separately,
  `v5_pre_tally_record` exempts any v5 record without the counter, whatever binary wrote it. A post-tally record with
  its tally stripped would therefore replay clean. Keying the exemption on the writing binary (or on the record date),
  and applying the derivation in `band.rs`, would close both gaps. Producer and Coordinator decision; not repaired here.

## Claim 2: the re-measurement

### Artifact and provenance checks

| check | result |
|:--|:--|
| `REGISTRATION.sha256` (PROTOCOL, `make_specs.py`, `run.sh`, `analyze.py`, 8 specs) | 12 / 12 OK. Unchanged from `9c525b01f5` to HEAD. |
| Protocol before data | `registered-at` 18:19:11Z, commit 18:19:13Z. Run started 18:19:27Z (benchmark record). Sessions ran 19:00:52Z-19:04:04Z (`session.json`). a0-n13's `wall_s = 2490` is a flock wait behind the producer's 41-min test build (`test3.keep.log`); wall time is not used. |
| Smoke check disclosed (PROTOCOL §6) | `v5t/smoke` at 18:18:36Z, before registration. a = 0, n = 13, 2 targets, seed 777001, arm `rho-v5` only, pinned binary, tally 8 = 8 and 12 = 12, never analysed: matches the disclosure exactly. |
| Raw sessions `v5t-sessions.tar.gz` | sha256 `eed5c2e8…d4b5`, equal to the benchmark record's raw entry. 8 sessions, each `status: complete` with 768 records, every record carrying the pinned binary hash. |
| Binary | `ecbench-v5tally` sha256 `92ae9054…44cb` = PROTOCOL = `run.sh` pin = `logs/binary.sha256` = every session and record. Build worktree `wt-w1`: tracked sources equal `61e4022114` (`git diff 61e4022114 -- src Cargo.toml Cargo.lock crates` empty, no untracked sources). Binary built 18:18:24Z, 33 s before the commit (F8). |
| Specs | Each session's `spec.json` is JSON-equal to the frozen `specs/<curve>.json`. |
| Audits | Each committed `audits/audit-*.json.gz` decompresses to the tarball's `audit-*.json`. |
| Pinned-binary `verify --replay-all` (validator run, all 8 sessions; the three nonce sessions first) | 768 records, 768 verified, 576 replays reproduced, 0 problems, for every session. Each audit equals the producer's, byte for byte, apart from `audited_unix_ms`. |
| Coverage | Per session: 4 arms x 48 targets x (3 rounds + 1 warm-up) = 768. 144 complete measured (workload, round) quadruples. Same target in all 4 arms; 144 distinct seeds; 0 censored; 0 unpaired. |
| Pricing inputs | On all 6144 records: `cost.unpriced` equals the `*_uncharged` counters with a positive count; `total_gae` equals the phase sum; `s = total_gae / √r`. No `counted:` markers, no phase natives, no calibration, so priced GAE(w) = `total_gae + w · Σ unpriced counters`. |
| Independent [d]G = Q (own GF(2^n) and curve code, `work/dgq.py`) | Field polynomial picked as the first trinomial or pentanomial under which G is on the curve. Per curve: the group order recomputed by the Koblitz Lucas recurrence equals the record's `group_order`; h·r = order; r prime by trial division; [r]G = O. **4608 / 4608 measured records**: Q on the curve and [d]G = Q with 0 ≤ d < r. |

### Recomputation (`work/recompute.py`, own code)

The validator recomputed the ratio of totals (paired by workload and round) and compare's linearised t(47) interval
(t = 2.011741, workload clusters, influence SE). Over 8 curves x 3 weights x 3 ratios, both agree with the producer's
recorded `compare` output to better than 1e-9 relative. The validator also re-ran `ecbench compare --resamples 4000 --seed
20261002` with the pinned binary for a0-n19, a1-n29, a1-n23 and a0-n37 at each weight; all 12 outputs (ratio, both
intervals, band) are identical to `results.json.gz`. A separate two-stage cluster bootstrap with the validator's own RNG
(4000 resamples) lands within about 0.003 of compare's bootstrap ends.

v5 / strong, interval = envelope of compare's linearised and bootstrap 95% intervals. All 24 cells equal the README, the
table, `summary.json` and the benchmark record's 24 metrics:

| curve | log2 r | w = 0 | w = 0.1 | w = 1 | validator bootstrap at w = 1 (own RNG, 4000) | look-ahead / steps |
|:--|--:|:--|:--|:--|:--|--:|
| a0-n13 | 10.97 | 0.062 [0.051, 0.082] | 0.067 [0.055, 0.090] | 0.104 [0.076, 0.149] | [0.075, 0.147] | 0.0067 |
| a1-n29 | 15.37 | 0.077 [0.072, 0.082] | 0.085 [0.079, 0.091] | 0.143 [0.130, 0.157] | [0.131, 0.157] | 0.0038 |
| a1-n17 | 16.00 | 0.097 [0.090, 0.103] | 0.108 [0.100, 0.115] | 0.181 [0.165, 0.197] | [0.165, 0.198] | 0.0043 |
| a0-n19 | 17.00 | 0.110 [0.101, 0.119] | 0.122 [0.112, 0.133] | 0.205 [0.184, 0.226] | [0.184, 0.226] | 0.0026 |
| a0-n31 | 20.46 | 0.174 [0.160, 0.191] | 0.194 [0.177, 0.213] | 0.312 [0.279, 0.347] | [0.279, 0.346] | 0.0011 |
| a0-n23 | 21.00 | 0.198 [0.181, 0.216] | 0.219 [0.200, 0.239] | 0.334 [0.299, 0.368] | [0.301, 0.369] | 0.0013 |
| a1-n23 | 22.00 | 0.251 [0.227, 0.276] | 0.277 [0.250, 0.306] | 0.413 [0.369, 0.462] | [0.366, 0.463] | 0.0011 |
| a0-n37 | 27.78 | 0.574 [0.507, 0.644] | 0.606 [0.534, 0.681] | 0.729 [0.638, 0.827] | [0.640, 0.827] | 0.0003 |

- **R1** holds: 0 unverified measured runs, the tally check passes on 384 / 384 records per curve, and both A/A ratios are
  exactly 1 at every weight (but see F4).
- **R2** holds: "v5 below strong" on all 24 curve-weight cells. The largest upper end is 0.827 (a0-n37, w = 1).
- **R3** holds: no crossover up to log2 r 27.78 at any weight.
- **R4** holds. At w = 0 against `koblitz_rho_reference_20261115/results.json`: n29 0.079 [0.075, 0.083], n17 0.093
  [0.089, 0.097], n31 0.168 [0.157, 0.180], n23 a = 1 0.261 [0.240, 0.284] and n37 0.581 [0.534, 0.632]. Each overlaps the
  new interval. The README's untallied w = 1 values (0.504, 0.310, 0.119) are what that file records. Its *derived*
  tally-charged w = 1 values, computed from old records plus `w · walk_operations`, all fall inside the new measured
  intervals: n37 0.737, n23 a = 1 0.428, n29 0.149, n17 0.174, n31 0.302.
- **Decay check.** The quantity should approach the walk-only ratio as r grows, because strong's fixed overheads shrink
  relative to √r. Measured: strong's set-up share falls from 0.789 (n29) to 0.324 (n37). Strong's extra walk steps
  (32 lockstep lanes, each leaving a post-DP trail) fall from 4.6x (n13) to 1.12x (n37). v5 / strong rises
  monotonically with log2 r, from 0.062 to 0.574 at w = 0. That is the expected direction, not an artefact. The parity
  near 2^39 is context from the 2026-11-15 study, not this protocol.

### Findings, claim 2

- **F4. A/A is a determinism control, not a noise control.** At every (workload, round), the A/A arms run with the same
  `algorithm_seed` as the arm they control, and counted GAE are deterministic, so A/A = 1.000 by construction. It checks
  pairing and determinism (which the replay also checks). Sampling noise is carried by the 48-cluster interval. The
  protocol pre-registered this control, so this is a limitation, not a failure.
- **F5. One "fresh" seed was not unused.** PROTOCOL §3 says target seeds 20261119601-608 were unused in the repository at
  freeze. 20261119601 (a0-n13's seed) is the `target_seed` of
  `research/ic_ladder_wwalk_ksum_20261115/results/specs/test/P44.json` (committed `beab06c168`, 2026-11-16) and appears
  in that study's PROTOCOL. That spec is a 44-bit prime-field curve, so targets are hashed into a different group and
  there is no overlap with a0-n13's targets. No effect on the result. The statement is inaccurate.
- **F6. "Wider of the two intervals" is the envelope.** `analyze.py` takes the minimum of the two lower ends and the
  maximum of the two upper ends. That is at least as wide as either interval and conservative. Wording only.
- **F7. Raw data is not durable.** The per-record sessions exist only in the session scratchpad tarball. The benchmark
  record lists it with `uploaded: false` ("store refused access"). The committed artifacts (audits, `results.json.gz`,
  `summary.json`, `tables/`) carry no per-record data. The receipt binds the tarball by sha256, and that hash verified,
  but a scratchpad clean-up would make the study unreplayable. Before any gate use (docs/VALIDATION_GATE.md: "raw data
  that is in the repository"), the Coordinator should upload the tarball by sha256 or commit compact per-record tables.
- **F8. The binary-to-source binding is behavioural, not bit-for-bit.** The sessions record `git_commit` and `git_dirty`
  as null. The binding rests on four things: the pinned sha256; the build worktree now equalling `61e4022114`; the binary
  behaving as that source says (pre-tally records replay clean, the counter is emitted only for v5, strong and 11 other
  rho methods are identical to a pre-tally binary); and the binary predating the commit by 33 s from the same tree. No
  bit-for-bit rebuild was attempted (disk, and Rust builds are path-dependent).
- **F9. One duplicated target.** In a1-n29, target indices 9 and 18 hash to the same point: a birthday collision in a
  group of r = 42457 (about 2.7% likely for 48 draws). Their seeds differ, and the effect is negligible.
- **Observation** (not investigated): a0-n31 is about 10x slower in wall time to run and replay (57 s) than the
  neighbouring curves. Wall time is not used; counted GAE are deterministic and replay exactly.

## Scope and non-claims

- Tested scope: the 8 Koblitz curves listed, log2 r 10.97-27.78, 48 public targets x 3 rounds each, L0 shared VM,
  counted GAE with the §5a band at 0, 0.1 and 1. No calibrated weight (`w_cal`) is involved: neither arm has
  field-arithmetic counters.
- Heuristic-validation checks: not applicable. No heuristic prediction is tested.
- Inventor-protocol §6 ladder: not applicable. No speedup is claimed; this is a reference measurement and an accounting
  change.
- Cost-model checks: the unit is declared (`ecbench.gae`; strong's scalar multiplications are charged at
  1.5 log2 r additions, an upper-bound convention the 2026-11-15 README discloses). Every run succeeded, so expected cost
  is not inflated by a success probability. Memory (DP-table entries) is counted but not compared, which is a
  limitation of a time-unit reference comparison.

## Artifact paths

- Report: `research/koblitz_v5_tally_remeasure_20261119/VALIDATOR_REPORT.md` (this file).
- Producer: `research/koblitz_v5_tally_remeasure_20261119/` (PROTOCOL, README, `summary.json`, `results.json.gz`,
  `audits/`, `tables/`, `logs/`), `research/koblitz_rho_reference_20261115/DECISION.md`, `README.md`, `results.json`,
  `benchmarks/records/20261119T1819Z-koblitz-v5-tally-remeasure.json`.
- Validator scratch (ephemeral), under `/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/val-v5t/`:
  `work/recompute.py` and `work/recompute.json`, `work/dgq.py`, `work/diffrecs.py`, `work/pretally_exec.py`,
  `work/replay/` (15 audits), `work/oldrun/` (pre-tally re-executions), `work/others/` and `work/nonce.txt`.

```yaml
validation_report:
  id: VAL-20261119-1246f3
  task_id: null   # dispatched in-session by the main session; no TASK-* card
  run_ids: [ECBS1haf18810ca58b, plus the other 7 session ids in v5t-sessions.tar.gz sha256 eed5c2e8360f02b83e1c09e0ac6fcebcafa491b6910934a4d2892b57e68ed4b5]
  snapshot: {source: 61e4022114, protocol: 9c525b01f5, results: 87c8e2f970, head: f09779a5cf}
  artifact_checks:
    - registration hashes 12/12; frozen files unchanged to HEAD
    - tarball sha256 = benchmark record; binary sha256 = pin = every record; specs JSON-equal; audits identical
    - pinned verify --replay-all 8/8 sessions, 0 problems, audits equal producer's except timestamp
  metric_recomputations:
    - 72 ratios + linearised CIs recomputed independently (<1e-9 rel); 12 compare re-runs byte-identical
    - 24 README cells and 24 benchmark metrics match
    - independent [d]G=Q 4608/4608; group orders by Lucas recurrence
  control_checks:
    - A/A = 1 exactly (same seeds: determinism control only, F4)
    - pre-tally binary cross-run: strong identical, v5 identical but the tally (and the bitop work block, F1)
    - 11 other rho methods identical across binaries
    - pre-tally lean_sib_confirm v5 records: tally == steps 576/576, replay 0 problems x3
    - smoke session matches its disclosure
  heuristic_validation_checks: []   # not applicable
  cost_model_checks: [unit declared (GAE); band 0/0.1/1 applied alike; worst-case untallied-hash sensitivity <= +0.014 at w=1]
  proof_architecture_checks: []     # not applicable
  verdict: passed                    # claim 1 passed; claim 2 passed
  limitations: [F1, F2, F3, F4, F5, F6, F7, F8, F9; Rust unit test not re-run; no bit-for-bit rebuild]
  artifact_paths: [research/koblitz_v5_tally_remeasure_20261119/VALIDATOR_REPORT.md]
review_attestation:
  joints_owned: [whole receipt; no review_plan]
  read_sibling_reports: false
  sources_read:
    - research/koblitz_v5_tally_remeasure_20261119/{PROTOCOL.md,README.md,REGISTRATION.sha256,registered-at.txt,run.sh,make_specs.py,analyze.py,summary.json,results.json.gz,tables/v5_over_strong.md,logs/*}
    - research/koblitz_rho_reference_20261115/{DECISION.md,results.json (by key)}
    - research/koblitz_lean_sib_confirm_20261118/{summary.json sessions, logs/binary.sha256, README.md (raw-data lines)}
    - research/koblitz_n37_sibwalk_k8_20261118/logs/binary.sha256 and VALIDATOR_REPORT.md (header and binary line only)
    - research/ic_ladder_wwalk_ksum_20261115/results/specs/test/P44.json (seed check)
    - benchmarks/records/20261119T1819Z-koblitz-v5-tally-remeasure.json
    - git show 61e4022114 (audit.rs, methods.rs, ic_boundary.rs, ic_wide.rs); ic_boundary.rs TunedWalker, rho_search_from, rho_walk_v3; koblitz_strong_rho.rs solve_accounted; methods.rs rho_report_v2, unpriced_of, strong report; band.rs pricing; compare.rs and stats.rs intervals; work.rs rules; ecbench.rs Exec
    - docs/VALIDATION_GATE.md; AGENTS.md section 5a; crypto-autoresearcher agents/validator.md, docs/agent-runtime-core.md, AGENTS.md "Review architecture"
    - producer scratch: v5t/{build.sh,build.log,benchrun.log,test*.log (summary lines),smoke/}, wt-w1 git state
  verdict: {tally: passed, remeasurement: passed}
```
