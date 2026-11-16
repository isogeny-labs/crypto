#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(justify:true)
= A whole chained transfer through outer norms and pullbacks
2026-11-12 · fixed F101 models · scientific standing unsigned

The two-hop Jacobian transfer has a prime-field evaluator that constructs
only its outer norm and pullback stages. Native33/PCC8e0523 checks all12,317
concrete outputs against oldsecond after the accepted purefirst and
separately staged puresecond after purefirst. Bijection, fixed-partner sums,
negations, identity, model and wrong-quarter controls pass. Every109/113
original source scalar, direct final endpoint, reverse196 and recovery104/64
also passes. The first-hop comparison depends on accepted72e1d8/global78176c.
This is local execution evidence with unsigned standing. [pcc:unverified]

#image("outer.svg",width:100%)

== The cleared global factorization
For the old product pulls and norms, let K_i=R_iP_i and H_i=N_iR_iHat.
Both H_iK_i and K_iH_i are multiplication by4. Each admitted hop has
4Psi_i=K_i phi_i H_(i-1). For F_n=Psi_n...Psi_1 and Phi_n=phi_n...phi_1,
the middle relation gives16F_n=4K_nPhi_nH0. Cancellation in the torsion-free
Hom group yields4F_n=K_nPhi_nH0. Geometric point groups are not assumed
torsion-free. For two hops the identity is4Psi2Psi1=K2 beta alpha H0.
CHAIN_COMPRESSION_PROOF_V2.md/PCC89e13d gives the induction. [pcc:unverified]

On the fixed rational groups, inverse4 is82 modulo109 and85 modulo113.
Therefore evaluate the two T0 norms, apply that quarter once, evaluate
alpha then beta, pull back toT2 and add. No middle T1 class is needed in
this expression. Original embedded elliptic inputs further satisfy
F_nK0=K_nPhi_n. These direct routes belong in staged-chain cold-cost
comparisons; no timing advantage is inferred from the stage reduction.
The geometric composite degree remains2401 and its adjoint norm49.
[pcc:unverified]

#table(columns:2,stroke:0.4pt,
 [Accepted local component],[Scope],
 [First forward72e1d8/adjointdcbb58],[Every12,317 old output;both norm7 compositions for adjoint],
 [T2 norms/pulls e383f2],[Every12,317 oldH2 output;all109/113 oldR2P2 and24,650 sums],
 [Second forward8e0523],[Every12,317 oldsecond output,selectedbeta coefficients and group controls],
 [Whole two-hop8e0523],[Every12,317 oldsecond-after-purefirst/stagedpure output and source recovery],
)

== Two faithful formal consequences
ChainCompression.two_hop_double_cleared/1b3e8b gives the16-versus4
module identity with explicit cleared-map, middle norm4 and linearity
hypotheses. rational_two_hop_factorization/167fc6 adds explicit injectivity
of multiplication by4 on the destination rational domain. Both statements
are individually bound and locally audited with zero trusted native proofs.
The written census is138helpers across26modules; actual varieties, Picard,
quotient geometry and global Hom cancellation remain written. [pcc:unverified]

#pagebreak()
= The three fixed Jacobians are geometrically different
The written argument b05286 extends the older base-field-only distinction
using CT37's standard ordinary endomorphism-field obstruction and the actual
elliptic embedding certificates. Native34/5c1a1b independently checks the
six numerical inputs with existing Rust arithmetic. It computes no
endomorphism ring or Jacobian isomorphism. [pcc:unverified]

#table(columns:3,stroke:0.4pt,
 [Family],[Exact models at stages0,1,2],[Input certificate],
 [E],[(1,46);(26,58);(59,27)],[order109;trace-7;j12,87,47],
 [F],[(44,21);(69,50);(23,7)],[order113;trace-11;j58,92,89],
)

Both families are ordinary. Their geometric rational endomorphism algebras
are Q(sqrt(-355)) and Q(sqrt(-283)), since Frobenius already spans the
ordinary quadratic algebra. The fields differ:355/283 has odd5-adic
valuation. Any nonzero cross-family geometric homomorphism would be an
isogeny and identify these algebras, a contradiction. The obstruction
persists through the selected isogenies and every finite field extension.
Galbraith Chapter9 Theorems9.9.1/9.11.2/9.10.3 and Sections9.6/9.7
supply the classification, Frobenius and isogeny facts. [pcc:unverified]

#image("geometry.svg",width:100%)

The old geometric2-torsion controls show that the actual elliptic pulls
embed and avoid each Richelot kernel. Thus K_(i,E) embeds the actual E_i.
Its image is the identity component of the complementary norm kernel:
that kernel is one-dimensional and contains the image. Cross-Hom0 makes
this E-type connected subvariety unique. A geometric unpolarized
isomorphism of any two surfaces would restrict to E_i isomorphic to E_j,
contradicting their distinct j-invariants12,87,47. The actual Picard,
finite-etale kernel and embedding constructions remain explicit old
dependencies;geometry is written,not fully Lean formalized. [pcc:unverified]

The old twist counterexample cd2506 is retained:orders109 and95 have
opposite traces,one geometric CM field and geometrically isomorphic
elliptic curves. Different rational Frobenius polynomials alone therefore
would not establish this conclusion. ExternalCT37 at cryptanalysis9d100
is credited prior work;its Python oracle was not executed here.
[pcc:unverified]

== Current checkpoints and next acceptance
Changed standalone35/829d69 passes20tests,strictClippy/fmt and original
reference/legacy digests. Root971's separate CI scope is19standalone/
136written/25modules,initiallyrunning43177796116,nextcheck08:39UTC.
The new20/138/26 scope needs its own source-changing checkpoint andCI.
Ready consumer1506 atb297 has full25native/90reference+five76871d,
final2page6a9aa0 and separate ownCI. Actual next adjoint child full03
runs on9frozen source02 files,expected92reference+five;no completion is
asserted here. Isolated second-adjoint/compressed-reverse acceptance is
also pending. [pcc:unverified]

S3/profile/livebudget admission is unavailable, so matched cold costs remain
open. KB/ecdb unavailable;atlas vault locked. Sources and proof boundaries
are in GEOMETRIC_JACOBIAN_DISTINCTION.md/b05286 and the named global
norm/pullback proofs. No novelty, timing or solver advantage is claimed.
[pcc:unverified]
