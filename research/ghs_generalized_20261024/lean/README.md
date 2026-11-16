# Rebuild the subgroup-transfer lemmas

The existing `lean-toolchain` pins Lean 4.34.1. This Lake library registers
the unchanged `SubgroupTransfer.lean` module and uses the pinned Lean standard
library, without a Mathlib dependency. The sibling cover-decomposition
project declares this library as a local path dependency.

From the repository root, build and audit the complete tracked module:

```sh
tools/lean/proof-gate.sh research/ghs_generalized_20261024/lean
```

The native gate verifies all 13 theorems with no trusted exceptions. The
geometric maps, composition and exact-order hypotheses retain their original
conditional scope; adding project metadata does not establish those inputs.
