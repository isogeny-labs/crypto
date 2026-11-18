# A second jet decoder for the difference surface

Let C be a smooth canonical plane quartic in characteristic different from2.
Let D=C-C in Jac(C), at a point z=P-Q with distinct P,Q whose canonical chord
meets C transversely in four distinct geometric points P,Q,R,S. Use translation
invariant tangent coordinates (formal logarithms truncated at degree2 suffice).
Let G be any local reduced equation of D, with gradient n and Hessian H at z.
Then n cuts out the chord line and, among its six unordered pairs, **only {P,Q}**
satisfies H(v(A),v(B))=0, where v(A),v(B) are canonical tangent vectors.
Changing local equation by a unit changes H on the tangent plane only by a scalar.
Thus the pair is determined by the second jet; its ordering remains undetermined.

## Proof and exact complementary-pair calculation

The difference morphism (A,B) -> [A-B] has tangent directions omega(P),-omega(Q).
They span the plane n=0. Its mixed second derivative vanishes in invariant
coordinates, hence H(v(P),v(Q))=0. Smoothness and a local reduced equation hold
at such a pair: a nonhyperelliptic curve has no degree2 pencil, so a nonzero
P-Q has a unique ordered representation. The tangent directions are independent.
The generically one-to-one difference morphism is locally an immersion here.

Choose coordinates with the chord Z=0 and none of its four points at infinity
in the line parameter t. Put v_i=(1,r_i,0), and write
f(t)=F(1,t,0)=a*product(t-r_i). Locally use t as parameter on C with F_Z!=0,
and the canonical differential vector (1,t,Z)dt/F_Z. Such a choice is possible
on a dense open set; the resulting identity is algebraic and extends to all
transverse chords by changing the transverse coordinate/parameter.

Normalize the gradient of G to dZ. Differentiating G(A(P(t))-A(Q(s)))=0 twice,
and using Z'=-F_t/F_Z, gives
H(v_1,v_1)=f'(r_1), H(v_2,v_2)=-f'(r_2), H(v_1,v_2)=0.
The first two signs reverse together if G is negated, with no effect on zeros.
Let d=r_2-r_1. In the basis v_1,v_2, v_j has coordinates
((r_2-r_j)/d,(r_j-r_1)/d). Therefore

H(v_3,v_4) = -2*a*(r_1-r_3)*(r_1-r_4)*(r_2-r_3)*(r_2-r_4)/d !=0.

Every other pair shares one endpoint with {1,2}; its bilinear value is a nonzero
basis coefficient times f'(r_1) or f'(r_2). Transversality and char!=2 make all
five competing pairs nonzero. This proves uniqueness, independent of F_Z at
particular points, since the final expression only uses distinct line roots.
For the sum morphism A(P(t))+A(Q(s)), the second diagonal has the same sign as
the first; then the complementary pair also has zero bilinear value. That is a
specific negative control: replacing subtraction by addition destroys uniqueness.

In general affine coordinates (x,y,1), with differential vector
(x,y,1)dx/F_y and n the gradient row, define
D(A)=n_x F_y(A)-n_y F_x(A). Then the difference Hessian satisfies
H(P,P)=-D(P), H(Q,Q)=D(Q), H(P,Q)=0. These are the formulas used by the
standalone destination-only gate; choosing the opposite normal reverses all.

## Application to the existing quotient and remaining proof obligations

Geometry13 supplies a translated Gamma00 section centered at the rational two-torsion image S=psi(T). Its divisor contains S+(C-C), not generally C-C itself. Translate by S before using the following origin-centered argument. To use the
algorithm, verify that the actual characteristic127 section defines the reduced
difference surface globally (not only its tangent cone): lift the identity with
the rank7 unit minor as in RECONSTRUCTION-PROOF.md, and compare divisor class
[C-C]=2Theta to exclude extra effective components. This last class comparison
and relative specialization must be recorded before interpreting source zeros; the final section below supplies that argument.
No empirical target-only decoder can replace that obligation.

For a source zero x, the decoded difference is psi(x-T), since the section is centered at psi(T). Translate the rational2torsion
Mumford chart to x before taking derivatives of the pulled-back section. Its
origin parameters have inversion t->-t; in a commutative formal group over
char!=2, the quadratic group-law term vanishes (a symmetric bilinear term has
zero diagonal), so these parameters agree with invariant coordinates to order2.
This avoids an uncharged connection correction. The isogeny is etale and its
tangent identification is the same one that produced the canonical quartic.
Check this construction and its jet regularity explicitly on the source.

This decoder alone maps only the difference surface. To map general public
inputs one must construct a decomposition into difference-surface points, recover
consistent signs and verify addition. Because 2T=0, zeros x and y=D-x decode to images summing to psi(D). Random intersection/decomposition cost,
exceptional chords, denominators, descent and verification must all be charged.
No 10x or end-to-end claim follows from this lemma or the destination-only gate.

## Prior art and search boundary

Pauly–Previato, Singularities of 2theta-divisors, introduction(1.2),
https://math.univ-cotedazur.fr/u/pauly/singularities.pdf : Gamma00 and C-C over C.
The characteristic127 use requires the explicit specialization above.
Tian https://arxiv.org/html/2007.03172 §4.2 uses Kummer alignment and lifting;
its comparison remains the established route. The second-jet calculation above
is derived here; no literature novelty assertion is made. KB lexical query
'difference surface C-C Gauss map second fundamental form quartic divisor pair'
on revision9141e386254340a9bdba3036f2bb706a produced no direct relevant decoder;
that does not establish absence. Web searches found Gauss-map material but no
primary result used beyond these identities. Repo source: geometry13 proof,
transport13 pinned section-space arithmetic and its explicit raw receipts.

## Global difference-divisor specialization argument

Throughout this argument translate the target origin to S=psi(T); translate back at the end, giving S+(C-C). Here is the additional argument needed above, separate from the destination-only
execution. Take the smooth canonical quartic lift over the Witt DVR used in
geometry13's reconstruction proof. Its relative Jacobian is smooth. Lift the
symmetric theta characteristic (2 is invertible), its eight second-order sections,
and the origin parameters. The rank7 special jet matrix has an invertible minor,
so its kernel has a primitive rank1 lift whose reduction is the known Gamma00
section. On the generic characteristic0 fiber Fay's identity makes that section
vanish on every [P-Q]. Pull back to the relative product of the lifted curve with
itself: a section zero generically is zero everywhere, since this smooth product
is flat over the DVR and its invertible sheaf has no uniformizer torsion.
Consequently the special section vanishes on C-C, as a scheme pullback.

The difference map CxC -> C-C has generic degree1: two ordered representations
P-Q=P'-Q' would give a degree2 pencil unless the pairs agree (or the zero case).
The image is irreducible. The class of its divisor is2Theta: the Pontryagin
product [C]*[-C] equals [C]*[C], as [-1] fixes the cohomological curve class;
the latter is twice[W2]=2Theta since the generic sum map to W2 has degree2.
One can compute this in characteristic0 and specialize intersection numbers
through the same smooth family; no identification of rational cycles is needed.
Thus C-C and the Gamma00 divisor both have intersection degree12 with Theta²
in dimension3. Their effective residual divisor would have zero intersection
with the square of an ample class, impossible unless empty. The multiplicity
along C-C is exactly1 by the same argument. Hence the special section defines
the reduced difference surface, and its generic local jets are legitimate input
to this decoder. This proves a geometric prerequisite, not execution of source
jet evaluation, nor a general-public-input transport algorithm.
