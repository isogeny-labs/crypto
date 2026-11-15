# LEAD-ICP-gfp5-symmetrized-gaudry

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_dominated (atlas 2026-11-07 survey; was open_next_step on 2026-11-01). Family: ic-prime. Last reviewed 2026-10-24. Source: atlas_survey_20261107 --pursuit.
- Title: Symmetrization-aware Gaudry/Diem decomposition on EcGFp5/EcMasFp5 (n = 5) using rational 2-torsion ((Z/2)^4 x| S_5) and side dichotomy.
- Next-step excerpt (truncated in leads.json): "Revisit when: Any of the following reopens the lead: - a finish cheaper than msolve's rational p..." (next_step_cost and why_it_might_beat_sqrt_p are empty in leads.json).
- Recorded result (atlas_survey.md A4 detail): quotient system at m=5, msolve 0.6.5, p' = 4111, 262231, 33554371, D = 2^16; whole-solve c_cyc 2^46.30 / 2^46.56 / 2^46.63 vs closure threshold 2^46 (PROMOTE 2^38); closes by only 0.30-0.63 bits. Reopen: cheaper finish than rational parametrisation, FGb-type F4, D < 2^16 construction, u = x/y ring form; block-Wiedemann finish unfinished.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 33-90, 300-330); /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (grep context only). atlas_tally.json not read.

## 2. Hypothesis
The recorded closure margin (0.30-0.63 bits) depends on the relation yield implied by the decomposition count: for E over F_{q^n} (a, b not in F_q), factor base {points with x in F_q}, the number of n-subsets of distinct, non-antipodal factor-base points is C(F, n) and their sums behave like T = C(F,n) balls in N = #E bins. Hypothesis H: at n = 3, q in {29..43}, the collision-excess ratio r (observed extra collisions divided by the uniform-balls expectation) has a pooled 95% bootstrap CI inside [0.9, 1.1], i.e. no hidden yield structure beyond conservation that could erase the 0.3-bit margin. (n = 3 is the stdlib-sized stand-in for n = 5; no (Z/2)^4 orbit exists in this toy because 2-torsion is not rational over F_q.)

## 3. Preregistered prediction and falsification criterion
- Statistic: r = (T - D_obs) / (T - N(1 - (1-1/N)^T)) pooled over curves (D_obs = distinct sums; N from exact point count).
- Prediction: r in [0.9, 1.1] (CI contained). Falsified if the CI lies wholly outside [0.9, 1.1] while controls pass (yield is non-conserved: a real lever, or a hidden obstruction, at toy size). Inconclusive if the CI straddles an edge or any control fails.
- Negative control / null: T balls thrown uniformly into N bins per curve (seeded); r_null must have CI containing 1 or lie in [0.9, 1.1].
- Positive control: same sums but keeping degenerate triples {P, -P, Q} (which sum to Q); r_deg CI lower bound must exceed 1.5 (detector sees real structure).
- A/A arm: curves split by index parity into two halves; pooled r difference must be <= bootstrap half-width tolerance + 0.05.

## 4. Method
Pure stdlib. F_{q^3} = F_q[t]/(t^3+ut+v), irreducible cubic drawn by seed. Per q in [29, 31, 37, 41, 43]: 8 random curves y^2 = x^3 + a x + b, a, b in F_{q^3}. N by brute-force count via a table of squares. FB = all points with x in F_q (about q points). Enumerate all C(F,3) triples, exclude antipodal pairs and repeated x, add affinely, record distinct sums. Seed 20261110 (derived per q); bootstrap B=400 over curves; null seed +11. Cost estimate: about 40 curves x 2-4 s, internal wall guard 480 s, RAM < 100 MB (q^3 = 80k-element dict), 1 core. Smoke: q in {7, 11}, 2 curves each.

## 5. Classification: runnable_local
A stdlib toy can test the yield-conservation input to the recorded cost model. It cannot touch the msolve finish itself (that is blocked/closed); the classification covers only the toy.

## 6. What a run cannot establish (honest limits; results will be labelled [pcc:unverified])
- n = 3 not 5; q about 2^5 not 2^12-2^26; no rational 2-torsion symmetry, no quotient D = 2^16, no F4/msolve/Wiedemann finish cost, so the 0.30-0.63 bit margin and c_cyc are not re-derived or moved.
- Yield conservation at toy size does not reopen or close the lead; it only checks that no hidden yield structure contradicts the model's input.
- Not a rho-matched comparison (AGENTS section 5a) and no leaderboard-eligible claim; collision-excess has roughly 2-5% sampling noise per curve, so CIs narrower than [0.9, 1.1] are not guaranteed and "inconclusive" is a live outcome.
