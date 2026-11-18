# DEVIATIONS from PREREGISTRATION.md (the preregistration file itself is unedited; sha256 in PREREG_SHA256.txt)

Recorded before the confirmatory run unless a line says otherwise. Each entry: what, why, effect on the preregistered predictions/analysis.

## Before the code freeze (found while writing the code and running toy smoke tests; none of the smoke data is read by the analysis)

D1. Transfer map formula (section 1.2). The preregistration states u(X') = X'^2 + c5^2/d. That is the correct divisor only when rho = 0, because the two points
    x_1 = -x_2 give X'_1 + X'_2 = -2 rho c5 / d, not 0. The code uses u(X') = X'^2 + (2 rho c5 / d) X' + c5^2 / d, v = (y0/c5) X'^3 mod u, and then the
    translation X'' = X' + sigma (sigma = coefficient of X'^4 over 5) that removes the X'^4 term of the quintic, so the Jacobian formulas can assume f4 = 0.
    Verified by the transfer checks (T(P1+P2) = T(P1)+T(P2), l T(G) = 0, u | f - v^2) on every curve of the run, not assumed. Effect: none on predictions.
D2. R_J selector (section 1.2 arm R_J says "the same walk"). The R_E selector uses x only. For R_J the code uses the coefficient u1 of the Mumford polynomial
    (negation invariant) for the selector and the DP test, not u0. Reason (toy smoke test on k = 8): for every curve of this gluing family the constant
    coefficient u0 is the same for all elements of T(E(F_q)) (c5 = 2 sigma rho identically), so a u0-selector sees one table index only and rho on the
    transferred subgroup degenerates. The IC walk selector is the preregistered "constant coefficient of u mod 16" and is not affected (the walk element carries a
    random component in J' = l J, so u0 is not constant there). Effect: R_J is a working rho; without the change the arm R_J would have been a sabotaged baseline.
D3. IC walk duplicate handling (section 3 "Duplicate rows (same D seen twice) are dropped and counted"). Because the walk is a function of D only, a repeated D means
    the walk is in a cycle and every further step would be a duplicate. The code drops the duplicate row, counts it (ndup), and restarts the walk from a fresh start
    element (a_0 G~ + b_0 H~ + R_0 with new random exponents); the restart costs Jops that are counted (nrestart, and inside the total Jops). The preregistration does
    not say what happens after a duplicate; this is the implementation detail it left open.
D4. Structured elimination (section 3). Weight-1 columns are deleted, weight-2 columns combined, then a Markowitz-style stage. The Markowitz stage is run in rounds
    (columns sorted by current weight once per round, large columns first for IC-D; a column is pivoted if its weight grew by at most 1 since the sort) instead of an
    exact minimum-weight pick after every pivot, because an exact pick scanned all columns per pivot and was too slow in GP at k = 16. The attempt stops at the first row with
    only the x column and a nonzero coefficient (success is independent of the elimination order; only the counted row operations depend on it). Row operations, row
    deletions and nonzero-entry updates are counted as preregistered. Effect: LAROW counts are those of this variant.
D5. Weight-1 relations (deg u = 1) are accepted by IC-S/IC-F only when the vertex is a factor-base vertex, by IC-G/IC-D always. Preregistration is silent; probability about 1/q per trial.
D6. Reference Cantor (section 1.1(e)). The reference is a PARI-polynomial Cantor (bezout, polynomial division). The same code is the counted fallback for the rare special cases
    (equal u, shared roots, degree < 2, vanishing resultants). Hence agreement of the explicit formulas with the reference is a real check on the explicit-formula
    (non-fallback) cases, and is only a consistency check on fallback cases. In addition the run checks associativity on 20 triples per curve, N*D = 0 on 8 elements per
    curve, validity (u | f - v^2) of every checked sum, and (D + D') - D' = D. The reference's operations are never counted. 334 pairs per curve (2004 per size) rather than 2000 per size.
D7. curve_setup (section 1.2). The gluing construction (interpolation, rho, c5, model change, quintic shift) is done with PARI polynomial arithmetic and is not field-operation
    instrumented. Only the number of root-finding calls is recorded (field `rc` in the CURVE lines). The sensitivity column "total incl. amortised setup" is therefore not produced.
    Effect: per-solve totals are unaffected (the preregistration excludes setup from them).
D8. NC-A population (section 6). NC-A uses its own random draws of genus-2 curves (N with largest prime factor >= q^2/32, tag +3 seeds), not the PC-C curves plus other draws.
    Reason: PC-C needs N = h' l'' with h' <= 4, a stricter condition; keeping the two populations separate avoids reuse of curves across controls. 5 curves x 4 instances per size.
D9. NC-A "CI excludes [0.85, 1.15]" is ambiguous. analyze.gp reports both readings (CI disjoint from the interval = literal; CI not contained in the interval = strict) and the H5 test
    (CI contained in [0.9, 1.1]). Neither reading changes a number.
D10. Per-size wall cap (section 6): implemented as a timeout of 2700 s per cell, two cells (curve halves) per size, so at most 90 core-minutes per size. A cell that times out leaves its completed
    instances; the size is flagged incomplete and larger sizes are excluded from the analysis by run.sh. Control cells have the same timeout.
D11. Pairing for the R_E/R_J/IC comparisons is by (curve, instance); a missing arm line drops that instance from the pair (reported as n).
D12. R statistic (H3): the mean over all instances of steps/sqrt(l/2) with censored (cap-hit) instances at their cap is the primary one; the mean over verified solves only is also printed.
     The preregistration bands for R apply for k >= 10 and pooled; smaller sizes are printed but are not judged.
D13. kappa_J: pooled over all IC runs of a size, explicit-formula Jops only (Jops - fallback Jops); fallback Cantor Jops have no field counts and are excluded from kappa_J and flagged (nfb).
D14. Seeds: curve/instance generation, arm seeds, FB multiplier, bootstrap and control seeds are exactly as in section 6. The per-curve checks (which consume randomness) use their own seed
     MASTER*100000 + k*100 + c and do not feed the arms.

## Smoke tests that ran (all outputs in dev/, none read by the analysis)
S1. Toy sizes k = 6, 7, 8 with seeds 777, 12345, 999 (section 0 allows k <= 8). k = 6, 7 are below the preregistered range and were used only to exercise the analysis code across several sizes.
S2. A rho code test on a separate curve over q = 1000003 (negation-map rho, 30 runs) to verify that the 2-cycle escape rate per step (1.4%) and R (1.22) behave as in G1. Not a preregistered size; no data from it enter the study.
S3. Synthetic solver timing with random fake relations at q = 65521 (no curve, no Jacobian element) to size the elimination code.

## After the code freeze
No frozen file was changed (`sha256sum -c FROZEN_SHA256.txt` passes after the run). The following files are NEW after the run and are not part of the frozen set:
P1. `posthoc.gp` (output `out/posthoc.txt`): exploratory diagnostics written after seeing the main tables, because three preregistered checks missed or were ambiguous
    (F2 slope of R_E steps, the pooled NC-A statistic of H5 which analyze.gp reports only per size, and the H4 slope). It computes (A) the pooled NC-A ratio with a bootstrap CI,
    (B) the R_E cofactor / setup-share / steps-versus-sqrt(l/2) diagnostics, (C) a phase-only ratio (IC-D trials over R_E steps, setup excluded). None of it is a ranking input
    and none replaces a preregistered row; it is labelled post hoc wherever quoted.
P2. `make_results.sh` assembles RESULTS.md from `RESULTS_head.md` (hand-written text) and the generated tables; it creates no number.
P3. Reduced replication (`KMAX=12 CTRLSET=reduced OUT=out_reduced`, same seeds, same frozen code): all 20 raw files for k = 8..12 and the controls at k = 8, 10 are identical to the
    full run in every field except the wall-time field (`diff` after stripping it), and every ratio geometric mean agrees exactly. This is a same-code same-seed determinism check by the
    run agent itself, NOT the independent Haiku replication and NOT the Sonnet adversarial verification that section 10 lists as deliverables (`VERIFICATION.json` was not produced:
    this run had no subagent tool). The independent replication and the adversarial review remain open obligations.
P4. The preregistered text that is not met in the data is not edited: see RESULTS.md, "Preregistered checks that missed".
