# Recorded native controls

Producer assertions, independent signed verdict pending. [pcc:unverified]

```text
reused_mode_equalities=240 independent_power_equalities=1320 reference_component_products=16960 reused_component_products=9136 cold_products_removed=2416 warm_products_removed=5408 mode_cache_max_entries=13 mode_cache_max_coefficient_slots=134 scale_resets=80 support_shapes=4
reused_canonical_states=2516
reused_public_cases=281 cap=64 source_calls=2881 reference_source_calls=2881 route_changes=0 unresolved=0 initializations=275 subsequent_updates=0 deferrals=0 materializations=275
reused_public_cases=281 cap=16 source_calls=3152 reference_source_calls=3152 route_changes=0 unresolved=0 initializations=275 subsequent_updates=260 deferrals=265 materializations=270
```

validation02 compile/check exit0;72transitive input pins checked. All inherited controls pass. Mode-cache entry/slot maxima exclude the arbitrary-power control. Product totals sum actual scalar/pair component calls, including all power construction; empty components contribute zero. No field-operation or timing ratio.

validation01 compile0/check101: EmptyTree in the empty-support checker fixture. Snapshot and receipt preserved, all pins reconciled using the original checker snapshot. Production evaluator unchanged for validation02. No cloud/timing trial; fullCargo not repeated because the historical fixture/deadline issue remains.
