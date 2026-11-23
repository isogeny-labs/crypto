# Fixed coefficient cache for the paired numerator

Derived 2026-11-09, T-1989. Keep Round67's baby algebra A=K[T]/H_G, V=beta(T), W=gamma(T), target kernel H_Q, and reduced numerator M0+s*M1. The construction below uses public target points and existing interpolants. Native controls and operation counts are unverified producer evidence pending independent review. [pcc:unverified]

Coefficient triples. Order coefficients by the giant x-coordinate degree0,1,2. For the curve y^2=x^3+a*x+b0, define

DG=(T^2,-2T,1), NG=(aT+2b0,T^2+a,T), FG=(a^2-4b0*T,-2aT-4b0,T^2).

For the other giant coordinate z replace T by V, obtaining DQ,NQ,FQ. Thus D_G=sum DG_i*x^i, N_G=sum NG_i*x^i, F_G=sum FG_i*x^i, and the analogous Q polynomials are quadratic in z. These identities are immediate coefficient expansions, with no interpolation at giant points. [pcc:unverified]

Cache definition. For0<=i,j<=2 precompute and reduce modulo H_G:

C_ij=FG_i*DQ_j+DG_i*FQ_j-2*NG_i*NQ_j,
J_ij=2*(NG_i*DQ_j-DG_i*NQ_j).

Then Round67's reduced pair numerator is exactly

M0=sum_ij C_ij*x^i*z^j+8*y*w*W,
M1=sum_ij J_ij*x^i*z^j.

To prove it, distribute the products in F_G*D_Q+F_Q*D_G-2*N_G*N_Q and2*(N_G*D_Q-N_Q*D_G). Reducing each coefficient commutes with scalar linear combination in the quotient A. The unchanged signed gamma term is added separately. The independent integer-polynomial checker reconstructs both full universal polynomials from these triples, compares them coefficientwise with the direct reduced expressions, retains all18 coefficient supports and checks a deliberately changed cache coefficient has nonzero residual. [pcc:unverified]

Online construction. Form xp=(1,x,x^2),zp=(1,z,z^2), then nine weights xp_i*zp_j. Each weight updates the two cached coefficient vectors; the gamma scalar8*y*w updates M0. With actual cached coefficient slots S=sum_ij(len(C_ij)+len(J_ij)), and gamma length g, the implemented formation makes exactly S+g+13 K multiplication calls:2 for x^2,z^2,9 weights,2 for8*y*w, and S+g coefficient updates. Coefficient vectors have length at most baby-width b, so S<=18b,g<=b and the formation bound is19b+13. There is no online polynomial product, polynomial division or field inverse in this formation stage. The field norm, nonzero denominator, output division and query point additions are priced separately. [pcc:unverified]

Setup and memory. The constructor makes a fixed number of polynomial products/reductions of degree O(b), taking O(b^2) field operations with the current classical arithmetic. This is once per changing target; it is not free online setup. Cache storage is at most18b field coefficients, in addition to gamma,H_G,H_Q,public point arrays and retained reference data. The qualifier builds the same cache even for its direct control to keep its instrument/input path fixed, records that extra qualification work, and excludes the unused tensor cache from baseline charged setup. For an end-to-end candidate cost, add its cache constructor to its selected queries. [pcc:unverified]

Response correctness. Apply the unchanged singular-safe norm to the cached M0+s*M1, then divide by H_G(x)^2*H_Q(z)^2 and recombine centers/tails. Equality of the numerator coefficients proves equality of every complete baby-block jet, hence of the complete finite-panel B-only response. No scalar value is used in cache construction. Known-answer scalars remain fixture/recovery checks. The old direct expanded/reduced sources are preserved. [pcc:unverified]

Sources checked. Round67 cancellation-proof.md supplies the reduced pair and bidegree; Round64 resultant-proof.md supplies the first-norm branches. BFLS arXiv2003.10118 Lemma4.3 and section4.16 were read in the preceding round; the present step is direct coefficient extraction/caching of the retained coupled expression. KB tools/wrapper unavailable; no novelty assertion for coefficient caching or a new coupled resultant bound. [pcc:unverified]
