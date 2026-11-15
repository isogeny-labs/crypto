# LEAD-ICB-oakley-quintic-tower

## 1. Lead as recorded
- Disposition: open_next_step (last reviewed 2026-10-24). Family: ic-binary. Source: atlas_survey_20261107 --pursuit.
- Title: Gaudry-Diem n = 5 point decomposition on the Oakley EC2N curves over F_{2^155} and F_{2^185}.
- Next-step excerpt (truncated in the record): "Execute the approved Oakley ladder contracts (EXP-OAKLEY-2c0552, -4276c4, -7f73ef have trial pla..."
- The record has no next_step_cost or why_it_might_beat_sqrt_p text (both empty in leads.json).
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (line 216 only); research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (entry only). No other material found; the EXP-OAKLEY contracts were not located, so this protocol works from the mechanism.

## 2. Hypothesis
For E: y^2+xy=x^3+a*x^2+b over F_{2^(5k)} (k=4, field 2^20, group order about 2^20), with factor base FB = {points with x in the subfield F_{2^k}} (negation-closed), the sums of 5-subsets of FB collide (equal sums) no more often than the sums of 5-subsets of a random negation-closed point set of the same size. I.e. the Gaudry-Diem n=5 heuristic of "FB sums behave like uniform random group elements" holds at toy size, with no extra subfield structure producing excess collisions.

## 3. Preregistered prediction and falsification criterion
- Statistic: R = (collisions among all C(|FB|,5) subset sums for FB) / (mean collisions over 30 random null sets of identical size and negation-closure).
- Prediction: 0.7 <= R <= 1.4 on both curves (a=0,b=1 and a=1,b=1; seeds 20261110, 20261111).
- Outcomes: R in band -> consistent_with_prediction (no structural shortcut/obstruction visible at this size). R outside band, with controls valid -> falsified (the uniformity assumption underlying the cost model fails at toy size; that sharpens, but does not decide, the Oakley run). Controls invalid -> inconclusive.
- Negative control / null: random negation-closed point sets, same size, containing the x=0 point.
- Positive control: set {+-jP, j=1..m} plus x=0 point (arithmetic progression); must give collision mean >= 3x the null mean, else inconclusive.
- A/A arm: two halves (15+15) of the null sets; ratio of their means must lie in [0.6, 1.67], else inconclusive.

## 4. Method
- Pure-stdlib GF(2^20) via log/exp tables (primitive polynomial x^20+x^3+1), half-trace table z^2+z -> z to enumerate curve points; affine add/double/negate in characteristic 2.
- FB: all curve points with x in F_16 (about 17 points). Enumerate all 5-subsets (about 6k), sum, count repeated sums.
- 30 null sets per curve, 6 positive-control sets per curve, fixed seeds as above; deterministic.
- Cost: table build about 5 s, rest well under 1 min per curve; RAM below 200 MB (about 100 MB). Smoke uses k=2 (field 2^10).

## 5. Classification: runnable_local
A stdlib toy decides the structural-collision question. The actual lead step (Oakley ladder contracts over F_{2^155}, F_{2^185}) is not run here.

## 6. What a run cannot establish
- Nothing about F_{2^155}/F_{2^185}: asymptotic cost, Groebner/Semaev solving cost, linear algebra, or any gap to rho.
- Toy collision counts are low (tens), so power is limited; trivial collisions from negation-closure appear in both arms by design.
- It does not execute the approved EXP-OAKLEY contracts. Results are labelled [pcc:unverified]; no leaderboard or improvement claim.
