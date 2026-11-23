# Character domain as a phase certificate

Let r be the certified odd prime, h=(r-1)/2, m>=3 divide h, L=h/m,
and epsilon in {1,-1}. The existing character factory returns disjoint
squarefree factors g of D_L(W)-2epsilon. For any root w of g and any nonzero
v satisfying v+v^-1=w, the Dickson identity gives

    (v^L-epsilon)^2 = v^L (D_L(w)-2epsilon) = 0.

In a field this implies v^L=epsilon. This argument is over an algebraic
closure, so it does not rely on enumerating base-field points. Every later
canonical intersection retains a subset of these geometric points.

## Initialization

CanonicalFiber::new separates W=+/-2 and stores their unique roots v=+/-1.
Away from these endpoints the quadratic v²-wv+1 is separable. Therefore the
canonical representation is reduced even though the unseparated quadratic
quotient at an endpoint contains nilpotents. Imposing v^L-epsilon on this
canonical representation changes no retained point. The specialized
initializer omits that power and canonical intersection, while retaining
canonical squarefree validation and all observation constraints.

It would be incorrect to claim v^L-epsilon is the zero element in the raw
quadratic quotient at an endpoint merely because its square is zero. The
implementation removes the intersection only after canonical endpoint
handling; the qualification compares canonical signatures, not raw pairs.

## Lifting

For monic squarefree g of degree d, put R(V)=V^d g(V+V^-1).
Its roots are the reciprocal preimages of g. Away from W=+/-2 each root
has multiplicity one, because the derivative of V+V^-1 is nonzero.
Each included endpoint produces exactly (V-1)^2 or (V+1)^2. Divide R
by V-1 once if g(2)=0, and by V+1 once if g(-2)=0. The result is precisely
the squarefree reciprocal preimage, and every root already satisfies the
character phase by the identity above. No modular V^L power or phase gcd
is needed. The old 2d output-capacity preflight is retained exactly.

The selected-orientation part still applies V-root(W), and final recovered
scalars still undergo point verification. This is an internal constructor
invariant, not a generic unchecked simplification: arbitrary W factors and
quartic phases do not qualify. The negative control uses r=19,L=3,g=W,
whose reciprocal roots do not satisfy V^3=1.

## Work still charged and next construction

The initializer still constructs degree-proportional-to-L domains and checks
squarefreeness. The current reciprocal substitution still uses a coefficient
loop with quadratic dense work. Endpoint evaluations/divisions, canonical
filtering, allocations/copies, source extraction, section evaluation,
streaming, history replay and final checks remain charged. Removing these
specific phase powers is not a total-work or timing ranking.

A next construction is divide-and-conquer homogeneous substitution for
R(V). If g=g_lo+W^k*g_hi with deg(g)=d and deg bound(g_lo)=k-1,
R_g=V^(d-k+1)R_lo+(V²+1)^k R_hi. This recurrence permits shared powers
and balanced polynomial products while preserving coefficient zeros and
endpoint correction. It is proposed here, not implemented or measured.

## Sources checked

- Predecessor character-domain.rs, canonical.rs and reciprocal.rs at
  84bdf57f7cb99ef35803970f3159a74542192920.
- NIST DLMF18.5.1-2, https://dlmf.nist.gov/18.5 : reciprocal definitions
  of first/second-kind Chebyshev polynomials and the Dickson identity.
- Research KB wrapper returned no executable; no novelty claim is made.
