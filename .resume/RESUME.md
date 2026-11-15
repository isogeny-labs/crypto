# Resume: crypto @ claude/bloom-hyperloglog-index-calculus-kh3pgr (sketches + mixed relation solver)

## Goal
User (2026-11-15): integrate Bloom / HyperLogLog / count-min sketch or algorithmic tricks into the IC
relation-solving step; then "yes for both" (structural-rank matching gate; Bloom large-prime sketch null)
"and try to find novel way to integrate these into a combined/mixed solver".

## Lead / hypothesis
research/ic_sketch_mixed_solver_20261115/PROTOCOL.md (frozen in 735883bee6): P1-P6. Gate removes doomed
deferred solves; mixed-peel (gate + matched square system + two-sided IBLT-style peel + Markowitz core,
one certified solve) cuts row_ops; Bloom screen is pure overhead; nothing moves S at w = 0 except rows read.

## Spec
Part A: examples/ic_mixed_solver_stream.rs -> research/ic_sketch_mixed_solver_20261115/partA/stream.jsonl.
Part B: specs/partB-p24.json, specs/partB-p28.json (ecbench run, local, L0). Part C: ic.large_prime spec
from research/ecbench_accounting_v2_20261023/specs/frozen-large-prime.json.

## State
Code + protocol committed (735883bee6). Release build of ecbench in progress; no data yet.

## Runs in flight
None (all local to this container).

## Stop and success conditions
Any arm returning a wrong logarithm voids it. Results reported against predictions, success or not.

## Next steps
unit tests (cargo test --release --lib linalg_mixed) -> Part A -> Part B ecbench run/verify/compare ->
Part C -> REPORT + figures + PDF -> session log -> PR (ready, auto-merge) -> subscribe.
