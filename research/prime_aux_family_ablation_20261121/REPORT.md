# Prime aux-family ablation (M19, matched `|V|=8`)

Standing: `[pcc:unverified]`. Receipts: `RESULTS.json`. Protocol: score `2A+3B+C` with A = lift rate of aux abscissae onto a prime-order target, B = solving degree / cols vs j1728 baseline (when an isogeny tower runs), C = trace-replay ops speedup. Field `p = 2^{19}-1`, depth `3`, `m = 2`.

## Ranking

| Rank | Family | Lift | Deg | Cols | Ops× | Verdict |
| ---: | --- | ---: | ---: | ---: | ---: | --- |
| 1 | **twist** (quadratic twist of target) | **0.625** | — | — | 1.00 | **CONTINUE_LIFT** |
| 2 | **j1728** `w^2=x^3-3x` (aligned) | 0.500 | 5 | 63 | **1.81** (wall **9.36×**) | CONTINUE_PREP_OR_LIFT |
| 3 | j0 abscissa `y^2=x^3+b` (`#E` with `v3=3`, `v2=2`) | 0.500 | — | — | 1.00 | HOLD_PREP_ONLY |
| 4 | montgomery→Weierstrass abscissa | 0.375 | — | — | 1.00 | HOLD_PREP_ONLY |

## What moved

- **Twist beats j1728 on axis A** at matched `|V|`: 62.5% vs 50% of sampled aux abscissae lift to the target `[pcc:unverified]`. No degree/oracle measurement (abscissa-only arm); next step is a smooth-order twist aux with a Vélu tower so B/C can be scored.
- **j1728 remains the only family with a working isogeny oracle** in this cell: planted hit verified, solving degree 5, trace/inherit/descent **1.81×** ops / **9.36×** wall `[pcc:unverified]` — prep win, consistent with #4820.
- **j0** found a `v3=3` smooth-ish order but only `v2=2`, so depth-3 2-isogeny towers are unavailable; lift ties j1728 at 0.5. Worth a **3-isogeny** follow-up, not another 2-tower.
- **Montgomery** (A=486662 birational) underperformed on lift (0.375) and lacked a cyclic full 2-power generator for a depth-3 tower despite `v2(#E)≥3` in the count — treat as **not competitive** at this scale unless a better A is searched for cyclic 2-primary part.

## Hard rejects (unchanged)

Panel A `γ̂≳3.5` at `m≥3` and Pollard-ρ floor from `research/prime_pkm_aux_chase_20261119` still apply. This ablation does **not** claim a sub-ρ break; it only ranks same-field aux *families* on lift + prep.

## Next falsifiers

1. Build a **twist aux with smooth `#E`** and run the same isogeny oracle as j1728; if lift stays ≥0.6 and `d_F`/cols ≤ j1728, promote twist.
2. Add a **degree-3 Vélu** path for j0 (`v3≥depth`) and re-score B/C.
3. Optional: CM search for mixed-smooth `2^a 3^b` on fixed NIST primes (rubric #8).
