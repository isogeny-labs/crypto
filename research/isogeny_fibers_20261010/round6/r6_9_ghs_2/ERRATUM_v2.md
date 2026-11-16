# R6.9 ERRATUM v2: the number of relation-rate runs more than 10x off Gaudry's 1/g!

Date: 2026-10-31. Task R7.5. Applies to R6.9 as merged in #2502. **The frozen files
(`RESULTS.md`, `report.typ`, `report.pdf`, both figures, `figure_data.tsv`, `summary.json`,
`relrate.json`, `walk.json`, `plot.py`) are not edited.** New files: this file, `recount_v2.py`,
`recount_v2.txt`, `recount_v2.json`, `plot_erratum_v2.py`, `erratum_v2_ratio_vs_gfact.svg`,
`erratum_v2.typ`, `erratum_v2.pdf`. Labels: *measured* = a field recorded in `relrate.json`;
*derived* = a quotient or count of recorded fields computed by `recount_v2.py`.

## 0. Summary

* **Wrong statement.** "Against Gaudry's heuristic 1/g! the rate is off by more than 10x in
  **6** of 34 runs: q = 4 with g in {6, 7, 8}, and q = 8 with g = 6." The records give
  **7 of 34** (derived): the six q = 4 runs (g = 6, 7, 8, two seeds each: 33.1, 81.8, 97.0,
  356.2, 584.6, 857.8) **plus** q = 8, g = 6, seed 1 (12.1x). The text's own list names the
  q = 8 case but the count omitted it; its own table (`meas / (1/g!)` column) shows the 12.1.
* **Not wrong.** The 34 runs; the range 0.88 to 3.22x against the exact split-divisor fraction
  (0 runs outside a factor 4); 585x and 858x at the smallest weak instance (N = 12,
  (l, n) = (2, 6), g = 8 over F_4); 136 order checks with 0 failures; standard error <= 0.0025.
* **Verdict.** No verdict changes. The falsifier ("relation rate off the model by more than
  10x") fires against 1/g! with 7 runs as it did with 6, and the hypothesis "within 4x" still
  holds against the exact model on all 34 runs. The walk (task 1) is untouched.

## 1. Recount (script and output)

Script: [`recount_v2.py`](recount_v2.py), run as `python3 -I recount_v2.py > recount_v2.txt`
from this directory (reads `relrate.json`; writes `recount_v2.json`). The ratio is
`measured_rate / model_heuristic_inv_gfact`; the script asserts that it equals the recorded
`ratio_measured_over_heuristic` for every run (it does, to the table's one-decimal rounding).
"More than 10x" is `ratio > 10`; no run has `ratio < 0.1`. Exact output
([`recount_v2.txt`](recount_v2.txt)):

```text
runs: 34
  q  g seed  measured       1/g!  meas/(1/g!)  recorded  >10x <0.1x
  4  2    1    0.7992  5.000e-01          1.6       1.6            
  4  2    2    0.8014  5.000e-01          1.6       1.6            
  4  3    1    0.2557  1.667e-01          1.5       1.5            
  4  3    2    0.3389  1.667e-01          2.0       2.0            
  4  4    1    0.3793  4.167e-02          9.1       9.1            
  4  4    2    0.2010  4.167e-02          4.8       4.8            
  4  6    1    0.0459  1.389e-03         33.1      33.1   YES      
  4  6    2    0.1135  1.389e-03         81.8      81.8   YES      
  4  7    1    0.0192  1.984e-04         97.0      97.0   YES      
  4  7    2    0.0707  1.984e-04        356.2     356.2   YES      
  4  8    1    0.0145  2.480e-05        584.6     584.6   YES      
  4  8    2    0.0213  2.480e-05        857.8     857.8   YES      
  8  2    1    0.6348  5.000e-01          1.3       1.3            
  8  2    2    0.6223  5.000e-01          1.2       1.2            
  8  3    1    0.1718  1.667e-01          1.0       1.0            
  8  3    2    0.1587  1.667e-01          1.0       1.0            
  8  4    1    0.1019  4.167e-02          2.4       2.4            
  8  4    2    0.0688  4.167e-02          1.7       1.7            
  8  6    1    0.0168  1.389e-03         12.1      12.1   YES      
  8  6    2    0.0049  1.389e-03          3.5       3.5            
 16  2    1    0.4152  5.000e-01          0.8       0.8            
 16  2    2    0.4766  5.000e-01          1.0       1.0            
 16  3    1    0.2154  1.667e-01          1.3       1.3            
 16  3    2    0.3038  1.667e-01          1.8       1.8            
 16  4    1    0.0238  4.167e-02          0.6       0.6            
 16  4    2    0.0314  4.167e-02          0.8       0.8            
 16  5    1    0.0056  8.333e-03          0.7       0.7            
 16  5    2    0.0070  8.333e-03          0.8       0.8            
 32  2    1    0.5358  5.000e-01          1.1       1.1            
 32  2    2    0.5158  5.000e-01          1.0       1.0            
 32  3    1    0.1746  1.667e-01          1.0       1.0            
 32  3    2    0.2268  1.667e-01          1.4       1.4            
 32  4    1    0.0658  4.167e-02          1.6       1.6            
 32  4    2    0.0466  4.167e-02          1.1       1.1            

runs with meas/(1/g!) > 10 : 7 of 34
   q=4 g=6 seed=1: 33.1x
   q=4 g=6 seed=2: 81.8x
   q=4 g=7 seed=1: 97.0x
   q=4 g=7 seed=2: 356.2x
   q=4 g=8 seed=1: 584.6x
   q=4 g=8 seed=2: 857.8x
   q=8 g=6 seed=1: 12.1x
runs with meas/(1/g!) < 0.1: 0 of 34
by q: {4: 6, 8: 1}
runs in (5,10]x (not counted): [(4, 4, 1, 9.1)]
meas/exact range: 0.88 to 3.22; runs outside [1/4,4]: 0
smallest weak instance (q=4,g=8) ratios: [584.6, 857.8]
range q>=8, g<=4: (0.6, 2.4, 18)
range q>=16 (all g): (0.6, 1.8, 14)
range q=8, g=6: (3.5, 12.1, 2)

neighbouring statements
  order checks / failures: 136 0 (text: 136 checks, 0 failures)
  max binomial standard error: 0.0025 (text: <= 0.0025)
  walk_steps values: [40000]
```

Observations (derived from the block above):

* 7 of 34 runs exceed 10x: q = 4: 6 runs; q = 8: 1 run (g = 6 seed 1, 12.1x). The other q = 8,
  g = 6 run (seed 2) is 3.5x. No q = 16 or q = 32 run exceeds 1.8x.
* The largest ratio not counted is q = 4, g = 4, seed 1 at 9.1x. The count is therefore
  sensitive to a threshold between 9.1x and 12.1x; at 10x it is 7. A reader who uses "at least
  9x" gets 8.
* The smallest weak instance is unchanged (q = 4, g = 8: 584.6x and 857.8x).

## 2. Statements affected, and the corrected statements

| # | where | original text | corrected text |
|---|---|---|---|
| 1 | `RESULTS.md` section 0, "Relation rate", second bullet | "the rate is off by more than 10x in 6 of 34 runs: q = 4 with g in {6, 7, 8}, and q = 8 with g = 6." | "the rate is off by more than 10x in 7 of 34 runs: q = 4 with g in {6, 7, 8} (both seeds, 6 runs, 33x to 858x) and q = 8 with g = 6 (seed 1 only, 12.1x; seed 2 is 3.5x)." |
| 2 | `report.typ` / `report.pdf`, "Verdict", relation-rate bullet | "Against 1/g! it is off by more than 10x in 6 runs." | "... in 7 of 34 runs." |
| 3 | `RESULTS.md` section 3 "Reading", second bullet | "...It fails badly at q = 4 for g >= 6." (the 0.6 to 2.4x range for q >= 8 with g <= 4 and q >= 16 is correct: derived 0.6 to 2.4, 18 runs, and 0.6 to 1.8, 14 runs) | "It fails badly at q = 4 for g >= 6 (6 of 6 runs) and in one of two q = 8, g = 6 runs (12.1x)." |
| 4 | `RESEARCH_ISOGENY_FIBERS_ECDLP.md` (lines about R6.9 near 2463 and in the status table near 2581, as of origin/main on 2026-10-31) | quotes "6 of 34 runs" and already notes "7 in its table" | already flags the discrepancy; the consolidation should state 7 of 34 and cite this erratum. Not edited here. |

Not affected: the verdict heading "the falsifier fires against the 1/g! model at q = 4 and
q = 8" (already names both q); the exact-model statements; the walk section; `fig2` (it plots
the 34 ratios with the 10x line and prints no count, so it is correct as drawn; the 7 points
above the dashed line include the q = 8 one); `figure_data.tsv`; `summary.json`.
`report.pdf` carries the wrong count (item 2) and is superseded for that sentence by
`erratum_v2.pdf`.

## 3. Does any verdict change?

No.

* **Falsifier of R6.9, relation-rate half.** "Off the model by more than 10x" fires against the
  1/g! model. It fired with 6 runs; it fires with 7. The additional run is a q = 8 run, which
  the text already named as part of the falsifier ("q = 4 and q = 8").
* **Hypothesis, relation-rate half.** "Within 4x of the model": holds against the exact
  split-divisor fraction (0.88 to 3.22x, derived, 34 of 34); fails against 1/g! at the
  smallest weak instance. Both unchanged.
* What the correction adds (derived): the 1/g! failure is not confined to q = 4; it reaches
  q = 8 once g = 6 (one run). That is consistent with the text's stated explanation (1/g!
  assumes q much larger than g), but the data here are two seeds per (q, g) and do not test
  that explanation.

## 4. Neighbouring statements re-derived

Last block of the output above: 136 order checks with 0 failures; maximum binomial standard
error 0.0025; all runs use 40 000 walk steps. All reproduce.

## 5. Files

* `recount_v2.py`, `recount_v2.txt`, `recount_v2.json`: the recount, output, machine copy.
* `plot_erratum_v2.py` → `erratum_v2_ratio_vs_gfact.svg`: measured / (1/g!) against g for all 34
  runs, coloured by q, with the 10x line; filled markers exceed 10x; the 7th run is annotated.
* `erratum_v2.typ` → `erratum_v2.pdf`: this erratum with the figure and corrected sentences.

## 6. Ready-to-paste note text for the section 20 consolidation

> **R6.9 erratum (v2, R7.5).** The merged `RESULTS.md` and `report.pdf` of R6.9 say that the
> measured fully-split relation rate is off from Gaudry's 1/g! by more than 10x in 6 of 34
> runs. Recounting `relrate.json` (34 runs, 40 000 walk steps each, q = 4, 8, 16, 32, g = 2 to
> 8) gives **7 of 34**: the six q = 4 runs with g = 6, 7, 8 (33x, 82x, 97x, 356x, 585x, 858x)
> and the q = 8, g = 6 seed-1 run (12.1x); the q = 8, g = 6 seed-2 run is 3.5x, the closest
> uncounted run is q = 4, g = 4 seed 1 at 9.1x, and no q = 16 or 32 run exceeds 1.8x. The
> text's list ("q = 4 with g in {6, 7, 8}, and q = 8 with g = 6") already named the q = 8 case;
> only the count was short. No verdict changes: the falsifier "off the model by more than 10x"
> fires against 1/g! either way, and against the exact split-divisor fraction all 34 runs stay
> within 0.88 to 3.22x. Limits are unchanged: random smooth genus-g curves over F_{2^l} stand in
> for the descended GHS Jacobian (the library cannot construct it for m >= 2), two seeds per
> (q, g), toy fields only. Cite `round6/r6_9_ghs_2/ERRATUM_v2.md`; the frozen `RESULTS.md` and
> `report.pdf` were not regenerated.
