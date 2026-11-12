#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(justify:true)
= Prime-field descent through an odd coefficient field
2026-11-12 · actual R0/R1/R2 · scientific standing unsigned

The accepted Richelot maps admit a second global F101 descent proof.
Their correspondence coefficients lie in F101³ even though evaluating
geometric points uses F101⁶. Stable scheme kernels and polarized target
automorphisms±1 leave a sign ambiguity. The odd coefficient degree rules
out the negative sign in the torsion-free Hom group. Existing rational
odd-witness proofs and all original source controls remain unchanged.
Written proof ODD_FIELD_DESCENT_PROOF.md/PCC6b3ff6. [pcc:unverified]

#image("descent.svg",width:100%)

== Exact theorem and geometric bridge
Let A,B be principally polarized abelian varieties over Fq and let a
nonzero isogeny R:A→B be defined over Fq^m for odd m. Assume its finite
scheme kernel is Galois stable, its polarization norm is an integer n,
and every geometric origin-preserving polarized automorphism of B is±1.
Then R and its polarized adjoint descend to Fq. The field degree and
automorphism assumptions are essential parts of this statement.
[pcc:unverified]

Conjugation gives another quotient with the same scheme kernel, so
sigma(R)=aR. Equal polarization pullbacks make a polarized: pullback
by an isogeny is injective on polarization homomorphisms. Thus a=±1.
If sigma(R)=-R, odd iteration gives sigma^m(R)=-R; the coefficient
field also gives sigma^m(R)=R. Hence2R=0, contradicting nonzero R in
torsion-free Hom. Finite Galois descent then supplies the base-field
morphism. This sigma is the Galois action on homomorphisms, not an
inseparable Frobenius morphism, and point groups are not torsion-free.
[pcc:unverified]

== Actual instantiation
The cubic constructor forms the quadratic factors and bracket/determinant
duals in F101[T]/(T³+6T²+13T+56), then embeds with zero second tower
component. Sources, targets and ordinate scales22/47/50 lie in F101.
The retained complete branch certificates check all120 triple-determined
PGL2 candidates for each T0/T1/T2; hyperelliptic theory and strong Torelli
give the required polarized Aut±1. Apply m3,n2. The separate HLP-map
descent arguments are unchanged; no Aut±1 assumption is added to C1/C2.
[pcc:unverified]

#pagebreak()
= Why the odd-degree hypothesis matters

== A concrete even-degree counterexample
Take E:y²=x³+x+46 and E':y²=x³+4x+65 over F101. With s²=2,
phi(x,y)=(2x,2s y) is an origin-preserving isomorphism over F101².
Its kernel is trivial and stable, and the generic j12 target has
polarized Aut±1. But sigma(s)=-s gives sigma(phi)=-phi and
sigma²(phi)=phi. This is a global equality of displayed maps.
Unequal rational point counts109/95 exclude any F101 isomorphism.
[pcc:unverified]

Existing-engine native control01/PCCcd2506 counts both groups, checks all109
source images and all109² additions, and finds negative Frobenius and
nonrationality for every108 nonzero image. It uses the existing flat field
Fp[s]/(s²-2) and existing elliptic operations; no new arithmetic engine
or timing benchmark. This preserves an actual counterexample to dropping
oddness rather than inferring an obstruction from failed search.
[pcc:unverified]

== Formal proof map and limits
OddFieldDescent.odd_sign_descent and cubic_sign_descent prove the faithful
module implication with explicit negation, fixed odd iterate, nonzero and
no2-torsion hypotheses, PCCfdec11/94f071. Three iterate lemmas have
individual records39f845/00162f/9c5016. The generic ring identity
quadratic_twist_curve/PCC8088e8 checks the coordinate scaling s²=d.
All six written statements have bound theorem records and local pass
verdicts; scientific standing remains unsigned. [pcc:unverified]

The final Lean4.35.0 gate audits six written theorems plus three generated
iterate equations, zero trusted native. The project source census is131
written declarations; generated equations are excluded. Complete standalone
correctness02 passes9tests and strictClippy. PCCb33222 pins these receipts.
CI now includes22 named Lean modules and the ninth standalone test;
their next exact-head CI remains a separate obligation. [pcc:unverified]

Actual fields, abelian varieties, Galois action, regular quotient,
polarization, Hom torsion-freeness, Torelli and finite Galois descent
remain written bridges. The formalization is partial; it does not claim
constructed Lean geometry. This result changes no curve, normalization,
catalog, DLP algorithm or measured cost. Existing transfer graph sources
remain current because no map or edge changed. [pcc:unverified]

== Sources and reproducibility
Primary descent source: Milne, Abelian Varieties v2(2008), ChapterIII
Proposition1.13(a), finite-Galois equivalence for quasi-projective varieties;
full faithfulness descends invariant morphisms. Checked PDF SHA256
`f5ca4e63e5092a4b102daad1470e4cbed5fe8f82115e3a28c8881e3f67f6aaef`.
Strong Torelli: Milne, Jacobian Varieties(2021), Theorem12.1, checked in
the retained chain proof. Actual coefficients and branch certificates are
the existing consumer sources, with full citations in the written proof.
[pcc:unverified]

Reproduce with `lean/OddFieldDescent.lean`, the ordinary proof gate and
`successor-preflight/src/odd_descent.rs`. Receipts and exact sources are in
`evidence/odd-descent-*` and `evidence/even-field-negative-*`.
KB/atlas access remains unavailable; no novelty or absence claim is made.
