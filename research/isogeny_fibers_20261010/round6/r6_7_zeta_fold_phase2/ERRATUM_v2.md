# R6.7 ERRATUM v2: the number of per-curve (c)/(b) intervals that contain 1

Date: 2026-10-31. Task R7.5. Applies to R6.7 as merged in #2217 (base commit `abee07ee73`,
run 20261018T2056Z). **The frozen files (`RESULTS.md`, `report.typ`, `report.pdf`, `ratios.svg`,
all `*.json`/`*.txt`, `summary.json`) are not edited.** This file, `recount_v2.py`,
`recount_v2.txt`, `recount_v2.json`, `plot_erratum_v2.py`, `erratum_v2_cb_containment.svg`,
`erratum_v2.typ` and `erratum_v2.pdf` are new. Labels: *measured* = a field recorded in the
committed JSON; *derived* = a comparison of recorded fields done by `recount_v2.py`.

## 0. Summary

* **Wrong statement.** "12 of 18 per-curve CIs contain 1" for the m = 2 (c)/(b) size-matched
  ratio on the torsion family. The committed `m2_torsion.json` records give **11 of 18
  (unpriced), 9 of 18 (priced as run) and 11 of 18 (priced essential)** (derived; recount
  below). 12 does not occur under any of the three pricings.
* **Not wrong.** The point range 0.542 to 1.127 (unpriced) is exactly reproduced. The claim that
  priced values differ from unpriced by at most 0.024 holds (largest recorded difference of
  points 0.0221, priced as run). Every other count in the file that I re-derived reproduces; one
  numeric bound (0.011) does not (section 4).
* **Verdict.** No verdict changes. The R6.7 verdict (RESULTS.md "Status against the falsifier")
  rests on the m = 3 rows, whose counts are reproduced (8 of 9, 8 of 9, 0 of 9), and on
  (b)/(a) = 1/3 on 30 of 30. The m = 2 (c)/(b) row has no stated verdict in the original; its
  corrected count is a weaker support for "(c)/(b) is consistent with 1" than the original
  wording read, and that is the only way the correction matters (section 3).
* **Other discrepancy found while recounting (outside the requested miscount).** The sentence
  "pricing moves no ratio by more than 0.011 at m = 3" is not reproduced: the largest recorded
  m = 3 shift of a point is 0.0151 (B/A, priced as run). See section 4. Verdicts are unaffected
  (containment counts at m = 3 are identical under all three pricings).

## 1. Recount (script and output)

Script: [`recount_v2.py`](recount_v2.py), run as `python3 -I recount_v2.py > recount_v2.txt` from
this directory (reads `m2_torsion.json`, `m2_prime.json`, `m3_9_10.json`, `m3_11.json`; writes
`recount_v2.json`). A curve counts as "containing 1" when `ci95[0] <= 1.0 <= ci95[1]` for the
recorded field `ratio_c_over_b_size_matched.<pricing>.ci95`. Exact output
([`recount_v2.txt`](recount_v2.txt)):

```text
(c)/(b) size-matched, m=2, torsion, law 1, n curves = 18
curve                        point(unp)          unpriced CI             asrun CI         essential CI
bits9/seed1/p=433                 0.784       [0.707,0.873]        [0.714,0.869]        [0.707,0.872] 
bits9/seed2/p=277                 1.111       [0.999,1.263]*       [1.006,1.263]        [0.998,1.261]*
bits9/seed3/p=421                 0.786       [0.679,0.919]        [0.680,0.909]        [0.679,0.919] 
bits10/seed1/p=919                1.089       [0.902,1.308]*       [0.907,1.286]*       [0.902,1.307]*
bits10/seed2/p=919                1.114       [0.933,1.401]*       [0.935,1.385]*       [0.934,1.401]*
bits10/seed3/p=733                0.542       [0.472,0.636]        [0.479,0.635]        [0.472,0.636] 
bits11/seed1/p=1447               0.809       [0.694,0.946]        [0.706,0.945]        [0.695,0.946] 
bits11/seed2/p=1531               1.127       [0.974,1.351]*       [0.987,1.346]*       [0.974,1.350]*
bits11/seed3/p=1669               0.936       [0.811,1.106]*       [0.816,1.106]*       [0.811,1.106]*
bits12/seed1/p=2647               0.749       [0.660,0.874]        [0.669,0.869]        [0.661,0.875] 
bits12/seed2/p=2557               0.997       [0.860,1.174]*       [0.852,1.154]*       [0.860,1.174]*
bits12/seed3/p=3169               0.899       [0.790,1.040]*       [0.777,1.005]*       [0.789,1.038]*
bits13/seed1/p=6073               0.990       [0.852,1.164]*       [0.859,1.145]*       [0.852,1.164]*
bits13/seed2/p=4639               0.779       [0.673,0.912]        [0.686,0.914]        [0.673,0.912] 
bits13/seed3/p=7933               0.763       [0.660,0.905]        [0.677,0.908]        [0.661,0.905] 
bits14/seed1/p=15349              0.891       [0.772,1.039]*       [0.781,1.032]*       [0.773,1.040]*
bits14/seed2/p=14923              0.861       [0.744,1.010]*       [0.747,1.000]        [0.744,1.007]*
bits14/seed3/p=12391              0.974       [0.827,1.140]*       [0.848,1.155]*       [0.826,1.140]*
(* = CI contains 1)
  unpriced         : 11 of 18 CIs contain 1
  priced_as_run    : 9 of 18 CIs contain 1
  priced_essential : 11 of 18 CIs contain 1
  unpriced         : point range 0.542 to 1.127
  priced_as_run    : point range 0.546 to 1.134
  priced_essential : point range 0.542 to 1.126
  max |priced as run - unpriced| (points): 0.0221
  max |priced essential - unpriced| (points): 0.0018
  curves whose unpriced CI excludes 1: ['bits9/seed1/p=433', 'bits9/seed3/p=421', 'bits10/seed3/p=733', 'bits11/seed1/p=1447', 'bits12/seed1/p=2647', 'bits13/seed2/p=4639', 'bits13/seed3/p=7933']
  curves whose as-run CI excludes 1: ['bits9/seed1/p=433', 'bits9/seed2/p=277', 'bits9/seed3/p=421', 'bits10/seed3/p=733', 'bits11/seed1/p=1447', 'bits12/seed1/p=2647', 'bits13/seed2/p=4639', 'bits13/seed3/p=7933', 'bits14/seed2/p=14923']
  curves whose essential CI excludes 1: ['bits9/seed1/p=433', 'bits9/seed3/p=421', 'bits10/seed3/p=733', 'bits11/seed1/p=1447', 'bits12/seed1/p=2647', 'bits13/seed2/p=4639', 'bits13/seed3/p=7933']

cross-check of the other table rows
  B/A m=3        n= 9 unpriced: 8  priced_as_run: 8  priced_essential: 8
  C/A m=3        n= 9 unpriced: 8  priced_as_run: 8  priced_essential: 8
  B/C m=3        n= 9 unpriced: 0  priced_as_run: 0  priced_essential: 0
  (b)/(a) m=2    n=30 unpriced: 30  priced_as_run: 30  priced_essential: 30

additional checks (statements next to the miscount)
  inversion share of priced-as-run cost: m=3 0.378 to 0.437 (27 arm-curves); m=2 0.360 to 0.510 (arms (a)/(b) on 30 curves, arm (c) on 18 torsion curves)
  m=3 max |priced point - unpriced point|, ratio_B_over_A: 0.0151
  m=3 max |priced point - unpriced point|, ratio_B_over_A_size_matched: 0.0149
  m=3 max |priced point - unpriced point|, ratio_B_over_C: 0.0093
  m=3 max |priced point - unpriced point|, ratio_C_over_A: 0.0028
  m=3 max |priced point - unpriced point|, ratio_C_over_A_size_matched: 0.0025
  m=3 max over all five ratios: 0.0151  (RESULTS.md / report.typ say 'no more than 0.011')
  m=2 max |priced - unpriced| for (c)/(b): 0.0221  (statement: <= 0.024)
```

`*` marks an interval that contains 1. The three counts are 11, 9 and 11 of 18. The three
statements agree with the repository owner's independent count on #2217 as quoted in
RESEARCH_ISOGENY_FIBERS_ECDLP.md section 19.11 (I did not re-read the PR comment itself).

Where the pricings differ (derived, from the table above):

* bits 9 seed 2 (p = 277): unpriced [0.999, 1.263] and essential [0.998, 1.261] contain 1;
  as run [1.006, 1.263] does not (interval entirely above 1).
* bits 14 seed 2 (p = 14923): unpriced [0.744, 1.010] and essential [0.744, 1.007] contain 1;
  as run [0.747, 0.9997] does not (upper end 0.0003 below 1).
* These two are the only curves that change status between pricings. In every other case the
  status is the same under all three. The counts 9 versus 11 therefore come from two borderline
  intervals, not from a systematic shift.
* Direction (derived): the 7 intervals that exclude 1 unpriced (and essential) all lie entirely
  below 1; as run adds one entirely above (bits 9 seed 2) and one below (bits 14 seed 2). Of the
  18 recorded unpriced points, 14 are below 1.

## 2. Statements affected, and the corrected statements

| # | where | original text | corrected text |
|---|---|---|---|
| 1 | `RESULTS.md`, section "m = 2 (LineOracle on both arms)", second bullet | "(c)/(b) size-matched, 18 torsion curves: 0.542 to 1.127; 12 of 18 per-curve CIs contain 1. Priced values differ from unpriced by <= 0.024." | "(c)/(b) size-matched, 18 torsion curves: 0.542 to 1.127; per-curve CIs contain 1 on 11 of 18 (unpriced), 9 of 18 (priced as run) and 11 of 18 (priced essential). Priced values differ from unpriced by <= 0.024." (the last sentence is unchanged and correct: 0.0221) |
| 2 | `report.typ` / `report.pdf`, results table, row "(c)/(b) size-matched, m=2" | "[0.542 to 1.127] ... [12 of 18]" in the column "CIs containing law" | "11 of 18 (unpriced); 9 of 18 priced as run; 11 of 18 priced essential" |
| 3 | `RESEARCH_ISOGENY_FIBERS_ECDLP.md`, section 19.11 "Correction to the source" and the open-obligations line near "12 of 18 ... await their corrections" | already states 11 / 9 / 11 | no change needed; the "awaits correction" line can be dropped in the consolidation, since this erratum is the correction |

Not affected: the table rows B/A, C/A, B/C (m = 3) and (b)/(a) (m = 2); `ratios.svg` (it plots
points and intervals and prints no count, so it is valid as drawn, but it does not make the
containment count visible; the new figure does); the relation-stream paragraph; the limits.

`ratios.svg` was checked and left unchanged. `report.pdf` carries the wrong count in its table
(item 2) and is superseded for that row by `erratum_v2.pdf`; it was not regenerated because the
files are frozen.

## 3. Does any verdict change?

* **Falsifier of R6.7.** Unchanged. It concerns m = 3 (B/A, C/A, B/C) and was reported as "within
  their CIs of the law on 8 of 9 curves each; B/C exceeds 3 by the wrapper's own cost". I
  reproduced 8, 8 and 0 of 9 under all three pricings (section 1, cross-check block).
* **(b)/(a) at m = 2.** Unchanged: 30 of 30 curves contain 1/3 under every pricing.
* **(c)/(b) size-matched at m = 2.** The original gave no separate verdict, so none is
  overturned. The count is lower than stated: 7 of 18 intervals (unpriced) exclude 1, all on the
  low side, and 14 of 18 points are below 1. This is a measured observation about the 18
  curves (3 seeds per size, 1000 targets each). It is not shown here that (c)/(b) differs from
  1 in expectation; it is also not shown to equal 1. Any downstream sentence that reads "12 of 18
  support (c)/(b) = 1" should use 11 / 9 / 11 and mention the low-side direction.

## 4. Other statements re-derived while recounting

Same script, same records (section 1, last blocks):

* Point range of (c)/(b), unpriced: 0.542 to 1.127. Reproduced. (As run 0.546 to 1.134;
  essential 0.542 to 1.126.)
* m = 3 inversion share of priced-as-run cost: 0.378 to 0.437. Reproduced for RESULTS.md ("0.38
  to 0.44"). `report.typ` says "38 to 44 % ... in every arm" without saying m = 3; at m = 2 the
  recorded shares are 0.360 to 0.510. So that sentence is correct for m = 3 only.
* "Pricing moves no ratio by more than 0.011 at m = 3" (`report.typ`; RESULTS.md says "about 40 %
  ... so pricing moves no ratio by more than 0.011"): **not reproduced**. Largest
  |priced - unpriced| over the m = 3 point values is 0.0151 (B/A, as run; 0.0149 size-matched),
  B/C 0.0093, C/A 0.0028, C/A size-matched 0.0025. Corrected statement: "pricing moves no m = 3
  ratio by more than 0.016 (largest: B/A, priced as run, 0.0151); the largest m = 3 shift of a
  size-matched ratio is 0.0149". This is a rounding or transcription discrepancy of about 0.004
  in absolute size. The recorded B/A and C/A interval widths are 0.05 to 0.40 and B/C stays at
  least 0.031 above 3 under every pricing (priced-as-run minimum 3.031, interval width < 0.001),
  and the m = 3 containment counts are identical under all three pricings (B/A 8, 8, 8; C/A
  8, 8, 8; B/C 0, 0, 0). Verdict unchanged.
* m = 2 "<= 0.024": holds (0.0221).

## 5. Files

* `recount_v2.py`, `recount_v2.txt`, `recount_v2.json`: the recount, its output and a machine
  copy of the per-curve table.
* `plot_erratum_v2.py` → `erratum_v2_cb_containment.svg`: per-curve (c)/(b) points and intervals
  in the three pricings; filled marker = interval contains 1; counts in the legend.
* `erratum_v2.typ` → `erratum_v2.pdf`: this erratum with the figure and the corrected table.
* Reading order for the original: `RESULTS.md` (frozen), then this file.

## 6. Ready-to-paste note text for the section 20 consolidation

> **R6.7 erratum (v2, R7.5).** The merged `RESULTS.md` and `report.pdf` of R6.7 state that 12 of
> 18 per-curve (c)/(b) size-matched intervals (m = 2, torsion family) contain 1. Recounting the
> committed `m2_torsion.json` (bootstrap 95% intervals over 1000 targets, 18 curves) gives
> **11 of 18 unpriced, 9 of 18 priced as run and 11 of 18 priced essential**; the repository
> owner's independent count on #2217 agrees. Only two curves (bits 9 seed 2 and bits 14 seed 2)
> change status between pricings, and both have an end of the interval within 0.006 of 1. The
> seven intervals that exclude 1 unpriced all lie below 1, and 14 of the 18 point values are
> below 1 (range 0.542 to 1.127). No verdict changes: the R6.7 falsifier rests on the m = 3
> rows (B/A and C/A contain their law on 8 of 9 curves under every pricing, B/C exceeds 3 by
> about 1.4 % on all 9) and on (b)/(a) = 1/3 on 30 of 30 curves, all of which reproduce. A second
> discrepancy found in the recount: the m = 3 pricing shift of a ratio is at most 0.0151 (B/A,
> priced as run), not 0.011; no containment count depends on it. Use the record counts and cite
> `round6/r6_7_zeta_fold_phase2/ERRATUM_v2.md`; the frozen `RESULTS.md` and `report.pdf` were not
> regenerated. Limits are unchanged: p up to 2^14 at m = 2, 3 seeds per size, Fermat inversion
> price, contended host.
