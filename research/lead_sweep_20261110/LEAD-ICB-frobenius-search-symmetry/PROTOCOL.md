# LEAD-ICB-frobenius-search-symmetry

## 1. Lead as recorded
- Disposition: open_next_step (atlas 2026-11-01 reclassification: closed_obstructed). Family: ic-binary. Last reviewed 2026-10-29.
- Title: Frobenius as a symmetry of the decomposition search: orbit CNF encodings, conjugate attempts, equivariant elimination.
- Next-step excerpt (truncated in leads.json): "A genuine stabilizer of a single target R in the descended system (not a conjugation), or an equ..."
- next_step_cost and why_it_might_beat_sqrt_p: empty in leads.json.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (only the one-line listing at line 208). Other hits (atlas_tally.json, candidate_matches.json) are tallies and were not read. Nothing in cryptanalysis was searched beyond this; work is from the entry plus the known mechanism.

## 2. Hypothesis
On a Koblitz curve E: y^2+xy=x^3+x^2+1 over F_{2^n}, Frobenius tau acts on decomposition searches only as a conjugation, not as a stabilizer of a single target: (a) for a factor base (FB) that is a union of tau-orbits, the number N(R) of 2-point decompositions of R equals N(tau R) exactly; (b) for an unstructured FB of equal size, N(R) and N(tau R) differ no less than N(R) and N(S) for a random S (ratio 0.9 to 1.1), i.e. tau gives no extra per-target success; (c) fewer than 1% of points are tau-fixed (a nontrivial stabilizer), so the only gain is the known factor-n relation multiplicity, not an exponent change.

## 3. Preregistered prediction and falsification criterion
- Statistic: T = mean over R of |N(R)-N(tau R)| for the random FB, divided by the same mean with tau R replaced by a random permutation partner S (tau_over_randompair); plus the stable-FB mean abs difference; plus the fixed-point fraction.
- Prediction: stable-FB difference == 0 exactly; T in [0.9,1.1] for both n; fixed fraction < 0.01 (for n = 11, 13).
- Decisions: consistent_with_prediction if all hold; falsified if the positive control passes and T < 0.8 or T > 1.25 for any n (tau would then carry non-conjugation structure for random FBs); inconclusive otherwise or if the positive control fails.
- Negative control / null: random FB, random-permutation partner S (same estimator, no tau).
- Positive control: tau-stable FB, where invariance is a theorem, so the detector must read exactly 0; failure marks the run inconclusive (implementation fault).
- A/A arm: the random-pair statistic is itself an independent random-vs-random comparison; a second shuffle with other seeds is the noise floor (spread across 5 seeds per n is reported via per-n means; T far from 1 beyond 0.8/1.25 is required to falsify).

## 4. Method
- Pure stdlib. GF(2^n) with polys x^11+x^2+1 (n=11), x^13+x^4+x^3+x+1 (n=13); exp/log tables found by searching a primitive generator. Points enumerated by the substitution y = x z and z^2+z lookup. Affine group law with explicit inverses.
- FB_stable: union of 40 Frobenius orbits of random points (about 440 / 520 points). FB_random: uniform random point set of the same size. N(R) counts unordered pairs {P,Q} from the FB with P+Q=R (all pairs i<=j, about 10^5).
- Seeds: n=11 {101..105}, n=13 {201..205}; random.Random per seed. Smoke: n=7, 4 orbits, seeds {1,2}.
- Cost: well under 1 minute and under 50 MB expected (tables 2^14 entries); limit 10 min / 200 MB.

## 5. Classification
runnable_local. A stdlib toy decides the toy-scale statement directly; the lead's real next step (a stabilizer inside a descended Semaev/Weil system) needs algebra engines and is not run here.

## 6. What a run cannot establish
- It tests only the group-level conjugation structure of 2-point decompositions on toy Koblitz curves at group orders ~2^11 to 2^13, not an index-calculus attack on a descended polynomial system, a SAT/CNF encoding, or equivariant elimination cost.
- A pass does not close the lead for m >= 3 decompositions, non-Koblitz curves, or non-orbit factor bases; a fail would not show an exponent gain.
- Nothing here is compared against a matched rho at 0, 0.1 and 1 GAE, so no cross-method claim follows. Results are [pcc:unverified].
