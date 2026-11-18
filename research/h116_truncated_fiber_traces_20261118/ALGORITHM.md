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
Deriving the displayed equation from the actual normalized rational isogeny
map remains the geometric bridge. [pcc:unverified]

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
