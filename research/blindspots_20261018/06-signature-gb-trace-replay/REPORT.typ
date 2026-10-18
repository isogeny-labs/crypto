#set page(margin: 2cm)
#set text(size: 10pt)
= Item 06: signature-based Boolean Groebner engine and trace replay
*Verdict: INCONCLUSIVE* (frozen rule applied mechanically by `b06 summarize`). Level: constant. Stage diagnostic: no `S`, no rho ratio, no GAE; the gap to matched rho is unchanged (distance remaining).

#image("figures/ratio-by-cell.svg", width: 100%)

*Figure.* R = touches(f4) / cost(X), geometric mean by cell, paired by target; recorded values from raw/card-14-summarize.stdout.log. sig bars are upper bounds (stopped jobs).

== Findings
- `sig` refuted 0 of the 56 targets that ran (of 64 planned) within its stop rules (2x F4 touches, time, column, memory). It qualifies on 0 of 4 cells; ADVANCES and TRADES are excluded.
- REGRESSES not reached: certain on c13-3-4 (R <= 0.494) and c17-4-3 (R <= 0.492); c15-5-3 has 8 of 16 targets unrun (R <= 0.498 on those run); c15-3-4 bound is 1.677 (cap mix).
- Zero reductions at the first degree-4 matrix fall by 719x, 873x, 484x, 1519x (c13-3-4, c15-3-4, c15-5-3, c17-4-3), but touches do not: f4/sig 0.456, 0.403, 10.68, 4.166 at that matrix, and sig never finishes within 2x F4.
- Exact-key replay f4r loses: gmean R 0.885, 0.727 (8/16), 0.587, with a fallback on every target. Negative-filter replay f4n (descriptive): 1.120, 1.405 (8/16), 1.160; never at the 1.5 bar.
- sigr and sign are BLOCKED in every cell (no refuting trace). Arm (e) msolve BLOCKED (not installed).

== Controls
Selftest OK (132 systems); xcheck 8/8 identical to item 05; fallback control f4rx ratio 0.9989 (4/4 fallbacks).

== Deviations and limits
Cards 04, 05, 08, 12 did not complete (no receipt); 05 and 12 produced no measured rows; card 03 command 5 died with a 3 GiB allocation failure under the 4 GiB limit. Toy scale (n = 13 to 17), shared 4-vCPU host, unit is matrix entries touched. See README.md for the complete list.
