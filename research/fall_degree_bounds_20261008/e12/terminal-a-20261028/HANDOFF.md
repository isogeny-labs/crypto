# E12a terminal evidence handoff — 2026-10-28

## What was done

Conductor T-733 reserves only
`research/fall_degree_bounds_20261008/e12/terminal-a-20261028/`.
The isolated worktree is `/var/tmp/crypto-e12a-terminal-20261028`, branch
`codex/e12a-terminal-evidence-20261028`, based on freshly fetched main
`98a2f7fa408fc6412f37d7d5225d7ddbdfdacc9f`. Main already contains the separate
E12c interruption and E12a gate-recovery packets; neither is changed here.

Copied 38 original artifacts: the 25 output files, original and canonical
archives, terminal/snapshot/instance receipts, logs, resource-usage record,
original hash/member lists, and canonical durability/PUT/GET receipts.
`manifest.json` binds those copies. `checks.json` records the observational
validation boundary; `evidence.sha256` also binds the new notes and metadata.
The original seven-file SHA-256 list verifies, all 38 manifest entries verify,
copied output files match the retained extracted files, canonical upload and
readback bytes match, and the protocol blob matches the saved protocol hash.
JSON parsing confirms zero target objects and 16 saved gate objects. These are
storage and record checks, not algorithm validation.

## Decisions made

The record states the terminal interruption and zero completed target records,
correcting the earlier still-running status by an append-only addition. It does
not interpret the absence of a record as a degree bound. The snapshot retains
available files, not the in-flight closure basis or a resumable process image.
The original launcher receipts and historical gate files remain immutable.

No solver, native test suite, gate replay, retry or cloud operation was executed
for this packet. No source, dependency, build configuration, preregistration,
previous result or degree plot was changed. The total USD 200 campaign cap and
the expired attempt's original deadline remain unchanged. The scoped handoff
avoids another writer's shared session log; its availability check was clear,
but expanding the write scope was unnecessary for this evidence-only addition.

The applicable sources are listed in the README. The PR/report-evidence
workflows preserve the distinction between byte verification and scientific
validation. This is a terminal preservation packet, not a fresh experimental
run, a formal independent review or a hypothesis decision.

## Open threads

The concrete packet is committed locally for parent review and delivery. It has
not been pushed and no PR has been opened. Current `AGENTS.md:77-80` requires
`cargo test --release --lib` before pushing. Its documentation-only provision at
lines 704-706 concerns changes that legitimately trigger no CI; the current Rust
workflows still invoke their scope jobs on main-base PRs, even when compilation
is skipped for these paths. No explicit waiver of the local pre-push rule was
found. Per the delegated delivery instruction, the parent must resolve the
applicable check requirement before publication; this packet does not borrow a
different source branch's test results as exact-head validation.

E12a still has zero of two requested target records. Its degree reading,
independent computational validation, full intermediate-state persistence and
the general first-/last-fall questions remain unresolved. A new computational
attempt is outside this packet's authorization. The canonical data bundle is
already durable; the parent retains responsibility for any new cloud action.

## Errors and recovery

An initial hash-list check was launched from the repository root, so no files
were verified; rerunning it from this evidence directory verified all seven.
An initial protocol-hash command was launched outside a Git worktree and failed;
the corrected command in this isolated worktree reproduced the exact saved
protocol hash. Neither failed command changed any source or evidence. Local
packet checks and source preservation are reported separately from pending
repository delivery checks.

`git diff --cached --check` reports the original `out/gate0.log` blank line at
EOF. Its bytes are preserved to retain its source hash; the derived Markdown
and JSON files pass the whitespace check. A proposed Node-based link checker
could not run because `node` is unavailable; native `rg`, `sed`, `test`, `stat`
and `jq` checks verified every local Markdown target and all 38 manifest lengths
instead. No failed check is represented as a successful one.

## Migration reconciliation — 2026-10-28T13:03Z

### What was done

The approved migration receipt at
`/srv/experiments/agent-migration/handoff-receipt.json` was read through the
documented scoped `sudo -n cat` path after an ordinary read was denied. The
restored worktree is now `/home/ec2-user/crypto-e12a-terminal-20261028`, on the
same branch at pre-edit commit `7c05a9a16cdcf4e6fd35504ff5d89a54d6307642`.
The old `/var/tmp/` worktree path above is historical. The working tree was
clean before resuming this delivery.

The migrated local Conductor service retained existing T-735, which was claimed
for `research/fall_degree_bounds_20261008/RESULTS.md`, and T-733, which was
reclaimed for this packet directory. No duplicate coordination task was filed.
The mutable study summary now marks its earlier E12a/E12c ongoing statement as
historical and links the terminal E12a observation. The packet's original
terminal receipts, data files, manifest and archives are unchanged. Only this
derived handoff and its checksum were updated inside the packet.

Post-migration checks verified all 42 entries in `evidence.sha256`, all seven
original source hashes, and all 38 manifest hashes and byte lengths. The target
JSONL still parses as zero objects; the terminal receipt still reports exit 143.
Both archive digests and their 9/28 member counts match the saved record. The
changed derived Markdown and checksum pass `git diff --check`.

### Decisions made

The existing job's terminal receipt and zero-byte target file determine the
status: interruption with zero completed target records. The saved S3 durability
receipt establishes the earlier fresh GET and byte comparison; this migration
follow-up makes no new cloud request. The frozen Amendments 13/14, final receipt,
and persistence contract remain the sources for the terminal-state pointer.
No new scientific result, trial, retry, deadline or compute allocation is added.

### Open threads

Publication remains delegated to the parent: run the required exact-source
release library check, inspect the resulting commit and existing PR state,
then push and open or update the regular PR with applicable checks and auto-merge.
No build, push or PR creation was performed by this migration follow-up. The
USD 200 total campaign ceiling and all original completion criteria are retained.

### Errors

The first ordinary read of the root-owned migration receipt returned permission
denied; the documented narrowly scoped noninteractive sudo read succeeded. The
initial Conductor check reported T-735's existing reservation; inspecting and
claiming that same ready task resolved ownership without bypassing the scope.
No protected source, old evidence, cloud resource or unrelated managed generator
was modified.
