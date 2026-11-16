# Identifying the two plane quartics and their descent signs

Post-run proof supplement. The frozen executable reports two smooth
Gamma00-jet candidates and jacobian_identified=false. That flag records the
scope of the numerical gate alone and is retained. This supplement supplies
the geometric identification and descent argument; it does not provide an
explicit divisor-map formula or independent validation signature.

## 1. Geometry and local coordinates already certified
PROOF.md shows each geometric quotient A=J/V is an indecomposable,
nonhyperelliptic genus3Jacobian. Its eight descended sections form the full
H0(A,2M) basis. The sourcekernel degree27 is prime to127, so the quotient
isogeny is étale. The registered rational2torsion T has

    U0=x³+15x²+90x+109, V0=0.

Since gcd(U0,f/U0)=1, the Mumford equations admit the three coefficients of
V=t0+t1x+t2x² as formal parameters. Their linearized equations solve uniquely
for U, and the printed U2/U4 formula is that solution through degree4.
The exact local-curve identity and scalar evaluation checks pass. Translation
by S=psi(T), and the étale differential of psi, identify this with a formal
parameter system at0on A, with an invertible tangent transformation overF127.

The unframed sections used in the run are pullbacks of H0(A,2M) in a regular
nonzero local frame (source T is off the source theta divisor). Any change
of that frame multiplies a section vanishing to order4by a unit, which
only rescales its leading quartic by the unit's constant term.

Each constant-plus-quadratic jetmatrix has rank7. Alloddjetcoefficients
vanish. Its unique kernel section therefore has orderatleast4, and its
nonzero degree4jet is the printed smooth ternaryquartic Q_i. This is the
translated Gamma00line, independent of choices up to scalar/tangentGL3.

## 2. Why the order-four jet is the canonical curve in characteristic127
Over characteristic0, the classical Fay statement identifies Gamma00 with
second-order theta sections containing C-C. It is stated as equivalence
(1.2), with the associated tangent-space formulation(1.3), in Pauly–Previato,
Singularities of2theta-divisors in the Jacobian,
https://math.univ-cotedazur.fr/u/pauly/singularities.pdf, introduction.
We use only the inclusion of C-C, not an unproved finite-characteristic
extension of the analytic theorem.

The tangent directions to the difference surface at0include the canonical
curve: along p-q with q tending to p, its first direction is evaluation
of holomorphic differentials at p, i.e. the canonical point. Hence a
nonzero leading quartic of a Gamma00section vanishes on that canonical
planequartic. Both have degree4and the latter is smooth/irreducible, so
they are scalar multiples. The statement is algebraic after this inclusion
and descends from complex to arbitrary characteristic0field by choosing a
finitely generated field of definition.

For completeness, specialize this fact instead of assuming it in char127.
Over an algebraic closure, write the unknown nonhyperelliptic curve C in
its smooth planequartic canonical model. Lift its coefficients to the Witt
DVR; smoothness remains, giving a smooth nonhyperelliptic family. Lift the
chosen symmetric theta line bundle as well: relative theta characteristics
and the required2torsion translate lift étale-locally because2is invertible.
The eight sections of its square form a free rank8module and commute with
basechange (ample higher-cohomology vanishing). Symmetry killsoddjets.

Lift the regular formalparameters at0and their localframe. The seven
constant/quadratic conditions form a7x8matrix over the DVR. Its special
fiber has rank7, witnessed by the registered matrix; a7minor is therefore
a unit. Its kernel is free rank1and its generator specializes to the
registered Gamma00section. The computed degree4jet is nonzero, so the
lifted leading quartic has at least one unit coefficient. On the generic
fiber it equals a scalar times the canonicalquartic of the lifted curve,
by the characteristic0argument. That scalar is a unit: the canonical
quartic is primitive with smooth specialfiber, and the leading quartic
has a nonzero reduction. Reducing proves that Q_i is the canonical model
of C in the chosen tangentcoordinates. No assumption that the target
isogeny itself lifts is required; only its special-fiber tangent coordinate
change, which is invertible and can be lifted.

Thus each printed Q_i over F127 is GEOMETRICALLY the curve whose polarized
Jacobian is A_i. This uses the exact rank7/nonzerojet and prior geometric
classification; smoothness/pointcounts alone would not establish it.

## 3. Functorial descent leaves only the Serre sign
The canonical curve recovered in projective tangent space is functorial
in the ppav and rationalpoint S. The tangent identification from the
source isogeny is over F127. Therefore Q_i is the canonical Torelli descent
curve for A_i, not an arbitrary geometrically isomorphic twist.

Here is the cocycle reason. Choose a geometric polarized isomorphism
Jac(Q_i)->A_i matching the induced canonical-curve tangent embedding.
Galois fixes the projective identification. Its descent cocycle therefore
acts trivially on the projective canonical curve. Strong Torelli for a
nonhyperelliptic curve says the ppav automorphisms are plus/minus curve
automorphisms. A curve automorphism acting projectively trivially on its
canonical embedding is identity. Consequently the cocycle is in{+1,-1}.

This is the quadratic Serre alternative, valid over any field in
Lachaud–Ritzenthaler–Zykin, Jacobians among Abelian threefolds, Theorem1.1.1,
https://arxiv.org/html/0802.4017 (attributing the precise theorem to Serre).
It is essential that the recovered curve uses this functorial descent;
geometric isomorphism and pointcount alone would leave other curve twists.

The source/quotients have L1=1, hence Frobenius trace-1. The exact registered
projective pointcounts give Q0:127points(trace+1), Q1:129points(trace-1).
Over F127 there are exactly two quadraticcharacters. The nontrivial[-1]
twist changes the Frobenius trace sign, and trace-1is nonzero. Thus:

- A1 is F127-isomorphic, as a ppav, to Jac(Q1). This is the same-field route.
- A0 is the nontrivial[-1]quadratic twist of Jac(Q0). Its Serre obstruction
  is nontrivial; it is not a Jacobian over F127. The curve Q0is retained,
  rather than silently identifying its rational Jacobian group with A0.

No modular-form numerical value or explicit polarized isomorphism formula
is asserted. The sign conclusion uses the functorial reconstruction proof
plus strong Torelli and the nonzero trace, not just the observed counts.

## 4. Derived arithmetic and exact model
In ascending T powers the quotient Weil polynomial is the prior certified
L=[1,1,-43,-1224,-5461,16129,2048383]. The above proof derives
L_Q1(T)=L(T) and L_Q0(T)=L(-T). Therefore the derived Jacobian orders are
2057786and2027974respectively. These are derived from the identification,
not new direct extension pointcounts. The targetprime1028893divides the
first; no target transport is yet implemented.

For Q1, the verified smooth same-field plane equation is, modulo127:

119x^4+44x^3y+65x^3z+5x^2y^2+107x^2yz+58x^2z^2
 +100xy^3+122xy^2z+90xyz^2+32xz^3
 +109y^4+98y^3z+17y^2z^2+119yz^3+z^4=0.

Q0and both normalized coefficientarrays are retained in RUN.txt. The
reconstruction, formal-evaluation and Buchberger implementation shares the
prior field/Cantor stack, so this is not independent external validation.
The next construction is the actual divisor transport from the source
Jacobian to Jac(Q1), followed by a destination solver and complete cost
comparison. Full mathematical10xgenus2/3/4scope remains active.

Original strong-Torelli source subsequently checked directly: Serre's
appendix to Lauter, Geometric methods for improving the upper bounds,
https://arxiv.org/html/math/0104247, Theorems2/3/5. They give uniqueness
of the signed curve isomorphism, Aut(J,polarization)={±1}xAut(C) in the
nonhyperelliptic case, and the quadratic descent statement. The appendix
explicitly treats perfect fields; F127is perfect. This supports the precise
cocycle step above without importing the complex modular-form sections
of LRZ into positive characteristic.
