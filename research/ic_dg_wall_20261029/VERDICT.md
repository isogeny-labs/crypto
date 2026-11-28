# XP-20261029-dg-wall: verdict (§6.2)

Frozen protocol: `backlog/XP-20261029-dg-wall/PROTOCOL.md` (commit 6e4804417baa). Every number below is copied from the tool-written files named in each row; this file computes nothing.

| b | wall-session §6.1 (1–8) | `wall.corrected_ratio` dg / srenew | `wall.corrected_ci95` | §6.2 reading | source |
|--:|:--|:--|:--|:--|:--|
| 24 | hold | 0.9128285470868935 | [0.9099393251078824, 0.9157269428683921] | improvement | `P24/wall/analysis/compare-srenew__dg-w0.json`, `wall_gate.json` |
| 28 | hold | 0.8861982635132132 | [0.8836721572316844, 0.8887315910395139] | improvement | `P28/wall/analysis/compare-srenew__dg-w0.json`, `wall_gate.json` |
| 32 | hold | 0.8485850634943188 | [0.8450922979447673, 0.8520922646400932] | improvement | `P32/wall/analysis/compare-srenew__dg-w0.json`, `wall_gate.json` |

Counted sessions: `EXIT42_CLAUSE=true` at 24, 28 and 32 bits (`P<b>/counted/analysis/counted_gate-stdout.txt`), so the matched rho is available at every size.

## 28-bit consequence (§6.2, conditions 1–4, wall session)

    E6_COND2_ALL_CURVES=false
    E6_COND2_ALL_CURVES_ROW_RATIO_ONLY=false
    E6_COND2_ALL_CURVES_RECOMPUTED_ONLY=false
    E6_COND3_ALL_CURVES=true
    E6_COND4_ALL_CURVES=true
    E6_COND1_W_ALL_1_TO_8=true
    E6_COND1_CORRECTED_CI95_UPPER_LT_1=true
    E6_COND1=true
    E6_ALL_FOUR=false

Condition 2 fails on both curves (per-curve `ratio_b_over_a` at w = 0 is above 1; `cond2_recomputed_sum_total_gae` dg exceeds srenew on each curve), so **`srenew` stays the 28-bit baseline** and the w = 0 tie stays unresolved in counted units, as §6.2 fixes.
