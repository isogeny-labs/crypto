# Peer review: exact transport of two chord labels at M31

Date: 2026-11-25, America/Los_Angeles. Status: a directly derived algebraic
statement with proposed native qualification; no formal certificate or measured
solving advantage. The prior handoff's unseen full text is not assumed here.

## 1. Inputs and conventions

Take `p = 2147483647`, `Fp² = Fp[i]/(i²+1)`, and a nonsingular elliptic
curve `E: y² = x³+a*x+b`. All projective coordinates below lie in Fp.
The Cayley parametrization is

`C(X:Z) = (X+iZ)/(X-iZ)`.

It is a bijection `P¹(Fp) -> T₂(Fp)`, and `C(1:0)=1`.
Because -1 is nonsquare, both X+iZ and X-iZ are nonzero whenever
`(X,Z)!=(0,0)`. Scaling by any nonzero Fp scalar does not change C.

Fix an affine residual target `K=(w,v)` with `v != 0`. For j=1,2 choose
and certify

`S_j=(s_j,d_j)`, `d_j=i*delta_j`, `delta_j in Fp^*`,

`d_j²=-delta_j²=s_j³+a*s_j+b`.

Then `S_j^p=-S_j`, and S_j is not rational. Put `h_j=s_j-w`.
Necessarily `h_j!=0`: an equality would give `v²=d_j²`, hence
`d_j=±v in Fp`, a contradiction. Distinct support pairs are a useful
test case, but are not necessary for the transport identity.

Let `eta_j=C(E_j:F_j)` be any fixed known torus factors, with nonzero
projective pairs `(E_j,F_j)`. “Fixed” means fixed conditional on the
current residual target and prefix. Neither eta_j depends on the
unselected residual chord.

## 2. Label and its inverse

The finite chord through -K has equation

`ell_m(x,y)=y-m(x-w)+v=0`.

Its evaluation ratio at S_j and -S_j is

`t_j(m)=ell_m(S_j)/ell_m(-S_j)`

`      =(v-m*h_j+d_j)/(v-m*h_j-d_j)`

`      =C(v-m*h_j:delta_j)`.

Set the raw label `z_j=eta_j*t_j`. At the vertical chord `m=infinity`,
the normalized label is 1 and the raw label is eta_j. No finite chord
evaluation vanishes: its real component lies in Fp, whereas d_j is
nonzero and purely imaginary.

Given `t_j != 1`, the quantity

`L_j=d_j*(t_j+1)/(t_j-1)`

is fixed by Frobenius and therefore lies in Fp. Its inverse slope is

`m=(v-L_j)/h_j`.

Together with `t_j=1 -> m=infinity`, this proves an exact bijection
between all p+1 slopes and all p+1 torus labels. The inverse can also
be computed from the Fp projective pair for t_j, avoiding an Fp²
inverse: if `t_j=C(X:Z)`, `Z!=0`, then

`m=(v*Z-delta_j*X)/(h_j*Z)`.

## 3. Transport matrix

Write `L_j=v-m*h_j`. Eliminating m gives

`h_1*L_2=h_2*L_1+(h_1-h_2)*v`.

Since `t_1=C(L_1:delta_1)`, the normalized transport on arbitrary
projective coordinates is

`N = [[h_2*delta_1, (h_1-h_2)*v], [0, h_1*delta_2]]`.

Indeed N maps `(L_1:delta_1)` to `(L_2:delta_2)` after cancelling the
nonzero common scale `h_1*delta_1`. Its determinant is

`det N=h_1*h_2*delta_1*delta_2 != 0`.

Multiplication by a raw torus factor eta=C(E:F) is represented by

`M_eta=[[E,-F],[F,E]]`,

because `C(E:F)*C(X:Z)=C(E*X-F*Z:F*X+E*Z)`.

Its determinant `E²+F²` is nonzero. A projective inverse is

`adj(M_eta)=[[E,F],[-F,E]]`.

Thus an entirely Fp raw-label transport is

`T=M_eta2 * N * adj(M_eta1)`.

The use of the adjugate, instead of the literal inverse divided by its
determinant, is legitimate because projective scaling cancels. With this
adjugate convention,

`det T=(E_2²+F_2²)*(E_1²+F_1²)*h_1*h_2*delta_1*delta_2 != 0`.

T is therefore a bijection of P¹(Fp). It maps raw eta_1 to raw eta_2.
Raw label 1 is generally NOT the vertical chord: the vertical chord has
raw label eta_j. Confusing these is a concrete exceptional-case risk.

For a direct Fp² cross-check put

`A=h_2*d_1`, `B=(h_1-h_2)*v`, `C0=h_1*d_2`.

Then

`t_2=((A+B+C0)*t_1+(A-B-C0))`

`    /((A+B-C0)*t_1+(A-B+C0))`.

The determinant of this Fp² representation is `4*A*C0 != 0`.
The following raw-label equation avoids division entirely:

`z_2*((A+B-C0)*z_1+(A-B+C0)*eta_1)`

` == eta_2*((A+B+C0)*z_1+(A-B-C0)*eta_1)`.

## 4. A precise connection to four summands

These chord labels must not be identified with the earlier extractor's
`product_j ((x_j+i)/(x_j-i))` coordinate labels without a separate
derivation. The following is one valid, explicitly defined full-divisor
label to which the transport applies.

For rational points U,V let W=U+V. In the ordinary nonexceptional case,
define

`f_UV = ell_UV / (x-x(W))`,

where ell_UV is the chord/tangent through U,V,-W. It has divisor

`div(f_UV)=[U]+[V]-[W]-[O]`.

If W=O and U,V are nonidentity opposites, use the vertical function
`x-x(U)` instead. If either input is O, use the constant function 1.
Tangency uses the usual tangent line. These choices give the same
divisor formula in every case. Evaluation at the nonrational supports
is well-defined: their x-coordinates cannot equal the x-coordinate of
a rational curve point, and the rational chord intersection points
cannot contain either nonrational support.

Given `P_1+P_2+P_3+P_4=R`, put

`A=P_1+P_2`, `K=R-A=P_3+P_4`.

Assume K is affine with nonzero ordinate, as required by this component.
Set

`F=f_P1P2 * f_P3P4 * f_AK`.

Adding the three divisors gives exactly

`div(F)=[P_1]+[P_2]+[P_3]+[P_4]-[R]-3[O]`.

The raw function-evaluation label is `z_j=F(S_j)/F(-S_j)`.
All vertical denominator factors have equal evaluations at S_j and
-S_j and cancel in the ratios. Therefore

`z_j=eta_j*t_j(m)`,

`eta_j=(f_P1P2(S_j)/f_P1P2(-S_j))`

`      *(f_AK(S_j)/f_AK(-S_j))`.

Both factors in eta_j are computable from R and the fixed prefix
P_1,P_2. The residual factor is exactly the chord label in section 2.
If A=O, both prefix factors can be taken to have ratio 1. F is unique
up to a nonzero scalar once its principal divisor is fixed, so the
evaluation ratio is unchanged by that scalar or by an alternative valid
addition-tree factorization.

There is a projective implementation that avoids even a chord-slope
inversion when evaluating the prefix factors. For finite distinct points
`U=(u,y)`, `V=(x,z)`, let `N0=z-y`, `D0=x-u`, and
`C0=N0*u-D0*y`. For a tangent use `N0=3*u²+a`, `D0=2*y`
and the same C0. The line is `D0*Y-N0*X+C0`; its evaluation ratio
is the projective torus coordinate

`C(C0-N0*s_j : D0*delta_j)`.

A nonconstant vertical line has D0=0 and produces the identity label.
For an input O use the identity label directly, rather than trying to
construct a line from affine coordinates. Prefix eta_j is the torus
product of the two resulting projective line labels. The equal vertical
denominators of f_UV have already cancelled at the two supports.

This establishes a conditional four-summand application for this
explicitly defined principal-divisor label. It does not establish that
the previous unseen note used this label, that the old x-Cayley-product
constraint is equivalent to it, that smoothness survives the map, or
that imposing one or two such labels improves the ECDLP relation yield.

## 5. Exact counting consequences

For fixed K, supports and eta factors, exactly p+1 raw-label pairs are
compatible: the graph `{(z,Tz):z in P¹(Fp)}`. Thus independent uniform
raw labels are compatible with probability `1/(p+1)`. After choosing
the first label, the second is uniquely forced; it is not a second
independent selector for this pencil.

For finite m, the residual x-coordinates solve

`X²-(m²-w)X+(w*m²+w²+a+2*v*m)=0`.

The discriminant is

`Delta(m)=m⁴-6*w*m²-8*v*m-3*w²-4*a`.

The corresponding y-coordinates are `m*(X-w)-v`. The infinite slope
represents `{O,K}` and is outside an affine factor base.

When the group has odd order r, the involution P -> K-P has one fixed
point. There are `(r+1)/2` unordered group-pair orbits, and exactly one
is `{O,K}`. Thus exactly `(r-1)/2` finite slopes have a rational affine
residual pair. The remaining `p-(r-1)/2` finite slopes are nonsplit.

For the primary certified M31 fixture with `r=2147487637`, these are
1,073,743,818 affine rational pairs, 1,073,739,829 nonsplit finite
chords, and one identity pair. The total is exactly `p+1=2147483648`.

For any affine base B with n points define

`h=|{P in B:K-P in B}|`, `d0=|{P in B:2P=K}|`.

Here d0 is 0 or 1. Exactly `U=(h+d0)/2` normalized labels give an
unordered pair wholly in B. Uniform one-label success is `U/(p+1)`;
independent uniform two-label success is `U/(p+1)²`. A forced second
label preserves the first-label experiment exactly.

These are exact conditional counts. They do not assert any useful rank,
runtime lower bound, uniformity of smooth labels, or complete DLP cost.

## 6. Allowed-set intersection kernel

For finite allowed sets A_1,A_2 of raw labels, compatible first labels
are exactly `A_1 intersect T^{-1}(A_2)`. Hashing both sets costs
O(|A_1|+|A_2|) expected time and memory under the ordinary hash-table
model. After that construction, enumerate the smaller set, apply T
or its inverse, and probe the other. This costs
O(min(|A_1|,|A_2|)) expected membership operations plus output handling.
Deterministic ordered tables give a log-factor membership alternative.

Duplicate labels must be removed or explicitly weighted; they do not
represent multiple residual slopes. Compatible labels may still yield
nonsplit residuals or points outside the base. Those rejections and
quadratic extraction must remain charged. If sets are rebuilt for each
target, their construction is part of each query's cost.

This kernel is an ordinary intersection under a known permutation. It
does not supply a new sublinear shared geometric index. Its useful role
is exact compatibility filtering and accounting before extraction.

## 7. Exceptions, scope and review checks

* Reject singular curves, noncanonical field input and invalid projective
  pairs. Curve/support certification belongs to setup and is charged there.
* K=O and zero-ordinate K require separate residual logic; neither is
  covered by the component's stated interface. The two supplied prime
  odd-order fixtures have no nonzero rational point of ordinate zero.
* Valid nonrational supports guarantee h_j and delta_j are nonzero.
  Do not quietly accept a rational support or a zero delta.
* Exercise raw eta_j, raw label 1, finite zero slope, tangent/double-point
  residuals, and nonsplit discriminants as distinct cases.
* Test matrices only projectively: scalar multiples represent the same
  label and transport. Canonicalization must not divide by zero.
* A second label on another unresolved branch or chord pencil is outside
  this redundancy statement. Prefix changes alter K, eta and T; there is
  no global fixed transport asserted across all prefixes.
* The direct Fp² reference should evaluate line functions independently,
  not call the matrix or reproduce its coefficients as its only oracle.

All identities in this note were derived algebraically from the stated
definitions. Neither the four-summand extension nor two-label transport
has been claimed as a measured speedup or as a published novelty result.
