# LEAD-TRF-padic-canonical-section

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_obstructed (atlas_tally_20261101; last_reviewed 2026-10-29). Family: transfer.
- Title: "Canonical point sections E(F_p) -> E(Q_p) and their p-adic digits on non-anomalous curves".
- next_step_excerpt: empty in leads.json (closed lead, no next step recorded).
- Recorded obstruction (atlas_update.md, quoting the atlas card): for p > 2 and p not dividing #E(F_p), the only homomorphic section E(F_p) -> E~(Z_p) is the prime-to-p torsion section, which the formal logarithm kills; non-homomorphic sections recover d at rate 1/p (survey item 12, with an EXO-padic-jet toy contrast "at chance"). Smart/Satoh-Araki only for #E = p.
- Files read (paths under /home/user/crypto):
  - research/prime_field_lead_assessment_20261107/atlas_survey.md (A4 line 58, item 12 line 148)
  - research/prime_field_leads_20261031/C1_lift_torsion/atlas_update.md (lines 1-28)
  - research/ecdlp_subexp_desk_20261101/README.md (lines 57, 77, 86, 88)
  - research/lead_sweep_20261110/leads.json (entry)
- Not read: the atlas YAML (encrypted here); no /home/user/cryptanalysis search was needed beyond the above.

## 2. Hypothesis
For a prime-order curve E/F_p with N = #E(F_p) prime and N != p, lifted to E~ over Z/p^2:
(H1) the section s(P) = [1 - N c]P~ (c = N^-1 mod p; i.e. multiplier = 1 mod N, 0 mod p) is well defined, reduces to P, satisfies N*s(P) = O exactly, and is a group homomorphism, for every lift curve; (H2) the Smart functional psi (psi(K) = (X/Y)/p mod p on the kernel of reduction) vanishes identically on N*s(P), so it carries no information on d; (H3) for non-homomorphic (random-lift) sections the estimate dhat = psi(N Q~)/psi(N G~) mod p equals d mod p only at the chance rate 1/p. Falsifiable: any failure of H1/H2, or a hit rate above the binomial bound in H3.

## 3. Preregistered prediction and falsification criterion
- Statistics: (a) counts of exact property passes (on-curve, reduction, N s = O mod p^2, additivity) out of n; (b) psi(N s(.)) nonzero count; (c) hits = #{trials: dhat == d mod p} vs Binomial(T, 1/p).
- Prediction: (a) all n pass on both lift curves; (b) 0; (c) hits < h*, where h* is the smallest h with P(Bin(T,1/p) >= h) <= 1e-4, at each of 3 sizes.
- Decides: any (a)/(b) failure or (c) excess => falsified (obstruction or implementation wrong). Otherwise, with controls passing => consistent_with_prediction. A failed control => inconclusive.
- Negative control / null: random (non-homomorphic) lifts; additivity must FAIL for >= 90% of triples (checker power), and hits follow Binomial(T,1/p).
- Positive control: anomalous curve (N = p, p ~ 2^11): Smart ratio on random lifts must recover d in 100% of non-degenerate trials (detector can see the effect).
- A/A arm: second lift curve y^2 = x^3 + ax + (b + p t) must give identical exact-property results; hit counts of the even/odd trial halves are reported and must both be under h*.
- Rho band (5a) is not applicable: no cost comparison with rho is made; this is an obstruction re-derivation, not a speed claim.

## 4. Method
- Python 3 stdlib, seed 20261110 (per-size rng seed+bits; positive control seed+999). Sizes: p = next prime above 2^b + rand(2^(b-2)), b in {12, 14, 16}; curve (a,b) found by naive point counting until N prime, N != p (orders ~2^12..2^16 but N prime so group order is N).
- Arithmetic: projective RCB complete addition over Z/p^2 (identities checked via on-curve and N s = O tests); random lift of an F_p point: x + p u, y by Hensel.
- Exact arm: H = 150 random pairs per size per lift curve; checks as in section 3; also tests s(P1+P2) = s(P1) + s(P2) with lifts re-randomised.
- Ratio arm: T = 20000 trials per size, random d in [1,N-1], independent random lifts of G and of Q = dG; dhat as above; degenerate (psi(N G~) = 0) trials skipped.
- Cost: well under 1 CPU-minute estimated (~1 s smoke-scale per 10^3 mults at p ~ 2^16), RAM < 50 MB (point-count dict of size p), no network.

## 5. Classification: runnable_local
A stdlib toy decides H1-H3 directly on a re-derivation of the recorded bound (smoke test passed in 0.05 s, verdict consistent).

## 6. What a run cannot establish (honest limits; results labelled [pcc:unverified])
- Toy sizes only (p <= 2^16, Z/p^2 not Z_p): it does not prove the theorem for Z_p or cryptographic p; the proof (Hom(finite, Z_p) = 0) is the real evidence.
- Only tests prime-to-p torsion section and uniform random-lift sections; it says nothing about cleverly structured non-homomorphic sections, or p | N cases other than the N = p control.
- Consistency with prediction does not imply a speedup or a new lead; it neither beats rho nor changes any leaderboard row.
- Smoke-scale hit threshold is not enforced; only the full run decides H3.
