# H116 paired-degree certificate proof

Lean now isolates the exact geometric bridge from the already-proved ring
factorization. [pcc:unverified]

```text
(V-1)(pi-1) = [p+1-t].
```

`degree_eq_norm_of_paired_factor` proves, for an abstract multiplicative degree,
that `left*right=scalar`, `deg(scalar)=N^2`, and
`deg(left)=deg(right)` imply `deg(left)=N`.
`verschiebung_sub_one_degree_of_dual_certificate` composes this lemma with the
existing norm factor and concludes `deg(V-1)=N`.
[pcc:unverified]

For an actual elliptic curve the remaining inputs have precise meanings:

| Certificate premise | Required elliptic theorem | State in pinned Mathlib |
|---|---|---|
| `degree (f*g)=degree f*degree g` | multiplicativity of curve-map/function-field degree | proved for an already-supplied finite field tower; constructing the pullback tower from the elliptic maps remains open |
| `degree [N]=N^2` | degree of scalar multiplication | exact displayed degrees `deg Phi_N=N^2`, `deg Psi_N^2=N^2-1` and the generic coprime-quotient extension-degree theorem are proved; coprimality and coordinate extension degrees four and nine are now proved for `N=2,3` on nonsingular short curves under the stated characteristic exclusions, while generic `N` and map identification remain open |
| `degree(V-1)=degree(pi-1)` | `(V-1)^vee=pi-1` and equal degree of duals | unavailable |

The pinned Mathlib audit found affine/projective elliptic points, generic finite
field-tower degree multiplication, exact reduced rational-function extension
degree, and exact division-polynomial coordinate degrees, but no elliptic-
isogeny type or dual-isogeny API. H116 now exposes the tower law, quadratic
sandwich and rational-coordinate extension theorem directly, while keeping the
concrete pullback tower, generic division-polynomial coprimality, scalar-map
identification and duality as explicit caller obligations.  The scalar-two
and scalar-three short-Weierstrass instances are now discharged separately by
explicit Bezout certificates.
`degree_product_without_duality_control` checks the logical
boundary: at `N=6`, unequal degrees 4 and 9 still multiply to `N^2=36`, so the
dual/equal-degree premise cannot be dropped.

The current Lean source SHA-256 is
`50496085a3c01d8cf7afd8e921611f0e3e8e8882fdb7e4b97826ba634e3401a2`.
The pinned build succeeds, and the repository auditor reports 150 theorems,
zero failures and zero trusted native decisions.  This proves the implication
from the geometric degree certificates; it does not construct those elliptic
certificates and is not a subexponential ECDLP proof. [pcc:unverified]

The eighteen degree-layer theorem declarations are:

- `finiteFieldTower_degree_mul`, Mathlib's finite-rank tower law in the
  orientation used by curve-map degrees;
- `finiteFieldTower_degree_eq_norm_of_paired`, which derives either adjacent
  degree from equal adjacent degrees and total degree `N^2`;
- `quadraticFunctionFieldSandwich_degree`, which compares two compatible
  towers with quadratic source and target coordinate projections;
- `rationalFunctionCoordinateExtension_degree`, which exposes Mathlib's exact
  `[K(X):K(f)] = max(deg num(f),deg denom(f))` theorem for canonical reduced
  rational functions;
- `reducedRationalQuotient_degrees`, which proves that a displayed coprime
  quotient retains both polynomial degrees after canonical normalization;
- `reducedRationalQuotient_extension_degree`, which composes the preceding
  result with the exact rational-function extension-degree theorem;
- `shortWeierstrassScalarTwo_explicit_isCoprime`, which gives an explicit
  Bezout witness with constant `4*A^3+27*B^2`;
- `shortWeierstrassScalarTwo_isCoprime`, which specializes Mathlib's exact
  `Phi 2` and `PsiSq 2` definitions to nonsingular short curves outside
  characteristic two;
- `shortWeierstrassScalarTwo_extension_degree`, which proves the reduced
  coordinate quotient has rational-function extension degree exactly four;
- `shortWeierstrassScalarThree_explicit_isCoprime`, which checks a
  degree-eight Bezout witness with constant
  `64*(4*A^3+27*B^2)^2`;
- `shortWeierstrassScalarThree_isCoprime`, which specializes Mathlib's exact
  scalar-three formulas and transfers coprimality to the squared denominator;
- `shortWeierstrassScalarThree_extension_degree`, which proves the reduced
  coordinate quotient has rational-function extension degree exactly nine;
- `shortWeierstrassScalarThree_singular_control`, which proves the `A=B=0`
  pair over `ZMod 5` is not coprime by evaluation at zero;
- `unequalCoordinateLineDegrees_control`, which proves that unequal projection
  degrees make the adjacent map degree nonidentifiable from a shared total;
- `scalarMultiplicationXMap_exact_degrees`, which exposes the exact
  `(N^2,N^2-1)` degrees of `Phi_N/Psi_N^2` when `N` is nonzero in the
  coefficient field;
- `scalarMultiplicationXMap_leadingCoefficients`, which proves leading
  coefficients `(1,N^2)`;
- `scalarMultiplicationXMap_denominator_degree_lt_of_char_dvd`, which proves
  a strict denominator-degree collapse whenever `N` vanishes in the field
  and `|N|>1`;
- `scalarMultiplicationXMap_characteristic_control`, which proves that this
  nonvanishing premise fails for `[3]` over `ZMod 3`.
[pcc:unverified]

The frozen scalar division-polynomial experiment independently instantiates
the coordinate layer on eight extension-field cells. Exact degrees
`(49,48),(25,24),(196,195)` agree with all 5,058 affine point actions, while
the characteristic controls collapse from expected denominator degrees
`24,48,120` to `20,42,110`. This is evidence for the displayed scalar
coordinate formulas, not a proof of generic-scalar coprimality or
identification with the remaining curve-map tower.  Scalar-two and
scalar-three coprimality are now proved independently of this panel.
[pcc:unverified]
