# T20 phaseB implementation notes and extra geometric control

Planning only,2026-11-12,after full39/84454e geometry acceptance. The
whole evaluator and the extra sixteen-point controls below are unexecuted.
Original protocola2c024 remains required;these notes preserve the derivation
and add a geometric two-torsion parity control before any B code.
All standing is unsigned. [pcc:unverified]

## Coordinate helpers and evaluator shape

After the phaseA commit,retain the current EvenModel point_class/support
source first. Move only their generic coefficient/chart arithmetic into
free original_point_class and original_support helpers;old methods delegate
without changing arithmetic. New generic Richelot curve contexts hold raw
coefficients,Chart,legacy::Hyp and RootCache,without fictitious even covers.
Every old39 test must remain. Announce the shared-helper scope before editing.

Use the exact phaseA factory T20Geometry:raw source C20,target
[25,3,69,63,52,62,76],F_i/G_i arrays,delta95 and q11. Do not reconstruct
a new normalization. Source Chart chooses the least branch root,which
lies in the matching's first pair,so source W is on F1. For the transpose,
cyclically rotate the paired (G_i,F_i) arrays to put its chart W on the
first source factor. A three-cycle preserves orientation;do not silently
reflect the factors. The reversed correspondence is negated as in the
old native reference to obtain positive dual norm[2]. [pcc:unverified]

For a generic finite source point (x,y),set

    U(z)=F1(x)G1(z)+F2(x)G2(z),
    V(z)=F1(x)G1(z)*(x-z)/y mod U.

Keep full polynomials rather than splitting the target fibre. Degree2 U
with U(targetW) nonzero goes through Chart::class(u,v,0),whose balanced
correction is0 at degree2. If U contains targetW,remove that one branch
factor and use the remaining finite point's original-point class;that
removed point has zero class relative to targetW. A repeated nonbranch
root retains its quotient algebra and multiplicity. Do not treat a
degree-one fibre as a degree-one balanced divisor and lose infinity.

For degree1 U,include both its actual finite point and the signed target
infinity. The infinity sign is

    rho_target=-F1(x)*leading(G1)/y.

Check its square against the raw target leading coefficient. If a constant
fibre occurs,derive its completed projective divisor;do not invent a sign
or discard it. In this cyclic nonsplit fixture the leading/linear fibre
coefficients should preclude an unsupported constant fibre;record actual
realized types and fail rather than silently skip any input.
[pcc:unverified]

For a source infinity with y/x³=rho_source,use

    U=leading(F1)G1+leading(F2)G2,
    V=leading(F1)G1/rho_source mod U.

Both directions' infinity U has degree2 in this fixture;check it. At
finite y=0,the completed classes are (G2,0) for F1=0,(G1,0) for F2=0,
and zero for F3=0,as in the original native prototype. The full class
map is sum fibre(P_i)-d*fibre(W) for a reduced source Mumford divisor.
The omitted-basepoint mutant must remain visible on the prescribed
122source/120target degree-one curve-point controls. [pcc:unverified]

Finite degree-drop inputs are forced by roots of
F1(X)*leading(G1)+F2(X)*leading(G2). Its coefficients lie in F101³ and
degree is at most2,so x lies in F101^6 and y in F101^12. The six branch
roots and all required rational/infinity controls also fit that field.
For these x∈F101^6 points,y is fixed or negated by Frobenius^6,and both
chart anchors are in F101^6. Their classes are fixed or negated by
Frobenius^6. The global F101 Richelot map preserves that exact relation,
so the corresponding reduced u remains in F101^6. Thus u's
roots fit F101^12. Assert this field condition if relying on it. Arbitrary
F101^12 Jacobian inputs can require degree24 support;do not claim that
broader evaluator domain. The required complete F101 class domain has
u coefficients in F101^6 and quadratic support in F101^12.
[pcc:unverified]

## Extra geometric parity/duality control,preregistered before B code

On both actual raw curves,enumerate identity and all15 distinct actual
Weierstrass pair classes. Require all256 sums and actual geometric
Frobenius. Predicted Frobenius-cubed fixed counts are16 for C20's3+3
branch action and4 for T20's6-cycle. Also evaluate R20 and its corrected
transpose on all16 classes,requiring each kernel/image size4,actual
kernel matching/Frobenius,and both compositions equal[2]. These are
new predictions,not already executed by the full39 geometry test.
[pcc:unverified]

An odd-degree F101 isogeny preserves the geometric two-torsion Galois
module,so these distinct fixed counts exclude a direct odd-degree map
between these two models. The degree4 Richelot bridge is compatible.
This is an independent native application of the external CT49 branch
parity observation;do not promote its C++/Python output as our execution.
The original source-to-two-glue degree7/49 maps remain valid. [pcc:unverified]

## Whole-group and chain implementation

Two new diagnostics can separate pointwise/geometric controls from the
whole12317-class test;complete41 tests is then the full B acceptance.
Build all C20 classes from physical E2/F0 rational points,never indices.
Cache actual input divisor→output divisor only. All target classes must
be distinct/valid/Frobenius fixed,with both dual[2] compositions,identity,
negation,generator translations,109/113 scalars and explicit curve/chart
tag rejection. Wrong transpose gives12316 disagreements/one identity.

Rational inverse2 modulo12317 is6159. For the C0→C20→T20 chain,use the
accepted WholeCase::FortyNine/actual edge maps,then R20. After R20's
actual dual,whole_adjoint_recovery(FortyNine) returns its adjoint plus
the Phi49-recovered class [2]D;apply6159 to recover D. The combined
adjoint is diag(98,2). Physical-key caches may be populated by actual
evaluations on the whole domain,not by product labels. Preserve all
original protocols,full source guards,failures,reports and CI requirements.
