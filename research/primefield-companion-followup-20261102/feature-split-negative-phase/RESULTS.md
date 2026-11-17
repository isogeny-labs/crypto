# Bounded negative phase split qualification

PCC-20261108-5e51c6 is unsigned producer evidence; independent signed review pending. [pcc:unverified]

Validation02 output:

```
support_equalities=20352 true_images_retained=10080 wrong_sign_exclusions=10080 zero_exclusions=192 public_target_capacity_cases=3372 route_changes=0 call_changes=0 subsequent_updates=2660 single_call_cap_cases=280 invalid_signs=rejected phase_choices=2_of_4
dickson_equalities=132 powered_initializations=4262 streaming_fallbacks=4126 public_outcomes_routes_stats_equal=true prior_phase_degree_checks=24 prior_capacity_boundaries=12
trace_field_controls=7 coprime_factorizations=208 phase_union_equalities=20352 zero_search_public_cases=281 root_searches=2290 candidate_tests=5433 roots_found=2131 split_domain_builds=2228 exhausted_search_fallback=pass split_capacity_boundary_groups=5
```

20,352 exact materialized-state comparisons cover six configurations, both
initial signs, section capacities0/16/96/1024, and radii2/1/3/2. At the fourth
radius, grouping all lifted branch polynomials by each original u gives20,352
additional equality checks. This deliberately handles repeated u labels rather
than comparing individual canonical signatures. True/wrong-sign and zero
controls are retained. [pcc:unverified]

All3,372 public target/capacity cases recover correctly with equal outcomes,
routes and calls/extra/replayed/lifted/image_count statistics to the direct-phase
predecessor. These are281 targets on E/F257:y²=x³+x+7, P=(1,3), order281,m5,
three lift caps8/16/64 and four section capacities0/16/96/1024. The96-slot
case lies between the new70-slot and old140-slot negative-phase thresholds.
Additional281 zero-search public cases retain correct scalars, routes and call
counts;280 one-call controls remain. [pcc:unverified]

Trace controls cover F17,F41,F73,F97,F193,F257,F281, zero budget and candidate
limits. The deterministic F17 one-candidate search exhausts, F13 is unsupported,
and a65-candidate request is rejected.208 factor-product comparisons also check
coprimality and squarefreeness. Five new boundary groups each check exact split
capacity, one-below rejection, zero-search rejection at the smaller capacity
and zero-search acceptance at the old capacity. Identity input also rejects
an invalid trace budget. [pcc:unverified]

Root searches, candidate tests and split builds in stdout are workload coverage
counts, not timings or evidence that branch splitting reduces total work.
Both revisions build and pass focused/inherited checks. Validation01 predates
the new boundary checks; its changed source snapshot and receipt are preserved.
127 copied sources and142 compiler dependencies are hash-bound. Final inherited
stdout is byte-identical. No compile/native failure, cloud job or timing panel.
Original4563-case cohort unchanged; historical full-Cargo missing-fixture limit
remains. No new Lean runtime refinement or independent signed verdict.

The current implementation keeps four negative branches after splitting and
recomputes the public trace per eligible initialization. Character-phase sharing
and odd-L admission are documented proposals, not implemented results.
