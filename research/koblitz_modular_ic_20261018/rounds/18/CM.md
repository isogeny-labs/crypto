# Integral level-77 correspondence and explicit elliptic-map search

CM19 evaluates a genuine deck correspondence on the actual incidence chart.
For A=1+c, K=b+A c, set u=c³/K and s=K/c. Re-normalizing the Tate model at
2P11=(b,bc), using x=u²X+b and y=u³Y+s u²X+bc, gives

    b'=bc/u³, c'=1+A/u,
    lambda'=(lambda+s)/u,
    nu'=(nu+lambda b+bc)/u³.

The curve is y²+Axy+by=x³+bx². The denominators c,K are nonzero in the
retained order-11 chart. The tangent at2P has slope s. Exact substitution
checks the new Tate coefficients. The universal integer residual in
KoblitzRound18Tate expresses the coordinate difference as a tangent
constraint plus2 times an explicit polynomial. Its characteristic-2
consequence and normalized Tate-form implication are proved in Lean.

All ten positive F32 affine incidence points are permuted by this map.
Its fifth iterate is the frozen diamond involution
(lambda,nu)→(lambda+A,nu+b); its tenth iterate fixes every input.
The coordinate residual is checked on10,240 pairs, including pairs off
the elliptic curve;210 curve-point controls agree. The moduli interpretation
supplies extension to the smooth normalization and the order-ten action.
The graph therefore gives an integral degree-one Jacobian correspondence
under that written bridge. This is not the missing A5→J factor inclusion.

## Why adding primitive diamonds does not separate the rational11 lines

For the actual CM type alpha²-alpha+2=0, the roots modulo11 are5,7.
With2 a primitive residue generator, their eta exponents are4,7.
The eight coefficient embeddings have k=1,3,7,9 and alpha=5 or7. Their
Satake roots are alpha*2^(4k), (1-alpha)*2^(7k), with diamond value2^k.
There are exactly two joint collisions between the E1/E0 branches:
(F,D)=(2,2) and(1,6). The latter is the actual rational11 system. Inverting
the diamond convention inverts D and leaves the collision intact.
These are coefficient-field residue controls, not an identification of the
unknown integral modular Tate lattice.

At k=9, alpha=5, the finite character is inversion modulo11. Consequently
phi(gamma) reduces to1 for every nonzero conductor residue. Native checks
all ten units and all100 pairs of nonzero CM coordinates. The induced
unramified residual Hecke system is1+epsilon_-7: trace2 at split primes,
trace0 at inert primes, with diamond epsilon_-7(p)/p. Exact norm-generator
checks cover23 unramified primes through97. Lean proves the actual finite
unit assertion, the common rational system, and the impossibility of a
scalar polynomial separating two identical systems.

This refines the failure of a Frobenius-only projector. It does not rule
out arbitrary integral geometric maps, divide a projector by a nonunit,
or compute the actual modular Hom lattice. Round17's conditional written
prime-to11 map-existence argument remains in force. An evaluated factor
inclusion and its subgroup image remain open.

## Complete affine-linear F32 ansatz, followed by independent holdouts

CM21 exhausts every x in span_F32(1,b,c,lambda,nu):33,554,432 coefficient
vectors per target sign. For each x passing the ten actual incidence
points' necessary x-coordinate condition, F2 linear algebra solves for
all y in that same five-dimensional F32 span. Every solution is substituted
back into every equation. Known elliptic identity maps provide positive
solver controls, and a regression checks both y and y+x signs.

| target coefficient a | x-valid linear systems | constant fits | nonconstant finite fits |
|---|---:|---:|---:|
|0|792646|43|90|
|1|805|21|12|

A finite fit is not a map. CM22 constructs eight new incidences over F1024,
on distinct Frobenius orbits of Tate bases, by independently finding actual
order-7 points over F2^30. It retains the exact order-11 checks, full
7-primary cofactors, order reduction, stable QR triples, embedding tables
and all inverse-witness incidence checks. The search inspects133,498 Tate
pairs and141 order-11 bases; torsion attempts are at most4, with no cap hit.
All90 and12 nonconstant finite fits have explicit nonzero counterexample
residues. All43 and21 constant controls survive.

A generated Lean module recomputes those102 residues by binary polynomial
convolution and long division, rather than the native field multiplication
algorithm, and proves every residue nonzero. This verifies the exported
counterexamples; it does not formalize the geometric point construction
or the complete native search exhaustion.

Thus no nonconstant map exists in this specific affine-linear ansatz,
under the written incidence interpretation and verified native exhaustion.
Maps with denominators or higher-degree numerators remain untested. The
next map construction should allow controlled poles on the proper modular
normalization and verify its function-field identity, divisor action and
actual11-primary image. Neither the deck action nor this bounded negative
search completes the requested integral factor map.
