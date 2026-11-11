# LEAD-EXO-cryptoscale-candidate-screen

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: exhausted_at_scale (reviewed 2026-10-29; same on 2026-11-01). Family: exotic.
- Title: "GOAL-CRYPTO-001 discovery screen for a reproducible non-generic crypto-size algorithm".
- next_step_excerpt (truncated in leads.json): "Revisit when: An externally sourced candidate with an explicit constructor arrives, or the scree..."
- Atlas survey wording: "13 screening batches FINAL_NO_CANDIDATE; revisit only with an externally sourced candidate with an explicit constructor."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 72-74, 156-166, 299-313 only);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (lines 53-58). No lead-specific batch records were found; the 13 batches are not available here.
- Not found in cryptanalysis (not searched beyond the crypto hits; the atlas text is the only source).

## 2. Hypothesis
The recorded obstruction is that a screen which declares a "non-generic algorithm candidate" when its cost beats the best generic method by a margin produces no candidate on generic inputs. Claim: at toy scale, a screen with statistic S = median(group ops / sqrt(r)) per arm, threshold "upper 95% bootstrap bound of S_arm / S_bsgs < 0.8", (a) declares NO candidate among generic arms (Pollard rho, A/A replicate rho, Pohlig-Hellman on prime-order curves), and (b) does declare the planted structural weakness (Pohlig-Hellman on smooth-order supersingular groups). That is, the screen is sound (no false positives) and sensitive (finds a known non-generic attack), so FINAL_NO_CANDIDATE on generic curves is informative about the screen, not an artefact of blindness.

## 3. Preregistered prediction and falsification criterion
- Statistic: S_arm = median over instances of (counted point additions/doublings) / sqrt(d), d = point order; CI by 300-resample bootstrap (seeded). Candidate flag: CI_hi(S_arm)/S_bsgs < 0.8.
- Prediction: candidate flag false for rhoA, rhoB, ph_generic; true for ph_smooth_planted.
- Falsified: any generic arm flagged (false positive, screen unsound at this threshold). Inconclusive: positive control not flagged, or any arm has fewer than 3 solved instances. Consistent: otherwise.
- Null / negative control: rhoA, rhoB, ph_generic on prime-order random curves (log2 p = 12, 14, 16). A/A arm: rhoB is an independent-seed replicate of rhoA on the same instances; report rhoB/rhoA median ratio (expected near 1; rho constant is implementation-dependent, ~5-30 at toy size from walk set-up overhead, which is why the reference is bsgs, the best generic arm).
- Positive control: Pohlig-Hellman on y^2=x^3+b over p = 2 mod 3 (order p+1) with p+1 having largest prime factor <= 2^(k/4), k = 28, 32.
- Reference: bsgs (best generic) at the same sizes; both are exact-solve, answers verified by multiplication.

## 4. Method
- Pure Python 3 stdlib, affine arithmetic, one op counted per add/double (inversions not priced separately).
- Generic instances: random Weierstrass curves, p random in [2^k, 1.5*2^k), order n found by point counting (squares table) and kept only if n is prime (n > 2^(k-1)); k in {12,14,16}; 14 instances per k. Random generator G, secret x, Q = xG.
- Planted instances: k in {28, 32}, 14 instances each, no point counting (order is p+1 by construction).
- Arms: rho (16-way adding walk, distinguished-point mask sqrt(d)/8, up to 400 restarts), bsgs, Pohlig-Hellman (digit loop, BSGS per prime factor).
- Seeds: master 20261110 drives one random.Random; arm RNGs derived from it; Python hash seed irrelevant (no sets of floats/strings iterated).
- Cost estimate: well under 1 minute at these sizes (smoke 0.2 s at k=10, 28; full run expected seconds to tens of seconds), under 50 MB, one core.
- Output: one JSON object with per-arm n, median, CI, ratio_to_bsgs, candidate flag, AA ratio and verdict.

## 5. Classification
runnable_local. The recorded obstruction is a screening outcome whose logic (soundness and sensitivity of the screen statistic) can be re-derived by a stdlib toy. This does not re-run the 13 recorded batches.

## 6. What a run cannot establish
- Nothing about crypto-size curves, the unavailable 13 batches, or any externally sourced candidate; those would need the candidate's constructor.
- Toy sizes: constants (rho overhead, PH digit cost) dominate; the 0.8 threshold and the planted k = 28, 32 were tuned during development on tiny pilots (k <= 32, a few instances), so the positive control is by design detectable.
- A pass shows the screen has no false positives against three generic arms and sees one known structural weakness; it does not show the screen sees unknown weaknesses (the atlas notes other structure instruments are blind to known weak classes).
- Operation counts treat one add as one unit (no 0, 0.1, 1 GAE band; no section 5a pricing of unpriced native work), so no cross-method ranking claim is possible.
- All results are labelled [pcc:unverified].
