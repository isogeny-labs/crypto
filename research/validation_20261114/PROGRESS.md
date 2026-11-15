# Validator V progress (2026-11-14), resumable checklist

Targets: A = research/full_jacobian_ic_20261113 (PR #4071), B = ecbench w_cal (PR #4070).
Validated from origin/main 224624a91a97639781ff336f2e8dc9a0738035b3.
Scratch: /tmp/claude-0/-home-user/b3fd4014-06f7-5e0c-8338-36d52644c351/scratchpad/valV/

## A
- [x] A0 binding: #4071 = c88c7b4e0c (code+data+protocol, one commit), 008a5cf3fb, f199720572 (rustfmt bin); merge cc075763d7; target files unchanged to HEAD 224624a91a; raw sha256 match record (out/: art.sha256 via scratch export)
- [x] A1 freeze ordering: git cannot order (same commit 18:56 after run 17:29). Scratch evidence: smoke 17:21:51 (no_calibration), record init 17:22:14, run 17:29:02-17:36:10 (benchrun.out == run.log sha), clippy fixes 17:39 AFTER run
- [x] A2 recompute: scripts/recompute_fj.py -> out/recompute_fj.json: 2057 values, 0 mismatches; 36/36 above; verdict unresolved. Text findings: '18.7 for A' (B 17.96 smaller), F6 range 0.483-0.540 not 0.536, rho-vs-ln r 0.234 and go/sqrt r 0.420/0.292 are 9-prime fits (stage1: 0.238, 0.074/-0.033)
- [x] A3 replay: FULL recorded command (all 9 primes) with HEAD-export binary cde53af3..: 3017 non-timing values 0 diff, 135 calibrated classes 0 diff; run.log identical mod wall/path (out/replay_compare.json, out/replay/). Record command/args/raw hashes consistent
- [x] A4 certificates: val_fj_dump (scripts/val_fj_dump.rs, bin c636b470) + check_fj_certs.py: 360/360 IC logs on E1 and J_C, 736/736 rho logs; 468 means equal; j not 0/1728; L(1)=#E1#E2 p<=3203; curves consistent (check_fj_curves.py). F6 per-run: 3/176 runs >0.6 at p=211
- [x] A5 verdict 'unresolved' matches frozen rule; findings listed in out/gate/validator-report.json
- [x] A6 VALIDATION.json written (gate: valid); out/gate/ via scripts/make_gate_a.py

Build: scratch export of HEAD at valV/tree (scripts/export_build_tree.py), target valV/target, build_head.sh (bins full_jacobian_ic, ecbench) -> valV/bin_head/
Next for A: replay full recorded command into valV/replay, compare; build val_fj_dump (copy scripts/val_fj_dump.rs into tree/src/bin), run, check_fj_certs.py

## B
State: B2 HEAD outputs -> valV/b2/out (then moved to valV/b2/head) via scripts/b2_outputs.sh; broot = git archive HEAD of docs/bounds + sessions; bound_records_ok.txt = 94 records with sessions in Git.
B1: tests at HEAD tree with research/docs overlay symlinks (valV/overlay_links.txt; remove with overlay_links.py ... remove before syncing tree) -> valV/test_head.log
Next: swap tree to ec56f03 (export to valV/tree_pre_tmp, sync_tree.py), build ecbench pre, run b2_outputs.sh into b2/out -> b2/pre, b2_diff.py
- [ ] B0 binding: commits of #4070
- [x] B1 HEAD tests (band/compare/challenge/bounds files == 97a8e0d): 67 pass, 6 fail; 5 = missing S3 k20-s0 records (4 band + 1 compare); 6th (bounds committed_version_1) was my symlink overlay, passes with real copies (out/b1/)
- [x] B2 pre (ec56f03, bin 7310255a) vs HEAD (bin f6801264): 21/23 stdout/json byte-identical, the 2 others are verify receipts differing only in audited_unix_ms and auditor_binary_sha256; all stderr identical; no calibrated key; 3 bound fits == committed; proc-1 verdict != committed under BOTH (pre-existing drift). Bounds calibrated path exercised with sidecar (out/b3/bound_fit_...). comparison_id ignores w_cal (finding)
- [x] B3 hand-check e3b f4-c1-sem-chain-t2 with sidecar w_cal 0.0285 (scratch copy): 9/9 grid cells, orders, ranked flag, band calibrated pair all equal (out/b3_handcheck_e3b.json)

## Report
- [x] REPORT.md: harness refuses report .md files from the validator; full report returned in the handback
- [ ] handback
Scratch: valV/target removed after use to free disk (binaries kept in valV/bin_head, valV/bin_pre; valV/tree now holds the ec56f03 export)
