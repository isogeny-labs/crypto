# A checked failure of unrestricted physical-port functoriality

This small project proves an elementary finite witness. It establishes no
novel contraction algorithm and does not rule out a different common
kernel. See `PROOF_MAP.md` for the complete theorem mapping.

Pins:

- Lean v4.35.0, official release commit eb6cafb7290e8bd5f2937522d8d804f58f02bafc.
- Shared CryptoLean `148f5dc389b341401e06d76abe7783c4ccf42f2e`.
- Mathlib `cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`.
- All transitive revisions in `lake-manifest.json`.

Rebuild:

```sh
lake build
lake build +GaugeCounterexample
lake env lean --run .lake/packages/CryptoLean/tools/AuditAxioms.lean GaugeCounterexample
lake env leanchecker --fresh --verbose GaugeCounterexample
```

The local `.lake/packages` symlink reuses pinned dependency caches and is
ignored. The project's `.lake/build` is isolated, so its proof inventory
contains only this source's three declarations.

Earlier successful full-Mathlib-import source is retained as a `.lean.txt`
snapshot with build logs. The final source narrows imports to matrix
multiplication, notation and norm_num. An initial auditor command used
the wrong working directory; the actual error is retained. A redundant
audit started before import narrowing was terminated after the final
source audit passed. Its termination is explicitly recorded. These are
development/infrastructure records, not mathematical counterexamples.

Final results: build PASS, explicit-source shared gate PASS,
transitive axiom audit3/3 PASS with only `propext`, `Classical.choice`,
`Quot.sound`, and fresh import-inclusive `leanchecker` replay exit0.
The authoritative final shared-gate receipt is `proof-gate-final/`;
the final module audit is `axiom-audit-final.log`; the fresh replay
receipt is `fresh-leanchecker.log`. This validates only the three finite
statements mapped in `PROOF_MAP.md`.

The copied dependency manifest's root package name was corrected from
`levelwise_binomial` to `gauge_counterexample` without changing any
dependency pin; the original is retained in `copied-root-manifest.json.txt`.
