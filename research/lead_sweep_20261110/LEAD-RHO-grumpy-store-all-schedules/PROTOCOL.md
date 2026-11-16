# LEAD-RHO-grumpy-store-all-schedules

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-24); family: generic.
- Title: "Exact average-case constant of store-all generic schedules (grumpy giants, compiled tiered walks)".
- Next-step excerpt (truncated): "Run EXP-GRUMPY-eb8675 (certified exhaustive ladder, Z/l from 2^10 to 2^20, covered-set counter a..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; grep hits in
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 67, 251: only a one-line listing among open constant-factor rho leads, no mechanism text) and
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (grep only, not read). Nothing in /home/user/cryptanalysis was searched beyond this. The EXP-GRUMPY-eb8675 spec was not found; the mechanism below is my own toy reading.

## 2. Hypothesis
In the generic group Z/l (points modelled as a+b*x), the exact mean group-operation count, in units of sqrt(l), of store-all schedules obeys single-list (rho-like, any collision with distinct b solves) = sqrt(pi/2) = 1.2533 < optimal-tuned BSGS = sqrt(2) = 1.414 < alternating random tame/wild two-list = sqrt(pi) = 1.7725, and the grumpy schedule (deterministic stored babies 0,1,2,.. interleaved with random stored wilds) is within 5% of the two-list constant. I.e. storing everything only reaches 1.25; no cross-list store-all schedule beats BSGS.

## 3. Preregistered prediction and falsification criterion
- Statistic: mean cost/sqrt(l) per schedule over l in {4093, 65521, 1048573, 4194301} (120-1500 trials each, random secret x).
- Prediction: strict ordering single_list < bsgs_opt < two_list at every l, and |grumpy - two_list| <= 5% of two_list.
- Falsified: ordering violated at any size, or grumpy differs from two_list by more than 5%. Consistent otherwise. Falsification would mean the toy model or implementation is wrong or a grumpy schedule has a different constant, which would reopen the lead.
- Positive control: single_list mean must match sqrt(pi/2) within 4 SE + 3% (birthday theory).
- Null/negative control: closed-form theory values (birthday) serve as the null; BSGS with exact deterministic cost is the known reference. No separate random-label null is applicable (no structure is being detected).
- A/A arm: every schedule re-run with a second seed set; |diff| <= 4 combined SE, else verdict inconclusive (also inconclusive if the positive control fails).

## 4. Method
Pure Python, one core. Point value v = a + b*x mod l with secret x uniform. Schedules: single_list (random a,b; stop at collision with different b), two_list (alternate tame b=0 / wild b=1, random a; cross collisions only), bsgs_opt (n=isqrt(l/2) babies, cost n + x//n + 1), grumpy_det_baby (baby k=0,1,2.. stored alternately with a random wild point; stop on cross hit). Seeds: 1000+10*i+j (arm A), 2000+10*i+j (A/A), i size index, j schedule index. Cost: about 1M dictionary ops at the largest size, well under 1 minute total, < 100 MB RAM (sets up to ~4k entries per trial at 2^22).

## 5. Classification: runnable_local
A stdlib toy decides the constants exactly; no GPU, build or vaulted record needed. This re-derives the constants; it does not run EXP-GRUMPY-eb8675 itself.

## 6. What a run cannot establish
- Nothing about real elliptic curves, tiered/compiled walks, covered-set counters or the real EXP-GRUMPY ladder (Z/l to 2^20 certified exhaustive); it is a model in Z/l with idealised random points.
- Does not price cache/memory cost or native step cost (AGENTS.md 5a band); the toy counts group operations only.
- Four sizes and finite trials give about 1-3% SE; small ordering gaps (grumpy vs two_list) are only tested at the 5% level.
- My reading of "grumpy giants" is an assumption since the lead text was truncated. Results are [pcc:unverified].
