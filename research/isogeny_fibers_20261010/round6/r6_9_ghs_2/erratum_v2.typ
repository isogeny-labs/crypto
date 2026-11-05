#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
= R6.9 erratum v2: runs more than 10x off Gaudry's 1/g!

Task R7.5, 2026-10-31. Applies to R6.9 as merged in \#2502. The frozen `RESULTS.md`, `report.typ` and `report.pdf` are not edited; this page supersedes their count only. Full text, script and output: `ERRATUM_v2.md`, `recount_v2.py`, `recount_v2.txt`. Toy binary fields only; no claim about deployed curves.

== Wrong statement and its correction
`RESULTS.md` section 0 and `report.pdf` say the measured fully-split relation rate is off from 1/g! by more than 10x in *6 of 34* runs. `relrate.json` gives *7 of 34* (derived: measured rate / (1/g!)):

#table(columns: (auto, auto, auto, auto),
  table.header([*q*], [*g*], [*seed*], [*measured / (1/g!)*]),
  [4], [6], [1, 2], [33.1, 81.8],
  [4], [7], [1, 2], [97.0, 356.2],
  [4], [8], [1, 2], [584.6, 857.8],
  [8], [6], [1], [*12.1* (the omitted run; seed 2 is 3.5)],
)

The original text listed "q = 8 with g = 6" but counted only the six q = 4 runs. The largest uncounted run is q = 4, g = 4, seed 1 at 9.1x; no q = 16 or q = 32 run exceeds 1.8x; no run is below 0.1x.

#figure(image("erratum_v2_ratio_vs_gfact.svg", width: 85%), caption: [Measured smooth rate / (1/g!) for all 34 runs against genus; filled markers exceed 10x (dashed line). Source: `relrate.json` via `plot_erratum_v2.py`.])

== Verdicts
No verdict changes. The falsifier "relation rate off the model by more than 10x" fires against 1/g! with 7 runs as with 6. Against the exact split-divisor fraction all 34 runs stay within 0.88 to 3.22x (reproduced). The smallest weak instance (N = 12, (l, n) = (2, 6), g = 8 over GF(4)) is unchanged at 585x and 858x. The walk results are untouched.

== Limits
Random smooth genus-g curves over GF(2^l) stand in for the descended GHS Jacobian; two seeds per (q, g); the count depends on the 10x threshold (9.1x is the nearest uncounted run). Recount of committed records only; no new run.
