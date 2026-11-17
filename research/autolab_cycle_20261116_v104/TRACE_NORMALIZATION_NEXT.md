# Next exact coordinate change: a reusable trace-zero predicate

For H=X^5+tX^4+aX^3+bX^2+cX+d in characteristic two, set G(X)=H(X+t).
Binomial expansion gives

    g4=0, g3=a, g2=b+t*a,
    g1=c+t^2*a+t^4,
    g0=d+t*c+t^2*b+t^3*a.

This is an affine bijection on the four variable coefficients for fixed t.
Its inverse is a=g3,b=g2+t*g3,c=g1+t^2*g3+t^4 and

    d=g0+t*g1+t^2*g2+t^3*g3+t^5.

The inverse constant term includes t^5; omitting it would be incorrect.
Root translation is a bijection of the whole finite field and retains every
multiplicity. Hence H splits iff G splits. All cubic Boolean relations pull
back to degree<=3 equations through this affine F2 coordinate map. Evaluate
the transformed wires directly instead of expanding those substitutions.

The singular residual transforms to g2=0,g0=0: g2=b+t*a and
g0=(d+t*c)+t^2*(b+t*a). Thus the proved quadratic guard transfers exactly.
A canonical field isomorphism is F2-linear on coordinates, preserves products
and absolute trace, and can transfer a single certified template per field
degree to other irreducible residue moduli of that degree. The transformed
relation density and total wire cost still need measurement; affine
equivalence alone is not a performance claim.

Next experiment: derive/select exact predicates at t=0 in canonical fields
GF(2^s),s=1..5; verify complete local cubes, the inverse map, all repeat/zero
branches and each field isomorphism; then compile on the SAME76global bits.
Freeze all templates and arithmetic costs. A target-independent template
could be reusable, but field construction, coordinate mapping and every
instance-dependent step must remain charged to end-to-end comparisons.

For odd characteristic, depressing a quintic requires translation by a
multiple of1/5, so characteristic5 is an exceptional case. NIST primes do
not have that obstruction, but their cross terms and quadratic splitting
conditions differ. Do not reuse the binary formulas or the Artin–Schreier
guard as an odd-field proof. This motivates separate exact backends sharing
the same original-root interface and unchanged release gate.
