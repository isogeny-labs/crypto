# LEAD-ICB-koblitz-setup-phase-cost

## 1. Lead as recorded
- disposition: open_next_step (last_reviewed 2026-10-30; source atlas_survey_20261107 --pursuit)
- family: ic-binary
- title: Cutting the reusable set-up of the one-target Koblitz pipeline (Table A), where 98.5-99.7% of S sits
- next-step excerpt (truncated in leads.json): "Port the S3 root-index build and the rank finder from the cryptanalysis n = 83 producers (experi..."
- Files read: research/lead_sweep_20261110/leads.json (entry);
  research/prime_field_lead_assessment_20261107/atlas_survey.md (line 211, one-line listing only);
  research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (id/title/disposition only).
  No fuller lead material found in the searched trees; the protocol works from the entry and the generic mechanism.

## 2. Hypothesis
In a toy Koblitz-curve index calculus (curve y^2+xy=x^3+x^2+1 over F_{2^n}, n = 11, 13, 17, m = 2 decomposition over
a factor base of points with x in an l-dimensional polynomial-basis subspace, l = (n-1)//2), the reusable set-up
(sums table, relation collection, rank finder and elimination) is at least 95% of the one-target cost S, with set-up
share f = setup/(setup+online) taken at every unpriced-work weight w in {0, 0.1, 1} (w prices one hash lookup or one
modular multiplication in group-addition units), and the break-even number of targets setup/online is at least 10.
This is the toy-scale shape of the recorded 98.5-99.7%; the claim concerns where cost sits, not a speedup.

## 3. Preregistered prediction and falsification criterion
- Statistic: median over 5 seeds (101..105) of f(n, w), per cell (n, w); also median break-even K* = setup/online.
- consistent_with_prediction: every cell median f >= 0.95 and K* >= 10, all controls pass, A/A |diff| <= 0.02.
- falsified: any cell median f < 0.90 (set-up is not dominant at toy scale, the premise of the lead is not
  reproduced; it decides nothing about n = 83).
- inconclusive: anything else, including any failed control or A/A drift above 0.02.
- Positive control: every online target's discrete log k is recovered and equals the planted k (the pipeline works).
- Negative / null control: the same solved log table with one log corrupted (+1) must make at least half of the
  targets fail, so the recovery check can see a wrong table.
- A/A arm: a second disjoint seed set (201..205), identical code and sizes; median f must agree within 0.02.
- Not applicable: matched-rho comparison (this measures phase shares, not a cross-method ranking, so no section 5a
  arm); no timing is used, only operation counts.

## 4. Method
- Field: GF(2^n) by log/antilog tables, primitive polynomials x^11+x^2+1, x^13+x^4+x^3+x+1, x^17+x^3+1 (asserted primitive
  at runtime). Group order N = 2^n + 1 - V_n with V_n = V_{n-1} - 2 V_{n-2}, V_0 = 2, V_1 = 1 (curve has t = 1 over F_2).
  r = largest prime factor of N, cofactor h = N/r; P has order r.
- Setup phase (counted separately): (1) factor base FB = {h * pt(x) : 1 <= x < 2^l, pt exists}, one representative per
  +-pair; (2) sums table: for all i <= j and both signs, store Fi +- Fj keyed by x; (3) relations from R = aP walked by
  +P (restart with a fresh random scalar multiplication every 64 steps), each table hit gives a row a = +-(li +- lj) mod r;
  (4) rank finder: incremental Gaussian elimination mod r, stop at the first moment rank = |FB|; (5) back-substitution.
- Online phase per target Q = k0 P: R = sQ + aP walked by +P, first table hit gives k = (+-(li +- lj) - a) / s mod r;
  verified against k0. 20 targets per seed.
- Cost units: group additions or doublings (weight 1), hash lookups and modular multiplications (weight w in
  {0, 0.1, 1}). f(w) = (setup_adds + w*setup_other) / (that + online_adds + w*online_other) per target.
- Sizes: (n, l) = (11, 5), (13, 6), (17, 8). Seeds 101..105 (main), 201..205 (A/A); random.Random(seed) only.
- Cost: pure Python, one core, about 65k table additions at n = 17; expected well under 2 minutes, under 100 MB.
  `python3 -I run.py --smoke` uses n = 9, 2 seeds per arm, runs in well under 1 s.

## 5. Classification
runnable_local. A stdlib toy can decide whether the set-up/online split reproduces at small n and gives the break-even
target count, which sharpens the premise that the lead's saving must come from the set-up.

## 6. What a run cannot establish
- Nothing about n = 83, the real Table A pipeline, S3 root-index build or the cryptanalysis producers; the toy has
  a different factor base, no Frobenius-orbit reduction, and a tiny linear system.
- Not an improvement: no set-up reduction is implemented, so it is not a speedup and cannot enter the leaderboard.
- The toy's set-up share depends on l = (n-1)//2 and the m = 2 table; a share >= 0.95 is partly built in by the
  table size, so it supports only the weak claim that set-up dominates in this regime.
- Operation counts, not wall time; the weights are a band, not measured native costs. No matched rho reference.
- Results are labelled [pcc:unverified].
