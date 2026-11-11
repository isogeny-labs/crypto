# LEAD-RHO-cheaper-step-primitives

## 1. Lead as recorded
- Disposition: open_unexamined (last reviewed 2026-10-24). Family: generic. Source: atlas_survey_20261107 --pursuit.
- Title: "Cheaper step primitives never measured here: tag tracing on EC, point-halving steps on binary curves".
- Next-step excerpt (truncated in leads.json): "Port a tag-tracing r-adding walk to a 64-bit prime toy curve in research/rho_walk_leads' harness".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (lines 75, 133: next step "ns per useful step paired against the plain step at 8-1024 walks per inversion"; `< 0.95` opens an engineering lead, `>= 1.0` closes it by cost model);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 65-67, 278). The directory research/rho_walk_leads does not exist here, so the harness cannot be reused.
- The binary-curve half (point halving) is out of scope: no GF(2^n) toy here.

## 2. Hypothesis
Under counted field-operation cost (M, S, inversion I in the band 10-40 M, S in {0.8, 1} M), an inversion-free Jacobian mixed-add r-adding walk ("tag tracing" cost model, 8M+3S per step) is NOT cheaper than the Montgomery-batched affine walk (2M+1S + 3M + I/W) at W >= 16 walks per inversion: ratio tag/plain >= 1.0 in every cell. Additionally, a tag that is a function of the unnormalised Jacobian representation is not a function of the point, so such walks do not merge.

## 3. Preregistered prediction and falsification criterion
- Statistic: min over primes, W in {16,64,256,1024}, I in {10,20,40} M, S in {0.8,1.0} M of (tag cost per step)/(plain cost per step), from instrumented op counters.
- Consistent with prediction: min ratio >= 1.0 and all controls pass (the cost model closes the lead, as the desk note predicts).
- Falsified: min ratio < 0.95 with controls passing (opens an engineering lead for pool/GPU kernels). Between 0.95 and 1.0: inconclusive.
- Positive control: two walks from the same point (one rescaled in Jacobian coordinates) with index = function of the normalised point stay in lockstep for 20 steps; rate >= 0.9 (expected 1.0).
- Negative control / null: the same test with index = X mod r of the raw Jacobian coordinate; rate must be <= half the positive rate (expected about 0, since the tag is not point-valued). Also the fraction of points whose X mod r changes under rescaling must exceed 0.8.
- A/A arm: plain batched walk vs plain batched walk with a different seed, same instrument: cost ratio within 1% of 1.
- If any control fails the verdict is "inconclusive".

## 4. Method
Python 3 stdlib. Primes p = 3 mod 4 at or above 2053, 4099, 8209, 16411; random curve y^2=x^3+ax+b (seed 20261110+p), order by naive point count (group order about 2^11..2^14). Table of r = 16 points. Plain walk: Montgomery batch inversion, affine add with counted M/S/I. Tag walk: Jacobian mixed add (8M+3S counted), index from X mod r at zero cost. 6 steps per (W, prime); counts per step are deterministic. Controls as in section 3 on the first prime (150 walks). Cost estimate: under 1 minute, under 50 MB. Seeds: 20261110 (+1, +2 for A/A).

## 5. Classification
runnable_local. A stdlib op-count model decides the cost-model half. The lead's real ns-per-step measurement on a 64-bit curve needs native code and is outside this host.

## 6. What a run cannot establish
- Not wall-clock ns/step: the I/M ratio is modelled, not measured on 64-bit native arithmetic.
- The Cheon-Hong-Kim tag-tracing scheme itself is not reproduced; only the cost model of an inversion-free step with a free tag. Any real tag bookkeeping would add cost, so the model favours the tag arm.
- Toy sizes only (2^11-2^14); no collision search, no L2/replay/matched-rho admission; point halving untested. Results are [pcc:unverified].
