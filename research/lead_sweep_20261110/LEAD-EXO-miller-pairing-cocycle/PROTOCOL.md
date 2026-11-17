# LEAD-EXO-miller-pairing-cocycle

## 1. Lead as recorded
- Disposition: open_next_step (last reviewed 2026-10-24). Family: exotic. Source: atlas_survey_20261107 --pursuit.
- Title: Miller functions, S-unit descent and pairing-free Miller cocycles on prime-order curves.
- Next-step excerpt (truncated in leads.json): "Verify IDEA-20261017-e3a6f1 claims K1 (Weil-reciprocity cocycle, U unique primitive) and K2 (U(k..."
- Extra context from desk notes: the desk README row 4 pairs this lead with LEAD-TRF-miller-cocycle-nonhom-transfer: verify K1/K2 on toy prime-order curves with gcd(N,p-1)=1, test equidistribution of l(k)=log_g U(kP) with a planted-structure positive control. The atlas survey says "non-homomorphic map into F_p^*; expected to be a hash". Equidistribution at the 1/p rate closes both leads.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry only), grep hits in /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 59, 116, 194) and /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (lines 106, 130). The full text of IDEA-20261017-e3a6f1 (the exact definition of U, K1, K2) was NOT found; U is taken below as the Miller value, which is an assumption.
- leads.json fields next_step_cost and why_it_might_beat_sqrt_p are empty for this entry.

## 2. Hypothesis
H0 (recorded obstruction): for P of prime order N on E/F_p with gcd(N,p-1)=1, the map k -> U(k) = f_{k,P}(Q) in F_p^* (Miller function with divisor k(P)-(kP)-(k-1)(O), normalised, Q a fixed other point) is indistinguishable from a hash. Precisely, the d-th power-residue class c(k) in Z_d (d the smallest odd prime factor of p-1) shows (a) uniform class counts, (b) no additive-character bias in k (no linear structure in k mod N), and (c) a homomorphism defect U(a+b)=U(a)U(b) at rate about 1/(p-1), i.e. the cocycle l_{aP,bP}(Q)/v_{(a+b)P}(Q) is never trivial.

## 3. Preregistered prediction and falsification criterion
- Statistic 1: M = max over T=400 random frequencies t of |sum_k zeta_d^{c(k)} e(kt/N)|^2 / n. Per frequency the null law is about Exp(1); family p-value p_M = 1-(1-exp(-M))^T.
- Statistic 2: chi-square of class counts (Wilson-Hilferty p-value).
- Statistic 3: count of random pairs (a,b) with U(a+b)=U(a)U(b).
- Prediction: for each of 3 prime sizes, p_M > 0.001, chi-square p > 0.001, and homomorphism hits <= max(2, 1% of pairs).
- Falsified if any configuration has p_M <= 0.001 or chi-square p <= 0.001 or hom hits above that bound, provided controls pass: this would mean U carries exploitable structure in k and K1/K2 deserve a real follow-up. Consistent: the recorded "hash" expectation holds at toy size and the lead's toy test is closed (this does not close larger p or other choices of Q).
- Negative control / null: uniformly random classes of the same length (spectral stat must give p_M > 0.001).
- A/A arm: a second independent random-class sequence with the same instrument and a different seed stream (must also give p_M > 0.001).
- Positive control: planted linear phase c(k)=round(d*(k t0 mod N)/N) kept with probability 0.4, else uniform; detector is given the planted frequency among its T, and must give p < 1e-6 (the analysis shows the detector can see an additive structure; it does NOT test blindness to other structures).
- If any control fails the verdict is inconclusive.

## 4. Method
- Primes: first prime >= 4099, 12289, 32771; per prime, curve y^2=x^3+ax+b found by seeded random search (seed 20261110+p) with prime order N (point counting by table of square counts, O(p)), gcd(N,p-1)=1.
- P, Q: random distinct affine points (seeded). U(k) computed by textbook Miller double-and-add with exact inversion mod p for k=2..6001 (n=6000; degenerate k with a zero line or vertical value skipped).
- c(k) = discrete log of U(k)^((p-1)/d) in the order-d subgroup, d the smallest prime factor >= 3 of p-1.
- 2000 random pairs for the homomorphism defect; controls as above with derived seeds seed+1.
- Cost: pure Python, about 1 minute total, <50 MB RAM, one core, no network. Smoke mode: p~1031, n=300, T=40, <1 s.
- Output: one JSON object with statistics, seeds, parameters, and verdict.

## 5. Classification
runnable_local. A stdlib toy decides the equidistribution test the desk note asks for, with a planted-structure positive control.

## 6. What a run cannot establish
- Toy sizes (p ~ 2^12..2^15) only; says nothing about cryptographic sizes or a subexponential algorithm.
- U is my reading of the Miller value; if IDEA-20261017-e3a6f1 defines U or the cocycle differently (S-unit descent, a different Q or normalisation), the result concerns a different map. K1 (uniqueness of U) and K2 are not verified; no Weil reciprocity statement is proved here.
- Only the order-d residue class of U is tested, not the full discrete log, and only additive-character structure in k; other structure (e.g. polynomial in k, elliptic-net recurrences) is not tested.
- A pass shows consistency with "hash"-like behaviour at three sizes, not a proof; a falsification would need an independent replay.
- All results are [pcc:unverified] assertions.
