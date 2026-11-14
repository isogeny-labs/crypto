# LEAD-EXO-lossy-object-propagation

## 1. Lead as recorded
- disposition: open_next_step (reviewed 2026-10-24); family: exotic; source: atlas_survey_20261107 --pursuit.
- title: Lossy tracked objects propagated through scalar multiplication (doubling coordinates, rigidity, belief propagation).
- next-step excerpt (truncated in leads.json): "Run the disjoint projection panel of IDEA-20260817-84755b in the probabilistic regime that the r..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; hits in /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (only a one-line list mention, no mechanism text) and research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (id only). IDEA-20260817-84755b was not found in the searched trees, so the panel's exact definition is unknown; this protocol is a stand-in built from the lead title and general mechanism knowledge.

## 2. Hypothesis
Tracking only a lossy projection (t<=3 bits: top bits of x, low bits of x, or top bits of x with parity of y) of a point Q on E(F_p) carries no information through one step of the group law: the mutual information I(proj(Q); proj(2Q)) over random Q, p in 2^16..2^22, equals that of independent variables (permutation null) for all three projections. Hence belief propagation over a double-and-add chain with such lossy objects has nothing to propagate beyond one step.

## 3. Preregistered prediction and falsification criterion
- Statistic: z = (MI_obs - mean of permutation null) / sd of null, per (prime, curve, projection) cell; summary = max z over cells.
- Prediction: max z <= 4.0 (consistent). Falsified if max z > 4.0 while controls pass: a lossy projection then transmits information through doubling and the panel is worth extending to chains. Inconclusive if any control fails.
- Negative control / null: row-shuffled pairs (30 permutations per cell), plus fully independent uniform 8-state pairs (|z| < 3.5 required).
- Positive control: lossy tracking over Z_N, top3(k) vs top3(2k mod N), a closed structure; must give z > 20.
- A/A arm: same instrument on the two halves of each sample; mean |MI_half1 - MI_half2| in null-sd units must be < 4.0 (halves have sd about 2x the full-sample null sd).

## 4. Method
Seed 20261110 (python random.Random). For bit sizes 16, 18, 20, 22, three curves each: random prime p = 3 mod 4, random a, b with nonzero discriminant, 40000 random points (x, y via sqrt pow((p+1)/4)), each doubled by the affine formula. Projections applied to Q and 2Q with the same function (8 states). Plug-in MI (nats) with 8x8 table. Cost: about 12 curves x 3 projections x 31 MI evaluations, under 2 minutes on one core, a few MB RAM. Stdlib only.

## 5. Classification
runnable_local. A stdlib toy decides the one-step transmission question exactly enough to sharpen the lead.

## 6. What a run cannot establish
It tests only one doubling step and three fixed projections, not the lead's actual disjoint projection panel (definition unavailable) nor multi-step belief propagation or rigidity. A null does not exclude richer projections, structured curves (CM, small embedding degree) or larger p; weak-class instruments elsewhere in the atlas are known to be blind. It says nothing about beating rho. Results are labelled [pcc:unverified].
