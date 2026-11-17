#set page(paper: "a4", margin: (x: 19mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.40em)
#set heading(numbering: none)
#show link: set text(fill: rgb("245c91"))
#heading(level: 1)[#text("Round07 — independent norms, odd reverse divisor and nonzero-image rank6")]

#text("2026-10-19 UTC. Parent b582a94c2; rounds01–06 stay frozen. This round performs exact constructions and checks. It measures no IC costs.")

#text("Two complete full-map principal relations raise the nonzero-image rank from4 to5 and then6 for both signs. The final evaluated matrix has rank7/768, including the earlier row on44 zero-image columns. The nonzero-image submatrices have rank6 on675/678 columns. New columns have degrees60 and660; no individual descent or solver experiment is supplied.")

#image("routes.svg", width: 100%)

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Requested track")],
[#text("Result")],
[#text("Remaining gap")],
),
[#text("Independent forward norms")],
[#text("All7200 even reverse entries checked; actual modular targets and point-algebra traces return the source without Gram evaluation")],
[#text("Reduced genus2841 Jacobian arithmetic")],
[#text("Corrected odd reverse divisor")],
[#text("All20 fibers,2200 point-algebra entries; degree6600 each side; formal F2 descent and independent forward equality")],
[#text("General Jacobian representation/reduction and higher11-power fixture evaluation")],
[#text("Expanded determinant19 quotients")],
[#text("Four actual mixed translation atlases; all1444 kernel points checked; expanded supersingular[19] cover equations and exact identity")],
[#text("Intrinsic mixed five-dimensional invariant image ideal and full chart elimination")],
[#text("Relations beyond nonzero-image rank5")],
[#text("Second full-map norm raises both signs to rank6, with120 new columns and no geometric joins to prior columns")],
[#text("Further relation families, collection costs and individual descent")],
[#text("Lean")],
[#text("17 new named results;18 declarations audited without nonstandard axioms")],
[#text("Global modular/Picard geometry and polynomial-mask interpretation remain written")],
)

#heading(level: 2)[#text("Full-map functions distinguish the second7 choices")]

#text("Let P0 be the recorded order11 point of E49(F32), and let phi_​a be the twelve degree660 modular maps. The first function is")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("f0 = Norm_F32/F2(x(phi0)+x(P0)).", block: true)]

#text("Its target quintic is x^5+x^4+x^3+x^2+1. Its simple zeros are exactly the ten nonzero rational11-torsion points of E49(F32). Pulling them back gives55 closed columns of degree60 from the split seed and five of degree660 from the etale seed, total degree6600. The pole divisor is 10 phi0^*(O). Cusp images vanish after projection: the eight E49 cusps are F8-rational and killed by14, while the target group is killed by11. The complete row has coefficient1 on each of the60 new columns.")

#text("For the second function use")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("f1 = Norm_F1024/F2(x(phi1)+x(P0)).", block: true)]

#text("Frobenius moves its lines through2^j modulo11. Its complete degree13200 zero divisor has110 degree60 and ten degree660 columns. Split columns are joined by complete geometric basis keys; none overlap prior columns. For etale columns, nonconstant P coordinates and the nonconstant half-period transform −P+2Q certify the stated disjointness, with the Galois bridge written in the proof map. Multiplicities are retained.")

#table(columns: 4, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Evaluated population")],
[#text("All-column rank")],
[#text("Nonzero a=1 columns/rank")],
[#text("Nonzero a=0 columns/rank")],
),
[#text("Round06")],
[#text("5/588")],
[#text("510/4")],
[#text("520/4")],
[#text("After f0")],
[#text("6/648")],
[#text("565/5")],
[#text("570/5")],
[#text("After f1")],
[#text("7/768")],
[#text("675/6")],
[#text("678/6")],
)

#text("Unlike the previous x(P)+1 row, these new rows have nonzero new column images. Each complete principal sum is zero. Separate F32 arithmetic checks every new image and both sums; independent modular elimination agrees with the BigInt ranks. The change in column population is explicit: these ranks do not measure relation-generation or destination-solving cost.")

#heading(level: 2)[#text("Independent even and odd forward calculations")]

#text("For the even reverse divisor, reconstruct the modular target coordinates from seven/eleven kernels in the split field or each degree11 point algebra. Check the commuting isogeny squares and take elliptic point traces over the coordinate algebra. Exact deck and Frobenius permutations then evaluate all7200 entries of the20-fiber divisor. The resulting forward image equals the source generator and agrees with the prior Gram receipt. The Gram matrix is not used to compute these norms.")

#text("For the odd sign, twist an order11 E0(F32) point to S0 on E49(F1024). It satisfies Frob(S0)=7S0 and Frob^5(S0)=−S0. The correct full11 evaluation field is F2^660, with modulus x^660+x^10+x^8+x^5+1. The underlying F1024 curve has trace53; Frobenius modulo11 has order22. The twelve reverse neighbors yield55 split points and55 irreducible degree11 components, total geometric degree660.")

#text("Use the formally antisymmetric divisor [8T]−[−8T]. Its class is 16T=5T on11-torsion. The ten signed lines produce20 pullbacks with 2200 coordinate entries, positive and negative degree6600. Every term has explicit point-algebra coordinates. Seed Frobenius10 descent and the complete formal-divisor Frobenius permutation pass. All12 target maps are independently evaluated on every seed component, and the full forward image is the source odd trace-zero tuple. A separate parser checks all2200 coordinate entries, their curves, torsion and selected line. An independent F32 check confirms the final source is nonzero of order11 on E0.")

#text("These are unreduced divisors. Neither result implements general Jacobian reduction; the odd representative is explicitly specialized to11-torsion.")

#heading(level: 2)[#text("Expanded determinant19 geometry: what was obtained")]

#text("The four frozen graphs glue an ordinary fourfold to a supersingular elliptic cofactor of trace±2. The exact common torsion field is F2^360, with modulus x^360+x^5+x^3+x^2+1. Actual CM and companion bases have matching Weil pairing. Each361-point graph lies in the correct polarization kernel and passes its geometric Frobenius permutation. The records give all translation maps, the mixed projective atlas and factored invariant coordinate circuits. Specializations agree under both graph generators.")

#text("The supersingular cofactor cover is now expanded. For v²+v=u³+u+c, c=0 or1, native rational-function group arithmetic gives")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("[19](u,v) = (Nx(u)/Dx(u), v+Nr(u)/Dr(u)).", block: true)]

#text("The numerator/denominator degrees are361/360 and540/540. The artifact q5b-expanded-cofactor-equations.txt lists all four polynomials explicitly. On the declared affine chart the equations are")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("Dx(u) U + Nx(u) = 0,\nDr(u) (V+v) + Nr(u) = 0,\nv²+v=u³+u+c,   V²+V=U³+U+c.", block: true)]

#text("Exact polynomial arithmetic verifies the cleared curve identity. A Lean kernel certificate checks its binary polynomial masks; a universal Lean identity explains curve preservation under the exposed rational identity. All722 cofactor kernel checks and64 pointwise scalar comparisons pass.")

#text("These are expanded cofactor-cover equations for the mixed atlas. The full intrinsic invariant image ideal of the five-dimensional quotient has not been eliminated. That original requirement remains partial; a translation atlas and cover equations are not presented as a completed mixed ideal or a Jacobian equation.")

#heading(level: 2)[#text("Failures, proofs and replay")]

#text("The first odd adapter inherited a q-mod11 matrix entry and degree5 embedding modulus from the even F32 case. It produced no accepted fiber. The matrix and modulus are corrected, full group orders are checked before torsion extraction, and primary loops are bounded. Interrupted probes and the explicitly stopped stale probe are retained. A determinant19 invariant sample initially hit infinity; the corrected sample condition excludes kernel translates. A BigInt type-inference failure is also retained.")

#text("The ")#link("ledger.json")[#text("ledger")]#text(", ")#link("PROTOCOL.md")[#text("protocol and amendments")]#text(", ")#link("PROOF_MAP.md")[#text("proof map")]#text(" and ")#link("VALIDATION.md")[#text("validation")]#text(" identify exact requirements, checks and open bridges. Fresh replay compares50 new JSON records, expanded cover equations and generated Lean byte for byte. Round06 replays unchanged. Construction and checking are by the same agent; separate paths share primitives where stated. There is no external audit.")

#text("The universal identities carry certificates ")#link("identity-deck-line-transport.json")[#text("IDC1h4a9fbbb8fcf6dd5d")]#text(" and ")#link("identity-supersingular-coordinate-lift.json")[#text("IDC1h9c84bbd67b8b148a")]#text(", each with64 issuer and128 extra checks and a Lean source hash. The frozen protocol inherited America/Los_​Angeles timezone metadata from the prior round; the current user environment and this report use UTC.")

#text("Primary references for the written bridges are ")#link("https://arxiv.org/pdf/1110.6801")[#text("Kilford–McMurdy’s modular equations")]#text(", ")#link("https://www.hyperelliptic.org/HEHCC/chapters/chap04.pdf")[#text("Miller arithmetic")]#text(", ")#link("https://stacks.math.columbia.edu/tag/07S5")[#text("finite free quotients")]#text(", and ")#link("https://www.jmilne.org/math/CourseNotes/AV.pdf")[#text("Milne’s abelian-variety notes")]#text(". The new finite results and constructions above are repository calculations.")

#heading(level: 1)[#text("Formal proof map and written arguments")]

#text("Seventeen new named results and one generated equation pass the actual Lean4.34.1 kernel build and transitive audit. Standard foundations are propext, Classical.choice and Quot.sound; no native trust or custom axiom is used. The generated cofactor certificate audits with propext alone.")

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Report claim")],
[#text("Lean declaration or certificate")],
[#text("Additional bridge")],
),
[#text("Coordinate-algebra norms commute with forward maps")],
[#text("trace_​commutes, weighted_​trace_​commutes")],
[#text("Modular maps and finite etale trace interpreted geometrically")],
[#text("Exact target-line permutations")],
[#text("deck_​line_​identity, requested_​deck_​line")],
[#text("Cyclic kernel determines the transported modular quotient")],
[#text("Formally odd reverse representative")],
[#text("antisymmetric_​neg, antisymmetric_​class, antisymmetric_​equivariant")],
[#text("Elliptic Picard identification and pullback")],
[#text("Odd Frobenius shifts")],
[#text("odd_​scalar_​eigenvalues")],
[#text("Actual S0 coordinates, normalized full11 moduli and110 seed-component checks")],
[#text("Cusp images vanish")],
[#text("cusp_​image_​zero")],
[#text("Eight actual F8 cusps, modular maps preserve cusps, target order11")],
[#text("New row independence")],
[#text("new_​row_​independent")],
[#text("Complete support and nonzero new coordinates; ranks checked twice natively")],
[#text("Degree accounting")],
[#text("new_​zero_​degree, odd_​divisor_​degree")],
[#text("Complete ordinary etale fibers, written pole divisor")],
[#text("Mixed graph injectivity and invariant coordinates")],
[#text("graph_​injective, orbit_​product_​invariant")],
[#text("Actual kernel points, pairings, finite free quotient and polarized descent")],
[#text("Cofactor curve identity")],
[#text("cofactor_​coordinate_​lift, cofactor_​curve_​preservation")],
[#text("Native rational[19] recursion, nonzero denominators and scalar comparisons")],
[#text("Specific expanded polynomials")],
[#text("KoblitzRound07Cofactor.binary_​coordinate_​identity")],
[#text("Binary-mask encoding as F2 polynomials and rational-map interpretation remain written")],
)

#heading(level: 2)[#text("1. Independent norm calculation")]

#text("For an ordinary source curve, derive the eleven kernel x coordinates from the actual paired basis. Apply the normalized Velu x map to its first7 kernel, to the11 kernel across that7 isogeny, and to the second7 kernel. The two ways around the first square have equal b coefficient. The three j invariants reconstruct the unique cyclic49 modular point using degree8 gcds and the fourth root of the eta-parameter product. The target curve equation is checked. This uses the explicit modular formulas, not a Gram entry or a cardinality prediction.")

#text("On each irreducible degree11 component, all coordinates are polynomials modulo its recorded minimal polynomial over the coefficient field. Absolute Frobenius to that field’s degree fixes the curve and cycles its11 embeddings. Summing those11 elliptic points gives the geometric pushforward of the closed point; the result is checked to have constant coordinates. Split components are degree1. This group trace is a divisor pushforward, rather than a multiplicative field norm of x alone.")

#text("Normalized Frobenius on full level is (P,Q)↦(P²,Q²/2). Under (P,Q)↦(−aQ,a^−1 P+Q), the new b line is generated by (b a^−1)P+(b−a)Q. The universal coefficient identity and its selected-line specialization are proved in Lean. The native evaluator checks the actual coordinate changes and uses this exact permutation to reuse target values. Consequently all even and odd divisor entries are evaluated from actual target point norms. Comparing with the earlier even Gram receipt happens after the independent computation. No genus2841 reduction is inferred.")

#heading(level: 2)[#text("2. Odd divisor and descent")]

#text("The explicit odd point S0 has order11 and Frobenius eigenvalue7. Put Pi=(−1)^i Frob^i S0; this is the odd trace-zero tuple. In the adjoint formula the extra (−1)^i from the signed line cancels this sign. The target class to pull back is therefore5 Frob^i S0, with opposite coefficients on the two lines±2^i. For an11-torsion T, [8T]−[−8T] has class16T=5T. Unlike a translated representative, it changes sign as a formal divisor under T↦−T. Lean proves both statements separately and proves compatibility with every supplied additive homomorphism.")

#text("Since7^9=8 and7^4=−8 mod11, the two targets are Frobenius powers i+9 and i+4 of S0. Their20 pullback terms are permuted by F2 Frobenius, including the coefficient exchange at the five-step wrap. The native permutation is bijective. The split seed points and etale components pass Frobenius10 descent with simultaneous basis sign handled explicitly. This proves descent of the complete formal divisor, not only its class. All20 fibers have degree660, giving positive and negative degree6600.")

#text("The pointwise forward image is Pi. The old integral-adjoint argument therefore identifies this divisor class with the reverse map on the chosen 11-torsion generator and implies it is nonzero of order11. Global group-law and Picard bridges remain written. The particular antisymmetric scalar8 is not asserted to represent5T for arbitrary higher11-power torsion.")

#heading(level: 2)[#text("3. Complete principal rows and column identity")]

#text("For f0, the target polynomial is the Frobenius norm of x+x(P0). Its five roots are distinct and have degree5; the elliptic zeros are precisely±Pi. They are ordinary noncuspidal points. Full prime-to2 level is etale there, so each pullback consists of660 simple points. Frobenius5 cycles the660 split bases in55 cycles of length12. On the negative etale components it has five cycles of length12; each component has degree11. This accounts for55*60+5*660=6600 zeros without omissions. The only poles are 10 phi0^*(O), of the same degree.")

#text("For f1, Frobenius moves the line to2a. The canonical deck change above commutes with this move, so Frobenius10 on the seed is the square of its Frobenius5 permutation. Its cycles have length6. Closing the ten line conjugates gives110 degree60 and ten degree660 columns. Exact basis keys check every geometric split-point orbit, including possible intersections with prior columns; the output has no such joins or repeated split points.")

#text("For etale points, P on the old seed is rational over K=F2^120, while Q has degree11. The recorded nontrivial Galois step is Q↦Q+cP, c≠0 mod11. Thus a combination dQ+eP with d≠0 cannot be K-rational. After the new deck change Pnew=−Q is nonrational. The possible five-step half-period identification would make an old rational P equal to −P+2Q; its x coordinate is checked to be nonconstant on every relevant field component. This excludes overlap with old rational-P columns and between the two half-period families. Other period changes are excluded by the actual source modular point and component cycles. Degree and principal sums are independently checked after these identity decisions.")

#text("All modular cusp targets are among the eight E49 cusps over F8, killed by14. Each projected pole trace is also in the order11 group, hence zero; the coprime implication is kernel checked. This justifies dropping pole columns from both rows. Native BigInt and separate modular elimination verify rank7 on768 columns and rank6 on the nonzero675/678 populations. The generic Lean span lemma verifies why a nonzero new-column entry cannot come from rows zero on that column. It does not substitute for support or point-image verification, and no complete matrix-rank Lean theorem is claimed.")

#text("The final m9c checker explicitly verifies that all12 source modular points for each seed have degree60 and their five phase sets are disjoint. It also records the actual nontrivial complement translation on all60 negative components and checks the half-period abscissas there. These checks supply the orbit-degree and phase hypotheses used in the written argument above.")

#heading(level: 2)[#text("4. Mixed determinant19 quotient and expanded cover")]

#text("Let E be the ordinary factor, with O=Z[w], w²+w+2=0. Choose an actual 19-torsion CM basis (P,wP) and let its pairing be zeta. The supersingular cofactor E' has equation Y²Z+YZ²=X³+XZ²+cZ³ and trace−2 or+2. Its companion basis (S,Frob S) is adjusted by an explicit centralizer matrix until its pairing is zeta. This retains its companion Frobenius matrix. The four frozen graph matrices then have actual geometric coordinates, not merely unbound finite-module labels.")

#text("For each graph, all361 points lie in ker(H)×E'[19], are distinct by projection to E', and satisfy the specified Frobenius permutation. The retained alternating pairing computation makes the graph maximal isotropic for H⊕[19]. Lifting the CM basis to the19-adic lattice gives the same pairing normalization: division of its integral Riemann form by19 on the polarization kernel uses this zeta. This lattice-to-geometric commutator pairing argument is written, not a new full formalization.")

#text("The free finite etale translation quotient is an abelian fivefold. Its polarization descends and has degree1, since the source polarization has degree19^4 and the isogeny degree is19². The explicit atlas is E^4×E' over F2^360, with all361 translation transitions recorded. Its cyclic augmentation descent and the graph permutation descend it to F2. The orbit polynomial product_​g(U+sum Z_​j coordinate_​j(P+g)) gives invariant coordinate circuits on the declared stable affine chart; reindexing by each group translation is a Lean theorem. Finite specializations check the actual translations. They do not compute the intrinsic image ideal.")

#text("The cover over the quotient’s E' base is [19]:E'→E'. Native rational function group arithmetic expands it as X=Nx/Dx, Y=v+Nr/Dr. Its cleared identity is")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("Nx³ Dr² + Nx Dx² Dr² + (u³+u) Dx³ Dr²\n  + (Nr²+Nr Dr) Dx³ = 0  over F2[u].", block: true)]

#text("The generated Lean theorem checks this exact coefficient-mask arithmetic in the kernel, using binary shifts and xor, without native_​decide. The mask-to-polynomial interpretation is written; the separate universal ring lift and characteristic2 implication expose the curve-preservation bridge. The four expanded polynomials have degrees361,360,540,540 and coprime numerator/denominator pairs. Rational group recursion identifies the map as[19] on its chart; scalar comparisons and all361 kernel points per cofactor trace are further checks. The chart requires nonzero denominators; the group morphism extends projectively by the standard elliptic group law.")

#text("The full mixed invariant image ideal has not been eliminated. Computing the cofactor cover and its atlas does not discharge that requirement, and no Jacobian equation or genus reduction is supplied.")
