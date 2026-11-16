# Assessment: CARD-neural-distinguishers-20261102-1 and -2

Execution only. `interpretation: not_assessed`. No accuracy is read or interpreted here.

## Card -2 (recovery of cell aes-r4)

- outcome: executed_as_specified (completed). Receipt `research/neural_distinguishers_20261102/cards/CARD-neural-distinguishers-20261102-2.receipt.json`, sha256 b8902ba693050473678484f19fc2762e88d50c9dbbd6bd4a6d243d2327c753dc (recomputed from disk, matches the dispatch).
- Receipt: `status: completed`, `commands_total: 6`, all six `exit_code: 0`, `succeeded: true`, `timed_out: false`, `attempts: 1` each (no retries). Card sha256 c8d0b184... equals the card on disk when run. Receipt `commit` f0993f8282ddbd2216185a6f3dd4e31d6b27eabf equals the card pin and the step-1 stdout. `dirty_before` lists only the card -2 YAML; `dirty_after` is empty.
- Argv ran in order and are exactly the card's six; step 4 was `./target/release/neural-distinguishers run results aes-r4` (135.667 s, inside its 900 s timeout). Sum of command seconds is far below the 30-minute deadline.
- Digests recomputed from disk equal the dispatch and the receipt:
  - binary 41b8d479741c2a3958e392ddba453932347de2e126d293a1536b27786cbabb31 (step-3 stdout; also identical to card -1 step 3)
  - rows.jsonl ab49ea47153d58d4f34277d5f1fe9c698eeaa97aa58523ba3ea2493c9e24a506
  - summary.json 2405a91a078a1da0fffc4d3aba8528882928ed9bc282b128557a5acefa379b6c
  - fig_accuracy.svg 2066b66069239aeb09d314d9105bb6c42a29f795a312e222501f20e7d00f3758
  - step-6 stdout lists the same three digests.
- Source binds: the five bound files hashed to the card's values at lint and run time (the tool refuses otherwise). The build step reported "Finished ... in 0.03s", i.e. no recompile: the existing binary was reused. Its sha256 is the same as in card -1 step 3, so both cards used one binary.
- rows.jsonl: 38 lines, 35 distinct (cell, model, arm) keys, 7 cells; per-cell counts 5 each except aes-r4 with 8 (3 duplicated keys + 5 appended, the appended 5 being lines 34-38). The first 33 lines are card -1's rows, unchanged in position.
- Duplicated aes-r4 keys, `correct` in line 31/32/33 (card -1) versus 34/35/36 (card -2):
  - mlp/primary: 65746 and 65746, equal.
  - mlp/aa_seed2: 65588 and 65588, equal.
  - logreg/primary: 65737 and 65737, equal.
  - `accuracy` also equal in all three; only `train_seconds` differs (40.73/42.20, 40.89/41.11, 8.81/8.42). Observation of agreement only, no claim about why.
- summary.json: top-level keys bonferroni, cells, decision_rule, schema; `cells` has 7 entries (speck-r3, speck-r5, speck-r6, speck-r7, aes-r2, aes-r3, aes-r4); summary stdout "summary.json and fig_accuracy.svg written (7 cells)".
- Write scope: `git status --porcelain` shows only the receipt, untracked, under the write scope. Nothing outside it.
- Relay verbatim: NOT VERIFIED BY ME. No executor report file was supplied, so `python3 tools/task_card.py check-relay <card> --report <file>` could not run. The dispatch states the executor reported exit 0 and the receipt sha256 b8902ba6..., and both match the disk. Save the executor text to a file and run check-relay to close this.

## Card -1 (original run)

- outcome: interrupted (killed by the harness 30-minute background limit on the executor's shell command; not a card timeout, not a tool failure). No receipt exists (`cards/*-1.receipt.json` absent). `stopped_at_step`: step 4 (`run-all-cells`) by on-disk evidence; the tool printed no exit status, so this is inferred from the surviving files.
- Surviving step outputs are steps 1-3 only: `run-20261102.step1.stdout` = e18b11d31447ed3857cb8044042642d8e6bb0949 (equals the card -1 pin); `step2.stderr` "Finished `release` profile ... in 0.05s", `step2.stdout` empty; `step3.stdout` = binary sha256 41b8d479... (same as card -2). No step 4/5/6 files and no `results/run.log`, consistent with the tool writing stdout only after the command ends.
- Consistency with partial rows: results/rows.jsonl lines 1-33 hold all 5 arms of speck-r3, speck-r5, speck-r6, speck-r7, aes-r2, aes-r3 (30 rows) and aes-r4's mlp/primary, mlp/aa_seed2, logreg/primary (lines 31-33), in cell order. Missing: aes-r4 mlp/label_shuffle_control and difference_table_nb/primary. That matches a step-4 run stopped between rows. The binary used is the same one card -2 used.
- The card -1 stdout stream was never written, so I cannot compare the 33 rows against it; the rows are only on rows.jsonl.
- Relay verbatim: not applicable to a result; the executor text ("refused ... background time limit reached; killed before the tool printed an exit status") was not supplied as a file, so not checked.

## Classification

| card | execution | receipt | relay verbatim |
|:--|:--|:--|:--|
| -1 | interrupted (harness limit, step 4) | none | not checkable (no report file) |
| -2 | completed | present, sha256 b8902ba6... | not checked (no report file); receipt and digests match the dispatch |

Recommendation: proceed_to_analysis on the 35-key set (rows.jsonl last row per key), after check-relay on the saved executor text for card -2.

## Flags for the reasoning tier

1. rows.jsonl has 38 lines, not 35. Acceptance holds on distinct keys; a plain line count of 35 fails. Summary uses last row per key (per the dispatch).
2. `results/` is excluded from git by `/home/user/crypto/.git/info/exclude` (local, not the repo), which is why `git status` and the receipt `dirty_after` do not show it. To commit rows.jsonl, summary.json and fig_accuracy.svg a forced add is needed.
3. HEAD is now c5ccea81f (card -2 YAML committed after the run); the run itself was at f0993f828, per the receipt.
4. The build in both cards reused a pre-existing binary (0.05 s, 0.03 s). Source hashes are bound and unchanged, but neither card's build step compiled anything.
