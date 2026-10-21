# Completed card and accounting validation

The frozen remote card ended at 06:48:53 UTC. All eighteen natural
attempts were archived; no old evidence or worker binary was replaced.

| Check | Result | Receipt |
|:--|:--|:--|
| Release library, merged solver/catalog source | 4401 passed, 0 failed, 118 ignored | `merge-release-lib-20261020-2.log` |
| Twelve affected examples | 77 passed, 0 failed | `merge-release-examples-20261020-2.log` |
| Final full-card auditor | 5 passed, 0 failed; release build passed | `full-m4-audit-final-tests.log`, `full-m4-audit-final-build-test.exit` |
| New native reuse accounting | 1 passed, 0 failed; actual pinned-card derivation passed | `reports/reuse-20261020/native-test.log`, `native-run.log`, `native-run.exit` |
| Full-card archive and audit | 329 files pass; fresh extraction gives byte-identical audit | `full-m4-v1/final/`, `receipt-ci-full-replay.log`, `.exit` |
| Earlier cache-card archive and audit | 294 files pass; fresh extraction gives byte-identical audit | `receipt-ci-cache-replay.log`, `.exit` |
| Reuse accounting repeat | byte-identical to retained `accounting.json` | `receipt-ci-full-replay.log`, `.exit` |
| New workflow | actionlint passes | `workflow-validation-20261020-final.log`, `.exit` |
| New Rust files | rustfmt check passes, edition 2021 | checked before commit |
| Visual reports | full-card and reuse PDFs, two pages each, all pages inspected; graph copies byte-identical | each report's `render` logs and `source-output.sha256` |

The full release library and twelve-example suite ran against the merged
solver implementation. Subsequent changes affect the separate receipt
auditor, accounting example, workflow, collection/reporting and evidence;
their targeted tests and actual archive replays are listed above.

The new workflow repeats native tests and byte-exact archive/accounting
replay in GitHub. Local success is not a statement about current-head
GitHub CI. Catalog validation receipts and historical failures remain in
their existing files. [Pending index edits](PENDING_INDEX_ROWS_20261020.md)
record the separate coordination gate.
