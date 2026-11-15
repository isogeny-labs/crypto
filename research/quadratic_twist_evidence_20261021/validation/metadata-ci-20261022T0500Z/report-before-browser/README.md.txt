---
title: "Quadratic-twist registry: metadata consistency follow-up"
date: "2026-10-22"
subject: "cover-transfer-assessments/v2; catalog_maps=4493; unique_cover_models=4491; geometry_verified_maps=4493; ec1_representations=1004; assessed_maps=4493; unassessed_maps=0; assessed_subgroups=1004; unassessed_subgroups=0"
---

<!-- cover-transfer-assessments/v2; catalog_maps=4493; unique_cover_models=4491; geometry_verified_maps=4493; ec1_representations=1004; assessed_maps=4493; unassessed_maps=0; assessed_subgroups=1004; unassessed_subgroups=0 -->

# Scope and evidence

At PR head `3e442fd3310d192495e5277a278aa2a0c0aab605`, the native finite
replay, validation gate, catalog assessment, and this study's Lean target passed
CI. The certificate job then rejected the registry's compact appended rows at
the metadata-only standards join. Its native standards replay and preceding
constructor tests had passed. The independent SEA proof target also failed;
its sources and proof-gate tooling are identical at the PR head, declared base,
and actual CI merge/base commits.

This follow-up corrects the Rust importer's appended-row layout, formats the
active registry, and refreshes dependent source digests and roster displays.
All registry values, aliases, model/cover/map identities, subgroup records and
geometry findings are unchanged. Metadata comparisons retain arbitrary-size
integer tokens exactly. The display now labels odd extension fields as
`GF(p^k)`. Measured leaderboard rows, protocols and other source records remain
unchanged; this is no performance or algorithm improvement.

The [validation record](../../validation/metadata-ci-20261022T0500Z/README.md)
contains the failing CI excerpt, local before/after join checks, 40 native
metadata/backend tests, native certificate byte replay and input-binding control,
and a fresh import from the original 2,572-model catalog. The fresh native import
matches the canonical 4,493-model registry byte for byte. The metadata-only
preservation receipt binds both registry layouts, checks every geometry and
identity value unchanged, and proves that no measured leaderboard row changed.
The remaining native cover, Cargo and independent-assessment checks are recorded
there with individual exit files.

The old catalog anchor `a11984a573441d5bf0d60ab1d26cfc34e71badf5`, its three
exact Git blobs, and the [earlier report](../catalog-20261022/README.md) remain
part of the provenance chain. The active assessment pins a descendant catalog
snapshot containing the layout correction and refreshed registry digests. Its
verifier requires both the predecessor's exact blobs and the current pinned
inputs; it retains every membership, formula, partition and semantic assertion.
Earlier dated reports, source archives, failed attempts and study receipts are
preserved.

| Profile | Models | Same-field geometry |
| --- | ---: | --- |
| Ordinary binary | 165 | genus 3, degree 3 |
| Direct odd prime | 2,011 | genus 2, degree 2 |
| Normalized odd prime | 30 | genus 2, degree 2 with target normalization |
| Direct odd extension | 2,287 | genus 2, degree 2 over the declared extension |
| Total | 4,493 | 4,491 distinct HC1 cover objects |

The graph still binds 538 declared fields, 4,491 Jacobian contexts, 4,493 maps
and 1,004 selected EC1 representations (140 binary and 864 odd-characteristic).
The 942-model native twist study is a separate finite workload. Its 434-field
parity, 84 IC matrix checks and 18 Lean theorems retain their original scope.

# Open gates and interpretation

SEA's build succeeded, but Lean 4.19 rejected the gate's `--trusted` argument
before its axiom audit could run. A zero audited-theorem count is not evidence
of zero theorems or an invalid mathematical proof. Its receipt and identical
source objects are preserved; this PR does not modify that inherited target or
the global gate. Cursor's approval agent also could not start because its
on-demand usage limit was reached. Pending/cancelled checks and absent review
remain unresolved; neither a neutral check nor local validation supplies approval.

The actual twist embedding's Frobenius-negation theorem (`hB`), full eigenspace,
summation-polynomial, pairing and trace-zero bridges, and actual shared Conductor
access remain open in the [problem record](../../../../problems/quadratic-twist-evidence-native-parity.md).
No reservation or replacement coordinator is claimed. No general ECDLP hardness,
GHS algorithm, IC speedup, or invalid-curve attack is claimed.

# Visuals and reproduction

![Map and subgroup support boundaries](typed_correspondence.svg)

![Typed contexts and executable-delivery boundaries](typed_transfer_context.svg)

The [PDF](report.pdf), [source](report.tex) and [build script](build.sh) include
this report and both diagrams. Diagram sources and SVG/PNG renders are byte-identical
to the preceding catalog report because every population, identity and support status
is unchanged. Canonical IC/rho cost graphs remain unchanged because no new cost
comparison was performed. The validation record lists exact commands and hashes;
no Python research arithmetic, experiment harness or certificate verifier ran.
