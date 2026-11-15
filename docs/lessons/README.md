# Lessons ledger

How the agents and skills in this repository learn across sessions
(AGENTS.md "Continuous learning loop"). One JSON file per entry in
`entries/`, schema `crypto.lesson/v1`, written only by `tools/lessons.py`.

| kind | what it records | required |
|:--|:--|:--|
| `lesson` | when *trigger*, do *action* (scope: skill or agent names, or `*`) | trigger, action, evidence |
| `test` | a lesson applied and confirmed or refuted | lesson id, outcome, evidence |
| `outcome` | a finished task of one skill or agent | scope, task, result, evidence |
| `retire` | a lesson withdrawn | lesson id, reason, evidence |

Status (candidate, adopted, contested, retired) is derived from the tests and
is never written down. Two confirmations count only if their evidence differs.

```sh
python3 tools/lessons.py brief --scope ecbench-measure     # before acting
python3 tools/lessons.py outcome --scope ecbench-measure --task "..." --result success --evidence <receipt>
python3 tools/lessons.py test --lesson L-... --outcome confirm --evidence <receipt>
python3 tools/lessons.py add --scope ecbench-measure --trigger "..." --action "..." --evidence <path>
python3 tools/lessons.py stats                              # outcome rates, 95% intervals
python3 tools/lessons.py verify --base origin/main          # what CI runs
```

An entry's file name is the hash of its content: an edited entry fails
`verify`, and two sessions writing at once never conflict. Never edit or
delete an entry; record a `test` or `retire` instead. Entries are
`[pcc:unverified]` records of practice, not research results.
