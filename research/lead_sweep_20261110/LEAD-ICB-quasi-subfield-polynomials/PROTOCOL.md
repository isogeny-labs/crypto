# LEAD-ICB-quasi-subfield-polynomials

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- id: LEAD-ICB-quasi-subfield-polynomials; family ic-binary; leads.json disposition `open_next_step`
  (disposition_20261101 = `closed_obstructed`; last_reviewed 2026-10-24; source atlas_survey_20261107 --pursuit).
- Title: Quasi-subfield-polynomial factor bases (Huang-Kosters-Petit-Yeo-Yun; Euler-Petit) at prime extension degree, ECC2K-130.
- Next-step excerpt (truncated in source): "A non-linearized or correspondence-defined QSP family (IDEA-20260916-a17f43) whose root count es..."
- next_step_cost / why_it_might_beat_sqrt_p: absent (empty) in leads.json.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (one-line entry only);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/candidate_matches.json (title/score only).
  No lead-specific note was found; the mechanism below is from general knowledge.

## 2. Hypothesis
The recorded obstruction is that at prime n the only Frobenius-compatible factor bases are trivial, and a sparse
non-linearized QSP f(x)=x^(2^k+1)+a x+b cannot have a root set of size ~2^k. Exact claim: for n prime and k in {2,3}
(gcd(k,n)=1), every f over F_{2^n} has at most 3 roots (Bluher), so root count never approaches the 2^k needed
for a subspace-like factor base, and its distribution matches that of random same-degree polynomials (mean ~1).

## 3. Preregistered prediction and falsification criterion
- Statistic: max and mean root count over N=1500 random (a,b) per cell, n in {11,13}, k in {2,3}.
- Prediction: max <= 3 in every cell; mean in [0.7,1.3]; A/A (first vs second half of the sample) means differ < 0.35.
- Falsified: any f with > 3 roots at prime n (would contradict the obstruction). Inconclusive: A/A gap >= 0.35 or the
  positive control fails. Otherwise consistent_with_prediction (only confirms the obstruction, not a speedup).
- Null: random dense polynomials of degree 2^k+1 (N/3 per cell), root-count histogram reported.
- Positive control: n=12, k=2 (gcd=2), 3000 samples; must reach 5 roots (Bluher), proving the counter can see large root sets.
- A/A: split-half of the QSP sample, same instrument and seed stream.

## 4. Method
Pure stdlib. Build GF(2^n) via brute-forced primitive polynomial and exp/log tables; count roots by exhaustive
evaluation over all field elements. Seeds: 20261110+n (cells), +99 (positive control). Sizes n=11,13 (and 12 for control).
Cost: about 6000 QSP evaluations x 8k elements plus the dense null (~1000 polys x 8k x degree<=9): estimated 3-8 minutes, <50 MB.
`--smoke` uses n=7, N=40, control n=6.

## 5. Classification
runnable_local: a stdlib toy re-derives the recorded obstruction (root-count ceiling at prime n) exactly and cheaply.

## 6. What a run cannot establish
It does not test correspondence-defined or other non-sparse QSP families, ECC2K-130 sizes, or any relation-phase cost
or speedup over rho. It only checks one trinomial family at toy n, and the Bluher bound is a theorem, so a pass is a
sanity check of implementation, not new evidence. Results are labelled [pcc:unverified].
