# Native shared traversal controls

Producer assertion, independent signed verdict pending. [pcc:unverified]

```text
shared_mode_groups=225 branch_value_equalities=450 streams=180 reader_coefficients=6540 separate_streams=450 separate_reader_coefficients=16350 max_branches=4 max_cache_coefficient_slots=346 max_returned_coefficient_slots=34 invalid_input_rejections=15
shared_canonical_states=2516
shared_public_streams cap=64 streams=550 reader_coefficients=15400 equivalent_separate_streams=2140 equivalent_separate_reader_coefficients=59920 max_branches=4 max_cache_coefficient_slots=89
shared_public_cases=281 cap=64 source_calls=2881 reference_source_calls=2881 route_changes=0 unresolved=0 initializations=275 subsequent_updates=0 deferrals=0 materializations=275
shared_public_streams cap=16 streams=1330 reader_coefficients=37500 equivalent_separate_streams=4280 equivalent_separate_reader_coefficients=120800 max_branches=4 max_cache_coefficient_slots=89
shared_public_cases=281 cap=16 source_calls=3152 reference_source_calls=3152 route_changes=0 unresolved=0 initializations=275 subsequent_updates=260 deferrals=265 materializations=270
```

Compile/check exit0;76transitive input pins checked. All inherited controls pass. Reader counts cover completed shared-mode calls and their equivalent separate evaluations on the same branch snapshots. They exclude initial support and later extraction streams. Cache/returned slot maxima are live coefficient lengths, not peak allocations. No compiler/native/render failure or cloud/timing workload. Historical fullCargo fixture/deadline limitation persists; fullCargo not repeated.
