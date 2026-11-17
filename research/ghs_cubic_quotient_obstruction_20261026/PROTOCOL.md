# Degree-two quotient audit protocol

26 October 2026, Conductor T-5393. Exploratory calculations identified
the candidate obstruction before this protocol was written. The audit
below is a deterministic replay of the stated claim.

## Inputs and checks

Use all four exact genus-three models in the
[LMFDB curve snapshot](lmfdb_curves_3974.txt), downloaded from the
[class page](https://www.lmfdb.org/Variety/Abelian/Fq/download_curves/3.16.a_a_aet)
on 26 October 2026. Require its first line to match the model in the
[catalog closure certificate](../ghs_cubic_catalog_q16_20261026/certificate.json):
`y^2+h(x)y=f(x)` over `GF(16)`, where

```text
h = a^2*x^4 + a*x,
f = x^8 + x^3 + a*x^2 + (a^2+a+1)*x + a^3+a+1,
a^4+a+1 = 0.
```

1. For each of the four models, verify genus 3 and point counts
   `(17,257,3728)` over `k`, `k^2`, and `k^3`. Verify all four branch points
   over the Conway `GF(4096)` extension. For each root `r` of `h`, check the coefficient of the
   simple pole remaining after Artin--Schreier reduction of `f/h^2` is
   nonzero.
2. For each model, enumerate all 24 permutations of the four branch points. Solve for
   the unique projective linear transformation carrying the first three
   points to their images and check the fourth. Require exactly 12 valid
   maps and require every one to be affine `x -> c*x+d` with `c^3=1`
   and `h(d)=0`.
3. For each model and each of the 11 nonidentity affine maps, compute
   `f(c*x+d)+c^2*f(x)`. It must be nonzero of degree at most 3.
   Verify the theoretical leading term: degree 3 with coefficient
   `gamma*(1+c^2)` when `c!=1`, or degree 2 with coefficient `gamma*d` when
   `c=1,d!=0`.
4. Independently prove in the report that a lift would require a
   rational `u` with `u^2+c*h*u=f(c*x+d)+c^2*f`. The absence of finite
   poles makes `u` polynomial; degree comparison rules out nonzero
   right sides of degree at most 3. State the family theorem and its
   scope separately from finite field replay.

This tests separable degree-two quotients of the four *listed curves*.
It says nothing about higher-degree correspondences or isogenies
between their Jacobians and a Weil restriction, or about other
polarizations in the isogeny class.

## Prior work

[Nart--Ritzenthaler](https://arxiv.org/pdf/0905.0546) classify
characteristic-two genus-three curves with a Klein-four group of
involutions and give their elliptic quotients. Their Proposition 2.1
provides a comparison class; the selected catalog curve is checked
against the involution condition rather than presumed to be in it.
[Gaudry--Hess--Smart](https://www.kantmath.de/publications/papers/weil_descent.pdf)
construct explicit maps for their direct-cover family. The obstruction
here excludes that degree-two quotient tactic for this particular
catalog witness, without excluding their other construction or the
Tate isogeny.
[Borodin--May](https://arxiv.org/pdf/2508.16885) survey known and
conjectured genus-three hyperelliptic Jacobian obstructions over small
finite fields. Their characteristic-two parity screen lists three
candidate exclusion residues, with proofs cited for the first two; our
polynomial has coefficient pattern
`(s,t,u)=(0,0,-123)=(0,0,1) mod 2`, so that obstruction does not apply.
This is a non-exclusion, not an existence theorem or map construction.
