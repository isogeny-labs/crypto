# Native neighboring-mode controls

Producer assertion; independent signed verdict pending. [pcc:unverified]

```text
neighbor_sequences=460 coefficient_pairs=6420 adjacent_batch_inversions=2415 singleton_sequences=100 maximal_order_sequences=50 top_index_reaches_h=5 exhausted_reader_rejections=460 invalid_neighbor_inputs=6
neighbor_mode_groups=75 branch_mode_equalities=300 ring_product_counters_equal=true cumulative_counter_observation_sum=44760
neighbor_canonical_states=2516
neighbor_public_work cap=64 pairs=275 anchor_coefficients=7700 anchor_transitions=7425 adjacent_inversions=1100 adjacent_multiplications_outside_exponentiation=39600 max_buffer_pairs=7
neighbor_public_cases=281 cap=64 source_calls=2881 reference_source_calls=2881 route_changes=0 unresolved=0 initializations=275 subsequent_updates=0 deferrals=0 materializations=275
neighbor_public_work cap=16 pairs=535 anchor_coefficients=14980 anchor_transitions=14445 adjacent_inversions=2140 adjacent_multiplications_outside_exponentiation=77040 max_buffer_pairs=7
neighbor_public_cases=281 cap=16 source_calls=3152 reference_source_calls=3152 route_changes=0 unresolved=0 initializations=275 subsequent_updates=260 deferrals=265 materializations=270
```

Compile/check exit0;80transitive input pins checked; inherited controls pass. Added inversion and outside-exponentiation multiplication counts cover the adjacent helper; existing anchor/source/setup costs remain charged. Removed second-stream transitions equal the recorded anchor transitions, assuming the same lengths/caps. No field-operation or timing ratio. The cumulative-counter observation debug field is not a total and is not used. No new compiler/native/render failure or cloud/timing workload. Historical fullCargo fixture/deadline remains; fullCargo not repeated.
