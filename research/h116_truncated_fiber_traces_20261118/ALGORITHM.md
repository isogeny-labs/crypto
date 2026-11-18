# H116 truncated Frobenius-fiber algorithm

Let `a=t-1 mod p` be the differential of `phi=V-1`. Compose `phi` with the
target coordinate scaling `(x,y) -> (a^2 x,a^3 y)`. The resulting normalized
isogeny has source coefficients `(A,B)`, target coefficients
`(a^4 A,a^6 B)`, and x-map

```text
S(x) = x + h1/x + h2/x^2 + ... .
```

The implementation uses Bostan--Morain--Salvy--Schost recurrence (16):

```text
h1 = (A-a^4 A)/5
h2 = (B-a^6 B)/7

hk = 3/((k-2)(2k+3)) sum_(i=1)^(k-2) hi h_(k-1-i)
     - (2k-3)/(2k+3) A h_(k-2)
     - 2(k-3)/(2k+3) B h_(k-3).
```

For `J=8`, only `h1,...,h7` are constructed. When `p` is prime, the frozen
`p>19` rule makes all displayed denominators units. The exact denominator set
is `5,7,9,11,13,15,17,22,39,60,85`; the Lean theorem
`bostanDenominatorsNonzero` proves the four values above `19` by factoring them
into residues below `p`. [pcc:unverified]

`BostanJetRecurrence` is the Lean statement of this exact finite recurrence.
`BostanJetRecurrence.unique` proves that any two solutions agree on
`h1,...,h7`, and `fiberDenominatorSeries_coeff_eq_of_bostanRecurrence` carries
that uniqueness into every `F_U` coefficient through degree eight. This is a
uniqueness/interface result; the geometric proof that the normalized isogeny
jet satisfies the recurrence remains open. [pcc:unverified]

The paper's second-order rational-map equation becomes the following equation
after writing `R(x)=xU(w)` with `w=1/x` and multiplying by `w²`:

```text
(3+A w²)(U-wU') + 2w²(1+A w²+B w³)U'' = 3U² + Atilde w².
```

`NormalizedIsogenySecondOrderEquation` is its distributed PowerSeries form.
Lean now proves that this equation, the `h1` and `h2` initial identities and
the nonzero denominators imply `BostanJetRecurrence`; the `ZMod p`
specialization discharges the denominator premises from prime `p>19`.
Lean also proves the preceding differential-algebra step: differentiating
`(x³+Ax+B)(R')²=R³+Atilde*R+Btilde` and cancelling a nonzero `R'`
gives equation (15). `NormalizedIsogenyFirstOrderEquation` is equation (14)
after the same at-infinity substitution, and Lean proves that it implies
`NormalizedIsogenySecondOrderEquation`; the factor cancelled in this
PowerSeries proof has constant coefficient one. Its degree-two and degree-three
coefficients also force `h1=(A-Atilde)/5` and `h2=(B-Btilde)/7`. Consequently,
for prime `p>19`, `NormalizedIsogenyFirstOrderEquation` alone implies the full
frozen recurrence. The remaining geometric bridge must construct the actual
normalized isogeny at-infinity series and prove that it satisfies this one
first-order identity. [pcc:unverified]

The remaining interface now has a finite certificate form. For reversed
numerator and denominator polynomials `N,D`, put

```text
H = D(N-wN') + wD'N.
```

`NormalizedRationalMapPolynomialCertificate` checks `D(0)=1`, `DU=N`, and

```text
(1+A w²+B w³)H²
  = D N³ + Atilde w² N D³ + Btilde w³ D⁴.
```

Lean proves that this cleared-denominator identity implies transformed
equation (14), then the complete recurrence for prime `p>19`. Producing and
independently validating `N,D` for the actual isogeny remains open; the
certificate theorem does not assert that arbitrary polynomials are an isogeny.
[pcc:unverified]

For the frozen `J=8` endpoint, the full polynomial equality is stronger than
needed. `NormalizedRationalMapJetCertificate` replaces it by

```text
D(0) = 1,
D U = N                         mod w^11,
(1+A w²+B w³)H² =
  D N³ + Atilde w² N D³ + Btilde w³ D⁴   mod w^10.
```

Lean proves this fixed certificate implies the same recurrence for every prime
`p>19`. The quotient needs one extra coefficient because it is differentiated;
the resulting first-order identity is known modulo `w^10`, and differentiating
again yields the second-order identity modulo `w^9`. The final recurrence step
reads coefficient eight, so no higher coefficient is used. Verification thus
accepts 11 coefficients of each of `N,D`, independent of `r`. Constructing the
correct prefixes and proving that they encode the intended kernel quotient are
not supplied by this theorem and remain in the candidate cost boundary.
[pcc:unverified]

The excluded `p=31`, `r=37` control now constructs an actual Vélu `N,D`,
checks this identity twice in independent Rust/Python implementations and
matches the recurrence through degree eight. The full degree-`r` construction
is retained only as a proof/control oracle; it is not part of the fixed-length
candidate cost. [pcc:unverified]

The formal-scalar constructor supplies a concrete candidate for the missing
low-jet construction. With `z=-x/y`, it computes the series defined by

```text
q = 1 + A z^4 q^2 + B z^6 q^3,
x = z^-2 q^-1,
L(z) = integral dx/(2y),
L([a](z)) = a L(z),              a=t-1.
```

It then substitutes the normalized parameter `[a](z)/a` into the curve with
coefficients `(a^4 A,a^6 B)`, divides its x-series by the source x-series, and
re-expands in `w=1/x`. At precision eleven, this path accepts only `(p,A,B,t)`;
the full numerator and denominator are reserved for the post-construction
oracle. On the excluded `p=31,r=37` fixture it returns
`[1,0,20,0,13,7,2,4,8,14,5]`, exactly the full-map quotient. [pcc:unverified]

The proposed reason the omitted Frobenius term is invisible is local: in
`V-1=[t-1]-pi`, the leading `z^p` term becomes degree `z^(p-1)` after removing
the normalized leading parameter, and the x-coordinate pole converts that to
the first possible `w^((p-1)/2)` difference. Synthetic p=23 and p=19 controls
place the first differences at coefficients 11 and 9 respectively. Lean proves
the monomial truncation at `z`-degree 22 for prime `p>19`. It now also proves
the generic one-degree-loss step: equality of `X*f` and `X*g` through one extra
degree implies equality of `f⁻¹²` and `g⁻¹²` through the requested degree when
both unit factors have nonzero constant coefficient. This discharges the pure
double-pole algebra. A follow-up substitution theorem also preserves any fixed
regular outer unit, its inverse and its product with the pole term for
prefix-equal zero-constant parameters. The curve-specific instance identifies
the outer unit by `q=1+A X^4q^2+B X^6q^3`, proves its cleared affine identity,
rescales it to target coefficients `(a^4A,a^6B)`, and proves the exact double-
substitution identity used by the constructor whenever
`a*zNormalized=zScalar`. Proving that the actual elliptic endomorphism supplies
that parameter identity remains separate. The generic formal-group filtration
is now proved against Mathlib's `FormalGroup`: `F(X,Y)-X` has a factor `Y`, so
substituting an `X^m` correction cannot change coefficients below degree `m`.
Composing this with the leading-`X` pole theorem proves the normalized
double-pole/regular-unit prefix below degree `m-1`. Associativity now also
proves the required subtraction orientation from
`F(actual,omitted)=scalar` and a certified inverse. The inverse-order premise
is derived from the omitted parameter's order in a commutative formal group.
For the short-Weierstrass parameter, a stronger endpoint directly uses the
explicit certificate `F(omitted,-omitted)=0`, eliminating the separately
constructed inverse series. An abstract possibly noncommutative-ring theorem
now derives `V+pi=[t]` from the Frobenius characteristic polynomial,
`pi*V=[p]`, left cancellation by `pi`, and centrality of `[t]`. Binding those
premises to `AddMonoid.End G` now proves scalar centrality automatically and
derives cancellation from pointwise injectivity of `pi`. A formal `ZMod 2`
mutation control satisfies both equations with noninjective zero `pi` but
rejects the sum conclusion. The positive Frobenius is now an actual Mathlib
affine-point endomorphism induced by `FiniteField.frobeniusAlgHom`; it is
additive and injective, and over `ZMod p` sends `(x,y)` exactly to
`(x^p,y^p)`. Over algebraic extensions of `ZMod p`, inverse Frobenius now
induces an inverse point map. Defining `V=[p]*piInverse` proves the norm
relation `pi*V=[p]` internally, so the characteristic equation is the only
remaining positive point-group algebraic premise for `V+pi=[t]`. Identifying
this pointwise `V` with the retained rational/scheme Verschiebung and binding
the resulting equality, elliptic negation, sign and scaling identifications to
the formal parameter are still endpoint obligations. The coordinate formulas
are tested separately by an exhaustive eight-cell extension-field panel. The
characteristic equation and pointwise `V` equality pass on all 5,066 points,
with exact agreement from an independent implementation. One frozen
missing-inverse mutation is degenerate on the 35-point `p=5`, degree-two group,
so the experiment-level protocol records seven accepted cells and one failed
control. This does not change the generic proof obligation. The coordinate
formulas themselves are formal as well:
the point-endomorphism equality is rearranged to `V-1=[t-1]-pi`, and evaluating
both sides through a total affine-point observable proves equality of the
chosen `z=-x/y` value. This is pointwise observation, not yet equality of the
retained rational map or its formal power series.
For a finite extension point group, the characteristic equation may now be
supplied by a generating-set certificate: Lean promotes zero evaluations on a
set with additive closure top to the global endomorphism equality and fixes
the trace to `p+1-Nat.card E(F_p)`. This compresses finite verification only;
it does not establish the algebraic-closure theorem.
On the frozen eight-cell panel, the corresponding certificate uses 13 points
instead of 5,066: three groups are cyclic and five need two generators. Exact
independent replay matches every coordinate, order and closure; wrong trace,
wrong norm and dropped-generator controls all reject. The measured compression
begins only after group enumeration and is not an ECDLP runtime claim.
Over an arbitrary algebraic extension, Lean now proves that every affine point
is fixed by some positive Frobenius power: its two coordinates generate one
finite intermediate field. Hence characteristic checks on every positive
fixed locus promote to the global point-endomorphism equation and the exact
`V-1` orientation. A checked `ZMod 3` mutation shows that a single fixed locus
does not suffice. This reduces the generic premise to the family of finite
fixed loci but does not prove the characteristic identity on that family.
For `z=-x/y`, Lean proves negation `z -> -z`, naturality under field maps, and
the exact `z -> z^p` formula for a supplied coordinatewise `p`-power
endomorphism. Connecting those formulas to the actual elliptic formal-group
object and retained map remains the endpoint. [pcc:unverified]

A frozen 20-case boundary panel over five primes and four cutoffs measures this
loss directly. All local-parameter pairs first differ at `m+1`, all normalized
inverse-square pairs first differ at `m`, and all boundary deltas equal
`-2 c f(0)^-3`; zero-correction and one-degree-shift controls also pass. Thus
the generic theorem's precision is tight on the panel and cannot be promoted
to one more coefficient without additional structure. [pcc:unverified]

A second frozen 20-case panel composes the perturbed parameters with a
nonconstant regular outer unit. Substitution and inverse substitution first
differ at `m+1`; the complete normalized pole coordinate first differs at `m`.
All 60 independently predicted boundary coefficients and all 20 control
triplets pass. Thus the generic regular factor does not consume an additional
coefficient on this panel. [pcc:unverified]

The curve-coordinate panel then replaces the synthetic outer unit with the
actual fixed-point series. At precision 24 on five frozen curves, independent
fixed-point and coefficient-recurrence implementations agree on

```text
q_target(zNormalized) = q_source(a*zNormalized)
q_source * (zNormalized/z)^-2 * q_target(zNormalized)^-1
  = q_source * (zNormalized/z)^-2 * q_source(a*zNormalized)^-1.
```

Both wrong `(a^2A,a^4B)` target scaling and wrong `(a+1)zNormalized` parameter
are rejected at coefficient four in all five cells. This measures the exact
coordinate expression consumed by the Lean theorem; the input equality for
the actual `V-1` local parameters remains the next formal-group obligation.
[pcc:unverified]

The first beyond-prefix control directly measures that omitted term on the
excluded p=31 map. The normalized `V-1` and independently constructed scalar
`[-6]` prefixes agree through index 14, first differ at the preregistered index
15, and have the preregistered delta ten modulo 31. Both cleared map identities
pass, while omitted scaling, wrong target and a coefficient-15 mutation are
rejected. The producer required 96,757 additions, 96,881 multiplications and
four inversions at precision 18. This single fixed cell checks the predicted
boundary and not the generic endpoint or any scaling claim. [pcc:unverified]

The current dense implementation uses 64,426 additions, 67,584
multiplications and 16 inversions for candidate construction at this fixed
precision. These counts exclude internal verification, oracle and controls.
They are construction measurements, not an end-to-end comparison or an
asymptotic bound. [pcc:unverified]

The unchanged constructor was subsequently compared with four additional full
maps on deterministic different-j models at degrees 197, 257, 397 and 547. It
matched all eleven coefficients in every constructed cell, while wrong-scalar
and unscaled-target controls disagreed. A fifth frozen p=107 cell had no
qualifying alternative j-invariant and was retained without replacement. The
candidate construction count remained exactly 64,426 additions, 67,584
multiplications and 16 inversions per completed cell because its precision and
dense control flow are fixed. [pcc:unverified]

For target `Q=(u,v)` and fixed reference `Q0=(u0,v0)`, set

```text
U  = a^2 u
U0 = a^2 u0
FU(w) = 1 - U w + h1 w^2 + ... + h7 w^8.
```

Truncate `FU^-1` and `FU'/FU` through degree seven. With array index `k=0..7`,
the candidate returns

```text
X[k] = coeff_k(FU0'/FU0) - coeff_k(FU'/FU)
Y[k] = a^3 v * coeff_k(FU^-1).
```

These are respectively
`sum_fiber x^(k+1)-sum_reference_fiber x^(k+1)` and
`sum_fiber y*x^k`, assuming the normalized-isogeny specialization. Candidate
online work is a fixed number of base-field operations and receives only point
coordinates, never the scalar label.

The direct control constructs `F_(p^d)`, projects a point onto full `r`-torsion,
forms a generator of `ker(V-1)` as `(pi-1)T`, obtains a rational section using
the inverse of `p-1 mod r`, and walks all `r` fiber points. It independently
sums the same moments. A λ=2 coordinate model is evaluated by mapping every
direct fiber point and separately rebuilding the candidate jet from the scaled
curve coefficients.

This is a component algorithm, not an ECDLP solver. A subexponential ECDLP
claim still requires an exact scalar predicate, a complete decoder, recovery,
verification, and every setup cost. [pcc:unverified]
