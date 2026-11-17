# Complete coefficient descent and the scalar boundary

The exact domain is the four Q8 F2 coefficient-block affine source graphs,
with every inverse witness, Kummer root and coefficient-Frobenius gate
retained. A scalar boundary model means every coefficient-algebra node
lies in the embedded scalar copy of F2, h=0 and at least one node is1. This is a diagnostic slice of
the geometric boundary, not an enumeration of all geometric points.

## Q35 written geometric bridge

Assume such a scalar point with U=y*S(x)=1 lies in the reduced projective
closure of the h!=0 source. Work over an algebraically closed coefficient
field. Curve selection and normalization give a DVR arc with generic
point in that source; ramified reparametrization is allowed. Normalize
h with orderN>0, rescaling the uniformizer over the algebraically closed
residue field so that the leading coefficient of h is1. All coefficient components of h*U have leading value1.

The finite coefficient algebra splits as360 copies. Its coefficient
Frobenius sigma permutes those copies and fixes the arc parameter; it is
NOT the absolute Frobenius of the Laurent-series field. All statements
below use this permutation action. Since U is a regular polynomial on the
ordinary elliptic affine chart, a pole is possible only at O. In the Y=1
chart t=X/Y,z=Z/Y, the elliptic equation is
z+t*z=t^3+a*t^2*z+z^3. Hence z=t^3 times a unit with constant1, x=t/z has
leading t^-2, and y=1/z has leading t^-3. Every actual S here is monic of
degree8, so U has exact leading t^-19. Thus N=19m and in every split
component t=c*T^m+... with c^19=1. No leading coefficient is zero.

The actual source descent is P0=-sigma(P3),
P1=sigma(P0)-sigma(P3), P2=sigma(P1)-sigma(P3),
P3=sigma(P2)-sigma(P3). Substitution gives
P0+sigma(P0)+...+sigma^4(P0)=O. The other ordinary coordinates
are partial sums P0+sigma(P0)+...+sigma^j(P0),so their fivefold
traces also vanish by additivity. This is an identity in the proper elliptic
group, extending the inverse-witness chart by properness/separatedness.
The formal group has linear term addition: its identity laws
F(t,0)=t and F(0,t)=t force the linear terms to be t1+t2,
with all remaining terms of degree at least2. Taking the order-m coefficient
therefore gives c+sigma(c)+...+sigma^4(c)=0 in every split component.
All five terms have nineteenth power1. This is impossible by the universal
root theorem below. Consequently these four Q34 scalar points cannot be
limits of the full source under these standard geometric bridges.

This argument uses the complete coefficient descent. It neither constructs
a simultaneous arc nor treats a split one-factor arc as a full source arc.
The DVR/curve-selection, proper elliptic group, split coefficient algebra
and formal-group leading-term bridges are written, not fully formalized
in Lean. No explicit saturated polynomial separator is extracted from it.
It justifies U=0 on the scalar boundary slice for each of the eight actual
ordinary monic degree8 y*S gates. It is not a global linear ideal equation.

## Universal root theorem

In characteristic two, five nineteenth roots of unity cannot sum to zero.
Let Tr18(z)=sum_{j=0}^{17} z^(2^j). Since2 has order18 modulo19, for
z^19=1 and z!=1 this is sum_{i=1}^{18}z^i=1; Tr18(1)=0. Among the19 roots
choose eta absent from the five terms. Dividing the proposed zero sum by
eta and applying the additive Tr18 gives five ones, or1, a contradiction.
A primitive nineteenth root can be adjoined before this argument. Lean
states its field/characteristic/primitive-root hypotheses explicitly and
proves the root theorem for every five-tuple, including repetitions. It
also proves the trace-zero implication from the four actual descent
identities in an arbitrary additive commutative group.

The independent native check uses an irreducible F(2^18) model, all19 roots,
a rank18 binary span,130321 ordered four-root prefixes and zero possible
fifth roots. Equal pairs and all19 roots give the positive zero-sum
controls. This finite computation is a control for the universal proof.

## Q36–Q40 retained failures and polynomial source repairs

Q36 adds the eight qualified scalar zeros to every prior constraint and
returns four models on155,157,184,186,51 visits per graph, no search cap reached.
Q37 computes all32 ordinary coupled norm relations independently. Each
has186 terms, degree19 and top(U+V)^19, and its complete denominator-cleared
elliptic substitution is zero. Their reduced field consequences U=V are
polynomial source consequences independent of the Q35 geometric argument.
Four relations reject the four Q36 points. Q38 nevertheless returns four
new models on180,182,71 visits each, no search cap reached.

Q39 extends the exact lower/higher pure-source polynomial resultant family
to every elliptic x-input0,2,4,6,8. Each relation has verified univariate
substitution zero and a unique top power of the lower node. Q40 retains
all earlier equations and returns one nonzero node187 per graph after9
visits, no search cap reached. Node187 is the first cumulative product of two ordinary
Miller functions. These four new models remain diagnostic survivors;
complete coupled divisors/arcs or rejecting source relations and whole
saturated-ideal equality are still open. The protocol stops this round
here; it does not silently drop the product gate or assert full saturation.
