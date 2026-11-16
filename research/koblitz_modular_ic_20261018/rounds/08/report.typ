#set page(paper: "a4", margin: (x: 19mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.40em)
#set heading(numbering: none)
#show link: set text(fill: rgb("245c91"))
#heading(level: 1)[#text("Round08 — complete line norms and intrinsic Kummer charts")]

#text("2026-10-19 UTC. Parent 244f1b850c. Earlier rounds remain frozen.")

#text("The remaining line-norm functions raise nonzero-image relation rank from 6 to 11 for both signs. The final evaluated matrix has rank 12/1308; its nonzero-image populations have 1165 and 1160 columns. These rows have new supports: the larger ranks are reported with the larger column sets.")

#text("For each of four determinant-19 graph quotients, two explicit Kummer equations now describe a dense intrinsic chart over F2^360. Their 40 Miller functions pass exact divisor checks. A written dual-isogeny argument identifies the rooted cover with the polarized quotient. Six covering charts per graph now have explicit transitions, checked by200 exact rational identities. A closed projective embedding and F2 coordinate descent remain open.")

#image("routes.svg", width: 100%)

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Requested direction")],
[#text("Verified result")],
[#text("Remaining gap")],
),
[#text("More direct relations")],
[#text("Five nonzero-line norms and the fixed line∞; complete supports and geometric joins; independent images and ranks")],
[#text("Coupled functions beyond this family, relation costs and individual descent")],
[#text("Intrinsic determinant-19 equations")],
[#text("Two localized Kummer equations and six covering charts per graph, expanded coefficient arrays and exact transitions")],
[#text("Closed projective embedding and explicit F2 descent on these coordinates")],
[#text("Lean")],
[#text("16 new named results for coordinate changes, Miller divisors, Kummer identities, chart coverage and cocycles")],
[#text("Native finite-field certificates and global Picard/isogeny identification remain written")],
)

#heading(level: 2)[#text("Complete line-norm family")]

#text("Use the same genus-2841 determinant-normalized full-eleven-level curve C, with twelve degree-660 maps phi_​a to E49. Over F2 the normalized E49 model is E1: y²+xy=x³+x²+1; E0: y²+xy=x³+1 is its quadratic twist. The order-11 point P0 is (8,31) over the pinned F32 polynomial basis. Actual modular target traces come from the independently reconstructed round07 point-algebra data, rather than a Gram matrix.")

#text("M10 evaluates")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("f_a = Norm_F1024/F2(x(phi_a)+x(P0)),  a=1,2,3,4,5.", block: true)]

#text("The a=1 row is the byte-exact round07 control. Each function has 110 degree-60 zero columns and ten degree-660 columns, total zero degree 13200. The poles have the same degree and their projected cusp images vanish as in round07. Lines a and−a give the same norm: reindex the ten Frobenius factors by five, using 2^5=−1 modulo11 and Frob^5(P0)=P0.")

#text("The four additional rows have no joins to previous columns and add 480 columns. M10 gives evaluated rank 11/1248 and nonzero-image rank 10 on 1115/1110 columns. The final M11 function is")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("f_infinity = Norm_F32/F2(x(phi_infinity)+x(P0)).", block: true)]

#text("Its 55 degree-60 and five degree-660 zero columns have total degree6600. None join the previous supports. This yields the final counts below.")

#table(columns: 4, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Evaluated population")],
[#text("All-column rank/count")],
[#text("Nonzero a=1 rank/count")],
[#text("Nonzero a=0 rank/count")],
),
[#text("Round07")],
[#text("7/768")],
[#text("6/675")],
[#text("6/678")],
[#text("Five-line family M10")],
[#text("11/1248")],
[#text("10/1115")],
[#text("10/1110")],
[#text("Add fixed line∞ M11")],
[#text("12/1308")],
[#text("11/1165")],
[#text("11/1160")],
)

#text("Column identity is geometric. Split orbits use the complete modular-base point and ordered torsion basis, modulo simultaneous sign. For nonsplit degree-11 components, the code changes primitive generator to x(Pnew), finds its minimal polynomial and expresses every other coordinate in that generator by invertible 11×11 field linear algebra. Substitution checks both directions. The canonical keys include the actual curve, base point, minimal polynomial and all remaining basis coordinates; they compare whole components independently of their field presentations. Ten or five coefficient-Frobenius phases close each orbit. Equal projected images are never used to identify columns.")

#text("The line∞ change is (P,Q)→(−Q,P). Its normalized Frobenius5 action commutes up to simultaneous sign, so it uses the original length-12 seed cycles. The other lines use (P,Q)→(−aQ,a^−1P+Q) and length-6 Frobenius10 cycles. All component degrees, coordinate keys and principal sums are checked. Separate small-field arithmetic checks all five coordinates of every new image and every principal row; independent modular elimination agrees with the BigInt ranks. This is a same-agent check with shared primitives, not an external audit. No IC cost or efficiency verdict is supplied.")

#heading(level: 2)[#text("Two equations for each mixed quotient chart")]

#text("Let B=E^4×E', let Γ be one of the four frozen actual graph kernels, and let A=B/Γ with its principal polarization. The ordinary factor has trace ±1; E': y²+y=x³+x+c has trace±2, with c=0 or1. All coordinates use F2^360 defined by x^360+x^5+x^3+x^2+1. The prior polarization certificate gives diag(H,[19])=q^dual λ_​A q. The two chosen graph generators are the recorded kernel points at indices1 and19.")

#text("For each component T_​ij of generator i, compute")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("f_ij(x_j,y_j) = (A_ij(x_j)+B_ij(x_j)y_j)/D_ij(x_j),\ndiv(f_ij) = 19([T_ij]−[O]).", block: true)]

#text("The coefficient arrays are fully expanded in q6-intrinsic-Kummer-charts.json. All40 denominators reduce to1; each A has degree9 and each B has degree8. The mixed equations are factored polynomial generators, not orbit circuits:")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("z_i^19 product_j D_ij(x_j)\n  + product_j (A_ij(x_j)+B_ij(x_j)y_j) = 0,  i=1,2.", block: true)]

#text("Together with the five elliptic equations, localize at every displayed numerator and denominator. Equivalently add w*Delta+1=0, where Delta is their product. This gives seven equations in twelve affine coordinates, plus the single localization coordinate/equation: dimension five.")

#text("Every one of the 40 functions has a normalized pole of order19 at O. All denominator roots lie among its nonzero19-torsion multiples. Exact local jets through order64 give order19 at T and zero at each of the other17 nonzero multiples. There are 720 finite valuation checks. A separate pointwise Miller recurrence agrees at 640 nonsupport points. The parser rechecks the stored coefficient arrays and all those checks.")

#text("The two graph classes are independent, checked by their projections to the supersingular companion basis. Therefore their Kummer classes give a connected degree-361 cover. The Jacobian entries in z_​1,z_​2 are z_​i^18 product_​j D_​ij, units on the declared chart; this is a finite etale cover of the base open set. The proof map explains the Picard and dual isogeny identification, including the choice above the origin that removes constant twists. It uses standard global geometry in a written argument.")

#text("For global gluing over the splitting field, choose R_​j=19S_​j on each factor, with19dR_​j nonzero for1≤d≤5. Six diagonal translates kR_​j, k=0..5, have pairwise disjoint bad sets on each factor. A point can fail at most one chart per factor, so at least one of the six charts contains every point of the fivefold. Lean proves the finite coverage implication.")

#text("Explicit Miller line quotients h_​T,R satisfy")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("f_T(P+R)=f_T(R) h_T,R(P)^19 f_T(P).", block: true)]

#text("All200 needed rational identities pass exactly, and all200 constants f_​T(R) have recorded, verified19th roots. Multiplying each h by its root gives the transitions of the two Kummer coordinates. Ratios give the transitions between any two charts, and their cocycle telescopes. A parser rechecks all200 stored transition functions,100 exact curve translation identities and400 point translations. This supplies an explicit global covering atlas over F2^360.")

#text("A closed projective embedding and explicit F2 transitions remain uncomputed. Neither a fivefold equation nor a principal polarization alone identifies a genus-5 Jacobian. That low-genus carrier question remains open.")

#heading(level: 2)[#text("Failures, proofs and reproducibility")]

#text("The first build lacked an explicit collection type and failed; its log is retained. The first M11 execution had incomplete function metadata even though its rows and ranks were computed. Its JSON is preserved separately; the corrected execution records the line∞ function and matches those mathematical outputs. A patch application failed before changing source. No failed arithmetic result is promoted.")

#text("The ")#link("PROTOCOL.md")[#text("protocol")]#text(", ")#link("ledger.json")[#text("ledger")]#text(", ")#link("PROOF_MAP.md")[#text("proof map")]#text(" and ")#link("VALIDATION.md")[#text("validation")]#text(" retain requirements, exact bounds, failures, source hashes and remaining work. Sixteen new named Lean results pass the kernel build and audit. Two source-bound identity certificates each pass 64 issuer plus128 extra checks. Fresh replay reproduces all eight new JSON records byte for byte, and round07 replays unchanged. All earlier archives remain frozen.")

#text("The universal identities carry certificates ")#link("identity-binary-function-product.json")[#text("IDC1h8a89a6a228e61b7e")]#text(" and ")#link("identity-kummer-jacobian-inverse.json")[#text("IDC1hc94fb6c016c0ae78")]#text(", each with 64 issuer and128 extra checks and a Lean source hash.")

#text("Primary sources for the written bridge are ")#link("https://stacks.math.columbia.edu/tag/03PK")[#text("Kummer theory, Stacks 03PK")]#text(", ")#link("https://www.jmilne.org/math/xnotes/AVs.pdf")[#text("Milne on dual isogenies")]#text(", and ")#link("https://www.hyperelliptic.org/HEHCC/chapters/chap04.pdf")[#text("Miller functions")]#text(". The finite constructions and numerical counts above are new repository calculations. No timing or destination-solver experiment was run.")

#heading(level: 1)[#text("Formal proof map and written arguments")]

#text("The new Lean module is KoblitzRound08. It imports the pinned earlier proofs and adds16 named results without custom axioms or native trust.")

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Claim")],
[#text("Kernel-checked statement")],
[#text("Written or native bridge")],
),
[#text("Invertible level change")],
[#text("deck_​basis_​inverse, fixed_​line_​swap_​descent")],
[#text("Actual basis coordinates and normalized Frobenius checked natively")],
[#text("Primitive coordinate changes preserve identity")],
[#text("coordinate_​change_​injective")],
[#text("Native11×11 inverse and every coordinate substitution checked")],
[#text("Kummer equation clearing/change")],
[#text("kummer_​denominator_​identity, kummer_​root_​change")],
[#text("Explicit coefficient arrays and displayed localization")],
[#text("Function multiplication on the binary curve")],
[#text("binary_​function_​product")],
[#text("Characteristic2 relation, polynomial reduction and rational denominators")],
[#text("Miller divisor")],
[#text("miller_​divisor_​recurrence, miller_​divisor_​torsion")],
[#text("Line-function divisors; exact chain points, support, jets and origin pole")],
[#text("Etale chart")],
[#text("kummer_​jacobian_​identity, kummer_​jacobian_​inverse, kummer_​root_​nonzero, kummer_​derivative_​nonzero, diagonal_​jacobian")],
[#text("Finite-free monic equations over localized base; Kummer/Picard identification written")],
[#text("Complete zero-degree sum")],
[#text("complete_​new_​norm_​degrees")],
[#text("Complete modular etale fibers and cusp pole images from earlier written bridge")],
[#text("Global chart coverage and gluing")],
[#text("six_​chart_​cover, transition_​cocycle")],
[#text("Actual translated bad sets, exact rational transition identities and verified constant roots")],
)

#heading(level: 2)[#text("1. Exact joins and relation ranks")]

#text("The old columns through index587 have degree6,30 or210. A point from the present supports projects to a modular base point of degree60, checked in round07. Since60 divides none of those degrees, these supports cannot meet those columns. M8 columns are then compared directly, and all subsequent split/etale columns have exact coordinate keys.")

#text("For a degree11 field K[t]/g, choose z=x(Pnew). Its powers0 through10 form a basis precisely when the computed matrix is invertible. The code checks that inverse. Express z^11 in the basis to obtain its monic minimal polynomial; express y(Pnew),x(Qnew),y(Qnew) in the same basis. Substitution back into the original algebra verifies every expression. This describes the actual closed component without choosing a root. Simultaneous sign changes keep x(Pnew) fixed and change the two ordinates, so taking the smaller of those two encodings removes exactly that level equivalence. The modular base point and curve coefficients are also included.")

#text("Applying coefficient Frobenius to this presentation gives the Frobenius component. The new complementary point must also be scaled by2^−j, as required by determinant-normalized level. The generated keys do this before the coefficient Frobenius. All closed orbits have60 distinct component keys: geometric degree60 in the split case and660 in the etale case. Matching complete key sets verifies an overlap; equality of group images does not enter the join decision. M10 and M11 have no new joins.")

#text("The complete divisors follow the same ordinary prime-to-characteristic level-cover argument as round07. M10 has five degree13200 zero divisors; M11 has degree6600. Poles are modular cusps with zero projected11-torsion images. Every row sum vanishes. BigInt and separate u64 elimination compute rank12/1308 and rank11 on1165/1160 nonzero-image columns. The finite full matrix rank is not a new Lean matrix theorem; exact finite calculations and the earlier generic support-independence lemma are disclosed separately.")

#heading(level: 2)[#text("2. Miller divisor certificate")]

#text("Write D_​n=n[T]−[nT]−(n−1)[O]. The standard line quotient for P,Q has divisor [P]+[Q]−[P+Q]−[O]. Thus adding its divisor to D_​n+D_​m gives D_​(n+m). Lean proves this formal-divisor recurrence and the specialization D_​19=19([T]−[O]) when19T=O. Binary19 uses the fixed bits0,0,1,1. The actual line/tangent points and sums are recorded.")

#text("Native function arithmetic represents (A+By)/D and reduces products by y²=h(x)y+f(x). The universal integer identity binary_​function_​product specializes to that reduction in characteristic2. Polynomial gcds cancel common factors. The final function has leading coefficient1 and pole order19 at O, using pole orders2 for x and3 for y. The two possible numerator leading terms have different parity, so cannot cancel.")

#text("The denominator factors over the listed subgroup abscissas. On the ordinary19-torsion x is nonzero, and on the supersingular curve h=1; therefore x−x(T) is a local uniformizer in every tested affine neighborhood. Exact implicit-function jets determine the valuations of numerator and denominator separately through order64. They give one finite zero of order19 and no finite poles. Together with the order19 origin pole and the degree-zero divisor law, this proves the recorded divisor. Independent pointwise Miller evaluations further check the rational expression.")

#text("These divisor calculations use exact native fields; their complete finite-field semantics are not formalized in Lean. The field irreducibility and subgroup support are checked by the native replay and earlier graph certificate. The certificate hashes bind the actual implementation.")

#heading(level: 2)[#text("3. Identification of the intrinsic quotient chart")]

#text("Work over K=F2^360, containing μ19. Let B=E^4×E' with its product principal polarization λ0, let q:B→A=B/Γ, and let λ_​A be the principal polarization already obtained from the isotropic graph. Set")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("r = λ0^−1 q^dual λ_A : A→B.", block: true)]

#text("Then r q=diag(H,[19]); both q and r have degree19². Under λ0 and λ_​A, the dual of r is q. Hence the kernel of the dual isogeny is exactly Γ, as an actual subgroup of B[19]. This uses the retained polarization pullback and graph certificates, not only a matching dimension or order.")

#text("A point T=(T_​j) of B[19] represents the degree-zero line bundle L_​T=⊗pr_​j^* O([T_​j]−[O]) under the product principal identification. The product F_​T=∏ f_​Tj trivializes its19th power. The Kummer construction therefore gives a μ19 torsor with this Picard class. For two generators of Γ, the two corresponding classes are independent: project a proposed linear relation to E', where the actual companion points form a basis. Equivalently, a19th-power relation among F_​1,F_​2 would make the same nonzero combination of line bundles principal. Thus the combined cover is geometrically connected of degree19².")

#text("The rigidification matters. Every component function has normalized leading coefficient1 at O. With local parameters t_​j, the unit obtained from F_​i by multiplying by∏t_​j^19 has value1 at the product origin. Since19 is invertible, it has a unique formal19th root with value1. This chooses a K-rational point above the origin and excludes an unrecorded constant torsor twist. The resulting connected etale cover of B is an isogeny with dual kernel Γ. It is therefore r:A→B, with this origin choice. This is the standard dual-isogeny/Kummer correspondence.")

#text("On the open set where every numerator and denominator is a unit, its intrinsic coordinate algebra is exactly the two displayed monic Kummer equations. It is finite free of rank361 over that base open set. The derivative entry z_​i^18 D_​i has inverse z_​i N_​i^−1 when z_​i^19 D_​i=N_​i. Lean proves the exact identity and inverse witness; the diagonal determinant is a unit. This gives a smooth finite etale dense chart. The connected function field identifies its prime component without a numerical choice among roots.")

#heading(level: 2)[#text("4. Six explicit charts cover the global quotient")]

#text("For each factor choose S and put R=19S, checking19dR≠O for1≤d≤5. The two torsion points T_​1,T_​2 and O define its bad set B_​j. If B_​j−kR and B_​j−lR intersect for k≠l in0..5, then(k−l)R is19-torsion, contradicting the check. Thus each factor excludes at most one chart index. The five factors exclude at most five of the six indices. The Lean six_​chart_​cover theorem proves precisely this finite implication. The six translated product opens cover B, and their preimages cover A.")

#text("Let g_​(T−R,R) be the usual Miller line quotient and let v_​R=x+x(R). Then h=g/v_​R has divisor [T−R]−[-R]−[T]+[O]. Hence f_​T(P+R)/(h(P)^19 f_​T(P)) is constant. At O, h has leading term t and f_​T has leading term t^−19, both with coefficient1, so the constant is f_​T(R). The native calculation verifies this full rational identity, not only the divisor argument or point values. It also computes a 19th root of that constant in K and verifies its19th power.")

#text("For each generator i and factor j, write rho_​ij,k for the recorded root times h, with rho_​ij,0=1. Then F_​i(P+kR)/F_​i(P) is the19th power of product_​j rho_​ij,k(P_​j). On an overlap, the Kummer coordinate change is")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("z_i,l = z_i,k product_j rho_ij,l(P_j)/rho_ij,k(P_j).", block: true)]

#text("Here P denotes the common unshifted base point; in chart k use its explicit inverse translation to express P. Ratios telescope, so the overlap cocycle is exact, as proved in Lean. Each ratio is regular and invertible on the overlap: its19th power is a unit and the chart is normal. The base transitions are the explicit elliptic translations (l−k)R_​j. A separate parser checks all200 stored rational functions against the original functions, plus their inverse identities,100 curve identities and400 point translations.")

#text("The atlas is therefore explicit over K and covers the global rooted quotient. A closed projective embedding and effective F2 coordinate descent are still not supplied. The original full F2 quotient-equation requirement remains partial. This does not identify the polarized fivefold as a Jacobian.")
