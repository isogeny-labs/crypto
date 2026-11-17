# R6.7: the ζ-fold of the Gaudry base over F_{p²}, phase two (round 6, 2026-10-18)

Example: `examples/r6_7_zeta_fold_phase2.rs`. Base commit: `base_commit.txt`
(abee07ee73). Run start (UTC): 20261018T2056Z. Commands: `run.sh`, plus
`--exp summarize` → `summary.json`. Raw records: `*.json` with `*.txt` logs.
Graph: `ratios.svg` (`plot_ratios.py`, reads the JSON). PDF: `report.pdf`.
Host: shared Claude Code cloud container, seven runs concurrent, no
isolation; wall times are contended and not used for any claim.

Toy synthetic curves only (p ≈ 2^9 to 2^16). No claim about rho, sub-rho
behaviour or deployed curves.

## What changed from round 4 (lead 16.4)

* The m = 3 arms use the repository `line_oracle::LineOracle` as the
  two-summand step (round 4 used the example's own sweep).
* Every inversion is counted and priced at `I_p` = multiplications of a
  Fermat inversion `a^{p−2}` at that p (`iratio.json`: 12 to 25). Three
  pricings: unpriced, solver as run, solver essential.
* Arm B (T-saturated F_c on E) is run directly on E through a wrapper
  (φ, LineOracle over F'' on E', preimage table, one ⟨T⟩ correction), not
  derived as 3 × the quotient arm.
* The counting copy of the LineOracle solver matched the repository
  solver's multiplication count on every call (`copy_mismatches` = 0 on all
  rows); B lift failures = 0 on all rows.

## Results (measured; per-curve bootstrap 95% CI over targets, 1000 reps)

### m = 3, torsion family, 9 curves (3 seeds × bits 9, 10, 11; 200/200/100 targets)

| ratio | law | measured point, range over curves | CIs containing law |
|---|---|---|---|
| B/A size-matched, unpriced | 3 | 2.976 to 3.277 | 8 of 9 (9-bit seed 1: [3.123, 3.469]) |
| B/A size-matched, priced as run | 3 | 2.969 to 3.267 | 8 of 9 |
| C/A size-matched, unpriced | 1 | 0.980 to 1.075 | 8 of 9 (9-bit seed 1: [1.024, 1.130]) |
| B/C (same curve) | 3 | 3.037 to 3.055, CI width < 0.001 | 0 of 9 |

B/C sits about 1.4 % above 3 on every curve. The interval is narrow
because B and C see the same targets and decisions. The 1.4 % excess is
the wrapper's own charged work (φ on R − P₁ and the lift), which the law
does not count. Inversions are about 40 % of the priced-as-run cost
(`inversion_share_of_priced_as_run` 0.38 to 0.44) in all three arms, so
pricing moves no ratio by more than 0.011.

### m = 2 (LineOracle on both arms)

* (b)/(a): 0.3333 on 30 of 30 curves (18 torsion, 12 prime, p up to
  2^16), under all three pricings (summary: paired shift CI ±2e−16).
* (c)/(b) size-matched, 18 torsion curves: 0.542 to 1.127; 12 of 18
  per-curve CIs contain 1. Priced values differ from unpriced by ≤ 0.024.

### Relation streams (`full_rank_stream`, inversions counted)

* Prime family, 9 curves: both arms reach full achievable rank and recover
  the log on all 9; (a)/(b) relations to full rank 2.79 to 4.97.
* Torsion family, 9 curves: the base is exhausted on all 9 before (a)
  reaches full rank (73/85 to 284/325); (b) reaches full rank on 3 of 9.
  The round-4 stall is reproduced; it is not an artefact of the old sweep.

## Status against the falsifier

Round 4's falsifier was a measured factor far from 3 (outside the sampling
intervals). With the repository oracle and priced inversions, B/A and C/A
are within their CIs of the law on 8 of 9 curves each. B/C exceeds 3 by
the wrapper's own cost (about 1.4 %), outside its narrow CI on every curve.

## Limits

p ≤ 2^11 at m = 3 and ≤ 2^16 at m = 2; 3 seeds per size; contended host;
inversion price is Fermat, not a measured hardware I/M (the contended wall
ratio is recorded in `iratio.json` for reference only).
