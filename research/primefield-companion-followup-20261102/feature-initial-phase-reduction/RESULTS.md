# Initial phase-domain powering qualification

PCC-20261108-b7a0ed is unsigned producer evidence; independent signed review pending. [pcc:unverified]

Final validation02 output:

```
support_equalities=15264 true_images_retained=7560 wrong_sign_exclusions=7560 zero_exclusions=144 public_target_capacity_cases=2529 route_changes=0 call_changes=0 subsequent_updates=1995 single_call_cap_cases=280 invalid_signs=rejected phase_choices=2_of_4
dickson_equalities=132 powered_initializations=2131 streaming_fallbacks=4160 public_outcomes_routes_stats_equal=true phase_degree_checks=24 exact_capacity_boundaries=12
```

15,264 exact support comparisons cover configurations (17,4),(41,5),(97,3),
(193,3),(281,5),(281,35), both initial signs, capacities0/16/1024 and
radii2/1/3/2. All scalars are used except the m35 case, which uses
0,1,2,3,7,17,280. Materialized polynomials are compared against the seeded
predecessor intersected with Z^L-sign, and retained canonical branch signatures
are compared. True/wrong sign and zero controls remain. [pcc:unverified]

All2,529 public cases recover correctly on E/F257:y²=x³+x+7, P=(1,3),
order281,m5:281 targets times lift caps8/16/64 times section caps0/16/1024.
Outcomes, routes and calls/extra/replayed/lifted/image_count statistics equal
the seeded predecessor.1995 subsequent updates and280 additional one-call
limits execute. These are finite cases on one curve, not independent curve
samples or a performance ranking. [pcc:unverified]

132 Dickson polynomials match a separate linear recurrence for degrees0..32
over four fields.24 squarefree phase degrees match the inverse-orbit formulas.
12 capacity boundaries test both exact acceptance and one-below rejection.
The whole focused workload records2131 powered initializations and4160
streaming fallbacks; these are coverage counts, not timings. [pcc:unverified]

Both revisions compiled and passed focused/inherited checks. Validation01 is
the conservative2mn version; changed source snapshots map its immutable source
hashes to retained copies. Validation02 uses the derived phase-degree bound.
120 copied-source hashes and134 compiler dependencies are bound in the final
receipt. Final inherited stdout is byte-identical. No native compiler/assertion
failure occurred. No cloud or timing panel ran. Original4563-case cohort is
unchanged. Historical full-Cargo missing-fixture limitation remains.

A coordination error is preserved in coordination-error.txt: the first shell
continued after a shared-log scope refusal and appended the planning entry.
The component directory was then explicitly reserved; further shared writes
waited until all shared scopes became available. The PCC record was created
only after reservation succeeded.

Written analysis separates coefficient-scan removal from degree/storage work,
derives a mode0-only worst-fiber bound, and gives unimplemented half-degree
squarefree formulas for the next positive-sign constructor. Those future
formulas are not included as implemented savings. No new Lean refinement proof.
