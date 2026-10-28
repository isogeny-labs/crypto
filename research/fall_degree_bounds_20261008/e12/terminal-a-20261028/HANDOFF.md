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
