# LEAD-TRF-prime-field-isogeny-to-weak

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_obstructed (atlas_tally_20261101; last reviewed 2026-10-24). Family: transfer.
- Title: Isogeny transfer of a prime-field ECDLP to a weaker curve (anomalous, low embedding degree, descent-friendly) in the same class.
- next_step_excerpt: empty in leads.json; next_step_cost and why_it_might_beat_sqrt_p not recorded for this entry.
- Recorded obstruction (atlas_survey.md A4): "closed_obstructed theorem, Tate; special set empty at every size; rho_special=0 at 20/24/28 bits EXP-IT-001".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 58 and neighbours); /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (entry only). No other hits; cryptanalysis not searched beyond that. The EXP-IT-001 record itself was not read.

## 2. Hypothesis
Over F_p, isogenous curves have equal group order (Tate), so any order-and-p-determined weakness (anomalous N=p, embedding degree k<=6 via the largest prime factor of N) is constant on an isogeny class. Hence for every sampled curve E and every curve E' reached by a chain of F_p-rational 2-isogenies, #E'(F_p)=#E(F_p) and weak status(E')=weak status(E): isogeny cannot move an instance from a non-weak to a weak curve. (Descent-friendliness is not order-determined and is not tested here.)

## 3. Preregistered prediction and falsification criterion
- Statistic: M = number of (E,E') isogenous pairs with #E != #E', plus W = number with differing weak status (anomalous or MOV k<=6 flag).
- Prediction: M = 0 and W = 0 in both seeds. Threshold: any single M>0 or W>0 falsifies the implementation or the theorem's application (a bug in Velu or counting would be found first; treated as "falsified" and inspected). Zero mismatch decides: the recorded obstruction re-derives at toy size; it does not reopen the lead.
- Negative control / null: independent random curves over the same p are not isogenous; their orders should differ (the null for "orders agree by chance").
- Positive control: the same mismatch detector applied to the independent-curve pairs must flag >=50% as order-differing, proving it can see differences. Also weak curves must exist in the sample (anomalous or MOV count >0), so the "special set" is nonempty in the field and only the class is closed.
- A/A arm: identical instrument with a second seed (20261111); must give M=W=0 too.
- Verdict: consistent_with_prediction if M=W=0 in both arms, positive control detects, weak curves exist; falsified if M>0 or W>0; else inconclusive.

## 4. Method
- 12 primes from [1000,1300) (every 6th), group orders ~2^10..2^11. Per prime, 150 random short Weierstrass curves (seed 20261110; A/A seed 20261111).
- For each curve: count #E by Legendre-table sum over x (O(p)); walk 3 steps of random F_p-rational 2-isogenies (Velu: root x0 of cubic, t=3x0^2+a, w=x0 t, a'=a-5t, b'=b-7w; discard singular), recount each step.
- Weak flags: anomalous = (N==p); MOV = largest prime factor l>3 of N has p^k=1 mod l for some k<=6.
- Cost: about 12*150*5*1200 ~ 1e7 basic ops per arm, well under a minute, <20 MB RAM. Smoke: 6 primes near 100.

## 5. Classification: runnable_local
A stdlib toy decides it in seconds; it is a cheap re-derivation of the recorded Tate obstruction.

## 6. What a run cannot establish
- It is a toy check of a theorem already known (Tate); it adds no cryptographic evidence at cryptographic sizes.
- Only 2-isogenies over F_p with a rational 2-torsion point are walked; other degrees are not sampled (the theorem covers them).
- Does not test descent-friendliness or isogenies defined over extensions, nor the recorded EXP-IT-001 rho_special counts.
- All results are labelled [pcc:unverified].
