# Exact transfer implemented by the experiment

The source is the Scholten family in Bernstein–Lange, *Hyper-and-elliptic-curve
cryptography*, sections 2–3 and Figure 2.7:
https://www.cambridge.org/core/services/aop-cambridge-core/content/view/4551593BD96904BCD48747DBB89F9615/S1461157014000394a.pdf/hyperandellipticcurve_cryptography.pdf

For p=3 modulo 4, work in F_p[i], i²=-1. Put r=33+56i,
s=159+56i and w=-384. The original elliptic model is
w y²=r x³+s x²+conj(s)x+conj(r). This differs from the normalized short
source used by rho only by X=c3*x+c2/3, Y=c3*y, where c_j are its
coefficients after division by w. The implementation retains both directions.

Set A=i+(1+i)t, B=-i+(1-i)t. The one-infinity cover is

    F(t) = (r A^6+s A^4 B²+conj(s) A² B^4+conj(r) B^6)/w,
    C: Y²=F(t),
    pi(t,Y)=((A/B)²,Y/B³).

The degree-six coefficient vanishes. Every remaining coefficient is in F_p;
the constructor checks both facts and squarefreeness. The cover at infinity
maps to the elliptic two-torsion point T=(-1,0).

For a finite source P=(x,y), the pullback divisor is represented by
U=A²-x B² and V=y B³ modulo U. A linear U is handled as a degree-one
Mumford divisor; the identity and torsion cases are included. Define

    tau(P)=pi*(P-O)+Frobenius(pi*(P-O)).

The implementation composes these two Mumford divisors over F_p² and checks
that their reduced coefficients descend to F_p. The dual norm sends a reduced
divisor sum(P_j)-d*infinity to sum(pi(P_j))-d*T. Exhaustive small-field checks
verify norm(tau(P))=2P, and all primary models have seeded addition/map checks.
These identities imply injectivity on the selected odd prime subgroups.
They do not imply that the factor base is smooth with a random-model probability.

Correctness tests exhaust every source point on eight small models, including
identity, torsion, degree drops and exceptional additions. A second existing
Jacobian backend supplies independent arithmetic checks. Larger source group
orders are certified by an r-torsion point with prime r>4p dividing N, where
N lies in the Hasse interval [(p-1)²,(p+1)²]; at most one multiple of r lies
in that interval. These are exact finite witnesses, rather than probabilistic
point-order estimates. The witnesses and normalized models are in fixtures.json.

The generic fast degree-two addition batches its two denominators into one
inversion and falls back to complete Cantor arithmetic when either vanishes.
Its measured primitive counts are reported as implemented. The supplied
41M+I formula is not assumed to describe this polynomial backend.

The integer coefficient certificate reduces the cover numerator modulo
`i²+1` to `(-384,-3200,-9568,-12160,-5984,-960)`. Equivalently the model is
`12Y²=12+100t+299t²+380t³+187t⁴+30t⁵`. An explicit quotient witness makes the
polynomial identity valid in unconstrained variables; the repository checker
accepts 64 identity-derived evaluations and 64 additional checker evaluations.
It is not a Lean proof of the geometric map. The independent zeta checks count
C over both F_p and F_(p²) and use `#J=(#C(F_p)^2+#C(F_(p²)))/2-p` on the eight
small models, agreeing with complete source elliptic point counts.
