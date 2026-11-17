# EXP-PFDR-2d36f9 (O-INVALID) diagnosis and EXP-PFDR-ada71b verification note

Date 2026-11-01. Measured = computed from raw records by the scripts in `analysis/`; Reasoned = argument only.

## 1. A v2 already exists: no third design was written
`EXP-PFDR-ada71b` (DEC-20261101-49576f design, DEC-20261101-cb3277 approval, frozen, `execution_state: ready`, `off_main_runs: []`)
repairs the v1 defects with fresh seed blocks and unchanged thresholds. A further replacement would fork the evidence for
H-PFDR-ed7ccf, so the diagnosis below was used to *check* ada71b, not to replace it.

## 2. Why v1 read O-INVALID (Measured, v1 RUN-PFDR-b126f3 / 601709 raw records)
* Sizing. The pilot (8 curves, margin 1.1) over-estimated the per-curve null mean at 16 bits / theta 0.30 by 18% (1.400 vs census 1.187),
  so M = 2333 < 2500. Pilot/census ratios over the ten cells ranged 0.85 to 1.20 (pilot relative SE 4-18%). Resampling the real curve
  population: under the v1 rule the expected number of short cells was 1.28 of 10 (worst cell P(M<2500) = 0.28). v1 was expected to fail,
  not unlucky. ada71b's rule (32 pilot curves, margin 1.5) simulates to 0 short cells in 4,000 trials per cell; realized pilot means were within 0.2% of
  v1's census mean at the hardest cell, every cell planned at M ~ 3750, and the rehearsal's minimum M over every rung of every line is 3670.
* C6. The 30 cells carried only 10 distinct z_cal values (identical to 4 d.p. across d), because the null bases are seeded without d and
  every d read replicate 0. The band [0.70, 1.30] was sized for 30 independent values (gate fails 4.8% under an exact N(0,1) z); with 10 distinct values it fails
  37.1% (sd band 20.7%, mean band 20.7%). The observed sd 0.660 is at the 11.4th percentile of the 10-value reference. Same defect in C5: the six A/A lines were two lines
  (identical delta_hat), so "5 of 6" meant both must cover 0 (passes 90.2% at nominal 95%).
* Variance estimator. No double counting and no upward bias: using all 20 replicates, Var(sum of replicate totals) / sum of per-curve variances is
  0.9997 mean over the ten groups (range 0.67-1.70, the expected spread of a 19-df quantity), and 0.998 over 70 curve blocks. Replicate 0 was one draw of that spread.
  Not widening the band was correct; the repair is distinct replicates (ada71b R6).
* ada71b operating characteristics on the v1 known-null replicates: A/A percentile-bootstrap CI covered 0 in 39/40 lines (97.5%), C5 pass ~0.99-1.00;
  C6 with 30 distinct values passes 95.2% theoretically (97.3% in the finite resample).

## 3. Verification of the ada71b rehearsal
* Independent numpy recomputation (different bootstrap): delta_hat identical to 1e-9 for all six lines, CIs within 5e-4, z_cal identical.
* 180 brute-force base counts over 36 sampled census jobs (null, structured, Z/N): 0 mismatches against the C kernel.
* Pilot and census re-run from scratch: byte-identical modulo seconds and record order.
* Order certification failures: 0; every cell has exactly n_curves records.

## 4. Red-team (independent read, `analysis/redteam_report.md`): consequences for wording
* The class tests the elasticity of kappa, not the exponent of the excess. Poisson excess eps = a N^delta injected into the real null records: ALIVE needs
  eps(32 bits) ~ 230 at delta = 1/3. An excess on the break-even exponent with eps(32) <~ 0.06-0.1 reads SUB-BE. What the data bound is amplitude: kappa in [0.916, 1.058], upper95 kappa(32) <= 1.105.
* Plants and Z/N validate a regime with kappa(16) = 1.3; they say nothing below kappa - 1 = 0.1.
* Untouched: FAM-1 small_x/subgroup/dickson arms, log-GAP, bbc53e base, j = 0 and 1728, rungs above 32 bits, h > 2.
* The "h = 3, 4 thresholds" compare an h = 2 slope; no 3- or 4-sum was counted.
