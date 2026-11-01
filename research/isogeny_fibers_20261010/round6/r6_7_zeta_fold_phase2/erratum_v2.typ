#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
= R6.7 erratum v2: per-curve (c)/(b) intervals that contain 1

Task R7.5, 2026-10-31. Applies to R6.7 as merged in \#2217 (base commit abee07ee73). The frozen `RESULTS.md`, `report.typ` and `report.pdf` are not edited; this page supersedes their (c)/(b) count only. Full text, script and output: `ERRATUM_v2.md`, `recount_v2.py`, `recount_v2.txt`. Toy synthetic curves only; no claim about rho or deployed curves.

== Wrong statement and its correction
`RESULTS.md` (section "m = 2") and the results table of `report.pdf` say that *12 of 18* per-curve bootstrap 95% intervals of the m = 2 (c)/(b) size-matched ratio (torsion family, 18 curves, 1000 targets each) contain the law value 1. The committed `m2_torsion.json` records give (derived by `recount_v2.py`):

#table(columns: (auto, auto, auto),
  table.header([*pricing*], [*intervals containing 1*], [*point range*]),
  [unpriced], [*11 of 18*], [0.542 to 1.127],
  [priced as run], [*9 of 18*], [0.546 to 1.134],
  [priced essential], [*11 of 18*], [0.542 to 1.126],
)

The count 12 does not occur under any pricing. Only two curves change status between pricings (bits 9 seed 2 and bits 14 seed 2), and both have an interval end within 0.006 of 1. The seven intervals that exclude 1 unpriced all lie below 1; 14 of the 18 unpriced points are below 1.

#figure(image("erratum_v2_cb_containment.svg", width: 100%), caption: [Recorded (c)/(b) size-matched points and bootstrap 95% intervals, 18 torsion curves, m = 2, three pricings. Filled marker: interval contains 1; hollow: excludes 1. Source: `m2_torsion.json` via `plot_erratum_v2.py`.])

== Verdicts
No verdict changes. The R6.7 falsifier rests on m = 3 (B/A 8 of 9, C/A 8 of 9, B/C 0 of 9; identical under all three pricings, reproduced) and on (b)/(a) = 1/3 on 30 of 30 curves (reproduced). The m = 2 (c)/(b) row had no stated verdict; the corrected count is weaker support for "(c)/(b) consistent with 1" than 12 of 18 read.

== Second discrepancy found
"Pricing moves no ratio by more than 0.011 at m = 3" is not reproduced: the largest recorded shift of a point is 0.0151 (B/A, priced as run). The m = 2 bound 0.024 holds (0.0221). No containment count at m = 3 depends on the pricing.

== Limits
p up to 2^14 at m = 2; 3 seeds per size; Fermat inversion price, not a hardware I/M; contended host. Everything above is a recount of committed records; no new run.
