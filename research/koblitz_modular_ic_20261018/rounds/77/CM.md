# R77: quadratic infinity orders correct the b divisor

R77 checks the infinity substitution directly against the quadratic
source equation. With `t=1/z` and `U=t^2 V`, the equation is

```text
U^2 + U = t + t^3.
```

Its derivative in `U` is one. The `t=0` fiber has the two distinct
rational roots `U=0,1`, so these are two unramified quadratic infinity
places. Put `B=1+(1+t)U`; then the Tate functions clear as

```text
R = t^-3 B,
b = t^-7 B(B+t^3),
c = t^-4 (B+t^3).
```

The accepted frozen Singular program checks in the full quadratic
quotient that `Norm(B)=t^5` and
`Norm(B+t^3)=t^3(1+t)^3`, and rejects the wrong `t^4` norm.
At `U=0`, both `B` and `B+t^3` are units. Their norms then force
orders 5 and 3, respectively, at `U=1`. The leading coefficients
there are one because the other conjugate has residue one and both
norms have leading coefficient one.

| Quadratic infinity place | `v(B)` | `v(B+t^3)` | `v(R)` | `v(b)` | `v(c)` |
|---|---:|---:|---:|---:|---:|
| `U=0` | 0 | 0 | -3 | -7 | -4 |
| `U=1` | 5 | 3 | 2 | 1 | -1 |

The full source field has degree 16 over this quadratic subfield.
For each quadratic infinity place, the sum of `e*f` over all source
places above it is 16. Pulling back a quadratic valuation multiplies
it by `e`; hence its total degree contribution multiplies by 16.
The proper `b` pole divisor has degree `16*7=112`; `b` has an
infinity zero divisor of degree 16 in addition to its R76 finite
zero degree 96. The proper `c` pole divisor has degree
`16*4+16*1=80`, with no infinity zeros. These totals reconcile
the full field norms from R76 and correct its initial `b` pole-degree
claim. The additive R76 `ERRATUM.md` records that correction while
leaving the sealed R76 source and raw custody unchanged.

One native attempt passed exit 0, exact marker and zero-error gates;
one pinned Lean attempt compiled and audited all three authored
polynomial/degree theorems with zero failing and zero trusted.
Eleven historical source-byte pins and five donor hashes match.
The Lean theorems prove the characteristic-two polynomial identities
and weighted arithmetic. The DVR interpretation uses the displayed
etale split and standard valuation/norm and tower equalities.

This determines the quadratic infinity places and aggregate degree-32
divisor contributions. It does not yet split the individual degree-32
infinity branches or give their `lambda,nu` and denominator orders.
