#set page(margin: 2cm)
#set text(size: 10pt)
= Item 06: signature-based Boolean Groebner engine and trace replay (repair pass)
*Verdict: REGRESSES* (frozen rule applied mechanically by `b06 summarize` over `raw/`, repaired instrument sha256 48a39c34...). Level: constant. Stage diagnostic: no `S`, no rho ratio, no GAE; the gap to matched rho is unchanged (distance remaining). First-run verdict INCONCLUSIVE is superseded (see README.md).

#image("figures/ratio-by-cell.svg", width: 100%)

*Figure.* R = touches(f4) / cost(X), geometric mean by cell, paired by target; recorded values from raw/assess-r2-summarize.stdout.log. sig bars are upper bounds (stopped jobs). Hatched: c15-3-4 is not regress-certain.

== Findings
- `sig` refuted 0 of 64 targets. Stops: c13-3-4, c15-5-3, c17-4-3 all 16 by the 2x touch cap (R <= 0.494, 0.497, 0.492); c15-3-4 mixed (5 touchcap, 4 colscap, 4 timelimit, 3 memcap; R <= 1.677). Three regress-certain cells give REGRESSES; ADVANCES and TRADES are excluded (0 of 4 cells qualify).
- Zero reductions at the first degree-4 matrix fall by 719x, 873x, 480x, 1519x, but touches do not: f4/sig 0.456, 0.403, 10.67, 4.17 at that matrix, and sig never finishes within 2x F4.
- Exact-key replay f4r loses with a fallback on every target: R 0.885, 0.630 (15 paired), 0.741, 0.587. Negative-filter replay f4n (descriptive): 1.120, 1.026 (7 of 15 at 1.5), 1.413, 1.160; never at the 1.5 bar in a cell.
- sigr and sign BLOCKED in every cell (no refuting trace). Arm (e) msolve BLOCKED (not installed).
- Long descriptive run c17-4-3 t0: sig memcap at 29x F4 touches, not refuted (R <= 0.034).

== Controls
Selftest OK (132 systems; r2 smoke test OK); xcheck 8/8 identical to item 05; fallback control f4rx 0.9989 (4/4 fallbacks); old and new instrument identical on 33 of 36 held-out rows (3 time-limit stops differ).

== Deviations and limits
Summarize run by the assess stage (no card-14-r2, no receipt); relay files abbreviated; f4 timelimit on c15-3-4 target 1; f4rs c15-3-4 partly skipped. Toy scale (n = 13 to 17), shared 4-vCPU host (L0, no isolation), unit is matrix entries touched. Not a speed result. See README.md for the complete list.
