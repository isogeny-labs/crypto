# Exact nonvanishing of the cancelled chart; action gate still open

This note uses the 21 frozen odd-prime Rosenhain models and their recorded
theta constants. **Correction (25 October 2026):** an earlier version
identified the degree-two chart matrix with
`src/main.rs::conversion_matrix` and claimed that the degree-one candidate
used the same matrix. That identification is false. The degree-two matrix
`C2` and the source degree-one matrix `C1` have different third columns.
The matrices are nevertheless equivalent **degree-two coordinate bases**.
Put `sigma=1+lambda+mu+nu=-f4`. Their third columns satisfy
`C2[:,2]=-C1[:,2]-sigma*C1[:,3]`, with all other columns equal. Hence
`C2(1,-a1,-a0,q3)=C1(1,-a1,a0,q3+sigma*a0)` exactly. N15's `general()`
inverts the valid `C1` representation; the different columns do not
invalidate its candidate inversion. The original finite replay used the actual source `C1`; its counts are
unchanged. A retained [wrong-basis diagnostic](results/exceptional-ideal-wrong-basis.json)
gives nonzero exceptional cross-products in all 105 pairs when `C2` is
incorrectly reused for degree one. The corrected [exact packet](EXCEPTIONAL_IDEAL.md)
checks both matrices and all branches. The algebra below is interpreted
with this correction, and the original mistaken matrix claim is retracted.

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
constants, and let `C2` be the **degree-two chart** matrix, with
row `i` equal to

```text
C2[i,*] = s[i] * (c[i]*d[i], -c[i], d[i], -1).
```

The distinct name `c_theta` avoids confusing a theta scale with the array
`c`. `C2` differs from `src/main.rs::conversion_matrix`, denoted `C1`;
the latter is used for degree one and for the alternate degree-two
coordinate vector above.
The direct cancelled chart has coordinate

```text
K_i = s[i] * ((c[i]-a0)*(d[i]+a1)-H).
```

Expanding gives the **polynomial identity**

```text
K = C2 * (1, -a1, -a0, a0*a1+H)^T.                          (2)
```

No Mumford equation or division is required for (2). The first entry of the
right-hand input is 1. The exact finite-field determinant checks in
`results/exceptional-ideal.json` show `det(C2) != 0` in each of the 21
fields. The older `results/summary.json` determinant refers to `C1`, not
`C2`. Since the equivalent `C1` vector also starts with 1, the older
nonzero `C1` determinant does prove degree-two nonvanishing after the
basis change. The direct `C2` determinant supplies a separate check.
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

The source candidate degree-one chart uses the **different** matrix `C1`:
`K_1(x)=C1*(0,1,x,x²)^T`. The exact packet independently checks
`det(C1) != 0` on all 21 fields, so this vector is nonzero for every `x`.
For degree zero, the four recorded theta coordinates are nonzero by the
separate exact checks. The chart's geometric interpretation remains a
separate question from its nonvanishing and algebraic action identities.

## Action identities and remaining interpretation

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

The [generic ideal packet](GENERIC_IDEAL.md) clears `U^6` and reduces every
projective cross-product

```text
K_i(u',v') * (M_r K(u,v))_j
    - K_j(u',v') * (M_r K(u,v))_i,  0 <= i < j < 4,
```

modulo equations (1) and (3) over the exact field and constants of every
model/root pair. It retains 630/630 zero remainders on `U!=0` and a
separate native checker. The [exceptional packet](EXCEPTIONAL_IDEAL.md)
uses the corrected source `C1` and proves the `U=0` identities and
`O`/torsion cases; matrix involution yields the reverse degree-one action.
These results establish the action identities for the **declared charts**
on every valid reduced class of the 21 models. A source-independent proof
that the declared formulas give the intended geometric Kummer embedding,
and a complete signed collector cost against rho, remain open; see
[`problems/kummer-generic-action-identity.md`](../../../problems/kummer-generic-action-identity.md).
