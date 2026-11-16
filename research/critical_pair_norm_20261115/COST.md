# Conditional bounded-degree work envelope

This is a hand bound on a degree-capped standard producer, not a sufficient
degree or positive-yield result for the norm systems. Actual software retains
small fixed admission/caps. It does not implement the unbounded parameter
family used in this conditional discussion. [pcc:unverified]

D bounds actual DAG-node and critical-product degrees, rather than final
printed degree or expanded input-multiplier degree. No flat Macaulay
certificate-degree theorem is supplied by these controls. [pcc:unverified]

Let v be the number of variables, s the number of input equations and
M=binom(v+D,D). Every admitted nonzero remainder has a leading monomial not
divisible by an earlier basis leading monomial: the native reduction visits
all current divisors before retaining an irreducible term. Thus each installed
remainder strictly enlarges the generated leading ideal inside the M columns;
there are at most M such basis elements. There are at most M² unordered
critical pairs and s initial reductions. [pcc:unverified]

Under a degree-compatible order, each reduction replaces its current term by
smaller terms. It visits at most M column positions per source/S row. Each
update and divisor scan has polynomial cost in v,D,M; a monomial multiplier
needs at most D variable-product nodes. Cache/status/queue storage, source
links and proof replay therefore have polynomial overhead in s,v,D,M, ignoring
the fixed implementation caps. Chain filtering is optional for this envelope.
It can affect a bounded schedule without improving this asymptotic upper
bound. Finite-field bit costs must also be charged. [pcc:unverified]

All exported basis polynomials have degree at most D. Final checker S-pairs
have degree at most2D and reductions do not raise it, so their universe has
binom(v+2D,2D)<=M² monomials. This retains a polynomial-in-M bound for full
completion/original-inclusion checking. Every exported polynomial also has a
source derivation; completion alone would only identify its own ideal.
Quotient enumeration/conversion/extraction have their separate limits and
must be charged on both successful and failed attempts. [pcc:unverified]

If a uniform sufficient D=O(m+t) and adequate inverse positive yield were
established for the existing v=O(mt) norm family at balanced m,t, this envelope
would fit the already reviewed subexponential selective-oracle reduction.
Neither the producer nor the two tiny panels establishes those premises.
The four positive auxiliary cases still have no accepted source certificate
at these caps. The eight unit ideals in001 give empty-set certificates only.
The old source review remains the controlling proof obligation.
[pcc:unverified]
