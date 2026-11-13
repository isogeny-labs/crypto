# Validation 20261113 progress (target: research/negation_remeasure_20261113/)

HEAD at start: 47dc95a934eff93f8b1e0a72d487439872a44392 (target uncommitted in working tree)
SP=/tmp/claude-0/-home-user/b3fd4014-06f7-5e0c-8338-36d52644c351/scratchpad

## Done
- out/git_status.txt, out/git_diff_stat.txt, out/snapshot_start.sha256 (target + code hashes at 05:09:52Z)
- read VALIDATION_GATE.md, AGENTS sections, prior REPORT.md, t4 VALIDATION.json, target PROTOCOL/RESULTS/PROGRESS
- frozen.sha256 OK; record raw sha256 (9 files) all OK -> out/record_raw_check.txt (uploaded:false on all)
- frozen source snapshot: $SP/validator2/tree (scripts/make_snapshot.py), hashes out/snapshot_tree.sha256
- build started 05:2xZ: cd $SP/validator2/tree && flock $SP/cargo.lock env CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=$SP/validator2/target cargo build --release --locked --offline --bin negation_remeasure > $SP/validator2/build.log

- out/recompute_tables.json: 162 arm-rows recomputed, 0 mismatches; 214 record metrics match (scripts/recompute_tables.py)
- out/old_values_check.json: 478 checks vs results_v3/tower_large/pushforward, 0 mismatches; NOTE tower p64007 'old' column is 4-seed reproduction, not recorded value (scripts/check_old_values.py)
- out/j_check_v3_cover.json: 11 curves (v3 4, cover 7) j matches, no j=0/1728 (scripts/jcheck.py, check_j_original.py)
- scripts/val_nr_dump.rs written; copied to $SP/validator2/tree/src/bin/ (validator tool, not producer code)

- out/headlines.json: RESULTS.md headline numbers + N1-N9 scorecard recomputed; all match (scripts/check_headlines.py). 0 below rows; 6 'unresolved (lower bound)' rows (cover first-collision arms); 1 row where neg vs best-of class differ (cover p419 birthday)
- scripts/check_nr_certs.py written (vlib-based cert check of val_nr_dump output + ties means to parts)

- out/merge_check.json: merged remeasure.json == parts (8272 fields, 0 diffs; only curves_checked_all added from _aut)
- out/results_md_tables_check.json: 1497 RESULTS.md arm-table cells match recompute
- out/stat_times.txt: PROTOCOL.md ctime 03:00:01 (< first run 04:20:02); source mtime 04:23:57 (after v3/cover/tower runs -> first build differs, Deviation 6)
- FINDING: arms A/B/C labelled complete but walk partition hash (mix64 per step), large-prime HashMap, DSU not emitted (ic_control.rs run_ic_with); N7 robust (needs >=165 extra native/op to reverse)
- replay dir prepared: $SP/validator2/replay (recorded inputs copied); scripts/compare_replay.py written

- BUILD DONE 05:35 (20m12s); negation_remeasure sha256 97ba6257...; val_nr_dump hash in $SP/validator2/bin_negation_remeasure.sha256
- REPLAY: v3 (840 non-timing values) / cover (2492) / tower<=16001 (2373): 0 diffs; out/replay_*_diff.json, out/replay/*.replay.json; cal classes stable (out/replay_cal_stability.txt)
- CERTS: v3 364/364 logs (out/certs_v3.json), cover 784/784 (out/certs_cover.json); tamper control rejects (out/tamper_control_v3.txt)
- tower dumps running in background -> $SP/validator2/dump_tower_le4001.jsonl (+ .done), dump_tower_8009_16001.jsonl

- CERTS tower: p<=4001 3480/3480 (out/certs_tower_le4001.json), 8009/16001 1392/1392 (out/certs_tower_8009_16001.json); dumps hashed in out/dumps.sha256 (files in $SP/validator2)
- end-snapshot check at 05:39: 95 files unchanged; working-tree src == build snapshot
- curve-only dumps for 32003/64007 running (6 procs) -> $SP/validator2/dump_tower_curve_<p>_<i>.jsonl; check with scripts/check_tower_curves.py out/tower_curves_32003_64007.json <files>
- scripts/make_gate_drafts.py written (pending mode -> out/gate_draft/, never into the target dir)

- tower curves 32003/64007 (6) checked: out/tower_curves_32003_64007.json; all 38 curves j != 0,1728 independently
- gate drafts (pending status) written: out/gate_draft/{replay-certificate.json,validator-report.json,VALIDATION.draft.json}; gate checks: only status errors; with status flipped 0 errors
- DECISION: not admitted now (raw data + code uncommitted; record binds dirty 47dc95a without the code; role rule). No VALIDATION.json written in the target dir.
- replay of tower p=32003 started 05:45 in $SP/validator2/replay (replay_32003.log, out/remeasure_tower_p32003.json)

## Pending
- NONE. Validation complete 05:48Z. Final: NOT ADMITTED (insufficient_evidence: uncommitted artifact set / binding). No VALIDATION.json in target dir.
- REPORT.md write was blocked by the harness (subagent report-file rule); the full report is in the handback message.
- tower p=32003 replay: 339 values, 0 diffs (out/replay_tower_p32003_diff.json)
- end check: out/snapshot_end_check.txt (95 files unchanged at 05:48:04Z)
- cleanup: $SP/validator2/target and $SP/validator2/tree deleted (binary hashes in out/binaries.sha256); dumps kept in $SP/validator2 (hashes out/dumps.sha256)

## To admit later (after the producer commits the exact bytes)
- sha256sum -c the target lines of out/snapshot_start.sha256 against the committed blobs; record closed and raw uploaded
- python3 -I scripts/make_gate_drafts.py pass ; write research/negation_remeasure_20261113/VALIDATION.json from out/gate_draft/VALIDATION.draft.json (re-pin hashes; drop status_note DRAFT)
- python3 scripts/validation_gate.py evidence research/negation_remeasure_20261113 -> valid
