# Native correctness results

Producer assertion; independent signed verdict pending. [pcc:unverified]

```text
split_projection_checks=738 mixed_modes=693 selected_only_modes=45 unresolved_only_modes=0 coefficient_reads=8418 scalar_products=15360 pair_products=14916
split_canonical_states=2516
split_public_cases=281 cap=64 source_calls=2881 reference_source_calls=2881 route_changes=0 unresolved=0 initializations=275 subsequent_updates=0 deferrals=0 materializations=275
split_public_cases=281 cap=16 source_calls=3152 reference_source_calls=3152 route_changes=0 unresolved=0 initializations=275 subsequent_updates=260 deferrals=265 materializations=270
```

Native compilation and checks exit0; 68 input hashes checked. All inherited controls also passed; see complete stdout. Mixed and selected-only projection cases are covered; unresolved-only count is zero in this dedicated panel. Kernel counts include evaluator powers, not field operation totals. No compile/native failure, cloud submission or timing run. Historical fullCargo fixture/deadline limitation remains; fullCargo was not repeated for this standalone native construction.
