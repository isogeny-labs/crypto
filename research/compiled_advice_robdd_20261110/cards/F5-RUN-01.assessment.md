# Assessment of CARD-compiled-advice-robdd-20261110-F5-RUN-01

Facts only. No interpretation of the scientific result.

## Classification

**Completed as specified** (receipt status `completed`; all 13 commands ran once, in card order, exit 0).
Observations below are listed as discrepancies/notes; none is an executor deviation.

## Checks

| Check | Source | Result |
|---|---|---|
| Receipt status, command count | F5-RUN-01.receipt.json | `completed`, commands_total 13, ids in card order (verify-freeze ... result-table), each `attempts: 1`, `succeeded: true`, `timed_out: false`, `exit_code: 0` |
| Receipt card_sha256 | receipt vs sha256sum of card | e946af9134b86caed6dd735ad8904f262f88cb65f0aae12825de425357fe4a79, equal |
| verify-freeze lines | F5-RUN-01.verify-freeze.stdout | `PROTOCOL.md: OK`, `tool/src/main.rs: OK`, `tool/Cargo.toml: OK`, `tool/Cargo.lock: OK`; stderr empty |
| Binds still match disk | sha256sum now | PROTOCOL.md e57b8ef0...76d1, FREEZE.sha256 9f4ef688...970a, Cargo.toml 01cec379...5278, Cargo.lock 2912a6c7...8c29, main.rs c0f41c93...d019, binary 7bb1a710...b2: all equal the card binds |
| git-head | git-head.stdout | da7ac2dbb8dc920652be41efb0be3ee14eaa17a7 = card `commit` |
| Test result line | F5-RUN-01.test.stdout | `test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s` (4 tests listed, all ok) |
| Binary sha256, step vs card | binary-sha256.stdout vs card binds | 7bb1a710a6ae3d3df15a78cde927275195f5cf3c521337d058c6bcbf339f8bb2 in both; equal |
| run.json sha256, step vs disk | result-sha256.stdout vs sha256sum results/run.json | 1ab1b4fded683de2fd55b9e53e5f3830a47b9a7229d2f5b2c99d6e18631f0f9a in both; equal. Receipt `outputs` entry for run.json also begins 1ab1b4fd |
| run.json header vs FREEZE | results/run.json `header` | protocol_sha256 e57b8ef0...76d1 = FREEZE.sha256 PROTOCOL.md line; source_sha256 c0f41c93...d019 = FREEZE.sha256 tool/src/main.rs line |
| run.json header settings | header | ns [16,18,20,22,24], rand_per_set 10 (= `--rand 10`), quick false, study compiled_advice_robdd_20261110 |
| Stdout table vs run.json | results/run.stdout.txt; result-table.stdout (byte-identical, `cmp` equal, sha256 814902e7...78e2 in receipt for both run-full and result-table) | All 5 curve header lines and all 15 rows (X2, XE, INT per n) regenerated from run.json `curves`/`rows` (nodes in order msb lsb sm sl; ratio and relsd to 3 decimals) match the printed lines exactly. Spot checks: n=16 X2 nodes 892 834 872 831, ratio cells see note 1; n=16 XE 8307 8290 8311 8284; n=24 X2 28158 27794 28263 27905; n=24 XE 1114066 1114057 1114064 1114055; n=24 INT 24 24 24 24 |
| Footer lines | stdout vs run.json summary | `decision: closed` = summary.decision_rule_outcome `closed`; `peak_rss_kb=610432` = summary.peak_rss_kb 610432 |
| Fatal / verification flags | run.json | `fatal: []`; summary.all_verifications_ok true; summary.p2_all_interval_le_2n true; every row/order `verified: true` |
| run-full exit | 11-run-full.exit.json | exit_code 0, 1044.765 s, finished 2026-11-10T19:54:43Z, timed_out false; run.stderr.txt 0 bytes |
| Empty stderr | receipts | all step stderr empty except build.stderr (`Finished release ... in 0.01s`) and test.stderr (`Finished ...`, `Running unittests ...`), which are cargo status lines |
| Writes inside write_scope | `git status --porcelain research/compiled_advice_robdd_20261110` | empty (results committed in b9b4f0a340 "F5: run F5-RUN-01 receipt and results"); receipt `dirty_after` lists only paths under research/compiled_advice_robdd_20261110/ for this card |

Note 1 (table spot-check detail): the n=16 X2 row printed is
`nodes[msb lsb sm sl]=892 834 872 831  ratio(relsd)=1.029(0.012) 1.010(0.006) 1.000(0.011) 1.007(0.009)`.
My regeneration from run.json gives 1.029(0.013) for the first pair: run.json `ratio_sd_from_rand_spread` is 0.01280243582678113, which rounds to 0.013 with Python `%.3f`, while the tool printed 0.012. See discrepancy D3.

## Discrepancies

- D1 (card defect, not executor): `python3 tools/task_card.py lint` on the card now prints `CARD FAIL`: `commands[11].argv` and `commands[12].argv` name `research/compiled_advice_robdd_20261110/results/run.json` and `.../results/run.stdout.txt`, which `binds` does not pin (these are run outputs, not inputs). The card was nonetheless run; the receipt records the card sha256 above. Lint did not stop execution; no lint output was supplied for the card before the run.
- D2 (note): the `build` step was a no-op (`Finished release ... in 0.01s`, 0.114 s); the binary on disk was built 2026-11-10 19:33:56 before the card, as the card `notes` state. Binary hash recorded by the card's step equals the bound hash, so no rebuild difference exists.
- D3 (formatting only): the printed relsd in n=16 X2 msb is 0.012 whereas Python `%.3f` of the run.json value 0.0128024... gives 0.013 (the tool, probably Rust, formatting/rounding or truncation differs; other 39 of 40 printed ratio(relsd) cells match my `%.3f` regeneration). Underlying run.json values are unchanged and the stdout table is byte-identical between run-full stdout and result-table. Not a card deviation; stated for completeness.
- D4 (note): the receipt `dirty_before` list contains paths of other studies (coord_spectrum_20261110, dp_hybrid_ic_design_20261110, lattice_bnb_design_20261110, p2_lift_linear_complexity_20261110) plus this card file; nothing from those studies is in this card's write scope or was touched by it per this card's `dirty_after` (only paths under compiled_advice_robdd_20261110/ for this card; other-study entries in `dirty_after` are not in this card's argv).
- D5 (not performed): `python3 tools/task_card.py check-relay <card> --report <file>` was not run because no executor report file was supplied in the dispatch; `relay_verbatim` is therefore not assessed.
- D6 (route, per card notes): local host execution, not the default isolab-eks route; the card notes state the EKS route was unavailable (no aws CLI) and that this is local evidence with the section 9a confirmatory EKS run open. Not an executor deviation; counts are exact, no timing is a result.
- D7 (receipt commit): receipt/card commit da7ac2db...17a7; repository HEAD is now b9b4f0a340 (later commit holding the results). No difference in bound file hashes.

No unlisted command ran (receipt argv list equals the card argv list in order).

```yaml
card_id: CARD-compiled-advice-robdd-20261110-F5-RUN-01
relay_verbatim: not_assessed
outcome: executed_as_specified
stopped_at_step: null
recommendation: proceed_to_analysis
reasons:
  - receipt status completed, 13/13 commands exit 0, once each (F5-RUN-01.receipt.json)
  - test line "test result: ok. 4 passed; 0 failed" (F5-RUN-01.test.stdout)
  - binary, run.json, protocol and source hashes match binds/FREEZE.sha256 (receipt step outputs and sha256sum)
  - stdout table equals run.json (40 ratio(relsd) cells and all 60 node counts compared; one relsd rounding difference D3)
  - card lint now FAILS on unpinned output paths (D1)
interpretation: not_assessed
```
