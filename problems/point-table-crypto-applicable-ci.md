# Native point-table PRs: inherited applicable benchmark gate

- **Status:** open
- **Kind:** unrelated main-branch CI dependency
- **Affected work:** Crypto PR4342 and its joint-target successor4351
- **Next action owner:** genus-ladder experiment/record lane
- **Last updated:**2026-11-15

## Problem

The applicable records job rejects an open genus-ladder benchmark record
imported unchanged from main. The point-table/joint code cannot merge under
the user's all-applicable-check requirement until that dependency is resolved.

## Evidence and uncertainty

Job132976237610, run43767641066, reports
`benchmarks/records/20261115T0609Z-genus-ladder-sge-scale.json: still open`.
The file is byte-identical on parent headdf51dae9 and its merged mainbb701788;
latest fetched mainb8cd452f also reports status open. Original job output and
source identities are retained in the refresh receipt. This is producer Git/CI
evidence, not a signed execution or performance verdict. [pcc:unverified]

The unrelated record's closure criteria and run output were not evaluated in
this tracking task. An open record is not evidence of failed mathematics or
permission to rewrite a benchmark, weaken its gate, or declare it complete.

## What would unblock it

The owning lane finishes and honestly closes the record under its actual
protocol, or lands a reviewed main repair for a demonstrated record/checker
defect. Then inspect this PR's actual refreshed head and every applicable
check/review. No benchmark work is launched from this tracking task.

## Next discriminating step

Inspect the next spaced current-head gate result after normal main refresh.
Retain the failure while it persists, report it on the PR, keep auto-merge off
and do not edit another study's record to make this PR green.

## Resolution and history

Open. Native source and old frozen receipts remain unchanged. PCC and lesson
prefix failures on the successor are separately reproducible stale-base
failures; a normal merge is being qualified without rewriting either history.
