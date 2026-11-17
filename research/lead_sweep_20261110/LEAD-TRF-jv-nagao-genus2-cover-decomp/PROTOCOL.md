# LEAD-TRF-jv-nagao-genus2-cover-decomp
## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_dominated (reviewed 2026-10-24; also 2026-11-01). Family: transfer.
- Title: Nagao decomposition on a genus-2 cover of E(F_{p^6}) (examples/jv_cover.rs), with F4 and sparse-FGLM engineering.
- Next-step excerpt (truncated in leads.json): "Revisit when: the sieve row fails validation, or a large-prime variation for the Nagao route is ..."
- Recorded obstruction (atlas_survey.md line 60): S/rho 151-293 (cost over matched rho).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 58-67, 311-313 only); atlas_tally.json matched but not read.
## 2. Hypothesis
For genus-2 Jacobian index calculus with a degree-1 factor base over F_p (the structure the Nagao route reduces to), the probability P that a random reduced degree-2 divisor is smooth is 1/2 (+-0.04), so relation collection alone costs (p+1)/P >= 2.2x the matched rho cost sqrt(pi N/4) (N ~ p^2) in group operations, at every toy size, before decomposition, F4/FGLM and linear algebra. Thus the recorded S/rho >> 1 is not a constant-factor accounting artefact: no lower-order saving can bring relation collection below rho.
## 3. Preregistered prediction and falsification criterion
- Statistic: measured P (99% Wilson CI) per prime and R = ((p+1)/P)/sqrt(pi p^2/4), counting 1 group op per trial.
- Consistent: |P-0.5|<0.04 (or CI contains 0.5) at all p and lower CI of R >= 1 at all p, with controls passing.
- Falsified: any p with upper CI of R < 1 (relation collection cheaper than rho, would reopen the lead).
- Inconclusive: controls fail (A/A mismatch or positive control < 0.99) or mixed outcome.
- Negative control / null: not applicable (no null population of Jacobians exists for a counting identity); the A/A arm and the exact-1/2 theoretical value serve as reference.
- Positive control: forced-smooth arm (x-pairs drawn without curve test) must give P=1.0 > 0.99, showing the detector registers smoothness.
- A/A arm: same procedure, different derived seed; point estimates must differ by less than the CI width.
## 4. Method
Primes p in {251,509,1021,2039,4093}; per prime 6 random monic quintics f (squarefree-ish: <=2 roots), 60000 random monic quadratics u=x^2+ax+b in total, each valid u weighted by its number of Mumford v (split u: 4 if both roots are x-coordinates of F_p-points; irreducible u: 2 if f(root) is a square in F_{p^2}, via the norm). P = weight of split / total valid weight, i.e. the fraction of reduced degree-2 divisors that are smooth over the degree-1 factor base (expected 1/2). The 99% Wilson CI treats weights as counts (approximate). Seeds: 20261110*1000+10p+arm. Pure stdlib, 1 core, expected < 1 min.
## 5. Classification: runnable_local
A stdlib toy decides the constant-factor part of the recorded bound; the full S/rho 151-293 (F4, linear algebra, cover F_{p^6}) needs Rust/Groebner tools and is not re-measured.
## 6. What a run cannot establish
It does not measure Jacobian group order (N~p^2 is assumed), Cantor arithmetic, the cover/descent from E(F_{p^6}), large-prime variation, sieving, F4/FGLM or linear algebra, nor the 151-293 figure. It says only that relation collection is >=1x rho in counted group ops. Result labelled [pcc:unverified].
