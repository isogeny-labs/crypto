# H116 universal multiplication-resultant bridge

## Purpose

This checkpoint separates a known arbitrary-scalar cancellation theorem from
the genuinely open H116 ECDLP predicate. It does not propose a new ECDLP
algorithm.

## Source theorem

Schmidt's 2015 theorem gives, for the standard multiplication-map polynomials
of `y^2=x^3+A*x+B`,

```text
Res(A_n,B_n) = (-16*(4*A^3+27*B^2))^(n^2*(n^2-1)/6).
```

Primary source: Harry Schmidt, *Resultants and discriminants of multiplication
polynomials for elliptic curves*, Journal of Number Theory 149 (2015), 70--91,
[DOI 10.1016/j.jnt.2014.10.007](https://doi.org/10.1016/j.jnt.2014.10.007).

## Lean interface

The tracked Lean bridge accepts the exact formula for Mathlib's
`WeierstrassCurve.Phi n` and `PsiSq n` as a premise. It then proves:

1. the resultant is a unit whenever `4*A^3+27*B^2` is nonzero;
2. `Phi n` and `PsiSq n` are coprime;
3. when the scalar is nonzero in the coefficient field, the reduced rational
   x-coordinate generates an extension of exact degree `n^2`.

The premise is deliberately explicit. Proving that Schmidt's `A_n,B_n` are
definitionally or propositionally equal to the pinned Mathlib polynomials is
the remaining convention-alignment obligation. Fixed-scalar Lean theorems
through ten provide controls for that alignment but do not replace it.

## ECDLP boundary

The resultant theorem removes numerator/denominator cancellation from the
H116 degree chain. It supplies neither the `V-1` scheme morphism nor a scalar
digit predicate, decoder, or subexponential runtime. Shipsey--Swart ePrint
2008/444 is the checked EDS-based ECDLP predecessor for `#E(F_q)=q-1`; H116's
generic-prime-field goal remains outside that special family. [pcc:unverified]
