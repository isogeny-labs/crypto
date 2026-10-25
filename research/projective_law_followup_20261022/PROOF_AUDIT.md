# Weighted-law proof audit, 2026-10-22

This audit addresses the exact case split in `src/bin/projective_law_gate.rs` on
the 42 recorded Scholten models. It adds a finite, deterministic audit; that
finite audit alone does **not** certify universal completeness. Later sections
give exact coefficient certificates and written divisor arguments. The two already measured
1,260-target collector arms and their cost ratios are unchanged.

## Input and execution

`projective-proof-audit` reads the 42 distinct prime/model receipts in
`../five_leads_e2e_20261019/correctness/genus2-model-gate-v1.jsonl`. Each
receipt verifies the affine coordinate map to the monic quintic without an
`x^4` term on 100 transferred divisors and 100 additions. For each mapped
model, the new Rust audit constructs a 2,048-step divisor pool and draws
deterministic 2+2 and 1+2 pairs at scales `(1,1,1,1)` and `(2,3,5,7)`.
It compares every returned weighted result to the independent Cantor
`Jacobian::add`. The seed and branch counts for each model are in
`results/proof-audit-20261022.jsonl`.

From the repository root:

```sh
cargo build --release --offline --locked --manifest-path research/projective_law_followup_20261022/Cargo.toml --bin projective-proof-audit
research/projective_law_followup_20261022/target/release/projective-proof-audit > research/projective_law_followup_20261022/results/proof-audit-20261022.jsonl
```

The audit checked 1,020,000 sampled 2+2 pairs and 204,000 sampled 1+2
pairs across all 42 mapped models, with zero mismatches or unresolved
`Option::None` returns. The 2+2 branch counts were 1,018,216 common
degree-two, 594 degree-one drop, 617 shared cancellation, 562 shared
positive, and 11 same-`u` mixed. The 1+2 branch counts were 203,752
common, 124 cancellation, 121 coincident double plus other, and 3 tripling.
These counts are finite observations, not probabilities or a proof.

## Case predicates that can be discharged algebraically

Assume a smooth quintic `y²=f(x)` over a field of odd characteristic,
valid reduced Mumford inputs, and nonzero projective weights. The following
facts concern branch *selection* only; they do not establish the output
formula identities.

1. In `add12_weighted_complete`, `k=0` means the point's `x` is a root
   of the quadratic `u`. Both its `y` and the `v` value at that root
   square to `f(x)`, so they are equal or opposite. Thus
   `coincident_y_mismatch` is unreachable for valid inputs.
2. In `add22_weighted`, `bb=0` and `dq=0` imply
   `bb=-dr²=0`, hence `dr=0`; the same-`u` branch has already run.
   Thus `shared_zero_q_delta` is unreachable for valid inputs.
3. In the same-`u` branch, equal `v` slopes and unequal constants
   imply `v₂-v₁=c≠0`. Because both `vᵢ²=f mod u`,
   `c(v₁+v₂)=0 mod u`. The sum has degree at most one, so it vanishes
   identically: `v₂=-v₁`. The inverse branch has already run. Thus
   `same_u_mixed_zero_slope_delta` is unreachable for valid inputs.
4. In the 2+2 shared-support branch, the recovered points share an
   `x` coordinate on the same curve. Their `y` values are equal or
   opposite after accounting for weights; `shared_y_mismatch` is
   unreachable for valid inputs.
5. For a non-Weierstrass point `P`, the degree-two representation of
   `2P` has `u=(x-x_P)²`. Therefore a second point `R` with
   `x_R≠x_P` cannot hit a secondary coincident support in the
   `2P+R` path. If `x_R=x_P`, the remaining possibilities are the
   inverse or tripling branches. Smoothness excludes a valid reduced
   `u=(x-x_P)²`, `v(x_P)=0` at a Weierstrass point because it would
   force `f'(x_P)=0`.

The [published complete law](https://doi.org/10.1007/s13389-024-00351-7)
is for the mapped equation family. The implementation corrected the
homogeneity of one `W` power in the doubling expression; the retained
`p=739` regression demonstrates that the literal transcribed term gives
a different result. The paper's completeness statement cannot by itself
certify the modified implementation.

## Exact coefficient certificates and divisor arguments

### Exact generic 1+2 identity, added after the finite audit

The new native `projective-symbolic-12` checker proves seven affine
equalities and nine weighted-scale equalities as canonical polynomials over
`Z[x,y,q,r,s,t,a0,a1,a2,a3,alpha,mu,zeta,omega]`. The output is
`results/symbolic-1plus2.json`. This is an exact coefficient proof for
the **generic weighted-input, K ≠ 0** branch, not another finite-field sample.

Write `u(X)=X²+qX+r`, `v(X)=sX+t`, `F(X)=X⁵+a3X³+a2X²+a1X+a0`,
and `F-v²=uH+C`, where `deg C<2`. A valid Mumford input has `C=0`.
For a point `P=(x,y)`, put `K=u(x)`, `A=y-v(x)`,
`B=sK+Aq`, and `D=tK+Ar`. The branch condition is `K≠0`.
Let `h1` be the coefficient of `X` in `H`; exact division gives
`h1=a3+q²-r`. Define

```text
Q = K²(x-q)-A²
R = K² h1 - A(B+sK) + xQ
S = AQ-BK²
T = ARK²-DK⁴
U = K²X²+QX+R
N = Kv+Au
G = K²H-2KAv-A²u.
```

The checker verifies, without reducing modulo a prime or substituting
random values,

```text
K²F-N² = uG + K²C
G = (X-x)U + G(x)
G(x) = K(F(x)-y²-C(x))
K²(K²N-AU)+K²SX+T = 0.
```

Hence, if `C=0` and `y²=F(x)`, then with `L=N/K`,
`F-L²=u(X-x)(U/K²)`, and
`L≡-[(S/K³)X+(T/K⁵)] mod U/K²`. These are exactly the generic
affine-input coordinates returned by `add12_weighted_generic` after
normalization: its `q3=Q`, `r3=RK²`, `s3=S`, `t3=T`, and
`z3=K,w3=1`. The standard divisor-of-`y-L` argument identifies this
reduced Mumford pair with `P+D₂`. This proves the branch's output
identity for valid affine inputs and `K≠0`. The native checker proves
the stated polynomial equalities; the displayed divisor argument is
written mathematics, not a Lean theorem.

For arbitrary nonzero point weights `(alpha,mu)` and degree-two weights
`(zeta,omega)`, substitute the prescribed projective input coordinates
into the literal implementation. Put `M=alpha⁵ zeta⁵ mu omega`. The
checker verifies its intermediates scale as
`K_w=alpha⁴ zeta⁴ mu K`, `A_w=MA`, `B_w=M zeta² B`, and
`D_w=M zeta⁴ D`. Its outputs satisfy
`(q3,r3,s3,t3,z3)=(M²Q,M⁴K²R,M³S,M⁵T,MK)`. These are precisely the
coordinate weights used by `normalize`; the weighted output therefore
normalizes to the affine pair above. The nine scale equalities are checked
over the integer polynomial ring, so they hold for every field
specialization where the input weights and `K` are nonzero.

### Exceptional 1+2 cancellation and nontripling positive branch

`projective-symbolic-12-exception` is a separate, versioned native
certificate in `results/symbolic-1plus2-exception.json`. It checks 12
additional equalities over the same integer polynomial ring.

When `K=u(x)=0`, put `x₄=-q-x`. Exact polynomial division gives
`u(X)=(X-x)(X-x₄)`. The literal weighted code computes
`x₄,w=alpha² zeta² x₄` and
`y₄,w=alpha⁵ zeta⁵ omega v(x₄)`, with
`z₄=alpha zeta,w₄=omega`. Its cancellation predicate is
`alpha⁵ zeta⁵ mu omega (y+v(x))=0`. With nonzero input weights, this
is precisely `y=-v(x)`. The normalized output is `(x₄,v(x₄))`.
The relation `P+ι(P)∼2∞` on the smooth odd-characteristic
hyperelliptic curve proves that this is the sum of `P` and the
degree-two input. This also covers repeated-root support when the
input is a valid reduced divisor.

For the remaining `K=0`, `y=v(x)≠0` branch, the certificate expands
the literal weighted `dbl(P)` output. After normalization it has
`u₂(X)=(X-x)²` and
`v₂(X)=y+[F'(x)/(2y)](X-x)`; the identity
`2F(x)-xF'(x)=2y²-xF'(x)` uses the point equation.
The tangent construction therefore represents `2P`. If
`x₄≠x`, then `u₂(x₄)=(x₄-x)²≠0`, so the already proved generic
weighted 1+2 formula applies to `P₄+2P`. This is exactly the
implemented `coincident_double_plus_other` path and equals the
desired `P+D₂`. If `y=0`, the code selects cancellation first;
the positive branch does not run. The `x₄=x` tripling formula is
handled by the separate exact certificate below.

### Tripling with repeated support

`projective-symbolic-triple` checks five affine and eleven weighted
integer-polynomial equalities in `results/symbolic-tripling.json`. Its
variables are independent over `Z[x,y,a0,a1,a2,a3,alpha,mu]`; the point
equation is applied only after the coefficient checks. Let
`F(X)=X⁵+a3X³+a2X²+a1X+a0`, `P=(x,y)`, `k=8y³`, and
`N(X)=aa X²+2bb X+cc`, where

```text
aa = 2F''(x)y² - F'(x)²
bb = 2F'(x)y² - aa*x
cc = y*k - (aa*x+2bb)*x
Q = 3x*k² - aa²
R = a3*k² - 4aa*bb + 3x*(Q-x*k²)
S = aa*Q - 2bb*k²
T = aa*R*k² - cc*k⁴.
```

The checked identities are

```text
N(x) = k*y
k²F(X)-N(X)² = (X-x)³(k²X²+QX+R)+k²(F(x)-y²)
k⁴N(X)-aa*k²(k²X²+QX+R)+k²S*X+T = 0.
```

The literal affine `triple` output has
`(q3,r3,s3,t3,z3,w3)=(Q,k²R,S,T,k,1)`; its normalized
`u=(k²X²+QX+R)/k²` and
`v=(S/k³)X+T/k⁵`. On the curve, the first polynomial identity
gives `F-(N/k)²=(X-x)³u`. Since `k≠0`, the function
`y-N(X)/k` has a triple zero at `P` and two residual zeros, with a
pole of order five at infinity. The second identity says the returned
`v` is `-N/k` modulo `u`, so its divisor class is `3P`. The code
reaches `triple` only in the `x₄=x`, `y=v(x)≠0` branch; `y=0`
selects cancellation first.

For arbitrary nonzero input point weights `(alpha,mu)`, the checker
recomputes the literal weighted intermediates and proves
`(q3,r3,s3,t3,z3,w3)` scales as
`(alpha³²mu⁶ Q, alpha⁶⁴mu¹² k²R, alpha⁴⁸mu¹⁰ S,
alpha⁸⁰mu¹⁶ T, alpha¹⁶mu³ k, mu)`. The normalized output is
therefore the same affine pair. These are exact integer-polynomial
equalities, with the divisor argument stated in writing rather than Lean.

### Generic coprime 2+2 branch

`projective-symbolic-22-common` checks nine affine and ten weighted
equalities in `results/symbolic-2plus2-common.json`. Write the valid
inputs as `uᵢ=X²+qᵢX+rᵢ`, `vᵢ=sᵢX+tᵢ`, and their coefficient
differences as `dq,dr,ds,dt` (first minus second). The code computes

```text
B = dq*(q2*r1-q1*r2)-dr²
A = dq*(dt*q2-ds*r2)-dr*dt
C = dq*dt-dr*ds.
```

`B` is minus the resultant of `u1,u2`, so `B≠0` means their supports
are coprime. The cubic numerator
`N=B*v1-(C*X+A)*u1` is checked to satisfy
`N≡B*v1 (mod u1)` and `N≡B*v2 (mod u2)`; hence `L=N/B`
interpolates the four input points. The branch requires `C≠0`, so
`L` is cubic. Define

```text
D = dq*C²+A*C
Q = D+A*C-B²
Rbar = A*(dq*C+A)-(s1+s2)*B*C+(q1+q2)*B²
U = C²X²+QX+Rbar.
```

The checker expands `E=N²-u1*u2*U` and verifies that its `X⁶`
and `X⁴` coefficients vanish, its `X⁵` coefficient is `B²`, and
`E≡B²vᵢ² (mod uᵢ)` for both inputs. Because `B≠0`, the two
quadratics are coprime. Both `E/B²` and the input curve `F` are
monic quintics with no `X⁴` term and the same residues modulo their
degree-four product. Their difference has degree at most three and is
divisible by that product; therefore `E=B²F` exactly. Thus
`F-L²=-(u1*u2/B²)U`. The code returns `r3=C²Rbar` and
`z3=C`, so its normalized output `u` is `U/C²`.

The separate [Lean uniqueness packet](formal/COMMON22_UNIQUENESS.md)
mechanizes the explicit determinant Bézout identity, its `B≠0` coprimeness
consequence, the degree-two and nonzero properties of both source-style
supports, both conditional residue bridges from interpolation and valid
curve-input congruences, the strict degree bound from a sextic upper bound
with zero sixth coefficient, matching quintic heads, and quadratic supports,
and the final coprime-divisibility-plus-degree
implication in this paragraph. Four further Lean theorems expand a generic
`N²-u1*u2*U` polynomial and identify its coefficients in degrees six, five,
and four. Four source-style Lean theorems substitute the common-branch
coefficient formulas into the proof-side polynomial, obtaining `0,B²,0`,
and identify the cubic with its interpolation expression. The source-style
cubic also has an exact factorization modulo the second support. The residual
has degree at most six; for any monic quintic with zero fourth
coefficient, its cleared difference has degree below `u1*u2`.
The source-style theorem now combines nonzero `B`, valid Mumford input
congruences, interpolation, coprimeness, and the degree bound to prove
`E=B²F` for that proof-side polynomial. This does not establish that every
Rust execution satisfies the theorem's premises. Rust-to-Lean semantic
correspondence has not been proved for every runtime execution.
The divisor-class step remains written mathematics.

The checker also verifies the cleared identity
`C⁵N+S*C²X+T = U*[-C⁴X+C³(-Cq1-A)+C²Q]`, where `S,T` are the
literal output formulas. With `w3=B`, its normalized output
`v=(S/(BC³))X+T/(BC⁵)` is `-L mod U`. The function
`y-L` has pole order six at infinity and vanishes at the four input
points and two residual points; the returned reduced divisor is their
sum class. The ten exact scale identities show that arbitrary nonzero
input weights give the same normalized pair. This proves only the
`B≠0,C≠0` branch of the implemented law, not its exceptional paths.

### Coprime 2+2 degree-one drop (`B≠0,C=0`)

`projective-symbolic-22-drop` checks eleven affine and seven weighted
integer-polynomial equalities in `results/symbolic-2plus2-drop.json`.
It uses the preceding notation and parameterizes `C=0` by
`(ds,dt)=h(dq,dr)`. This covers every field input with `B≠0,C=0`:
`B≠0` implies `(dq,dr)≠(0,0)`, and zero determinant `C`
then makes the two difference vectors proportional. The checker
verifies `A=B h`, so `N=Bv1-Au1` and `L=N/B=v1-hu1` have degree at
most two and interpolate both input divisors.

Put `Xnum=(q1+q2)B²+A²`. The checker expands
`E=N²+u1u2(B²X-Xnum)` and verifies that its `X⁶,X⁴`
coefficients vanish, its `X⁵` coefficient is `B²`, and it is
congruent to `B²vᵢ²` modulo each `uᵢ`. The same uniqueness argument
as in the generic branch gives `E=B²F` for any valid input curve;
therefore `F-L²=u1u2(X-Xnum/B²)`. The literal affine code returns
`x=Xnum/B²`. An independent cleared equality in the certificate
identifies its `y` numerator with `-B⁴N(Xnum/B²)`, so its normalized
ordinate is `-L(x)`. The function `y-L` has pole order five at
infinity, four input zeros, and this one residual zero. Its inverse
is the code's degree-one output, proving the branch's divisor class.

For nonzero input weights, let `Z=z1z2` and `W=w1w2`. The literal
weighted output satisfies `(x,y,z,w)=(Z¹⁸W² Xnum,
Z⁴⁵W⁵ Ynum,Z⁹W B,1)`, with `Ynum` the affine code numerator;
the seven checked identities include these scales. Normalization
therefore gives the same point. This certificate is for `B≠0,C=0`
only; shared support and same-`u` paths are separate.

### Same-`u` mixed support

`projective-symbolic-22-mixed` checks seven exact integer-polynomial
identities in `results/symbolic-2plus2-mixed.json`. It verifies that the
literal crossed-coordinate calculations in `add22_weighted` extract
the point with affine coordinates
`x₀=(t₂-t₁)/(s₁-s₂)` and `y₀=v₁(x₀)` whenever
`k=s₁-s₂≠0`; the weighted output sent to `dbl` has
`z=(z₁z₂)⁴(w₁w₂)k`, nonzero for nonzero input weights.

For valid inputs with the same `u`, both `F-v₁²` and `F-v₂²` are
divisible by `u`, so `u` divides `(v₁-v₂)(v₁+v₂)`. In the mixed
branch `v₂≠v₁` and `v₂≠-v₁`. Both factors are nonzero. The existing
branch-selection lemma excludes a nonzero constant `v₁-v₂`, so
`k≠0`. If `v₁+v₂` were constant, the product would have degree
at most one and divisibility by monic quadratic `u` would force
the product to vanish identically, a contradiction. Thus both
factors are linear, their product is a nonzero scalar times `u`,
and `x₀` is one support root. At that root `v₁=v₂=y₀`;
at the other root `v₁=-v₂`, so those two points cancel in the
Jacobian sum. The common point cannot be Weierstrass: if `y₀=0`,
the two distinct linear factors would share root `x₀`, forcing
`u=(X-x₀)²` and a valid reduced divisor with doubled smooth
Weierstrass support, which is impossible. Hence `dbl(P₀)` has a
nonzero output weight and equals `2P₀` by the separately certified
weighted point-doubling formula. This proves the mixed branch's
output class. Equal-`v` same-`u` divisor doubling is certified separately
below.

### Same-`u`, equal-`v` divisor doubling

`projective-symbolic-22-double` checks 30 exact integer-polynomial
equalities in `results/symbolic-2plus2-double.json`, including literal
weighted intermediates and all three `B,C` paths of
`double22_weighted`. The input is a valid reduced Mumford pair
`u=X²+qX+r`, `v=sX+t` on the stated monic quintic. Divide
`F-v²=uH+R` with `deg R<2`; validity means `R=0`. In affine
coordinates, the code's values are

```text
n1=qs-t, n2=rs,
B=2(n1*t-n2*s),
n5=s²-q(4r-q²)+q*a3-a2,
n6=2r-3q²-a3,
A=n1*n5-n2*n6, C=s*n5-t*n6.
```

The checker verifies the exact Hermite identity
`B H+2v(CX+A)=uJ` for a polynomial `J`. Put
`N=Bv-(CX+A)u` and
`U=C²X²+(2AC-B²)X+(A²-2sBC+2qB²)`.
It checks `BJ-(CX+A)²=-U`, hence

```text
B²F-N² = -u²U+B²R.
```

When `B,C≠0` and `R=0`, `L=N/B` is cubic and
`F-L²=-u²U/B²`. The function `y-L` has the doubled input
zeros and two residual zeros; its pole order is six. The output
has normalized `u₃=U/C²`, and the checker verifies the cleared
congruence `C⁵N+S C²X+T≡0 (mod U)` for the literal `s3=S,t3=T`.
With `z3=C,w3=B`, the returned `v₃=-L mod u₃`, so it is the
divisor-class double. Thirteen scale identities check the literal
weighted generic path, including the corrected `w²` factor in `n6`.

When `B≠0,C=0`, the same exact identity gives
`U=-B²X+Xnum` with `Xnum=2qB²+A²`, so
`F-L²=u²(X-Xnum/B²)`. The code's degree-one branch returns
`x=Xnum/B²`, and its checked `y` numerator equals
`-B⁴N(Xnum/B²)`, giving `y=-L(x)`. Three further scale
identities verify the literal weighted point output. This is the
degree-one divisor-class double.

Finally `B=-2 Res(u,v)` exactly. If `B=0,s=0`, odd
characteristic forces `t=0`; both support points are
Weierstrass, so the divisor is 2-torsion and the code's
identity output is correct. If `B=0,s≠0`, the root
`x₀=-t/s` of `v` is a Weierstrass support point. The other
support point is `x₁=-q+t/s`; it is non-Weierstrass for a
valid reduced divisor on a smooth curve. The certificate checks
that the weighted code extracts `(x₁,v(x₁))` and calls the
already certified point-doubling law. Since twice the
Weierstrass point is the identity, this equals twice the
original divisor. These are written divisor arguments backed
by exact polynomial identities; no Lean divisor-class theorem
is claimed.

### Distinct-`u` shared support (`B=0`)

`projective-symbolic-22-shared` checks 27 exact integer-polynomial
identities in `results/symbolic-2plus2-shared.json`. For distinct
`u₁,u₂`, the earlier branch lemma gives `dq=q₁-q₂≠0` when `B=0`.
The checker verifies
`dq²uᵢ(-dr/dq)=-B` for both quadratics. Thus
`x₀=-dr/dq` is their common support root. Their other roots are
`x₂=-q₁-x₀` and `x₄=-q₂-x₀`; all three root formulas are checked
after clearing `dq` denominators. Write `Z=z₁z₂`. The literal
weighted branch constructs four points with common `z=Z³dq`,
each `x` numerator scaled by `Z⁶`, and each `y` numerator scaled
by `Z¹⁵wᵢ`. The point/scaling equalities prove that these are exactly
`P₁=(x₀,v₁(x₀))`, `P₂=(x₂,v₁(x₂))`,
`P₃=(x₀,v₂(x₀))`, and `P₄=(x₄,v₂(x₄))`.
The checked crossed-ordinate equality shows the code's comparisons
are exactly `y(P₁)=±y(P₃)`. The curve equation and odd
characteristic imply one of these for valid inputs; the
`shared_y_mismatch` return is unreachable.

When the common ordinates are opposite, `P₁+P₃∼2∞`, and the
sum is `P₂+P₄`. Distinct `u` ensures `x₂≠x₄`, so the literal
`add11` distinct-point path applies. The same certificate checks
its weighted output: normalized `u=(X-x₂)(X-x₄)` and `v`
is the line through both points; two additional exact incidence
identities check that line. This is the required divisor class.

When the common ordinates agree and are zero, the opposite test
runs first, so the cancellation argument applies. Otherwise the
sum is `2P₁+P₂+P₄`. The code calls the certified point-doubling
law, then `add_point_to_result` on `P₂` and `P₄`. If
`P₂≠P₁`, adding it to `2P₁` uses the certified generic 1+2
branch because `u_{2P₁}=(X-x₀)²`; if `P₂=P₁`, it uses the
certified tripling branch. The final addition is dispatched
through the certified 1+2, 1+1, or identity cases, including
its coincident and degree-drop paths. Thus the two shared-support
output paths follow from exact extraction/point-addition
identities and the previously proved helper branches. This is
a written compositional divisor argument, not a Lean theorem.

Proof map: the generic 1+2 coefficient identities are discharged by
`src/bin/projective_symbolic_12.rs` and its committed result; the
exceptional cancellation and point-doubling identities are discharged
by `src/bin/projective_symbolic_12_exception.rs` and its separate
result. Tripling, both coprime 2+2 branches, and both same-`u` paths have independent certificates
in `src/bin/projective_symbolic_triple.rs` and
`src/bin/projective_symbolic_22_common.rs` and
`src/bin/projective_symbolic_22_drop.rs` and
`src/bin/projective_symbolic_22_mixed.rs` and
`src/bin/projective_symbolic_22_double.rs` and
`src/bin/projective_symbolic_22_shared.rs`. A later pinned
[Lean boundary packet](formal/README.md) declares three branch-selection
lemmas and three literal formula fragments, including both generic 2+2
interpolation factorizations. It does not formalize the runtime semantics,
exceptional-branch composition, smooth-curve/Mumford model, or the
principal-divisor and `Pic^0` bridge. Its conditional bridge theorem only
applies a caller-supplied implication and is not claimed as the full
group-law proof.

### Remaining review boundary

The implemented 1+1, 1+2, and 2+2 branch map now has exact
integer-polynomial output certificates and written divisor arguments
for each reachable case in the implemented mapped family: smooth monic
quintics with no `x^4` term over odd-characteristic fields, with valid
reduced inputs and nonzero weights. This is not a mechanized universal theorem: the divisor-class
reasoning, branch composition, and smooth-curve hypotheses have no
faithful Lean declarations, and an independent mathematical review
has not yet been recorded. In particular, no exhaustive all-divisor
pair enumeration was run on any of the 42 curves. The finite audit
alone cannot establish universal completeness; no counterexample to
the implemented weighted law was found in that audit.

### Secondary read-only static audit

A separate agent inspected the branch predicates and `None` exits in
`src/bin/projective_law_gate.rs`, this written case split, and all eight
`projective_symbolic_*` certificate programs. It found no concrete
missing reachable branch or contradictory predicate for that mapped family
under the stated smoothness, odd-characteristic, valid reduced-input, and
nonzero-weight assumptions.
The eight committed certificate results report 145 exact
integer-polynomial equalities in total: 16, 12, 16, 19, 18, 7, 30,
and 27 respectively. This is a static review observation, not an
additional algebraic certificate.

The certificate programs transcribe the production formulas into their
own polynomial expressions; they do not invoke or mechanically link to
the runtime functions. Agreement of that transcription with the runtime
code was inspected manually and sampled by the finite 42-model Cantor
comparison, not proved by a source-to-certificate theorem. The
divisor-class and exceptional-branch composition arguments above are
also written mathematics rather than mechanized declarations. This
secondary agent audit is not an independent mathematical peer review
or an exhaustive all-divisor enumeration.
