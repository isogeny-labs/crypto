---
title: "Expanded curve-cover catalog assessment"
subtitle: "2,771 declared maps; replay and transfer boundaries"
date: "2026-10-22"
subject: "cover-transfer-assessments/v2; catalog_maps=2771; unique_cover_models=2769; geometry_verified_maps=2771; ec1_representations=977; assessed_maps=2771; unassessed_maps=0; assessed_subgroups=977; unassessed_subgroups=0"
---

<!-- cover-transfer-assessments/v2; catalog_maps=2771; unique_cover_models=2769; geometry_verified_maps=2771; ec1_representations=977; assessed_maps=2771; unassessed_maps=0; assessed_subgroups=977; unassessed_subgroups=0 -->

# Expanded catalog validation

This dated addendum assesses the catalog at commit
`525a4404bd0cb3fb1e74ce6983fc0e992079692c`. It supplements the
[2,572-map historical report](../../README.md); the earlier report, graphs,
PDF, and release replay are retained as frozen evidence.

The current catalog has **2,771 map rows**, **2,769 unique cover objects**,
and **977 linked EC1 representations**. The independent verifier reconstructs
the exact curve, map, cover, declared-field, and Jacobian-context joins. It
replays the explicit cover equations over each declared field and records
2,771/2,771 geometry-verified map rows, 2,771 assessed maps, 977 assessed
subgroup triples, and zero unassessed members. The field/context inventory is
687 declared fields, 2,769 Jacobian contexts, and 2,771 typed maps.

| Map profile | Rows | Evidence boundary |
|:--|--:|:--|
| Binary cubic, same declared field | 186 | Explicit cover geometry and bounded typed point-class operations; arbitrary Jacobian norm remains unsupported. |
| Odd prime, direct quadratic | 1,184 | Explicit cover geometry; balanced two-infinity Jacobian transfer is unavailable. |
| Odd prime, normalized quadratic | 30 | Explicit geometry and checked target normalization; no full transfer pipeline. |
| Odd extension, direct quadratic | 1,371 | Explicit cover geometry over the declared extension; no descent or full transfer pipeline. |

The [profile graph](assessment.svg) displays the map counts and verified
geometry boundary. The [typed-context graph](typed-context.svg) displays the
field, Jacobian, map, and EC1 binding counts. Both are rendered from their
committed Graphviz sources; the PDF embeds the profile graph.

## Provenance and checks

The three catalog files (`registry.json`, `covers.json`, and
`cover-links.yaml`) are byte identical to commit `525a4404`. Their 199 new
models compared with the earlier 2,572-row catalog match their named source
fixture field, coefficients, group order, and trace. The v2 fixture source
is the study-relative `coverage-fixtures-v2/fixtures.json` member of the
committed `research/three_leads_e2e_20261018/runs.tar.xz` archive, whose
SHA-256 is
`826c8c1a29830f27bfb6956d766fdb2fea122087b9afd5a013ab9e7fa91483fc`.

Run `node scripts/build_cover_transfer_assessments.mjs --check` to check
the generated assessment byte for byte, then run
`node research/cover_transfer_assessment_20261013/verify_assessment.mjs`
to independently reconstruct the joins and formulas. The latter binds this
report, both graph sources and renderings, and the PDF by SHA-256; the PDF
subject is the exact coverage marker above. The release library suite passed
4,494 tests with 119 ignored when run serially, and the standalone
`hyperelliptic-cover` suite passed 13/13 including all 2,771 current models.

## Claim boundary

Classification and cover geometry do not establish a full Jacobian transfer
pipeline, a discrete-log improvement, or a wall-speed result. The 186 binary
maps have bounded typed point-class operations. The 2,585 odd-characteristic
maps have explicit cover geometry without compatible balanced two-infinity
Jacobian transfer. The existing conditional Lean group lemma does not import
this catalog or certify its finite-field calculations. The old packaged
release replay tested a smaller bundled registry and remains historical; it
is not a 2,771-command packaged-artifact replay.
