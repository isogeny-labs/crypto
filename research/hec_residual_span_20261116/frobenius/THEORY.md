# Character-sum Frobenius operator and a linear eigenvalue certificate

Status: written derivation using classical diagonal-curve/Jacobi-sum and tame cohomological trace theory. This is not a priority claim, independent proof or generic-curve result. Conventions below are fixed before finite execution. The purpose is to replace candidate group scalar multiplications with a finite-field polynomial certificate on the existing binomial fixture.

## Hypotheses and convention

C:y²=x^n+c over Fp, n=2g+1 odd, c≠0, p odd with p≡1modn, p∤n; smooth projective model, rational infinity. Let zeta∈Fp have exact ordern and phi(x,y)=(zeta*x,y). Let chi(t)=t^((p-1)/n), identified with the same powers zeta^j, and eta the quadratic character with eta(0)=0. Define integral coefficients

 A_j = -sum_{t∈Fp*, chi(t)=zeta^j} eta(t+c),
 P(X) = sum_{j=0}^{n-1} A_j X^j.

This convention is tied to the pushforward phi, not an unspecified inverse character. Computing the sum costs O(p) field operations for fixedn; it is not free or a logarithmic-in-p algorithm. The existing fixture has n9,p19,c2.

## Trace derivation of the operator relation

Let pi be p-power Frobenius. The map phi^a*pi has fixed points satisfying y^p=y and x^p=zeta^{-a}x. For x≠0putt=x^n∈Fp*. Each t with chi(t)=zeta^{-a} has nchoices ofx; its ychoices are1+eta(t+c). Include infinity and x0separately. Thus

 #Fix(phi^a*pi) = p+1+eta(c)+n*sum_{chi(t)=zeta^{-a}}eta(t+c),
 Tr(phi^a*pi | H1) = -eta(c)-n*sum_{chi(t)=zeta^{-a}}eta(t+c).

The fixed-point formula uses tame phi and Frobenius's zero derivative, giving simple fixed intersections. On H1 the n−1nontrivial characters of phi each occur once: the characteristic polynomial is (X^n−1)/(X−1), the oddn,2g≡−1case of Guralnick–HoweTheorem1. Since pi commutes with phi, Fourier inversion identifies its scalar on the phi-eigencharacter zeta_complex^k, k≠0, as

 pi_k = -sum_{t∈Fp*}eta(t+c)*chi_complex(t)^k = P(zeta_complex^k).

The constant -eta(c)has zero nontrivial Fourier coefficient. Hence pi=P(phi) on the Tate/cohomology realization; faithfulness gives the corresponding endomorphism relation, and it restricts to prime-to-p torsion. This is a standard character-sum/Jacobi-sum calculation with an explicit action convention. We do not infer an endomorphism formula merely from an ordinary characteristic polynomial or point count: joint phi-character traces identify the operator.

## Eigenvalue by Bezout, without subgroup-action enumeration

Let ell≠p be a prime not dividingn. On rational ell-torsion, pi−1=0 and phi^n−1=0. Compute over F_ell

 G(X)=gcd(X^n−1,P(X)−1).

If G=X−lambda is linear, extended Euclid supplies S(X)(X^n−1)+T(X)(P(X)−1)=X−lambda. Substituting phi on any rational ell-torsion point gives phi(D)=lambda D. This certifies rational subgroup action without a candidate scalar multiplication or an a priori stable one-dimensional subgroup. A non-linear gcd is a retained eligibility failure, not permission to choose a convenient root.

The finite test verifies the exact polynomial Bezout identity, root equations, original-group phi(D)=lambdaD, the full P(phi)(Pj)=Pjoperator on every raw catalogue point, and all projected orbit-point reconstructions. A reversed character convention P(X^{-1})is a negative control: its inferred eigenvalue should be lambda^{-1}, and it must fail the original pushforward group equation on this fixture. It cannot silently replace the prescribed convention after execution.

## Catalogue logs and target descent

If all canonical rational points lie in the signed phi-orbit of P0, and D=13P0has orderell8623 as already verified, then log_D(13Pj)=sign*lambda^a. Signed full coverage remains checked. The algebraic preparation itself requires only field characters, extended polynomial gcd/Bezout and orbit lookup. Original-group action and all point reconstructions remain explicit qualification controls in this correctness gate. A later production interval may replace those redundant qualifications only under a prospectively stated proof/validation boundary; no already measured interval is redefined.

Existing target descent and inverse13formula remain unchanged; the recovered scalar must verify in the original Jacobian. Three same public target controls and native controls are retained, with all input/state freshly prepared. This does not change the fullgenus2+objective to a single eligible curve or claim tenfold from stage removal.

## Primary sources and gaps

Guralnick–Howe, Characteristic polynomials of automorphisms of hyperelliptic curves, Theorem1(1), https://arxiv.org/pdf/0804.0578 : supplies the tame odd-order automorphism character multiplicities here, not the new implementation or timing. Blache–Cherdieu–EstradaSarlabous, Some Computational Aspects of Jacobians of Curves in the Family y³=gamma*x⁵+delta over Fp, 2004, sections1.1–1.2, https://edoc.hu-berlin.de/server/api/core/bitstreams/0ee18a15-b3f6-46f4-8018-a0eab7643329/content : primary diagonal-curve character sums/Frobenius context; its worked (3,5)family is not our (2,9)fixture. The trace/Fourier derivation above fixes our convention independently. Classical endomorphism factor-base reduction remains Tsakou–Ionica2021, primary text linked in orbit/THEORY.md. No novelty inferred.

KB query `Jacobi sums hyperelliptic y2 x n Frobenius endomorphism eigenvalue` returned off-topic infrastructure/report hits, not a source for this formula; source:paper:cde616e4484c4dab8db33ea58d2bb992257073c046884fd1b0180ac1c7b9e94bconcerns unrelated hyperelliptic cover custody. Scores were not taken as evidence. Snapshot revision9141e386254340a9bdba3036f2bb706a predates current work; primary reads above govern. The existing local fixed-curve group/action and cost receipts motivated the hypothesis. A finite operator mismatch falsifies the derivation/implementation conjunction and must be diagnosed without changing character signs or hypotheses after seeing results.
