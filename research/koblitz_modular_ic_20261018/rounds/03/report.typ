#set page(paper: "a4", margin: (x: 19mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.55em)
#set heading(numbering: none)
#show link: set text(fill: rgb("245c91"))
#heading(level: 1)[#text("Koblitz carriers: twists, polarized gluing, and n=5 relations")]

#text("Round 03, 2026-10-18. Experiment parent d6c722e19a381f2b6178b6d5d75ce6a87aea4335. The user's requested three tracks were executed as separate bounded experiments, with failures preserved and thirteen new Lean lemmas. No performance measurement or IC efficiency assessment was made.")

#text("The two missing del Pezzo twist searches are complete within their specified linear systems. Integral gluing now has explicit positive forms, polarization kernels, pairings and anti-isometry graphs. The n=5 same-field cubic-cover control recovers all 22 public fixtures from geometric principal-function relations, after three weaker collections exposed a rank deficiency. An evaluated F2 modular-Jacobian map and its interior relation collector remain open.")

#image("routes.svg", width: 100%)

#heading(level: 2)[#text("Recorded results")]

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Experiment")],
[#text("Actual scope")],
[#text("Result")],
),
[#text("DP3")],
[#text("Types(5),(3, 2), 65,535 sections each, six extension counts")],
[#text("131,070 sections; no A5 polynomial candidate for either sign")],
[#text("GL2")],
[#text("2,401 integral circulant parameters, coefficients in[-3, 3]")],
[#text("15 positive; five determinant 5 forms before sign duplication; scope misses four congruence cosets")],
[#text("GL2b")],
[#text("Five congruence cosets, 12,005 forms")],
[#text("29 positive; 24 retained sign-specific candidates:10 determinant 5, 14 determinant 19")],
[#text("Gluing graphs")],
[#text("All elliptic cofactor traces in[-2, 2]")],
[#text("10 determinant 5 cases with 60 anti-isometries; 4 determinant 19 cases with 80 anti-isometries")],
[#text("GL4")],
[#text("All6,560 nonzero eight-coordinate vectors in[-1, 1] per determinant 19 gluing case")],
[#text("Four cases, minimum 3 within the box, no norm-one witness; global minimum and indecomposability unresolved")],
[#text("GL3")],
[#text("Published genus 5 census, three strata")],
[#text("7,829 point-count records; no A5 match")],
[#text("DP4")],
[#text("Published genus 6 census")],
[#text("38,327 isogeny classes, 72,227 curves; no A5 match; all label/Newton roundtrips pass")],
[#text("G4")],
[#text("Putative genus 4 polynomial for both signs")],
[#text("Positive sign N1=4>N2=2; negative sign N5=77>published maximum 71")],
[#text("MR3")],
[#text("Determinant-compatible character screen, five primes, n=3, 5,131")],
[#text("n=5 fails at 3 and 5; n=3 controls pass; scope remains a necessary character-pair condition")],
[#text("N5d,a=0")],
[#text("43 rational-point columns; cubic, vertical and monic-quartic functions over F32")],
[#text("1,770 rows, rank 42 modulo 11, 11/11 fixtures recovered")],
[#text("N5d,a=1")],
[#text("21 columns; add Hermite-generated quartics")],
[#text("81 rows, rank 20 modulo 11, 11/11 recovered; 34,804 subdivisions give22,514 unique interpolants and 60 additional rows")],
[#text("Formal checks")],
[#text("New module plus prior proofs")],
[#text("13 new named Lean lemmas build; 14 declarations audited including the defining equation; standard foundations only")],
)

#text("The ")#link("ledger.json")[#text("ledger")]#text(" contains the input/source hashes, requirements, failed and partial runs, and next experiments. The ")#link("PROTOCOL.md")[#text("protocol")]#text(" records each added test before execution. ")#link("REPLAY.sh")[#text("REPLAY.sh")]#text(" reproduces the eight final outputs and runs separate verifiers. The ")#link("runs.tar.xz")[#text("raw archive")]#text(" and ")#link("runs.sha256")[#text("file hashes")]#text(" preserve intermediate failures and published inputs. ")#link("VALIDATION.md")[#text("VALIDATION.md")]#text(" records checks and their actual outcomes.")

#heading(level: 2)[#text("The two del Pezzo searches")]

#text("The five-point conic construction realizes both missing Frobenius types. The degree-five anticanonical system consists of quintics double at five conic points. Its square consists of decics quadruple there; the respective native dimensions are 6 and 16. Count the zero divisor on the blowup, using order-four Hasse jets at exceptional fibers, then contract the conic. The independent conic check verifies that restriction is either zero everywhere or nowhere zero, including exceptional tangent directions. The correction is subtract q when the whole conic lies in the zero divisor.")

#text("There are 72 direct section-count controls, 12 surface-count controls and 264 conic-fiber vector checks across the two types and six fields. Every section is tested for both A5 signs. Singular/reducible sections remain in the enumeration; a smooth carrier would necessarily pass the polynomial test, and none does. Together with round02, all seven del Pezzo Frobenius types have now been tested in these anticanonical-square systems. This alone does not classify every genus-six curve. The construction and geometric argument are given in the ")#link("PROOF_MAP.md")[#text("proof map")]#text(", with a primary-source link.")

#heading(level: 2)[#text("Polarized gluing: explicit progress and remaining geometry")]

#text("GL2 starts with the actual augmentation lattice of E to the fifth power, with endomorphism order O_-7. An initial integral circulant sublattice misses four congruence classes. GL2b includes them by dividing the restricted circulant matrices by 5 after explicit integrality checks. It finds small positive forms of determinants5 and 19; it finds none of determinant 1 or11 within its stated coefficient range. That bounded absence is not a general polarization obstruction or a search over all isogeny representatives.")

#text("For each retained form the record contains H, its positive leading minors, J, the actual Frobenius matrix, kernel basis, kernel pairing and all elliptic cofactor Hom/anti-isometry counts. The separate checker uses integer Bareiss determinants and exhaustive two-by-two matrix enumeration, avoiding the generator's Hom-nullspace solver. It confirms 24 forms, 24 kernel pairings and 14 nonempty graph cases. Four determinant 19 cases have20 anti-isometries each, matching elliptic trace-2 for target trace-1 and trace+2 for target trace+1. The other ten determinant 19 cases do not match any F2 elliptic cofactor in the five tested isogeny classes.")

#text("These graphs are Frobenius-stable and maximal isotropic in the finite polarization kernel, with trivial factor intersections. Under the standard Hermitian-to-polarization identification and polarization-descent theorem, they specify principally polarized quotients. The ")#link("PROOF_MAP.md")[#text("proof map")]#text(" exposes those geometric bridges and the exact finite evidence. No quotient coordinate equations, curve equation or Jacobian realization was constructed. The genus 5 census result below concerns the entire isogeny classes, so a principally polarized fivefold from this route is not automatically a Jacobian. The bounded norm-one search supplies no decomposition witness; it does not prove these polarizations indecomposable.")

#heading(level: 2)[#text("Full-A5 carriers through genus 6: conditional census conclusion")]

#text("The native genus 5 lookup scans 1,070 hyperelliptic, 2,854 trigonal and 3,905 complete-intersection vectors in the pinned ")#link("https://github.com/DusanDragutinovic/MT_Curves")[#text("Dragutinovic census")]#text(". All have integral Newton polynomials, and none is divisible by either A5 polynomial. The native genus 6 lookup scans every exported isogeny class in the pinned ")#link("https://github.com/junbolau/genus-6")[#text("Huang--Kedlaya--Lau census")]#text(", including strata outside our del Pezzo searches. It verifies all encoded coefficient labels and recovers every polynomial from its point counts; there are no A5 matches. No Sage or Python research program was executed.")

#text("At genus 4, the positive sign is excluded by N2>=N1. The negative sign has N5=77, whereas ")#link("https://arxiv.org/abs/2007.07822")[#text("Xarles, Example 16")]#text(" gives maximum 71 among F2-defined genus-four curves. Smaller genera cannot contain the four-dimensional full factor. Thus the combined evidence excludes F2 Jacobian carriers containing the full A5 factor through genus 6, conditional on the published census results and maximum. This is not an independent Lean classification and not a bound on arbitrary finite-subgroup representations or other base fields. It does not extrapolate to n=131.")

#heading(level: 2)[#text("n=5 maps and geometric relations")]

#text("The source curves are E_a/F32: Y squared+X*Y=X cubed+a*X squared+1. They have group orders44 and 22, with unique order-eleven subgroups. The augmentation embedding is the Frobenius tuple (P,F(P),...,F to the fourth(P)). All eleven points and 121 additions per sign are evaluated and checked for Frobenius-fixedness and trace zero. The glued quotient and modular-Jacobian coordinates are not evaluated by this representation.")

#text("The concrete relation control uses the existing cubic map C_a/F32: v^2+u^2*v=u^7+a*u^4+u -> E_a, (u,v) -> (u^3,u*v+1). The source curve has genus 3. Pullback is nonzero and injective on the entire order-eleven subgroup, checked directly and supported by the written prime-to-three push-pull argument. Every target is represented by a reduced Mumford pair, including nonsplit fibers. The field is F32, not F2.")

#text("The initial collector exhausts all 32 to the fourth power polynomials of degree at most3. For a=0 this suffices; a=1 yields only ten rows of rank 10 on21 columns. Adding vertical functions raises that rank to 16. The selected monic-quartic family adds no a=1 rows. These failures are frozen. Finally, Hermite interpolation enumerates all allowed effective degree-five rational subdivisors, covering the missing completely split quartic functions. It adds 60 rows and reaches the unique-scale condition.")

#text("The final solver anchors a nonzero projected factor-base class, verifies every projected log using independent library arithmetic, and performs bounded individual descent with principal-function witnesses. It uses at most32 trials per fixed offset and at most eight point offsets after the unshifted attempt. All22 fixture answers pass elliptic multiplication. The separate verifier reconstructs1,851 principal rows, recomputes ranks42 and 20 with BigInt elimination, checks 64 projected classes, and checks all 22 individual descents. Fixture labels enter final comparison only; no planted logarithm is a matrix row or solver input.")

#text("This supplies an executable n=5 map/relation control over F32. The requested evaluated F2 modular factor and its interior relations remain unresolved. General F32 Jacobian-class pushforward is unsupported in the reused library; we do not claim that norm capability. The determinant-compatible screen removes a narrower Eisenstein character-pair possibility at n=5: with the determinant condition, u squared=-p and u^D=1 imply (-p)^(D/2)=1. For D=2310 the p=3, 5 tests fail modulo 11. This is not a complete theorem excluding every modular-unit, cusp or interior relation.")

#heading(level: 2)[#text("Proof coverage, failed attempts, and next experiments")]

#text("The ")#link("PROOF_MAP.md")[#text("proof map and written arguments")]#text(" state all thirteen new Lean declarations and their exact hypotheses. They cover the normalized character square, halved exponent, CM norm, alternating determinant, anti-isometry graph isotropy, both graph intersections, quotient injectivity, cubic substitution, conic intersection arithmetic, conditional adjunction, and two genus 4 count obstructions. The first proof attempts failed; the final module and transitive audit pass. Geometry and census completeness are not hidden as custom axioms.")

#text("The first N5 build needed borrowed point coordinates. The initial Lean file-write used the wrong directory; its following build did not build any round03 theorem. Two later Lean builds failed product subtraction/zero simplification. All are retained. N5c's historical polynomials_tested field reports1,048, 576 although two families, 2,097, 152 per sign, were executed; N5d corrects that metadata without rewriting the old file. The matrix-rank failures and bounded short-vector negative are results, not nonexistence proofs. Generation and verification were performed by the same agent using separate algorithms; external independent replay has not been claimed.")

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Requested track")],
[#text("Status")],
[#text("Next discriminating work")],
),
[#text("Remaining del Pezzo twists")],
[#text("Specified searches verified complete")],
[#text("Full-A5 F2 carriers in nonexcluded higher genera; retain census assumptions")],
[#text("Polarized gluing")],
[#text("Integral forms and finite graphs verified; geometric realization partial")],
[#text("Complete short-vector enumeration from inverse-Hermitian bounds; classify quotient polarizations and seek explicit equations")],
[#text("n=5 maps and relations")],
[#text("Both same-field subgroup controls verified; modular evaluator open")],
[#text("Actual divisor evaluator/projector on the specified n=5 modular factor, with subgroup and exceptional-point checks")],
[#text("Lean proofs")],
[#text("Thirteen new named lemmas checked; full geometry partial")],
[#text("Instantiate Picard/divisor certificate, polarization and modular-cusp bridges")],
[#text("IC efficiency")],
[#text("Not measured in this round")],
[#text("User assesses implications and selects any subsequent timing protocol")],
)

#text("These are named follow-up experiments, not background jobs. The shared ")#link("../../../../problems/koblitz-modular-carrier-transfer.md")[#text("carrier-transfer problem")]#text(" remains open. Canonical carrier summaries are updated; timing graphs and performance leaderboard rows have no new measured value to add.")

#heading(level: 1)[#text("Formal proof map and written arguments")]

#text("Lean 4.34.1 and mathlib d13f23b723b8a846827a245b89c10fc7d3f11612 remain pinned. The new source is ")#link("../../lean/KoblitzRound 03.lean")[#text("KoblitzRound 03.lean")]#text(". Thirteen new named lemmas compile. The transitive audit counts 14 declarations, including the equation generated for intersection. They use only propext, Classical.choice, Quot.sound, or no axioms. There is no admitted proof, custom research axiom or native evaluation bypass in the final source. The earlier 13 named lemmas remain in their original module. Failed builds and their temporary sorryAx diagnostics are retained as failed receipts. lean4checker remains unavailable; the actual Lean kernel and axiom auditor run.")

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Claim")],
[#text("Lean declaration")],
[#text("Exact hypotheses and external bridge")],
),
[#text("Determinant compatibility gives a normalized square")],
[#text("determinant_normalized_square")],
[#text("Field; chi nonzero, epsilon*eta=chi squared, epsilon+p*eta=0; identification of Dirichlet/Hecke characters is external")],
[#text("The necessary exponent can be halved")],
[#text("determinant_half_exponent")],
[#text("Commutative ring; u^(2m)=1 and u squared=-p imply (-p)^m=1")],
[#text("CM norm formula")],
[#text("cm_norm_identity")],
[#text("Commutative ring; w squared+w+2=0; conjugation and actual elliptic endomorphism ring are external")],
[#text("Alternating pairing transforms by det(M)")],
[#text("alternating_determinant")],
[#text("Universal commutative-ring identity in all eight variables")],
[#text("Anti-isometry graph is isotropic")],
[#text("anti_isometry_graph_isotropic")],
[#text("Pairing coefficient times det(M)=-1; proves cancellation of the two explicit product forms for all vectors")],
[#text("Graph has zero left-factor intersection")],
[#text("graph_transverse_left")],
[#text("Additive homomorphism psi; explicit membership witness (a, 0)=(psi(b),b)")],
[#text("Graph has zero right-factor intersection")],
[#text("graph_transverse_right")],
[#text("Same graph, psi injective; explicit membership witness")],
[#text("Quotient map preserves the left factor")],
[#text("graph_quotient_left_injective")],
[#text("Additive quotient map, explicit implication identifying its left-factor kernel with graph membership; no Jacobian assumption")],
[#text("Cubic-cover substitution")],
[#text("binary_cubic_cover_substitution")],
[#text("Commutative ring of characteristic 2, arbitrary u,v,a,d; rational-map extension and geometric degree are external")],
[#text("Five-point conic pullback intersections")],
[#text("conic_pullback_intersections")],
[#text("Exact integral intersection arithmetic for the specified degree/multiplicity pairs; blowup geometry external")],
[#text("Adjunction gives genus 6")],
[#text("decic_adjunction_genus")],
[#text("Explicit adjunction intersection equality; establishing it for the actual smooth curve is external")],
[#text("Positive-sign genus 4 count contradiction")],
[#text("genus 4_positive_sign_count_obstruction")],
[#text("Finite sets with an injection, cardinalities4 and 2; scheme-points/base-change bridge external")],
[#text("Negative-sign genus 4 count contradiction")],
[#text("genus 4_negative_sign_count_obstruction")],
[#text("Count77 and supplied published upper bound71; census bound not formalized")],
)

#heading(level: 2)[#text("The two del Pezzo twists")]

#text("Let Y be the blowup of P2 in five distinct points on the smooth rational conic Q: xz+y squared=0. Their Frobenius types are (5) and(3, 2). No three are collinear. The strict transform Q has self-intersection4-5=-1 and contracts to a smooth rational point on a degree-five del Pezzo S. This is Zaitsev's five-point conic construction, section4 of ")#link("https://arxiv.org/abs/2302.04937")[#text("Forms of del Pezzo surfaces of degree 5 and 6")]#text(".")

#text("Writing H for a plane line and E_i for exceptional divisors, pi^*(-K_S)=5H-2*sum(E_i) and pi^*(-2K_S)=10H-4*sum(E_i). The latter is orthogonal to Q, has square20, and intersection10 with -K_S. Adjunction gives 2g-2=20-10=10, hence arithmetic genus 6. The native kernels have dimensions 6 and 16, respectively. For a smooth section this is its geometric genus; singular sections are not promoted to carriers.")

#text("The zero set on Y is counted using order-four Hasse jets at each exceptional fiber. A section restricted to Q is either nowhere zero or identically zero, since the restricted line bundle has degree 0. The native checker verifies this independently: at every rational point of the strict transform Q the 16 evaluation coordinates are a common nonzero multiple of those at (1:0:0), including exceptional tangent directions. If Q is contained, replace its q+1 zero points by the one contraction point, subtracting q. Otherwise there is no correction. The conic has q+1 rational points; the surface count is q squared + q*(fixed points of the five-point permutation)+1. These are written geometric arguments, with exact finite checks, not a full formal blowup/contraction implementation in Lean.")

#heading(level: 2)[#text("Integral polarizations and their finite graphs")]

#text("For O=Z[w], w squared+w+2=0, conjugation sends w to -1-w. The norm of a+bw is a squared-a*b+2b squared. The ordinary base elliptic curve has fundamental Frobenius discriminant-7, so its geometric endomorphism ring is O. Over the algebraic closure the norm-kernel A5 is E to the fourth power, using augmentation basis e_i-e_4 in E to the fifth power.")

#text("Restrict the circulant with first row (0,u,v,bar(v),bar(u)). It is invariant under cyclic permutation. The initial integral u,v sublattice has determinant five examples; it is not all integral invariant forms. The additional integral congruence classes have numerators (a,b,c,d) congruent to r*(1, 2, 2, 4) modulo 5. The restricted matrix entries are divided by 5 and checked integral. This is still a bounded parameter enumeration on one augmentation model, not every integral representative of its isogeny class.")

#text("Exact positive leading principal minors certify positive definiteness of the complex Hermitian form. Writing its endomorphism matrix in the basis(1,w) and multiplying by the product elliptic alternating form gives integral J. For both signs F=pi*S satisfies F-transpose*J*F=2J; here pi=w or -w and S is the augmentation action. The checker uses independent integer Bareiss elimination to verify det(J)=det(H) squared. At prime det(H)=ell the kernel has dimension2 moduloell. The induced pairing on kernel representatives x,y is x-transpose*J*y/ell moduloell, independent of integral lifts because J*x and J*y vanish moduloell. It is nondegenerate in every retained case.")

#text("An elliptic intertwiner M is an anti-isometry exactly when pairing*det(M)=-1 moduloell. This is the universal alternating identity formalized above. Graph columns(M,I) have rank 2, and the graph is isotropic in the four-dimensional polarization kernel, hence maximal isotropic. Its factor intersections are zero. The separate checker exhausts all two-by-two matrices for compatible characteristic polynomials, avoiding the generator's Hom-nullspace solver.")

#text("Under the standard identification of positive integral Hermitian forms with polarizations on E to the fourth power, Frobenius invariance supplies an F2 polarization on A5. Glue it to an elliptic curve with ell times its principal polarization along the stable graph. Since ell is odd, the graph is etale in characteristic 2. The product polarization has degree ell to the fourth power; the quotient has degree ell squared, so its descended polarization is principal. The graph criterion supplies a principally polarized quotient, not a curve equation. See the general discussion in ")#link("https://www.math.u-bordeaux.fr/~damienrobert/pro/publications/notes/2022-09-kani.pdf")[#text("Robert, Reducible gluing of abelian varieties")]#text(". The elliptic Hermitian identification, polarization descent, algebraic quotient and theta/Jacobian criterion are not formalized here. The finite matrices do not bypass these geometric bridges.")

#text("The short-vector search checks a bounded box only. It finds minimum 3 within that box for all four determinant 19 gluing cases and no norm-one witness. It does not prove global minimum 3 or geometric indecomposability. An exact inverse-Hermitian coordinate bound is the next discriminating extension.")

#heading(level: 2)[#text("n=5 map, rows, and descent")]

#text("The source is E_a/F32: Y squared+X*Y=X cubed+a*X squared+1. Its order-eleven subgroup is unique, since the full orders are 44 and 22. The tuple (P,F(P),...,F to the fourth(P)) is fixed under Frobenius combined with inverse cyclic shift. Its sum is zero on eleven-torsion because the norm lands in E_a(F2), of order 4 or2. Thus the tuple evaluates an actual point of the augmentation model of A5(F2). All11 points and 121 additions per sign are checked. No quotient coordinates on the modular Jacobian or on the glued abelian variety have been evaluated.")

#text("The same-field control is C_a: v^2+u^2*v=u^7+a*u^4+u, with phi(u,v)=(u^3,u*v+1). The defining identity is proved in Lean. The Artin--Schreier form has two poles of order 3, giving genus 3; smoothness at zero and infinity is checked by the existing native constructor. The cubic extension is separable and degree 3: X on E has zero and pole orders2, hence is not a cube in its function field. The map extends to the two ramified fibers at infinity and at(0, 0). Pullback/norm composition is[3], and 4*3-11=1 gives injectivity on eleven-torsion. The existing Lean push-pull lemma proves that implication once the geometric composition is supplied. All11 pullback classes per sign are also checked directly for nonzero order and scalar compatibility. General F32 Jacobian-class pushforward is still unsupported by the reused library; no such norm evaluator is claimed.")

#text("The factor base is every affine rational point minus infinity. A function y+v(u), degree(v)<=3, has pole 7 at infinity; a quartic v gives pole 8. Its norm is v squared+u squared*v+f. Completely split norms supply point-divisor rows, with exact multiplicities. Vertical functions u-c supply conjugate pairs or twice the ramification point. Every admitted row sums to the identity under the independently maintained library's Mumford arithmetic. The generator and separate verifier use different rank algorithms.")

#text("For the final a=1 extension, enumerate all effective degree-five divisors on the rational factor base, excluding a pair of conjugates at a nonbranch abscissa and repeated support at the branch point. Hermite interpolation uniquely determines a polynomial v of degree<=4. Every completely split norm of degree 7 or8 has an allowed degree-five subdivisor, so this procedure covers the missing quartic functions without enumerating all 32 to the fifth power coefficient tuples. The constructor explicitly checks inclusion of every earlier admitted y+v relation. The rank rises from 16 to 20 on21 columns. This completeness argument is written, not formalized in Lean.")

#text("Projected factor-base logs are verified by multiplying every point class by Jac(C)(F32)/11, which is prime to 11. Their correctness is independent of the linear-algebra solution. For individual descent, v=V+c*U gives a function whose divisor contains the shifted Mumford divisor. A fully split residual supplies its logarithm. There are 32 trials per offset, with no offset and then at most eight fixed rational-point offsets. Witnesses are checked by norm division and class addition. Only after solving is the recovered scalar compared by elliptic multiplication. The finite group and the base field are stated explicitly; this is no F2 carrier or modular-Jacobian evaluator.")

#heading(level: 2)[#text("Conditional small-genus carrier exclusion")]

#text("A curve whose Jacobian contains the full A5 factor must have genus at least4 by dimension. At genus 4 its Frobenius polynomial is exactly P_A5. For the positive sign, the counts N1=4,N2=2 violate rational-point injection. For the negative sign N5=77 exceeds Xarles's published maximum 71 among F2-defined genus-four curves, ")#link("https://arxiv.org/abs/2007.07822")[#text("Example 16")]#text(". The two conditional Lean lemmas expose the injection and maximum hypotheses.")

#text("The pinned ")#link("https://github.com/DusanDragutinovic/MT_Curves")[#text("Dragutinovic genus-five census")]#text(" has no matching polynomial among 7829 vectors. The pinned ")#link("https://github.com/junbolau/genus-6")[#text("Huang--Kedlaya--Lau genus-six census")]#text(" has no match among 38327 isogeny classes representing 72227 curves. Every encoded genus-six polynomial is label-roundtripped and Newton-roundtripped. Combining these with the genus-four tests excludes full-A5 F2 Jacobian carriers through genus 6, conditional on the published bounds and census completeness/correctness. The census classifications are not independently reproduced or proved in Lean. This does not exclude other finite-subgroup representations, other base fields, higher genera, or direct modular relations.")

#heading(level: 2)[#text("Integer identity certificates")]

#text("The native assembly issues and checks these identity.certificate/v1 records at 64 deterministic sample points plus 128 extra points each:")

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Certificate")],
[#text("ID")],
[#text("Relation to Lean")],
),
[#text("")#link("identity-cm-norm.json")[#text("CM norm integer lift")]#text("")],
[#text("IDC1hea9f559dcc2b81fc")],
[#text("Universal integer polynomial with the residual multiple of w squared+w+2 retained; the Lean CM norm lemma assumes that residual is zero")],
[#text("")#link("identity-alternating-determinant.json")[#text("Alternating determinant")]#text("")],
[#text("IDC1h2c8208a50a83bbaf")],
[#text("Direct universal statement, with the pinned Lean source hash")],
[#text("")#link("identity-cubic-cover.json")[#text("Cubic-cover integer lift")]#text("")],
[#text("IDC1h1fe2e21ad051cd8a")],
[#text("Retains the residual 2*(u*v*d+d squared); the characteristic-two Lean lemma removes it")],
)

#text("Odd-prime certificate samples alone do not prove a characteristic-two specialization. The exact kernel proofs and the displayed residuals provide that bridge. Certificates with a different statement have no misleading Lean proof attachment.")
