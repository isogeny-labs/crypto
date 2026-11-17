# PR3744 append-only ledger conflicts with concurrent main history

- Status: open; requires a ledger-maintainer reconciliation procedure
- Observed:2026-11-09T06:47Z
- Branch implementation:8abd9cb5989a658151d4d3a9647d9dd4fb5434f5
- Fetched main:62d8e2640db860af7a3bb2f7352e7f45b9b51513
- Evidence: [read-only receipt](../research/trace_zero_point_chains_20261109/checks/base-merge-status.txt)

Read-only `git merge-tree --write-tree HEAD origin/main` reports conflicts in
docs/session-logs/2026-11-09.md and pcc/ledger.jsonl. The feature branch has463
internally linked entries, while newly fetched main has468. The main-prefix
verifier rejects the branch after that fetch. This is concurrent branch
history, not evidence that the session deleted main's entries. The earlier
prefix check passed against its then-current base; that historical receipt
does not establish compatibility with this new base.

The repository's merge instructions explicitly forbid resolving frozen
ledger/evidence conflicts by selecting a side. No ledger rows, claim records
or conflicted session-log bytes were rewritten. Copilot base-merge request
comment6734027719 includes that constraint; its result requires review before
any acceptance. Do not rebase, force-push, renumber or recompute frozen rows
to manufacture an apparent clean history.

The next action is a maintainer-approved append-only history reconciliation
that preserves both branches' evidence and hash/signature meaning. After it,
rerun ledger/base compatibility and all affected source/merge checks. Neither
passing local native tests nor the two observed successful review/migrate
checks clear the missing merge gates.

No code, benchmark records or validation settings are changed by this record.
The prepared264-row experiment remains separately pending its execution
exception or actual S3/cloud access.
