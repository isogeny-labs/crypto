# Eightfold projected-column recurrence from cross-cubic anchor lines

**Result.** Fixing one query root and taking residual endpoints from split lines to roots of other query cubics produces a projected column in **eight distinct active complete bundles**. The frozen eight-endpoint policy gives 418 verified endpointwise rows, rank 202, and 1,478 active projected columns. Combining its rows with the 22 previously frozen cross-cubic known-scalar rows gives rank **222 on 1,507 columns**. This constructs the degree-eight recurrence identified by the preceding incidence bound and measures its effect on the relation system.

Let `A` be the fixed query root and `B` a root in another cubic. A split line has projected equation `A+B+R+T=S`. Use one residual `R` as an endpoint. The cut from `B` then returns `A,T`; if the other two roots `U,V` in `B`'s cubic have split cuts returning `Y,Z` and `H,J`, its complete bundle is `X_R=A+T+Y+Z+H+J`. With the query-cubic equation `B+U+V+kq=C`, the bundle satisfies `X_R+3R+C=3S+kq`. The anchored column `A` can therefore recur across partner cubics while each bundle retains the known-scalar equation. [CrossCubicAnchorFan.lean](../../lean/CrossCubicAnchorFan.lean) proves this identity and the finite-set implication from eight distinct bundle keys to degree at least eight.

The [base-field screen](../../screen_cross_cubic_anchor.py) fixes root index zero before inspecting eligible endpoints. Of its 34 split lines to roots in other cubics, the 68 residual endpoints yield 14 with at least two complete cubics including their partner cubic. The selection rule freezes the first eight in partner-root/residual-side order. These eight endpoints are distinct. Their partner bundles occur at `(cubic, endpoint)` keys `(1,81)`, `(2,82)`, `(3,83)`, `(3,84)`, `(4,85)`, `(7,86)`, `(7,87)`, and `(9,88)`. Each contains the same geometric root `A`; the Sage projection and independent integer-coordinate replay verify its common subgroup column. The separate [all-86 same-cubic residual screen](../../screen_residual_point_reuse.py) reaches maximum exact-point bundle degree four, motivating the cross-cubic family.

| Frozen policy | New directed cuts | New split / certified / nonsplit | Active bundles | Endpointwise rows | Rank | Active columns | Maximum bundle-column degree |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Earlier targeted eight same-cubic residual endpoints | 648 | 293 / 16 / 339 | 276 | 404 | 199 | 1,454 | 4 |
| Cross-cubic anchor, eight endpoints | 648 | 307 / 16 / 325 | 279 | 418 | **202** | 1,478 | **8** |
| Cross-cubic anchor plus 22 frozen cross-cubic rows | 648 | 307 / 16 / 325 | 286 selected | 440 total | **222** | 1,507 | **8** |

The combined policy has 1,716 distinct bundle-column incidences across 1,508 selected support columns: 1,308 occur in one selected bundle, 197 in two, two in three, and one in eight. One selected support column cancels from every combined row, leaving 1,507 row-active columns. At frozen rank 222, the incidence bound still requires maximum degree at least eight for closure. This construction meets that **necessary** degree threshold; the measured system remains far from column closure. The next geometry experiment should increase recurrence for many columns at once or reduce the distinct support per bundle, then measure ordinary-query yield and rank under a fully specified collector.

| Mathematical or empirical result | Declaration or receipt | Verification boundary |
| --- | --- | --- |
| Cross-cubic anchor bundle preserves the known-scalar equation | `cross_cubic_anchor_bundle` | Lean-checked additive-group implication from one anchor line, two other cuts, and the partner query-cubic sum. |
| Eight distinct active bundle keys containing one anchor have column degree at least eight | `cross_cubic_anchor_eight_bundle_degree` | Lean-checked finite-set implication. Concrete P-192 keys and column membership are verified by coordinate replay. |
| P-192 split lines, quotient images, subgroup equations, and endpointwise rank | [Sage receipt](../../type_i_cross_cubic_anchor_degree_eight.json) and [independent replay](../../type_i_cross_cubic_anchor_degree_eight_replay.json) | Replay checks quartic collinearity and elliptic-group equations for all 323 new split edges, all 418 rows, and rank 202. The quotient projection is computed by the checked Sage producer and its output group equations are independently replayed. |
| Combined row rank and incidence histogram | [Combined analyzer](../../type_i_cross_cubic_anchor_combined_rows.json) | Maximum- and minimum-pivot modular elimination both return rank 222; the analyzer recomputes selected supports and the degree histogram. |

The Lean toolchain is v4.34.1 with mathlib revision `d13f23b723b8a846827a245b89c10fc7d3f11612`. The build, `#print axioms` audit, and fresh checker logs are retained beside this report. The group theorem uses standard `propext`; the finite-set theorem uses `propext`, `Classical.choice`, and `Quot.sound`. Both theorems have no placeholders or claim axioms. Formalizing the concrete plane-quartic-to-quotient map remains a separate bridge.

The measured interval for the Sage scan, group checks, bundle construction, and rank was **50.0 seconds** on an unisolated host. It excludes checked-launcher startup and model initialization; [runtime-info.json](runtime-info.json) records the accepted Sage build. This is a geometry-stage measurement under frozen known-scalar queries. The candidate remains `candidate_id: null` until the full factor-base, relation-collection, linear-algebra, and target-descent stages are specified and measured.

Reproduce from the `crypto` worktree root:

```sh
python3 -B research/p192_extension_cover_20261012/screen_residual_point_reuse.py
python3 -B research/p192_extension_cover_20261012/screen_cross_cubic_anchor.py
python3 -B research/p192_extension_cover_20261012/verify_cross_cubic_anchor_degree_eight.py
python3 -B research/p192_extension_cover_20261012/analyze_cross_cubic_anchor_combined_rows.py
cd research/p192_extension_cover_20261012/lean
lake build CrossCubicAnchorFan
lake env lean CrossCubicAnchorFan.lean
lake env leanchecker --fresh --verbose CrossCubicAnchorFan
```
