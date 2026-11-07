# Native fused inversion controls

Producer assertion; independent signed verdict pending. [pcc:unverified]

```text
fused_sequences=460 coefficient_and_state_equalities=6420 chunk_boundaries=2415 inversions=2415 reference_inversions=4635 multiplications_outside_exponentiation=64315 reference_multiplications_outside_exponentiation=66535 max_denominators=128 inverse_identity_checks=544 invalid_inverse_inputs=4
fused_mode_groups=75 branch_mode_equalities=300 ring_product_counters_equal=true cumulative_counter_observation_sum=44760
fused_canonical_states=2516
fused_public_work cap=64 pairs=275 coefficients=7700 transitions=7425 inversions=1100 reference_inversions=2200 multiplications_outside_exponentiation=76725 reference_multiplications_outside_exponentiation=77825 max_denominators=14
fused_public_cases=281 cap=64 source_calls=2881 reference_source_calls=2881 route_changes=0 unresolved=0 initializations=275 subsequent_updates=0 deferrals=0 materializations=275
fused_public_work cap=16 pairs=535 coefficients=14980 transitions=14445 inversions=2140 reference_inversions=4280 multiplications_outside_exponentiation=149265 reference_multiplications_outside_exponentiation=151405 max_denominators=14
fused_public_cases=281 cap=16 source_calls=3152 reference_source_calls=3152 route_changes=0 unresolved=0 initializations=275 subsequent_updates=260 deferrals=265 materializations=270
```

Compile/check exit0;85transitive input pins checked. Inherited controls pass. Fused multiplications outside inverse exponentiation are counted; reference counts follow exact predecessor loops on the same chunks. Counts exclude initialization, interval products, polynomial arithmetic and source costs, all still charged. The cumulative-counter observation debug field is not a product total and is unused. No new compiler/native/render failure or cloud/timing workload. Historical fullCargo fixture/deadline remains; fullCargo not repeated.
