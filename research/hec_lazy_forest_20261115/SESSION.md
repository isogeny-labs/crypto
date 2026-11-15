# Lazy label session, 2026-11-15

## First exactness gate
### What was done
Protocol commit1ea6e79f7e preceded lazy.rs. Native rustc --test and its four tests
pass:216streams,110592inserts, every emitted Option<Row> and union/pin counters
match dense reference; planted rows evaluate to zero. GATE.txt retains counters
and explicit controls. Includes two unchanged reference tests. No timing study.
### Decisions made
The first arena retains all emitted-cycle expressions. For example h127,n63,
ell1009,seed2 planted retains73936bytes versus68592dense bytes. This is evidence
against an automatic memory advantage. Correct ephemeral expression lifetime
before any complete-cost integration. Prefix exactness remains mandatory.
### Open threads
Amend gate with the lifetime rule, repeat the same deterministic streams;
document scope and commit/publish. PR4274 head de220aa93c all required checks
now pass, including records/custody and validation; signing is skipped.
### Errors
No exactness failure. The memory inefficiency is retained in first-gate output.

## Pre-code lifetime amendment
### What was done
Reviewed allocation order: endpoint find compression can store persistent DAG
nodes, whereas the new on leaf/hot expression is temporary when a row emits.
### Decisions made
Perform both finds first and retain their beta IDs. Record arena length AFTER
find compression; create on/hot after that watermark. If no roots remain,
expand and truncate to watermark. No stored beta may refer to a truncated ID:
finds precede watermark, emission stores no new parent/beta. On merge/pin keep
the new labels. Preserve exact scalar/root ordering and all test inputs.
Compare deterministic counters and retained bytes on identical streams, no
wall-time claim. Copy original output to GATE-RETAIN-ALL.txt before rerun.
### Open threads
Implement this change and rerun exactness in debug and optimized native builds.
### Errors
None; this amendment follows the observed retained-memory issue explicitly.

## Corrected gate and joint cost derivation
### What was done
Native gate.sh rebuilds/discovers the rlib and runs default/optimized tests;
both pass4tests, with216streams/110592prefixes in the differential gate.
Twelve release library tests pass. Native counts.jq emits COUNTS.json and
source.sha256 pins source/library. RESULTS.md retains all memory regressions.
JOINT-COST.md derives sparse a,b challenge labels without per-batch auxiliary
unknowns, and the known sieve's conditional supply/cost balance; read primary
Section2.2/2.3with explicit key sign, distinct-root and touched-bucket rules.
### Decisions made
Scratch truncation is safe only after endpoint compression. Retained storage
improves4.30..11.68atH127in synthetic streams, loses at smallH; no speed claim.
Real-stream expansion/rank/cost gates are required before a timing experiment.
Class engineering prerequisite; no record/default/leaderboard promotion.
### Open threads
Publish/checkpoint these source-linked gates, then implement the established
point sieve with sparse public labels under a separate exactness protocol.
Keep the mathematical10xobjective intact and test joint downstream costs.
### Errors
No gate failed. The first retained-arena inefficiency and scoped correction
remain reproducible; no historical run or closed record was edited.
