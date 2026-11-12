# Assessment of CARD-fp31-phase5-percurve-20261112-1

card_id: CARD-fp31-phase5-percurve-20261112-1
card sha256: 9e90a1f34840216a9fcbed6ce89ef18ab7bc72f120baa808a9127b3748a7ede8 (matches the receipt's card_sha256)
relay_verbatim: not checked (no saved report file was supplied; `task_card.py check-relay` was not run). The numbers in the
  dispatch (receipt status, exit codes, the four digests, 727 ok / 41 FAILED) all match the files on disk.
outcome: executed_as_specified (41 of 768 sessions are missing under PROTOCOL section 4: "a refused or failed session is
  recorded as missing and not replaced")
stopped_at_step: null
recommendation: proceed_to_analysis (the main session decides whether the 41 missing sessions need a separate, new card;
  the frozen protocol does not replace them)
interpretation: not_assessed

## Receipt and digests
- receipt.json: status completed, commands_total 4; exit codes 0,0,0,0; run-phase5 13667.371 s, detached, one attempt.
  Start 04:22:23 UTC (stderr mtime), end 08:10:11 UTC (stdout mtime), consistent with 13667 s.
- Receipt commit 5f8552da4ca3 = parent of the later re-pin commit 327524a55c (committed 04:22:36 UTC, 13 s after this run
  started). 327524a55c changes only the card's `commit:` line (93a944db -> 5f8552da). The card sha256 on disk equals the
  receipt's card_sha256. The receipt's dirty_before lists the card file (uncommitted re-pin at start).
- stdout sha256 73dabfed...f0a9 (recomputed, equal); all stderr files empty (e3b0c442...); analyze stdout empty.
- Recomputed output digests equal the executor's: analysis_rho 3127210793de...4f10, analysis_ic3 6a4fa1441077...5f4555f,
  analysis_ic2 dbed1c05801e...a511a823.
- The 7 binds were recomputed and equal both the card and PROTOCOL.sha256: PROTOCOL.md 2a8467cf, run_phase5.sh fd378c37,
  classes.json f5e04adc, followup.rs 48e1d925, stats.rs df661a28, target/release/ecbench 86a9421d,
  fp31-class-census 8690aaf9.
- Observation: `task_card.py lint` run now prints CARD FAIL (commands[1..3].argv name analysis_{rho,ic3,ic2}.json, which now
  exist in the repository and are not in binds). It is a lint property of the post-run tree, not an execution deviation.

## Writes outside write_scope
- `git status --porcelain` at assessment time showed only the three analysis_*.json (inside write_scope; untracked at that
  moment). runs/ is git-ignored by phase5/.gitignore. No other path was dirty. HEAD has since moved to 836ea3538e through
  the main session's own commits (not the executor's).

## FAILED sessions
- stdout: 727 `ok`, 41 `FAILED`, 0 `skip`, 0 other lines; 727+41 = 768 = 48 classes x 16 sessions. The 727 ok lines are
  unique; the set of FAILED lines equals the set of `*.partial` directories (41) and no session has both `X` and `X.partial`.
- Every one of the 41 run.log files ends with "ecbench: host is not quiet (PSI cpu some avg10 <v> > 5.00); wait, or pass
  --allow-busy ...". Reported PSI values 5.05 to 11.17. Each partial holds only run.log, spec.json, spec.log (no session).
- Timing (run.log mtime, UTC):
  - 04:22:26-04:22:58: 13 (c008 rho s0-s7, ic3 s0-s3, ic2 s0)
  - 04:24:24-04:24:34: 5 (c009 rho s5-s7, ic3 s0-s1)
  - 04:26:15-04:26:26: 5 (c010 rho s2-s6)
  - subtotal 04:22-04:26: 23
  - 05:08-07:39: 18 = c022 rho s7 (05:08:05); c024 rho s4,s5 (05:13:53); c026 rho s6,s7, ic3 s0,s1 (05:22:21); c034 rho s7,
    ic3 s0 (05:56:36); c039 rho s2-s6 (06:22:09); c042 rho s7 (06:39:22); c047 rho s5,s6 (07:13:33); c051 rho s4 (07:39:14)
- The first FAILED (04:22:26) is 10 s before the commit time of 327524a55c (04:22:36). Whether the main session's activity
  caused the PSI reading cannot be determined from the files; the 18 later failures occur in bursts of 1 to 5 consecutive
  sessions, each at PSI 5.05-6.62.
- Per class, missing sessions: c008 rho 8/8, ic3 4/4, ic2 1/4 (rho and ic3 have no data for c008); c009 rho 3, ic3 2; c010
  rho 5; c022 rho 1; c024 rho 2; c026 rho 2, ic3 2; c034 rho 1, ic3 1; c039 rho 5; c042 rho 1; c047 rho 2; c051 rho 1.

## Ok sessions (727 = rho 353, ic3 183, ic2 191)
- Every one has done, spec.json, session/, audit.json, seconds (checked by script).
- Every audit.json: ok true, problems [], session_status complete, records_bound true, all `checks` true, records ==
  planned_executions, 2 replays all reproduced.
  Note: `verified_records` equals `records` for all rho sessions; for ic2/ic3 it is 0 (all IC rows have status `exhausted`),
  as in c008/ic2_s1/verify.log "audit OK: 36 records, 0 verified, 2 replays reproduced". Records: 77088 rho (all `verified`),
  9972 ic3, 10236 ic2.

## Spec against PROTOCOL section 3 (all 727 specs)
- measurement: rounds 1, warmup 0, order shuffle, isolation_required L2, timeout 7200; `seed` = 124 + session index.
- workloads: target_kind planted_shared; targets_per_curve rho 8 / ic3 2 / ic2 2; target_seed = 20262113 + s (rho),
  20264113 + s (ic3), 20263113 + s (ic2). The PROTOCOL's formula "20261113 + 1000*kind + session" holds with kind index
  rho 1, ic2 2, ic3 3 (not asserted by PROTOCOL; recorded as found).
- Session counts per kind: rho 8, ic3 4, ic2 4 (slots); curves per session = class curves + iso_models in classes.json, every
  curve with the class's p and group_order == r == n of classes.json.
- rho arms: plain (rho.plain, reference), plain-aa (rho.plain, control), control (rho.negation, candidate), no params.
- IC arms: main / main-aa (reference / control) and pc (candidate), ic.pipeline, prime-abscissa, linalg
  incremental-gauss-full-rank, oracle mitm:m=3 (ic3) or mitm:m=2 (ic2).
  - ic3: |F| 378, pc 402 in all 183 specs. max_trials 5965 in 136, 5964 in 47.
  - ic2: |F| 1448, pc 1586 in all 191 specs. max_trials takes the values 102418-102427 (ten values).
  - max_trials depends on the class's n. It equals the p2tool rule as coded in followup.rs (ic_params), recomputed
    independently for every class. PROTOCOL's "5,965" and "102,425" are the pilot values, so they do not hold literally for
    every class (specification-text difference, not an executor deviation).
- Curve order: the permutation is a seeded shuffle of the class's curves. Within a class and kind, all done sessions have
  different permutations in 133 of 142 class-kinds. The 9 with a repeat: c009 rho (4 curves, 5 sessions, 4 distinct), c010 ic3
  and ic2 (3 curves, 4 sessions, 3 distinct), c014 rho (6 curves, 8 sessions, 7 distinct), c018 rho (3 curves, 8 sessions, 3
  distinct), c019 rho (4 curves, 8 sessions, 7 distinct), c022 rho (3 curves, 7 sessions, 5 distinct), c022 ic3 and ic2 (3
  curves, 4 sessions, 3 distinct). PROTOCOL section 3 says "a different permutation per session"; this is not met in these
  9 class-kinds (for c018 rho and c022 rho it is impossible by count, 3! = 6 < 8; for c010/c022 ic and the others a distinct
  permutation existed). This is a property of the frozen p5-spec tool, not of the executor.
- Group order is the same for all curves within a class (as it must be for isogenous curves and their models); it differs
  between classes.

## Analyses against FAILED lines
- The `missing` list of each analysis equals exactly S minus the FAILED count per class and kind:
  rho: c008 0, c009 5, c010 3, c022 7, c024 6, c026 6, c034 7, c039 3, c042 7, c047 6, c051 7 (found of 8);
  ic3: c008 0, c009 2, c026 2, c034 3 (found of 4); ic2: c008 3 (found of 4). All three agree with the stdout FAILED lines.
- classes: rho 47, ic3 47, ic2 48 (c008 has no rho or ic3 data).
- records_measured 77088 / 9972 / 10236, equal to my count of records.jsonl lines; rho_records_not_verified 0 in all three.

## Reported for the reasoning tier (not interpreted)
- Row isolation grades: records_not_L2 in the analyses (rho 4769, ic3 535, ic2 485; total 5789) equals my count of records
  graded L1 in records.jsonl (rho 72319 L2 / 4769 L1; ic3 9437 / 535; ic2 9751 / 485). They lie in 667 of the 727 ok
  sessions. In c055/rho_s0 the stated L1 blockers are "2 involuntary context switches during a N ms solve (at most 1)",
  "run-queue delay ... % of the solve", and "solve too short to admit wall time (< 1.0 ms minimum)". PROTOCOL section 5
  states no exclusion rule for non-L2 rows.
- Receipt dirty_before shows the card file path truncated ("esearch/...") as written by the tool; no consequence found.
- No command beyond the card's four ran (4 attempts_log entries, 1 attempt each).
