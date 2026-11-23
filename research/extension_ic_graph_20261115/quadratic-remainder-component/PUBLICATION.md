# Native component follow-up: PR4452

The reusable library source and retained qualification are published at
https://github.com/isogeny-labs/crypto/pull/4452 . Parent PR4288 merged as
365d2b95 while this source qualification/publication was underway. The
standalone stage40 rank experiment remains a separate original record.

Source qualification is the19-test release log and warnings-denied library
Clippy log, with the exact original source03 hashes and environment retained.
The6 restored device checks pass; the full parent suite is censored at120s.
PCC-20261116-dfda61 is the unverified canonical integration record. This
document is a publication receipt, not another scientific run.
[pcc:unverified]

GitHub reports this regular PR as OPEN/CONFLICTING. A read-only merge-tree
check confirms conflicts in exactly `docs/session-logs/2026-11-16.md` and
`pcc/ledger.jsonl`. The repository rule says: “Conflicts in frozen evidence,
ledger rows, or claim files are never resolved by picking a side”. Both
development histories are retained, with reconstruction instructions in
INTEGRATION.md. No frozen journal is overwritten to obtain a clean merge.

Auto-merge was enabled on the PR. Required checks/review and frozen-journal
reconciliation remain pending. After publishing this receipt, request the
AGENTS.md-prescribed Copilot base-merge handoff, explicitly requiring that
frozen evidence stay intact and that unresolved journal conflicts be reported.

The first attempted push targeted the just-merged/deleted parent branch.
It was canceled before pack transmission while the partial clone was fetching
missing base objects; readback confirmed that branch remained absent. Fetching
enough main history and using Git's push negotiation then published the owned
follow-up branch normally. No force push or parent-branch recreation occurred.

The next scientific step is to qualify production use of this reusable API
against the separate prototype and then measure the complete pipeline. Do not
compose stage40's component counts with these correctness controls.
