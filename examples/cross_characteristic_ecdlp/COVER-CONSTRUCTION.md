# A concrete cubic cover and a smaller, solvable target

The construction ends in a seven-element elliptic subgroup over a field of
four elements. The source field has 65,537 elements, but its selected subgroup
also has order seven. This is an existential toy construction; it does not
claim to weaken a secure large-prime-order ECDLP. [pcc:unverified]

## The two elliptic fibers and their cubic covers

Let `k_p=F_65537` and `k_2=F_4={0,1,ω,ω+1}`, with `ω²+ω+1=0`.
Use the actual four-element subfield in `CoverDecodeExample.Small`, whose
cardinality and characteristic are checked in Lean. The curves are

\[
E_p:y^2-xy-4y=x^3-4x^2,\qquad
E_2:y^2+\omega y=x^3+\omega x^2.
\]

Both are nonsingular, with the chosen order-seven points `P=(0,0)` and
`R=(0,0)` respectively. Their covers are the smooth projective models of

\[
C_p:v^2-u^3v-4v=u^9-4u^6,\qquad
C_2:v^2+\omega v=u^9+\omega u^6.
\]

In each fiber, `π(u,v)=(u³,v)`. The substitution identities are checked over
integer polynomial rings and at the two actual fields. The coordinate-algebra
extension is `R_E[U]/(U³-x)`: Lean constructs a basis indexed by `Fin 3`,
so this is a finite free cover algebra of rank three. [pcc:unverified]

The geometric degree is three. On each elliptic curve,
`div(x)=[P]+[-P]-2[O]`. The three support points are distinct and the field
characteristics are prime to three. Adjoining a cube root of `x` is totally
ramified at each of these places: the valuations 1, 1 and −2 are prime to
three. The extension is a field because the valuation 1 excludes `x` being
a cube. Riemann–Hurwitz then gives

\[
2g(C)-2=3(2g(E)-2)+3(3-1)=6,
\]

so `g(C_p)=g(C_2)=4`. This is a written geometric proof; the full genus and
Jacobian models are not formalized in this Lean project. [pcc:unverified]

## The explicit cross-characteristic bridge

The bridge is supplied by a common torsion-marked integral family, not by a
field map between the two fibers. Put

\[
b=t^3-t^2,\quad c=t^2-t,\quad
\mathcal E_t:y^2+(1-c)xy-by=x^3-bx^2.
\]

Work over the open base where `21Δ(t)b(t)c(t)(t−1)` is invertible.
Both specializations `t=2` in `F_65537` and `t=ω` in `F_4` lie in this open
base. The first gives `(b,c)=(4,2)`; the second gives `(b,c)=(ω,1)`.
The two specializations of the elliptic equation are checked in Lean.

There is also a single, fixed quadratic integral lift realizing both fibers:

\[
A=\mathbb Z[\zeta]/(\zeta^2+\zeta+1),\qquad t_0=2+65537\zeta.
\]

At the prime above 2, `A` maps to `F_4`, `ζ↦ω`, and `t₀↦ω`. At the prime
above 65537, it maps to `F_(65537²)`, with `t₀↦2`. The reduced equation and
all seven marked points on that side descend to `F_65537`, so the original
source instance is unchanged. `CoverIntegralLift.lean` constructs the two
ring maps, checks `CharZero A`, both parameter images, both reduced models,
and the nonzero discriminants. These are maps out of a common integral ring,
not a homomorphism from one finite field to the other. [pcc:unverified]

The section `𝒫=(0,0)` has the following multiples:

| n | Section `[n]𝒫` |
| --- | --- |
| 0 | `O` |
| 1 | `(0,0)` |
| 2 | `(b,bc)` |
| 3 | `(c,b−c)` |
| 4 | `(c,c²)` |
| 5 | `(b,0)` |
| 6 | `(0,b)` |

Here is the algebra behind order seven. Tangent/chord addition gives the
second and third rows. For doubling the third multiple, its slope is `t−1`:
the denominator is `b−c−c²=−c(t−1)²`, a unit on the chosen open base, and

\[
(t-1)(b-c-c^2)=2c^2-bc-b+c,\qquad b^2-bc-c^3=0.
\]

These integer-polynomial identities are checked in Lean. Substitution in the
addition formulas gives `2([3]𝒫)=(0,b)=−𝒫`; hence `[7]𝒫=O`.
The displayed point is affine and therefore different from `O`, so its order
is exactly seven. The other rows follow by inversion. All seven sections are
integral and disjoint over the chosen open base. They identify a constant
order-seven subgroup in the two fibers. [pcc:unverified]

The induced fiber identification `τ([n]P)=[n]R` is explicit on these seven
sections and does not require an unknown logarithm oracle: the table has only
seven rows. At large subgroup order, constructing or evaluating such a lift
would be a separate obligation. The current construction supplies no bound or
algorithm for that extension.

## Pull back, specialize, and push forward

Let `T_t=(0,0)` on the cubic cover and let `∞_t` be its unique point at infinity.
Write `D_t=[T_t]-[∞_t]`. The ramification indices above `𝒫_t` and `O_t` are
three; their residue degrees are one. Consequently

\[
\pi_t^*([\mathcal P_t]-[O_t])=3D_t,\qquad
\pi_{t*}(D_t)=[\mathcal P_t]-[O_t].
\]

Thus `3D_t` is a seven-torsion section of the cover Jacobian. It has exact
order seven: its norm is `[3]𝒫_t`, which is nonzero. The seven multiples of
this section provide an identification `σ` between the covered subgroups of
`J(C_p)` and `J(C_2)`. This is specialization on the marked sections, not a
map between all points of the two Jacobians.

For the source subgroup, the concrete composite is

\[
\Phi=\pi_{2*}\circ\sigma\circ\pi_p^*=[3]\circ\tau.
\]

This formula is why the cover and de-cover preserve the logarithm. Since
three is prime to seven, `Φ(P)=[3]R` still has order seven. If `Q=[k]P`, then
`Φ(Q)=[k]Φ(P)`; injectivity reflects this equality as well. The bridge in
the middle is essential: cover and norm alone keep the characteristic fixed.
The norm is taken through the matching elliptic quotient, not the complementary
bielliptic quotient whose composite can vanish. [pcc:unverified]

## The smaller target and the scalar table

The source challenge is `Q=(2,2)=[3]P`. Put `G=Φ(P)=(1,ω+1)` and
`T=Φ(Q)=(ω,ω)`. For every integer k, Lean checks

\[
T=[k]G\quad\Longleftrightarrow\quad Q=[k]P
\quad\Longleftrightarrow\quad k\equiv3\pmod7.
\]

| n | Source `[n]P` | Composite image `Φ([n]P)` |
| --- | --- | --- |
| 0 | `O` | `O` |
| 1 | `(0,0)` | `(1,ω+1)` |
| 2 | `(4,8)` | `(0,ω)` |
| 3 | `(2,2)` | `(ω,ω)` |
| 4 | `(2,4)` | `(ω,0)` |
| 5 | `(4,0)` | `(0,0)` |
| 6 | `(0,4)` | `(1,1)` |

The target logarithm is recovered by enumerating these seven multiples of G.
It is the requested “something weaker” option; no GHS vulnerability or GHS
solver is claimed. The source order-seven subgroup is also enumerable, so
this establishes existence, not a new attack advantage. [pcc:unverified]

## What Lean does and does not certify

`CoverDecodeExample.lean` checks the two field sizes, the actual elliptic
points and orders, the affine cubic cover equations, rank-three cover-algebra
bases, universal integer-polynomial identities, the injective composite
`[3]τ`, the seven-row scalar table and the scalar-equivalence endpoint.
The geometric construction of the genus-four Jacobians, induced Picard maps,
and specialization of divisor sections is the written proof above; it is not
a fully formal Jacobian construction in the pinned Mathlib environment.
No arbitrary group is presented as a Jacobian. [pcc:unverified]

For the written specialization argument, normalize the integral elliptic
model in the cubic function-field extension. Near the two selected primes,
3 is a unit, the three branch sections stay disjoint, and their valuations
are prime to 3. The resulting tame cyclic cover has a smooth proper model
with those marked ramification sections. Thus its relative Picard group
admits the displayed pullback sections and their two specializations. This
is the geometric good-reduction input behind `σ`; it is not supplied merely
by the nonzero elliptic discriminants or postulated as a Lean axiom.
[pcc:unverified]

The exact-polynomial identity receipts are
`IDC1hd9c51d3b1a10865f`, `IDC1h7b0661d4e033557f`,
`IDC1hb3b3ac5e66594860`, and `IDC1h2bc056bd8eacea35`.
They point to the Lean source and are retained with the new raw receipts.
The publication claims are `PCC-20261115-1148a8` (scalar endpoint) and
`PCC-20261115-97a477` (integral specializations), unsigned pending independent
repository verification. The earlier unpublished claim and its ledger bytes
are preserved separately in the receipts; no incompatible ledger histories
were reconciled. [pcc:unverified]
