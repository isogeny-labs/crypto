# LEAD-RHO-cheon-aux-acquisition

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_obstructed (last reviewed 2026-10-24; source atlas_tally_20261101). Family: generic.
- Title: "Run Cheon's auxiliary-input algorithm on ordinary ECDLP by generating [x^d]P yourself".
- Next-step excerpt: empty in leads.json (no next step recorded; the lead is closed).
- Recorded obstruction (atlas_survey.md, item 15): producing [x^d]P is square-DH, Omega(sqrt r) generically; on deployed curves E(r) ~ 0.254 needs d-degree supply, "not a vulnerability".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 65-67, 151);
  research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (hit only, id confirmed, not read further).
  Nothing found in /home/user/cryptanalysis docs beyond atlas-input JSON lists (not read).
  Knowledge base (research-kb) not queried in this subagent; no novelty claim is made.

## 2. Hypothesis
For ordinary cyclic groups of prime order r (toy r ~ 2^12..2^20, d | r-1), obtaining [x^d]P from (P, [x]P) by any known generic route costs Theta(sqrt r) group operations, because it is done by solving the DL first. So self-acquired Cheon (acquire aux, then run Cheon) costs strictly more than plain Pollard rho, while Cheon with aux supplied free is cheaper than rho (so the aux, not the algorithm, is the cost).

## 3. Preregistered prediction and falsification criterion
- Statistic S: least-squares slope of log2(mean rho group multiplications) against log2 r over sizes 12,14,16,18,20 bits (40 rho runs per size).
- Prediction: S in [0.45, 0.55]; and the ratio (acquire-by-DL + Cheon) / rho > 1 at every size.
- Falsified if S is outside [0.45, 0.55], or the heuristic null below hits the target more often than its binomial expectation + 5 sd + 3.
- Consistent otherwise, provided controls pass; inconclusive if a control fails.
- Negative control / null: "span heuristic": draw 20*sqrt(r) random elements g^(a+bx) from the known span and test equality with g^(x^2); expected hits = trials/r (about 0), i.e. no linear-span shortcut to acquisition.
- Positive control: Cheon with free aux (g, g^x, g^(x^d)), d ~ 2^(bits/3), recovers x with cost (in mult-equivalents, 1.5*log2 r per exponentiation) below rho at some size; shows the detector sees an aux-driven gain.
- A/A arm: the rho runs split into two halves (seeds identical process); mean ratio must lie in [0.7, 1.4].
- Deciding: the decision is "obstruction re-derived in toy" (stays closed) vs "obstruction not reproduced" (reopen for audit).

## 4. Method (exact algorithm, parameters, seeds, sizes; cost estimate within the machine limits)
- Group: subgroup of order r in Z_p^*, r prime, r = 1 mod d, p = 2kr+1 prime; group op = one modular multiplication.
- Seed: random.Random(20261110 + bits) per size; bits in {12,14,16,18,20}; d = 2^(bits//3).
- Rho: Brent cycle detection, 16-way r-adding walk (X mod 16 selects g^u h^v), restarts when b-collision degenerate; cost = walk multiplications (precompute excluded). 40 runs/size.
- Cheon: zeta primitive root mod r; BSGS for t mod (r-1)/d using g^(zeta^(d i)) against Q=g^(x^d), then BSGS for k in [0,d); cost counted in group exponentiations, converted at 1.5*log2 r mults each; 20 runs/size.
- Acquire-by-DL cost = rho mults + one exponentiation.
- Unpriced native work (section 5a band 0/0.1/1 GAE) is not applied: toy, same unit (mults) on all arms, so no cross-method ranking is claimed.
- Cost: pure Python, well under 1 minute expected, <50 MB RAM, one core.

## 5. Classification: runnable_local
A stdlib toy can decide the slope and the null; the closed lead's recorded Omega(sqrt r) bound is cheaply re-measured. run.py provided; `--smoke` passes (trivial size, verdict forced inconclusive).

## 6. What a run cannot establish (honest limits; results will be labelled [pcc:unverified])
- It does not prove the generic lower bound; the Omega(sqrt r) claim is a GGM theorem, and the toy only re-measures that the known route (DL by rho) scales as r^0.5.
- The acquire-by-DL arm is partly tautological (it contains a rho); the real content is the null for span shortcuts and that Cheon alone needs free aux.
- Multiplicative subgroups of Z_p^* are not elliptic curves; they have index calculus, which is irrelevant at toy size but not equivalent to ECDLP.
- Says nothing about deployed curves (E(r) ~ 0.254, needs d-degree supply) or non-generic acquisition (e.g. leaked [x^i]P).
- Slope estimate over 5 sizes with 40 runs each has about +-0.03 noise; the band is a judgement, not a calibrated test.
- No replay certificate, validator report or matched rho under the gate: not leaderboard-eligible.
