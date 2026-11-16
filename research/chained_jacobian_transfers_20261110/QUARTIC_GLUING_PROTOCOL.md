# Actual quartic gluing on the three fixed Jacobians

Question:do the admitted product pullbacks K_i:E_i x F_i→Jac(T_i) realize
an actual maximal graph on4-torsion,attaining CT41's order16 two-adic
gluing bound? All new numerical outcomes below are frozen predictions,
not observations. Prior e78426 is complete30/149written30modules;
no old arithmetic bodies,fixtures or acceptance criteria may change.
[pcc:unverified]

## Prior review and competing outcomes

Read exact CT41/ca083268,primary Howe–Lauter1202.6308§2.3/Prop2.8/
Lemmas2.9–10 and Milne7.2/7.3 (f07968);read actual embedding/crossHom
proof b05286 and old global norm/polarization/degree constructions.
Galbraith corrected Chapter26,Theorem26.2.3,Exercise26.2.4,formula26.1
and §26.3.1 Miller definition/recurrence/Algorithm29 were read,8b9b20.
The pairing convention warning is explicit;simultaneously inverting
both factor pairings preserves the anti-isometry test. KB tools absent,
wrapper has no executable;local sources/primary papers are fallback.
No novelty claim. Existing BLS12 pairing has a fixed different field;
transfer_audit/pairing.rs prices a route but supplies no generic evaluator.
[pcc:unverified]

Hypothesis H1:the actual K_i kernel is a4-torsion graph of order16 with
trivial factor intersections and inverse factor Weil pairings. H2:a
smaller actual gluing or an incorrect ramified-fibre formula causes
kernel/projection/group-law/pairing failure. Any disagreement is retained
with full source/raw/status and diagnosed;criteria stay fixed. Degree16,
polarization,primitive Tate factors and geometric classification are
separate global obligations,never inferred from a finite count alone.
[pcc:unverified]

## Native construction and frozen controls

Use only the accepted F101^12 FpR/poly/curve/legacyCantor engines.
For each of six elliptic models,enumerate O,three roots of x³+ax+b with
y=0,and the two ordinates on each of the six roots of

    h4=x^6+5a*x^4+20b*x^3-5a²*x²-4ab*x-8b²-a³.

The polynomial follows by setting the duplication ordinate to zero;
check every point independently on-curve,killed by4,distinct,Frobenius^6
fixed and all256 elliptic additions. Require exactly16 points. Find a
basis of two order4 points whose16 linear combinations are distinct.
Keep actual point coordinates and basis actions in the native stream.
[pcc:unverified]

For nonzeroY retain the old pull evaluator unchanged. A newY=0 wrapper
uses U=N-XQ²,g=gcd(F_source,U),with predicted degU4,degg2 and U/g a
square of a linear polynomial. The non-Weierstrass conjugate pair cancels
in Picard;the resulting balanced class is (g,0,0). If g contains the chart
branch a,remove(x-a),and convert the remaining Weierstrass point to its
quintic point-minus-W class directly;do not feed a half-integer infinity
coefficient into the old chart. Require all three nonzero2-points for
each factor,exactly one branch-chart corner per factor,and valid reduced
classes. Every old Y≠0 evaluator and old chart body remains unchanged.
[pcc:unverified]

For each T_i,form all256 sums of the two16-point pullback populations.
Predict kernel16/image16,all valid/killed by4,trivial single-factor
kernels and both kernel projections bijective onto all16 elliptic points.
Check both factor pullback addition tables and every256 kernel graph
addition. Check Frobenius stability and fixed kernel counts [1,1,4,16]
at extension degrees1,2,3,6. This enumerates K_i(B_i[4]),not allA_i[4].
[pcc:unverified]

Use Miller's two-doubling identity on independent order4 bases:

    f4P(T)=line_P(T)^2 / (x_T-x_[2]P),
    e4(P,Q)=f4Q(P)/f4P(Q)

with the Chapter26 convention,nonzero denominators and normalized leading
coefficients. Require e4²=-1,e4⁴=1 and e4 in F101. Cross-check against
formula26.1 using a shifted disjoint divisor,searching only16 existing
points for a valid auxiliary point and rejecting every undefined support.
Bilinearity then defines all pairings via basis determinant modulo4.
Check inverse pairing product equals1 on all256 kernel pairs. Retain
a wrong-sign control:replacing the F pairing by its inverse must reject
at least one pair (predicted128 of256). Do not omit vertical factors or
apply reduced-Tate denominator elimination;the cited lemma requiresr>4.
[pcc:unverified]

## Resources,evidence and completion

One selected Rust correctness unit with15-minute hard limit first;then
complete changed31tests/old source guards/format/strict Clippy,also bounded.
Thin shell orchestration only;new research execution stays native.
Record exact commands,before/after source and fixture digests,full raw log,
status,failures,inputs and deterministically chosen basis. Standard whole
XZ/hash/roundtrip for large raw logs. No timing,solver,paid run or experiment
admission bypass. Live budget/S3 blockers9e1812 remain unchanged.
[pcc:unverified]

If accepted,write the ramified pullback proof,global degree/polarization
bridge to an actual (4,4)-split Jacobian,and the primitive Tate-lattice
index argument. Attempt faithful Lean structural kernel/projection and
isomorphism transport lemmas;actual geometry is an explicit partial bridge.
Using the old unique crossHom0 elliptic subvarieties,compare canonical sum
degrees4 for C_i and16 for T_i to extend geometric distinction to all six
surfaces. This remains a written obligation until established.
Source-linked diagram/quantitative chart/report/PDF,new PCC,commit/PR/queue
and exact-head CI follow;advance to the next unblocked task afterward.
[pcc:unverified]
