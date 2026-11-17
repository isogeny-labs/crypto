card_id: CARD-ecbench-batched-screen-20261101-1
card_sha256: 62b2be31379b6b6580e12d88386dcf9ee0daba7aa80cae584187ce5443da7689
relay_verbatim: not_checked (no executor report file supplied; check-relay not run)
outcome: executed_as_specified
stopped_at_step: null
recommendation: proceed_to_analysis
interpretation: not_assessed
inferential_validity: not_assessed

reasons:
  - Receipt research/ecbench_batched_screen_20261101/cards/card-1.receipt.json (sha256 9a39d159f27aaef8208c75213c38f60db1cb4d195b763cc5a075ca68dceb31a0) has status completed, commands_total 63, card_sha256 equal to the card file's current sha256, commit 39f147d96bc78a46a2e124ec466f9b8abb44f7f6 (= git HEAD), dirty_before empty.
  - All 63 commands (checkpoint-gate through list-outputs) have exit_code 0, attempts 1, timed_out false, succeeded true; no command skipped, none retried (sum of attempts = 63).
  - tools/task_card.assessment_facts computed outcome executed_as_specified, outputs_changed_since_run [], writes_outside_scope []; all 128 dirty_after paths are inside the card write_scope.
  - Declared outputs: 8 (records.jsonl and session.json per cell, from the run-<cell> commands); all exist and the sha256 recomputed from disk equals the receipt digest for each.
  - Sessions (n13a0, n17a1, n19a0, n23a0): session.json status complete, executions_planned 1600, records_written 1600, status_counts {verified: 1600}, last_seq 1599; records.jsonl has 1600 lines, seq contiguous 0..1599, every outcome.status verified and matches_target true.
  - Audit files audits/<cell>.verify.txt: ok true, problems [], records 1600, verified_records 1600, replay_selection all, replay_candidates 1280, stderr line "audit OK: 1600 records, 1600 verified, 1280 replays reproduced, 0 problems" for all four cells; the audit-gate command printed AUDIT-GATE PASSED with reproduced 1280 for each.
  - Spec in each session (sessions/<cell>/spec.json) is byte-identical to specs/<cell>.json, and its sha256 equals the card-bound digest (n13a0 6aead47e..., n17a1 6fd12efd..., n19a0 5e8e647b..., n23a0 37504b15...).
  - A/A control ic-b64-aa against ic-b64, ops.ratio_b_over_a 1.0, ops.ci95 [1.0, 1.0], ops.status ok on n13a0, n17a1, n19a0 and n23a0; aa-gate printed AA-GATE PASSED each time.
  - Protocol conformance read from specs and records on every cell. Arms in order ic-b1, ic-b2, ic-b4, ic-b8, ic-b16, ic-b32, ic-b64 (baseline), ic-b128, ic-b256, ic-b64-aa (control), method ic.koblitz_batched, batch_trials equal to the arm name, factor_base_points 78, 102, 114, 138. targets_per_curve 32; target_seed 202611011001, 202611011002, 202611011003, 202611011004; measurement rounds 4, warmup 1, order alternate_measured (Amendment 1), seed 202611012001, isolation_required L0. Records per arm 160 (10 arms x 32 targets x 5 rounds = 1600); rounds 0 (warmup) to 4, 320 records each.
  - Isolation: every record isolation.level L0 (blocker "L1: no CPU reservation"), session options cpus none, allow_busy true, as Amendment 1 specifies; run argv match run.sh (--cpus none --allow-busy --settle 1 --quiet; compare --a ic-b64 --b <arm> --save).
  - Binary: session.json binary_sha256 and host.json build.binary_sha256 are 447a6d3c4842b5dadb5c1601e9648cda5cc08c41668bfd87741e6fc369bc24c0 in all four cells (host.json has no top-level binary field; the digest is under build); the audits' auditor_binary_sha256 is the same; sha256sum of ./target/release/ecbench now is the same; equals the card-bound digest.
  - Session git_commit 39f147d96bc78a46a2e124ec466f9b8abb44f7f6, git_dirty false, for all four cells.

deviations_from_protocol: none found.
records_not_verified: none (0 of 6400).

observations_not_deviations:
  - PROTOCOL.md "Instrument" names source commit 729847dcea (branch claude/ecbench-batch-trials); the sessions record git_commit 39f147d96b (this worktree HEAD) and the binary digest matches the card binding. The binary was not rebuilt or compared to 729847dcea by this assessment.
  - The within-round arm order (alternate_measured) was not re-derived; only the spec value was checked.
  - Disk was 415M available at start and 345M (100%) at the end (card-1.disk-start.stdout, record-disk-end).
  - No executor report file was supplied, so tools/task_card.py check-relay was not run and relay_verbatim is unchecked; the verdict rests on the tool-written receipt and files on disk, which the tool's own facts agree with.
  - Per-arm ratios and tables (comparisons/, tables/) were not read or interpreted.
