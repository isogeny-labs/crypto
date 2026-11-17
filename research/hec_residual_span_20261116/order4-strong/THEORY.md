# Strong order-four controls without fourth-extension enumeration

This is an exact input-qualification method, not a DLP speedup or a priority claim. The family remains y²=x⁹+a7*x⁷+a5*x⁵+a3*x³+2*x over prime fields p=5 mod8. The order-four map and simplicity arguments are those separately derived in order4/THEORY.md. No binomial identity pi=P(sigma) is imported.

## A certified integer order interval

For genus four the Hasse interval is [(sqrt(p)−1)^8,(sqrt(p)+1)^8]. Write its endpoints as A±B*sqrt(p), where

 A=p4+28p3+70p²+28p+1,
 B=8p3+56p²+56p+8.

Since A is integral, the exact enclosing interval for integer #J is [A−floor(sqrt(B²p)),A+floor(sqrt(B²p))]. The implementation uses integer square root and no floating rounding for this certificate.

Counts over Fp and Fp² give c1,c2 by Newton identities. The reciprocal Frobenius polynomial yields

 #J=K+(p+1)c3+c4,
 K=p4+1+(p3+1)c1+(p²+1)c2.

Weil root magnitudes imply |c3|≤56*p^(3/2) and |c4|≤70*p². Thus K±R, with R=(p+1)*56*p*ceil(sqrt(p))+70*p², is another certified integer interval. Intersect it with the Hasse interval.

Every verified rational point order divides #J; therefore their LCM L does too. If exactly one multiple of L occurs in the intersection, it must equal #J. This does not assume that the points generate the whole Jacobian or that L itself is #J. The native BSGS search is only a producer of proposed point orders: [o]P=0 and [o/r]P!=0 for every prime divisor r independently certify each order before it enters L. The known nonzero branch-point class (0,0)−infinity has order two and initializes L=2.

A count over Fp³ then gives c3. The certified #J recovers c4 from the displayed identity, giving the entire degree-eight Frobenius polynomial. Its fourth trace is checked against the Weil bound. Six earlier full-count records calibrate this reconstruction against independently enumerated fourth-extension counts; the new search itself does not enumerate Fp4. Small calibration intervals may contain several multiples and are not falsely assumed unique.

## Selection and group strength

The branch polynomial outside x is required irreducible of degree eight. This controls the rational 2-torsion rank and avoids an unnecessary extra branch-factor bias; it does not prove absolute simplicity. The constant2 is a nonsquare at the fixed p5mod8 fields, permitting such an irreducible even polynomial. Singular/reducible models are retained as rejections.

Only exact orders with largest odd prime ell of valuation one and actual cofactor #J/ell≤4 proceed. No upper-bound surrogate is used for this cofactor. Ordinary irreducibility, finite absolute-simplicity witnesses and original-group action/projection checks are still required. Coefficients are selected by a frozen grid and algebraic eligibility, never target runtime or a desired speed ratio.

The missing proof obligations are unchanged by a faster order certificate: independent modular replay must verify declared absolute models; strong subgroup and actual target solves remain separate. No index-calculus performance claim follows from this search.
