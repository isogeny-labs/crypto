# Generator-multiplier factor bases: complete-cold gate

This study tests whether the relation-count advantage of generator-specific
multiplier bases survives the full native `ecbench` method call. The mechanism
under test is narrow: many signed table vertices have known weights on a small
number of seed-log columns, so a two-sum table can be much wider than the
relation matrix.

The prime and binary panels are independent prospective experiments. Each has
12 held-out targets, six measured rounds, one warm-up, alternating measured
order, an A/A timing control, same-seed ablation, coordinate and plain-base
controls, and rho. Their primary end-to-end metric is the L2-admitted complete
cold method-call wall ratio. Counted `S` and total GAE are co-primary mechanism
checks; fork-to-reap wall time is the broader practicality check.

The formal module at [`lean/FormalBoundary.lean`](lean/FormalBoundary.lean)
separates the algebra from the benchmark. It proves without untrusted axioms
that:

- invertible additive coordinate changes preserve pair relations;
- split extension embeddings preserve transported pair relations;
- an odd-order quadratic trace-zero sum cannot be a nonzero Frobenius-fixed
  endpoint; and
- multiplier vertices evaluate as known weights on seed columns.

Those theorems do **not** establish a speedup and do not rule out
higher-degree extension-field or polynomial-system algorithms. Conversely, a
successful panel establishes only a constant-factor benefit for this native
two-sum signed-graph pipeline, not a new ECDLP exponent.

## Files

- `prime/spec.json`, `prime/PROTOCOL.md`, `prime/protocol.json`: prime CM panel.
- `binary/spec.json`, `binary/PROTOCOL.md`, `binary/protocol.json`: held-out
  random-binary panel.
- `*/analyze.sh`: replay-first, pre-specified comparisons.
- `lean/FormalBoundary.lean`: coordinate, extension, trace-zero and weighted
  column statements.

No measured run belongs in this directory until both protocol documents are
frozen, committed before affected data, and the repository recovery checkpoint
gate passes.
