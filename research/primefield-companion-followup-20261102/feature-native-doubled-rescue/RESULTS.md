# Conditional doubled-order rescue — T-1879

Producer observations; independent signed verdict pending. [pcc:unverified]

validation01 passed compilation and native execution on the first attempt, including inherited checks. Seven configurations (r,m): (41,5), (73,3), (97,3), (193,3), (241,3), (241,5), (281,5). The 10503 scalar-mode values cover every scalar and radii 1,2,3. The 5103 arbitrary sign vectors are 729 per configuration: exhaustive for m=3, partial for m=5. The wrong-sign control has 4050 mismatches; 21 coefficient sections and 10 invalid inputs are checked.

## Public panels

| Cap | Direct calls | Rescue calls | Saved calls | Recovered, each | Rescue contexts | Phase candidates |
|---|---:|---:|---:|---:|---:|---:|
| 8 | 4053 | 4053 | 0 | 271/281 | 0 | 0 |
| 10 | 3603 | 3278 | 325 | 281/281 | 260 | 220 |
| 16 | 3378 | 3228 | 150 | 281/281 | 260 | 220 |
| 20 | 3374 | 3224 | 150 | 281/281 | 260 | 220 |
| 32 | 3068 | 3068 | 0 | 281/281 | 205 | 165 |
| 64 | 2881 | 2881 | 0 | 281/281 | 0 | 0 |

These are exact logical source-call counts on fixed fixtures, not timings. All 1686 target/cap executions assert no per-target call increase and preservation of every reference success. The cap-8 panel retains 10 unresolved targets on both paths. At cap 64 the helper is never invoked, so all original early recoveries remain intact. Fifteen nonopposite-schedule fixtures preserve the reference scalar, route and call count.

## Added work and misses

At caps 10,16,20: 260 extra coefficient contexts and 2080 upper F0 deliveries per panel. At cap 32: 205 contexts and 1640 deliveries. These deliveries are only the initial upper F0 stream, excluding F1/F2 and setup. At cap 8, all 265 attempts reject before constructing a context. At caps 10/16, 40 attempts miss the materialization cap; at caps 20/32, 40 are nonsingletons. No zero denominator is observed on these public fixtures, so that branch is not claimed covered by this panel. Retained reference state makes each miss fall back without querying again.

Additional costs include eta/exact-order validation, separate product setup, folded arrays, polynomial contexts, phase evaluation and verification. Therefore source-query reduction alone does not establish total-work or elapsed-time improvement. No new source grid is built for the rescue; all original source setup remains charged.

```
doubled_mode_values=10503 reindexed_sign_vectors=5103 wrong_sign_controls=4050 coefficient_sections=21 invalid_rejections=10
doubled_public_cases=281 cap=8 source_calls=4053 reference_source_calls=4053 recovered=271 reference_recovered=271 route_changes=0 preserved_early_exits=6 targets_fewer_calls=0 attempts=265 contexts=0 cap_misses=265 non_singletons=0 zero_denominators=0 phase_candidates=0 upper_f0_reads=0
doubled_public_cases=281 cap=10 source_calls=3278 reference_source_calls=3603 recovered=281 reference_recovered=281 route_changes=220 preserved_early_exits=6 targets_fewer_calls=60 attempts=260 contexts=260 cap_misses=40 non_singletons=0 zero_denominators=0 phase_candidates=220 upper_f0_reads=2080
doubled_public_cases=281 cap=16 source_calls=3228 reference_source_calls=3378 recovered=281 reference_recovered=281 route_changes=220 preserved_early_exits=6 targets_fewer_calls=30 attempts=260 contexts=260 cap_misses=40 non_singletons=0 zero_denominators=0 phase_candidates=220 upper_f0_reads=2080
doubled_public_cases=281 cap=20 source_calls=3224 reference_source_calls=3374 recovered=281 reference_recovered=281 route_changes=220 preserved_early_exits=6 targets_fewer_calls=30 attempts=260 contexts=260 cap_misses=0 non_singletons=40 zero_denominators=0 phase_candidates=220 upper_f0_reads=2080
doubled_public_cases=281 cap=32 source_calls=3068 reference_source_calls=3068 recovered=281 reference_recovered=281 route_changes=165 preserved_early_exits=6 targets_fewer_calls=0 attempts=205 contexts=205 cap_misses=0 non_singletons=40 zero_denominators=0 phase_candidates=165 upper_f0_reads=1640
doubled_public_cases=281 cap=64 source_calls=2881 reference_source_calls=2881 recovered=281 reference_recovered=281 route_changes=0 preserved_early_exits=6 targets_fewer_calls=0 attempts=0 contexts=0 cap_misses=0 non_singletons=0 zero_denominators=0 phase_candidates=0 upper_f0_reads=0
doubled_nonopposite_fallbacks=15
```

Compiler: rustc 1.99.1 (58cf8f9c3 2026-10-13). Native dependencies: 94. PCC-20261107-67816b pins DERIVATION.md; independent signed verdict pending. Original cohort unchanged; no cloud/timing work.
