# Complete cold timing handoff

## What was done
Revalidated terminal cold/native correctness outputs and exact regenerated
objects; recorded coldbuild13 results before this experiment. Copied its worker
with timing boundaries and added nativeFast/Hybrid/Compact controls. Built Rust
write-once20cell collector and frozen protocol. No timing execution yet.

## Decisions made
Same public source pair, one binary, five alternating sequential blocks;
core and processwall separate. Compare to best native median, not a weaker arm.
All construction/validation included; metadata emission outside core explicitly.
L0/unisolated, producer unvalidated, no broad10x claim. No science in Python.

## Open threads
Source/fingerprints commit, open benchrec and checkpoint; run collector once,
archive/uploadraw, report exact outcome including gate failure/censoring. Publish
both cold correctness and cost reports/benchmark entry/PR4274/current-headCI.

## Errors
None in benchmark build. Prior cold compile adapters fixed before source gate.
TaskT-6257 scopes held; .benchrec bookkeeping separately checked/claimed.
