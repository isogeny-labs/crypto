# D(8,3) decisive verdict (2026-11-13 ~05:55Z), [pcc:unverified]

Cell: m=8, t=3, N=24, p=16801793, seed 1. Reading fixed in AMENDMENTS.md A3 before the run.

## Certified (exact closure, pcclosure3, degree 9, 43.7M columns)
- Levels 3..8 complete, `one_in_W=false` throughout → **1 ∉ W_8**, so **D(8,3) ≥ 9** (certified).
- Level 9 did NOT complete (no level-9 record after 3h+; ~132 GB used), so the "level-9 complete
  without 1 → D ≥ 10" branch is NOT certified. CENSORED at level 9.
- Planted (consistent) control: levels 3..8, one_in_W=false — no spurious 1. Consistent.

## Uncertified evidence (msolve F4, -t8 -g2)
- Completed all degree-9 rounds (last round still 7 new pivots), then **advanced into a degree-10
  round** before crashing at the memory/time cap (core dump; output file 0 bytes).
- Entering degree 10 means the basis was not complete at degree 9 → **solving degree ≥ 10**.
  This is strong but uncertified (no finished basis, no exact max degree).

## Reading under A3
- Clean A3 criteria for D=9 (1∈W_9, or msolve finished at D_F4=9 with basis {1}): NOT met.
- Clean A3 criterion for D≥10 (level-9 closure complete without 1): NOT met (censored).
- **Verdict: INCONCLUSIVE, leaning KILL.** Certified bound D(8,3) ≥ 9; msolve evidence D(8,3) ≥ 10.

## Consequence for m=8 (w=1 cost model)
- D(8,1)=7, D(8,2)=8 (exact), D(8,3) ≥ 9.
- Slope t=2→3, ΔN=8: D(8,3)=9 → σ=0.125 < α*(8,1)=0.156 → SURVIVE (E≈0.43);
  D(8,3) ≥ 10 → σ ≥ 0.25 > 0.156 → KILL.
- So m=8 is decided by whether D(8,3) is exactly 9 or ≥10. Certified data alone is consistent
  with both; the only solver that reached the question (msolve) indicates ≥10, i.e. KILL.
- No breakthrough. m=8 is NOT shown to beat rho; evidence leans to closure. Certifying it needs
  the degree-9 closure (~140 GB+) or a completed large-memory msolve run to finish.

## m4t4 (never-run SURVIVE route)
- Certified 1 ∉ W_6 → D(4,4) ≥ 7; level-7 step did not finish (same wall as earlier runs). CENSORED.
