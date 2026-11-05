# Rank 4: target-state reuse control

Date: 2026-11-01

Scope: current `structured::Prepared` plus exact `contraction::Counter`

## Result

The current implementation retains no target-dependent online state.  A
fresh counter and a counter reused across target history have identical exact
work signatures for the same target.

The deterministic sequence was fresh `A`, fresh `B`, then one reused counter
on `A -> B -> A`, using the F5 rational-two-isogeny fixture, arity 2 and the
shared local membership message.

| Quantity | target A | target B |
| --- | ---: | ---: |
| Exact ordered-tuple count | 2 | 1 |
| Charged query steps | 826,199 | 815,527 |
| Contractions | 1 | 1 |
| Ring-edge assignments | 0 | 0 |
| Functional assignments | 127 | 127 |
| Pruned blocks | 47 | 47 |
| Fresh equals first reused query | yes | yes |
| Fresh equals query after other target | yes | yes |

The immutable graph has 31,828 edges and 8,546 vertices.  Its construction
record used 670 diagnostic charged steps; these counters are software
diagnostics, not a calibrated complete-cost unit.

## Interpretation

`Prepared` reuses a target-independent graph and local membership diagram, but
each `Counter::count` call starts a complete functional enumeration.  The
counter stores only configuration and cumulative statistics; it has no learned
clauses, target-parametric tensor message, memo table, or retained elimination
state.  Consequently the registered `1/8/64/512` reset-versus-reuse factorial
cannot test target-state reuse on this implementation: the online work is
exactly additive and history-independent by construction and by the dynamic
control above.

The current reuse candidate is therefore **exhausted within implementation
scope**.  This does not disprove target-parametric reuse in another solver.
The broader lead remains open until a provenance-audited retained state exists;
the missing concrete mechanism is a compact target-independent message for the
complete summation predicate, not merely local membership or graph rebuilding.

## Reproduction

```text
cargo +1.78.0 test --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --locked --release target_history_does_not_change_online_count_work \
  -- --nocapture
```

The control is in `crates/hashed-ecdl-tensor/tests/structured.rs`.  It compares
exact tuples of count, charged steps and per-query counter deltas, rather than
only comparing final answers.
