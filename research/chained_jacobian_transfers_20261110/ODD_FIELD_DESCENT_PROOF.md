# Odd coefficient-field descent for the actual Richelot maps

Scientific standing unsigned. This gives a second global descent argument
for the accepted R0/R1/R2. The frozen rational odd-witness arguments and
all native witness controls remain valid and unchanged. The new argument
uses the actual cubic COEFFICIENT field,not the degree6 evaluation field.
No new target,normalization,backend or measured advantage is claimed.
[pcc:unverified]

## Precise finite-field statement

Let A and B be principally polarized abelian varieties over k=Fq. Let
R:A→B be a nonzero isogeny over L=Fq^m,with m odd. Assume its finite
scheme kernel is Galois stable over k,that R pulls the target polarization
back to n times the source polarization for an integer n,and that every
geometric origin-preserving polarized automorphism of B is +1 or-1.
Then R is defined over k,and its polarized adjoint is also defined over k.
This theorem retains the automorphism and coefficient-field hypotheses.
[pcc:unverified]

Let sigma generate Gal(L/k). It acts on Hom(Abar,Bbar) by conjugating
coefficients using the canonical k-descent data of A and B. This is the
GALOIS action on homomorphisms;it is not an identification with an
inseparable Frobenius morphism of a variety. It commutes with negation,
and sigma^m(R)=R because the displayed map is defined over L.
[pcc:unverified]

The maps sigma(R) and R have the same finite scheme kernel. The abelian
quotient universal property gives sigma(R)=a R for an origin-preserving
automorphism a of B. Both pull the target polarization back to n times
the source polarization,so a preserves it. Indeed pullback by an isogeny
is injective on polarization homomorphisms:if Rdual b R=0,surjectivity of
R and the finite kernel of Rdual force the connected image of b to be
zero. Thus a=+1 or-1. [pcc:unverified]

If sigma(R)=-R,negation compatibility gives sigma^j(R)=(-1)^j R.
Since m is odd,sigma^m(R)=-R,while its coefficients give sigma^m(R)=R.
Therefore2R=0. Hom groups of abelian varieties are torsion-free:a killed
homomorphism has connected reduced image in a finite group scheme,and
hence zero image. This contradicts R being a nonzero isogeny. Consequently
sigma(R)=R. Cancellation takes place in HOM,with no torsion-free claim
about elliptic or Jacobian point groups. [pcc:unverified]

Finite Galois descent now yields a k-morphism. A primary statement is
Milne,Abelian Varieties v2(2008),ChapterIII Proposition1.13(a):base change
is an equivalence between quasi-projective k-varieties and the corresponding
L-varieties with descent data. Its full faithfulness applies to this
invariant morphism between the canonical base changes of A and B.
Abelian varieties are projective. Polarizations and duality commute with
base change,so the adjoint descends as well. The checked PDF SHA256 is
f5ca4e63e5092a4b102daad1470e4cbed5fe8f82115e3a28c8881e3f67f6aaef.
[pcc:unverified]

## Actual coefficient-field and automorphism hypotheses

For all three accepted maps,the source/target equations and normalization
scalars are in F101. The old cubic field is
F101[T]/(T³+6T²+13T+56). The selected-pairing constructor forms the
quadratic factors and their bracket/determinant duals in THIS cubic field,
then embeds them into the existing quadratic tower with zero second
component. Thus every correspondence coefficient lies in F101³.
Completing that correspondence and inducing its Picard homomorphism
preserves this field of definition. Degree6 intermediate input points do
not change the coefficient field. [pcc:unverified]

The accepted raw target products descend to F101 and the ordinate scales
are22,47 and50 for R0,R1,R2. These are already F101 scalars. The actual
stable branch matchings give the scheme-kernel invariance;characteristic101
is different from2,so native geometric two-torsion stability suffices here.
Earlier nonsquare-leading unsupported charts are not accepted by this
instantiation. [pcc:unverified]

For each displayed target T0/T1/T2,the complete branch certificate checks
all120 transformations determined by an ordered root triple and finds
only the identity preserving all six roots. Every geometric branch
automorphism has coefficients in that root field,because three root images
determine it. Hyperelliptic theory and strong Torelli therefore give
Aut(Ti,lambdai)={+1,-1}. The primary Torelli statement is Milne,Jacobian
Varieties(2021),Theorem12.1,whose checked PDF hash is retained in the
frozen chain proof. These are the previously retained full certificates,
not a new rational-only automorphism search. [pcc:unverified]

Apply the theorem with m=3 and n=2. Each actual Ri is therefore defined
over F101 without using a numerical nonzero rational-image witness to
choose its sign. Their adjoints descend by duality. The HLP maps Phi1/Phi2
keep their separate graph-quotient descent proof;this argument instantiates
the automorphism hypothesis on the Richelot TARGETS Ti. All original
source109/113 and witness tests remain acceptance controls.
[pcc:unverified]

## A real even-degree counterexample

Over F101 take E:y²=x³+x+46 and its quadratic twist
E':y²=x³+4x+65. Let s²=2 in F101². The scaling

    phi(x,y)=(2x,2s*y)

is an origin-preserving isomorphism over F101²,with inverse scaling.
The generic identity for a twist with s²=d maps
(x,y) to(d*x,s³*y),with coefficients(d²*a,d³*b). The Lean declaration
quadratic_twist_curve checks that identity over arbitrary commutative rings.
Here the source discriminant factor is71≠0,and both j-invariants are12,
different from0 and1728 modulo101. Their geometric polarized automorphisms
are ±1. Equivalently,the short-model automorphism conditions u⁴=u⁶=1
force u²=1 because both coefficients are nonzero. [pcc:unverified]

The kernel is trivial and stable. But sigma(s)=-s,so sigma(phi)=-phi and
sigma²(phi)=phi. This is a global equality of the displayed coordinate
maps. Native existing-engine control01/PCCcd2506 independently counts
E(F101)=109,E'(F101)=95,checks all109² additions under the scaling,and
finds negative Frobenius/nonrationality for every108 nonzero source image.
The unequal rational point counts also preclude an F101 isomorphism.
Thus stable kernel and target Aut±1 permit the negative sign over an EVEN
coefficient extension. This is a descent counterexample,not a new attack.
[pcc:unverified]

## Formal proof map and remaining bridges

OddFieldDescent.odd_sign_descent/PCCfdec11 and cubic_sign_descent/
PCC94f071 prove the faithful module implication with explicit negation,
odd-iterate,nonzero and no2-torsion hypotheses. The intended module is
Hom(Abar,Bbar). The three iterate lemmas are PCC39f845/00162f/9c5016;
the twist identity is PCC8088e8. Six WRITTEN declarations plus three
generated equation lemmas pass the Lean4.35.0 kernel/axiom gate,zero
trusted native. Every written statement has its own bound theorem record.
Local pass verdicts do not supply signed scientific standing.
[pcc:unverified]

Actual finite fields,abelian varieties,Galois action,regular quotient,
polarization,Hom torsion-freeness,Torelli and finite Galois descent are
the explicit WRITTEN bridges above. The formalization does not silently
replace them with constructed objects. Existing source snapshots,full
native claims and old proofs remain immutable. KB/atlas access is absent;
no novelty,large-prime,algorithmic cost or signed-validation claim is made.
[pcc:unverified]
