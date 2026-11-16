# Independent fixed-sextic spectral check

Question:do the actual T0/T1/T2 models have the two-field Frobenius
fingerprint reported in external CT39,and does its smooth mutation refute
identification from a base-field curve count alone? Prior-work receipt48d283
pins CT38/39/40 at cryptanalysis PR1502 head a30c63d1. Predictions below are
hypotheses until the Rust check and a new result record exist. [pcc:unverified]

Freeze the three constant-first monic sextics T0[31,75,74,54,71,55,1],
T1[95,26,19,27,94,91,1],T2[95,93,43,39,92,17,1]. Negative control changes
only T0's x coefficient75→79. For each require gcd(F,F')=1 over F101.
Use existing isogeny_algos Zp,FpR,poly exclusively,with the independently
chosen quadratic basis F101[z]/(z²−3);the constructor checks irreducibility.
Enumerate all101 and10201 x values. Character0/±1 gives the exact affine
ordinate count1+character,plus the two monic-sextic infinity points. Check
Euler's character against an independently enumerated square-membership
table on every field element before using it. No producer Jacobian,cover
reconstruction,old map,external oracle or new field/group engine is called.
[pcc:unverified]

Acceptance:all3 actual models return N1=120,N2=10436;Newton reconstruction
returns [10201,1818,279,18,1],whose integer factors are T²+7T+101 and
T²+11T+101,and evaluation at1 gives12317. Smooth mutant must retain120
but give10314,middle218,order12256. Any mismatch fails;preserve the exact
source/stdout/diagnostic before a new version. No selected cases may be
dropped. Counter is one bounded Rust unit test in the existing standalone
crate,then run its changed complete suite with old source guards/fmt/strict
Clippy. Expected complete count21 after adding this test to accepted20.
No timing comparison,solver,cloud build or paid experiment is authorized
by this protocol. [pcc:unverified]

Source:Galbraith corrected Chapter10 Theorems10.7.1,10.7.5,Lemma10.7.6,
Definition10.7.10 and Tate Theorem10.7.13 (official cached PDF). The spectral
interpretation establishes an isogeny class,not a chosen correspondence,
geometric kernel or geometric isomorphism. Existing global degree/Picard
proofs and independent whole-map obligation remain separate,as CT38's
rational/tangent shadow explains. Next feasible Lean task formalizes the
shadow module consequences and conditional Frobenius-polynomial projector
algebra;actual varieties/Tate-module bridges remain written and open.
[pcc:unverified]
