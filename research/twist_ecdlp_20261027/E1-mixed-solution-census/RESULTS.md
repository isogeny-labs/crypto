# RESULTS E1-mixed-solution-census

Protocol: `PREREGISTRATION.md` (frozen 2026-10-21), executed verbatim by `run.sh`; deviations D1-D7 in `DEVIATIONS.md`.
All numbers below are copied from `summary.tsv`, `pooled.tsv`, `controls_summary.tsv`, `verdict.txt`, `results.json`,
`extra_stats.txt` (raw outputs: `raw/`, symlink `out/`). gp 2.15.4, seed S0 = 20261021, no Rust, no main-repo build.
Original run 2026-10-21 (stage 0 ~3 min, stage 1 ~5 min, stage 2 ~25 min of wall time by file mtimes). A second complete run of the
same frozen commands on 2026-10-26 (`../E1-replication`) reproduced every non-timing number bit-for-bit (`replication_check.txt`).
No claim about ECDLP cost, speedup, or improvement over index calculus is made or implied; no rho arm, so no GAE pricing (prereg section 4).

## Headline (read this first)

Formal outcome: INCONCLUSIVE. The preregistered instrument gate F6 is unmet: the literal NC2 test passed in only 3 of 16 cells
(prereg section 8, F6; D4). The pre-declared consequence is that nothing is reported as evidence for or against the lemma. The
substantive numbers are nevertheless reported below, and a reader should weigh them as follows.

- Exact structure (H1): 0 mismatches between observed mixed-solution counts and the independent structural predictor S in all
  91,200 (config, target) pairs over 592 configs; m=2 mixed count 0 everywhere; m=3 with n_T=0: 0 non-degenerate mixed in
  41,815,598 non-degenerate E-only solutions.
- The claimed lemma "B has 2B = O, hence B = O for odd order" is consistent with every run, but "B = O" is false when the curve has rational
  2-torsion: mixed solutions exist there, equal to the predicted family (two twist points differing by a 2-torsion point T plus an E point R+T),
  and have A = B = T, not A = R.
- "About 1/q" (H2) holds as a scaling, mixed/E-only ~ 1.5 n_T / q for m=3 with n_T>0, but the literal threshold 5/q was exceeded
  with Wilson lower bound above 5/q (falsifier F4) in 6 of 143 resolvable configs, all m=3, n_T=3, q in {2609, 2917} (12 bits).
  The threshold 5/q lies only 11% above the predicted 4.5/q for n_T=3, so small-q finite-size deviations breach it.

## Sample sizes

- 592 configs (320 with m=2, 272 with m=3), 7 primes (12/16/20 bits x 2, 24 bits x 1), 4 curves per prime, 4 factor bases, 3 sizes
  (duplicates after capping run once), 91,200 (config, target) pairs in total (64,000 for m=2, 27,200 for m=3), 0 truncated, 0 not run,
  0 incomplete configs.
- Curve flags: 524 configs `ok`, 68 configs `FALLBACK_no_gcdodd_in_5000` (curve_idx 1 at pidx 1 for 12/16/20 bits, where no n_T=0 curve with
  gcd_odd>1 was found in 5000 draws; gcd_odd = 1 there). Curve 0 of pidx 0 has gcd_odd = 15 (12 bits), 3 (16, 20, 24 bits), n_T=0,
  so the coprimality assumption is tested in the n_T=0 stratum anyway. Strata with gcd_odd>1: 124 n_T=0 configs (m=2 and 3 together) etc. (`extra_stats.txt` E):
  non-degenerate mixed = S in both gcd_odd=1 and gcd_odd>1 (e.g. m=3, n_T=0: 0 vs 0; m=3, n_T=3: 6956 vs 6139, each equal to S_nd).
- Targets with x_R in V: 4112 of 91,200 (counted as degenerate where applicable).

## Controls (prereg section 6), all run before the grid

- Stage 0 V0 (Detector A vs Oracle B vs Oracle G vs Oracle S): 640 rows, 0 disagreements; extra toy block (V = all of F_q, q in {101,103}):
  320 rows, 0 disagreements; closed-resultant formula check: 0 bad. Solutions seen by A in V0: 18,008, of which 628 non-degenerate
  mixed and 12 degenerate mixed. TOY: 3900 non-degenerate mixed, 7940 degenerate mixed, all agreeing. Result PASS.
- Positive controls: PC1 (planted non-degenerate mixed, n_T>=1) 400 of 400 found; PC2 (planted degenerate mixed) 400 of 400; PC3 (planted
  E-only, m=2,3) 800 of 800 (counts corrected on 2026-10-26 from `controls_summary.tsv`; an earlier draft said 600 and 1200, see D8). Sensitivity 100%, 0 bad groups.
- Negative control NC1 (E-only factor base): 32 groups (16 oracle-checked, 16 large), mixed = 0, 0 bad groups. PASS.
- NC3 (target on the twist, mirror): 32 groups, 0 bad. Mirror prediction held: m=2 gives 0 "mixed" in all 16 groups; m=3 mixed equals the
  swapped structural predictor in every group (n_T>0, random FB: 35, 58 non-degenerate at 12 bits; 16, 43 at 16 bits; 0 for twistmult and n_T=0).
- NC2 (b -> b+1, no lemma; mixed must look like 1-2^-m): observed mixed fractions 0.7266 to 0.9101 (min 0.7266). Literal criterion
  (Wilson 95% on pooled counts contains 1-2^-m): PASS in 3 of 16 cells, FAIL in 13. Post-hoc diagnostic (D4, not preregistered): target-cluster
  bootstrap against the conditional null 1-(nE/n)^m contains the observed fraction in 14 of 16 cells; maximal absolute deviation from
  that conditional null 0.0219. So the instrument sees mixed zeros at 73 to 91% when the structure is absent (versus 0 observed on E), but the
  preregistered gate as written is unmet. Interpretation per D4: the literal null ignores the realised class imbalance nE/n and
  the clustering of solutions per target; this is a post-hoc explanation and is not accepted as the formal outcome.

## Falsifier ledger (section 8)

| Falsifier | Result |
|---|---|
| F1 mixed solution with m=2 | 0 of 320 configs; 0 mixed (degenerate or not) in 64,000 targets |
| F2 non-degenerate mixed, m=3, n_T=0 | 0 of 136 configs |
| F3 any target with N_M != S (m=3) | 0 mismatches over 27,200 m=3 target-config pairs (and 91,200 overall) |
| F4 f > 5/q with Wilson lower bound > 5/q, N_E_nd >= 4q | FIRED in 6 of 143 resolvable configs: 12_0_3_m3_interval_0.65 (f = 0.00419), 12_0_3_m3_random_0.65 (0.00301), 12_0_3_m3_random_0.8 (0.00241), 12_0_3_m3_farey_0.65 (0.00307), 12_1_3_m3_random_0.65 (0.00236), 12_1_3_m3_random_0.8 (0.00184) |
| F5 H_rand wins | 0 strata |
| F6 instrument validity | stage 0 PASS, PC/NC1/NC3 PASS, NC2 literal 3/16: GATE UNMET |

H2 resolvable configs 143: point estimate <= 5/q in 130, > 5/q in 13 (12 bits: 1 with n_T=1, 9 with n_T=3; 16 bits: 3 with n_T=3), of which
F4 fires in 6 (all n_T=3, 12 bits). m=2 and n_T=0: 0 failures.

## Pooled by stratum (m, n_T, FB), counts summed; intervals recomputed on pooled counts (`pooled.tsv`)

N_E_nd, N_M_nd, S_nd = non-degenerate E-only, mixed, predicted mixed solutions. f = N_M_nd / (N_M_nd + N_E_nd), 95% interval
(Clopper-Pearson upper bound when N_M_nd = 0, Wilson otherwise).

| m | n_T | FB | configs | targets | N_E_nd | N_M_nd | S_nd | N_M_deg = S_deg | f | 95% interval |
|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 0 | farey | 40 | 8000 | 536153 | 0 | 0 | 0 | 0 | [0, 6.88e-6] |
| 2 | 0 | interval | 40 | 8000 | 545187 | 0 | 0 | 0 | 0 | [0, 6.77e-6] |
| 2 | 0 | random | 40 | 8000 | 544117 | 0 | 0 | 0 | 0 | [0, 6.78e-6] |
| 2 | 0 | twistmult | 40 | 8000 | 564662 | 0 | 0 | 0 | 0 | [0, 6.53e-6] |
| 2 | 1 | farey / interval / random / twistmult | 20 each | 4000 each | 270275 / 269945 / 273910 / 291326 | 0 | 0 | 0 | 0 | upper 1.36e-5 / 1.37e-5 / 1.35e-5 / 1.27e-5 |
| 2 | 3 | farey / interval / random / twistmult | 20 each | 4000 each | 269815 / 271553 / 276066 / 436414 | 0 | 0 | 0 | 0 | upper 1.37e-5 / 1.36e-5 / 1.34e-5 / 8.45e-6 |
| 3 | 0 | farey | 34 | 3400 | 11115484 | 0 | 0 | 42940 | 0 | [0, 3.32e-7] |
| 3 | 0 | interval | 34 | 3400 | 12156480 | 0 | 0 | 34411 | 0 | [0, 3.03e-7] |
| 3 | 0 | random | 34 | 3400 | 11499267 | 0 | 0 | 38146 | 0 | [0, 3.21e-7] |
| 3 | 0 | twistmult | 34 | 3400 | 7044367 | 0 | 0 | 23896 | 0 | [0, 5.24e-7] |
| 3 | 1 | farey | 17 | 1700 | 5398867 | 1621 | 1621 | 15378 | 3.00e-4 | [2.86e-4, 3.15e-4] |
| 3 | 1 | interval | 17 | 1700 | 5511474 | 1333 | 1333 | 21541 | 2.42e-4 | [2.29e-4, 2.55e-4] |
| 3 | 1 | random | 17 | 1700 | 5490592 | 1406 | 1406 | 18568 | 2.56e-4 | [2.43e-4, 2.70e-4] |
| 3 | 1 | twistmult | 17 | 1700 | 6839134 | 0 | 0 | 27087 | 0 | [0, 5.39e-7] |
| 3 | 3 | farey | 17 | 1700 | 5698771 | 4164 | 4164 | 24795 | 7.30e-4 | [7.08e-4, 7.53e-4] |
| 3 | 3 | interval | 17 | 1700 | 5357685 | 4098 | 4098 | 22879 | 7.64e-4 | [7.41e-4, 7.88e-4] |
| 3 | 3 | random | 17 | 1700 | 5665106 | 4833 | 4833 | 20193 | 8.52e-4 | [8.29e-4, 8.77e-4] |
| 3 | 3 | twistmult | 17 | 1700 | 8108803 | 0 | 0 | 25559 | 0 | [0, 4.55e-7] |

(m=2 rows with "/" give the four per-FB values in the order listed; each FB's pooled counts are in `pooled.tsv`.) The twistmult FB has 0
non-degenerate mixed even for n_T>0, exactly as S predicts (S_nd = 0): its twist points are multiples of one generator, so no two differ by T.
Degenerate mixed solutions (P - P + R - R pattern, m=3) are plentiful and exactly predicted (N_M_deg = S_deg in every stratum).

## Scaling of mixed/E-only with q (m=3, n_T>0, FB in {farey, interval, random}); `extra_stats.txt` A

Count-weighted mean of f*q/n_T over configs with N_M_nd > 0 (heuristic prediction 1.5):
12 bits: n_T=1: 2.20 / 1.64 / 1.56 (farey / interval / random); n_T=3: 1.56 / 1.70 / 1.92. 16 bits: n_T=1: 1.57 / 1.69 / 1.54; n_T=3: 1.28 / 1.60 / 1.21.
20 bits (single configs, N_M_nd = 1 to 2, uninformative): 7.06 (n_T=1 interval), 3.90 and 5.52 (n_T=3 farey, random).
24 bits: no config with N_M_nd > 0 (E-only counts too small to resolve). Where resolvable the ratio stays within about 1.2 to 2.2 of order 1.5, i.e.
mixed/E-only scales like n_T/q; it does not scale like a constant fraction.

## H3 baseline (random FB only; `extra_stats.txt` B, C)

148 random-FB configs. Refined baseline B_ref = 2^m nE^m / (m! #E): observed/B_ref in [0.828, 1.139], all 148 inside [0.7, 1.4].
Naive baseline n^m/(m! q): 142 of 148 inside [0.7, 1.4]; 6 outside (12_0_0_m3_random_0.5: 0.698; 12_0_0_m3_random_0.65: 0.694; 12_0_2_m3_random_0.5: 1.622;
12_1_2_m3_random_0.5: 0.621; 16_0_2_m2_random_0.5: 0.698; 16_1_0_m3_random_0.5: 0.644), consistent with the naive formula ignoring that only about half of V lies on E.
Per prereg, H3 failure would indicate a wrong baseline formula, not a failure of the lemma.

## Operation counts (primary cost metric; added 2026-10-26, `opcounts.tsv`, `opcounts_summary.json`, `opcounts_verify.txt`)

Unit: one `pkeys` call = the root set of one Semaev S_3(x_i, x_j, z) (one quadratic's coefficients, one modular inversion, O(1) field
multiplications; the per-call field-multiplication count is not instrumented and is not reported). Detector A cost per config, by construction of
`e1_census.gp`: m=3 table build n(n+1)/2 calls; every target n calls (+ at most 2n sorted-table lookups for m=3, at most 2n hash lookups for m=2).
Counts are computed from the recorded n, m, targets_done in `summary.tsv` and were verified EXACTLY against an instrumented re-run of Detector A on
3 configs (`opcounts_verify.txt`: 12-bit m=3 n=52: table build 1378 = 52*53/2, target loop 5200 = 52*100, 779 distinct solutions = opcounts.tsv row;
12-bit m=2 n=52: 10400 = 52*200, 102 solutions; 16-bit m=3 n=209: 21945 and 20900, 2756 solutions). Instrumented extras for the m=3 configs
(not derivable from the TSV): table entries 2084 and 33042; raw join hits 2208 and 8181 versus distinct solutions 779 and 2756 (each multiset is found
about 2.8 to 3 times, once per choice of the last index).

| m | configs | targets | pkeys (table build) | pkeys (target loops) | pkeys total | lookups (upper bound) | distinct solutions reported | brute-force-equivalent tuples (Oracle B style, not run) |
|---|---|---|---|---|---|---|---|---|
| 2 | 320 | 64,000 | 0 | 263,795,800 | 263,795,800 | 527,591,600 | 4,553,625 | 1,546,209,060,600 (S_3 evaluations, n(n+1)/2 per target) |
| 3 | 272 | 27,200 | 129,453,766 | 21,567,800 | 151,021,566 | 43,135,600 | 91,247,277 | 5,824,043,684,300 (n(n+1)(n+2)/6 resultants per target) |
| all | 592 | 91,200 | 129,453,766 | 285,363,600 | 414,817,366 | | 95,800,902 | 7,370,252,744,900 |

Detector A therefore used about 5.9e3 (m=2) and 3.9e4 (m=3) times fewer elementary root-set/tuple evaluations than the brute-force enumeration it replaces
(ratio of tuples to pkeys calls; not a cost comparison of ECDLP methods). Secondary, contention-affected wall seconds (sum over configs of the
recorded loop times, `opcounts_summary.json`): m=2 target loops 1065.5 s, m=3 table builds 922.7 s, m=3 target loops 969.3 s. Oracles B and G and stage 0/1
controls are not included in these counts. No group-operation or GAE pricing applies (no ECDLP solver arm, prereg section 4).

## Prediction assessment

- H1 (exact structure, m=2 none, m=3 N_M = S): held, 0 mismatches, with the correction that rational 2-torsion creates predicted mixed solutions
  (so the task's "B = O" is true only for n_T = 0, i.e. odd group order; the 2B = O statement is what survives).
- H2 (mixed fraction <= 5/q): holds in 130 of 143 resolvable configs; falsifier F4 fires in 6 (n_T=3, 12 bits). The qualitative claim
  "mixed is negligible and scales as ~1/q" holds where resolvable; the numeric 5/q cutoff as preregistered does not hold uniformly.
- H3: refined baseline held (148 of 148); naive baseline 142 of 148.
- Overall formal outcome: inconclusive because F6 (NC2 literal gate) is unmet; a mis-specified-gate explanation exists but is post hoc (D4).
- Not established: anything about ECDLP cost. A negative/near-zero mixed census says the twist adds essentially no relations to a plain
  Semaev census for R in E(F_q) (except the n_T>0 family above), which is not an improvement over index calculus.

## Limitations and missing items

- Oracles B and G only run where n <= 120 (as preregistered); large-n configs rely on Detector A, validated against B, G, S in stage 0 and by the exact N_M = S match.
- No run failed, was truncated or not run. No 20-bit or 24-bit config is resolvable under H2 for m=3 with n_T>0.
- Not executed through the director/Haiku relay (D6); this reporting session only verified and replicated (D7).
- Figures: `fig_pooled_fraction`, `fig_eonly_ratio`, `fig_mixed_vs_predictor` (.png/.svg), made by `plot_e1.py` from `summary.tsv`.
  `fig_mixed_vs_predictor` has a title clipped at the right edge (cosmetic).

## Reproduce

- Full: `sh /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E1-mixed-solution-census/run.sh` (stages 0 to 3 frozen, then stage 4: `extra_stats.txt`,
  `opcounts.tsv`, `opcounts_summary.json`, `opcounts_verify.txt`, `validate_results.txt`). About 35 to 70 minutes on 4 shared cores (sum of recorded grid job times 2965 s,
  stage 1 about 5 min, 2 concurrent grid jobs). `./replication_check.sh` compares with the second run in `../E1-replication`.
- Reduced: `bash .../E1-mixed-solution-census/run_reduced.sh` re-runs stages 0 and 1 and the 12-bit grid (64 jobs) into `reduced/` with the time guard raised (`E1_GUARD_S_OVERRIDE=1000`) and
  compares with this run (`reduced_compare.sh`, timings and the guard column ignored). Cost: about 15 min of CPU; wall 15 to 35 min depending on
  contention. Result: stages 0 and 1 byte-identical, 64 of 64 12-bit grid files identical in CFG/TGT lines, 0 truncated (`reduced_compare.txt`; first attempt and its one
  guard-truncated job are in `DEVIATIONS.md` D9).
- No re-run needed to check traceability: `sh validate_results.sh` (seconds, no gp) recomputes every headline count in this file from `summary.tsv`, `controls_summary.tsv`, `verdict.txt`, `raw/`.
