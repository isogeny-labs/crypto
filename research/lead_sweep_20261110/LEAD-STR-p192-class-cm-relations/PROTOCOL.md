# LEAD-STR-p192-class-cm-relations
## 1. Lead as recorded
- Disposition: closed_obstructed (last reviewed 2026-10-24; atlas_tally_20261101). Family: structure.
- Title: "P-192 isogeny class, smooth principal CM relations and loops". next_step_excerpt: empty in leads.json.
- Survey line (atlas_survey.md, item A3): "103/103 CM relations scalar, completeness not proved"; item 11: endomorphism tricks give only the sqrt|Aut| constant, End(G)=Z/N on prime-order G.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 51-55, 144-150 only); research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (id row only). No other hits read; cryptanalysis not searched.
## 2. Hypothesis
Recorded obstruction: on a prime-order group E(F_p) every CM endomorphism a+b*zeta (smooth norm) acts as the scalar a+b*lambda mod N, lambda a root of the min polynomial mod N, so a smooth CM relation is a known integer relation and carries no information on the discrete log d. Falsifiable toy form: for j=0 curves with prime N (N=1 mod 3), 100% of tested points Q and 7-smooth-norm elements satisfy (a+b*zeta)(Q) = [(a+b*lambda) mod N]Q.
## 3. Preregistered prediction and falsification criterion
- Statistic: fraction of (element, point) checks passing, per arm, plus fraction of curves where lambda is a root of x^2+x+1 mod N.
- Threshold: prediction holds if fraction = 1.0 in the CM arm and the A/A arm and all lambda are roots. Any single mismatch falsifies the scalar-action claim (verdict "falsified"). Fewer than 1 curve or failed control gives "inconclusive".
- Negative control: conjugate scalar lambda^2 (wrong eigenvalue); must match < 5% of points (it should match 0%).
- Positive control: lambda recovered by exhaustive DLP search (zeta(P0)=lambda P0), shows the instrument can see an actual endomorphism action; the negative control shows it rejects a wrong one.
- A/A arm: independent seed set (2002 vs 1001), same code, same sizes; both must be 1.0.
- Decision: consistent => the toy re-derives the recorded scalar obstruction (only for j=0, Z[zeta3]); falsified => reopen lead; it does not bear on rho gap (no cost priced, nothing about section 5a).
## 4. Method
Curves y^2=x^3+b over F_p, p=1 mod 3 prime in [2^12,2^17) (smoke: 600..2000), b random, N=#E counted by Legendre sum, keep N prime with N=1 mod 3 (12 curves per arm). zeta(x,y)=(omega x,y), omega a primitive cube root of 1 mod p. Per curve: 8 random points; for a in [-12,12], c in [1,12] with a^2-ac+c^2 7-smooth, check aQ + c*zeta(Q) == [(a+c*lambda) mod N]Q. Seeds 1001 and 2002 (random.Random, derived seeds seed*7+1 for points). Python stdlib only, single core, estimated < 2 minutes full, < 5 s smoke.
## 5. Classification
runnable_local. A stdlib toy re-derives the recorded scalar-action bound at order 2^12..2^17.
## 6. What a run cannot establish
- Nothing about P-192 itself (order ~2^192, class group size, actual smooth CM relations): the toy uses j=0, h=1 only, not the P-192 order of discriminant D.
- It cannot prove the completeness gap ("completeness not proved"); it re-confirms a theorem-level fact on instances.
- No cost, rho gap or speedup claim; the sqrt|Aut| constant is not measured. Results are [pcc:unverified].
