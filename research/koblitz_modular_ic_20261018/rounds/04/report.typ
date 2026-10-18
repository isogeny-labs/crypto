#set page(paper: "a4", margin: (x: 19mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.55em)
#set heading(numbering: none)
#show link: set text(fill: rgb("245c91"))
#heading(level: 1)[#text("Explicit polarized quotients and direct n=5 modular relations")]

#text("Round 04, 2026-10-18. Parent f5b90fb286dde2e8b0fa15b127a37514a8d64258. The requested geometry and modular-map tracks now have explicit additional constructions. This round evaluates a modular twisting correspondence on closed divisors and collects principal relations on the modular curve. Its relation matrix has rank 4 on 544 columns; no scalar recovery or timing experiment was attempted. The user assesses implications for IC.")

#image("routes.svg", width: 100%)

#heading(level: 2)[#text("Recorded outcomes")]

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Experiment")],
[#text("Result")],
[#text("Scope limit")],
),
[#text("Q1")],
[#text("14 explicit quotient bases, integral principal alternating forms and Frobenius matrices")],
[#text("Prime-to-2 polarization presentations; equations for individual nonnatural quotients remain missing")],
[#text("Q1 short vectors")],
[#text("Both distinct determinant-19 Hermitian forms have global minimum 3, with ten vectors of norm 3 each")],
[#text("Minimum on A5, not an indecomposability theorem for the glued fivefold")],
[#text("Q2")],
[#text("Natural quotient A5(E) x E -> Res(E), affine F2 equations and rational inverse for both signs")],
[#text("Target is a principally polarized fivefold; mixed projective charts use the abstract Weil-restriction gluing")],
[#text("Q2 kernel")],
[#text("All 25 geometric kernel points per sign over F_(2^24); 2,420 F2 rational additions checked")],
[#text("No curve/Jacobian realization follows")],
[#text("M1/M2")],
[#text("Actual X0(49) coordinates, two degeneracy maps and degree-11 j-correspondence through extension degree 10")],
[#text("A compatible j-pair alone is not transported level structure")],
[#text("M3b")],
[#text("24 supported F64 base inputs; twelve actual transported Hecke targets each; T11=[4]; fixed Weil pairing")],
[#text("18 ordinary nonzero-j inputs outside the selected trace classes remain unsupported; six j=0 inputs are outside this ordinary chart")],
[#text("M4")],
[#text("15,840 geometric modular points, 544 closed columns: 20 of degree 6 and 524 of degree 30")],
[#text("Selected finite divisor domain; no arbitrary Jacobian normal-form arithmetic")],
[#text("M4 images")],
[#text("510 nonzero columns for a=1, 520 for a=0, in their order-11 trace-zero groups")],
[#text("Forward factor projection, not a reverse subgroup embedding into the modular Jacobian")],
[#text("M4 relations")],
[#text("102 principal-function rows, rank 4 modulo 11")],
[#text("539 further independent ranks are needed for unique scale; no individual descent")],
[#text("M5")],
[#text("x(P)+1 has ordinary zero degree 9,240, with zero zeros represented in this factor base")],
[#text("Rejected collector extension; unresolved zeros and poles, no admitted row")],
[#text("Lean")],
[#text("Eleven new named structural theorems build and pass the transitive axiom audit")],
[#text("Modular moduli/descent and full divisor/polarization geometry remain written bridges")],
)

#text("The ")#link("ledger.json")[#text("ledger")]#text(", ")#link("PROTOCOL.md")[#text("protocol and amendments")]#text(", ")#link("PROOF_MAP.md")[#text("proof map")]#text(", ")#link("VALIDATION.md")[#text("validation")]#text(" and ")#link("REPLAY.sh")[#text("replay")]#text(" expose the inputs, exact scopes and remaining obligations. All research arithmetic and assembly are Rust. Python only renders this report and the recorded figure. Raw outputs, failed runs and the published modular-polynomial input are frozen in ")#link("runs.tar.xz")[#text("runs.tar.xz")]#text(".")

#heading(level: 2)[#text("Explicit polarized quotient geometry")]

#text("For every retained graph, write its integer top block as G=K*M and set B=[[ell*I8,G],[0,I2]]. The quotient lattice has basis B/ell. The inclusion matrix is Q=[[I8,-G],[0,ell*I2]]. Native exact checks give integral J'=B^t*diag(J_A,ell*J_E)*B/ell^2, determinant one, integral quotient Frobenius, and both polarization-pullback and Frobenius identities. The separate checker repeats these checks with BigInt matrices. The elliptic Frobenius must be its signed integer companion, not positive modular residues; the first failed run exposed that distinction.")

#text("The determinant-19 short-vector result is now global. For the doubled real Gram matrix S, every norm-at-most-three vector satisfies z_i^2 <= 6*(S inverse)_ii. Exact inverse cofactors force every coordinate into [-1,1]. Rational LDL enumeration gives ten norm-three vectors and no norm-one or norm-two vectors for each distinct form. A separate exhaustive box check, positive-minor checks and BigInt cofactors agree. The earlier round's bounded statement remains unchanged as historical evidence.")

#text("The natural determinant-five graph has actual quotient equations: q(a,Q)=a-diag(Q) maps A5(E) x E onto R=Res_(F32/F2)(E). Its kernel is (diag(T),T), T in E[5]. Over the algebraic closure the target is E^5 with its product principal polarization. Restriction to the augmentation lattice gives H=I4+ones4, while the diagonal block is 5 and mixed terms vanish. The report's universal pairing identity is proved in Lean.")

#text("In the pinned F32 basis, expand X=sum(X_i*z^i), Y=sum(Y_i*z^i) and the elliptic equation. Q2 records the five resulting F2 polynomials in ten variables. It checks every affine solution (43 and 21), all rational points (44 and 22), all additions, and all eleven subgroup points. Infinity uses its identity chart. The full Weil-restriction scheme supplies mixed charts; a single affine patch is not represented as a complete projective embedding. The inverse uses the trace to remove the diagonal component. Full five-torsion and all 1,250 kernel additions are checked again with the maintained binary library over F_(2^24).")

#text("This supplies one explicit polarized quotient for each sign. The other individual graph quotients, especially the determinant-19 cases with supersingular elliptic cofactor, still have integral polarization data rather than explicit coordinate equations. A complex uniformization for an ordinary variety cannot simply be asserted for those mixed products.")

#heading(level: 2)[#text("Actual modular transport and factor projections")]

#text("The modular base is E1=X0(49)/F2: Y^2+XY=X^3+X^2+1. The integral change from the minimal model to eta coordinates reduces to x=X, y=(X^2+X+1)*Y+X+1. This avoids reducing a formula with denominator 2. Its equation and both characteristic-zero and characteristic-two polynomial identities are checked in Lean. The degeneracy parameters are x^4/y and y. The two maps to j are (t^2+t+1)^4/t^7 and (t^2+t+1)^4/t. These come from ")#link("https://arxiv.org/abs/1110.6801")[#text("Kilford--McMurdy, section 2")]#text(".")

#text("M3 transports the two successive cyclic-seven kernels across all twelve cyclic-eleven isogenies, using actual kernel polynomials and characteristic-two Velu x-maps. It checks the commuting square, the published ")#link("https://math.mit.edu/~drew/ClassicalModPolys.html")[#text("Sutherland degree-11 polynomial")]#text(", reconstruction on X0(49), and the complete T11=[4] sum. The paired extension uses Miller functions, alternation and a bilinearity control to put every basis at the same primitive eleventh root. The unpaired output is preserved and is not used for relations.")

#text("The modular source C is the conjugated full-eleven-level model over X0(49), with canonical determinant-normalized descent. Its analytic group is conjugate to X_H(5929), d congruent to +/-1 modulo 11, of genus 2,841. The explicit moduli/descent argument is in the proof map. Enumerating SL2(F11)/{+/-I} gives all 660 level bases per supported ordinary base point. Canonical Frobenius, including the quadratic-twist correction of the universal elliptic model, produces the 544 closed-point columns. Every one of the 15,840 geometric inputs satisfies the computed twisting-tuple equivariance.")

#text("Even sums of Hecke images give the order-five character; signed odd differences give the order-ten character. Integral augmentation and the twisted Frobenius trace evaluate their images in A5(F2). The odd character uses the quadratic isomorphism over F4 before descending to E0(F32). The map outputs actual points, not just Frobenius-module labels. The Gauss-sum identification with the twisted newform is a written geometric bridge using ")#link("https://doi.org/10.1007/BF01390245")[#text("Atkin--Li twisting")]#text(", not a new full Lean modularity theorem. The finite evaluator supports the recorded closed divisors; arbitrary Jacobian-class reduction, the reverse map and saturation at 11 remain separate requirements.")

#heading(level: 2)[#text("Direct modular relations and the observed failure")]

#text("For lines and verticals on X0(49), take their norm from F64 to F2 and pull back the entire divisor to C. Every admitted ordinary fiber includes all 660 level points. Partial rational fibers are rejected. Modular cusps map to cusp points on X0(49), killed by 14, so their projected order-11 images vanish; the coprime-torsion implication is checked in Lean. Multiplicities, functions and full rows are retained, not inferred from a zero point sum.")

#text("The 102 resulting rows are verified against both sets of actual projected points. A separate checker reconstructs every row, repeats its group sum with the maintained binary library, and computes rank 4 with BigInt forward elimination. Their dependence on only four closed base fibers explains the rank ceiling, now proved as a general matrix theorem in Lean. This collector cannot reach rank 543 by collecting more functions with the same four-fiber support pattern.")

#text("The next function, x(P)+1, depends on the eleven-level section and could break that pattern. Its exact division polynomial at x=1 has degree 15 in b, with no roots in F64. Its complete ordinary zero degree is 15*56*11=9,240; the current factor base contains none. This attempt therefore admits no row. The next discriminating work is to factor that b-polynomial, construct the missing cyclic49 and complementary-eleven orbits, and resolve the poles. A vanishing image of a partial divisor would not repair the missing support.")

#heading(level: 2)[#text("Validation, failures and outstanding work")]

#text("The separate checker verifies 14 quotient modules, two global short-vector certificates, 1,250 geometric-kernel additions, 24 Hecke/basis controls, 1,088 projected point classes, all 544 closed orbits, all 102 principal rows and independent rank 4. Native tests and the full repository library pass. Seven frozen output files replay byte for byte. Eleven named Lean theorems build and audit with standard foundations only. The PDF includes the full proof map and is inspected page by page.")

#text("Failed receipts include an incorrect integer lift, the degree-one Rabin edge case, a projective-point counting error in the first Hecke check, first Lean elaborations, a ledger-builder borrow error and a replay mismatch from newly added unpaired metadata. Corrections preserve earlier outputs; matching historical serialization is restored. The rank deficiency and incomplete interior-function support are mathematical/collector outcomes, not build failures or nonexistence claims.")

#text("The open requirements are explicit equations/decomposition for nonnatural quotients, full formalization of the modular/Picard bridges, a reverse order-11 subgroup transfer or optimal saturation certificate, richer direct relations and individual descent. No recovery or speed conclusion follows from the current rank. The ")#link("../../../../problems/koblitz-modular-carrier-transfer.md")[#text("shared problem")]#text(" remains open. Canonical carrier summaries are updated; performance charts have no new measured quantity to add. Named next experiments are not background jobs.")

#heading(level: 1)[#text("Formal proof map and written arguments")]

#text("The pinned Lean 4.34.1/mathlib project now includes ")#link("../../lean/KoblitzRound04.lean")[#text("KoblitzRound04.lean")]#text(". Eleven new named theorems build and pass the transitive axiom audit. They use only propext, Classical.choice and Quot.sound, or a subset. Failed proof elaborations are archived; they are not claimed as proofs. No sorry, custom research axiom or trusted native evaluation is used in the final module.")

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Report claim")],
[#text("Lean theorem")],
[#text("Exact scope and missing bridge")],
),
[#text("Integral X0(49) coordinate change")],
[#text("modular49_integral_change")],
[#text("Universal commutative-ring identity; identifying the modular eta functions uses the published model")],
[#text("Characteristic-two specialization")],
[#text("modular49_binary_change")],
[#text("Exact identity over every commutative ring of characteristic 2; rational-map chart/gluing remains geometric")],
[#text("Natural quotient polarization pullback")],
[#text("natural_polarization_pullback")],
[#text("Universal bilinear polynomial identity with the fifth coordinate equal to minus the other four; Picard/polarization identification external")],
[#text("Natural quotient kernel")],
[#text("natural_quotient_kernel")],
[#text("All elements of every additive commutative group, trace-zero five-tuple; kernel is diagonal five-torsion")],
[#text("Rational inverse")],
[#text("natural_quotient_rational_inverse")],
[#text("Additive trace/diagonal maps with trace*diagonal=id; on F2 points this follows from five acting as one on E(F2)")],
[#text("Integral augmentation has zero sum")],
[#text("augmentation_sum_zero")],
[#text("Every five-tuple in every additive commutative group")],
[#text("Six-term Frobenius trace is fixed")],
[#text("trace6_fixed")],
[#text("Additive endomorphism and actual period-six input; identifying the twisted modular Frobenius is a separate bridge")],
[#text("Inverse-Gram coordinate bound")],
[#text("dual_coordinate_bound")],
[#text("A real quadratic expression nonnegative for every real t, positive dual norm; positive Gram/inverse specialization supplied by exact certificates and the written argument")],
[#text("Quadratic-twist substitution")],
[#text("binary_twist_substitution")],
[#text("Characteristic 2, s squared+s=1, arbitrary x,y,a,b; supplies the actual curve-equation substitution")],
[#text("Cuspidal projected images vanish")],
[#text("coprime_cusp_image_zero")],
[#text("Every group element killed by both 14 and 11; geometric cusp-image orders are the external premise")],
[#text("Four-fiber rank ceiling")],
[#text("fiber_relation_rank_bound")],
[#text("Every field and every finite matrix factoring through four columns; no empirical rank hypothesis is assumed")],
)

#heading(level: 2)[#text("Quotient basis, positivity and the explicit natural case")]

#text("For the first graph of each nonempty gluing case let G=K*M. The subgroup has coordinates (G*z,z)/ell in the product Tate module. Its full inverse image is the lattice with basis B/ell, B=[[ell*I,G],[0,I]]. This basis contains both the old lattice and the graph, and its index is ell squared. The old lattice inclusion is Q=[[I,-G],[0,ell*I]]. Thus Q^t*J'*Q=J_total, where J'=B^t*J_total*B/ell squared. Kernel isotropy supplies its integrality. Its determinant is one, and Frobenius is B inverse*F_total*B, checked integral using F_A*G-G*F_E divisible by ell. These identities, including F'^t*J'*F'=2J', are exact native certificates, not full Lean matrix proofs. For supersingular cofactors this presentation is prime-to-two Tate data; it is not asserted to be an ordinary complex-lattice realization.")

#text("For positive real S and a vector z, apply positivity to z-t*S inverse*e_i. It gives 0 <= z^t*S*z-2*t*z_i+t squared*(S inverse)_ii. The generic Lean quadratic lemma implies z_i squared <= (z^t*S*z)*(S inverse)_ii. Here S is twice the real Hermitian Gram, so norm at most 3 gives the factor 6. BigInt determinant/cofactor and positive-minor checks certify the bounds. Exact rational LDL search and a separate complete box enumeration agree: each of the two forms has ten norm-three vectors and none of smaller positive norm. These computations prove the stated minimum with the written positivity-to-lattice bridge; no indecomposability of the final fivefold is inferred.")

#text("For the natural graph let A be the kernel of sum:E^5->E after geometric base change, with its F2 permutation descent. The map q:A x E->E^5, (a,Q)->a-diag(Q), is surjective as an isogeny, and its kernel is (diag(T),T), 5T=0. The five-torsion is etale of order 25 in characteristic2. The abstract additive kernel statement is proved in Lean; the isogeny and etale assertions use standard abelian-variety geometry. The product principal polarization pulls back to H=I4+ones4 on A and [5] on E; the mixed terms cancel since sum(a_i)=0. Its degree is 25 squared, so descent through the kernel gives the product principal polarization on the quotient. The target's F2 form is R=Res_(F32/F2)(E), with the descended product polarization. This describes the complete quotient geometrically, plus an explicit affine coordinate patch and identity chart. It is not a fully expanded projective ideal for every mixed chart.")

#heading(level: 2)[#text("Integral modular model and actual isogeny transport")]

#text("")#link("https://arxiv.org/abs/1110.6801")[#text("Kilford--McMurdy, section2")]#text(" gives the eta model and degeneracy parameters. Starting from Y^2+XY=X^3-X^2-2X-1 over the integers, put x=X-2, D=x^2+7x+7 and y=D*Y+4x^3+22x^2+35x+7. The universal identity in Lean has the eta residual equal to D squared times the Weierstrass residual. Both sides have integral coefficients. Specializing at 2 gives the explicit binary eta map without dividing by 2. D=0 is an exceptional inverse chart, not a discarded point; the base enumeration retains these points and the Fricke involution is evaluated by elliptic addition there.")

#text("For ordinary underlying j, normalize the elliptic model to v^2+u*v=u^3+a*u^2+b, b=1/j. Its x-coordinate doubling formula is u squared+b/u squared. A cyclic odd-order kernel with pair-root polynomial k has characteristic-two Velu x-map u+u*k'/k+u squared*(k'/k) squared. Writing T for the sum of the pair roots, its quotient has a4=T, a6=b+T; translating v by T normalizes a4 to zero and changes b to b+T+T squared. The integer-lift certificate records the residual multiple of 2. This Velu construction and Miller's divisor algorithm are standard geometric bridges; the present Lean module does not formalize them.")

#text("The code reconstructs both degree-seven kernels from the three j-values of an X0(49) point. Division-polynomial divisibility, separability and doubling invariance check each kernel. Applying the eleven x-map to these kernels is implemented as the characteristic polynomial in their quotient algebras. The transported eleven kernel on the first seven quotient gives an independent commuting-square equality. The native published-polynomial test, curve equation and complete Hecke sum are additional finite controls. The separate maintained library checks basis orders/independence, every transported target, and T11=[4] for all 24 supported inputs.")

#heading(level: 2)[#text("Modular source, canonical Frobenius and the twisted factor")]

#text("Let C be the determinant-normalized full-eleven-level cover of X0(49). A point over a field containing mu11 is represented by an ordinary base point with its cyclic49 subgroup and an eleven-torsion basis (P,Q) of fixed Weil pairing zeta, modulo simultaneous sign. There are |SL2(F11)|/2=660 such bases at each ordinary nonelliptic base point.")

#text("Conjugation by diag(11,1) takes [[a,b],[c,d]] with c divisible by 49*121 and d=+/-1 modulo 11 to [[a,11*b],[c/11,d]]. The latter group is Gamma0(49) intersect +/-Gamma(11). On moduli the conjugation quotients by the first cyclic11 subgroup and transports the coprime49 structure. The quotient subgroup and the dual generator give the full basis, with the determinant recording its Weil pairing. This supplies the correspondence with the X_H(5929) model; the normalization of the determinant specifies its F2 descent, not just its complex analytic group. This scheme/moduli argument is written, not Lean.")

#text("Under F2 Frobenius, canonical normalization is (P,Q)->(P^2,Q^2/2), where division by 2 means multiplication by 6 modulo 11. The pairing transforms as zeta->zeta squared and is restored by that factor. Changes between the chosen universal elliptic twists are explicit y-shifts. The actual permutation of all 15,840 supported geometric points has20 orbits of length6 and 524 of length30. No geometric point is omitted from these selected complete fibers.")

#text("Let phi_a be the actual transported X0(49) point associated to the line P+aQ, including the two fixed lines a=0,infinity. For i modulo 5 set D_i^+=phi_(2^i)+phi_(-2^i), and D_i^-=(-1)^i*(phi_(2^i)-phi_(-2^i)). Replace D_i by 5D_i-sum(D_j) to land in augmentation without division. Canonical modular Frobenius transforms the even tuples by sigma followed by inverse cyclic shift, and the odd tuples by its negative. All these equalities are checked at every geometric point, with both trace-zero and order conditions. Their closed-point traces evaluate actual A5(F2) points.")

#text("The standard Gauss-sum formula for the weighted degeneracies identifies these with the primitive order-five/order-ten twists of the conductor49 form, up to Galois conjugacy and the explicitly stated integral scaling. ")#link("https://doi.org/10.1007/BF01390245")[#text("Atkin--Li")]#text(" supplies the newform twisting framework. The characteristic-zero correspondence and its prime-to2 level reduction supply the global Picard map. Identification of that map with the optimal modular quotient, its polarization and saturation at 11 are not computed here. The nonzero finite images do not establish a reverse subgroup-to-Jacobian embedding on eleven-torsion.")

#text("For the odd sign, choose s in F4 with s squared+s=1 and apply (x,y)->(x,y+s*x). The Lean substitution proves the twist equation. The twisted fixed-point condition then makes both coordinates F32-rational. Their norm to E(F2) vanishes. Since E0(F32) has order 44, E1(F32) has order 22, and the respective base orders are4 and 2, the norm kernels have order 11. The maintained library checks every one of the 1,088 recorded projected points and its eleven-torsion condition. Labels for a planted scalar are never used in relation construction.")

#heading(level: 2)[#text("Genuine principal rows, cusp removal and the rank ceiling")]

#text("Take a line or vertical function on E1=X0(49) overF64. Its norm toF2 is a rational function; its full pullback to C has a principal divisor. At each admitted ordinary base zero the entire degree 660 fiber is represented. Base points outside the supported set are rejected, even if a partial projected sum might vanish. The polynomial zero multiplicities determine the row, with coefficients constant on the four supported closed base fibers. The separate verifier reconstructs these rows from their functions and root multiplicities, then checks their point sums using a different binary arithmetic implementation and rank using BigInt elimination.")

#text("Every transported cusp is an X0(49) cusp. These are defined overF8 and killed by 14 in E1(F8). Consequently a projected closed cusp class in A5(F2) is killed by 14 and 11 and vanishes; this coprime implication is proved in Lean. A compatible Tate cusp supplies a degree-zero anchor, with zero image. The construction of the anchor and all cusp charts is geometric, not formalized or implemented as arbitrary C divisor-class arithmetic.")

#text("Every row factors as W times the four-fiber incidence matrix. The Lean rank lemma proves its rank is at most4 over every field. The computed rank 4 therefore exhausts this row pattern; collecting more of those pullback functions cannot supply the missing539 ranks. A level-dependent function is the next discriminating family, not a supplied cross-row control.")

#text("For x(P)+1 the native recurrence gives psi11(1,b) of degree 15, constant coefficient nonzero and no F64 roots. Each ordinary b-root supplies a nonzero point P up to sign,11 complementary paired bases and 56 cyclic49 subgroups. This gives full zero degree 9,240, with multiplicities inherited from the polynomial. None is in the current factor base. Poles also need resolution. The proposed row is rejected rather than substituted for a complete principal divisor. This moduli-degree argument is written; the polynomial and empty-support checks are native exact evidence.")

#heading(level: 2)[#text("Identity records and remaining formalization")]

#text("The loose integer identity certificates cover the integral modular change, natural polarization pullback and Velu normalization lift. Their identifiers are listed in ledger.json and cited below after issuance. Each receives 64 issuer points and 128 extra checker points. The first two have direct Lean source bindings. The third retains its multiple-of-two residual and has no misleading Lean attachment.")

#text("Full moduli/Picard geometry, polarization descent, Weil/Miller correctness, actual divisor fiber completeness as a scheme theorem, and the reverse projector/composition remain formalization obligations. The finite tests and structural Lean statements are not presented as those full theorems.")

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Identity certificate")],
[#text("ID")],
[#text("Kernel proof binding")],
),
[#text("")#link("identity-modular49-change.json")[#text("Integral modular change")]#text("")],
[#text("IDC1h5c93d35226502b13")],
[#text("Direct universal statement, modular49_integral_change")],
[#text("")#link("identity-polarization-pullback.json")[#text("Polarization pullback")]#text("")],
[#text("IDC1h7a01e3d29f1f37ab")],
[#text("Direct universal statement, natural_polarization_pullback")],
[#text("")#link("identity-velu-integer-lift.json")[#text("Velu integer lift")]#text("")],
[#text("IDC1h5d4af0a173c60eeb")],
[#text("No direct Lean attachment; residual 2*(y*T+T squared) is explicit")],
)
