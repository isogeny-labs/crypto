# Run report — factor-base → D* campaign, corrected full rerun (run-20261017T2300Z)

**Engine:** `sage/factor_base_dor.sage` with the seed-overlap fix
(`struct_rng_for`) and built-in exact reachable-set yields; selftest clean.
**Orchestrators:** the five round scripts chained; engine-budget
chained-m=3 grid cells and the PolyBoRi-segfault cell culled
(`cull_m3.sh`/`cull_badcells.sh`), failures preserved in the logs.
**Raw records:** 124 cells under `raw/` (m=2 grids n=9..29 seeds 1–3,
subfield contrasts, δ sweeps, symmetric-S4 cells through n=41, m=2 n=29
extension, stack/frobwin checks).
**Measured:** D* mean/min/max, corrected decomposable fractions, exact
reachable-set sizes and yields, Macaulay shapes, δ rows.
**Derived:** pooled correlations (153 m=2 aggregates), per-(n, structure)
work-per-relation optima, exact-yield law |S| ≈ 2^(3l−1).
**Comparisons vs the seed-overlap era (`raw_v1_seedoverlap/`):** D* values
unchanged; fractions corrected downward (small-fraction cells most); optima
shift to smaller dimensions for random bases at n=11/13.
**Missing:** the two culled chained-m=3 grid cells; sym4 random at n=15 l=3
(censored above dmax); wall-clock claims absent.
