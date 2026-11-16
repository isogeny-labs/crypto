# Common 2+2 affine coordinate-ring bridge

Status: partial formalization. The projective genus-2 law is **not** proved
from the source implementation to `Pic⁰` by this packet.

For any field `K` and polynomial `f : K[X]`,
`Common22CoordinateRing.Coordinate f` is the quotient
`K[X][Y] / (Y² - f(X))`. Its distinguished elements `ordinate f` and
`abscissa f g` satisfy `ordinate f ^ 2 = abscissa f f` in Lean. This is a
concrete affine coordinate ring, rather than the abstract `Div`/`Class`
interface in `RuntimeSemantics.lean`.

The theorem `Common22CoordinateRing.norm_factorization` takes the exact
polynomial identity `n² - u₁u₂·s = b²f` and proves in that ring

`(n(X) - bY)(n(X) + bY) = u₁(X)u₂(X)s(X)`.

`Common22SourceHead.source_curve_eq` supplies the stated polynomial identity
for the source-style common 2+2 coefficients under its explicit valid-input,
nonzero-determinant and normalized-quintic hypotheses.
`Common22CoordinateRing.source_style_norm_factorization` applies that result
to the same source-style coefficients, with those hypotheses exposed in its
signature. This closes the proof-side polynomial-to-affine-coordinate-ring
link, not the Rust execution or divisor-class links.

| Claim | Lean declaration | Exact scope | Checked status |
| --- | --- | --- | --- |
| `Y²=f(X)` | `Common22CoordinateRing.ordinate_sq` | Any field and polynomial `f` in the quotient coordinate ring | proved |
| Norm product | `Common22CoordinateRing.norm_factorization` | Conditional on `n²-u₁u₂s=b²f` | proved |
| Source-style norm product | `Common22CoordinateRing.source_style_norm_factorization` | Valid Mumford congruences, nonzero determinant, monic quintic with zero fourth coefficient | proved |
| Principal divisor and class sum | none | Smooth projective curve, zero/pole multiplicities, output reduction | open |

Build: `lake env lean -o .lake/build/lib/lean/Common22SourceHead.olean
Common22SourceHead.lean` followed by `lake env lean Common22CoordinateRing.lean`.
The latter output is preserved in `coordinate-ring-build.log`; all three
`#print axioms` checks show only `propext`, `Classical.choice`, and
`Quot.sound`.

The remaining geometric obligation has four separate parts:

1. Construct the smooth projective completion of `Y²=f(X)` over the intended
   odd-characteristic field, with `f` squarefree and monic of degree five.
   Establish that the affine coordinate ring embeds in its function field.
2. Prove the zero divisor of `n(X)/b - Y` includes the two input Mumford
   effective divisors, with scheme-theoretic multiplicity even when support
   roots repeat or touch branch points. When the source coefficient `c≠0`,
   identify the other degree-two zero divisor with the *specific* output
   Mumford pair produced by reduction. When `c=0`, the support has degree at
   most one and the output is the degree-one branch.
   The norm identity alone does not allocate zeros between conjugate factors.
3. Prove the pole divisor at the unique point at infinity is `6·∞` when
   `c≠0`, and `5·∞` when `c=0`, for the smooth monic-quintic model. The
   source-style interpolant has leading coefficient `-c`, so a nonzero
   determinant `b` alone does not imply pole order six. The later Lean
   `cubic_degree_le_two_of_c_zero` and
   `support_degree_le_one_of_c_zero` prove the degree drops, but neither
   formalizes this infinity valuation. For example, the source coefficients
   `q1=q2=0`, `r1=1`, `r2=2`, `s1=s2=0`, `t1=t2=1` give `b=-1≠0` and
   `c=0`; this is an algebraic gate witness, without a claimed smooth-curve
   instance.
4. Use the resulting principal divisor equation to identify the output class
   as the sum of input classes in `Pic⁰`, including all exceptional branches.

No theorem here assumes any of these four conclusions. No performance or
cohort ratio changes follow from this algebraic result.
