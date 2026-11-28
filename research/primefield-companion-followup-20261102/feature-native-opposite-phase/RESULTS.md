# Native correctness results — T-1878

Producer observations; independent signed verdict pending. [pcc:unverified]

validation01 passed on its first compiler/native attempt. The subsequent-update counter was wired to the old adapter counter; it is not evidence of zero subsequent work. Its three changed source files are retained. validation02 corrects instrumentation and adds 28 rejected-domain controls and 10 public fallback comparisons. Neither attempt failed compilation or native checks. Both include inherited tests.

## Separate deterministic panels

Seven (r,m) configurations: (41,5), (73,3), (97,3), (193,3), (241,3), (241,5), (281,5). For r <= 73, all c+/c- pairs and all four phases; for larger fields, diagonal pairs and actual scalar-polynomial values. Each support equals a direct gcd reference, is squarefree, and has degree <= n/2 for unequal observations. Complete candidate states compare initialization after two radii and updates at 2 and 3, including zero handling.

```
opposite_support_equalities=36464 equal_observation_controls=4984 unequal_observation_controls=31480 empty_support_controls=35657 candidate_states=3501
opposite_public_cases=281 cap=64 source_calls=3122 reference_source_calls=2881 route_changes=255 unresolved=0 initializations=270 subsequent_updates=0 deferrals=0 materializations=270
opposite_public_cases=281 cap=16 source_calls=3376 reference_source_calls=3378 route_changes=10 unresolved=0 initializations=270 subsequent_updates=65 deferrals=70 materializations=265
```

Both public panels recover all 281 targets. Cap 64 costs 241 additional calls (3122 versus 2881); cap 16 saves 2 calls (3376 versus 3378). These are exact logical source-call totals for these fixtures, not timing or total arithmetic measurements. The baseline uses the same opposite-radius schedule. Early endpoint exits are retained. Fallback on a nonopposite schedule agrees on result, source calls and route for k in {0,1,17,140,280} at both caps.

The first gcd domain degree is n/2 by construction. Initial folding remains degree-dependent, and lifting may reach n. No claim about peak memory or elapsed time is supported by these checks. Full source grids, initial observations, source calls and final point verification remain charged. Original 4563-case cohort is unchanged.

Replay: README.md and validate-recovery.py. Compiler: rustc 1.99.1 (58cf8f9c3 2026-10-13). validation02 records 91 native dependency hashes. No cloud jobs or timed workloads were launched. PCC-20261107-6848bb pins the derivation; its independent signed verdict is pending.
