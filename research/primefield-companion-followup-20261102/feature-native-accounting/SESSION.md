# Native stage accounting — T-1507

## What was done (commands, files, features)
Read all primitive arithmetic layers and official Rust GlobalAlloc contract. Generated seven routed instrumented modules with exact inverse-transform checks. Added thread-local fixed-array stage counters and forwarding allocator; fixed eight-target reference-equivalence and arithmetic contracts.

## Decisions made (and why)
Separate setup, preparation, query and verifier; distinguish components, field operations, scalar modular operations, decoder mask work and allocator requests. Unit-operand inversions are a subset of inversion calls, not a second cost. Inverse Euclid iterations recorded; retained memory, CPU timing and other integer work remain separate.

## Open threads (unfinished, pick up next time)
Run frozen checks, audit coverage and observed concentration; then choose concrete optimization and finish original-cohort runner.

## Errors (what went wrong, how it was handled)
Two generator failures before execution: duplicate fixture routing and one-line decoder-negation layout. Failure records preserved; fixed exact substitutions and regenerated. No native build/test yet, no cloud job, no previous source edits.
