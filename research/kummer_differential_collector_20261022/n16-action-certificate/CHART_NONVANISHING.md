# Exact nonvanishing of the cancelled chart; action gate still open

This note uses the 21 frozen odd-prime Rosenhain models, their recorded theta
constants, and the matrices in `results/summary.json`. It is an algebraic
argument over each recorded field, not an extrapolation from the 27,470
finite action comparisons. It does **not** certify that a recorded torsion
matrix implements translation on every Jacobian class.

## Degree-two chart

Write `u=x²+a1*x+a0`, `v=b1*x+b0`, and `f mod u=R0+R1*x`, with `f(0)=0`.
The constant Mumford equation is

```text
b0² - a0*b1² - R0 = 0.                                      (1)
```

The remainder recurrence in `REPORT.md` proves that `H0=R0/a0` is a
polynomial in `a0,a1` even at `a0=0`. Set `H=b1²+H0`. For the four projection
coordinates, define the following fixed arrays from the recorded Rosenhain
constants `lambda,mu,nu`:

```text
c = (mu, nu*lambda, nu, mu*lambda)
d = (lambda+nu, 1+mu, lambda+mu, 1+nu).
```

Let `s=(a,b,c_theta,d_theta)` be the four recorded nonzero theta scaling
constants, and let `C` be the recorded `general_conversion` matrix, with
row `i` equal to

```text
C[i,*] = s[i] * (c[i]*d[i], -c[i], d[i], -1).
```

This is exactly the construction in `src/main.rs::conversion_matrix`; the
distinct name `c_theta` avoids confusing a theta scale with the array `c`.
The direct cancelled chart has coordinate

```text
K_i = s[i] * ((c[i]-a0)*(d[i]+a1)-H).
```

Expanding gives the **polynomial identity**

```text
K = C * (1, -a1, -a0, a0*a1+H)^T.                           (2)
```

No Mumford equation or division is required for (2). The first entry of the
right-hand input is 1. The exact finite-field determinant checks in
`results/summary.json` show `det(C) != 0` in each of the 21 fields.
Consequently `K != 0` for **every** `a0,a1,b1` in each such field, including
the source-zero locus `a0=b0=0`. In particular, the direct chart cannot fail
by producing the zero vector on a valid degree-two Mumford divisor. This
nonvanishing conclusion uses exact full-space matrix invertibility; its
geometric interpretation as a Kummer map still depends on the projection
derivation.

Equation (1) gives `a0*T_i-b0² = a0*(T_i-H)` in the Mumford quotient ring,
where `T_i=(c[i]-a0)*(d[i]+a1)`. Thus for `a0 != 0` the cancelled chart is
projectively equal to the source `project` formula. At `a0=0`, equation (2)
supplies a nonzero polynomial continuation. The independent native replay
checked this agreement on the 21,595 translated degree-two classes for
which the source chart was nonzero and found the remaining 5,599 source-zero
classes nonzero under the continuation.

The candidate degree-one chart uses the **same** invertible matrix:
`K_1(x)=C*(0,1,x,x²)^T`. Its input vector has second entry 1, hence it is
also nonzero for every `x` over all 21 fields. This proves nonvanishing,
not that `K_1(x)` is the geometric projection of `(x,y)-infinity` for every
point, nor that it obeys every torsion action. For degree zero, the four
recorded theta coordinates are nonzero by the separate exact checks.

## Remaining action identity

For each of the five recorded roots `r`, let `U=u(r)` and work first on
`U != 0`. Put `c_r=-v(r)/U` and `L=v+c_r*u`. Because `L(r)=f(r)=0`, and
because `f-L²` is divisible by `u` modulo both Mumford equations, the
localized quotient admits

```text
u' = (f-L²)/(u*(x-r)),   v' = -L mod u'.
```

The second Mumford equation is

```text
2*b0*b1 - a1*b1² - R1 = 0.                                (3)
```

An all-state action certificate for the recorded matrix `M_r` must clear
the powers of `U` appearing in `u',v'`, substitute the polynomial direct
chart into every projective cross-product

```text
K_i(u',v') * (M_r K(u,v))_j
    - K_j(u',v') * (M_r K(u,v))_i,  0 <= i < j < 4,
```

and reduce the resulting six polynomials modulo equations (1) and (3) over
the **exact** field and constants of every one of the 21 models, for each
of the five roots. The result must be zero polynomials, with the denominator
and exceptional branches documented. The matrix identities and the finite
Jacobian comparisons in this packet do not perform that reduction; the
generic geometric identity remains **not established**. See
[`problems/kummer-generic-action-identity.md`](../../../problems/kummer-generic-action-identity.md)
for the explicit exit conditions. A successful generic certificate would
still need the `U=0` branches and the degree-one/zero geometric charts to
establish a complete all-state action law.

**Subsequent result (25 October 2026):** the [generic ideal packet](GENERIC_IDEAL.md)
now contains exact zero-remainder certificates for all 630 cross-products
on the 105 recorded model/root pairs with `U!=0`, independently checked by
native code. The preceding paragraph records the prior state of this
packet; the exceptional and degree-one/zero branches it names remain open.
