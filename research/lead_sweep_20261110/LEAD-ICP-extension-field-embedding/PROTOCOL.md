# LEAD-ICP-extension-field-embedding

## 1. Lead as recorded
- id: LEAD-ICP-extension-field-embedding; disposition: open_next_step (disposition_20261101: closed_obstructed); family: ic-prime; last_reviewed 2026-10-24.
- title: "Embedding E(F_p) into E(F_{p^n}) to buy a subfield factor base".
- next_step_excerpt (truncated): "A decomposition over F_{p^n} with exponent below 1/(2n) in |E(F_{p^n})|, i.e. far beyond Gaudry/..."
- Recorded obstruction (atlas_survey A1 item 19): Gaudry cost O~(p^(2-2/n)) vs sqrt(p): p at n=2, p^(4/3) at n=3. Reopen only with exponent < 1/(2n) in |E(F_{p^n})|. Desk README row 7: the embedding buys a subfield factor base for a larger group, not for the original one.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 33, 170, 235); /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (lines 81, 145, 170). Session logs and JSON hits not read. KB (research-kb) not queried in this sub-task.

## 2. Hypothesis
For n=2, prime p = 3 mod 4, ordinary E/F_p and a generator G of (nearly all of) E(F_{p^2}), the counted cost of a Gaudry-style subfield-factor-base index calculus (factor base = the 2p points with x in F_p; 2-decompositions; sparse linear algebra) grows faster than Pollard rho on the same group: the slope of log(IC cost / rho cost) against log|E(F_{p^2})| is >= 0.4 when natives are priced at 0.1 or 1 GAE per unit, so the recorded obstruction (no sub-rho exponent at n=2) holds at toy scale even with a FREE decomposition oracle. Side claim: subfield-FB sums cover the whole group (yield ~1), more than the random-FB Poisson value 1-exp(-lambda)~0.86.

## 3. Preregistered prediction and falsification criterion
- Statistic: slope s_w of ln(ratio_w) vs ln N2, ratio_w = (group ops + w*natives)/rho group ops, w in {0.1, 1}; rho ops measured by Floyd walks (40 trials per size, 16-branch r-adding walk). IC arm: relations needed = p columns, attempts = p/f_sub (one group op each), natives = attempts*2p (oracle enumeration) + 2p^2 (Wiedemann mults). Rho arm has no unpriced natives (stated limit).
- Consistent_with_prediction: min(s_0.1, s_1) >= 0.4 and ratio > 1 at both w for the larger half of sizes. Falsified: min slope < 0.1, or ratio_0.1 < 1 at the largest size. Otherwise inconclusive. w=0 is reported only: there IC < rho by construction (free oracle and LA), so no rank holds across the 0/0.1/1 band and none is claimed (AGENTS section 5a).
- Positive control: f_sub vs the Poisson model (1-exp(-lambda), lambda=|FB|^2/2N) must be visible: random-FB null reproduces the model within 0.03. Negative control / null: 2p random multiples of G as factor base (same size, negation-closed). A/A arm: two rho seed sets (1000+p, 2000+p); mean-step ratio must lie in (0.7, 1.43) at every size, else verdict inconclusive.

## 4. Method
Sizes p in {67,103,151,211,307,431,607} (N2 ~ 2^12..2^18.5). Curve y^2=x^3+Ax+B with A,B from Random(20261110+p); #E(F_p) by Legendre sum, N2 = n1(2p+2-n1); F_{p^2} = F_p[i], i^2=-1. G = sum of one real-y FB point and one imaginary-y (twist) FB point; r = ord(G) by factoring N2, require r >= N2/8. Coverage = |distinct F_i+F_j| / N2. Pure Python affine arithmetic. Cost estimate: coverage ~2*0.74M adds at p=607 plus rho ~ 7*80*1.5k adds; under 3 minutes expected, far below 10 min, memory under 100 MB (set of <=0.74M tuples; plausibly 150 MB worst case at p=607 -- if over, drop 607).

## 5. Classification
runnable_local. A stdlib toy decides the cost-slope question and measures yield. Smoke: `python3 -I run.py --smoke` (primes 67, 83).

## 6. What a run cannot establish
Seven sizes at n=2 only; n=3 (where Gaudry beats rho on the big group) is not tested. The oracle is priced, not implemented (no Weil-restriction Groebner solving). It does not address the lead's real question (embedding E(F_p) subgroup DLP, exponent in |E(F_{p^n})|): G here generates the whole of E(F_{p^2}), not the embedded E(F_p). Rho carries no native pricing. Slopes over 6 doublings are noisy; the slope threshold is a heuristic of mine. All results are [pcc:unverified].
