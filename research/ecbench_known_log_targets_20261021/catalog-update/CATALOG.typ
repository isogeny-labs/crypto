#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: none)
= Cover catalog update for benchmark fixtures
2026-10-21 · Versioned metadata report · Geometry is not transfer advantage

Three synthetic cofactor models and five exact generator representations extend
the catalog to 2,575 map rows, 2,573 unique cover models/Jacobian contexts,
982 exact EC1/curve/map triples and 510 declared-field contexts. The previous
2,572-row report and its evidence remain unchanged. Registry, covers and graph
are frozen in commit 9be2f56ac440b5eeedf1438094a59e8795d66936.

#image("typed_correspondence.svg", width: 100%, height: 180mm, fit: "contain")

The three added models have p=65519 and cofactors 2, 4 and 8. The other two
new representations use [2]G and [7]G on the existing 16-bit benchmark model.
They join the direct odd-prime quadratic geometry profile; no claim status or
implementation domain is widened. README.md lists their exact identities.

#pagebreak()
= Typed bindings and verification

#image("typed_transfer_context.svg", width: 100%, height: 180mm, fit: "contain")

Native generation and replay verify all 2,575 catalog geometry rows with zero
invalid/unsupported inputs. Eleven native checker tests pass, including the
entire catalog replay. The standalone-constructor test checks every current
catalog model. The native 2,751-row trait rebuild matches the guarded
incremental update byte for byte. See the study's verification logs.

The assessment retains its supported/refuted/unknown/not-applicable matrix.
Geometry does not establish catalog-general induced Jacobian maps, an arbitrary
norm, exact source subgroup order or a cover-integrated IC solver. The known-log
ECDLP benchmark works directly on E and uses no cover transfer. This catalog
update makes no performance claim. The older packaged-artifact receipt remains
evidence only for its bundled 592 rows and historical allowlist comparison.

Sources: this README and graph sources; docs/curves/registry.json, covers.json,
cover-links.yaml and cover-transfer-assessments.json; the live independent
cover verifier and the study verification logs. The previous report is preserved
at research/cover_transfer_assessment_20261013/README.md.
