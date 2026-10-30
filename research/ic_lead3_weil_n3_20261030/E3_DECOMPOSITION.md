# E3: cost decomposition and crossover for Lead 3 (n = 3 Weil descent IC)

Desk analysis of the recorded data only (`results.json`, `data/armP_*.log`, `fit_output.md`, `NOTE.md`). No new runs, no builds. The refits were done with a throwaway numpy script over those files (OLS in log2-log2, bootstrap resampling instances within each p, 4000 draws). Arm P (2lp = Wiedemann + double large primes, F' = B^{2/3}) is the only arm that can beat rho in theory (NOTE sec. 1), so everything below is about 2lp. Unit = counted ops (affine group adds + Fp muls/63), as in NOTE. Rho = S_rho * sqrt(n), pooled S_rho = 1.336 +- 0.104 (30 instances). p = q, n = group order ~ p^3, so log2 n = 3 log2 p.

Labels: **[M]** measured, **[F]** fitted to measured points (interpolation), **[X]** extrapolated beyond p = 8209 (log2 n = 39).

## 1. Where the cost goes [M]

Additive split of total ops, mean over 5 seeds (shares from the `.log` fields `Fp_muls/residual`, `group_ops`, `total_ops`; LA from `results.json` `la_phase`):

| p | log2 n | Groebner/decomposition solves | group ops (oracle, residual generation) | linear algebra | total / rho |
|---:|---:|---:|---:|---:|---:|
| 271 | 24.2 | 99.55% | 0.50% | 0.017% | 3535 |
| 523 | 27.1 | 99.45% | 0.55% | 0.028% | 2891 |
| 1039 | 30.1 | 99.37% | 0.61% | 0.033% | 2442 |
| 2083 | 33.1 | 99.32% | 0.66% | 0.032% | 2264 |
| 4099 | 36.0 | 99.25% | 0.71% | 0.040% | 1781 |
| 8209 | 39.0 | 99.19% | 0.77% | 0.042% | 1618 |

Dominant at every measured p: the per-residual Groebner solve, >99%. Linear algebra is negligible in cost share (<0.05%), but note that on its own it is already 0.59-0.81 x rho at every p (LA/rho column below), i.e. LA is the same order as rho and has no headroom to hide behind.

Multiplicative structure (this is what scales). total = |F'| x (decomps per independent relation) x (residuals per decomposition) x (ops per residual) + LA:

| p | |F'| (indep. relations, median) | decomps/indep (median) | residuals/decomp | ops/residual | LA / rho |
|---:|---:|---:|---:|---:|---:|
| 271 | 27 | 7.8 | 6.8 | 1.40e4 | 0.59 |
| 523 | 42 | 12.9 | 6.1 | 1.43e4 | 0.81 |
| 1039 | 67 | 17.7 | 5.6 | 1.45e4 | 0.80 |
| 2083 | 106 | 28.3 | 5.9 | 1.47e4 | 0.73 |
| 4099 | 164 | 43.9 | 5.9 | 1.47e4 | 0.71 |
| 8209 | 258 | 69.2 | 6.1 | 1.49e4 | 0.69 |

Check at p = 8209: 258 x 69.2 x 6.1 x 1.49e4 = 1.6e9, matches measured 1.61e9. (Medians are used because two seeds, p=1039 seed 2 and p=2083 seed 2, ended with |F'|-sized rank 127 and 415 instead of ~67 and ~103, inflating the means in `fit_output.md`; see sec. 2 for the effect on the fits.)

Term mapping to the requested categories:
- **Decomposition solves per relation**: ops/residual x residuals/decomp x decomps/indep. The first two factors are flat (see sec. 2); the solve cost is a constant ~9.3e5 Fp-muls per residual, of which the log reports macaulay 5.08e5 (echelon 1.5e5 + normal form 3.57e5) and precompute 4.1e4; the remaining ~3.8e5 is not broken out in the logs (not attributed here).
- **Smoothness yield**: the decomposition probability, 0.151-0.180 (~1/n! = 1/6 for n = 3), flat.
- **Large-prime overhead**: decomps per independent relation, 7.8 -> 69, the only factor besides |F'| that grows.
- **Linear algebra**: ~ unknowns^2 x weight. Logged Wiedemann weight rises 8.6 -> 22.0 over p = 271 -> 8209 (single-instance log fields, mean of 5), unknowns ~ B^{2/3}.

## 2. Scaling of each term in p, with uncertainty vs theory [F]

Exponents e in cost ~ p^e (NOTE reports exponents in n; multiply by 3). Bootstrap 95% in brackets. "Late" = p >= 2083 only (3 sizes, weak).

| quantity | fit, all 30 rows | late (p>=2083) | theory (Gaudry-Diem, n=3, double LP) | verdict |
|---|---|---|---|---|
| ops per residual | 0.017 [0.016,0.018] | 0.012 | 0 (poly in constants only) | consistent (tiny drift) |
| decomposition rate | 0.15-0.18, p^-0.02 [-0.06,0.02] | | 1/3! const | consistent |
| |F'| = indep. relations needed | 0.65 [0.62,0.68] | | 2/3 (F' = B^{2/3}, 255 predicted vs 257 seen at p=8209) | consistent |
| decomps per indep. relation | 0.63 [0.61,0.65] | 0.78 [0.61,1.06] | 2/3 (needs q^{4/3} decomps over q^{2/3} relations) | consistent, drifting up |
| residuals | 1.257 [1.233,1.283] | 1.254 [1.13,1.34] | 4/3 = 1.333 | below theory, CI excludes 4/3 on all rows |
| **relation phase (= total)** | **1.274 [1.251,1.300]** | 1.266 [1.14,1.36] | **4/3 = 1.333** (n^{4/9}) | below theory by ~0.06; late CI includes 4/3 |
| LA phase | 1.515 [1.462,1.564] | 1.461 [1.32,1.59] | q^{4/3} x (weight growth) | see below |
| rho | 1.500 | | 1.5 | by construction |

Notes on the fits.
- Removing the 4 instances whose |F'| is >1.3x the median at their p changes nothing material: total 1.270 [1.249,1.294], LA 1.522 [1.473,1.565].
- The relation phase fits *below* the proven exponent. Two readings: (a) pre-asymptotic (small graphs; the overhead exponent is rising, 0.63 all rows, 0.72 for p>=1039, 0.78 for p>=2083, which is the direction that would restore 4/3); (b) genuine. The data cannot separate them: the three largest sizes give exponent CI [1.14,1.36] for the relation phase. Treat 1.333 as the realistic asymptote and 1.27 as the optimistic reading.
- LA: expected ~ unknowns^2 x weight = p^{4/3} x p^{~0.28} (weight 8.6 -> 22.0) = p^{~1.61}; the fitted 1.515 is lower because the per-entry constant falls (ops/(unk^2 x w) 4.85 -> 3.41 across the sizes, from the logs). So the LA exponent of 1.515 > 1.5 (rho) is measured, not a theory number; theory-wise it could be as low as 4/3 if weight stayed bounded. This is the swing parameter for the crossover (sec. 3).
- Unit caveat on LA: `results.json` `la_phase` = raw `la_ops` x `la_fp_equiv`/63. The raw logged `ops=` is ~7.9x larger (4.76e6 vs 5.9e5-7.6e5 at p=8209). I did not verify the `la_fp_equiv` conversion (~8 Fp-mul equivalents per LA op, implied). If the raw count were the right unit, LA alone would be ~5.4 x rho at 2^39, which only strengthens the close-n=3 verdict.

## 3. Crossover with rho [X]

Anchor: measured mean at p = 8209 (log2 n = 39.0), total/rho = 1618, relation phase 99.96% of that.
If total ~ p^e and rho ~ p^1.5 then total/rho ~ n^{-(1.5-e)/3}, so log2 n* = 39.0 + 3 x log2(1618)/(1.5 - e) = 39.0 + 31.9/(1.5 - e) with e in p-exponent units.

| scenario | relation exp | LA exp | crossover log2 n | 
|---|---:|---:|---|
| A. proven exponent both phases (central), fitted constants | 1.333 | 1.333 | **~2^230** (rho-constant band 1.232..1.440: 2^233..2^229); p ~ 2^77 |
| B. fitted relation exp, LA at 1.333 | 1.274 | 1.333 | 2^181 |
| B-lo. same, bootstrap CI of relation exp | 1.248..1.302 | 1.333 | 2^166 .. 2^201 |
| C. fitted both phases (NOTE's projection) | 1.274 | 1.515 | **never**; min total/rho = 1.5 at 2^226 |
| C'. robust fit both | 1.270 | 1.522 | never; min 1.9 at 2^210 |
| D. relation 4/3, LA at fitted 1.515 | 1.333 | 1.515 | never; min 1.7 at 2^282 |
| E. late-slope fit both (p>=2083) | 1.266 | 1.461 | 2^179 (but each slope has CI ~ +-0.1; weak) |

Error band: if the proven exponent holds for *both* phases, the crossover is **2^230 (2^166 to 2^233 across all constants and exponent CIs for the crossing scenarios)**, and in the scenarios where LA keeps its measured exponent above 1.5 there is no crossover at all. The extrapolation distance is 127-192 bits beyond the last measurement, so these are indicative, not predictions. What is measured: the 1618x gap at 2^39 and the trend 3535x -> 1618x over 15 bits (about -0.075 in log2 ratio per bit of log2 n). What is extrapolated: everything past 2^39, including the assumption that constants stay put.

Required speed-up at a "computable" scale (log2 n = 80, i.e. p ~ 2^27) to reach parity, from the same anchor: total/rho there is 1618 x 2^{-41(1.5-e)/3}: 334x (e = 4/3), 190x (e = 1.274), 148x (e = 1.248, CI edge).

## 4. Verdict

Crossover is beyond 2^80 in every scenario (best case 2^166, central 2^230, or never), by more than 85 bits. **Recommend closing n = 3.**

Single-term attack check, to be explicit about why no experiment is worth naming: the dominant term is the Groebner/decomposition solve (99.2%). Even if it were made free, the group-op and LA terms remain: (0.77% + 0.04%) of 1.61e9 = 1.3e7 ops = 13 x rho at 2^39, a maximum total reduction of ~123x. The required reduction at 2^80 is 148-334x (sec. 3), so even a free solve falls short of parity by a factor of 1.2-2.7 at 2^80, and that bound further assumes the oracle group-op and LA terms don't grow faster than the relation phase (the LA exponent fit says they might). The remaining growing factors (|F'| ~ p^{2/3}, decomps/indep ~ p^{0.63-0.78}) are the algorithm's structure, not an implementation constant. I see no single term whose reduction by an achievable factor brings the crossover under 2^80.

Caveats: one rho walk per instance with S_rho spread 1.2-1.8 (pooled 1.336 +- 0.104 enters only as +-2 bits in scenario A); 5 seeds per size; the largest 3 sizes carry the trend; the proven exponent and the Gaudry-Diem constants were not independently verified (NOTE sec. 1 cites Joux-Vitse and a search snippet for Diem). Pricing (AGENTS.md 5a) moves the 8209 ratio only 1618 -> 886 across 0..1 GAE and does not change any conclusion.
