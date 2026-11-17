# Planned interval width — T-1884

Producer observations; independent signed verdict pending. [pcc:unverified]

validation01 passes on the first attempt. Twenty scalar-stream panels compare 760 coefficient deliveries, with four invalid-plan rejections and twelve wrapper/fallback checks. No-rescue snapshots are taken before any doubled stream is created. Values agree in all three arms.

## Scalar-stream accounting

Counts and slot sums across the 20 independent panels:

| Metric | No rescue: old | No rescue: planned | Doubled: old shared | Doubled: planned shared | Doubled: separate native |
|---|---:|---:|---:|---:|---:|
| Polynomial setups | 16 | 16 | 16 | 16 | 32 |
| Progression setups | 16 | 11 | 16 | 16 | 32 |
| Evaluation batches | 5040 | 4869 | 10147 | 9782 | 9953 |
| Convolution calls | 4474 | 4450 | 9330 | 9115 | 9139 |
| Query-tree calls | 562 | 415 | 809 | 659 | 806 |
| Retained live slots | 387 | 334 | 387 | 480 | 867 |
| Retained capacity slots | 454 | 359 | 454 | 532 | 986 |

Planning reduces combined batches by 365 versus old shared width and by 171 versus separate native widths on these panels. Combined shared retained capacity rises by 78 slots (454 to 532). The no-rescue snapshots differ: planned progression setup occurs in only 11 panels versus 16, so aggregate retained capacity there is lower despite a weakly larger width. Preserve the per-panel rows below; do not infer one monotone memory effect from width alone.

## Public recovery

All 2248 target/cap comparisons preserve scalar outcome, route and source-call count. The m=5 results remain 4053,3278,3228,3224,3068,2881 calls across caps 8,10,16,20,32,64. Only cap 8 leaves ten unresolved targets. No retained interval cache is constructed in those short-product panels.

At m=35, both caps 64 and 128 recover all 281 targets with 9486 source calls each, and neither invokes a successful doubled-phase rescue. In each panel both policies have 245 polynomial setups, 245 progression setups and 4655 batches. Planning changes convolution/query-tree counts from 490/4165 to 980/3675. Final retained capacity sums rise from 5390 to 5880 slots; the maximum per-context final capacity rises from 22 to 24 slots. These are retained cache observations, not peak memory, temporary allocations or weighted work.

The public warm-cache control demonstrates changed evaluation mix without fewer batches or queries. A convolution and a query tree need not cost the same, and their sizes differ across widths. No timing or aggregate arithmetic gain is claimed. All source grids, coefficient setup, tails, copies, inversions and group verification remain charged.

## Exact native rows

```
width_panel r=281 m=35 block_cap=1 old_width=1 planned_width=1 no_rescue_old=[0, 0, 0, 0, 0, 0, 0] no_rescue_planned=[0, 0, 0, 0, 0, 0, 0] doubled_old=[0, 0, 0, 0, 0, 0, 0] doubled_planned=[0, 0, 0, 0, 0, 0, 0] doubled_separate=[0, 0, 0, 0, 0, 0, 0]
width_panel r=281 m=35 block_cap=2 old_width=2 planned_width=2 no_rescue_old=[1, 1, 203, 191, 11, 9, 10] no_rescue_planned=[1, 1, 203, 191, 11, 9, 10] doubled_old=[1, 1, 411, 395, 14, 9, 10] doubled_planned=[1, 1, 411, 395, 14, 9, 10] doubled_separate=[2, 2, 411, 395, 14, 18, 20]
width_panel r=281 m=35 block_cap=7 old_width=6 planned_width=7 no_rescue_old=[1, 1, 21, 3, 18, 21, 22] no_rescue_planned=[1, 1, 20, 4, 16, 24, 24] doubled_old=[1, 1, 45, 21, 24, 21, 22] doubled_planned=[1, 1, 38, 16, 22, 24, 24] doubled_separate=[2, 2, 39, 15, 24, 45, 46]
width_panel r=281 m=35 block_cap=16 old_width=6 planned_width=9 no_rescue_old=[1, 1, 21, 3, 18, 21, 22] no_rescue_planned=[1, 0, 9, 0, 9, 10, 10] doubled_old=[1, 1, 45, 21, 24, 21, 22] doubled_planned=[1, 1, 21, 4, 17, 30, 38] doubled_separate=[2, 2, 33, 7, 26, 51, 60]
width_panel r=281 m=35 block_cap=64 old_width=6 planned_width=9 no_rescue_old=[1, 1, 21, 3, 18, 21, 22] no_rescue_planned=[1, 0, 9, 0, 9, 10, 10] doubled_old=[1, 1, 45, 21, 24, 21, 22] doubled_planned=[1, 1, 21, 4, 17, 30, 38] doubled_separate=[2, 2, 33, 7, 26, 51, 60]
width_panel r=1009 m=63 block_cap=1 old_width=1 planned_width=1 no_rescue_old=[0, 0, 0, 0, 0, 0, 0] no_rescue_planned=[0, 0, 0, 0, 0, 0, 0] doubled_old=[0, 0, 0, 0, 0, 0, 0] doubled_planned=[0, 0, 0, 0, 0, 0, 0] doubled_separate=[0, 0, 0, 0, 0, 0, 0]
width_panel r=1009 m=63 block_cap=2 old_width=2 planned_width=2 no_rescue_old=[1, 1, 743, 719, 23, 9, 10] no_rescue_planned=[1, 1, 743, 719, 23, 9, 10] doubled_old=[1, 1, 1497, 1463, 32, 9, 10] doubled_planned=[1, 1, 1497, 1463, 32, 9, 10] doubled_separate=[2, 2, 1497, 1463, 32, 18, 20]
width_panel r=1009 m=63 block_cap=7 old_width=7 planned_width=7 no_rescue_old=[1, 1, 63, 25, 38, 24, 24] no_rescue_planned=[1, 1, 63, 25, 38, 24, 24] doubled_old=[1, 1, 126, 71, 55, 24, 24] doubled_planned=[1, 1, 126, 71, 55, 24, 24] doubled_separate=[2, 2, 126, 71, 55, 48, 48]
width_panel r=1009 m=63 block_cap=16 old_width=8 planned_width=12 no_rescue_old=[1, 1, 44, 4, 40, 27, 34] no_rescue_planned=[1, 0, 21, 0, 21, 13, 13] doubled_old=[1, 1, 91, 32, 59, 27, 34] doubled_planned=[1, 1, 42, 3, 39, 39, 42] doubled_separate=[2, 2, 65, 7, 58, 66, 76]
width_panel r=1009 m=63 block_cap=64 old_width=8 planned_width=12 no_rescue_old=[1, 1, 44, 4, 40, 27, 34] no_rescue_planned=[1, 0, 21, 0, 21, 13, 13] doubled_old=[1, 1, 91, 32, 59, 27, 34] doubled_planned=[1, 1, 42, 3, 39, 39, 42] doubled_separate=[2, 2, 65, 7, 58, 66, 76]
width_panel r=2017 m=63 block_cap=1 old_width=1 planned_width=1 no_rescue_old=[0, 0, 0, 0, 0, 0, 0] no_rescue_planned=[0, 0, 0, 0, 0, 0, 0] doubled_old=[0, 0, 0, 0, 0, 0, 0] doubled_planned=[0, 0, 0, 0, 0, 0, 0] doubled_separate=[0, 0, 0, 0, 0, 0, 0]
width_panel r=2017 m=63 block_cap=2 old_width=2 planned_width=2 no_rescue_old=[1, 1, 1487, 1439, 47, 9, 10] no_rescue_planned=[1, 1, 1487, 1439, 47, 9, 10] doubled_old=[1, 1, 2997, 2927, 68, 9, 10] doubled_planned=[1, 1, 2997, 2927, 68, 9, 10] doubled_separate=[2, 2, 2997, 2927, 68, 18, 20]
width_panel r=2017 m=63 block_cap=7 old_width=7 planned_width=7 no_rescue_old=[1, 1, 126, 47, 79, 24, 24] no_rescue_planned=[1, 1, 126, 47, 79, 24, 24] doubled_old=[1, 1, 250, 132, 118, 24, 24] doubled_planned=[1, 1, 250, 132, 118, 24, 24] doubled_separate=[2, 2, 250, 132, 118, 48, 48]
width_panel r=2017 m=63 block_cap=16 old_width=8 planned_width=12 no_rescue_old=[1, 1, 86, 4, 82, 27, 34] no_rescue_planned=[1, 1, 42, 1, 41, 39, 42] doubled_old=[1, 1, 178, 56, 122, 27, 34] doubled_planned=[1, 1, 84, 6, 78, 39, 42] doubled_separate=[2, 2, 128, 9, 119, 66, 76]
width_panel r=2017 m=63 block_cap=64 old_width=8 planned_width=12 no_rescue_old=[1, 1, 86, 4, 82, 27, 34] no_rescue_planned=[1, 1, 42, 1, 41, 39, 42] doubled_old=[1, 1, 178, 56, 122, 27, 34] doubled_planned=[1, 1, 84, 6, 78, 39, 42] doubled_separate=[2, 2, 128, 9, 119, 66, 76]
width_panel r=2521 m=315 block_cap=1 old_width=1 planned_width=1 no_rescue_old=[0, 0, 0, 0, 0, 0, 0] no_rescue_planned=[0, 0, 0, 0, 0, 0, 0] doubled_old=[0, 0, 0, 0, 0, 0, 0] doubled_planned=[0, 0, 0, 0, 0, 0, 0] doubled_separate=[0, 0, 0, 0, 0, 0, 0]
width_panel r=2521 m=315 block_cap=2 old_width=2 planned_width=2 no_rescue_old=[1, 1, 1883, 1871, 11, 9, 10] no_rescue_planned=[1, 1, 1883, 1871, 11, 9, 10] doubled_old=[1, 1, 3771, 3755, 14, 9, 10] doubled_planned=[1, 1, 3771, 3755, 14, 9, 10] doubled_separate=[2, 2, 3771, 3755, 14, 18, 20]
width_panel r=2521 m=315 block_cap=7 old_width=7 planned_width=7 no_rescue_old=[1, 1, 156, 139, 17, 24, 24] no_rescue_planned=[1, 1, 156, 139, 17, 24, 24] doubled_old=[1, 1, 312, 289, 23, 24, 24] doubled_planned=[1, 1, 312, 289, 23, 24, 24] doubled_separate=[2, 2, 312, 289, 23, 48, 48]
width_panel r=2521 m=315 block_cap=16 old_width=16 planned_width=16 no_rescue_old=[1, 1, 32, 13, 19, 51, 66] no_rescue_planned=[1, 1, 32, 13, 19, 51, 66] doubled_old=[1, 1, 62, 36, 26, 51, 66] doubled_planned=[1, 1, 62, 36, 26, 51, 66] doubled_separate=[2, 2, 62, 36, 26, 102, 132]
width_panel r=2521 m=315 block_cap=64 old_width=18 planned_width=26 no_rescue_old=[1, 1, 24, 5, 19, 57, 74] no_rescue_planned=[1, 0, 12, 0, 12, 27, 27] doubled_old=[1, 1, 48, 23, 25, 57, 74] doubled_planned=[1, 1, 24, 5, 19, 81, 86] doubled_separate=[2, 2, 36, 10, 26, 138, 160]
width_summary panels=20 coefficient_equalities=760 rejected_controls=4 metrics=[polynomial_setups,grid_setups,batches,convolutions,query_trees,live_slots,capacity_slots] no_rescue_old=[16, 16, 5040, 4474, 562, 387, 454] no_rescue_planned=[16, 11, 4869, 4450, 415, 334, 359] doubled_old=[16, 16, 10147, 9330, 809, 387, 454] doubled_planned=[16, 16, 9782, 9115, 659, 480, 532] doubled_separate=[32, 32, 9953, 9139, 806, 867, 986]
width_public cases=281 m=5 cap=8 source_calls=4053 recovered=271 rescue_routes=0 old_metrics=[0, 0, 0, 0, 0, 0, 0] planned_metrics=[0, 0, 0, 0, 0, 0, 0] max_retained_capacity_old=0 max_retained_capacity_planned=0
width_public cases=281 m=5 cap=10 source_calls=3278 recovered=281 rescue_routes=220 old_metrics=[0, 0, 0, 0, 0, 0, 0] planned_metrics=[0, 0, 0, 0, 0, 0, 0] max_retained_capacity_old=0 max_retained_capacity_planned=0
width_public cases=281 m=5 cap=16 source_calls=3228 recovered=281 rescue_routes=220 old_metrics=[0, 0, 0, 0, 0, 0, 0] planned_metrics=[0, 0, 0, 0, 0, 0, 0] max_retained_capacity_old=0 max_retained_capacity_planned=0
width_public cases=281 m=5 cap=20 source_calls=3224 recovered=281 rescue_routes=220 old_metrics=[0, 0, 0, 0, 0, 0, 0] planned_metrics=[0, 0, 0, 0, 0, 0, 0] max_retained_capacity_old=0 max_retained_capacity_planned=0
width_public cases=281 m=5 cap=32 source_calls=3068 recovered=281 rescue_routes=165 old_metrics=[0, 0, 0, 0, 0, 0, 0] planned_metrics=[0, 0, 0, 0, 0, 0, 0] max_retained_capacity_old=0 max_retained_capacity_planned=0
width_public cases=281 m=5 cap=64 source_calls=2881 recovered=281 rescue_routes=0 old_metrics=[0, 0, 0, 0, 0, 0, 0] planned_metrics=[0, 0, 0, 0, 0, 0, 0] max_retained_capacity_old=0 max_retained_capacity_planned=0
width_public cases=281 m=35 cap=64 source_calls=9486 recovered=281 rescue_routes=0 old_metrics=[245, 245, 4655, 490, 4165, 5145, 5390] planned_metrics=[245, 245, 4655, 980, 3675, 5880, 5880] max_retained_capacity_old=22 max_retained_capacity_planned=24
width_public cases=281 m=35 cap=128 source_calls=9486 recovered=281 rescue_routes=0 old_metrics=[245, 245, 4655, 490, 4165, 5145, 5390] planned_metrics=[245, 245, 4655, 980, 3675, 5880, 5880] max_retained_capacity_old=22 max_retained_capacity_planned=24
width_wrapper_checks=12
```

Compiler: rustc 1.99.1 (58cf8f9c3 2026-10-13); native source hashes: 102. PCC-20261107-fb6d9b pins the derivation; independent signed verdict pending. Original cohort unchanged; no compiler/native/render failure or cloud/timing work.
