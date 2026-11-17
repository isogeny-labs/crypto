# LEAD-EXO-curve-model-representation

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (2026-11-01 desk label: closed_obstructed). Family: exotic. Last reviewed 2026-10-24.
- Title: Alternative curve models and point representations (Edwards, Hessian, Montgomery, Kummer, x-only).
- Next step (truncated): "A coordinate E -> P^1 with summation-polynomial degree below the common degree-2 law (Kummer and ..."
- Cost: cheap if a new coordinate is exhibited. Why it might beat sqrt(p): coordinate with summation degree below 2; none known.
- Recorded evidence: d_reg=2 for Weierstrass and twisted Edwards m=2,3; u=x/y ring D=16 at m=3, 512 at m=4 (Kummer-class law); desk row 8: "models and representations change constants only".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep excerpts, lines 74, 113, 179);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (line 82);
  /home/user/crypto/docs/session-logs/2026-11-01.md (line ~1414).

## 2. Hypothesis
For every nonconstant coordinate f: E -> P^1 tested, the generic fibre size D(f) of the 3-term summation relation
S_3 = {f(-(Q2+Q3)) : f(Q2)=f2, f(Q3)=f3} (the per-variable degree of the summation polynomial) is >= 2 on
E(F_p) toy curves, with equality for x and its Mobius transforms (positive control). No tested coordinate attains D < 2.
This is the recorded obstruction: degree 1 is impossible since genus(E)=1 admits no degree-1 map E -> P^1.

## 3. Preregistered prediction and falsification criterion
- Statistic: mean |S_3| over sampled pairs (f2 != f3, both with generic fibres) per coordinate, curve.
- Prediction: mean = 2.000 exactly for x and Mobius(x) (positive control); A/A (x, different pair-sampling seed) = 2.000;
  null (random 2-to-1 relabelling of points, +-P pairing destroyed) > 3.0 (expected ~3.9); no coordinate with
  >=2 rational preimages in its generic fibre has mean < 2 - 1e-9.
- Falsified: any such coordinate with mean < 2 (a coordinate below the degree-2 law).
  Consistent: no violation and positive/null controls pass. Inconclusive: controls fail.
- Coordinates y, x/y, x+y are injective on F_p-points (generic fibre 1), so their rational fibre statistic is
  degenerate; they are reported as "unscored", not as degree below 2. x^2 is even in x so its rational fibre equals x's (known caveat).
- Consistent outcome decides only: no cheap coordinate is exhibited among those tested; the lead stays at "none known".

## 4. Method
Python stdlib. Primes near 2053, 3001, 4093 (group orders ~2^11..2^12; a, b random from seed 20261110+i, nonsingular, >40 points).
Affine group law; for each coordinate build fibres over all rational points, choose modal fibre size as generic,
sample 1500 pairs (seed 20261110+100+i), compute |S_3|. A/A seed +200, null seed +300 (labels) and +400 (pairs).
Cost: < 1 minute, < 50 MB. Smoke: p=211, 150 pairs.

## 5. Classification: runnable_local
A stdlib toy can verify the degree-2 law and its controls in seconds; it cannot exhibit new coordinates.

## 6. What a run cannot establish
- Only F_p-rational fibres are used; degree over the algebraic closure of non-even coordinates is not measured.
- It does not compute summation-polynomial degrees of regularity at m>=3 (needs Groebner engines) nor attack cost vs rho.
- A consistent result is a toy-size sanity check of a known theorem, not an improvement; results are [pcc:unverified].
