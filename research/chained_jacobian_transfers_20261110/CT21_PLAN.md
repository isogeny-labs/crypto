# CT21: explicit7-kernel and transported Richelot continuation

Prospective implementation/proof plan after actual Phi checkpoint. No49-point
kernel enumeration,64-lift action,R1 or Psi test has run yet. Preserve all
unsupported charts and failures. Native Rust/Lean only. [pcc:unverified]

## Frozen starting rung

Consumer2450815e2a3b84687b405ef8498dae9dc2663fe1,readyPR1484,
branch research/chained-transfer-seven-glue-20261111. Full local check11
passes25native/57reference unit+two integrations,one old ignored benchmark.
Crypto SEVEN_JACOBIAN_PROOF.md/PCC917adf supplies written global geometry;
PCCb320bb binds8 partial helpers,total80. Science unsigned. Do not edit
frozen source snapshots,SEVEN_TRANSFER.md,proof-source identity pointers or
old PCC/ledger records. Continue in a fresh child consumer branch while
1484's exact-head CI/review is tracked. [pcc:unverified]

Fixed source C0=[56,0,13,0,6,0,1],target C1=[68,0,42,0,26,0,1],p101.
Phi uses E109 kernel[72,100,35,1] to(26,58),E113 kernel[59,46,8,1]
to(69,50). C1 quotient order is swapped;shifts76/14,scales21;raw scale33.
Keep lexical model choice,not external A/B ordering. Existing seven/102/five
source guards at75a737b8 remain immutable. [pcc:unverified]

## Reuse and validate common Fp6 bridge

New seven_torsion.rs can be a child of seven_glue.rs to access its checked
extension pull and existing tower Jacobian API;do not implement another
Cantor/field/elliptic engine. Reuse FpR with flat modulus
[33,46,2,18,56,12,1],generator gamma=T+s,s²=2,T³+6T²+13T+56=0.
Recover s=(gamma³+6gamma²+19gamma+68)/(3gamma²+12gamma+15),T=gamma−s.
Embed flat elements by Horner at existing tower gamma=T+s. Require exact
root/field equations,both generator inverses/canonical tails and Frobenius³
fixingT/negatings. This bridge already has an independent128-witness
receipt;reuse unchanged witnesses,check the actual adapter constants.
[pcc:unverified]

## Actual49-point degree7 kernel

Use unchanged native polynomial roots/division-polynomial/elliptic routines
in Fp6 to enumerate each selected elliptic7-kernel as O plus three abscissae
with both ordinate signs. Verify seven distinct valid points,annihilation7,
closure and actual Kohel map-to-O. The expected kernel of Phi is
P0(Ker alpha1 × Ker alpha2):Phi P0=P1 alpha kills it,and P0 is injective
on7-torsion because its kernel is2-primary. DegreePhi=49 then makes this
the whole geometric kernel. This is a prospective explicit certificate of
the existing written map,not a new degree formula. [pcc:unverified]

Compute all49 existing-tower divisor images via old extension_product_pull;
require distinctness,order7,all2401 addition cases and Frobenius closure.
Determine actual Frobenius eigenvalues from the elliptic generators instead
of assuming companion Jordan types or trusting the old abstract plane count.
Tie the result to CT18's four candidate module planes. If the chosen field
does not split a required kernel,retain that boundary;no unannounced field
change or guessed point. [pcc:unverified]

## Phi on geometric2-torsion from64 lifts

Phi P0=P1 alpha computes the action without Jacobian halving. The old graph
KerP0 has four points(O,O) and the three paired elliptic2-torsion points
with abscissae a_i+2,56/a_i+38 for the conjugates a_i ofT.
For each graph point take all four halves on each elliptic factor:
64 product points total. Halves ofO are E[2];for(ri,0),use
x=ri±sqrt((ri−rj)(ri−rk)),both ordinate signs. Verify all four actual
doublings/distinctness/completeness. They lie in Fp6 because Frobenius³
is identity modulo2 and its square is identity modulo4. [pcc:unverified]

Old P0 images must be exactly all16 geometric J0[2] classes,four preimages
each. New P1 alpha images must be independent of preimage. Require their
membership/bijection in the new16-class branch corpus,all256 additions and
Frobenius equivariance. For new branch roots,start from the actual C1 first
quotient2-torsion U obtained by inverse scale/shift from alpha2. Take a
square root,both signs and Frobenius orbit. Verify complete roots,Frobenius³
and descent into the existing cubic coefficient subfield;do not assume a
root order or branch pairing from a previous fixture. [pcc:unverified]

## New selected Richelot kernel and actual next odd hop

The original R0 cross-kernel labels are(0,4),(1,5),(2,3) in the existing
source ±Frobenius root order. Find their Phi2 images in the new15 nonzero
branch classes;these three classes select the R1 matching. Verify complete
disjoint matching,Frobenius stability,squarefree factor product=source,
nonzero determinant,smooth target and explicit valid normalization.
Generalize RichelotMap with an explicit selected-pairing constructor while
retaining every old check/old fixture assertion. New cubic coefficients
can live in the OLD cubic field;its modulus need not be the new curve's
elliptic quotient polynomial. [pcc:unverified]

If the selected R1 requires an unsupported chart,retain the exact boundary
and test the other predeclared gluing model as a separately documented
alternative. Do not silently retwist,change lexical acceptance,remove a
validation assertion or substitute the previous dual as a new destination.
Keep actual Phi68 positive results even if the next chart is unsupported.
[pcc:unverified]

For admitted R1,put C=R1 Phi R0Hat:T0→T1. Its kernel contains T0[2]
because R0Hat(T0[2])=KerR0 and Phi maps that kernel onto KerR1. Factor
C=Psi[2] globally. Polarization/degree gives type(7,7),degree49 and both
global dual norms[7]. Rational evaluation can half input by6159 mod12317
before the composite;the reverse uses the reversed adjoint diagram.
Check both complete12317-class domains,bijections/paired additions/dual[7],
all109 original source scalars and actual coefficient descent before
accepting the next target hop. [pcc:unverified]

The original source chain E109→J0→T0→T1 should reverse to[28]P under
N0 R0Hat PsiHat;inverse74 mod109 recovers P. New target coordinates
N1 R1Hat should equal([4]alpha1(P),0). Show Psi R0=R1 Phi from the
global diagrams and Hom cancellation. Keep those direct elliptic/source
controls and distinguish the degree49 hop from a scalar/backtrack.
[pcc:unverified]

For unpolarizedFp nonisomorphism,check cross-kernel intersections with
embedded elliptic2-torsion are trivial;then R0/R1 retain elliptic embeddings.
The trace-type uniqueness/j argument in the Phi report applies. Do not
infer a Jacobian distinction solely from differing curve models or j.
[pcc:unverified]

## Publication and runtime

Register native receipts/proofs and preserve all failures before citing new
results. Produce source-linked report,diagram,inspected PDF and faithful
feasible Lean helpers;keep actual geometry's missing formal bridge explicit.
Record all source hashes/commands/head refs. The existing full local Phi
matrix took629.56seconds as a correctness receipt,not a benchmark. Assess
new whole-chain CI scope/timeout before publishing;no new expensive PR
trigger,no ignored/quarantined test or weakened full-domain acceptance.
Parallel immutable correctness contexts are allowed,not a speed claim.
Follow actual final-head CI/review/merge requirements. S3/livebudget still
blocks experiment/solver/benchmark/paid work;root ledger/archive and KB/
vault integration remain separately unresolved. [pcc:unverified]
