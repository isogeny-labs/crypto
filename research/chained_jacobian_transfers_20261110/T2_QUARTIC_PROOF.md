# The two exact quartic covers of T2

Scientific standing unsigned. This extends the fixed-model argument of
QUARTIC_PULLBACK_PROOF.md/4934b4 to the accepted R2 target. Old
Richelot/Picard/polarization/descent constructions remain dependencies;
finite certificates do not replace those constructions. [pcc:unverified]

Over F101 put F2=[95,93,43,39,92,17,1], low coefficient first, and
T2:y²=F2(x). The maps are f_j=(N_j/Q_j²,y M_j/Q_j³):

| Target short model | N | Q | M | f(infinity+) |
|---|---|---|---|---|
| E109''=(59,27) | [97,93,7,0,97] | [55,0,1] | [71,49,20,63] | (97,63) |
| E113''=(23,7) | [74,0,81,42,64] | [23,0,1] | [37,9,9,35] | (64,35) |

Construction25/412aba uses exactly H2,j=N2,j R2Hat from the retained
SuccessorChain. Its values on degree-one balanced classes are centred by
q=[55 or57]H2,j([infinity+-infinity-]). The old split norm coordinates
and actual R2 adjoint fix the target model and ordinate sign. The shared
existing interpolation/reconstruction body returns the coefficients above.
The original three T0/T1 constructor tests pass after the extraction.
[pcc:unverified]

The independent existing flat-field replay28/2fee3f checks that F2 is
irreducible, smooth and monic, each elliptic target is smooth, and

    F2 M² = N³ + a N Q⁴ + b Q⁶.

It checks gcd(N,Q)=gcd(F2,Q)=gcd(Q,Q')=1, nonzero derivative of N/Q²,
and x-degree4. Thus each rational function defines a nonconstant separable
degree-four map on the completed smooth curves. Both infinity values are
the displayed finite q and -q. All118 finite rational source points occur
exactly once in each frozen certificate; E109'' has118 finite images and
E113'' has116 finite images plus the two origins at(49,12),(49,89).
Wrong-codomain controls reject. [pcc:unverified]

Each cover has four simple geometric pole branches. For E109'', these
form one Frobenius orbit of length4: the independent checker uses
F101[z]/(z⁴-2), verifies x^101 != x, x^(101²)=x and y^(101²)=-y != y
for all four points, and verifies fourth Frobenius fixes them. They lie
in F101^4 and do not lie in F101^6, whose intersection with F101^4 is
F101². The failed degree-six assumption in replay27/bf2591 remains
preserved. For E113'', two poles are rational and the complete four-pole
fibre is checked in the declared F101^6 representation. These are pole
splitting fields, not fields of definition of the maps. [pcc:unverified]

The old global R2 adjoint identity and split norm/pullback identity give
H2,j H2,j†=[4]. The Abel–Jacobi curve map for H2,j therefore has degree4,
by the same Picard duality argument as4934b4. The old centred map is odd
and has the same q. Both it and f_j have degree4. Milne AV Corollary5.3
(the cube identity for arbitrary regular maps) bounds the degree of their
difference by16. The118 and116 distinct finite agreements exceed16;
a nonconstant difference cannot have that many points in one fibre,
counting total degree. Hence the maps agree globally with the old centred
maps, with the old-H value provenance and global degree facts explicit.
It follows that f_j*=H2,j and f_j^*=R2 P2,j. [pcc:unverified]

The finite-algebra pullback and norm formulas from4934b4 apply unchanged.
For rational finite P=(X,Y), the odd group orders109/113 exclude Y=0.
In F101[x]/(N-XQ²), Q and M are units even on repeated fibres, and the
ordinate is V=YQ³/M. Before the existing balanced reduction, the finite
fibre has plus count -1, corrected for actual infinity multiplicities at
P=±q. The denominator divisor is f^*(O)-2(infinity++infinity-).
No pole root, branch root or individual support point is selected by the
prime-field formula. The geometric pole field of degree4 therefore does
not force extension-field arithmetic in this evaluator. Runtime acceptance
of T2 direct norms/pullbacks belongs to separate receipts. [pcc:unverified]

Sources checked: SUCCESSOR_CHAIN_PROOF_V2.md and SUCCESSOR_NORMS_PROOF.md
for the exact old R2 and charts;4934b4 and6afa9c for the Picard and
degree-difference argument; Milne AVv2 Cor5.3 and Galbraith corrected
Chapter8 for degree/fibre/extension facts. KB/ecdb unavailable. No new
field-family, novelty or measured computational advantage claim.
[pcc:unverified]
