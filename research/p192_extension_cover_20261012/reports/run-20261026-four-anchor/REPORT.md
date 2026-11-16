# Four-anchor recurrence in the P-192 extension-cover relation graph

**Result.** Four fixed query roots in different cubics each occur in at least eight complete relation bundles under a frozen cross-cubic residual-endpoint policy. The combined endpointwise system has **613 verified rows, rank 282, and 2,064 projected columns**. Adding the 22 previously frozen cross-cubic known-scalar rows gives **rank 302 on 2,093 row-active columns**. The maximum selected bundle-column degree is nine; four columns have degree at least eight. The four fans contribute 101 rank beyond the rank-181 baseline, while their largely separate supports add 748 columns.

The [field screen](../../screen_multi_anchor.py) fixes anchors `0,3,6,9`, the first roots of four distinct frozen query cubics. For each anchor, it takes the first eight globally distinct residual endpoints in partner-root/residual-side order whose partner cubic and at least one other cubic have complete cuts. The 158 split cross-cubic parent lines give 316 candidate residual endpoints. Eligibility counts by anchor are 14, 17, 30, and 27; the rule selects 32 distinct endpoints before quotient projection. Each partner-cubic bundle contains its anchor as an exact quartic residual. The earlier [anchor bundle theorem](../../lean/CrossCubicAnchorFan.lean) proves the known-scalar group equation for such a bundle. [FourAnchorRecurrence.lean](../../lean/FourAnchorRecurrence.lean) proves that four distinct anchor columns, each in eight distinct active bundle keys, give at least four columns of degree at least eight.

| Frozen policy | New endpoints | Active bundles | Endpointwise rows | Rank | Projected columns | Columns of degree at least eight |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Reciprocal query-root baseline | 0 | 250 | 370 | 181 | 1,316 | 0 |
| Root-zero cross-cubic fan | 8 | 279 | 418 | 202 | 1,478 | 1 |
| Four-anchor cross-cubic fan | 32 | **383** | **613** | **282** | **2,064** | **4** |
| Four-anchor fan plus frozen cross-cubic rows | 32 | 390 selected | 635 total | **302** | 2,093 row-active | **4** |

The four individual eight-endpoint groups raise the baseline rank to 202, 211, 206, and 206, respectively: increments of 21, 30, 25, and 25. These sum to the measured combined increment of 101. The [exact point incidence audit](../../analyze_four_anchor_field_incidence.py) independently obtains 383 active bundles, 613 rows, 2,064 exact quartic points, and exact-point column rank 282. Its new-support intersections between different anchor groups contain only one point, shared between the root-zero and root-nine groups. The projected counts coincide with the exact-point counts for this support. The Sage run screens 2,592 new directed cuts: 1,254 newly split, 64 certified parent-line cuts, and 1,274 nonsplit.

[Joux and Vitse, §§3–4](https://eprint.iacr.org/2011/020) provide the cover-and-decomposition relation framework and the genus-3/genus-2 branches. This run adds a frozen four-anchor selection on the P-192 Type I quotient, exact recurrence counts, and independent coordinate replay for its known-scalar line equations. The source paper's relation collection, final linear algebra, and target descent remain the cost stages to instantiate for this selection.

The [combined analyzer](../../analyze_four_anchor_combined_rows.py) adds the 22 frozen cross-cubic rows and obtains rank 302 with both maximum- and minimum-pivot modular elimination. The 390 selected bundles have 2,340 distinct bundle-column incidences across 2,094 selected support columns: 1,877 appear in one bundle, 209 in two, four in three, three in eight, and one in nine. One selected support column cancels from every row, leaving 2,093 row-active columns. Selected support per rank is 2,094/302 = 6.93, compared with 1,508/222 = 6.79 for the frozen single-anchor combined policy. The extra anchors increase verified rank and the number of recurring columns; this selection rule does not increase support density. The next bounded geometry policy should select cross-anchor lines for shared residual support, then test whether its rank and column-cost ratio improves under the same known-scalar row law.

| Result | Lean declaration or receipt | Verification boundary |
| --- | --- | --- |
| Four distinct anchors with eight active bundle keys each yield four high-degree columns | `four_anchor_eightfold_column_count` | Lean-checked finite-set implication with injective anchor and per-anchor key maps. |
| Individual anchored bundle known-scalar equation | `cross_cubic_anchor_bundle` | Previously checked additive-group theorem, imported by the four-anchor module. |
| Exact quartic split and point recurrence | [Field receipt](../../type_i_four_anchor_field_incidence.json) | Deterministic arithmetic over the frozen P-192 field; maximum- and minimum-pivot rank both return 282 when exact points are separate columns. |
| Quotient images, subgroup equations, 1,318 split edges, and 613 endpointwise rows | [Sage receipt](../../type_i_four_anchor_relation_fan.json) and [independent coordinate replay](../../type_i_four_anchor_relation_fan_replay.json) | Integer-coordinate replay checks quartic collinearity, group equations, the selected complete cubics, row scalars, and rank 282. The Sage producer computes the quotient projection; a full formalization of that concrete map remains open. |
| Joined rank, selected support, and cross-anchor column overlap | [Combined receipt](../../type_i_four_anchor_combined_rows.json) | Independent minimum- and maximum-pivot modular elimination, with frozen triple-row scalar replay. |

The pinned Lean v4.34.1/mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612` build, axiom audit, and fresh checker pass. The new theorem uses standard `propext`, `Classical.choice`, and `Quot.sound`; it has no placeholders or custom claim axioms. The formal theorem preserves the distinctness and bundle-membership hypotheses, while the coordinate receipts check those hypotheses for this finite P-192 selection.

The checked Sage launcher saved [runtime-info.json](runtime-info.json) before the workload. The unisolated scan, group checks, bundle construction, and rank interval was 528.4 seconds; checked-launcher startup and cover-model initialization are outside that interval. This timing is a diagnostic for this run and host load. The candidate remains `candidate_id: null` while ordinary-query yield, a complete factor-base policy, final relation linear algebra, and target handling remain to be specified and measured.

The Sage receipt stores its 1,318 projected edges in 16 compact, sorted-key JSON shards, two endpoints per shard. Every shard is below the repository's 64 KB data threshold; the 15 KB main receipt records each shard's SHA-256, endpoint range, and edge count. After the scan, only the receipt serialization and producer source digest were refreshed to match this writer; the recorded lines, points, rows, counts, and elapsed interval were preserved. [The loader](../../four_anchor_receipt.py) verifies all shard hashes, endpoint ranges, counts, and edge-key uniqueness before independent replay and combined-rank analysis, both of which were rerun against the committed bytes.

Reproduce from the `crypto` worktree root:

```sh
python3 -B research/p192_extension_cover_20261012/screen_multi_anchor.py
python3 -B research/p192_extension_cover_20261012/analyze_four_anchor_field_incidence.py
python3 -B research/p192_extension_cover_20261012/verify_four_anchor_relation_fan.py
python3 -B research/p192_extension_cover_20261012/analyze_four_anchor_combined_rows.py
cd research/p192_extension_cover_20261012/lean
lake build FourAnchorRecurrence
lake env lean FourAnchorRecurrence.lean
lake env leanchecker --fresh --verbose FourAnchorRecurrence
```
