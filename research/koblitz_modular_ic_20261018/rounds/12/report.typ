#set page(paper: "a4", margin: (x: 19mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.40em)
#set heading(numbering: none)
#show link: set text(fill: rgb("245c91"))
#heading(level: 1)[#text("Round 12: supersingular divisors and actual projective saturation")]

#text("19 October 2026, America/Los_​Angeles; raw protocol date 20 October UTC. Parent commit: 539325b5e66719f4cea7ee1315659808103d5bcb.")

#text("All supersingular zeros of the eight coupled functions now have closed supports and exact multiplicities, under the written modular and etale bridges. The antidiagonal contributes degree 740 there, leaving ordinary degree 500. Every whole unfinished divisor still has an ordinary part. The four actual quotient circuits have explicit false points under naive homogenization and a certified intermediate linear saturation step. Whole proper quotient embeddings remain open. No IC costs are assessed.")

#image("routes.svg", width: 100%)

#heading(level: 2)[#text("Complete supersingular strata")]

#text("The 56 cyclic-49 labels are now attached to actual points of the minimal X0(49) model y²+xy=x³+x²+1. All six target points lie in F4; four have 12 labels each and two have 4. We enumerate all 3,080 supersingular source points, their Frobenius permutation, and 308 closed points of degree 10. Every function is evaluated through order 15. No zero jet remains censored.")

#table(columns: 4, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Function")],
[#text("Complete SS zero degree")],
[#text("Remaining ordinary degree")],
[#text("Noncuspidal degree")],
),
[#text("x0+xinf")],
[#text("1600")],
[#text("500")],
[#text("2100")],
[#text("x0+xinf+1")],
[#text("760")],
[#text("1760")],
[#text("2520")],
[#text("y0+yinf")],
[#text("1540")],
[#text("1880")],
[#text("3420")],
[#text("y0+yinf+1")],
[#text("1020")],
[#text("2760")],
[#text("3780")],
[#text("x0*xinf")],
[#text("0")],
[#text("0")],
[#text("0")],
[#text("x0*xinf+1")],
[#text("2060")],
[#text("580")],
[#text("2640")],
[#text("x0*yinf+xinf*y0")],
[#text("2080")],
[#text("1370")],
[#text("3450")],
[#text("x0*yinf+xinf*y0+1")],
[#text("1720")],
[#text("2180")],
[#text("3900")],
)

#text("The complete x0*xinf divisor was already cuspidal in round 10. The other seven whole divisors remain incomplete. Their ordinary degrees are derived by subtracting the complete supersingular contributions from the independent round-10 noncuspidal degrees; they are not enumerated ordinary supports.")

#text("For x0+xinf, the old diagonal degree 860 is reproduced at all 400 geometric labels, including contacts 14. The antidiagonal has 420 contacts of order 1, 40 of order 2, and 60 of order 4: degree 740 on 52 closed degree-10 points. Its remaining degree 500 is ordinary. The full antidiagonal and full x-sum principal relation are therefore not complete.")

#text("Every supersingular target value is killed by 8. The previously established transfer projections preserve this property; their 11-primary images are zero by coprimality. These completed strata cannot add nonzero 11-primary columns. This is a structural image calculation, not an IC efficiency assessment. Evaluated rank stays 12/1308 and nonzero-image rank 11 on 1165/1160 columns. No new whole principal row is promoted.")

#heading(level: 2)[#text("Actual level-49 coordinate adapter")]

#text("Use E0:y²+y=x³ over F4096. It has 3,969 rational points and 49 points of 7-torsion. Calibrate the actual basis P,I(P) to the retained quaternion module modulo 7. The 192 norm-7 Hurwitz elements give 8 kernels. Each chosen map is bound to an actual normalized Velu map and target unit by 3,962 nonkernel point checks: 31,696 checks in all, with a written global degree bound. Explicit points of order 49 have not been constructed.")

#text("A cyclic-49 subgroup gives a nonbacktracking chain of two 7-isogenies. For each retained abstract line S, its first quotient is alpha7. Dividing alpha7(S) by 7 modulo 49 identifies the second kernel in the actual 7-torsion basis. All 56 chains are distinct. The compatibility of this module lift with actual 49-torsion remains a written Tate-module bridge.")

#text("Lift the 24 source 7-torsion x-roots and 24 roots on each of the 8 first quotients through order 127. For quotient coefficients A,B, use the actual general Weierstrass 7-division polynomial. A kernel trace c changes them by")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("A' = A+s+s²*c,\nB' = B+1+s*c+s³+s⁴*c.", block: true)]

#text("With discriminants d0,d1, recover the 7-Hauptmodul t from")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("t^6=d0/d1,  t^8+t^4+1+j0*t^7=0.", block: true)]

#text("Both supersingular constant candidates are retained until one satisfies the equations through order 63. The two edge parameters give eta x via x^4=t1*t2. The minimal y coordinate is recovered from (eta_​y+x+1)/(x²+x+1), including leading-term division at its vanishing constant denominator. This distinguishes branches that j-triples alone cannot distinguish at supersingular points.")

#text("All target x-series coefficients lie in F4. Normalize each of the 12 11-quotients back to E_​z with z=s/u and its actual recorded special-fiber unit. Composing these parameters with the target series produces the local modular coordinates. The minimal y-series is recovered by the implicit elliptic equation, whose derivative in y is a unit here. This supplies all eight local functions on all 3,080 source labels.")

#text("A separate parser substitutes exported data and checks 27,648 division-root, 8,192 Hauptmodul, 768 normalization, and 394,240 function coefficients. It shares primitives with the generator and is not an external audit. The Frobenius orbits preserve every recorded valuation.")

#heading(level: 2)[#text("Four actual homogenized systems fail; an intermediate repair is explicit")]

#text("Each full round-10 circuit now has a projective F2 point satisfying every homogenized equation but violating a certified saturated linear relation. The native checker checks all 1,679,760 scalar equations across the four systems. Eight generated Lean theorems check the four restricted finite circuits and the four nonzero missing relations in the kernel. The passage from these finite scalar restrictions to the full coefficient algebra is identified separately in the proof map.")

#text("The defect is an auxiliary constant multiplication, not an obstruction intrinsic to the abelian quotient. If a is a node representing constant c, its homogeneous equation is a=c*h. The multiplication gate gives h*out=a*input. Their saturation contains out=c*input, since")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("h*(out-c*input)=(h*out-a*input)+input*(a-c*h).", block: true)]

#text("The false points violate this relation at h=0. Thus the original full homogenized ideals are strictly smaller than their h-saturations.")

#text("Native constant propagation now evaluates every constant-only node over the pinned F2^360 and emits 150,150,146,146 new K-linear relations, or 213,120 generated scalar linear relations. The maximum certified h-power membership depth is 5. The original equations are retained; the enlarged ideal has the same h-chart and sits inside the full h-saturation. Relations may be redundant. No independence, elimination, full saturation, global boundary gluing, or proper polarized quotient embedding is claimed.")

#heading(level: 2)[#text("Proofs, failures and remaining work")]

#text("Twenty-four new named Lean proofs are supplied: eight actual-circuit certificates and sixteen algebraic or structural results. These include saturation membership, strictness from a counterpoint, parameter recovery, local-order invariance under a unit, coprime-primary image vanishing, the six target points and exact degree partitions. Full project audit counts and axiom receipts are in ledger.json and the raw archive. Modular geometry and native field semantics remain partially formalized.")

#text("Preserved attempts include the first incorrect one-branch stabilizer bound, 8-jets censoring twenty diagonal points, the first aggregate h-depth count omitting scaling relations, and interrupted duplicate Lean builds before minimal imports and sequential compilation. The unrestricted root test run also exposed a shared-counter failure. That test passes in isolation, and the full serial rerun passes: 4,405 passed, zero failed, 118 ignored. The native suite passes 106 tests with one ignored; the site and workflow suites pass 49 and 4 tests. The original failure remains archived. No test was removed or weakened.")

#text("Two earlier statements are corrected explicitly. Round 11's proof map called the three Artin-Schreier subcovers independent: there are two independent characters and their nonzero sum, the three nontrivial characters of a rank-2 group. The genus-3 result is unchanged. The previous status misidentified expected TODO negative-test output as a CI failure; the actual backlog failure is a concurrent-claim integration test. The study's round-11 Lean CI job passed. Broader CI failures and cancelled checks remain separate publication gates.")

#text("Next: construct the ordinary antidiagonal support of degree 500 and the ordinary x-product-plus-one support of degree 580, with every closed point, local length and projected image. On quotient geometry, continue saturation of the actual circuits after the explicit constant-gate step, and recover the proper Kummer boundary charts and polarization. The present results do not discharge those requirements.")

#text("")#link("PROTOCOL.md")[#text("Protocol")]#text(" · ")#link("PROOF_MAP.md")[#text("proof map")]#text(" · ")#link("ledger.json")[#text("ledger")]#text(" · ")#link("REPLAY.sh")[#text("exact replay")]#text(" · ")#link("../11/REPORT.md")[#text("previous round")]#text(".")

#text("The first main Lean build also needed explicit normalization of nested powers in the characteristic-2 parameter factorization. That failed elaboration log is retained; only the successful rebuilt module and full axiom audit support the formal claims.")

#text("An early ledger invocation ran before the proof-gate wrapper had exported its audit receipt and failed on that missing file. Its log and partial identity exports are retained; the final ledger is issued after the proof gate finishes and all raw logs are closed.")

#heading(level: 1)[#text("Formal proof map and written arguments")]

#heading(level: 2)[#text("Exact formal scope")]

#text("KoblitzRound12.lean has sixteen named theorems. The generated KoblitzRound12Boundary.lean has eight further named theorems checking the actual four boundary circuits and their violated saturated relations. Full audit counts are recorded in ledger.json. There are no custom axioms, sorry terms, or native decision trusts. Standard Lean foundations are reported by the transitive audit. A separate lean4checker --fresh run was unavailable: lean4checker is not installed on this runtime's PATH.")

#table(columns: 2, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Declarations")],
[#text("Exact scope")],
),
[#text("constant_​multiplication_​saturation, constant_​inverse_​saturation, saturated_​constant_​product")],
[#text("Universal ideal-membership identities for actual constant gates")],
[#text("strict_​saturation_​from_​counterpoint")],
[#text("Nonmembership under an evaluation vanishing on an ideal, plus a displayed h-multiple in the ideal")],
[#text("homogeneous_​constant_​false_​point")],
[#text("Scalar characteristic-2 boundary witness")],
[#text("graph0_​all_​boundary_​gates through graph3_​all_​boundary_​gates")],
[#text("Kernel evaluation of every emitted restricted circuit gate and residual")],
[#text("graph0_​missing_​linear_​relation through graph3_​missing_​linear_​relation")],
[#text("The four actual emitted points violate the selected saturated linear relation")],
[#text("velu_​discriminant_​residual, two_​seven_​quotient_​coefficients")],
[#text("Exact coefficient algebra before/after characteristic-2 specialization")],
[#text("seven_​parameter_​factorization, seven_​parameter_​ratio, eta_​y_​recovery_​unique")],
[#text("Hauptmodul factorization, ratio and coordinate recovery with explicit unit hypotheses")],
[#text("uniformizer_​order_​unit_​factor")],
[#text("Power-series order is unchanged by a factor of order zero")],
[#text("coprime_​primary_​image_​zero")],
[#text("An element killed by 8 and 11 vanishes in an additive group")],
[#text("F4_​six_​supersingular_​target_​points")],
[#text("All six displayed points satisfy the minimal elliptic equation when w²+w+1=0 in characteristic 2")],
[#text("antidiagonal_​SS_​degree, SS_​closed_​point_​count, eight_​divisor_​degree_​partition")],
[#text("Exact finite degree bookkeeping")],
)

#text("The multiplication-saturation and Velu-discriminant residual identities have source-bound IDC certificates with 64 issuer and 128 additional checks each. These certify the universal identities, not the entire modular construction. Native finite-field implementation, circuit parser semantics, supersingular moduli identification and deformation geometry remain written bridges. Partial formalization is explicit.")

#heading(level: 2)[#text("What the finite boundary theorems mean")]

#text("Each circuit value is an actual point in the scalar F2 subspace of K, where K=F2[z]/(z^360+z^5+z^3+z^2+1). At h=0, all constant-node values are zero, Add and coefficient Frobenius give linear scalar equations, and every multiplication or inverse gives a zero-product condition. Coefficient Frobenius fixes the scalar subfield. Native bit Gaussian elimination and bounded branching find all four displayed models.")

#text("The generated Lean data contains every gate, point bit and residual index from these actual programs. Its kernel checker evaluates exactly these restricted equations. It does not impose Boolean equations on the original scheme. To interpret its Boolean values as a point of the full coefficient ring, send each value to coefficient 0 and set coefficients 1 through 359 to zero. The multiplication tensor sends 1*1 to 1 and coefficient Frobenius fixes 1. Thus every checked scalar equation gives all 360 coefficient equations. Native checks explicitly visit all 1,679,760 scalar equations. This tensor-to-scalar interpretation remains written; it is not silently imported as a custom Lean axiom.")

#text("For the selected gate out, constant-node a and constant c=1, the universal membership identity places h*(out-input) in the original ideal. The boundary point has out-input=1. Evaluation therefore proves the linear relation is not in that ideal. Each of the four actual homogenized ideals is strictly smaller than its h-saturation. This concerns the auxiliary presentation. It is not an obstruction to a proper quotient model.")

#text("Constant-only gate evaluation adds further linear relations. Original constant nodes have h-membership depth 0. Addition or coefficient Frobenius preserves the maximum existing depth. Multiplication by a constant of depth k yields its linear scaling relation at depth k+1. Two constant factors give depth at most 1+max(k,l). A constant inverse uses the displayed inverse identity. Native propagation records these depths at every node and all scale relations; the maximum is 5. Expanding constants through the pinned basis tensor specifies every scalar polynomial losslessly. The resulting intermediate ideal contains the original one, has the same h-localization, and lies in its saturation. No complete saturation, independence or elimination is inferred.")

#heading(level: 2)[#text("Actual 49-level targets from local 7-isogenies")]

#text("The maximal Hurwitz order and its actual unit maps are the retained round-10/11 construction. Over F4096, all 7-torsion points are explicit. Choose P,I(P) so J acts by the reduction modulo 7 of J=[[2,17],[17,-2]] modulo 49. The 192 norm-7 quaternions give all eight 7-kernels. Bind every chosen map to Velu by the doubled-map identity on 3,962 nonkernel points. For twice-quaternion coordinates of norm 7, the sum of absolute coefficients is at most 10. The difference degree is at most (2*sqrt(7)+10)^2, strictly below 256. The finite identities therefore imply a global map identity. Cancellation of [2] is in the endomorphism domain. The point controls alone are not the proof.")

#text("The actual 49-Tate module realizes the split maximal order modulo 49. Its reduction is calibrated by the actual 7-basis. For a retained cyclic 49-line S, alpha7 kills its order-7 subgroup; alpha7(S)/7 modulo 49 is the resulting order-7 subgroup of the quotient. Applying the inverse post-Velu unit identifies it with actual 7-torsion coordinates there. This yields all 8*7=56 nonbacktracking chains. Their composite kernels are cyclic of order 49; the excluded backtrack has noncyclic kernel. The compatibility of the abstract split module with a lift of the actual 7-basis is a Tate-module/Skolem-Noether argument, not explicit coordinates of points of order 49.")

#text("Odd-order torsion is finite etale. Its points and subgroups lift uniquely through nilpotent thickenings. The relevant framework is ")#link("https://stacks.math.columbia.edu/tag/0BQB")[#text("Stacks, finite-etale invariance")]#text(". Thus the selected special-fiber roots lift to the intended kernel, not merely arbitrary roots of a division polynomial. Native Newton iteration checks all 216 root series through order 127. The general Velu formula specializes to the A',B' in REPORT.md; see ")#link("https://ddd.uab.cat/pub/pubmat/pubmat_a2007vEXTRA/pubmat_a2007vextrap147.pdf")[#text("Miret–Moreno–Rio, section 1")]#text(".")

#text("The actual degree-7 Hauptmodul has j0=A(t)/t^7 and j7=A(t)/t, with A(t)=t^8+t^4+1. These are the frozen modular coordinates from ")#link("https://arxiv.org/pdf/1110.6801")[#text("Kilford–McMurdy")]#text(". At the supersingular fiber, t has constant w or w². The squared ratio and cube-root equations recover the two possible series. One and only one satisfies the modular equation through order 63. Their two-edge product is a fourth power, giving the retained eta x coordinate. Recovering minimal y by leading term division distinguishes the branches where x²+x+1 vanishes. All six resulting targets satisfy the actual elliptic equation, and their Hurwitz orbits and Frobenius agree with the retained labels.")

#heading(level: 2)[#text("Local multiplicities and completeness of the SS strata")]

#text("The round-11 Weierstrass slice E_​s=y²+sxy+y=x³ is retained. The source full 11-level removes all noncentral automorphisms, while the central minus sign extends and acts trivially on the deformation base. Thus s is a uniformizer on the source moduli curve. This is the same written moduli bridge as round 11.")

#text("A lifted 11-quotient has A=s+s²*c and B=1+s*c+s³+s⁴*c. Normalize it back to E_​z using its recorded special-fiber unit. For x=u²*X+r, y=u³*Y+u²*v*X+t, the a2 equation gives r=s*v+v², the a3 equation gives u³=1+s*r, and the remaining a4,a6 equations have unit Jacobian in v,t. The a1 equation gives z=s/u. This determines the parameter change to all orders. Exported coefficients are substituted and checked through order 15. All kernel sums and target series lie in F4; no unstated compositum-field adapter is used.")

#text("Compose the 49-level x-series with z(s). At each target x is nonzero, so the derivative of the minimal elliptic equation with respect to y is a unit. Its y-series is uniquely recovered by Hensel recursion. Consequently the eight displayed function jets are the actual local functions on the source. Every zero has a nonzero coefficient before order 16. Their first nonzero orders are therefore exact, not lower bounds. The 400 old diagonal contacts are an independent construction control and agree, including the twenty contacts of order 14.")

#text("All supersingular elliptic curves in this characteristic are represented by E0 over the algebraic closure. Cyclic49 lines and full11 bases modulo its actual automorphisms give precisely the retained 3,080 source labels. The native F2 permutation uses actual Frobenius, determinant-normalized 11-level, and the same source-unit canonicalization. All 308 orbits have degree 10, and every valuation is orbit invariant. This proves complete supersingular closed support under the written moduli bridge. It does not exclude ordinary zeros. Subtraction from the independently retained noncuspidal degrees gives the seven positive ordinary residual degrees.")

#text("Elliptic negation on the genus-one base is not the modular inertia action. It can exchange target points with different modular stabilizers. This is why the first one-branch stabilizer bound was invalid. The final contacts use the actual coordinate jets; the preliminary export is retained and explicitly superseded.")

#text("All supersingular target values are in the computed eight-element E49(F4), and are killed by 8. Integral correspondences, Frobenius and norms preserve this torsion property. Their projection to a group killed by 11 vanishes by the formal coprime-primary lemma. This uses the retained projection maps; it is not a new runtime image evaluation or rank gain.")
