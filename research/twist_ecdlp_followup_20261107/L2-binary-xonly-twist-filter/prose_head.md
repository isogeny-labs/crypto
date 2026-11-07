# RESULTS: L2-binary-xonly-twist-filter

Frozen protocol: `PREREGISTRATION.md` (unchanged; `FROZEN.sha256`). Deviations: `DEVIATIONS.md` (read it with this file: rho baseline
changes D-01/D-02, k = 29 m = 3 full solve not run D-03, stop-rule handling D-04). Driver: `run.sh`; raw per-curve counters:
`results/*.tsv`; GP cross-check output: `results/gp_verify.txt`; machine-readable summary: `results.json`; figures: `figs/`.
Every number in sections A-I below is produced by the `analyze` subcommand from `results/*.tsv` (code output, not typed); the prose
only points at them.

## Summary (what the data say; numbers are in the tables below)

* **What was tested.** In x-only relation search over binary curves E: y^2 + xy = x^3 + x^2 + b (k odd, prime n, #E = 2n), a solver that
  enumerates Semaev S_3 solutions over a factor base V (an F_2-subspace) wastes work on factor-base elements that are twist-type
  (no point of E over F_q has that x). The **filter** arm F restricts the enumerated x-values to the E-type part V_E of V. It is compared with
  the same solver without a filter (arm U), and with the solution-side check (arm S). This is a comparison inside this instrument:
  whether the repository's index-calculus baseline already restricts to V_E was **not checked** (open item O1), so a gain here
  is "vs the unfiltered arm of this instrument", not a claim against the repository baseline, and may be a relabelling if the baseline already filters.
* **H1 (exact zero waste) was not falsified.** Over all 9 binary cells the number of non-degenerate solutions with a twist-type coordinate is
  0 (rule-of-three style 95% upper bounds per cell in section A), against 0.75 / 0.875 for the independent-lift model; the 8 prime cells give 0 as well (section E); the prime call ratios match 2 and 4 under the declared rule (one cell fails the stricter variant, see section K).
  The only solutions with a twist coordinate are the degenerate family D (flagged targets). m = 3, k = 29 is **underpowered** (fewer than 1000 unique solutions).
* **H2** (about half the columns are dead): 0 of 9 cells reject, pooled sd of z within [0.8, 1.25] (section B).
* **H3** (counted cost): the measured native-operation ratio U/F is within +-10% of the registered table in all 9 cells (about 1.31-1.33 for m = 2, 1.67-1.75 for m = 3),
  with cluster-bootstrap CIs excluding 1 (section C). Ranking across the 0 / 0.1 / 1 GAE band: F < U in every cell, **weak** at c = 0 because both arms have
  identical group-operation counts (tie at c = 0). The solver-call ratios are about 2 (m = 2) and 4 (m = 3).
* **H4**: the solution-side filter S avoids no lifts (nothing to avoid) and costs slightly more than U (section C, S/U column), so U < S.
* **Instrument validity (F7)**: PC-A, PC-B (mirror lemma: every twist-target solution is all-twist), PC-C, PC-D (exact count |V_tw|) and the GP cross-check all pass (section F). Results are therefore reportable.
* **F8 fails at k = 17 and 19** (rho mean G is outside [0.8, 1.6] x sqrt(pi n)/2 because of fixed per-instance table setup that dominates tiny instances).
  At those sizes no ranking against rho is reported. At k >= 23 rho is within the band.
* **Distance remaining to matched rho (the success criterion, reported as distance, not pass/fail).** The enumeration solver is Theta(q); end-to-end cost is
  far above rho at c >= 0.1 in every cell (section H gap columns: measured complete-solve rows are about 10^3 to 10^5 times rho at c = 1 for k = 17-23;
  extrapolated rows reach 10^6-10^7 at k = 29-31). At c = 0 (group operations only) the measured gaps are about 2-4 (rho cheaper); the m = 3, k = 29
  extrapolation has fewer group operations than rho at c = 0 but is unordered across the band, and it is not a ranked win. The filter lowers each IC row
  by a constant factor of about 1.3 (m = 2) to 1.7 (m = 3) in the relation phase only; it does not change the exponent.
