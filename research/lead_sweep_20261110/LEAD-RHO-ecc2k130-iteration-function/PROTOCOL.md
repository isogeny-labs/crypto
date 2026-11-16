# LEAD-RHO-ecc2k130-iteration-function

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (2026-11-01 unchanged); last_reviewed 2026-10-24; family: generic.
- Title: ECC2K-130 iteration function: sigma walk vs v3 table walk vs class-signed sigma steps.
- Next step (truncated excerpt): "Measure the v3 table-walk kernel's paired rate on the RTX PRO 6000 against the fused sigma-table".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines ~250-266, id and title only).
  Other hits (atlas_tally.json, ATLAS_INPUTS.json) are id listings, not read in depth. Nothing in cryptanalysis.
  The kernel material itself was not found; the mechanism below is from general knowledge.

## 2. Hypothesis
The GPU rate comparison itself cannot be done here. The locally decidable part is the iteration-count side
of the trade: a walk on orbits of an order-m automorphism (Frobenius sigma, or sign for m=2) with a
16-entry additive table needs mean steps-to-first-collision of about sqrt(pi n / (2m)). That is, the
ratio to the plain table walk (m=1) is 1/sqrt(m) within 15%, for m=2 and m=8, at toy n ~ 2^18.
Fruitless-cycle effects are the known risk; the claim is that they do not break 1/sqrt(m) at r=16.

## 3. Preregistered prediction and falsification criterion
- Statistic: ratio of mean steps-to-first-revisit (class walk / plain walk), m=2 and m=8.
- Threshold: both |ratio / (1/sqrt(m)) - 1| <= 0.15 gives consistent_with_prediction.
  Either outside 0.15 (two-sided) gives falsified: class reduction does not deliver sqrt(m) with an
  unmitigated 16-table, so a class-signed sigma step needs cycle handling (larger table, cycle escape),
  which is a cost that must enter the paired-rate comparison.
- Positive control: plain walk m=1 must be within 25% of sqrt(pi n/2) (random-mapping theory).
- A/A arm: plain walk, new seed; must have ratio within 10% of 1.
- Null/negative control: the plain m=1 walk is the null (no symmetry, ratio 1 by definition).
- If positive control or A/A fails: verdict inconclusive.

## 4. Method
Group Z_n, n prime with n = 1 mod 8, n >= 2^18 (n=262297 region). Point = element of Z_n; step =
x -> canon(x + T[h(x)]), h = Knuth multiplicative hash mod 16, T random 16-entry table, canon =
min over orbit {g^k x} for g of order m (m in {1,2,8}). Each trial: random start, store all visited
classes in a dict, stop at first revisit (includes fruitless cycles). 1200 trials per arm, 4 arms.
Seeds: 1001 (m1), 1002 (m2), 1008 (m8), 7777 (A/A). Python stdlib only; about 20-60 s, under 50 MB RAM.
Smoke: n~4096, 40 trials.

## 5. Classification: runnable_local
The GPU measurement is blocked here, but a stdlib toy decides a sharpened sub-question
(step-count side of sigma/class-signed walks), so runnable_local.

## 6. What a run cannot establish
Nothing about GPU kernel rate, v3 vs fused sigma-table, memory behaviour, ECC2K-130 field arithmetic,
or distinguished-point overhead. Additive toy over Z_n with a multiplicative automorphism is only an
analogue of Frobenius orbits on a Koblitz curve; no real sigma action or negation map is used.
Results are labelled [pcc:unverified] and are not a lead-atlas disposition change.
