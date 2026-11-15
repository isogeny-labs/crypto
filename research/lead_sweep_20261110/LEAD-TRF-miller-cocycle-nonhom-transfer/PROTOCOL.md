# LEAD-TRF-miller-cocycle-nonhom-transfer

## 1. Lead as recorded
- Disposition: open_unexamined (last_reviewed 2026-10-24). Family: transfer.
- Title: Non-homomorphic transfers into F_p^* when the pairing is trivial: Miller-cocycle primitive U, zeta-defect, generalized Jacobians.
- Next-step excerpt (truncated in leads.json): "Test H1 of IDEA-20261017-e3a6f1 at toy size: equidistribution of ell(k) = log_g U(kP) in Z/(p-1)".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 57-67, 116, 281);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (lines 106-133, row 4: "equidistribution at the 1/p rate closes both (a non-homomorphic map of this kind is a hash); if r_k needs k the primitive collapses into the closed elliptic-net lead").
- IDEA-20261017-e3a6f1 (H1, K1/K2) was not found in the readable files; U is instantiated below by my own reading (assumption).

## 2. Hypothesis
Instantiation: U(kP) := Miller function f_{k,P} (divisor k(P)-(kP)-(k-1)(O), normalised by the iterative line/vertical recursion) evaluated at a fixed auxiliary point S in E(F_{p^2}) outside E(F_p), mapped to F_p^* by the norm; ell(k) = log_g Nm f_{k,P}(S) in Z/(p-1), on prime-order curves with gcd(N,p-1)=1 (pairing trivial).
H: ell(k), k=1..N-2, is statistically indistinguishable from i.i.d. uniform on Z/(p-1) in (a) its residues mod small q | p-1, (b) its increments ell(k+1)-ell(k), (c) its doubling defect ell(2k)-2 ell(k), i.e. the map is hash-like and carries no usable algebraic structure at the 1/p rate. Falsified if any curve shows structure beyond the matched null.

## 3. Preregistered prediction and falsification criterion
- Statistic T: maximum over tests (marginal, increment, doubling-defect) and over moduli q in {3,4,5,7,8,9,11,13,16} dividing p-1, of the standardized chi-square z=(chi2-df)/sqrt(2 df).
- Prediction: T_real <= 99th percentile of the null for every curve (consistent_with_prediction; lead closes as "hash", cheap re-confirmation of the atlas expectation).
- Falsified: any curve has T_real > max of the null replicates AND T_real > 5 (structure present: lead deserves a real follow-up).
- Inconclusive: otherwise, or if either control gate fails.
- Negative control / null: 150 i.i.d. uniform Z/(p-1) sequences of the same length per curve (seeded 1000+p).
- Positive control: planted ell(k)=c*k (+ uniform noise on 30% of k) mod p-1; must give T > null max and > 5 on every curve, else verdict inconclusive.
- A/A arm: one further independent null sequence through the same instrument; must lie within the null range.
- Not a guarantee about p ~ 2^12..2^14 generalising.

## 4. Method
Python 3 stdlib. Seed 20261110 (curve search), random.Random(p) for P and S, derived seeds above. Primes p = 3 mod 4 in [6000,12000), 6 curves y^2=x^3+ax+b, random (a,b), prime order N (group order ~2^13), gcd(N,p-1)=1 by N prime and N not dividing p-1... checked with gcd. P random; S=(x_s, c*i), i^2=-1, with x_s^3+ax_s+b a non-residue so S lies on the twist (a point of E(F_{p^2})). Loop T=kP: line through kP,P evaluated at S gives a+bi, vertical at (k+1)P gives F_p element; f updated in F_p[i]. Norm to F_p, discrete log by table against a primitive root. Abort curve if a line or vertical vanishes at S. Cost: about 6 x 10^4 Miller steps plus about 900 null sequences of length ~10^4; estimate 1-3 minutes, <50 MB, one core.
Smoke: one curve, p in [400,700), 20 null reps, <1 s.

## 5. Classification
runnable_local. Everything is a stdlib toy: point counting by O(p) scan, Miller loop, chi-square tables.

## 6. What a run cannot establish
- That U matches IDEA-20261017-e3a6f1's U (here an assumption: norm of a normalised Miller function at a twist point).
- Anything about cryptographic sizes, other auxiliary points, other normalisations, or structure not captured by residue/increment/doubling chi-squares (e.g. higher-order cocycle relations).
- It does not show an attack or a speedup, and does not address p-1 with only large prime factors (few small q; fewer tests).
- Results are [pcc:unverified] and a "consistent" verdict only fails to find structure.
