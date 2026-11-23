# H116 regular-unit substitution protocol

Protocol: `H116-REGULAR-SUBSTITUTION-001`. Frozen before execution.
[pcc:unverified]

For each of five primes and four cutoffs, perturb the normalized parameter unit
`f` by `c X^m`, form local parameters `X f` and `X(f+cX^m)`, and substitute
them into the fixed regular unit `q`, whose constants are `q(0)=3` and
`q_1=4`. Measure the first differences of `q(Xf)`, its inverse, and the full
normalized pole coordinate

```text
f^-2 * q(Xf)^-1.
```

The frozen closed-form predictions are recorded in the JSON protocol. All 20
cases must pass. Zero correction, constant outer series, and zero-constant
outer rejection are controls. No case may be replaced. [pcc:unverified]

This is a finite-series control for the generic Lean composition theorem. It
does not identify `q` with an elliptic curve's coordinate series or prove the
endomorphism relation. [pcc:unverified]
