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
Earlier SCIS 2010 and 2011 papers by Shimura, Momose, and Chao specifically
study elliptic curves with weak coverings over cubic extensions in even
characteristic; their titles are recorded in the primary references of
Iijima, Momose, and Chao (2013).
The available abstract does not expose the full case table, and the related
thesis is access restricted. Therefore this report does **not** claim that
the triangle type, the trace gate, or its density is new in the literature.

Primary references:

* Momose–Chao, https://eprint.iacr.org/2006/347
* Nart–Ritzenthaler, https://arxiv.org/abs/0905.0546
* Murai–Shimura–Iijima–Chao, SCIS 2022 abstract,
  https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf
* Iijima–Momose–Chao, *Classification of Elliptic/Hyperelliptic Curves with
  Weak Coverings against the GHS Attack under an Isogeny Condition* (2013),
  reference list for the SCIS 2010 and 2011 even-characteristic cubic papers,
  https://eprint.iacr.org/2013/487.pdf
* Murai thesis repository entry (full text access restricted),
  https://chuo-u.repo.nii.ac.jp/records/17759
* Diem, *On the discrete logarithm problem for plane curves* (2012),
  https://www.numdam.org/item/10.5802/jtnb.815.pdf
* Joux–Vitse, *Cover and Decomposition Index Calculus on Elliptic Curves made practical*
  (EUROCRYPT 2012), https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf
* Tian, *Cover attacks for elliptic curves with prime order* (2020),
  https://arxiv.org/pdf/2012.07173
* Magma handbook, canonical images and plane-curve index calculus,
  https://magma.maths.usyd.edu.au/magma/handbook/text/1473

The exact equations and independently replayed point counts below give
**local, explicit Jacobian witnesses** for two isogeny classes whose LMFDB
`has_jacobian` field was `0` (unknown) on 27 October 2026. In that database,
`-1` denotes a negative status and `1` a positive one. These witnesses are
potentially new as exact equations or catalog additions; prior publication
of equivalent curves has not been excluded.
Tian's cubic-extension transfer through a genus-three Jacobian is also
relevant prior art for the attack complexity, though its map uses an
`(ell,ell,ell)`-isogeny rather than the pullback/trace correspondence here.
The present ordinary binary source curves all have even group order and
therefore do not instantiate Tian's prime-order setting.

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

The curve is **geometrically non-hyperelliptic**. Each of the three branch
poles has a different nontrivial inertia subgroup of V4, and all three
nontrivial V4 quotients have genus one. If a hyperelliptic involution existed,
it would be central and could not lie in V4. It would induce an automorphism
of the quotient projective line commuting with V4 and therefore fixing each
of the three individually labeled branch poles. An automorphism of the
projective line fixing three distinct points is the identity, placing the
involution in V4, a contradiction. Thus the canonical model is a smooth
plane quartic over `k`.

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

## Explicit base-field model and evaluated Jacobian map

The invariant
`u=(alpha+alpha^(q^2))*y0+(alpha^q+alpha^(q^2))*y1`, with `alpha=t0`,
generates the whole degree-four extension over `k(x)`. Indeed the V4 deck
translations send `u` to `u`, `u+p`, `u+v`, and `u+p+v`, where
`p=alpha+alpha^(q^2)` and `v=alpha^q+alpha^(q^2)` are distinct nonzero
elements. Thus `L(U)=U(U+p)(U+v)(U+p+v)` is a separable additive quartic
over `k`, and reducing `L(u)` by the two Artin–Schreier equations gives a
rational function `N(x)/D(x)` with coefficients in `k`. The executable
`descent_model.py` constructs this exact `k(x,u)` equation for each case.

For order 4222, the common field coefficient codes give the explicit model

```
(u^4 + 2622*u^2 + 3158*u) *
  (x^6 + 3362*x^4 + 2623*x^2 + 2623)
 = 3159*x^5 + 3158*x^4 + 2891*x^3 + 1641*x^2 + 1821*x.
```

All displayed coefficients lie in `k=F16` embedded in `K` under the bitmask
convention above. The affine equation is singular as a plane model; its
smooth projective normalization is `C`. The four generated `descent_*.json`
files give the analogous exact equations and one `F16` affine lift per
`x in F16`. `audit.py` rechecks the rational identities at every unramified
`x in K` with independent bit-level field arithmetic.

Magma 2.30-1 independently computes each normalization's geometric genus
as three, obtains a non-hyperelliptic smooth canonical plane quartic, and
returns the predicted L-polynomial. The exact calculator input, full
response, coefficient vectors, and canonical maps are retained in
`canonical_*.magma`, `canonical_*_receipt.json`, and `canonical_data_*.json`.
For an immediately readable exact plane equation, put

```
(M0,...,M12) = (X^4, X^2 Y^2, X Y^3, Y^4, X^2 Y Z, X Y^2 Z,
               Y^3 Z, X^2 Z^2, X Y Z^2, Y^2 Z^2, X Z^3, Y Z^3, Z^4)
F_N(X,Y,Z) = sum_(i=0)^12 c_i M_i.
```

The following rows give `c_0,...,c_12` as the same embedded-field bitmasks.
Magma checks `F_N=0` smooth over `F16`, and the Sage map evaluator checks
the named source fibers lie on it.

| Source order | Exact canonical quartic coefficient vector |
|---:|---|
| 3972 | `[1,1641,2890,3362,2623,2890,372,372,3362,3159,3363,3158,373]` |
| 4006 | `[1,1641,2890,3362,2623,2890,1821,372,3362,2622,3363,1640,373]` |
| 4198 | `[1,1641,2890,1641,2623,2890,2891,372,3362,1640,3363,2890,373]` |
| 4222 | `[1,1641,2890,3159,2623,2890,2890,372,3362,1,3363,2622,373]` |

Finally, `subgroup_map.py` evaluates both source points in the actual Sage
Jacobian group of each canonical quartic after base change to `K`. It chooses a known multiple
`R=[n]G` whose fibers, and those of `R+G` and `R+Q`, all split over `K`.
Because `[(R+P)]-[R]` represents `P` on an elliptic curve, the difference of
their degree-two pullbacks represents `pi_0^*P`. Summing its three Frobenius
conjugates gives `Phi(P)` without needing a rational fiber above the identity.
Each run checks that `Phi(G)` is nonzero, `[r]Phi(G)=0`, `Phi(Q)=[7]Phi(G)`,
and `[r]Phi(Q)=0`. This is an evaluated subgroup-preserving map, not just
the push-pull existence proof.

| Source order | Translated base `n` | `r` | Jacobian setup (s) | Two map evaluations (s) | Group checks |
|---:|---:|---:|---:|---:|---|
| 3972 | 17 | 331 | 14.68 | 5.06 | all pass |
| 4006 | 36 | 2003 | 14.12 | 5.56 | all pass |
| 4198 | 7 | 2099 | 14.76 | 5.95 | all pass |
| 4222 | 10 | 2111 | 14.93 | 5.65 | all pass |

These are observed stage times on a shared host, with process setup and
field construction outside the timed region. They are retained only as map
operation evidence and do not measure a complete discrete-logarithm attack.

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

The cover equations, canonical quartics, and evaluated correspondence
establish **transfer eligibility** for the named odd-prime subgroups.
Relation collection, linear algebra, target recovery through the quartic,
and same-source-subgroup rho measurements are still needed for a full attack
cost. There is no measured GHS-over-rho advantage for these candidates.
The exact equations for the two catalog-unknown classes remain potential
new curve witnesses; their cryptanalytic vulnerability and publication
novelty remain open.
Magma's built-in plane-curve `IndexCalculus` rejects the `q=16` quartics:
its documented line-relation implementation requires at least `q >= 4! = 24`.
An order-4222 call returned “Field too small with respect to degree.” A
custom small-field relation implementation or a `q>=32` member of the
family is required for that specific engine. This software limit does not
alter Diem's asymptotic theorem.

There is also a theorem-level asymptotic comparison. Diem's Theorem 1 gives
expected `O~(q)` for the DLP in the Jacobian of a non-hyperelliptic genus-three
curve over `F_q`; the explicit quotient correspondence takes each source
odd-prime DLP into that Jacobian, and the fixed-genus canonical quartic is
effective from the function-field equations. For a sequence of eligible
source instances with prime subgroup order `r=Theta(q^3)`, generic rho has
expected `Theta(sqrt(r))=Theta(q^(3/2))` group operations. This is an
**asymptotic algorithmic bound**, with the condition on `r` stated explicitly.
It does not prove that such large-prime instances occur infinitely often,
give a concrete crossover, or replace the missing full attack measurements.
Joux–Vitse's characteristic-two genus-three **hyperelliptic** direct-cover
condition `Tr_(K/k)(b)=0` selects `Theta(q^2)` curves; the present quartic
construction selects `q^3-1` normalized models and has a different geometric
target. The 2022 classification remains relevant prior art, so this density
comparison is not a global novelty claim.

Reproduce in this directory from the repository root:

```
sage -python research/ghs_twisted_v4_triangle_20261027/triangle.py
python3 research/ghs_twisted_v4_triangle_20261027/census.py
python3 research/ghs_twisted_v4_triangle_20261027/audit.py
lean research/ghs_twisted_v4_triangle_20261027/TriangleGate.lean
sage -python research/ghs_twisted_v4_triangle_20261027/descent_model.py
python3 research/ghs_twisted_v4_triangle_20261027/magma_canonical.py
sage -python research/ghs_twisted_v4_triangle_20261027/subgroup_map.py
```
