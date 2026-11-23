# Rank 1: theorem-kernel differential and mutation controls

Date: 2026-11-01

Scope: bounded exact controls for the implemented Appendix D.4 kernel

## Outcome

All 23 focused release tests passed.  The controls exercise the same packet,
mode, child, taper and Hermite-reconstruction code used by the recursive
kernel, but force tiny parameters below the source's asymptotic admission
range so that independent direct contraction remains feasible.

The load-bearing checks include:

- forced one-level recursion versus independent complete ring-edge
  contraction across predicate tables and both slot orientations;
- coupled parallel ports and degree-two mode products, where incorrectly
  treating repeated ports independently changes the count;
- both `v2(t)=0` and `v2(t)=1` taper levels on crossing and local ports;
- two nested recursive layers while retaining the packet's creation-time
  precision;
- tapered Hermite reconstruction, with a missing-jet mutation rejected;
- exhaustive weighted-mode tuple enumeration against an independent set
  construction;
- conditional packing with repeated ports;
- explicit mutations that lower a required incoming exponent from 2 to 1 or
  drop an outgoing mode; both change the exact packet entry;
- symbolic source-guard controls, including the `2^479` fail / `2^480` pass
  power-of-two boundary.

Command:

```text
cargo +1.78.0 test --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --locked --release --lib contraction::
```

Result: 23 passed, 0 failed, 48 filtered out, 17.70 seconds.

## Decision

No bounded implementation mismatch was found, and the mutation controls show
that the test fixtures are sensitive to omitted coupled, quadratic and
outgoing-mode terms.  The finite code-correspondence gate passes at this
scope.

This does not prove Appendix D, establish its bit complexity, or execute the
admitted recursion on a materialized production graph.  The conditional
theorem remains open; the production boundary-rank natural/null experiment
also remains separate.
