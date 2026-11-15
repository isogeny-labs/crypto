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
