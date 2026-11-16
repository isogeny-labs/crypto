# Trace +22 degree-eight quadratic-square cover screen

16 November 2026. The frozen all-carrier search completed all 24 cells with **zero exact cover identities** in the denominator shape `B=L H²`, where `L` is the carrier's unique rational branch factor and `H` is a monic, squarefree quadratic coprime to the branch polynomial. This is a finite, shape-specific exclusion [pcc:unverified], not a degree-eight classification and not an attack-cost result.

## Equation and exhaustive search within the stated shape

Let `K=F3[a]/(a^5+2a+1)`, `Y_f: y²=f(x)` and `E_b: v²=u³+u²+b`. Each of the twelve listed carrier polynomials is `f=x^11+Σ_i digit_3(fmask,i)x^i` and factors over `K` as `L Q_1...Q_5`, with `L` monic linear and the `Q_i` distinct irreducible quadratics. The source masks are `bmask=68,84`, representing the two Frobenius orbits of trace +22 source models. The exact masks and field model are in [AMENDMENT_ALL_CARRIERS.md](AMENDMENT_ALL_CARRIERS.md).

Any map of the searched form has `u=A/B` and `v=C y/B²`, with `deg A=8`, `B=L H²`, and `gcd(A,B)=1`. The defining-equation test is exactly

```text
f C² = B (A³ + A² B + b B³).
```

At each `Q_i`, `B` is invertible and `A/B` must be one of the three roots of `T³+T²+b` in `K[x]/(Q_i)`. The code verifies that these roots are distinct and present in every cell. It enumerates all `3^5=243` compatible assignments. CRT then fixes `A` modulo `M=Q_1...Q_5`, of degree ten. A degree-eight numerator forces its `x^9` coefficient to vanish. For each assignment, the search specializes all 243 values of the linear coefficient of `H`, solves the resulting polynomial for every constant coefficient, and enumerates all 243 values if that polynomial is zero. It checks every retained `(A,B)` against the full identity. The point evaluations preceding the square test are necessary conditions only and cannot remove a true identity.

Thus a zero exact-hit count excludes *this rational-function denominator shape* for the fixed carrier/source models, subject to the Sage implementation and receipt audit. It does not exclude other degree-eight denominator divisors, higher-degree covers, correspondences, or different carrier models. A degree-eight map outside this shape is the next constructive question.

## Run and custody

The [manifest](manifest.json) records 24 complete cells, source and runner hashes, and 333.357 seconds of runner wall time. The [audit](AUDIT.json) checks every cell's model identifiers, source hash, completion, counters, and empty stderr. It is a structural receipt audit, not an independent algebraic implementation. The cumulative cell CPU is 1,535.568 seconds; it measures the search, not a GHS attack.

| Aggregate counter | Value |
|---|---:|
| Branch assignments | 5,832 |
| Coefficient specializations | 1,417,176 |
| Parameter roots, including zero-polynomial fallback | 2,883,285 |
| Squarefree, branch-coprime quadratics | 2,858,748 |
| Degree-eight, coprime `(A,B)` | 1,437,384 |
| Passed necessary point test and `f` divisibility | 2,014 |
| Passed the exact polynomial square test | **0** |

The twelve carriers are defined over `F3`, so conjugating a map by `a↦a³` preserves each carrier and the searched denominator shape. Since masks 68 and 84 represent both five-element source orbits, the exclusion extends to 120 exact carrier/source equation pairs in this shape. The conjugation argument uses the checked source-orbit arithmetic in [Trace22Orbit.lean](../ghs_trace22_carrier_variants_20261116/Trace22Orbit.lean); the finite elimination itself is checked by the Sage run and structural audit, not by Lean.

The prior-work screen in [PROTOCOL.md](PROTOCOL.md) compares the construction to [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf) and [Howe 2022](https://arxiv.org/pdf/2110.04221). It does not establish publication priority. The trace +22 class remains an open cover-construction lead; the zero-hit screen supplies a rigorous *scope* for what has been ruled out, not a new vulnerable family.
