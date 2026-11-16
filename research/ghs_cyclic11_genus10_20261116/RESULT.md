# Low-branch cyclic-11 Kummer covers over F3: an exact GHS factor screen

## Scope and result

Consider smooth projective normalizations of rational Kummer equations
`y^11=f(x)` over F3 with exactly three or four geometric branch points.
All ramification is tame. Three branch points give genus five; four give genus
ten by Riemann–Hurwitz. This screen covers the branch orbits and exponent
choices listed below. It does **not** cover arbitrary genus-five/ten curves,
non-Kummer twists, or cyclic covers with five or more branch points.

Within this scope, **no non-descended elliptic Weil-restriction factor over
F243 occurs**. The genus-five equations have trace `-24`, `-9`, or `24`, none
an elliptic trace over F243. Of the 1,093 genus-ten equations, 801 have a
degree-ten Weil-restriction factorization and 292 have no such quadratic
factor in `U=T^5`. The elliptic-eligible component traces are exactly
`{-31,2,31}`. Each is the fifth-extension trace of an elliptic curve over F3,
so these factors do not identify a new non-descended source family.

## Finite classification and proof formula

The point-count shortcut is exact: if `5` does not divide `r`, then
`gcd(11,3^r-1)=1`, so `y -> y^11` is bijective on F_(3^r). Every rational
point of P1 has exactly one point above it and `N_r=3^r+1`. Thus the
genus-five and genus-ten numerator polynomials lie in `Z[T^5]`. We counted
points only over F243 and F59049 using the order-11 residue test and the
correct branch and infinity contributions.

For genus five, write `L(T)=1-t*T^5+243*T^10` and recover
`t=(244-N5)/5`. For genus ten, write `U=T^5` and
`L(T)=1-s1*U+e2*U^2-243*s1*U^3+243^2*U^4`. From counts,
`s1=(244-N5)/5`, `s2=(59050-N10)/5`, and `e2=(s1^2-s2)/2`.
A factor `1-tA*U+243*U^2` exists only if
`D=s1^2-4*(e2-486)` is a nonnegative square giving integer Hasse-bounded
traces `tA,tB=(s1±sqrt(D))/2`. The 292 nonsplit rows fail this exact test.

PGL2(F3) has order 24. The position audit finds one orbit of four rational
branch points; two orbits of two rational points plus a quadratic pair,
of sizes 6 and 12; two orbits of one rational point plus a cubic orbit,
of sizes 8 and 24; and one orbit of two quadratic pairs, of size 3.
For rational Kummer equations, an orbit of four conjugate branch points alone
cannot occur: equal orders at the four conjugates would make infinity a fifth
branch point because `11` does not divide `4a` for `a≠0 mod 11`.

| Genus | Branch configuration | Representative factors in `f` | Exponent vectors |
|---:|---|---|---:|
| 5 | three rational | `x`, `x-1` (infinity also branches) | 90 |
| 5 | rational plus quadratic pair | `x^2+1` (infinity branches) | 1 |
| 10 | four rational | `x`, `x-1`, `x+1` (infinity branches) | 910 |
| 10 | two rational plus quadratic, first orbit | `x`, `x^2+1` (infinity branches) | 90 |
| 10 | two rational plus quadratic, second orbit | `x`, `x-1`, `x^2+1` (infinity unbranched) | 90 |
| 10 | rational plus cubic, fixed rational point | `x^3-x-1` (infinity branches) | 1 |
| 10 | rational plus cubic, other rational orbit | `x`, `x^3-x-1` (infinity unbranched) | 1 |
| 10 | two quadratic pairs | `(x^2+1)/(x^2+x+2)` | 1 |

The full exponent vectors, `N5`, `N10`, and factor tests for all 1,184
parameter rows are in `THREE_BRANCH.json` and `SCREEN.json`. Several rows
describe isomorphic curves; these counts are not isomorphism-class counts.

## Elliptic trace gate and an exact example

The elliptic trace criterion over F243 permits `3∤t` or
`t∈{0,±27}`. The only permitted factors in the genus-ten screen have
`t=-31,2,31`. For a curve E/F3 with trace `s`, its trace over F243 is
`s^5-15s^3+45s`; `s=-1,2,1` gives `-31,2,31`, respectively.
The exact equation `E/F3: y^2=x^3+2*x^2+1` has trace `s=-1`, hence
`#E(F243)=275` and fifth trace `-31`.

For example, the genus-ten normalization of
`y^11=x*(x-1)*(x+1)^4` has `N5=444`, `N10=58700`, and
`L(T)=(1+31*T^5+243*T^10)(1+9*T^5+243*T^10)`.
The `-31` factor is elliptic-eligible but descended; the `-9` factor is
elliptic-ineligible. This is a factorization statement, not an evaluated
subgroup transfer or attack benchmark.

## Verification and prior work

`positions.py` independently enumerates the branch-position PGL2 orbits.
`screen.py` computes every exponent row with finite-field log histograms.
`verify_three.py` independently recounts one representative of each of the
three genus-five trace cells by direct eleventh-power tests over F243.
`verify_representatives.py` independently recounts one representative of
each of the 16 case/Weil-polynomial cells by direct eleventh-power tests
over F243 and F59049. `audit.py` hashes the receipts and applies the
elliptic-source and descent filters. `TraceFilter.lean` checks the finite
trace recurrence and split-pair filter with Lean 4.35.0. The Lean proof
does not formalize the geometry, position enumeration, or point counts.

Cyclic three-branch covers and their automorphism groups were studied by
Kallel and Sjerve (2005). Kodama and collaborators (2009) explicitly
discuss the hyperelliptic genus-five model `y^2=x^11+1`, which is birational
to one of these three-branch covers, and its cyclotomic CM type. Álvarez
studied p-ranks of cyclic-cover reductions through Jacobi sums. The cover
construction and cyclotomic method are therefore established prior art.
This finite factor screen has not been matched to a publication proving
the same exact F3 exclusion; publication priority is unresolved.

- Kallel and Sjerve, *On the group of automorphisms of cyclic covers of the Riemann sphere* (2005): https://math.univ-lille1.fr/~kallel/Papers/pcyclic.pdf
- Kodama, Top and Washio, *Maximal hyperelliptic curves of genus three* (2009), Section 5: https://pure.rug.nl/ws/files/2652897/2009FiniteFieldsApplKodama.pdf
- Álvarez, *The p-rank of the reduction mod p of Jacobians and Jacobi sums* (2011): https://arxiv.org/abs/1111.0545
- Joux and Vitse, *Cover and Decomposition Index Calculus on Elliptic Curves of Prime Order* (2012): https://eprint.iacr.org/2011/020.pdf
