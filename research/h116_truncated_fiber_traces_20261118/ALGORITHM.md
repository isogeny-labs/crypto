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
prefix-equal zero-constant parameters. Proving that the actual elliptic
endomorphism instantiates the monomial model, and identifying the supplied
outer series with the regular factor in the elliptic x-coordinate, remain
separate steps. [pcc:unverified]

A frozen 20-case boundary panel over five primes and four cutoffs measures this
loss directly. All local-parameter pairs first differ at `m+1`, all normalized
inverse-square pairs first differ at `m`, and all boundary deltas equal
`-2 c f(0)^-3`; zero-correction and one-degree-shift controls also pass. Thus
the generic theorem's precision is tight on the panel and cannot be promoted
to one more coefficient without additional structure. [pcc:unverified]

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
