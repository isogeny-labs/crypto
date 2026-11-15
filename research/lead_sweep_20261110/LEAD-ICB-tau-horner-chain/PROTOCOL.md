# LEAD-ICB-tau-horner-chain

## 1. Lead as recorded
- Disposition: open_next_step (last reviewed 2026-10-24). Family: ic-binary. Source: atlas_survey_20261107 --pursuit.
- Title: tau-Horner decomposition chain on Koblitz curves (R = tau^{m-1}P_1 + ... + P_m over one base).
- Next-step excerpt (truncated in leads.json): "Run IDEA-20261018-f2cd4d: cost per distinct relation of the plain, symmetry-broken and tau-Horne..." (rest not available here).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 218 only, a one-line listing);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (title/disposition only).
  No note with the IDEA-20261018-f2cd4d body was found; the mechanism below is from the title and general knowledge.

## 2. Hypothesis
Let E: y^2+xy=x^3+x^2+1 over GF(2^n), tau the Frobenius endomorphism, and F a factor base closed under tau. Then the Horner chain tau^2 P1 + tau P2 + P3 = R (P_i in F) yields exactly the same set of distinct factor-base relations (as multisets of F elements) as the plain sum P1+P2+P3 = R: the chain is a relabelling of plain relations. Its ordered enumeration costs about twice the group additions per distinct relation (|F|^2 ordered pairs vs |F|(|F|+1)/2 unordered). So the chain gives no gain in cost per distinct relation when F is tau-closed, and loses relations when F is not tau-closed.

## 3. Preregistered prediction and falsification criterion
- Statistic: ratio = D_horner / D_plain, summed over 40 random targets R, where D_* is the number of distinct relations (sorted index triples over F; Horner counted only when tau^2P1 and tauP2 lie in F, i.e. FB-expressible).
- Prediction: ratio == 1.0 exactly in the tau-closed arm and in the A/A arm; adds_per_distinct_horner / adds_per_distinct_plain in [1.5, 2.5].
- Falsified if the closed-arm or A/A ratio differs from 1.0 (a bijection argument says any difference is a bug or a hole in the argument). Consistent if both are exactly 1.0 and the control passes.
- Negative control / null: same size F drawn at random (not tau-closed); expected ratio < 0.5 (roughly (|F|/N)^2 of tuples survive expressibility). This shows the detector can see a loss of relations.
- Positive control: the same control arm doubles as the sensitivity check (a detector that returns 1.0 on a non-closed FB would be broken). No separate positive effect exists, because the claim is a null/identity.
- A/A arm: tau-closed arm re-run with an independent seed (targets and base points differ); must also give ratio 1.0.
- Inconclusive: D_plain < 50 in either closed arm, or control ratio not < 0.5 with D_plain > 0.
- Decides: consistent means the chain is not an improvement by itself for tau-closed F (relabelling, not a gain, under the owner's success rule); it leaves open whether the lead's real gain is elsewhere (e.g. symmetry-broken weighting, larger m, or sieving over the orbit structure).

## 4. Method
- Field GF(2^13), poly 0x201B (primitive; checked by the code). Curve a=1, b=1, all affine points enumerated by half-trace (n odd). #E about 2^13. Smoke: n=7, poly 0x83.
- F (closed arm): 6 full tau-orbits of random points (78 points, none negated-closed). Control: 78 random points. m=3.
- Plain: for i<=j, k = index of R - Fi - Fj if k >= j. Horner: for all ordered (i,j), C = R - tau^2 Fi - tau Fj, in F => relation {tau^2 Fi, tau Fj, C}.
- Seeds: 20261110 (closed), 20261111 (A/A), 20261112 (control); Python random.Random; 40 targets uniform over the point list.
- Cost: about 40*(78^2*1.5)*2 group additions per arm, a few seconds to under a minute total, memory a few MB (field tables 2^14 entries).
- Output: one JSON object with counts, per-target pairs, adds per distinct relation, and "verdict".

## 5. Classification
runnable_local. A stdlib toy decides the identity claim directly. run_cmd is `cd .../LEAD-ICB-tau-horner-chain && timeout 660 python3 -I run.py`.

## 6. What a run cannot establish
- Nothing about Koblitz curves at n = 61..83, about the real factor bases (trace-zero, subfield), or about asymptotic exponent.
- It does not test the lead's symmetry-broken arm (omitted: negation not modelled) or m > 3, and it counts group additions only, not Semaev/linear-algebra costs or the Section 5a native-work pricing band.
- The identity is partly tautological by design; the run checks code and argument consistency and the control's sensitivity, not an unforeseen gain.
- All results are [pcc:unverified] toy measurements, not an improvement over the index-calculus baseline.
