# DEVIATIONS and implementation decisions, G3-E1-mixed-census-recalibrated-NC2

Section A was written BEFORE any preregistered-seed data was produced (the scripts were hashed after it, see raw/script_hashes.txt). Smoke tests
before the freeze used seed G3_SEED=1 (NOT the preregistered seed 20261026), 12-bit worlds and tiny target counts, only to check syntax and that
the pipeline runs end to end; their outputs are not part of any result and live under scratchpad/g3tmp/ (outside this directory).
Section B is appended after data and lists anything that changed afterwards (none unless stated).

## A. Interpretations and execution choices made before data (not changes of the protocol's science)

A1. The preregistration says the unchanged functions of e1_census.gp are reused. g3_census.gp = e1_census.gp with: env prefix G3_, default seed 20261026,
    new mkprime/mkcurve (section 5), new stage_controls (section 6), stage_grid default guard 120 s, new stages nc2 and res, new dispatch.
    Detector A, Oracles B, G, S, classify, pkeys, tab functions are byte-identical to E1.
A2. Controls are executed as separate gp processes per bit size (G3_CPART=pc|nc1|nc3, G3_CBITS=b) and their outputs concatenated into
    raw/stage1_controls.out. Same code and seeds; the split only keeps every single run under the 15-minute limit. Positive controls (pc) run
    before every other stage, including NC1/NC3 and the grid. A failing PC (G1) stops the driver; failing NC1/NC3 do not (they only make
    evidence inadmissible per section 6).
A3. PC robustness: E1 used target T[1] for all planted-tuple controls. If x_R of T[1] already lies in the base V the PC2 loop could never
    terminate; g3_census.gp therefore uses the first target whose x_R is not in the base V (index logged on PCW lines). Not a change of
    the planted constructions. Planted-control base n = min(ceil(q^0.5),400) exactly as section 6.
A4. NC3 factor-base size: alpha 0.65 with the grid's cap rule (cap 16384 for m=2, 1500 for m=3; section 5 "n = min(ceil(q^alpha), cap)").
    (E1's NC3 stopped at 16 bits; at 20 bits an uncapped m=3 pair table would not fit in memory.) NC1 detector-only size uses cap 1500.
A5. NC2 mean-pi power condition: computed as the unweighted mean of pi_t over all targets completed in the cell (the weighted mean over
    solutions is also written). Bootstrapped statistic D = sum(m_t - N_t pi_t)/sum(N_t); resamples with sum(N_t)=0 are skipped and counted
    (boot_valid); a cell with < 10 valid resamples fails. A cell also fails if not all K primes x 6 curves are present.
    pi_t computed from the realised n_t, nW and m by exact binomials (section 6).
A6. Pooled F4 estimator (section 7, "pooled c_hat/c_ref"): R = sum_t M_nd,t / sum_t (N_nd,t * f_ref), f_ref = c_ref n_T/q per config, N_nd = E_nd+M_nd.
    For one config this equals the per-config (f q/n_T)/c_ref exactly. Hierarchy: primes -> curves (target vectors of a curve summed over its
    configs of the pooled FBs and alphas, so a target is resampled jointly across configs) -> targets. The naive-1.5 variant
    (denominator N*1.5 n_T/q) is written next to it. Pooled bits-level strata use all alphas of the main grid (m=3, n_T in {1,3}, FB random, interval, farey).
    A stratum is RESOLVED when sum over targets of S_ref (expected mixed count) >= 5.
A7. Category rule (section 7) operationalised: "inside" = hierarchical 95% interval within [0.8,1.25]; "excludes" = interval disjoint from [0.8,1.25].
    A: inside at 12, 16, 20 bits. B: inside at 16 and 20, 12 not inside. C: otherwise. If any of the three bit levels is unresolved the category
    is UNDETERMINED. The 20-bit entry is the pooled 20-bit grid stratum if resolved, otherwise the R20 random+farey stratum if resolved
    (the R20 cells exist to resolve 20 bits). C with an interval that merely overlaps the band is reported as "H3 not confirmed, not refuted";
    H3 is falsified only if the interval excludes the band (section 3).
A8. F4 (refutation of Rt) is evaluated on: the bits-level pooled grid strata at 16 and 20 bits (if resolved) and the resolved R20 strata
    (random, farey, random+farey); finer (bits, n_T, FB) strata are in f4_table.tsv for reading but do not gate.
A9. R20 RESOLVED requires both random and farey per-FB structural counts S_nd >= 20 (pooled over worlds), R24 requires S_nd >= 10 (random).
    Resolution strata: hierarchy primes (2 at R20, 1 at R24) -> curves (2) -> targets (block targets concatenated). The R24 stratum has 1 prime,
    so its prime-level resampling is degenerate; its interval reflects curves and targets only.
A10. Resumability: run.sh skips a job if its output file already contains its completion marker (same seeds, byte-identical rerun would result).
     G5 output lines are "G5 OK file" / "G5 MISMATCH file" (the frozen command printed mismatches only); G5 passes iff all re-run jobs print OK.
A11. Per-config bootstrap intervals for the E-only mean (E1) are dropped: they were not preregistered here (section 7 gives point ratios B_naive, B_ref
     and per-config f intervals by ESS). Point ratios are in summary.tsv.
A12. NC2 time guard 120 s per config (same as the grid), not listed in the nc2 command text; truncation would be reported.

## B. Deviations discovered after data
B1. After all data and the verdict were produced (run.sh start 2026-10-26 about 18:08 UTC, aggregation finished 2026-10-27 00:14 UTC; one uninterrupted
    run.sh process, no job re-run except the preregistered G5 re-runs), operation-count tooling was ADDED at the request of the orchestrating task
    (it asks for operation counts as the primary cost metric): g3_opcounts.gp (reads recorded CFG/END, NCCFG/NCEND, RCFG/REND lines), g3_opcount_verify.gp
    (instrumented counter: 5 configs x 20 targets, formulas table=[m==3]n(n+1)/2 and target=n*targets match the counted pkeys calls exactly, raw/opcount_verify.out)
    and replicate_subset.sh (reduced replication). No frozen script (g3_census.gp, g3_aggregate.gp, plot_g3.py) and no raw file was changed. The aggregator's
    gates, verdict and tables were produced before these scripts existed and are unchanged.
B2. run.sh was extended by a final Stage 4 (ops counts + splice of an "ops" key into results.json). Its sha256 therefore differs from SCRIPTS.sha256
    (frozen run.sh: 494a6444...07; extended run.sh: see raw/post_hashes.txt). Stages 0-3 of run.sh are unchanged. results.json before the splice is kept as
    raw/results_aggregator_only.json; the splice only appends the "ops" key.
B3. Disclosure, not a protocol change: PREREGISTRATION section 7 says twistmult is "excluded since S=0 by structure". The data show S_nd > 0 for twistmult in the
    12- and 16-bit grid (e.g. nT=3, S_nd = M_nd = 11842 over all bit sizes; 0 at R20 and at 20/24 bits), i.e. the structural predictor is not zero for twistmult; M = S holds
    exactly for twistmult as for the other FB types (0 target mismatches). twistmult remains excluded from the F4 categories as preregistered and is reported separately.
B4. Disclosure: the F4 category rule (A7) uses the pooled 20-bit GRID stratum for the 20-bit entry because it is resolved (S_ref expectation 36.8 >= 5); the R20 strata
    (1185 mixed, random+farey) are reported next to it in f4_table.tsv and RESULTS.md. Both have wide hierarchical intervals because the resampling unit has only 3 (grid) or
    2 (R20) primes and 4 R20 curves.
B5. Wall times in this directory are secondary and contention-affected (4 cores shared with up to about 20 agents; the full run took about 6.1 h wall for the whole pipeline
    including a 12-minute aggregation). Operation counts are the primary cost metric.
