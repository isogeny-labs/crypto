# Level77: explicit affine incidence and the integral factor obstruction

Round16 continues Round15's CM construction on the actual n=5, level77
carrier. Exact arithmetic below is replayed by --round16-cm and
--round16-carrier. The compact modular interpretation remains a written
bridge; no normalized proper model or Koblitz-factor evaluator is asserted.

## The stabilizer and its moduli interpretation

For the level77 character epsilon7*eta11, eta11 has order10. Its kernel H
consists of d11=1 with d7 a quadratic residue, and d11=-1 with d7 a
nonresidue. A configuration (E,P11,{Q,2Q,4Q}) has exactly this stabilizer
in the diagonal modular group, after the simultaneous sign identification.
Here P11 has exact order11 and Q exact order7. This is a degree16 cover
of X1(11): there are48 nonzero7-torsion points, divided into16 triples.
It is the Gamma_H curve of Round15, whose compact genus61 was checked
by two independent coset computations. This identifies the smooth modular
normalization of our open incidence, not every component of a naive closure.

Use the characteristic2 Tate form

    E: y²+Axy+by=x³+bx², A=1+c, P11=(0,0).

The discriminant is b³(b+cA³). The native circuit requires it nonzero,
c nonzero, x(4P11) nonzero and Ac+b nonzero. These denominators do not
vanish for an exact order11 point in this Tate chart. The exact formulas
in carrier16.rs impose5P11=-6P11. Independent group arithmetic checks
11P11=O on every finite-field base. The chart also permits A=0;
it is not asserted to be exclusively an ordinary elliptic locus.

For y=lambda*x+nu put

    q=x³+s1*x²+s2*x+s3,
    s1=b+lambda²+A*lambda, s2=A*nu+b*lambda, s3=nu²+b*nu,
    D=A*x+b, M=x²+A*lambda*x+A*nu,
    N=M²+A*M*D+b*D²,
    L=(M+(A+lambda)*D)*N+(M+lambda*D)*x*D²+b*D³.

The three coefficients of L mod q vanish. Inverse witnesses retain
s1*s2+s3 !=0 (separability of q) and
b³+A*b²*s1+A²*b*s2+A³*s3 !=0 (Norm_q(D)). Thus doubling is defined and
preserves the reduced three-point intersection. The group-law sum of its
points is zero. Doubling is injective there; a fixed nonzero point is
impossible. A two-cycle would consist of Q,-Q with3Q=0, forcing its third
point to be O, excluded by the finite line chart. It is therefore a
three-cycle, giving8Q=Q and7Q=0. Conversely the triple of any nonzero
7-torsion point is a reduced nonvertical line intersection satisfying these
equations. This group/moduli bridge is written, with polynomial and scalar
identities formalized separately.

The JSON stores lossless quadratic Add/Mul/Inverse circuits. An inverse
node u of d means d*u=1. Input coordinates are b,c,lambda,nu; residual
nodes vanish. These are explicit affine equations over F2, not projective
saturation or an explicit map from their Jacobian to A5.

## Diamond twist derived over F4

The diamond involution sends(lambda,nu) to(lambda+A,nu+b). It fixes q,
is an involution, and has no fixed affine point because b!=0. Choose
w²+w=1. Substituting y_old=y+w(Ax+b) changes the Tate curve to

    E': y²+Axy+by=x³+(b+A²)x²+b².

It sends lambda_old=lambda+wA, nu_old=nu+wb, hence shifts s1 by A²
and s3 by b² while fixing s2. It sends N to N+A²D² and fixes both
M+(A+lambda)D and M+lambdaD. These substitutions produce the second
explicit F2 incidence circuit. Under w↦w+1 its coordinate change differs
by the diamond involution, so it is the quadratic diamond twist, without
inferring a twist merely from counts. The P11 data parameterize the
original E; they are not claimed rational on E'.

Native exact counts for degrees k=1,2,3,4,5:

| k | Tate order11 bases | original incidence | twisted incidence |
|---:|---:|---:|---:|
|1|0|0|0|
|2|0|0|0|
|3|0|0|0|
|4|20|0|0|
|5|15|10|10|

For every base over F16 and F32, both incidence sets agree exactly with
an independent enumeration of actual7-torsion over F_(q³), partitioned
into QR triples and tested for q-Frobenius stability. A z²+z table solves
all point quadratics; generic addition/scalar multiplication supplies the
independent torsion test. Positive agreement occurs over F32. The earlier
F2/F4/F8 controls were vacuous and are retained as such. The next run hit
the2b conservative cap; the preregistered8b repeat completed with charge
2415610624. Charge is an upper bound, not a measured operation count or
an IC cost.

## Why the obvious Frobenius sign projector fails at11

CM16 computes the actual two degree8 trace-zero polynomials Pminus,Pplus.
Their resultant is29721861554176=2^24*11^6. The primitive integer Bezout
identity has right side704=64*11. Modulo11 their gcd has degree6; both
polynomials vanish at X=1. Modulo3,5,7,13,19,23,29,31 their gcd is1.

The exact coefficient lists and Bezout multipliers are in the JSON;
Lean proves that identity universally and proves no Bezout identity with
right side1 exists modulo11. Hence a Frobenius-polynomial separation
based on coprimality is not integral at11. Both Pminus(1) and Pplus(1)
are11, so evaluating the opposite factor on rational11-torsion annihilates
it rather than proving subgroup preservation. This excludes that particular
sign-projection method. It does not exclude an arbitrary integral
correspondence, a different quotient, or an injective subgroup map.

The next map experiment must construct an integral correspondence and
compute its actual11-primary kernel or composition. Dividing a rational
projector by704 cannot be treated as a unit-at11 operation.
