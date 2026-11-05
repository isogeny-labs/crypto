# Projected translations with distinct syndromes

This project checks six named structural declarations. It does not prove
the entire coded-kernel study or a faster contraction/ECDLP algorithm.
See `PROOF_MAP.md` for exact scope.

Pins: Lean4.35.0 release eb6cafb7290e8bd5f2937522d8d804f58f02bafc;
CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e;
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2.
All transitive revisions are pinned in `lake-manifest.json`.

Rebuild with the pinned toolchain:

```sh
lake build
lake build +CodedProjection
lake env lean --run .lake/packages/CryptoLean/tools/AuditAxioms.lean CodedProjection
lake env leanchecker --fresh --verbose CodedProjection
```

Final build, shared axiom audit and fresh official replay pass. Only
`propext`, `Classical.choice`, `Quot.sound` are used; no admitted proof,
custom axiom or native evaluator is trusted. Three generated congruence
lemmas mean the automatic auditor reports9 theorems; there are6 named
research declarations. Initial failures/snapshots are retained, including
scalar-ring inference errors and a missing matrix-multiplication import.
These are development failures, not mathematical results. `.lake` caches
are not distributable source and are ignored.

The pinned shared repository proof gate also passes:9 audited declarations,
0 trusted native declarations. It was run against this isolated Git-tracked
project; a consuming repository should run its own actual-path gate after
copying the package.
