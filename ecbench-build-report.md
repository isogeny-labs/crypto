# ecbench build report (2026-10-30)

Untracked, not committed. No research state changed, no result claimed.
No source patched; no tracked file altered; nothing pushed; no PR.
The smoke runs are a correctness/plumbing check only (the spec says "Not a
performance claim"); no S, ratio or timing from them is a measurement.

## Result
| item | value |
|---|---|
| commit built | `11173ecd0230a9ceedc4d833736402934fdcb95e` (merge of PR #2967; detached checkout) |
| contains repair | yes: PR #2972 merge `2c8f16e2eca1d864dcb658ac2fe4b5dacda60e36` is an ancestor (`git merge-base --is-ancestor`) |
| binary | `target/release/ecbench`, 61462456 bytes |
| binary_sha256 | `b38cffb4f64c4ec7289ee4ef19bf60560d2d99684d4f996114eb403560081ec4` |
| smoke | pass (72/72 executions `verified`, twice) |
| replay | pass (`--replay 8` on both sessions, `--replay-all` on session 1: 54 reproduced, 0 problems) |
| determinism | pass (counted fields of all 72 records identical across two sessions; see below) |

The auditor's own binary hash in the receipt equals the sha256 above.

## Source and toolchain
- HEAD at build: `11173ecd0230a9ceedc4d833736402934fdcb95e`, detached, fetched
  with `git fetch origin 11173ecd…` (reachable; the fallback SHA was not needed)
- `git status --porcelain` before the build and after everything: only
  `?? ecbench-build-report.md` (this file); tracked tree clean
- `rustc 1.97.0 (2d8144b78 2026-07-07)`, `cargo 1.97.0 (c980f4866 2026-06-30)`
  (no `rust-toolchain` file in the repo root)
- Paths materialized (`scripts/sparse-checkout.sh add --path …`):
  `research/pencil_large_prime_20261029/` (still present after the checkout;
  no other path was needed)

## Build
- Command: `cargo build --release --bin ecbench`, started 2026-10-30T18:59:23Z
- Result: `Finished release profile [optimized] target(s) in 7m 03s`; `time`:
  real 7m3.290s, user 18m20.936s, sys 0m54.057s; exit 0

### Earlier failed attempts (same session, for the record)
1. At `3c13bc69…`, before materializing the sparse path: exit 101 in 0.1 s,
   `failed to read research/pencil_large_prime_20261029/Cargo.toml`.
2. At `3c13bc69…`, after materializing it: exit 101 after 5m14.8s,
   `error: this file contains an unclosed delimiter` at
   `src/cryptanalysis/binary_semaev_chain_sat.rs:1703:3` (a bad merge left two
   `if` guards in `chain_s3_max_variables`; fixed upstream by PR #2972).
   `3c13bc69…` therefore does not build and must not be used as the instrument
   commit.

## Smoke test (outputs trimmed only where noted)
Spec: `docs/ecbench/specs/smoke-prime.json` (9 arms: rho.negation, rho.plain,
rho.frozen_reference, bsgs.textbook/interleaved/negation, kangaroo.vow,
ic.pipeline mitm, and an A/A control; one registered 20-bit prime-order curve
`icv1-fp20-t727-cd198a38`; 2 targets; 3 rounds + 1 warm-up = 72 executions).
Scratch outputs are in the session scratchpad, not in the repository:
`…/scratchpad/smoke-1`, `…/scratchpad/smoke-2`.

1. `./target/release/ecbench plan --spec docs/ecbench/specs/smoke-prime.json`
   exit 0. Printed a warning that order `alternate` with 3 measured rounds
   cannot be balanced (use an even round count or `alternate_measured`), then
   `spec ECS1h94f54998e76f … 72 executions` and the 9 arms/2 workloads.
2. `./target/release/ecbench run --spec docs/ecbench/specs/smoke-prime.json --out <scratchpad>/smoke-1`
   exit 0, real 5.9 s. Last line:
   `session ECBS1hc26b85ad45c3 complete -> …/smoke-1 (72 records: {"verified": 72})`.
   Every line read `verified`; e.g. `72/72 rho-neg … verified S=1.776 L1`,
   `ic-mitm … verified S=13.308 L2`.
3. `ecbench verify --dir …/smoke-1 --replay 8` exit 0, schema
   `ecbench.audit/v1`, `ok: true`, `problems: []`, every check true; replays
   `reproduced: true, detail: "identical"`. Final line:
   `audit OK: 72 records, 72 verified, 8 replays reproduced, 0 problems`.
4. `ecbench verify --dir …/smoke-1 --replay-all` exit 0. Final line:
   `audit OK: 72 records, 72 verified, 54 replays reproduced, 0 problems`
   (54 deterministic runs eligible). Receipt file hashes: host.json
   `a7975cc0…451d`, plan.json `56981572…ef629`, records.jsonl `8493227f…b226d9b13c`,
   session.json `3ad74479…61e0`, spec.json `684c9aeb…79cc2`.
5. Second session, same command: `ecbench run … --out …/smoke-2` exit 0,
   `session ECBS1hacd00ca37ab1 complete … (72 records: {"verified": 72})`;
   `ecbench verify --dir …/smoke-2 --replay 8` →
   `audit OK: 72 records, 72 verified, 8 replays reproduced, 0 problems`.
   Re-running `verify --replay 8` on session 1 with a fresh random seed also
   gave `audit OK … 8 replays reproduced, 0 problems`.

## Determinism check
The harness has no separate self-test; determinism was checked by running the
same spec twice (session ids differ: `ECBS1hc26b85ad45c3`, `ECBS1hacd00ca37ab1`)
and comparing, per record keyed by arm, target index, round and warm-up flag,
the fields: `algorithm_seed`, `cost.total_gae`, `cost.s`, `phases` (with
`wall_ns` removed), `counters`, `work`, `field_ops`, and `outcome` (without
`stderr_sha256`). Sorted, the two 72-line files are byte-identical
(sha256 `ccb2cc632e65fc763a4decae143e208351bcb2ccb6eb6434796c9679c836c988`
for both). All 72 records carry `cost.deterministic: true`.
Not compared: `time`, `online` and `isolation` blocks, which contain wall-clock
and host measurements and differ between runs by design. Caveat: my first two
comparisons differed only because `phases[].wall_ns` and `online.wall_ns` are
timing fields inside otherwise counted blocks; stripping timing, not counts,
removed the difference.

## Host facts (measured)
- Disk `/`: 252G size, 21G used, 19G avail (53%); memory 15 GiB, 14 GiB
  available, no swap; load average 1.46 / 2.33 / 1.28 at the first check
  (session hooks and a SageMath install were running), so not idle
- CPU: Intel Xeon ("Intel(R) Xeon(R) Processor @ 2.10GHz"), 4 CPUs, 1 thread
  per core, hypervisor KVM (full virtualization); kernel 6.18.44-fc-v185
- PMU: `perf` is not installed, so `perf stat -e instructions true` was not run
  and I installed nothing. The harness's own probe recorded
  `hw_solve.error = "perf_event_open: No such file or directory"` with null
  instructions/cycles, so no hardware PMU counters were readable here
- Isolation as graded by ecbench itself (per record): level **L1**. Blockers
  recorded: L2 — the 20-bit solve (~115–125 µs) is under the 1.0 ms minimum for
  wall time; L3 — run CPU 3 is in neither isolcpus nor nohz_full, governor is
  None rather than performance, turbo/boost not known off, not known to be
  bare metal. Only the ic-mitm rows show L2 because those solves are long
  enough. The L2 blockers other than solve length were not itemized for L1
  records beyond what is quoted here
- This is a shared cloud VM with other processes running and no verified
  isolation. It can support counted metrics (operation counts, which the
  determinism check shows reproduce exactly) but not timed ones; wall-clock
  figures need the spec's L2 level (ecbench L0–L3, isolab tiers A–D), which
  this host has not been shown to meet. It should not be the L2 host for the
  frozen scaling-study protocol

## cargo test (requested afterwards; at `11173ecd0230a9ceedc4d833736402934fdcb95e`)
Command: `cargo test --release --locked --no-fail-fast` (the form CI uses),
started 2026-10-31T06:24:41Z; `time`: real 65m31.5s, user 230m13s; exit 101.
Totals over 121 suites: **6382 passed, 15 failed, 138 ignored**. Failures are
in three targets (`error: 3 targets failed`). Nothing was fixed or edited.

Library (`--lib`), 7 failures:
- `cryptanalysis::binary_semaev_chain_sat::tests::source_bound_covers_degree_83_without_constructing_a_model`
  (`assert_eq` at `binary_semaev_chain_sat.rs:1105`, left `Some(190568)`)
- `cryptanalysis::koblitz_index_calculus::tests::chained_s3_four_to_six_summands_lift_on_the_original_curve`
  (m=4, `ChainS3Stats` all zero: `patterns_attempted: 0`, `solver_calls: 0`)
- `cryptanalysis::koblitz_index_calculus::tests::chained_s3_frobenius_domain_lifts_and_rejects_incomplete_orbits`
  (m=4, same all-zero stats)
- `cryptanalysis::koblitz_index_calculus::tests::chained_s3_strategy_feeds_verified_relation_rank_and_target_solve`
  (m=4, `relations: 0`; expected value is `None`)
- `ml_dataset::restore::tests::native_budgeted_restore_retains_failed_attempt_and_retries_with_read_debits`
  ("existing remote object has 459 bytes and digest None; expected 459 bytes and c7ceb2d2…")
- `ml_dataset::storage::tests::native_budgeted_publication_resume_catalog_lab_receipt_and_fetch_share_debits`
  (same "digest None" error)
- `ml_dataset::storage::tests::resumed_publication_rejects_same_size_remote_corruption_without_overwrite`
  ("existing remote object has mismatched size or digest; refusing overwrite")

`ecdlp-audit` binary, 2 failures: `tests::checked_in_plan_passes`,
`tests::pinned_source_replays_historical_blob_despite_worktree_divergence`
(both: "pinned source revision is not a commit … git exited 128", pinned SHA
`da6ae6d1…`).

`tests/backlog_cli.rs`, 6 failures: `a_conflicted_found_still_stops_every_runner`,
`concurrent_claims_resolve_to_one_holder_after_sync`,
`every_seed_card_publishes_unchanged`,
`first_success_found_is_set_only_on_a_checked_published_artifact`,
`publish_claim_exec_release_and_status_round_trip`,
`the_stop_rule_fails_closed_on_a_missing_lab`. For
`first_success_found_…` the message is "backlog: cairn 15.0.0 is outside the
supported range 8.0.0 <= v < 10.0.0"; I read the message for only that one.

Likely causes (not verified): `ecdlp-audit` — this checkout is shallow, so the
pinned commit is probably absent; `backlog_cli` — the installed cairn 15.0.0
is outside the supported range; `ml_dataset` — unknown, possibly the sandbox
storage backend; Semaev/Koblitz — look like logic failures, not environment
(the three Koblitz tests share a "no relations at m=4" symptom; a connection
to the `chain_s3_max_variables` m-range repair is a guess). My count is 15; the
peer's was 16, and I did not reconcile the difference.

How the run got there: attempt 1 (`cargo test --no-fail-fast`, debug) stopped
at target resolution because two targets (`volcano-ic-summarize` bin,
`conductor_gap_ecbench` example) point into sparse-excluded directories; I
materialized `research/full_volcano_graphwide_ic_20261017/` and
`research/conductor_gap_cm_orbits_20261015/`. Attempt 2 ran out of disk while
linking (examples alone were 9 GB); I deleted `target/debug` (my own output)
and set `CARGO_PROFILE_*_DEBUG=0`, `CARGO_INCREMENTAL=0`. Attempt 3 hit the
2-hour background cap with 2337 of 5732 library tests finished in debug mode
(4 failures seen, a subset of the 15; no summary). The release run above is the
complete result. The `ecbench` binary hash in this report is unchanged
(`b38cffb4…081ec4`; re-hashed after the release test run, file mtime still
2026-10-30 19:06:27, so `cargo test --release` did not rebuild it).

## Not done / caveats
- The peer session noted that 16 ml_dataset/ecbench tests fail on main. The
  `cargo test` results are in the next section (15 failures); the smoke run,
  replay and determinism checks above all passed on the built binary.
- Only the 20-bit smoke spec was run; no other family, size or method
  parameters were exercised. Nothing here validates larger sizes or the
  prime/random-binary families of the scaling study.
- Used the checkpoint hook's `CHECKPOINT_SKIP` for the build, read-only
  commands and the tiny smoke runs (each time with a stated reason); no
  `.resume/RESUME.md` was created and nothing was pushed to S3, because this was
  a correctness check, not an experiment. If the owner counts it as a run,
  that bypass should be reviewed.
- No Conductor task claimed (no tracked file changed); no session-log entry
  written (that would modify a tracked file, which the task forbids).
- The scratch sessions (`smoke-1`, `smoke-2`) are outside the repository and
  will be lost with the container.
