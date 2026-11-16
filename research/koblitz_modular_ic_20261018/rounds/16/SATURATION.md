# Actual source-bound quartic separator; saturation still incomplete

Q19 starts with the original DAG gates15=Constant1,16=15²,17=0*16,
18=17²,19=0*18, together with12=1². The old Q17 boundary has z19=z12=1.
For an arc with h=t^e the unit gates imply v(input0)=2e/3 and
v(input1)=e/2, so6 divides e. A base elliptic formal arc at e=6 passes the
first25/26 source gates to precision96. It is not a full quotient arc.

Q20 propagates forced valuations through the actual gates. All four old
models require value-1/3 at node80, contradicting nonnegative projective
coordinate valuations. The retained15-assignment witness uses no
coefficient-Frobenius gate. Coefficient Frobenius need not preserve an
arbitrary geometric arc's valuation: across the360 embeddings only a
uniformly established value may be shifted. The final witness avoids this
issue entirely. No blanket valuation-preservation assumption is used.

Q21 eliminates the valuation hypothesis and gives a polynomial certificate.
Set x=input0,U=z19,V=z80. Actual source expansion gives

    F1=x³-h²U,
    F2=c3*x⁴+c2*h*x³+c1*h²*x²+c0*h³*x-h³V,
    h²F1 in I, h²F2 in I.

The actual coefficients are nodes69,72,75,78, in descending order. They
are all nonzero. Native output includes the normalized source expansions,
exact depth2 derivations and literal gate bindings. Put

    B=c3*U+c0*h, Q=c2*U-V, A=c1,
    R=U*B³+h*Q³+h²*U²*A³-3hUABQ.

The universal integer residual identity h⁴R=F2*Adj-F1*Cof is proved in
Lean, with explicit adjugate and cofactor in KoblitzRound16. Consequently
h⁶R belongs to I, so R belongs to I:h^infinity. At h=0,U=1,
R=c3³!=0. All four actual old Q17 points are therefore excluded from the
projective closure of the affine source graph, under the exported native
field/circuit interpretation. This is stronger than a first-jet obstruction:
it excludes arbitrary ramification and directly certifies ideal membership.

In characteristic2 the scalar quartic is obtained by expanding in the
actual field basis. Native nonzero coefficient bits[1,3,1,1] separate the
four old models; the generated Lean certificates prove the finite scalar
specializations and source gate bindings. The interpretation of the native
field basis and the whole Weil-restriction circuit remains a written bridge.

Q22 adds only the valid radical boundary condition node19=0, retaining
all original and Q13–Q17 constraints, including every expanded coefficient
quadric. It finds checked models in four search visits per graph:

| graphs | nonzero source nodes at h=0 |
|---|---|
|0,1|26,28,31,34,35|
|2,3|25,27,30,33,34|

Generated Lean checks each finite original/repair model. These survivors
are not declared false boundary points. They could represent actual closure
points; the next step is another source-bound separator or a genuine arc,
followed by ideal equality. Node19 is not asserted to lie in the saturated
ideal itself: its vanishing here follows from the radical boundary equation
c3³U⁴=0. A nonzero survivor alone does not prove incomplete saturation.

No complete saturated ideal, full boundary atlas or proper polarized quotient
embedding has yet been computed.
