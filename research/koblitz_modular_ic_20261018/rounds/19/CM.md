# Controlled-pole level77 map spaces

CM23 uses the actual CM17 Tate/QR incidence coordinates b,c,lambda,nu.
The ten original F32 points and all eight independently constructed CM22
F1024 points are retained unchanged. Coefficients lie in F32 with polynomial
z^5+z^2+1. The same recorded embedding into F1024 is used throughout.
For each q in1,b,c,1+c,K=b+(1+c)c, search

    x=N/q^2, y=M/q^3,

where N is any F32 linear combination of1,b,c,lambda,nu and M is any
F32 linear combination of the15 monomials of total degree at most2.
The target is y^2+xy=x^3+a*x^2+1 for each a=0,1. For fixed x this is
an F2-linear system in the75 coefficient bits of M. The solver retains
the particular solution and complete nullspace and enumerates all its
solutions. No space exceeds the registered enumeration cap.

| q | x forms per sign | x valid a=0 | x valid a=1 | consistent systems a=0,a=1 | finite maps |
|---|---:|---:|---:|---|---|
|1|33554432|1935|11|22,11|43,21 constants|
|b|33554432|1934|6|0,0|none|
|c|33554432|1978|4|0,0|none|
|1+c|33554432|1991|5|0,0|none|
|K|33554432|1918|4|0,0|none|

All335544320 x numerators are examined. All64 finite fits have only
constant coefficients; they are actual constant maps and are checked
again by an independent Lean binary polynomial convolution/reduction.
The known identity map and its elliptic conjugate satisfy the same native
linear solver in a positive control. An independent exhaustive small
binary-system test checks every point of the returned affine solution
space. The per-family JSON records every skipped zero denominator and
every retained fit. Constraints at a zero denominator are omitted.

## Exact pole scope

On the smooth proper normalization, let D be the sum of the positive pole
divisors of b,c,lambda,nu. The elementary valuation inequality gives
(N)_infinity<=D and(M)_infinity<=2D. Consequently the poles of x and y
are bounded by D+2(q)_0 and2D+3(q)_0. Cancellation can lower these bounds.
They are genuine divisor bounds for these rational functions, rather than
an assertion that q is a cusp parameter. The actual multiplicities and
supports of D and(q)_0 have not been computed. They do not identify a
specified cuspidal Riemann--Roch basis.

The weighted clearing identity is

    q^6 E(N/q^2,M/q^3)
      =M^2+q*N*M+N^3+a*q^2*N^2+q^6,

where E(x,y)=y^2+xy+x^3+a*x^2+1 and q!=0. Lean proves both the
polynomial weighted identity and this field consequence. Divisor geometry,
the exhaustive search and the incidence-normalization bridge remain
written/native, as specified in PROOF_MAP.md.

This excludes precisely the registered coefficient spaces using necessary
actual-point constraints. Higher-degree N, additional denominators, larger
coefficient fields and general correspondences remain untested. The actual
integral factor inclusion and its divisor evaluator are still open.
Round17's conditional prime-to11 existence result is unchanged. The next
map experiment should compute a proper normalization and concrete pole
divisors, then search a complete prescribed Riemann--Roch space with a
verified function-field identity and a checked11-torsion image.
