# LEAD-ICP-prime-ic-constant-engineering

## 1. Lead as recorded
- disposition: open_next_step (reviewed 2026-10-30); family: ic-prime; next_step_cost: cheap-medium (engineering).
- title: Constant-factor engineering of the prime-field IC pipeline (native arithmetic, batching, pair-table build, generator, LA filter, planner).
- next-step excerpt (truncated in leads.json): "The section 13 second-host replay of the 20-bit holdout is done (crypto research/prime_base_orac..."
- recorded evidence: 0.818 [0.773,0.866] / 0.836 [0.806,0.866] on the 20-bit pair table; gap to rho still 5-13x; constant factor only; hash-table floor about 53% of a relation trial; packed filter -4% wall at 32 bits only.
- files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 23, 240); /home/user/crypto/research/prime_field_lead_assessment_20261107/README.md (line 146). Other hits (session logs, notes) not read.

## 2. Hypothesis
In a pair-table relation walk on a prime-field curve, batching the affine-addition inversions (Montgomery trick, W=64) lowers the counted cost (M + c*I, c = I/M in {5,10,20}) per relation trial by a constant factor below 1 at every price, with an identical relation sequence, and the factor does not vary with field size (no exponent change). This is the batching component of the lead only.

## 3. Preregistered prediction and falsification criterion
- Statistic: R_c = cost(batched)/cost(base) per cell (3 sizes x 3 seeds), c in {5,10,20}.
- Prediction: max R_c < 1 for every c (analytic: about 0.77, 0.50, 0.31), spread of R_c across cells <= 0.05, hit sequences bit-identical across arms.
- Falsified if any R_c >= 1 or spread > 0.05. Inconclusive if any control fails or (full run) the A/A wall ratio leaves [0.8,1.25].
- Negative control / null: batch size 1 must reproduce the baseline counts exactly (R = 1.000).
- Positive control: a pair-table key planted equal to the x of walk 0 after step 1 must be hit at step 0 by every arm.
- A/A arm: baseline run twice (same seeds); wall-time ratio must lie in [0.8,1.25].
- Decisions: consistent -> batching is a real but constant-factor gain in the counted model (not an exponent change); falsified -> the counted gain model is wrong.

## 4. Method
Python 3 stdlib. For p of 12, 14, 16 bits (p = 3 mod 4), random short-Weierstrass curve; B=24 random base points, pair table = set of x(+-F_i+-F_j); 32 random step points; 64 walks, 400 lockstep steps (x-dependent step choice). Arms: base (one pow(.,-1,p) per add, cost 1I+3M), batched (W=64, 1I + 6M per add), base_AA, batch1. Seeds 101,202,303 (seed*1000+bits). Cost about 10 s total, under 50 MB, one core.

## 5. Classification
runnable_local: a stdlib toy decides the counted-cost question. Smoke test: `python3 -I run.py --smoke`.

## 6. What a run cannot establish
- Op counts are a model: Python wall time is dominated by interpreter overhead and is reported, not decided on.
- Spread <= 0.05 is near-trivial by construction (same formula per add); it checks no exponent change only in the model, not real pipeline stages.
- No pair-table build, generator, LA filter, planner, native Montgomery u64, or 20-bit holdout is tested; no replay certificate or matched rho (section 5a) is produced.
- Results are [pcc:unverified]; relabelling and accounting changes are not improvements.
