# Fourfold projected-column recurrence from residual endpoints

**Result.** Adding residual points from split same-cubic root-pair lines as endpoints creates a known-scalar bundle with a repeated residual column. A frozen 86-endpoint third-cut screen identified two projected query-root columns that each enter four complete parent-cubic bundles. In the targeted eight-endpoint fan, all eight bundles contribute to relation rows: exact bundle-column degree is **four** for both columns. The independently replayed augmented system has **219 rank on 1,486 active columns** after combining 404 endpointwise and 22 cross-cubic rows. This is a concrete increase in cross-bundle recurrence over the query-root-only graph.

For a line with projected points `P+Q+R+T=S`, choose `R` as a new endpoint. The line cuts from the two roots `P,Q` return `(Q,T)` and `(P,T)`. If the third root `W` has a split cut `W+R+Y+Z=S`, then the complete parent-cubic bundle is `X_R=P+Q+2T+Y+Z`. With the query-cubic relation `P+Q+W+kq=C`, it satisfies `X_R+3R+C=3S+kq`. The repeated `T` has coefficient two in a row; it is one distinct support column. [ResidualEndpointClique.lean](../../lean/ResidualEndpointClique.lean) proves the group equation with these exact hypotheses.

The [first Sage pilot](../../residual_endpoint_clique_pilot.py) chose both residuals from the first 16 lexicographically ordered split lines joining two roots of one of the 27 frozen query cubics. The [field-only screen](../../screen_same_cubic_residual_endpoints.py) then checked the missing third-root cut for both residuals of all 43 such lines: **45 of 86 endpoints** have a complete parent-cubic bundle. Roots 58 and 78 each appear in four of those bundles. The [targeted Sage run](../../residual_endpoint_high_degree.py) selected the eight endpoints belonging to their four parent lines, before scanning any other cuts at those endpoints. Its exact root-bundle hits are:

| Projected query root | Parent cubic | Active bundle endpoints |
| ---: | ---: | --- |
| 58 | 19 | 81, 82, 83, 84 |
| 78 | 26 | 85, 86, 87, 88 |

| Frozen relation policy | Selected bundles | Endpointwise rows | Cross-cubic rows | Exact rank | Active columns | Maximum bundle-column degree |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Query-root endpoint baseline | 250 | 370 | 0 | 181 | 1,316 | 2 |
| First 32 residual endpoints | 369 | 600 | 0 | 272 | 2,006 | 2 |
| Targeted 8 residual endpoints | 276 | 404 | 0 | 199 | 1,454 | 4 |
| First 32 plus query-cubic triples | 376 | 600 | 22 | 292 | 2,038 | 2 |
| Targeted 8 plus query-cubic triples | 283 | 404 | 22 | **219** | **1,486** | **4** |

The first pilot screened 2,592 directed cuts at its new endpoints: 1,251 newly split, 1,277 nonsplit, and 64 certified parent-line cuts. It produced 15 active bundles with five distinct columns among six appearances, but no column degree above two. The targeted run screened 648 cuts: 293 newly split, 339 nonsplit, and 16 certified parent-line cuts. All eight new endpoints have at least two complete bundles, so their parent bundles enter the endpointwise rows. Its two degree-four columns and 198 degree-two columns occur among 1,486 selected columns; 1,286 selected columns occur in one bundle.

For the targeted combined policy, the 283 selected bundles have **1,690 distinct bundle-column incidences** (eight supports of size five and 275 of size six). The checked `mixed_query_bundle_incidence_bound` gives `1690 <= d B` if every column has bundle degree at most `d`. At the frozen rank 219, closure would require `B <= 219`, hence `d >= 8`. The checked `targeted_bundle_closure_requires_eightfold_reuse` proves this necessary threshold from explicit incidence and closure hypotheses. The measured degree four is an advance, while the actual 1,486 active columns remain far above rank. The all-86 parent-line screen has maximum forced root incidence four, so a higher-degree construction must use additional line families or column identifications.

| Result | Lean declaration or receipt | Verification boundary |
| --- | --- | --- |
| Known-scalar six-appearance bundle with one duplicated column | `same_cubic_line_duplicate_residual_bundle` | Checked additive-group proof from one split parent line, the third-root cut, and the query-cubic sum. |
| Mixed-size bundle incidence bound | `mixed_query_bundle_incidence_bound` | Checked finite double-counting proof; columns are distinct support elements within each bundle. |
| Rank-219 closure at 1,690 incidences requires degree at least eight | `targeted_bundle_closure_requires_eightfold_reuse` | Checked conditional proof; the finite receipts establish the frozen counts, not a Lean encoding of the P-192 coordinates. |
| P-192 geometry, quotient points, group equations, root hits, and modular ranks | [Sage receipts](../../type_i_residual_endpoint_high_degree.json), [independent replay](../../type_i_residual_endpoint_high_degree_replay.json), and [combined analyzer](../../type_i_residual_endpoint_combined_rows.json) | The independent integer-coordinate replay checks quartic collinearity, P-192 group equations, 309 new split edges, all 404 endpointwise rows, and rank 199. The combined analyzer checks ranks with both minimum- and maximum-pivot modular elimination. The Sage producer computes the quotient projection; that map itself is not reimplemented by the integer replay. |

Lean is pinned to v4.34.1 and mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612` by the [toolchain](../../lean/lean-toolchain) and [Lake file](../../lean/lakefile.toml). The module has no placeholders, custom claim axioms, or native proof evaluation. The axiom audit reports `propext` for the group theorem and `propext`, `Classical.choice`, and `Quot.sound` for the finite-set bounds. The concrete coordinate-to-Lean bridge remains an explicit proof obligation.

Both measured Sage runs used `/Volumes/SSD990/cryptanalysis/sage -python`, with verified runtime manifests saved [here](../run-20261024-residual-endpoint/runtime-info.json) and [here](runtime-info.json) before the workloads. Their unisolated scan, bundle verification, and rank intervals were 221.3 and 83.3 seconds, respectively; those intervals exclude launcher and model initialization and serve as run diagnostics, not a performance comparison. The [first pilot receipt](../../type_i_residual_endpoint_clique_pilot.json) and [its independent replay](../../type_i_residual_endpoint_clique_pilot_replay.json) preserve the full 32-endpoint result. The candidate remains a geometry-stage proposal with `candidate_id: null`; the next step is a bounded degree-at-least-eight line family followed by ordinary-query yield, rank, and complete target handling under the repository's measurement contract.

Reproduce from the `crypto` worktree root:

```sh
python3 -B research/p192_extension_cover_20261012/verify_residual_endpoint_clique_pilot.py
python3 -B research/p192_extension_cover_20261012/screen_same_cubic_residual_endpoints.py
python3 -B research/p192_extension_cover_20261012/verify_residual_endpoint_high_degree.py
python3 -B research/p192_extension_cover_20261012/analyze_residual_endpoint_combined_rows.py
cd research/p192_extension_cover_20261012/lean
lake build ResidualEndpointClique
lake env lean ResidualEndpointClique.lean
lake env leanchecker --fresh --verbose ResidualEndpointClique
```
