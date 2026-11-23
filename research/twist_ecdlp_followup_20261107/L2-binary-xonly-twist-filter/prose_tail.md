
## J. Extrapolation (labelled; no data-like numbers beyond the registered model table)

Not measured. Values below are the registered model (`frozen_pre_data/prediction_table.txt`), not results.
* Lemma B is size independent (E[2] = {O, T0}); the waste fraction of non-degenerate solutions is predicted to stay 0 and the dead-column density 1/2 at ECC2K-130 (k = 131).
* Enumeration-instrument model ratios at k = 131: R2 = 1.266 (l = 66) and R3 = 1.625 (l = 44). Moot: enumeration needs 2^66 or 2^88 attempts per target.
* Groebner / Weil-descent solvers: the effect of restricting x_i to V_E is not modelled and is open (V_E is not an F_2-subspace; the trace condition adds lift variables). Koblitz orbit structure: no separate prediction.

## K. Limits and open items

* O1: the repository baseline may already filter non-lifting x-values; then the F-over-U gain is a relabelling and not an improvement.
* O2: the enumeration solver is Theta(q) in total; the gain is a constant factor on that phase. O3: Weil descent, m >= 4, Koblitz orbits not measured.
* Extrapolated IC rows (k = 29, 31 and m = 3, k = 29) use measured per-relation cost times N_rel and a formula model for linear algebra; they are not measured full solves.
* The registered stop rule (full column rank) failed for every m = 2 instance and for part of the m = 3 instances at k = 19, 23 (section H table "Complete-solve certificate cells"); the d-determined stop rule is exploratory.
* One prime cell (m = 2, k = 31) fails the stricter known-sd variant of the call-ratio check although the ratio is 1.993 against 2.0 and it passes the declared rule (section E); see D-08.
* Rho at k = 17, 19 is flagged by F8. Wall times (contention-affected: the host load average was above 8) are secondary and not used for any conclusion.
* No PDF of this report: no LaTeX toolchain is present; `figs/RESULTS_figures.pdf` contains the four figures (matplotlib).

## L. Replication

`VERIFICATION.json` holds (i) a same-seed `./run.sh quick` replay compared column by column with this run (all 19 TSVs of the subset are identical
apart from the contention-affected wall time, and the sample files and curve list are byte-identical), (ii) an independent replay with seed offset
`L2SEED_XOR=777` (different curves, factor bases, targets and DLP instances; k <= 19 binary and prime arms, all controls, rho k <= 23, full solves k <= 19)
with its mechanical verdict rows: F1, F2, F3, F4, F5, F7, F9 hold; F8 flags k = 17 and 19 as in the main run; F6 reports one prime cell (m = 2, k = 19) outside the
declared call-ratio band (mean z = -0.51 with the sd estimated from 10 curves; it passes the known-sd variant), while the structural part of F6 (zero non-degenerate prime waste) holds.
Under the null, 17 cells at alpha = 0.01 give a few percent chance of such a fluctuation; it is reported, not reconciled. (iii) The GP cross-check summary (`results/gp_verify.txt`).
