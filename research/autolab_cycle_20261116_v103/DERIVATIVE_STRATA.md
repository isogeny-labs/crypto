# Why the proposed exceptional family has a distinct algebraic stratum

This derivation is general in characteristic two; it is not another executed
experiment and does not assert the untested cubic/guard equivalence.

Write H=X^5+tX^4+aX^3+bX^2+cX+d. Its formal derivative is

    H' = X^4+aX^2+c = (X^2+sqrt(a)X+sqrt(c))^2.

Direct polynomial addition gives

    H + (X+t)H' = (b+t*a)X^2 + (d+t*c) = R.

Consequently gcd(H,H')=gcd(R,H'). If R=0, H=(X+t)Q^2 with
Q=X^2+sqrt(a)X+sqrt(c); this is exactly the family proposed to explain the
496 cubic false positives. It is the maximal repeated-factor stratum:
gcd(H,H') has degree four.

If R is a nonzero constant, H is squarefree. If R has degree two, it is a
scalar times the square of a unique linear polynomial over the perfect
finite field. Thus gcd(H,H') has degree either zero or two. There is no
intermediate degree-four repeated-factor case outside R=0. This accounts
for every branch, including zeros and multiplicities, without dividing by a
potentially zero coefficient. A future implementation must dispatch on the
coefficient pair of R before inversions or quadratic recovery.

The proposed finite-certificate result is therefore precise: for the six
actual GF(32) predicates, are the observed cubic equations sufficient on
R!=0, with a quadratic trace guard resolving all R=0 cases? The C103 evidence
does not yet prove this implication; C104 must check the complete masks.

If true, this supplies an exact characteristic-two local predicate with an
explicit exceptional branch. It does not imply a global factor search, nor
does it transfer unchanged to odd characteristic, where polynomial squaring
has cross terms and the derivative structure differs. This is a concrete
reason to keep separate binary and odd-characteristic backends while sharing
the exact all-roots interface and performance gate.
