# LEAD-RHO-lhw-preprocessing-frontier

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_unexamined (last reviewed 2026-10-24); family: generic; source: atlas_survey_20261107 --pursuit.
- Title: "Low-weight DLP with preprocessing: the (P, S, T, epsilon, U) frontier".
- Next-step excerpt (truncated in leads.json): "Price the two incumbent rows of IDEA-20260930-79abd9 (Coppersmith-split advice tables and a weig..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 67, 279: lead listed among "LHW low-weight leads ... not generic-key leads"; no further detail).
  Other hit (not read): research/ecdlp_subexp_desk_20261101/data/atlas_tally.json. No cryptanalysis hits searched beyond this; the IDEA row text was not found, so the "weighted" incumbent row is unknown and not modelled.

## 2. Hypothesis
For a key x drawn uniformly from weight-w n-bit integers (n=30, w=8) in a prime-order subgroup of Z_P^* (q ~ 2^31), the Coppersmith-split attack with preprocessing (advice table of S left-half weight-w/2 vectors, online scan of T right-half vectors, group ops only) has success probability eps = p_split * (S/L) * (T/R), exactly, where L=R=C(15,4)=1365 and p_split = C(15,4)^2/C(30,8) = 0.3183. I.e. the incumbent row's frontier is the product law eps = p_split*S*T/(L*R), with no hidden slack from collisions or false positives.

## 3. Preregistered prediction and falsification criterion
- Statistic: chi2 = sum over the 9 cells (S/L, T/R in {0.25,0.5,1}) of z^2, z = (obs - 1500*eps_pred)/sqrt(1500*eps_pred*(1-eps_pred)).
- Threshold: chi2 <= 9 + 4*sqrt(18) (about 26.0) in the main arm AND in the A/A arm.
- Outcomes: both under threshold = consistent_with_prediction (frontier is priced exactly as the product law); both over = falsified (the pricing of this row needs a correction); mixed or any control failure = inconclusive.
- Negative control: 600 keys of weight w+2 against the full table and full scan; recovery count must be 0 (null: attack cannot find keys outside the model).
- Positive control: S=L, T=R cell must recover at rate p_split within 4 binomial sigma (detector can see the effect). Zero wrong recoveries (checked against g^found = h and found == x).
- A/A arm: identical protocol, different seed (777 vs 20261110); both must pass independently, giving the noise floor of chi2.
- Descriptive only: online T per cell versus rho reference sqrt(pi*q/4) ~ 4.1e4 group ops (not a decision statistic; no section 5a pricing done).

## 4. Method
- Group: q = first prime >= 2^31+1; P = k*q+1 smallest prime (k even); g = r^k, r from Random(1). Python stdlib only, one core.
- Left vectors: weight-4 subsets of bits 0..14; right vectors: weight-4 subsets of bits 15..29. Table maps g^a to a for all L; "advice" = random subset of S indices chosen per trial (random S-subset, modelling table choice independent of the key).
- Per trial (1500 per cell, 9 cells): draw key by Random(seed).sample of 8 of 30 positions, h=g^x, scan T random right vectors computing h*g^{-b}, look up in table, accept if in advice set; verify.
- Seeds: 20261110 (main), 777 (A/A), 1 (group generator), 5 (negative control).
- Cost: about 2 arms x 9 cells x 1500 x <=1365 modmults, tens of seconds, < 50 MB.
- `python3 -I run.py --smoke` uses n=16, w=4, 60 trials (tens of ms) and always reports "inconclusive".

## 5. Classification
runnable_local. A stdlib toy can check the product-law pricing of the Coppersmith-split row directly. The "weighted" incumbent row could not be identified from the readable material, so it is not tested.

## 6. What a run cannot establish
- The statement is close to an identity (uniform key, independent random table); a pass confirms the implementation and the accounting, not a new frontier point or any improvement over the index-calculus baseline.
- Advice tables here are random subsets, not optimised (e.g. weighted or cover-based selection), and preprocessing cost P and storage cost U are not priced.
- Toy size only (q ~ 2^31, M = C(30,8) ~ 5.9e6); no extrapolation to cryptographic key sizes; no section 5a band, replay certificate or matched rho; no extra tie to the unseen IDEA-20260930-79abd9 text.
- Results are labelled [pcc:unverified].
