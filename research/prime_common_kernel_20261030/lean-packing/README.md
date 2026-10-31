# Exact error-image packing

Four named general theorems prove component recovery, injectivity of the
assembly map and the full finite-dimensional packing inequality. No fixed
finite instance or assumed final dimension inequality substitutes for the
general result. `PROOF_MAP.md` states the exact hypotheses and limits.

Pins: Lean4.35.0 official releaseeb6cafb7290e8bd5f2937522d8d804f58f02bafc;
CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e;
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Transitive revisions are
in `lake-manifest.json`.

```sh
lake build
lake build +CodedPacking
lake env lean --run .lake/packages/CryptoLean/tools/AuditAxioms.lean CodedPacking
lake env leanchecker --fresh --verbose CodedPacking
```

Initial failures exposed an unavailable finite-dimensional abbreviation and
missing import supplying vector-space freeness. Earlier attempts and actual
logs are retained. Final source builds cleanly, and its printed axioms use
only `propext`, `Classical.choice`, `Quot.sound`. Audit/fresh/shared-gate
outcomes are recorded separately. No custom axiom, admitted proof or native
evaluator is trusted.

This is a new versioned package; the earlier projection package is frozen
unchanged. Shared caches under `.lake` and this audit-only Git index are
excluded from portable source. A consuming repository should run its proof
gate on the copied target itself.
