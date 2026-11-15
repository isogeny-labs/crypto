# LEAD-ICP-isogeny-transported-relation-yield

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: open_blocked (reviewed 2026-10-30); family: ic-prime
- title: Isogeny-transported or volcano-level factor bases and the arity-3 relation oracle (crater vs floor)
- next step (truncated): "Finish the 17 missing arms of EXP-ECTD-0cf008 (32,768 targets each) after the recovery amendment"; cost: medium (17 arms x 32,768 targets)
- why it might beat sqrt(p): only a coverage/variance excess growing with field size; mean yield is class-invariant.
- Recorded evidence: conductor-gap IC floor/crater 0.975 [0.603,1.451] (ell=8209), 1.127 [0.922,1.342] (ell=131101): no separation; only 3 of 20 arms valid.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (items 14, 139-146);
  /home/user/crypto/research/prime_field_leads_20261031/C2_volcano/atlas_update.md.
  The 17 missing EXP-ECTD-0cf008 arms need a launcher/records absent here; not rerun.

## 2. Hypothesis
Over prime fields p = 1009..16001 (group orders ~2^10..2^14), for an isogeny class (fixed trace, hence one
cyclic group Z_N), the pair (m=2) sumset coverage of an x-ordered point factor base of size B = round(1.2 sqrt N)
has (a) mean equal to that of a matched random log-set of the same size (class-invariant mean yield) and (b)
variance across the distinct-j curves of the class NOT exceeding the matched-null variance, with no growth of
the variance ratio with p. A coverage/variance excess growing with p would be the only thing that matters for the lead.

## 3. Preregistered prediction and falsification criterion
- Statistics per p: z_mean = (mean cov_A - mean cov_null)/SE; var_ratio = Var(cov_A)/Var(cov_null) with 400-resample
  bootstrap 95% CI. cov = |{F_i+F_j mod N, i<=j}|/N.
- Prediction: |z_mean| < 4 and var_ratio in [0.4, 2.5] at every p (consistent_with_prediction).
- Falsified if |z_mean| >= 4 at any p, or the CI lower bound of var_ratio > 1 at both of the two largest p with the
  larger-p ratio exceeding the smallest-p ratio (excess growing with size). That would justify spending the 17 arms.
- Null / negative control: random B-subsets of Z_N\{0}, same N and B as each curve, 5 draws per curve.
- Positive control: factor base with half an arithmetic progression of logs; requires z < -3 at every p.
- A/A arm: odd vs even curve indices within the class; requires |z_AA| < 3.5 at every p.
- If a control fails, or fewer than 3 primes finish, the verdict is inconclusive.

## 4. Method
For each p in [1009, 2003, 4001, 8009, 16001] (seed 20261110, Random(seed*1000003+p)): enumerate j in 2..p-1
(skip 0,1728), curve a=3c, b=2c(1728-j), c=j(1728-j); count N = p+1+sum chi by Legendre table;
group curves by N (same trace = isogenous, j distinct = non-isomorphic); take the largest class, keep up to 24 curves
with a point of full order N (cyclic). Compute logs of all multiples of the generator, sort by x, FB = first B logs
(includes +-y pairs). Coverage by bytearray sumset. Cost: dominated by p^2 counting (about 2.6e8 steps at p=16001),
a few minutes; time guard skips the largest p when elapsed exceeds the budget. Limits: stdlib only, 1 core, <200 MB, <10 min (timeout 660).

## 5. Classification: runnable_local
A stdlib toy decides the sharpened question (does an isogeny-class x-interval factor base show any coverage mean
shift or variance excess vs matched null at toy sizes). The recorded next step itself (17 EXP-ECTD arms) is blocked.

## 6. What a run cannot establish
- It does not finish EXP-ECTD-0cf008, does not test arity 3, volcano level (crater vs floor is not labelled here;
  curves of one class are unlabelled by endomorphism ring), real solve costs or p >= 2^20; results are [pcc:unverified].
- Within a class all groups are isomorphic to Z_N, so only the x-coordinate (geometric) basis choice can differ; a null
  result says nothing about other transported bases. Few curves per class (<=24) limit variance power; no GAE pricing vs rho.
