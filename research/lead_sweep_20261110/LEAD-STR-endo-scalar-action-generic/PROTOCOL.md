# LEAD-STR-endo-scalar-action-generic

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: closed_obstructed (last_reviewed 2026-10-24); family: structure; source: atlas_tally_20261101.
- title: Endomorphism-augmented generic and witness-lattice attacks on a prime-order subgroup.
- next_step_excerpt: empty in leads.json (closed lead; no next step recorded).
- Recorded obstruction (atlas_survey.md): "End(G)=Z/N on prime-order G, GGM-simulable"; non-unit endomorphism orbits are the whole group; ceiling is sqrt|Aut| (sqrt 2, sqrt 4, sqrt 6).
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 52-55, 145-155 via grep context); research/ecdlp_subexp_desk_20261101/README.md (lines 60-66). data/atlas_tally.json was a hit but not read.

## 2. Hypothesis
In a generic cyclic group of prime order N, an endomorphism acts as multiplication by a scalar lambda, so a rho walk on orbit classes under H=<lambda> of order k needs about 1/sqrt(k) of the plain walk's steps (matching the sqrt|Aut| ceiling for k in {2,3,6}), but for a large orbit (k=60, standing in for a non-unit endomorphism whose orbit is large) the per-step canonicalisation costs k group-operation equivalents, so the charged cost ratio to plain rho is > 1: no net gain beyond the unit-group ceiling.

## 3. Preregistered prediction and falsification criterion
- Statistic: mean steps-to-recovery (verified d) over 300 paired trials per arm; step_ratio = arm/baseline; charged_cost_ratio = step_ratio * k.
- Prediction: step_ratio within +-25% (relative) of 1/sqrt(k) for k=2,3,6; charged_cost_ratio(k=60) > 1.
- Falsified if any k in {2,3,6} deviates by more than 25% from 1/sqrt(k), or charged_cost_ratio(k=60) <= 1 (the obstruction's "no gain beyond sqrt|Aut|" would then fail in this model).
- Inconclusive if the controls fail.
- Positive control: k=2 (negation) must show step_ratio < 0.85 (detector sees a known sqrt2 effect).
- A/A arm: plain k=1 with disjoint seeds (900000+); |ratio-1| must be <= 3 combined standard errors + 0.02, else inconclusive.
- Null/negative control: the k=1 baseline (no endomorphism action) is the null; the k=60 arm is the negative control for "free large orbit" since it is charged.

## 4. Method
- Group: Z/N, N the first prime >= 2^17 with N = 1 mod 60 (so subgroups of order 2,3,6,60 of Z_N^* exist). Elements tracked as (a,b), point aP+bQ, label x=a+b*d mod N, d secret random per trial (simulation knows d only to form labels, as a GGM oracle would).
- Walk: 24 additive jumps (random (a_j,b_j)), step index from a hash of the canonical label. Canonical rep = argmin over h in H of h*x mod N; (a,b) multiplied by that h. Restart on in-walk revisit (fruitless cycle) or after 8*sqrt(N) steps; global table across restarts; cross-walk collision with b differing gives d=(a1-a2)/(b2-b1), verified against truth.
- Seeds: trial t uses random.Random(1000+t) for baseline and every arm (paired); A/A uses 900000+t. Smoke: N~1021, 8 trials.
- Cost: baseline ~ 500 steps/trial; k=60 arm ~ 60 mins per step; total well under 1 minute estimated, memory < 20 MB, stdlib only.

## 5. Classification
runnable_local: pure-Python toy decides whether the 1/sqrt(k) scaling and the charging argument hold in the generic model.

## 6. What a run cannot establish
- It runs in an abstract group (GGM-style), not on an elliptic curve; it does not test real endomorphism computation cost or any non-generic structure.
- The k=60 arm's cost model (k ops per canonicalisation) is an assumption, not a measured curve cost; it re-illustrates, not proves, the recorded theorem End(G)=Z/N.
- Toy size 2^17, 300 trials; fruitless-cycle handling is crude, so constants are not matched-rho calibrated (no section 5a band). Result labelled [pcc:unverified].
