# Graph-IC: cache equation-edge inverses for coefficient-aware LP witnesses

Date: 2026-11-23. Owning draft: [PR #4988](https://github.com/isogeny-labs/crypto/pull/4988). Status: **source authored; hosted native gate pending at this checkpoint**. [pcc:unverified]

## Concrete bottleneck and mathematical mechanism

The exact forest witness extractor admits canonical rows of arity at most two over the relation field F_ell. The prior rooted-parent implementation followed H parent edges across all projections and called extended Euclid once **per hop** to divide by the child coefficient, even though the equation-edge coefficients are immutable after admission. On long or frequently queried paths, these repeated inversions can dominate lightweight graph bookkeeping. This does not concern the coordinate field F_p or the elliptic-curve group law.

For an independent edge a*x_u+b*x_v=0, precompute a^(-1) and b^(-1) while validating both nonzero coefficient units. Save the pair under the admitted original row ID. A traversal from either endpoint selects the matching inverse, multiplies the incoming coefficient, and propagates to the opposite endpoint exactly as before. Reorienting a smaller component's equation tree changes **which** inverse is selected, not the stored row coefficients or their inverses. Non-tree rows and anchors do not need persistent edge inverses.

Thus each traversal is exactly the old modular elimination, but inversion is replaced by a table lookup. The control path remains the previous head `21418acd899edcb07a90efd5a665b6b0d305687f`; no producer/source/group relation changes.

## Operation and memory bounds

Let I be total input LP incidences, R input rows, H total path hops and A the number of anchored-root normalizations. The old implementation counted I + H + A modular inversions; this implementation counts **I + A**, with A <= 2R in its arity-two domain. Thus the inversion complexity falls from O(I+H+R) to O(I+R), with O(R) storage for optional two-entry caches and no change to O(H) path traversal or output-sensitive source-map operations. The word arithmetic, inverse bit costs, peak RSS, sparse payload, factor-base output and reconstruction remain chargeable. This is a component operation-count improvement, **not** a measured total solver speedup or an ECDLP exponent improvement.

A composite odd modulus is outside the rank theorem's field assumptions. The existing explicit inverse check remains before all semantic mutation; cached inverses are used only after that check. Dependent rows and anchored singletons are never referenced as parent edges.

## Native falsification gate and controls

A new original-source Rust regression forms a 32-vertex F101 equation path, appends 48 identical balanced closing edges, and checks 48 exact coefficient-bearing witnesses, original-source reconstruction and nonzero path traversals. All 79 rows have two canonical coefficients; consequently the new `inverse_calls` count must equal **158**, while path work exceeds three times that count. This is a deliberately synthetic long-path case, not a Prime31 collector, performance timing or an independent PCC audit. Run the existing 23 tests with the new test through `research/lp_graph_algorithms_20261123/native_gain_dsu/run.sh`; specifically retain the 450 dense-rank tests, 160 source replays, 96 pivot-projection comparisons and anchored/rerooting controls.

Prior evidence: [rooted-parent study](ROOTED_WITNESS_PATHS_20261123.md), [forest witness design](FOREST_WITNESS_DESIGN_20261123.md), merged [graph catalog PR #4917](https://github.com/isogeny-labs/crypto/pull/4917), and the original `large_prime_graph.rs` and `witness.rs` in PR #4988. The primary algebraic identity used here is `a*a^(-1)=1` in F_ell; no novelty claim. **Sources checked (KB):** no research-KB or Conductor tools are exposed in this execution; the local wrapper is unavailable without a checkout. Bounded GitHub primary-source fallback as above; literature survey remains limited.

## Next measurement and exact acceptance gate

Acquire an **authentic public Prime31 coefficient-bearing two-LP collector stream** with independently checked original elliptic-curve source equations and recorded subgroup order ell. Replay same stream through the frozen rooted-parent control and cached arm, along with pivot and MST filters. Require identical source coefficients, RHS and projected factor-base rank in uncapped controls, no invalid elliptic-curve relation, and zero replay failures. Charge inversions, modular multiplies, input normalization, path and reroot hops, witness map operations, coefficient caps/refusals, peak memory, and isolated same-host wall time. Test capped cases separately: completeness claims do not survive arbitrary witness refusal. Only after this phase and a complete-cold target solve is a performance claim admissible.

No paid compute, merge, auto-merge or leaderboard promotion authorized.
