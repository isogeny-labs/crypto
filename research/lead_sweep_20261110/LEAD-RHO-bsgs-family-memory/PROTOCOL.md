# LEAD-RHO-bsgs-family-memory

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_known (last_reviewed 2026-10-24). Family: generic.
- Title: Baby-step giant-step family (textbook, interleaved, negation, paired, signed-Frobenius claw).
- Next-step excerpt (truncated): "Revisit when: A BSGS-type schedule with o(sqrt r) memory and BSGS-level operations (would contra..." (presumably "contradict the generic lower bound / the memory-time tradeoff").
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 63-67, 315-321: records claw.pair_table 2.20x [1.90,2.55] of strong rho, bsgs.negation alpha 0.529 [0.485,0.596], Shoup/Nechaev bound);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (tally row only, no new content).
- No cryptanalysis checkout search was done; the atlas YAML is encrypted here.

## 2. Hypothesis
The recorded obstruction is the generic memory-time tradeoff: for BSGS in a generic group of prime order n with table size m = n^b (b < 1/2), group operations scale as n^(1-b) (log-log slope 1-b, so 2/3 at b = 1/3 and 3/4 at b = 1/4), not as n^(1/2). Only a low-memory method (rho with distinguished points) keeps slope near 1/2. So no BSGS schedule with o(sqrt n) memory reaches BSGS-level operation counts, and the closed disposition stands at toy scale.

## 3. Preregistered prediction and falsification criterion
- Statistic: least-squares slope of log2(mean counted group additions) against log2(n) over six sizes (n = first prime above 2^12, 2^14, ..., 2^22), 40 random targets per size, bootstrap 95% CI (200 resamples over targets).
- Prediction: BSGS at m = n^(1/3) has slope in 2/3 +- 0.07 with CI lower bound > 0.58, and exceeds the rho slope by more than 0.08.
- Falsified if the m = n^(1/3) slope CI upper bound is < 0.58 (a sub-sqrt-memory schedule with near-sqrt operation scaling).
- Inconclusive if controls fail or neither criterion is met.
- Positive control: BSGS at m = sqrt n must give slope 0.5 +- 0.06 (detector sees the known sqrt scaling). Also an m = n^(1/4) arm (expected 3/4) as a dose-response check, reported but not decisive.
- Negative control / null: rho with distinguished points (memory about n^(1/2)/32 stored DPs, no table of size sqrt n) is the low-memory sqrt-time reference; it is the null for "memory reduction costs time".
- A/A arm: second positive-control run with a different seed; slope difference must be < 0.03, else inconclusive.

## 4. Method
Model: generic additive group Z_n, generator 1, the solver uses only addition, equality and hashing; cost = number of group additions (baby steps + giant steps for BSGS, walk steps for rho). Seeds: base 20261110, arm offsets +1..+5. BSGS: m baby steps stored in a dict, giant steps of -m until a hit; solution verified by assertion. Rho: 16-adding walk, 8 interleaved walks, DP mask about sqrt(n)/32, walk restart after 20*mask steps, solve from a DP collision with invertible b-difference, accepted only if equal to the target. Cost: under 1 minute total, memory under 20 MB, standard library only. Estimated 0.5-2 minutes.

## 5. Classification
runnable_local. A stdlib toy decides the slope question directly; it re-derives the recorded tradeoff rather than the elliptic-curve constants.

## 6. What a run cannot establish
- It does not test the lead's real content (negation, pairing, signed-Frobenius claw tables on prime-field elliptic curves, constants like 2.20x); it uses an abstract additive group where logs are trivial, so it measures only generic operation counts.
- Toy sizes (2^12..2^22) give slopes with finite-size bias; the rho slope in particular carries DP-startup overhead.
- It cannot rule out non-generic or non-BSGS schedules, and says nothing about the Shoup lower bound beyond consistency.
- Results are labelled [pcc:unverified]; no replay certificate, matched-rho band (section 5a) or VALIDATION.json exists, so nothing here is a leaderboard row.
