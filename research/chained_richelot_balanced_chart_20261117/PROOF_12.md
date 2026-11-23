# Balanced shifted chart: geometric argument and executable obligations

This is a written argument using the cited classical divisor theory. It is not
a new Lean construction of curves, valuations or Picard groups. Executable
coverage is governed separately by `PROTOCOL_12.md`. [pcc:unverified]

Let k have characteristic different from2, let H be a monic squarefree sextic,
and let a,b lie in k with b nonzero and b²=H(a). Set

    F(w)=w⁶ H(a+1/w)/b²,
    w=1/(x−a), z=y w³/b,
    x=a+1/w, y=b z/w³.

F is monic of degree6 because its leading coefficient is H(a)/b². Its constant
coefficient is1/b². The inverse rational functions identify the function fields
and preserve the curve equations. [pcc:unverified]

At x=a, the source has the two unramified points(a,±b). In the output infinity
chart t=1/w, r=z/w³, the map is t=x−a, r=y/b, so it is regular there and maps
the two points to ratios±1. At source infinity use s=1/x and q=y/x³; then

    w=s/(1−as), z=q/[b(1−as)³].

This is regular at s=0 and sends the two source infinities to Q±=(0,±1/b).
The inverse expressions give the reverse local maps at both sets of points.
Together with the affine formulas these cover the smooth complete curves and
give mutually inverse k-morphisms. No square root outside k is chosen.
[pcc:unverified]

By Galbraith corrected8.3.8/8.3.10, pushforward preserves principal divisors,
induces a group homomorphism on Pic⁰, and composes with the inverse map to the
identity. Thus it is a group isomorphism. It restricts to a bijection on every
actual cyclic subgroup and preserves scalar relations. This statement concerns
this degree1 chart; it does not construct a nontrivial Richelot correspondence.
[pcc:unverified]

Write K=∞++∞− for the balanced reference divisor. Its image is Q++Q−. The
divisor of w on the target is Q++Q−−K_target, so these references are linearly
equivalent. Consequently the class of a degree2 effective D minus K_source
maps to the class of its pointwise image minus K_target. Galbraith10.4.9/10/14/19
then supplies the balanced representation and reduction law. [pcc:unverified]

For finite support(U,V) of degree m≤2, substitute into its binary homogenization:
w^m U(a+1/w). Its degree drops by exactly the multiplicity of x=a in U. Each
lost occurrence goes to the infinity branch V(a)/b; unramifiedness at x=a
ensures this is±1 even for a tangent double. Other support is represented by
the monic remaining polynomial and ordinate w³V(a+1/w)/b modulo it. The
source n copies of∞+ and2−m−n copies of∞− go to Q+ and Q−. The inverse has
the formulas in the protocol, including its lost branch bV(0).
[pcc:unverified]

For implementation, begin with the remaining finite support in a balanced
class with n=0. Add one δ_target=∞+−∞− for every lost positive infinity,
then add each old-infinity image using its point class Q−∞−. The latter is
the balanced degree1 representation with n=1. The resulting divisor is exactly
the image effective divisor minus K_target; reducing it introduces only
principal-divisor changes. The inherited group reduction algorithm must still
pass the declared finite oracle/roundtrip/addition controls. [pcc:unverified]

The same substitutions apply to quadratic closed support. They are defined
over k and commute with Frobenius; conjugate points remain conjugate, so the
support and ordinate coefficients descend. The unit checks this explicitly in
F101². It does not execute arbitrary higher-degree closed support.
[pcc:unverified]

Changing b to−b composes the output with the hyperelliptic involution. For any
point P, P+ι(P) is a hyperelliptic fibre linearly equivalent to K_target, so the
involution acts as−1 on Pic⁰. Both signs therefore give legitimate class maps,
with outputs opposite in the group. This does not select the sign of a
separately implemented Richelot dual. [pcc:unverified]

Conditionally, if an actual Richelot map φ and its dual satisfy φ̂φ=[2], and
σ,τ are checked source/target chart isomorphisms, the conjugated maps
τφσ⁻¹ and σφ̂τ⁻¹ still compose to[2]. That follows from cancellation and
commutation of group isomorphisms with scalar multiplication. Evaluating this
composition on the CT49 divisor classes, with exceptional fibres and correct
orientation, remains the subsequent native obligation. [pcc:unverified]
