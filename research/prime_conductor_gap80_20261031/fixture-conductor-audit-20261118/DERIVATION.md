# Conductor certificates for four retained maps

All endomorphism orders here are geometric orders of ordinary elliptic curves.
The fields have prime characteristic greater than three. This is a standard
order/CM argument with native arithmetic checks, not a new theorem or a compiled
Lean proof. The numerical checks are unsigned producer attestations.
[pcc:unverified]

## Ordinary order and prime-degree criterion

Write `Delta_pi=t^2-4p=f_pi^2 D_K`, with fundamental `D_K`. Then
`Z[pi] <= End(E) <= O_K` and the endomorphism conductor `f_E` divides `f_pi`.
An ordinary prime-degree ell isogeny, with ell different from p, preserves
all conductor valuations except possibly that at ell; the two ell valuations
differ by at most one. Thus ell not dividing f_pi certifies a horizontal
edge. This follows from the prime-degree volcano structure, rather than from
a comparison of curve equations or from the degree alone.

Primary sources: [Galbraith, introduction and section 2](https://antsmath.org/ANTSXVI/papers/Galbraith.pdf)
and [Galbraith–Gilchrist–Robert, section 2.1](https://www.math.u-bordeaux.fr/~damienrobert/pro/publications/articles/climbing_volcanoes.pdf).
The checked versions and PDF hashes are in sources.json. [pcc:unverified]

Both small fields have freshly recomputed endpoint cardinalities. The degree19
fixture has fundamental Frobenius discriminant, so both actual conductors are
one. The degree10009 fixture has Frobenius conductor three; its isogeny prime
does not divide this, so its conductor ratio is one. No identification of
f_pi with f_E was made. [pcc:unverified]

## Resolving the two degree10009 endpoint orders

For this fixture, put beta=(pi-1)/3 in the endomorphism algebra. Its trace is
`(t-2)/3` and its norm is `#E(F_p)/9`; both are integral, and its discriminant
is the fundamental `D_K`. Therefore `Z[beta]=O_K` in the field. Each actual
endomorphism order is either `O_K` or `Z+3 O_K`. [pcc:unverified]

The full three-torsion is rational exactly when Frobenius minus one kills
E[3]. By the universal quotient property of the separable map [3], this is
equivalent to beta being an endomorphism. Since beta generates O_K, this is
equivalent to the curve having conductor one. Otherwise its conductor is
three. The checker counts rational three-torsion using the separable polynomial
`3x^4+6Ax^2+12Bx-A^2` and the rational ordinates at every root; it independently
checks the result against PARI group invariant factors. Both endpoints have
three rational three-torsion points, hence conductor three. This also checks
the horizontal conclusion independently at the only possible conductor prime.
[pcc:unverified]

## Explicit degree2 descending map on the target field

The source is `y^2=x^3-140x+784`, with `j=-3375`. The frozen field prime splits
in Q(sqrt(-7)), so the good CM reduction is ordinary. Independently, for
`s^2=-7`, set `r0=7+s`, `u=(1-s)/2`. The normalized degree2 quotient with kernel
`{O,(r0,0)}` has coefficients `u^4 A,u^6 B`, so composing with inverse scaling
gives a degree2 endomorphism of differential u. The native check verifies the
quotient/scaling and `u^2-u+2=0`. [pcc:unverified]

For an elliptic endomorphism of degree d, trace is an integer bounded by
`2 sqrt(d)` and reduces to the sum of the endomorphism and dual differentials.
Here the trace reduces to `u+2/u=1`. The characteristic exceeds the possible
trace differences, hence the integral trace is one and the order generated
has fundamental discriminant minus seven. The source order is maximal, with
conductor one. This uses the standard trace/norm identity, not finite point
testing as a substitute for an endomorphism identity. [pcc:unverified]

The audited map instead has rational kernel `{O,(-14,0)}`. A prime-degree2
edge from this maximal source has target conductor one or two. The maximal
order has class number one (the only reduced form is `[1,1,2]`), whose unique
geometric j invariant is minus3375. The target j is different. Thus its
conductor is two. This proof does not require recounting the large-field
cardinality or inferring the actual order from a supplied Frobenius trace.
[pcc:unverified]

## The pilot degree5 edge

The frozen source has `j=-884736` and the prime splits in Q(sqrt(-19)). The
source and target j invariants are equal. The native check additionally
constructs their scaling: if `C=B' A/(B A')`, then `C^2=A'/A`, `C^3=B'/B`.
For `u^2=C`, `(u+5/u)^2=1`. The normalized degree5 isogeny followed by inverse
scaling is therefore an endomorphism of integral trace plus or minus one;
the same degree/trace bound and characteristic argument makes its discriminant
minus19. This is fundamental, so both isomorphic geometric endpoint orders
are maximal with conductor one. This small map is not the retained original
47-bit pilot isogeny. [pcc:unverified]

## Independent direct map checks and subgroup certificate

The checker enumerates every nonzero iK for `1 <= i < ell`, using the actual
prime-order kernel. Direct sums check the normalized quotient coefficients:

    A' = A - 5 sum_Q (3 x_Q^2 + A)
    B' = B - 7 sum_Q (5 x_Q^3 + 3 A x_Q + 2 B).

For a non-kernel P it uses the independent classical formula

    phi(P) = (x_P,y_P) + sum_Q ((x_(P+Q),y_(P+Q))-(x_Q,y_Q)).

This last sum is coordinate addition, not an elliptic-curve group sum.
If P+Q is infinity for some Q in the kernel, the result is infinity. Infinity
maps to infinity. Every retained input/image pair is checked, including
repeated and exceptional inputs. This avoids the rational-polynomial
expansion that was expensive in earlier PARI qualification. [pcc:unverified]

For each fixture the subgroup order q is prime and coprime to ell. Nonzero
G and phi(G) have exact order q, so the kernel intersection with <G> is
trivial. Every supplied scalar k is reverified on both original endpoint
generators, and its direct-map input/image appears in the frozen panel.
These are verification operations, not new logarithm solves or time samples.
The dual identity would permit scalar recovery modulo q because ell is
invertible; no dual-map cost or implementation is supplied here.
[pcc:unverified]

The scope is exactly four retained fixtures. Their classification is not an
exhaustive survey, a world record, or satisfaction of the original minimum
80-bit crossing and measured endpoint criteria. [pcc:unverified]
