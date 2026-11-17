# LEAD-ICP-prym-witness-relation-diversity

## 1. Lead as recorded
- disposition: open_blocked; family: ic-prime; title: (empty in leads.json)
- next-step excerpt (truncated): "Locate the measured-experiment and relation-diversity records (the owner's brief cites 4,587,520 ..."
- cost: "run nothing" until records located; why it might beat sqrt p: "Prym witnesses as relations; duplication caps rank (13/15, 9/15)".
- atlas_survey.md item 13: owner's brief cites 4,587,520 attempts, 97 and 67 accepted witnesses reducing to 13 and 9 distinct equations (ranks 13/15, 9/15); none found on any merged main or open PR (2026-11-06). Also: "until located and a mechanism for attaching a prescribed target Q exists, run nothing."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 27, 93, 296 only);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/README.md (line 162 only).
  /home/user/cryptanalysis: no hits.

## 2. Hypothesis
The recorded rank cap (97 -> 13 and 67 -> 9 distinct equations, K = 15) is a property of a
support-limited witness generator, not of the group or factor-base size: in a generic toy
(cyclic group Z_Q, Q = 4093, K = 15 random factor-base elements, relations = differences of
L1<=4 signed combinations with equal sums), the same number of accepted witnesses (A = 97)
yields almost no duplication and full rank 15, and doubling A keeps adding distinct equations;
a capped-support generator saturates exactly at its support S. This tests only the
"duplication caps rank" obstruction, not Prym geometry.

## 3. Preregistered prediction and falsification criterion
- Statistics, over 100 seeds: (a) fraction of seeds with distinct/A >= 0.9 at A = 97;
  (b) fraction with rank mod Q == 15 at A = 97; (c) mean gain in distinct equations from A = 97 to A = 194.
- Prediction: (a) >= 0.95, (b) >= 0.95, (c) >= 50, A/A mean-distinct gap < 3.
  Outcome consistent: generator support (not rank capacity) explains the recorded cap; the lead's
  recorded obstruction is a witness-source property and any real Prym source must show distinct/A near 1.
- Falsified if (a) < 0.8 or (b) < 0.8: generic harvests at this size also saturate, so the cap says
  nothing about Prym witnesses. Otherwise inconclusive.
- Negative control / null: the generic collision harvest itself (no structure). Positive control:
  capped-support generators with S = 13 and S = 9 (draw 97 with replacement from S random pool
  equations) must give distinct == S and rank <= S in every seed, else verdict inconclusive.
  A/A arm: independent seed stream (base 5000) of the identical generic arm; mean distinct gap vs
  main arm < 3.

## 4. Method
Python stdlib. Q = 4093 (prime), K = 15. Per seed s (random.Random(1000+s), 100 seeds): draw K random
FB elements in Z_Q; enumerate all integer vectors with L1 <= 4 (39,041); bucket by sum mod Q; each
collision gives a relation (difference vector, sign-canonicalised); pool = set of distinct relations
(~2,500). Accepted witnesses = A draws with replacement from the pool (A = 97, 67, 194);
distinct = size of set; rank by Gaussian elimination mod Q. Capped arm: S pool members, 97 draws.
Cost: ~0.1 s per pool, well under 1 minute total, < 100 MB RAM. Smoke: L1 <= 3, 3 seeds.

## 5. Classification
runnable_local. The records are not found, but a stdlib toy decides whether the stated duplication
cap is generator-specific. Prescribed target Q attachment is not modelled.

## 6. What a run cannot establish
It does not use Prym varieties, real curves, the owner's 4,587,520-attempt data or any target Q.
The relation source is a synthetic collision harvest in Z_Q (dlogs trivially known); it cannot show
that Prym witnesses are useful, or that their duplication is or is not of the capped kind. The pool
is itself duplication-prone (shift invariance), so distinct/A is bounded by pool size. Result is
[pcc:unverified].
