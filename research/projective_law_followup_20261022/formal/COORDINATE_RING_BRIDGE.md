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
   roots repeat or touch branch points. Identify the other degree-two zero
   divisor with the *specific* output Mumford pair produced by reduction.
   The norm identity alone does not allocate zeros between conjugate factors.
3. Prove its pole divisor is `6·∞`. The cubic interpolant has pole order six
   at the unique point at infinity; this valuation is not represented by the
   affine quotient theorem.
4. Use the resulting principal divisor equation to identify the output class
   as the sum of input classes in `Pic⁰`, including all exceptional branches.

No theorem here assumes any of these four conclusions. No performance or
cohort ratio changes follow from this algebraic result.
