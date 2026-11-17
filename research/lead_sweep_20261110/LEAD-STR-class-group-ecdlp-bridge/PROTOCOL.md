# LEAD-STR-class-group-ecdlp-bridge

## 1. Lead as recorded
- Disposition: open_next_step (last reviewed 2026-10-24). Family: structure. Source: atlas_survey_20261107 --pursuit.
- Title: Reduction between Cl(O)-vectorization (or CM self-pairings) and ECDLP in the same class.
- Next-step excerpt (truncated): "Write and review the simulation lemma of H-CLGP-63f471 / IDEA-20260807-1b7241. In a curve-aware ..."
- Cost: cheap (simulation lemma, theory). Why it might beat sqrt(p): "Likely closure at exponent 1/2; a failing simulation names the bridge".
- Desk summary (atlas_survey.md l.55; ecdlp_subexp_desk README l.128): in a curve-aware generic group model with free class-group action, ECDLP still needs Omega(sqrt N); h ~ p^(1/2) vs N ~ p.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep excerpts, l.53-66, 255-269);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (grep excerpts, l.90-134). No cryptanalysis hits.

## 2. Hypothesis
In a toy generic model where the instance lives in Z_N (prime N) on each of h curves, and the adversary also has a free oracle
that transports a group element between curves (an exponent-preserving isomorphism, as a prime-to-N isogeny induces), the best
collision-walk cost in oracle queries (group ops + action calls) is not lower than the no-action baseline: cost ratio B/A >= 0.95,
with log-log slope in N between 0.4 and 0.6 (exponent 1/2).

## 3. Preregistered prediction and falsification criterion
- Statistic: R = median(cost_B/sqrt N) / median(cost_A/sqrt N), pooled over N = 2^12..2^22, 120 runs per size per arm, bootstrap 95% CI (400 resamples, seed 99); also log-log slope of median cost vs N for arm B.
- Predict: lower CI bound of R >= 0.95 (expected >= 1, action calls are pure overhead) and slope_B in [0.4,0.6].
- Falsified: upper CI bound of R < 0.95 (the action oracle lowers cost; the simulation lemma would fail in this toy, naming a bridge).
- Null / negative control: arm A (no action oracle). A/A arm: A2 (same algorithm, different seeds); valid only if its R CI contains 1 or lies in [0.9,1.1].
- Positive control: arm C, a planted leak (instance effectively in a subgroup of index 8, cost reported against sqrt N); detector must show R_C upper CI < 0.6 (expected 0.35). If A/A or positive control fails, verdict is inconclusive.
- Decides: consistent supports (does not prove) the lemma's exponent 1/2 closure in a model with identity transport; falsified names the action as a cost source.

## 4. Method
Exponent-tracking collision walk (16 hashed multipliers, blake2b step selection, all points seen stored); k recovered from a collision with
coefficient check. Arm B: with probability 1/2 per step an extra action call moves the walk to another of h = max(2, sqrt(N)/8) curves, cost 1 each. Primes: next prime >= 2^e.
Seeds: A=1, A2=2, B=3, C=4 (run seeds seed*1000003+i), bootstrap 99. Cost: roughly 1.3*sqrt(N) steps per run, 120 runs per size per arm, expected a few minutes total, memory < 20 MB, stdlib only.

## 5. Classification
runnable_local. A stdlib toy can sharpen the question (it checks the cost-neutrality of a transport oracle in the model); the lemma itself is a proof done at the desk.

## 6. What a run cannot establish
- It does not prove the simulation lemma, quantifier order or the h vs N size-gap argument; a toy with exponent-preserving transport is by construction a relabelling.
- It models no real isogeny or class-group structure (no CM curves, no vectorization hardness, no eigenvalue or non-prime-to-N maps).
- One algorithm family (collision walk) is tested; GGM lower bounds quantify over all algorithms.
- Results are labelled [pcc:unverified]; toy sizes only.
