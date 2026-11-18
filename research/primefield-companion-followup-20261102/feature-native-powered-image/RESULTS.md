# Powered image and phase qualification

PCC-20261108-f1fedf is unsigned producer evidence; independent signed review pending. [pcc:unverified]

Validation03 passed:

```
quadratic_section_equalities=1008 canonical_state_equalities=7632 public_target_capacity_cases=2529 subsequent_updates=2205 powered_calls=18265 algebra_products=167278 fallback_branches=19112 outcomes_routes_source_stats_equal=true
powered_phase_calls=700 fallback_phase_calls=350
```

All2529 public target/capacity cases return the correct fixture scalar and match the streamed adapter's route and calls/extra/replayed/lifted/image_count statistics. These are281 targets at three lift caps and three power capacities on one curve, not2529 independent curve instances. The state controls match materialized support, zero/round state and canonical branch signatures, including repeated radii. The section controls separately match dense polynomial evaluation in the quadratic quotient. [pcc:unverified]

Coverage confirms powered and fallback branch paths and700 powered versus350 fallback terminal phase calls. Counter totals are over the deterministic fixture workload; they are not timings or a performance ranking.113 copied source records and128 compiler dependencies are hash-bound. Final inherited stdout is byte-identical. [pcc:unverified]

Validation01 qualified branch integration; inspection then identified the terminal scalar coefficient scan. Validation02 qualified its powered helper. Validation03 added explicit phase-path coverage assertions and reran inherited checks. All three builds/runs succeeded; source snapshots preserve the additions. No failed native assertion or compiler error occurred.

Lean4.34.1 checked the conditional direct signed-product ring identity with standard axioms propext/Classical.choice/Quot.sound. Full finite-field weighted projection, quotient-section proof and runtime refinement remain outside that lemma. The report was rendered and visually inspected.

The caller's power_slots cap bounds logical represented output coefficients, not peak memory. It controls capacity fallback and is not a selected speed threshold. Initial mode0 construction, source extraction, public-radius normalization, canonical filtering/lift/replay and point verification remain charged. The new adapter is optional; no default policy or frozen original cohort changes. No cloud or timing panel ran. Historical full Cargo missing-fixture limitation remains.
