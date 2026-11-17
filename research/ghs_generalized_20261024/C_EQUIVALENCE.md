# Canonical parameter equivalence for the surviving quadratic profiles

The four complete screens at absolute degrees 6, 8, 10, and 12 contain 232 genus-two profiles over the index-two subfield. This note explains why those 232 Hess parameters do not describe 232 independent attacks. It uses the characteristic-two Artin--Schreier presentation documented for Magma's `WeilDescent(E,k,c)` ([Magma handbook](https://magma.maths.usyd.edu.au/magma/handbook/text/1561), lines 43--47). The full independent enumeration and point checks are in `verify_c_equivalence.sage` and `results/c-equivalence-audit.json`.

Let `K=GF(2^(2s))`, `k=GF(2^s)`, `E: Y^2+XY=X^3+aX^2+b`, and `beta=sqrt(b)`. For a nonzero Hess parameter `c`, put `d=beta/c`, `z=c/X`, and `v=(Y+beta)/X`. Then, directly from the elliptic-curve equation,

```text
v^2+v = c/z + a + d*z,       c*d=beta.
```

The exact four source curves have `beta` outside `k`. If `d` is in `k`, the `k`-rational change `w=d*z` gives `c/z+d*z=beta/w+w`. If `c` is in `k`, the `k`-rational change `w=c/z` gives the same canonical expression. Thus both branches have the same canonical Artin--Schreier equation `v^2+v=beta/w+a+w`, with canonical parameter `c=beta,d=1`.

Since `beta` is outside `k`, `c` and `d` cannot both lie in `k`. Each branch has exactly `2^s-1` choices. For a quadratic extension, the Frobenius-order genus formula gives genus two when exactly one of `c,d` is in `k`, and genus three when neither is. Consequently the genus-two counts are exactly `2(2^s-1)`: 14, 30, 62, and 126. The Sage audit independently enumerates every nonzero `c` in each exact polynomial-basis field, compares these counts with the saved screen histograms, and verifies the rational-function identity for every `c` and every nonzero base-field `z`. It also checks that the canonical `beta` and degree-two cover match each executed direct receipt; the four canonical values are `0x24`, `0xc5`, `0x42`, and `0x208`.

The two normalized presentations of a source point have `w=beta/X` when `d` is in `k`, and `w=X` when `c` is in `k`. Let `T=(0,beta)`, the rational point of order two on `E`. For every point with `X != 0`, the curve law gives

```text
X(T-P) = beta/X(P),
(Y(T-P)+beta)/X(T-P) = (Y(P)+beta)/X(P).
```

So the `c in k` presentation of `P` is the canonical presentation of `T-P`. On degree-zero divisor classes, `(T-P)-(T-O)` represents `-P`; hence its additive Jacobian transfer differs from the canonical one by sign. The checked Lean theorems `normalize_d_branch` and `normalize_c_branch` prove the two rational identities over any field, while `translated_negation` proves the additive-map sign implication abstractly. Sage checks the displayed point identities on all nonzero elements of each selected prime-order subgroup: 18, 66, 42, and 198 points respectively. The previous exact attack receipts check the canonical composite transfer on every subgroup scalar, including nonzero image and target-log preservation.

This is a parameter-equivalence result for the four tested source curves. It establishes that the 232 surviving *profiles* reduce to four canonical presentations, each already represented by a direct end-to-end run. It does not construct 232 separate Jacobian objects or time each parameter presentation. The concrete trace-conorm and isogeny coordinate formulas remain executable finite checks rather than Lean geometric theorems. The 21 other low-genus degree-12 profiles over smaller subfields are separately excluded for the order-199 source subgroup by `results/subgroup-weil-audit.json`.
