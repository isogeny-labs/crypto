# PhaseB implementation notes — proposals, not executed map results

Accepted model diagnosticfeff57 gives raw C10=[89,0,93,0,8,0,61] and
C20=[40,0,45,0,75,0,18]. Both leading and constant coefficients are
nonsquares overF101:all120 rational curve points are finite,no rational
infinity orx0 points. Do not retwist to force a monic/two-rational-infinity
model. Full34 source-frozen run02 is separate and pending at note creation.
[pcc:unverified]

These notes preserve analysis for the next implementation;none of the new
whole-map,dual,kernel-polynomial or recovery checks below has executed yet.

1. Reuse existing Chart/Cantor conversion. Add Chart::from_sextic with a
   supplied square root rho of the leading coefficient overFp12. Keep old
   Chart::new calling it with the old monic coefficients and rho=1. Preserve
   the old transformation arithmetic. At source infinity,the odd coordinates
   are w=0,z=±L²*rho,so the q divisor changes from v=L² to L²*rho. Old
   canonical models must retain identical outputs. Any refactor needs full
   changed-suite tests;do not edit the current frozen622 subjects during02.
2. EvenModel quotient cubics are[c0,c2,c4,c6] and[c6,c4,c2,c0]. Existing
   cubic_to_short has X=a*u+b/3,Y=a*y;checked scale sends X→s²X,Y→s³Y.
   Covers may swap target factors:actualC10/C20 both first cover→F0 and
   reciprocal→E1/E2. Use the recorded destination field rather than an index
   assumption. The current EvenCover methods already evaluate these maps.
3. Degree2 point pullbacks:undo target scale. Direct branch gives
   t=(X_internal-shift)/lead, U=x²-t,V=Y_internal/lead. Reciprocal gives
   t=lead/(X_internal-shift), U=x²-t,V=(Y_internal*t/lead)*x.
   The origin pulls to identity. The reciprocal denominator-zero case is
   an infinity fibre:±2q according to Y_internal=±lead*rho. For C10/C20 it
   is not hit by rational target points,but retain it over the extension.
4. Nonzero2-torsion pullbacks can include the chosen source branch W=(a,0).
   For U=(x-a)h(x),the class W_other-W converts to odd Mumford
   u=[L/h(a),1],v=0,as in existing quartic pull_four. Do not pass such a
   fibre through a chart function assuming its support avoids W. NonzeroY
   cannot contain W. Repeated x0 fibres at nonzeroY use the existing chart's
   exact polynomial transformation,not a distinct-root approximation.
5. Pointwise push norm of a reduced odd-chart divisor D=(u,v),degree d:
   split u overFp12 with multiplicities,convert each odd point back to the
   original sextic through the existing w,z inverse;w=0 means a source
   infinity and uses EvenCover::infinity. Sum their elliptic images and
   subtract d times the image of W. This basepoint correction is essential.
   Frobenius of the odd chart is not coefficientwise when W is not rational.
6. Source C0 is the old monic model,with covers(dest0,lead1,shift2,scale1)
   and(dest1,lead56,shift38,scale1). Generate its full12317 rational Jacobian
   corpus by all actual109×113 elliptic point pairs and product pulls.
   Check validity and12317distinct outputs;globaldegree4/oddgroup and zeta
   establish completeness. Do not use scalar logarithm labels in the map:
   compute norms from each Mumford input before applying alpha/half/pulls.
7. Map F=P_target*(alpha×id)*H_source/2,using inverse2=55/57 modulo109/113.
   Dual factors alphaHat×id give nonuniform norm diag(7,1),not[7]. Compare
   on all classes to product pulls of(7P,Q). The wrong uniform[7] predicts
  12208 differences and109 equalities,not12316. For the two-E-hop case
   norm is diag(49,1),with the same uniform-mutant difference count. Actual
   inverse uses inverse7=78 modulo109 after the dual;F113 is unchanged.
8. Reuse SevenEdge.reverse for normalized duals or lift its existing dual
   rational map toFp12,retaining scale29. Both E0→E1 andE1→E2 forward raw
   models already equal canonical targets,so their extra scale is1. For any
   later canonicalization,undo its u²/u³ before applying the stored dual.
9. A full Fp42 point engine need not be invented to certify the cyclic49
   kernel. Candidate actual kernel polynomial is
   h49(x)=h1(x)*D1(x)^3*h2(N1(x)/D1(x)),degree3+21=24.
   Construct denominator-cleared coefficients with the existing polynomial
   engine. Check monicity,squarefreeness,division into division_poly49 and
   gcd(h49,division_poly7)=h1. The composition's exact global degree49 and
   kernel characteristic prove cyclicity (intersection with E[7] has7,not49
   points). This is an actual geometric kernel scheme certificate,not an
   enumeration of allFp42 points. Transport it by the source degree2 pull.
   Existing CyclicFortyNine.first_unique_scalar identifies Frobenius33mod49
   from characteristic polynomial/selected eigenvalue5;its exact order42
   determines the splitting degree. Keep the scheme/point distinction.
10. For the degree7 case,also export all actual7 elliptic-kernel points and
    their source Jacobian pulls,validate membership/Frobenius and all49
    sums. Preserve the complete rational-domain checks,mutants and raw
    source/hash/status files,then written/global degree/norm/field proof,
    faithful feasible Lean,and report/PDF before checkpointing phaseB.

PhaseC still needs the C20 image of Q20,its four-element Richelot kernel,
unique matching and actualT20 equation/correspondence/full replay. Q20's
classA6 is a target-class identification,not equality of maps with six
balanced hops or measured solving-cost improvement. The one-factor7
four-graph failure96/256 is accepted actual-coordinate CT44 controlfeff57.
[pcc:unverified]
