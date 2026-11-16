# Reuse quotient-ring powers in balanced streamed evaluation

T-1872. Producer assertion, independent signed verdict pending. [pcc:unverified]

## Algorithm and scope

The published split evaluator recursively returned both a block value and its base power, recomputing powers at repeated block lengths. Preserve that implementation as the reference. The new ReusedRing owns an immutable split-ring context, a scale key and a map from exponent to SplitValue. On a changed scale, clear the map and install powers0/1. A changed support uses a new ring. There is no cross-target or cross-support cache. C2's two modes share the same ring and scale, and therefore may share computed powers.

The value recursion reads the same descending coefficient stream once. For a leaf it returns its coefficient. For consecutive left/right blocks, it returns left_value * base^(right_length) + right_value. It does not compute or return the top-level power. Powers are computed by binary exponentiation on first request: power(2j)=power(j)^2 and power(2j+1)=power(j)^2*base, then stored by exponent. Lookup returns a clone, whose allocation/copy cost remains charged.

Induction on block length proves equality with the reference polynomial. Induction on exponent proves each cached power. The scale key prevents reuse at a different base; owned support/root polynomials prevent accidental support changes in the recovery adapter. This is ordinary memoized exponentiation, not a novel algebraic transfer.

## Structural cost

For L coefficients, value composition uses L−1 ring products per present component and reads exactly L coefficients. Let P be the number of cold power-construction products actually performed. New evaluator work is L−1+P, compared with 2(L−1) products in the reference. Warm calls pay only newly needed exponents. Empty components do no polynomial products.

Balanced recursion has at most two distinct node lengths at each depth. Hence O(log L) distinct right-length exponents are requested. Binary exponentiation of each requires O(log L) products, giving the conservative bound P=O(log² L), even without exploiting overlap between their exponent chains. At most O(log² L) power entries are retained under this bound. For total support degree d, retained power coefficients are O((d+1)log² L). This bound concerns one bounded mode family, not arbitrary externally requested powers. The checker separately requests arbitrary powers only to test correctness.

The cache replaces repeated arithmetic with BTreeMap lookups, allocations, stored polynomials and clones. Live coefficient lengths reported by the checker omit Vec capacities, map-node overhead and temporary recursion values; they are not peak allocated bytes. Scalar and pair ring products are counted separately by the underlying evaluator and summed for the reported component-product totals; a mixed state contributes two component products per abstract product and an empty state contributes zero. Pair products still contain three base polynomial multiplications, scalar products one; these are not field-operation or wall-time ratios.

All initial support/C0 construction, source grids, exact-order checks, coefficient generation, modular reductions, inverse/CRT work, source queries and final public-point validation remain charged. Four-radius limits, caps, reference adapters and the original4,563cohort remain unchanged. No cloud or timed workload is run.

## Controls and failure preservation

Five fields r=17,41,97,257,1009 with corresponding m=4,5,3,4,4 test four support shapes: unresolved-only, mixed, selected-only and empty. Four scale transitions and all three modes compare against the reference and check coefficient-read equality, actual component-product counters and scale resets. Independent repeated multiplication checks powers0..65 in each support shape. Counts for mode cache sizes exclude this deliberately broader arbitrary-power checker.

The inherited state panel compares all canonical triples, branch supports, zero flags and materialized candidates through four radii; two public panels compare the previous split adapter and new adapter. See RESULTS.md and validation02 for observed outcomes.

validation01 failed with EmptyTree because its empty-support fixture called product_tree([]). The correct empty support is the unit polynomial. Preserve failed-reused-checks.rs, source-snapshot.json, receipt and stderr. Only checker fixture/accounting changed for validation02; production evaluator was unchanged. The second attempt also counts actual present components so empty supports contribute zero to totals. The inherited fullCargo fixture/deadline limitation remains; fullCargo is not repeated for this standalone native construction.

## Sources and next direction

Read feature-native-split-modes/split.rs and its checks, plus the canonical/persistent adapters before editing. Official FLINT polynomial quotient and powering conventions: https://flintlib.org/doc/nmod_poly/index.html. The KB MCP is unavailable and scripts/research-kb status reports no executable; exact predecessor and primary documentation are the fallback. No novelty claim.

Next avoid regenerating the same coefficient stream separately for each surviving phase branch. A shared coefficient traversal must retain per-branch support changes and charge simultaneous live evaluation state. Any timing claim requires a separately admitted frozen workload and full source/setup accounting.
