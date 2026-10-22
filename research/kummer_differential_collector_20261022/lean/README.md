# Kummer price aggregation proof

`CostAggregation.lean` proves that a nonempty, aligned list of candidate and
rho **natural-number** prices has a strict pooled candidate loss if every
paired target price is strictly higher. The companion weak-sum lemma is part
of the proof. Lean 4.34.1 checks both theorems. The repository proof gate
[audited three declarations](evidence/2026-10-22/audit.log), with zero
failures, zero trusted native declarations, and only `propext` and
`Quot.sound` in the reported theorem axioms.

This is a conditional theorem about summing price vectors. The archived Rust
analyzer checks the recorded per-target arithmetic and reports 45,360
**full-cold** target/price losses; the Lean file does not import the CSV, prove the
counter-to-price conversion, establish exact rational equivalence for the
analyzer's floating-point GAE values, or prove the Kummer group formulas.
Those are separate validation and formalization tasks. The theorem therefore
must not be cited as a Lean proof of the 8.959687–55.935424 ratio band or of
the collector itself.

Run from the repository root:

```sh
ELAN_TOOLCHAIN=leanprover/lean4:v4.34.1 tools/lean/proof-gate.sh research/kummer_differential_collector_20261022/lean/CostAggregation.lean
```

The [run report](../reports/run-20261022T1354Z/RUN.md) maps the theorem to
the empirical comparison and records the unproved boundary.
