---
title: "Expanded curve-cover catalog assessment"
subtitle: "15,990 declared maps; replay and transfer boundaries"
date: "2026-10-23"
subject: "cover-transfer-assessments/v2; catalog_maps=15990; unique_cover_models=15988; geometry_verified_maps=15990; ec1_representations=700; assessed_maps=15990; unassessed_maps=0; assessed_subgroups=700; unassessed_subgroups=0"
---

<!-- cover-transfer-assessments/v2; catalog_maps=15990; unique_cover_models=15988; geometry_verified_maps=15990; ec1_representations=700; assessed_maps=15990; unassessed_maps=0; assessed_subgroups=700; unassessed_subgroups=0 -->

# Expanded catalog validation

This dated addendum records the catalog at commit
`cf3fad47b02159e018c2a6329fadd5213f9aa40e`. It supplements the
[2,572-map historical report](../../README.md); that report and its replay
artifacts remain frozen.

The current inventory contains **15,990 map rows**, **15,988 unique cover
objects**, **735 declared fields**, **15,988 Jacobian contexts**, and **700
linked EC1 representations**. The catalog checker reports 15,990 verified
maps, zero invalid inputs, and zero unsupported inputs. The assessment assigns
one base profile to every map UID and one subgroup profile to every exact
`[EC1, curve UID, map UID]` triple. In this report, *assessed* means that every
listed claim has an explicit supported, refuted, unknown, or not-applicable
result. It does not mean that every transfer capability is supported.

| Map profile | Rows | Recorded boundary |
|:--|--:|:--|
| Binary cubic, same declared field | 186 | Explicit cover geometry and bounded typed point-class operations; arbitrary Jacobian norm remains unsupported. |
| Odd prime, direct quadratic | 14,403 | Explicit cover geometry; balanced two-infinity Jacobian transfer is unavailable. |
| Odd prime, normalized quadratic | 30 | Explicit geometry and checked target normalization; no full transfer pipeline. |
| Odd extension, direct quadratic | 1,371 | Explicit cover geometry over the declared extension; no descent or full transfer pipeline. |
| **Total** | **15,990** | **15,990 assessed maps; zero unassessed maps.** |

| Subgroup profile | Linked representations | Recorded boundary |
|:--|--:|:--|
| Binary, degree 3 | 141 | Generator-level typed identities and degree-inverse recovery are recorded; all-element and arbitrary-Jacobian recovery remain conditional or unknown. |
| Odd characteristic, degree 2 | 559 | Identity and degree/order coprimality are recorded; induced maps and recovery remain unavailable or unknown. |
| **Total** | **700** | **700 assessed subgroup triples; zero unassessed subgroup triples.** |

The [profile graph](assessment.svg) shows the map-profile counts and the
geometry boundary. The [typed-context graph](typed-context.svg) shows the
field, Jacobian, map, and EC1 binding counts. Both graphs are rendered from
the committed Graphviz sources, and the PDF embeds the profile graph.

## Provenance and checks

The three catalog files (`registry.json`, `covers.json`, and
`cover-links.yaml`) are fixed together by commit `cf3fad47`. Relative to the
previous 2,771-row catalog, this inventory adds 13,219 prime-field models. The
[2026-10-23 registry audit](../../../audit-20261023/README.md) records their
source bindings and order certificates, including the limits of those checks.
This report does not extend the audit's claims.

At the catalog commit, the following native freshness check completed with
`verified=15990`, `invalid_input=0`, and `unsupported=0`:

```text
./target/release/curve_cover_check --check
```

The generated assessment is checked byte for byte with
`node scripts/build_cover_transfer_assessments.mjs --check`. The independent
`verify_assessment.mjs` reconstructs the exact model, map, cover, field, and
Jacobian joins, replays the cover equations over the declared fields, and
binds this report and its figures by SHA-256. The exact coverage marker in the
front matter is also stored as the PDF subject.

## Claim boundary

The classification and cover-equation replay do not establish a complete
Jacobian transfer pipeline, a discrete-log cost improvement, or a wall-speed
result. The 186 binary maps have bounded typed point-class operations. The
15,804 odd-characteristic maps have explicit cover geometry without a
compatible balanced two-infinity Jacobian transfer. The conditional Lean
group lemma does not import this catalog or certify its finite-field
calculations. The older packaged release replay covered a smaller bundled
registry; it is retained as historical evidence rather than a replay of all
15,990 current inputs.
