# Frobenius-cycled triangle covers in characteristic two

Research status: 27 October 2026. This is a cover-eligibility and transfer-map
result. It does not establish a faster discrete-logarithm attack.

## Prior-work boundary

The genus-three V4-cover construction and its three elliptic quotients are
within an established research area. Nart and Ritzenthaler (2009) study
genus-three curves with three involutions in characteristic two. Momose and
Chao (2006) classify `(2,...,2)` Weil-restriction covers; the SCIS 2022
abstract by Murai, Shimura, Iijima, and Chao explicitly reports a complete
even-characteristic classification under the isogeny condition `g(C)=d*g(E)`.
The available abstract does not expose the full case table, and the related
thesis is access restricted. Therefore this report does **not** claim that
the triangle type, the trace gate, or its density is new in the literature.

Primary references:

* Momose–Chao, https://eprint.iacr.org/2006/347
* Nart–Ritzenthaler, https://arxiv.org/abs/0905.0546
* Murai–Shimura–Iijima–Chao, SCIS 2022 abstract,
  https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf
* Murai thesis repository entry (full text access restricted),
  https://chuo-u.repo.nii.ac.jp/records/17759

The exact equations and independently replayed point counts below give
**local, explicit Jacobian witnesses** for two isogeny classes whose LMFDB
`has_jacobian` field was `0` (unknown) on 27 October 2026. In that database,
`-1` denotes a negative status and `1` a positive one. These witnesses are
potentially new as exact equations or catalog additions; prior publication
of equivalent curves has not been excluded.

## Family theorem and exact lower bound

Let `q=2^m`, `k=F_q`, `K=F_(q^3)`, and let `E/K` be an ordinary normalized
curve

```
E_(a,b): Y^2+XY=X^3+aX^2+b,       b != 0.
```

Write `Tr` for the absolute trace `K -> F_2`, and take the unique exponent
`d mod (q^3-1)` satisfying `2(q+1)d = 1 mod (q^3-1)`. The inverse exists:
`q^3-1` is odd and `gcd(q+1,q^3-1)=1`. The following condition constructs a
degree-two genus-three cover of `E` after cubic descent:

```
                         Tr(a) = Tr(b^d).                (1)
```

This is an **exact criterion for this triangle construction**. It is a
sufficient condition for a low-genus cover, not a classification of every
possible GHS or cover attack. Since `b -> b^d` permutes `K*`, the trace-zero
fiber has `q^3/2-1` nonzero values and the trace-one fiber has `q^3/2`
values. There is exactly one eligible normalized twist for each `b != 0`:
`q^3-1` of the `2(q^3-1)` normalized ordinary models. At `q=16`, this is
4095 of 8190 models, split 2047/2048 by absolute trace. The proof applies
to all `m>=1`; the finite q=16 census in `census.json` is an independent
enumeration of its first large example here.

Choose any nonzero `delta in K` with `Tr_(K/k)(delta)=0`. There are `q^2-1`
choices, and none lies in `k`. Solve `t0+t0^q=delta`, then put
`t1=t0^q`, `t2=t0^(q^2)`. Set `lambda=b^d` and

```
A = delta^(q^2-q+1) * lambda,
f0(x) = A/(x+t0) + A^q/(x+t1),
f1(x) = A^q/(x+t1) + A^(q^2)/(x+t2),
f2(x) = f0(x)+f1(x).
```

The smooth projective normalization of
`y0^2+y0=f0(x), y1^2+y1=f1(x)` is a connected V4-cover over `K`.
Its three simple branch poles have inertia two. The different contributes
four at each pole, so Riemann–Hurwitz gives
`2g(C)-2 = 4*(-2)+3*4 = 4`, hence `g(C)=3`. The coefficient Frobenius
cycles `(f0,f1,f2)`; the order-three semilinear action
`sigma(x)=x, sigma(y0)=y1, sigma(y1)=y0+y1` supplies descent to `k`.

Here is the explicit first quotient. Put `Delta=t0+t1=delta`,
`beta=A/Delta`, `gamma=A^q/Delta`, `s=beta*gamma`, `c=beta+gamma`.
Then `s^2=b`. The absolute-trace identity

```
Tr(c) = Tr(A/Delta + A^q/Delta)
      = Tr(A*(1/Delta + 1/Delta^(q^2)))
      = Tr(A*Delta^(q-1-q^2)) = Tr(lambda)
```

uses `Delta+Delta^q+Delta^(q^2)=0`. Condition (1) therefore gives
`h^2+h=c+a` in `K`. On the quotient `y0^2+y0=f0(x)`, define

```
X = gamma*(x+t0)/(x+t1),
Y = X*y0+s+X*h.
```

Direct substitution gives `Y^2+XY=X^3+aX^2+b`. Conversely, if (1)
fails, this fixed triangle quotient is the other normalized twist;
changing `delta` does not change `Tr(c)`.

## Subgroup-preserving correspondence

Let `pi_i:C_K -> E_i` be the three degree-two elliptic quotients, with
`E_i=E_0^(q^i)`. For `P in E_0(K)`, the divisor-class correspondence

```
Phi(P) = sum_(i=0)^2 pi_i^*([P^(q^i)]-[O_i]) in Jac(C)(k)
```

is Galois invariant. V4 character orthogonality makes cross terms
`pi_i* pi_j^*` zero for `i != j`, while `pi_i* pi_i^*=[2]`.
Thus `pi_0*Phi(P)=[2]P`. On every odd-order subgroup, `Phi` is injective.
The induced product map `Jac(C)_K -> E_0 x E_1 x E_2` and the reverse sum
compose to `[2]` on each factor, so the Jacobian is k-isogenous to
`Res_(K/k)(E_0)` with 2-primary kernel. For `t_K=q^3+1-#E(K)`, its Weil
polynomial is `T^6-t_K*T^3+q^3` and `#Jac(C)(k)=#E(K)`.

The certificate field `pullback_fibers` records the exact x-coordinate,
`y0`, Artin–Schreier right-hand side for `y1`, its two rational roots when
the degree-two divisor splits, and the projected `2P` on E. A nonsplit
fiber is still a degree-two divisor over K. Sage constructs these fibers;
`audit.py` checks them using a separate bit-level field and curve group law.
`TriangleGate.lean` kernel-checks the q=16 exponent/count arithmetic and
the trivial kernel of doubling on each named prime-order source group.
It does not formalize the function-field geometry or compute Jacobian divisor
arithmetic.

## Exact q=16 witnesses

`K=F_2[w]/(w^12+w^7+w^6+w^5+w^3+w+1)`; integer codes are coefficient
bitmasks in the basis `1,w,...,w^11`. All rows use `delta=14` and
`(t0,t1,t2)=(574,560,3373)`. `Q=[7]G` in each row.

| Catalog class | E parameters `(a,b)` | `#E(K)` | prime `r` | `G` | `Q` | `#C(F16),#C(F256),#C(K)` | Catalog Jacobian status |
|---|---:|---:|---:|---:|---:|---:|---|
| `3.16.a_a_aev` | `(0,51)` | 3972 | 331 | `(1226,1765)` | `(1143,3210)` | `17,257,3722` | unknown (`0`) |
| `3.16.a_a_adn` | `(32,198)` | 4006 | 2003 | `(199,2783)` | `(738,3018)` | `17,257,3824` | positive (`1`) |
| `3.16.a_a_dx` | `(32,116)` | 4198 | 2099 | `(117,3864)` | `(1053,3821)` | `17,257,4400` | positive (`1`) |
| `3.16.a_a_ev` | `(32,853)` | 4222 | 2111 | `(852,2301)` | `(112,1443)` | `17,257,4472` | unknown (`0`) |

For the two catalog-unknown classes, the Weil polynomials are respectively
`T^6-125*T^3+4096` and `T^6+125*T^3+4096`. The LMFDB rows say these are
simple abelian-threefold isogeny classes; the constructed covers show that
their Jacobian status is positive if the displayed descent and point-count
proof are accepted. The existing `has_jacobian=0` entries are catalog gaps,
not mathematical nonexistence claims.

At q=16 the fixed-delta family has 1472 models outside the previously
certified Hess genus-three isogeny closure. Among those, 1356 models lie
in 44 simple classes with a positive catalog Jacobian status, while 24
models (12 each) lie in the two catalog-unknown classes above. These
numbers are relative to the frozen Hess and simple-restriction inputs whose
SHA-256 hashes are recorded in `census.json`; they are not a global count
of newly discovered attacks.

## Attack-cost and novelty limits

The cover equations and correspondence establish **transfer eligibility**
for the named odd-prime subgroups. A working genus-three Jacobian divisor
implementation, relation collection, linear algebra, target recovery, and
same-source-subgroup rho measurements are still needed for a full attack
cost. There is no measured GHS-over-rho advantage for these candidates.
The exact equations for the two catalog-unknown classes remain potential
new curve witnesses; their cryptanalytic vulnerability and publication
novelty remain open.

Reproduce in this directory from the repository root:

```
sage -python research/ghs_twisted_v4_triangle_20261027/triangle.py
python3 research/ghs_twisted_v4_triangle_20261027/census.py
python3 research/ghs_twisted_v4_triangle_20261027/audit.py
lean research/ghs_twisted_v4_triangle_20261027/TriangleGate.lean
```
