# LEAD-TRF-one-point-self-pairing

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_obstructed (last reviewed 2026-10-24; atlas_tally_20261101). Family: transfer.
- Title: "A non-degenerate self-pairing or distortion map computable from one point of the prime-order subgroup".
- Next-step excerpt: none recorded (empty in leads.json; next_step_cost absent).
- Atlas wording (theorem): self-pairings on a prime-order subgroup are 1-dimensional; trivial (target group trivial) for embedding degree k>1; E[N] rational iff k=1; random curves have k ~ N.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 35, 58, 149); /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (line 58). Sibling LEAD-ICP-self-pairing-prime-order is the same obstruction.

## 2. Hypothesis
For E/F_p with prime order r = #E(F_p) and r != p: (a) the target of the Tate self-pairing, F_p^*/(F_p^*)^r, is trivial exactly when k = ord_r(p) > 1, so no non-degenerate pairing of P with itself exists; (b) k is large, typically k/(r-1) >= 0.1 at median for random such curves, so MOV-type transfer is infeasible. The recorded obstruction is re-derived at toy size; the nontrivial case needs r | p-1, which prime order rules out.

## 3. Preregistered prediction and falsification criterion
- Prediction: over random prime-order curves, p in [2^12, 2^16), r > 1000: (i) 100% have gcd(r, p-1) = 1 (trivial pairing target); (ii) median k/(r-1) >= 0.1; (iii) at most 5% of curves have k <= 20.
- Falsified if any curve has gcd(r,p-1) > 1, or median k/(r-1) < 0.1, or more than 5% have k <= 20. Consistent otherwise.
- Positive control: k=1 curves (r | p-1, r | #E, r^2 does not divide #E, cofactor 2..8) where the reduced Tate self-pairing T(P,P) is computed by Miller's algorithm; require >= 90% nontrivial and T(P,2P) = T(P,P)^2 in every tested case (the detector can see a non-degenerate self-pairing and it is bilinear). n >= 30.
- Negative control / null: the random prime-order population itself (the claim is that nothing nontrivial exists there).
- A/A arm: the null population split in two halves (first/last) by the same instrument; |median_A - median_B| <= 0.15 else inconclusive.
- Verdict "inconclusive" if the positive control fails, A/A fails, or n_null < 100.

## 4. Method
- Python 3 stdlib only. Seeds: positive control 20261111, null 20261112.
- Null: draw random primes p in [2^12, 2^16), random (a,b), naive Legendre-table point count, keep if N prime, N > 1000, N != p; k = multiplicative order of p mod N (factoring N-1 by trial division). Target n = 240 curves.
- Positive: prime r < 2^13, p = m*r+1 prime (m in {2,4,6,8}), random curves with r | #E; P of order r; T(P,P) = (f_{r,P}(P+R)/f_{r,P}(R))^((p-1)/r), random auxiliary R (retry on degenerate); n = 60.
- Cost: well under 2 minutes, < 50 MB, one core.

## 5. Classification
runnable_local. A stdlib toy decides the tally; the theorem is also re-derived (k=1 iff r | p-1, which with #E = r forces r near p, impossible beyond tiny p).

## 6. What a run cannot establish
- Not a proof of the theorem; it samples toy sizes (r <= ~2^17) and says nothing about cryptographic sizes or about non-pairing homomorphisms.
- Only the Tate self-pairing is exercised; distortion maps and Weil alternation are not tested (Weil e(P,P)=1 holds by alternation).
- Does not address cofactor/composite-order or twisted constructions. Results are labelled [pcc:unverified].
