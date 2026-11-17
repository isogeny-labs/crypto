# Round70: explicit shifted fibers and P-192 point action

Status: local Rust exact checks and written derivations, without independent signed scientific review. [pcc:unverified]

## Construction and outcomes

For f(X)=X^3+aX+b, normalize C_s: y^2=f(X), w^2=f(X-s). The maps to the two copies of E and H_s: Z^2=f(X)f(X-s) are explicit: (X,y), (X-s,w), and (X,yw). P-192 is primary. The input-p192.json and input-orders.json files retain exact curve and subgroup identities.

The P-192 native panel completed with both special fibers and four elliptic quotient models over Fp2. Polynomial identities, even sextics, point membership, finite exceptional charts, source doubling and source order checks passed. Across all four quotients, unbased doubling failed; across both infinity basepoints for each quotient, every sign-adjusted prefix failed already at index2. All24 based order-r tests on G,2G,publicQ had nonzero images and nonzero [r]-images. Those outcomes address the actual branch-selected maps, not all possible correspondences or all target torsion. Target group cardinalities and a cofactor projection were not computed.

Seven rational streams and six marked streams of14points were checked. The G and minus-G rational streams have s=0; the other five rational streams and all six marked streams have nonconstant s. These are the frozen source generators, not independently sampled populations. Each retained (s,X,Z) lies on its corresponding family member. No full group arrays were allocated.

The boundary torus order p^2+p+1 has gcd1 with each of the five supplied P-192-run orders. These are inherited orders, not fresh point counts of skipped curves.

## Written derivations and their assumptions

Assume p>3, s is nonzero in Fp, and f is irreducible cubic. Roots shared by f(X) and f(X-s) would, under the Frobenius3cycle, imply3s=0. Thus the finite branch sets are disjoint. Irreducibility persists over Fp2 and the same argument applies there. The V4 cover C_s of P1 has six finite branch values and the shared infinity branch, each with inertia2. Riemann-Hurwitz gives2g-2=-8+14=6, hence genus4. The closure in E x E has arithmetic genus5 and an ordinary node at(O,O); normalization separates its two infinity branches. H_s is a smooth monic degree6 genus2 model with two infinity points. Standard V4 Jacobian decomposition gives J(C_s) isogenous to E x E x J(H_s).

For a divisor P-O on the first elliptic factor, the canonical pullback then pushforward to H produces its two hyperelliptic conjugates above X=x(P), minus the two infinity points. This divisor is div(X-x(P)), hence principal. The second elliptic factor gives the same cancellation with shifted X. Consequently that canonical correspondence is zero. Choosing one square root does not inherit the group-homomorphism property. The P192 quotient checks record the conjugate-image cancellation separately from branch-lift outcomes.

For a nonzero rational translation step T in E x E, the closure of C_s cannot be invariant under T: its unique singular point would have to be invariant. Its divisor class is2F1+2F2, with self-intersection8. If ten distinct consecutive translation-orbit points lay in C_s, C_s and its translate would have at least nine intersection points, a contradiction. This establishes the stated fixed-fiber closure obstruction, with distinctness required. It does not prevent transport in the total family with changing s.

For rational streams one can also use the doubling identity F2(X)=N4(X)/(4f(X)), N4=X^4-2aX^2-8bX+a^2. A nonzero constant difference c forces roots of
R_c(X)=4N4(X)f(X-c)-4N4(X-c)f(X)-16c f(X)f(X-c).
It has degree6 and leading coefficient -12c. A14point constant prefix provides seven distinct roots from j=1..7 and their doubles, impossible for the declared large prime subgroup. This polynomial argument is a written derivation; the native record evaluates the actual streams, not a symbolic universal polynomial checker.

At s=0, after separating the persistent infinity node, H degenerates to two rational components glued at three finite2torsion nodes. The dual graph has first Betti number2. The Frobenius3cycle gives character matrix [[0,-1],[1,-1]], characteristic polynomial T^2+T+1, and torus order p^2+p+1. With gcd(r,p^2+p+1)=1, a homomorphism from the rational order-r subgroup into this rational torus is zero. This refers to the cubic branch field, not the imaginary quadratic Frobenius field from the original L-value hypothesis.

For a nonzero a and delta_f^2=-4a^3-27b^2, set s=+-delta_f/(3a), t=(s-3b/a)/2, B=a/3-ts. Then M=(tX+B)/(X-t) satisfies M^2=-s^2 and F_s(M)(X-t)^6=-s^6F_s(X). Over Fp2 set l=t+i*s,m=t-i*s,i^2=-1,u=(X-l)/(X-m),V=Z(u-1)^3. The resulting sextic G(u)=A6u6+A4u4+A2u2+A0 is even. The two explicit degree2 quotients have cubics (A0,A2,A4,A6) and (A6,A4,A2,A0), with coordinates(u^2,V) and(1/u^2,V/u^3). The implementation includes X=l,m and infinity charts, then converts the general cubic to short Weierstrass form before group tests. The construction asserts no isogeny to the original E merely from these equations.

## Execution, failures and replay

Source70cd70daf181278e1bbee4194ef3b8f4b99dcaee was published before first build/data. Build101 stopped at missing explicit const-generic type in the small-field minus-one lookup. Correction2a72e643377ac5af14df23e1a21d949bf1424e2a was published before first native data; the changed build passed. The unused small-field parameter warning remains recorded.

The small-field executable exited101 at wrong>0. It incorrectly requires a wrong-Z rejection in every fiber rather than some retained points as frozen. Its final JSON was not emitted; no complete small-field count, L-polynomial panel or small positive toric control is claimed. No scientific rerun was performed. The full236fiber panel remains pending under a separately authorized corrected attempt. This execution record preserves that gap rather than silently changing the control after data.

The P192 executable exited0. A fresh source copy compiled offline and reproduced its155827byte record exactly. Flipping one stored order-annihilation boolean was rejected with exit101. Fresh replay and mutation are declared qualification checks, not additional scientific trials. All commands retained120second limits, one build job and one native process; cloud jobs,generators and scientific retries were zero.

The P192 record counts110423 quadratic multiplications and12833 quadratic inversions, excluding inverse internals. Special construction plus group qualification accounts for73624/9108 of those calls. The legacy point-add counter does not include generic quotient Model additions, so its value must not be presented as complete group-operation cost. These are instrumentation counts of the complete qualification program, not an attack cost or timing comparison.

## Sources and next construction

Sources checked (KB): loaded tools and migrated wrapper were unavailable in the earlier frozen research pass. Bounded prior reports: Round59 signed surface and Round69 norm/probe. Primary sources read before construction: Kawakita, Fibre products of three hyperelliptic curves with many points, section3, https://manypoints.org/upload/1258693314.pdf (finite-field V4 decomposition); Snowden, Lecture19, https://websites.umich.edu/asnowden/teaching/2013/679/L19.html (nodal Picard gluing); Marcucci, On the genus of curves in a Jacobian variety, section2, https://www.numdam.org/item/ASNSP_2013_5_12_3_735_0.pdf (complex-field background only). The finite-field boundary formula above is derived directly. No novelty assertion is made for fiber products or the standard decomposition.

The next scalar-action construction should return to a costed horizontal principal relation with a fully specified action on G and Q, then an implementable orbit representative. Keep class-number/L-value effects confined to relation discovery and cost until that explicit action is available. Another permitted theoretical direction is parameter-changing transport in the total family; it must account for changing target groups and cannot reuse a fixed-fiber group law implicitly. The unfinished small panel is separate from either proposed construction.
