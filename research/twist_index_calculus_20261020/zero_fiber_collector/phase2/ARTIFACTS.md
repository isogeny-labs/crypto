# Phase 2 evidence and pending archive

The [prerun freeze](FREEZE.json) and `workload-v1.{json,tsv,sha256}` are
committed in `af66996a19` before the full panel starts. It names the verified
89-test native runner, exact standalone Rust reducer build, all public and
reference seals, and passing 64/65 launcher controls.

Compact gate/build/preparation/control receipts are committed under
`../results/phase2-*`. Native reductions of all four retained p37 positive
controls are in `../results/phase2-workload-v1/inspect-*.json`; the verifier's
immutable p37 reference fixtures are already tracked in `../fixtures/`.

The complete raw Phase 2 tree is still local under `../runs/`. Preserve
the compile-check failures, original overlapping snapshot and exact
reconstruction, bootstrap/preregistration, both native gates, native
metadata build, both launcher-control versions, all layout attempts and
the managed `phase2-panel-v1` outputs. The [running owner](RUNNING.md) and
per-phase native receipts must be reconciled before continuation.

A new versioned archive, extraction/byte-comparison/repack verification,
native analysis replay and canonical publication remain pending until the
panel's terminal artifacts are available. This document does not claim
that the new raw archive has been published. The existing Phase 1
`../runs.tar.xz` and its verified canonical object remain immutable; their
publication is documented in [the parent artifact record](../ARTIFACTS.md).

The final two-page [build/control PDF](BUILD_AND_CONTROLS.pdf), its Typst
source and the SVG/150 dpi PNG diagram are retained here and copied byte
for byte to the unlisted `benchmarks/20261025-twist-zero-pipeline-controls/`.
Both actual rendered pages are inspected. Layout v1's overflow, corrected
v2, canceled queued v3 and final v4 output/source snapshots remain separate.
None alters the native results or admits a timing/GAE claim.
