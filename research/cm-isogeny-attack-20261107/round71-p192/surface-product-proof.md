# Four-polynomial surface contraction for a full marked response

Derived 2026-11-09, T-2076. This is a written algorithmic reduction, with local exact qualification separate from independent scientific review. [pcc:unverified]

## Input and required output

Let E/Fp:y^2=f(x)=x^3+ax+b, p>3, and G,Q be nonzero rational points in an odd prime-order subgroup r. K=Fp2 suffices for the inherited nonrational shift P. The primitive returns the two coefficients modulo eps^2 of
F(eps)=product_(j=0..r-1)(x(P+jG)-x(P+U+jQ)+eps).
No scalar relating Q and G is supplied. U is rational and P is chosen so no point used has a pole. For the inherited B-only decoder retain its stronger shift guards. The coefficient is the same signed marked response qualified in Rounds60–69.

Choose arbitrary 1<=b<r/2, m=2b+1, n=ceil(r/m). Center i has c_i=b+im and giant pair A_i=P+c_iG, B_i=P+U+c_iQ. Signed baby pair j has (T_j,V_j,W_j)=(x(jG),x(jQ),y(jG)y(jQ)),1<=j<=b. Thus W_j^2=f(T_j)f(V_j). Full blocks cover c_i-b..c_i+b. Keep scalar center factors and the final clipped tail explicit. Coverage is exact, with O(b+n) such outside factors and public point generation, not O(r) outside work.

## Constant-size factor before baby interpolation

For giant coordinates(x,y),(z,w), define D_G=(T-x)^2, N_G=(T+x)(Tx+a)+2b, F_G=T^2x^2-2aTx-4b(T+x)+a^2. Substitute(V,z) for(T,x) to obtain D_Q,N_Q,F_Q. To avoid ambiguity, b in these curve expressions is the curve constant; the baby width is the independent integer above.

The paired numerator is
U_i=M0_i+eps*M1_i,
M0_i=F_G D_Q+D_G F_Q-2N_G N_Q+8yw W,
M1_i=2(N_G D_Q-D_G N_Q).
Each ordinary polynomial part has individual degree<=2 in T,V; the odd W coefficient is constant. Round67's cancellation identity identifies this numerator with the product of the two signed x-difference factors, multiplied by (x-T)^2(z-V)^2. This identity retains the relative y-sign, not only the two x-coordinates.

Work in S=K[T,V,W,eps]/(W^2-f(T)f(V),eps^2). Every product has a unique normal form A+WB+eps(C+WD), four ordinary bivariate polynomials. Multiplication uses W^2=f(T)f(V), discards eps^2, and never divides by the constant jet. In particular, zero, same-root double-zero and distinct-root cases require no unit assumption.

Assign degree3/2 to W in each coordinate separately. Each input factor has weighted individual degree<=2, while substituting W^2=f(T)f(V) preserves that upper bound because the right side has degree3 in each coordinate. Therefore the product of k factors has four coefficient polynomials with individual degree<=2k. There are at most4(2k+1)^2 coefficient slots. This bound is independent of the unknown scalar and uses no target interpolant beta or gamma polynomial.

## Evaluation and exact response

For a chunk of k complete giants, form its surface product by a balanced product tree. Evaluate its four coefficient polynomials at all original paired baby x-values (T_j,V_j). Combine the values with the known signed W_j to obtain a jet for each baby. Multiply those jets over all baby points and chunks. Since evaluation is a ring homomorphism, the result is exactly product_(i,j) U_i(T_j,V_j,W_j). Reordering products is valid over dual numbers, including at zeros.

For complete giants divide by the nonzero scalar
D=product_i H_G(x(A_i))^2 H_Q(x(B_i))^2,
where H_G=product_j(X-T_j),H_Q=product_j(X-V_j). Multiply by the retained center/tail scalar jets. This gives F modulo eps^2. It makes no quotient-algebra norm call and does not need baby interpolation. Pole guards imply D!=0; no numerator or intermediate constant coefficient is inverted.

## Fully specified asymptotic backend

A product of k constant-degree surface factors can be formed in k^(2+o(1)) field-operation work by the balanced tree with fast bivariate multiplication. Use Kronecker substitution with radix greater than twice the current per-variable degree to prevent coefficient collisions, fast univariate polynomial multiplication, then unpack. Each node also makes a fixed number of such products and multiplications by f(T)f(V). Summing squared node sizes over tree levels preserves the k^(2+o(1)) bound.

Evaluate each of the four ordinary bivariate polynomials with Kedlaya–Umans2011 Corollary4.5, m=2, d=2k+1, N=b, K=Fp2. For every fixed delta>0 and sufficiently large d it supplies a deterministic bit algorithm with work ((2k+1)^2+b)^(1+delta) log(|K|)^(1+o(1)). Its fixed-number-of-variables hypothesis is satisfied. The four channels are evaluated over the field K; the argument needs no differentiation of the nonalgebraic MME algorithm and no assumption about its action on arbitrary nilpotent rings.

Compute H_G,H_Q by fast univariate product trees and evaluate them at all giant x-values using fast univariate multipoint evaluation. This charges O~(b+n) field work and avoids an uncharged b*n denominator scan. Source and changing-target public points cost O(b+n) group operations; center/tail factors also cost O(b+n) field work. Field/group bit costs and the field-extension representation contribute polylog(p) factors.

For arbitrary retained b,n, chunks k approximately sqrt(b) give coefficient/evaluation input size O(b+n sqrt(b)) up to exponent slack. Alternatively use all complete giants in one product, k<=n: total size O(b+n^2). Choose baby width b approximately r^(2/3), so n approximately r^(1/3). This yields, for every fixed epsilon>0, a deterministic algorithm for the full two-coefficient response using O(r^(2/3+epsilon) polylog(p)) bit work. Standard small-r fallback covers the finite pre-asymptotic range. The theorem is about response construction; it is not an inference from prototype timings or small input sizes.

The existing calibrated B-only point extractor makes a fixed number of these queries and solves a fixed-degree cubic, then verifies candidate point identities. With standard Las Vegas finite-field root finding its expected cost inherits this exponent. This does not make its subsequent orbit search free. A complete ECDLP algorithm still needs its full number of response/action calls, setup and orbit-representative costs. The construction makes no class-number or L-value saving assumption; those invariants remain relevant to a separate CM compression/transport improvement.

## Implemented qualification versus theorem

The Rust prototype uses classical sparse polynomial multiplication and direct sparse evaluation. It checks the four-channel identity, singular cases, unchanged complete small-field decoder inputs, and prescribed P19227-factor responses. It does NOT implement KU, fast multivariate multiplication, fast denominator multipoint evaluation, the full P192-width layout or an asymptotic performance experiment. Legacy baby interpolants/caches remain in the inherited qualification harness but are not required by the mathematical algorithm; their costs must not be erased from recorded prototype work.

## Sources checked

Research KB tools and migrated wrapper unavailable. Bounded prior reports: Round25 actual relation/orbit, Round62 marked transport, Round68 coefficient-cache-proof, Round69 one-norm-proof. The bounded search found no implementation of this four-polynomial response reduction; that is not an absence/novelty proof. Primary source: Kedlaya and Umans, Fast polynomial factorization and modular composition, June2011 version, section4.3 and Corollary4.5, https://users.cms.caltech.edu/~umans/papers/KU08-final.pdf . The initial PDF open returned502; its indexed PDF text was then read, including the extension-ring setup and the full theorem hypotheses. MME and product trees are established tools; the application and cost balancing above are the present derived reduction.

## Direct scalar extraction from the evaluated zero, without an action-oracle loop

This is an additional written application of the same computed surface evaluations, not a native zero-location qualification. Take Q=[k]G, U=G, with Q=O or Q=G handled directly. Require2P not rational as well as the original pole guards. Equality of x(P+jG) and x(P+G+jQ) cannot arise from opposite points: that would make2P a rational point. It therefore implies j(G-Q)=G. Since k!=1, exactly one j*=1/(1-k) modulo r satisfies this equation, and j*!=0. The spectral product has exactly one zero factor.

Keep the per-baby constant evaluations of the giant surface product instead of immediately contracting them all to a scalar norm. If a complete paired block contains the zero, exactly one baby position has zero constant. The baby position is already a known integer j_b. Evaluate each complete giant's constant-degree factor at this ONE paired baby point until its zero is found, costing O(n) field work. With chunked products the recorded zero identifies a chunk, so only its k giants need be checked. Compute the two actual signed point pairs at that giant/baby position to distinguish j*=c_i+j_b from j*=c_i-j_b. This uses a fixed number of group additions and public equality checks; no scalar logarithm is taken. Retained center and tail scans detect the complementary possibility and already carry their full integer indices.

Return k=1-(j*)^(-1) modulo r, then verify [k]G=Q. The extraction uses only the supplied points and public integer indices. It does not receive k as an input, factor a curve-identity collision, or assume a nonzero elliptic-to-H_s homomorphism. The existing native prototype currently discards these intermediate zero indices, so its381decoder checks are not claimed as native executions of this new scalar extractor.

Consequently, given an admissible shift P over the explicitly represented quadratic field, the written construction supplies a deterministic ECDLP algorithm with the same O(r^(2/3+epsilon) polylog(p)) bit bound: one surface-product construction, its multipoint values, O(b+n) outside scans, O(n) zero localization, one modular inverse and one verification scalar multiplication. Shift construction, if not supplied, is a separate finite-field point-generation obligation; the concrete P192 shift is an inherited explicit input. For smaller characteristic or cofactor variants retain the stated subgroup and shift assumptions. This is an algorithmic upper-bound derivation, not a novelty certificate or an implemented fast-backend result. It does not yet exploit the proposed L-value/class-number effect.
