# Recovery-evidence handoff — 2026-10-28

## What was done

Conductor T-723 reserves only this directory. The previous E12c evidence PR
[#2764](https://github.com/isogeny-labs/crypto/pull/2764) merged at 10:22:47 UTC,
before this addition was ready. This follow-up uses the isolated worktree
`/var/tmp/crypto-e12a-recovered-gates-20261028`, branch
`codex/e12a-recovered-gates-20261028`, based on verified main commit
`524490275ad6c7f0120e908b2cf26a6ba7904e2c`. The earlier worktree, reports, raw
results and PR history are preserved.

Copied seven original JSONLs, their source-host hash list, bounded recovery
commands, SSM requests and receipts, and canonical data/provenance S3 receipts.
`manifest.json` binds the 24 copied artifacts. `evidence.sha256` binds those
artifacts plus the manifest, checks and Markdown documents, excluding itself.
The source files and archive hashes were checked again locally; no remote
command or upload was repeated. `checks.json` records the exact preservation
scope. Every JSONL has four objects and distinct `(cell,draw)` pairs within its
file: 24 gate-result objects plus four input objects, 7,647 bytes total.

## Decisions made

The daily session log is reserved by T-722, so this scoped handoff records the
work without editing that shared file. The original PR could not receive new
evidence after merging; the parent authorized a regular follow-up PR based on
main, with auto-merge and no manual merge. The scope is only evidence retention.
No source, dependency, build configuration, solver, protocol, previous result
or degree metric is edited. Cargo and experimental validation were not rerun
for this evidence-only addition, as explicitly directed.

The original E12a files were outside the periodic uploader's `/work/out`
coverage. This bundle records that omission and its recovery; it does not
silently strengthen the earlier persistence claim. S3 integrity references and
frozen source paths are listed in the README. The E12c closure basis was
in-memory state, not a missing checkpoint file recovered by this work.

## Open threads

E12a target completion and degree, broader intermediate-state retention, and
formal independent review remain separate unfinished requirements. No replay
certificate or independent algorithm validation is supplied. No new compute,
retry, IAM change, experiment budget allocation, or verified-billing statement
is authorized here; the total USD 200 campaign cap is unchanged. Hosted checks
for the follow-up commit must be reported at their actual status.

## Errors and recovery

The old PR merged during reconciliation; a new branch was authorized instead
of altering its historical description. Initial no-checkout materialization
left three tracked documentation files absent; restoring their exact tracked
bytes cleared those apparent deletions before staging. No source file was
changed. An initial Markdown patch had an invalid hunk marker and applied no
edits; the corrected addition was then applied. The first detached heartbeat
did not persist; a foreground tool session now renews the same T-723 lease.
