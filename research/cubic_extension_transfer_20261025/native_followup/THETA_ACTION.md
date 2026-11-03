# An explicit rational three-torsion lift

This is a concrete symbolic calculation toward the square-class problem.
It replaces a generic-point interpolation step by a rational formula. It
does not establish any relation among the neighboring chi18 obstructions.
The construction is classical theta-group/elliptic translation arithmetic;
novelty of this presentation is not claimed.

Let the characteristic differ from 2 and 3. For s,v with
v != 0 and v != 4*s^3, put

    u = 3*s^2,  m = 3*s,
    a = 6*s*v - 27*s^4,
    b = v^2 - 18*s^3*v + 54*s^6.

On E: y^2=x^3+a*x+b, P=(u,v) is a flex and has order three:
the tangent slope is m and m^2=3u, so 2P=-P. Conversely, for any
nonzero three-torsion point on a nonsingular short-Weierstrass curve,
v=y(P) is nonzero, the tangent slope obeys m^2=3u, and s=m/3 gives
these parameters. The discriminant calculation is

    4*a^3+27*b^2 = 27*v^3*(v-4*s^3).

In the row basis (1,x,y) of H0(E,3O), define M=N/(2v), where

    N = [ m*u-v    m*u^2-3*u*v    3*v^2-u*m*v ]
        [ -m       2*v-m*u        m*v           ]
        [ -1       -u             -v            ].

For a generic R=(x,y), direct substitution into the elliptic addition
formulas gives the rational-function identity

    (1,x,y) M = -(v-m*u+m*x+y)/(2*v) * (1,x(R+P),y(R+P)).

The numerator is the tangent section at -P. The same explicit matrix
has M^3=I. Thus it is invertible and extends the translation action on
the space of sections across the apparent rational-function poles.
The normalized tangent coefficient is alpha_P=-1/(2*y(P)).

## Exact evidence

theta_identity.cpp performs integer polynomial expansion after the
substitution u=3s^2. For translation it clears powers of x-u and proves
that the remaining polynomial is an explicit multiple of
y^2-x^3-a*x-b. It emits the quotient in the certificate statement; no
sampled elliptic points are used for this calculation. For order three it
checks all nine entries of N^3=(2v)^3*I, aggregated with independent formal
weights. It separately verifies the discriminant factorization.

The identity.certificate/v1 records are replayed by the repository's
existing Rust checker with extra checker-chosen points and false-identity
controls. These are evaluated certificates plus exact integer coefficient
calculations, not Lean proof artifacts. IDs and verdicts are retained in
results/theta-symbolic-audit.json and results/theta-certificate-replay.json.

normalization_family_audit.cpp compares the formula with the previous
generic-point action construction on all saved component/source fixtures.
This checks convention compatibility on those fixtures. It does not prove
all global cross-point theta cocycle conventions or Frobenius descent.

## What remains for extra-step coverage

This calculation makes the three-torsion action rational and explicit.
One still needs a consistent level-nine labelling for every eligible
neighbor, the resulting chi18 rational functions, their divisors modulo
two, and the arithmetic square classes of constants. No odd-parity or
conditional norm-product identity is inferred from this translation lift.
