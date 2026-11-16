# Actual primitive-four T20 kernel,raw equation and prime-field quotient

Full39/84454e accepts the geometry phase of protocola2c024:all626 native
inputs unchanged,all old tests/guards retained,6141-byte geometry JSON
identical to selected296872,and the old24475-byte kernel JSON identical
to9f7c80. This note establishes the actual global quotient and its raw
target normalization. The new proper whole-Mumford evaluator is phaseB
and is not supplied by this acceptance. Scientific standing is unsigned.
[pcc:unverified]

## Actual subgroups and quotient compatibility

Let A=E2×F0 over F101 with E2=(59,27),F0=(44,21),and let Q20 be the
actual transported original anti-four graph (beta alpha,id)(Q0). The
native test constructs it from the original sixteen geometric points
and the two existing normalized seven maps;it checks16 distinct points,
all256 sums and all256 anti-isometry pairing identities. Since49 is1
modulo4,the transported first-factor pairing agrees with its original
pairing. No candidate target coefficients enter this graph construction.
[pcc:unverified]

Put Gamma20=[2]Q20. It has4 points and is the actual two-graph used to
construct C20:y²=18x⁶+75x⁴+45x²+40. The old optimal-cover identities
give P20=i_E2+i_F0:A→Jac(C20),with kernel Gamma20 and degree4. Every
actual Gamma20 point is again checked to map to zero. Therefore

    K20=P20(Q20) has16/4=4 points,
    Jac(C20)/K20 ≅ A/Q20.

Native data represents all4 image points as valid C20 Mumford classes,
checks annihilation by2,all16 sums and geometric Frobenius. Identity is
fixed and the other three points form one Frobenius three-cycle.
The cover/Picard quotient statements are the written geometric inputs
128eb1/afd164;the finite points are computed,not asserted by labels.
[pcc:unverified]

The six actual C20 branch roots are permuted as3+3. For a matching pair
(a,b),the class of W_a-W_b equals the sum of their two classes relative
to the chosen chart Weierstrass point,because those classes are killed
by2. The test enumerates every15 perfect matching and finds the unique
one whose three nonzero pair classes equal K20\{0}. Thus K20 is the
actual maximal isotropic two-subgroup for that quadratic splitting,by
the standard even-subset description of genus-two two-torsion. This is
the hypothesis used by Bruin–Doerksen §4,not inferred from point counts.
[pcc:unverified]

Exactly three matchings are Frobenius stable. Their monic determinants
are0,34,67;the even opposite-root pairing is singular. The actual K20
matching is the nonsingular one with monic determinant67. The selected
matching follows from the actual Mumford kernel,not from selecting one
of the external CT49 equations. The actual data realizes the queued
Q20=A6 primitive-four class from the unchanged full-suite graph checks;
no extra count is added to the fixed72-Jacobian catalog. [pcc:unverified]

## Common leading scale and normalized raw sextic

Let q=11 in F101,so q³=18. For each selected branch pair put

    Q_i(X)=(X-a_i)(X-b_i),
    F_i(X)=q Q_i(X).

All three factors have the same leading coefficient,are permuted by
Frobenius,and multiply to the original raw source polynomial. This is
the common-cube leading-coefficient convention in Bruin–Doerksen(4.2).
Absorbing18 into just one factor would lose that literal Galois-stable
unordered splitting. Native checks retain the raw source and do not
retwist it. [pcc:unverified]

For monic determinant Delta=67,the determinant of the F_i rows is

    delta=q³ Delta=18*67=95 (mod101).

If B_i=Q_j' Q_k-Q_j Q_k',the unchanged generic Richelot constructor
computes bracket(F_j,F_k)=q² B_i. Define the normalized target factors

    G_i=bracket(F_j,F_k)/delta=B_i/(q Delta).

Then productG_i=productB_i/(18 Delta³),the normalized native prototype
convention. The existing generic genus2::Richelot::codomain instead uses
delta*product(bracket(F_j,F_k));it differs by the square delta⁴. We use
the explicit normalized factors throughout,and do not silently mix the
two ordinate coordinates. [pcc:unverified]

The exact result is

    T20: y²=76x⁶+62x⁵+52x⁴+63x³+69x²+3x+25.

Every coefficient descends individually to F101. The target is smooth,
its six roots form one Frobenius six-cycle,and unchanged native point
counts give N1=120,N2=10436,#Jac(T20)(F101)=12317. Its leading76 is
50²,while source leading18 is nonsquare. These raw conventions are
retained;point counts confirm the constructed model and never choose
a retwist. The raw target agrees with the independently authored CT49
shift2 candidate at cryptanalysisf7d4c6be;that work is credited.
[pcc:unverified]

## Why this quotient and correspondence descend to F101

The coordinate algebra L of K20\{0} is the cubic field F101³,because
Frobenius acts transitively as a three-cycle. Its discriminant square
class is trivial:the square root of the discriminant transforms by the
sign of the permutation of the three embeddings,and a three-cycle has
positive sign. Equivalently the Vandermonde product of a primitive
element's three conjugates is Frobenius fixed and nonzero. This supplies
the actual discriminant-twist hypothesis,not merely a coefficientwise
target descent test. [pcc:unverified]

Bruin–Doerksen Proposition4.3 identifies Jac(C20)/K20 with the Jacobian
of dY²=productG_i,where d is that discriminant square class. Here it is1,
so the actual normalized T20 equation is the prime-field quotient. The
field has odd characteristic and more than5 elements,the source is
smooth of degree6 with leading q³,and the selected determinant95 is
nonzero. Every relevant hypothesis has a concrete native or written
check. This use of characteristic101 does not invoke the paper's
characteristic-zero Proposition2.8. [pcc:unverified]

There is consequently a global separable (2,2)-isogeny

    R20:Jac(C20)→Jac(T20), degree4, kernel K20,
    R20^dagger R20=[2].

The degree4 and Rosati relation follow from the maximal isotropic
two-kernel and the induced principal polarization. Alternatively,
(R20 P20)^*lambda_T=4lambda_A and P20^*lambda_C=2lambda_A give
R20^*lambda_T=2lambda_C. The explicit correspondence is BD(4.5);
normalizing its swapped x-z factor fixes the transpose sign as in the
unchanged native reference. PhaseB must still evaluate these formulas
properly on the complete Mumford domain with their exceptional fibres.
[pcc:unverified]

The odd prime groups109 and113 meet no nontrivial geometric two-kernel,
so this prime-field homomorphism preserves their DLPs. Composing with
the accepted C0→C20 isogeny gives global degree196 and adjoint norm
diag(98,2) in its source elliptic coordinates. These are geometric
composition consequences;new full native chain/scalar/recovery checks
belong to phaseB and are not claimed by the geometry test.
[pcc:unverified]

## Faithful formal and executable scope

Four exactly bound local T20Geometry declarations pass with zero trusted
native:common_leading_determinant/89e906,
normalization_factorization/02c312,normalization_from_units/fee2b1,
actual_normalization_constants/151a58. They establish the universal
determinant scale,normalized product factorization and unit cancellation,
plus concrete Fin101 q/determinant/leading-square constants. Project
census is192 written declarations/38 modules. No axiom asserts the
actual curve,Picard quotient or discriminant identification.
[pcc:unverified]

The independent unchanged identity checker carries
IDC1h11c6b82bbe669b8f and IDC1h3147241afb3d4b6b for the two unconditional
polynomial identities. It checks32 issuer+64 domain-separated points
over fixed2^61-1,not over F101;the universal Lean statements and explicit
unit hypotheses have their own scope. Full algebra01 accepts23 tests,
10 outputs/all13 source hashes,all old8 outputs byte-identical,and
all10 RHS+1 mutants rejected. [pcc:unverified]

Full native output134348bytes is standardXZ11468 with raw/archive hashes,
exact roundtrip,source snapshots,start/finish/exit/all626 afterhashes and
both geometry/old-kernel comparisons. The6141-byte geometry JSON retains
the field modulus,actual Q20 coordinates,kernel Mumford classes/Frobenius,
branch roots/matching/determinants/common scale/factors/raw target/counts.
Static01/7c05b4 compile failures are preserved before the child-only fix.
[pcc:unverified]

Sources checked before code:Bruin–Doerksen0902.3480 §4 including4.1–4.3,
native/src/richelot.rs and richelot/model.rs,unchanged generic genus2
det/bracket/count functions,128eb1/afd164/063773,and external CT49 exact
f7d4c6be. KB queries and theorem query are unavailable/no executable,
with actual receipts in t20-primary-review-01;no novelty inference.
Next is the proper whole Mumford correspondence/dual/basepoint/infinity
implementation,then additive consumer replay and admitted matched costs.
No measured computational advantage is reported here. [pcc:unverified]
