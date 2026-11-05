# Exact overlap bound and rank-aware anchor selection for P-192

**Result.** A deterministic 32-endpoint selection on the frozen Type I P-192 quotient gives **722 verified endpointwise rows, rank 310, and 2,221 projected columns**. Joining the same 22 frozen cross-cubic known-scalar rows gives **rank 330 on 2,252 selected columns**. Selected support per rank is 2,252/330 = **6.824**, compared with 2,094/302 = **6.934** for the preceding four-anchor policy, a 1.58% reduction. Independent integer-coordinate replay verifies all 1,397 selected split or certified edges and all 722 rows. The exact-point field audit and quotient projection agree on the row rank and column count.

The [field screen](../../screen_overlap_anchor.py) enumerates every residual endpoint from the 158 split parent lines attached to query-root anchors `0,3,6,9`. Its 25,596 directed cuts yield 88 eligible anchor-labelled candidates at 87 distinct endpoint coordinates. Eligibility requires a complete partner cubic and another complete cubic, so every selected endpoint produces a known-scalar relation. The [independent field incidence audit](../../analyze_overlap_anchor_field_incidence.py) reconstructs the selected cuts and rows from the frozen quartic equation.

## Exact cross-anchor overlap bound

Across all eligible candidates with different endpoint coordinates, only four exact points outside the reciprocal baseline support can appear in two different anchor groups: query roots `0,9,57,64`. Root `0` requires candidate `(9,0,1)` in the other group; root `9` requires candidate `(0,9,1)`. Those two differently labelled candidates have the **same residual endpoint coordinate**. A selection of distinct endpoint coordinates therefore realizes at most three of the four cross-anchor shared points. The selected policy attains three, at roots `9,57,64`. This is an exhaustive bound for the 88-candidate pool and this exact-point support definition.

[OverlapAnchorBound.lean](../../lean/OverlapAnchorBound.lean) proves the structural implication: an endpoint-unique selection excludes one of the two conflicting candidate labels, and the resulting subset of four possible points has cardinality at most three. It also checks an abstract three-point witness. The field screen verifies the concrete four-point universe, the two candidate implications, the equal endpoint coordinates, and the selected three-point witness. The P-192 field arithmetic and quotient map remain computationally checked premises of the Lean theorem.

## Selection and rank

The complete 87-distinct-endpoint field row family has 924 rows and rank **425**. Its rank equals the reciprocal baseline rank 181 plus the sum 244 of the individual endpoint row ranks. Thus every subset of this fixed unique-endpoint pool has additive row rank, and the screen can evaluate a selection's exact-point column count and rank before quotient projection.

The policy fixes five candidate keys needed to attain shared roots `9,57,64`; fills eight slots per anchor from four deterministic orders and 24 pseudorandom orders with seed `20261027`; then applies the best strictly improving single-key swap until locally stable. It minimizes the exact-point columns/rank ratio subject to distinct endpoints and the five fixed keys. All 28 starts reach the same 2,221-column, rank-310 score. This certifies the reported search path and local swap outcome; it is not a global optimization certificate over every 32-endpoint subset.

| Frozen relation selection | New endpoints | Endpointwise rows | Endpointwise rank | Endpointwise columns | Joined rank with 22 cross-cubic rows | Joined selected columns | Joined columns/rank |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Reciprocal query-root baseline | 0 | 370 | 181 | 1,316 | — | — | — |
| First eight eligible per anchor | 32 | 613 | 282 | 2,064 | 302 | 2,094 | 6.934 |
| Three-point overlap with rank-aware selection | 32 | **722** | **310** | **2,221** | **330** | **2,252** | **6.824** |

The rank-aware selection improves endpointwise columns/rank from 2,064/282 = 7.319 to 2,221/310 = **7.165**. The joined selected bundles number 418; their maximum column degree is nine, with four columns of degree at least eight. The three exact cross-anchor intersections persist after quotient projection, one point for each of anchor pairs `0,9`, `3,6`, and `3,9`.

[Joux and Vitse, §§3–4](https://eprint.iacr.org/2011/020) give the cover-and-decomposition relation framework and the genus-3/genus-2 branches. This run supplies a finite P-192 selector, a sharp overlap bound for its eligible pool, and replayed known-scalar line equations. The next cost experiment is ordinary-query relation yield and novel rank under a fixed query law, followed by final relation linear algebra and target handling. The research record retains `candidate_id: null` while that pipeline is unspecified.

## Proof and verification map

| Result | Checked declaration or receipt | Boundary |
| --- | --- | --- |
| Endpoint uniqueness excludes the two conflicting candidate labels; four possible shared points then give at most three | `distinct_endpoint_excludes_pair`, `overlap_card_le_three_of_distinct_endpoints` | Lean proof with explicit candidate-to-point implications. |
| Three-point abstract overlap witness | `three_overlap_indices_exist` | Lean proof; concrete roots are checked by the field screen. |
| Exhaustive eligible-pool overlap and additive row rank | [Field screen](../../type_i_overlap_anchor_field_screen.json) | Exact quartic cuts over the frozen P-192 field and modular elimination over the recorded subgroup order. |
| Selected exact-point incidence and rank | [Field incidence receipt](../../type_i_overlap_anchor_field_incidence.json) | Independently reconstructs 411 active bundles, 722 rows, 2,221 exact points, and rank 310. |
| Quotient images, subgroup equations, and row rank | [Sage receipt](../../type_i_overlap_anchor_relation_fan.json), [coordinate replay](../../type_i_overlap_anchor_relation_fan_replay.json) | Integer-coordinate replay checks quartic collinearity, source and quotient group equations, every projected edge, row scalars, and rank. |
| Joined rank and support | [Combined receipt](../../type_i_overlap_anchor_combined_rows.json) | Independent minimum- and maximum-pivot modular elimination and cross-cubic scalar replay. |

The Lean v4.34.1/mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612` build, axiom audit, and fresh checker passed locally. `#print axioms` reports only standard `propext`, `Classical.choice`, and `Quot.sound` for the main theorem. No proof placeholders or custom claim axioms are present.

The checked Sage launcher saved [runtime-info.json](runtime-info.json) before projection. Fifteen endpoint projections and 626 split edges were reused from the independently replayed preceding run; the producer projected 707 new split edges. It also checked 64 certified parent-line cuts and 1,195 nonsplit cuts. The unisolated scan, subgroup checks, bundle construction, and rank interval was 323.2 seconds, excluding checked-launcher startup and cover-model initialization. This interval is a diagnostic for the recorded host load and reuse policy.

The 1,397 edge records are stored in 32 compact, SHA-256-linked JSON shards, one endpoint per shard and each below the repository's 64 KiB data threshold. [The loader](../../overlap_anchor_receipt.py) verifies shard hashes, endpoint ranges, counts, and unique edge keys before replay. The [run manifest](ARTIFACTS.sha256) checks the complete immutable source, result, and proof-log set.

Reproduce the replay from the `crypto` worktree root:

```sh
python3 -B research/p192_extension_cover_20261012/screen_overlap_anchor.py
python3 -B research/p192_extension_cover_20261012/analyze_overlap_anchor_field_incidence.py
python3 -B research/p192_extension_cover_20261012/verify_overlap_anchor_relation_fan.py
python3 -B research/p192_extension_cover_20261012/analyze_overlap_anchor_combined_rows.py
cd research/p192_extension_cover_20261012/lean
lake build OverlapAnchorBound
lake env lean OverlapAnchorBound.lean
lake env leanchecker --fresh --verbose OverlapAnchorBound
```
