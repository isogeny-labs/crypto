# Reproduce the finite proof and inspect the proposal

This assessment has no native solver execution. The full construction is
written in REVIEW.md and is not implemented by SpectrumAssembly.lean.

From the repository root, install the exact Lean toolchain and materialize
the pinned Mathlib dependency with the normal Lake/cache procedure. Then:

```sh
cd research/curve25519_torus_native_20261115/coset-spectrum-v1/lean
lake build SpectrumAssembly
lake env bash ../../../../tools/lean/proof-gate.sh \
  --out /tmp/curve25519-spectrum-gate \
  research/curve25519_torus_native_20261115/coset-spectrum-v1/lean/SpectrumAssembly.lean
```

The second command runs the maintained gate's supported focused file mode;
its working directory remains inside the same Git repository. The default
whole-project mode additionally fetches the whole Mathlib cache. Both checks
use the source/toolchain/manifest pins; no custom trust manifest is supplied.
Expected:3 named theorems,0 trusted-native exceptions. Inspect each printed
axiom list, rather than treating a build exit alone as complete evidence.

Final local qualification used the already-pinned Mathlib source/cache hosted
in crypto-lean5f2ca251. That local symlink is ignored, not a reproducibility
dependency. No source changes were made after final checking.

Render the editable report and diagram, without computing crypto results:

```sh
cd research/curve25519_torus_native_20261115/coset-spectrum-v1
dot -Tpdf diagram.dot -o diagram.pdf
dot -Tsvg diagram.dot -o diagram.svg
pdflatex -interaction=nonstopmode -halt-on-error report.tex
```

The retained PDF is the inspected2-page output. Rerendered PDF metadata may
differ. Retain original report bytes once pinned; new revisions are separate.
All original native archives/source/criteria and consumer98b6 source pin stay
unchanged. No archive upload is implied by this proof/report procedure.
