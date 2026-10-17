# Pre-run control clarification

**Date:** 2026-10-17 UTC. This addendum is committed before the first
capacity worker run. It corrects the test location in the frozen protocol;
it does not alter the retained object, public fixture, model caps, memory or
wall envelope, or outcome rules.

The existing checked m=4 planted relation and rank control is
`cryptanalysis::koblitz_index_calculus::tests::chained_s3_strategy_feeds_verified_relation_rank_and_target_solve`.
It runs m=4 and m=5 on a small field and checks verified relation, rank and
answer behavior. The degree-83 preflight controls are
`cryptanalysis::binary_semaev_chain_sat::tests::source_bound_covers_degree_83_without_constructing_a_model`
and
`cryptanalysis::binary_semaev_chain_sat::tests::frobenius_orbit_preflight_caps_before_mutation_and_counts_n83`.
Run those exact native library tests and the exporter's example tests before
the capacity worker. They check the core and importer integration at their
stated sizes; they do not constitute a degree-83 planted relation on the
K=16,627 object. That remains the next gate if capacity succeeds.
