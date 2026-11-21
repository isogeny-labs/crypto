# LEAD-RHO-non-automorphism-equivalence

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_unexamined (last reviewed 2026-10-24). Family: generic. Source: atlas_survey_20261107 --pursuit.
- Title: Equivalence classes larger than Aut(E): quotients by subgroups H of (Z/N)^*.
- Next step (truncated in leads.json): "Prove or refute H-EWALK-29cb90's per-step canonicalisation bound: show that for h = |H| > 6 on o..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 65-67, 278-281; lists the lead among open constant-factor rho leads); /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (grep hit only). H-EWALK-29cb90 itself was not found; the bound is reconstructed from the mechanism.

## 2. Hypothesis
For a walk on classes {h.x : h in H}, H <= (Z/N)^* of order h > 6 not realised by curve automorphisms, the step count falls by about sqrt(h), but canonicalisation needs h-1 scalar multiplications by generic elements of size ~N (about 1.5 log2 N group operations each). So total cost per solve is above the plain-rho cost for every h > 6 (and every h > 2). Break-even needs image cost c* < 1/(sqrt(h)+1) steps, under 0.27 step for h >= 8, below any scalar multiplication.

## 3. Preregistered prediction and falsification criterion
- Statistic: R_cost(h) = (mean steps(h) / mean steps(1)) * (1 + (h-1)*c_img), c_img = 1.5*log2 N group ops per generic image (1 op = one walk step); also the step ratio R_step(h) vs 1/sqrt(h).
- Prediction: for every h in {8,12,16,24,32}, R_cost > 1 and |R_step/(1/sqrt h) - 1| <= 0.25 (step law holds; fruitless cycles are not removed, so R_step may fall somewhat below 1/sqrt h).
- Falsified if any h > 6 gives R_cost <= 1 or the step law fails by more than 25%. Consistent if neither. Inconclusive if a control fails.
- Positive control: h=2 negation (free automorphism, c_img=0), R_step in (0.6, 0.85) near 0.707.
- Null detector: with free images (cost ratio = R_step) the instrument must show R < 1 for h > 1.
- A/A arm: h=1 reseeded, |ratio-1| < 0.1 (0.3 in smoke).
- Negative control: h=1 (no quotient) and the pure cost-model null (h=1 gives R = 1 by construction).

## 4. Method
Cyclic group Z/N (dlog known, so images h.x are computable by the simulator and priced separately). Prime N >= 2^20 with h | N-1 (2^12 in smoke). H = powers of an element of exact order h (h=2 gives {1,-1}). Class = canonical min over H of h.x mod N. Walk: x <- x + J[hash(canon(x)) mod 64], 64 random jumps, random start. Trial = steps until a class repeats (birthday). 400 trials per arm (20 in smoke), h in {1,2,3,4,6,8,12,16,24,32}, seeds 20261110 (base), +1 (A/A), +100+h (arms). Python 3 stdlib only. Cost: about 1 to 3 minutes, memory under 50 MB, one core.

## 5. Classification: runnable_local
A stdlib toy decides the step law and the break-even arithmetic. Run as: `python3 -I run.py`. Smoke: `python3 -I run.py --smoke`.

## 6. What a run cannot establish
- It uses an abstract Z/N, not elliptic curves, and a prices-per-op model (1.5 log2 N per image) rather than measured field operations. It cannot rule out special H, such as images that are cheap because N | g^h - 1 with small g, or shared work across images (Frobenius-like structure).
- A repeat of a class on one trajectory includes fruitless cycles, so step gains are slightly overstated (about 10% at h=2..8 in a 2^14 pilot). Distinguished points, tail handling and cycle escape are not modelled.
- Toy sizes only (about 2^20). No matched-rho band (section 5a) is applied, and the result is no leaderboard row. Results are [pcc:unverified].
