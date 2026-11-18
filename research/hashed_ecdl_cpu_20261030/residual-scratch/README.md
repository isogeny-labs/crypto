# Residual numerator scratch elimination

## What was done
T-1589 starts from published suffix-only source 6ca7537e243b3952a3cd3a6c931950708c3dfce4 after reconciling PR #2980 and its clean worktree. Scratch::subtract no longer stages a Vec<u64> of numerators. Its reverse pass already loads each original point's x; it now loads y and forms the ordinary or doubling numerator immediately before applying the recovered inverse.

One u64-vector buffer and its per-row push/read stream are removed. The numerator is still evaluated once per active row, at a different time. This does not claim fewer total machine loads or faster execution: reading y and branching in the reverse pass may offset the removed buffer traffic. Denominators, prefix products, exceptional outputs, inversion counts, residual order, table construction and exact witness selection are unchanged. Scratch storage was excluded from the retained-table statistic before this change and remains excluded.

## Decisions made
Read batch-endpoints/README.md, residual-filter-revert/README.md, the round24 execution record, and the existing Scratch equations and independent affine-reference tests. This is a scheduling/storage transformation of the existing Montgomery batch inversion implementation, not a new curve formula. Round24 rejected adding a residual membership branch; this candidate adds no membership test or rejection flag. Its reverse pass distinguishes ordinary subtraction from doubling to reconstruct exactly the original numerator.

For distinct x, numerator = (-yy mod p)-y mod p. For equal x with an active denominator, the existing doubling numerator is (3*x*x+a) mod p. Zero-denominator cases have already produced their exceptional output and are skipped before numerator evaluation. These are the same expressions as the parent.

## Validation
47 standalone release library tests pass. Coverage includes all point pairs on the 199-point curve, independent affine checks on the 65521 and 68719476731 fields, empty/exceptional batches, doubling, scratch reuse, lexicographic oracle witnesses, archived certificate byte equality, and both full replays. Strict all-target release Clippy, example compilation/tests (zero registered example tests), formatting and authored-source whitespace checks pass. Validation used a separate source materialization and the existing standalone target directory; no root/CUDA build or cloud build was run. Raw logs and source digest are retained alongside this note.

## Budget and execution status
The current owner policy supersedes the historical unlimited-budget text in round24. IsoLab has one shared USD200 daily allowance and a USD1000 monthly ceiling. Before another cloud build or benchmark, publish the exact source, frozen protocol, manifest, seeds and complete launch argv with a finite all-in cost ceiling, CPU/RAM, concurrency, deadline and total retries. Verify both remaining unreserved funds and live resource enforcement. Do not infer live caps from the MCP overview.

Read-only AWS Budgets inspection on 2026-11-05 returned daily ActualSpend USD104.44 / limit USD200 and monthly ActualSpend USD538.72 / limit USD1000. These are delayed reported totals, not verified unreserved funds. Outstanding commitments and live NodePool/ResourceQuota enforcement remain unreconciled. No cloud work was submitted and no limits were changed. The overview showed no active jobs, only completed/failed jobs and the previously suspended unrelated qualification job.

## Open threads
Freeze the next complete-cost comparison against 6ca7537e243b with all original fixtures, controls, full certificate comparisons and both replay costs. This source is correctness-validated but unmeasured. Preserve immutable round24 and its rejected candidate. The full tenfold improvement objective stays active; this storage reduction is a candidate toward it, not acceptance evidence.

## Errors
No software validation failed. Filesystem operations were polled using their existing handles. No interrupted work was restarted.
