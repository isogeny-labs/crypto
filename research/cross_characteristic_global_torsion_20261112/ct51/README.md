# CT51: a degree-eight prime-to-binary torsion bridge

**Status:** independently implemented *local arithmetic checks* only; [pcc:unverified]. This is a classical division-field specialization, **not** a scalable ECDLP reduction. This stacked branch starts from cross-characteristic PR #4012 and leaves concurrent native CT branches unchanged.

## Explicit global and two local models

Let the rational curve be Cremona 11a1,
`E: y^2+y=x^3-x^2-10x-20`.
Its third division polynomial is
`f(X)=3X^4-4X^3-60X^2-237X-21`.
Set
`K = Q[X,Y]/(f(X), Y^2+Y-X^3+X^2+10X+20)`.
The mod-two reduction of `f` is `X^4+X+1`, irreducible over `F_2`: neither element of `F_2` is a root and its reduction modulo the only irreducible quadratic `X^2+X+1` is 1. In `F_16=F_2[X]/(X^4+X+1)`, `R=X^3+X^2` has absolute trace one. Hence `Y^2+Y=R` is irreducible over `F_16`, and **[K:Q]=8** (irreducibility lifts from reduction mod 2 at the monic-in-Y stage). The 2-adic local residue field is `F_256`; the reduction of the global 3-torsion point is `(X,Y)`.

At the other prime, the simultaneous congruences `X=18,Y=5 (mod 29)` solve both equations; the specialization is the point `P_29=(18,5)`. Thus the same algebraic point specializes to **two genuinely different residue characteristics**:
`< (18,5) > in E(F_29)[3] <- (X,Y) in E(K)[3] -> < (X,Y) > in E(F_(2^8))[3]`.
Both points have exact order 3, and `O,(18,5),(18,23)` map respectively to `O,(X,Y),(X,Y+1)`. This is the full subgroup isomorphism for the two specified generators. The source group has order 30 and the target has order 225; the latter contains all eight nonzero geometric 3-torsion points.

**Sharper negative result for this particular global curve:** every nonzero geometric 3-torsion point has an `X` coordinate of degree 4 over `F_2`, and its `Y` coordinate has degree 8. Therefore no nonzero 3-torsion point on this particular `E` is defined over `F_{2^m}` for `m<8`. An arbitrary global point of order 3 has degree exactly 8 over Q: the transitivity follows from the irreducibility of the degree-eight point equations, not a conjectural Galois-group assertion.

## Reproduce without Sage or paid compute

```sh
python3 -B research/cross_characteristic_global_torsion_20261112/ct51/degree_eight.py
python3 -O -B research/cross_characteristic_global_torsion_20261112/ct51/degree_eight.py
node research/cross_characteristic_global_torsion_20261112/ct51/independent.js
node --no-opt research/cross_characteristic_global_torsion_20261112/ct51/independent.js
```

Python constructs `F_16[Y]/(Y^2+Y+X^3+X^2)`; JavaScript separately constructs `F_2[U]/(U^8+U^6+U^5+U^4+U^3+U+1)`, with `X=1+U+U^4`, `Y=U`. Both independently check the full 3-torsion, 9 group-law pairs, 3 public DLP answers, complete source/target orders, and wrong-generator-orientation controls. A swapped sign is *also* a group homomorphism, so the negative control must fix the distinguished public generator; it is incorrect to claim that every sign swap breaks the group law. These are producer-run controls pending signed PCC verification.

## Complexity gate and mathematical scope

This test moves beyond rational 5-torsion by requiring a **degree-eight global number field** for order 3. It still constructs an inverse specialization only for three prelisted points. This is an O(N) subgroup enumeration in the obvious generalization, not a uniform polynomial-time algorithm on arbitrary encoded inputs. Nor does `[K:Q]=8` establish an asymptotic lower bound for engineered genus-g abelian varieties. If a fixed elliptic curve over Q has **surjective mod-N Galois representation** for prime N, the stabilizer index of a nonzero vector in `GL_2(F_N)` gives the conditional exact point field degree `N^2-1`; surjectivity is a hypothesis, **not** claimed here for N=3 or all curves. The exact degree eight in this example is independently established instead.

**Source check and overlap:** root PR #3877, cross-characteristic PR #4012 (rational 5-torsion and local/global degree floor), `problems/lift-reduce-second-prime-transfer.md`, CT47–50 same-characteristic torsion/gluing work. Related primary references: Jacobson–Koblitz–Silverman–Stein–Teske, *Analysis of the Xedni Calculus Attack* (2000); Lozano-Robledo, *Division fields of elliptic curves with minimal ramification* (2015), https://doi.org/10.4171/RMI/870; Masser–Wuestholz, *Galois Properties of Division Fields of Elliptic Curves* (1993), https://doi.org/10.1112/blms/25.3.247. The shared KB MCP was not exposed (checked available tools); fallback GitHub search was performed across all three repos. No novelty or security claim is made.

**Next gate:** stop enumerating N torsion points; find a coordinate-level algorithm computing compatible global lift images of *arbitrary* source subgroup points at cost o(sqrt(N)), including field-degree/height construction. The higher-genus alternative remains open. No paid compute, rho speedup, or full solver claim.
