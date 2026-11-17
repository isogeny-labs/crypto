#set page(paper: "a4", margin: (x: 19mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.55em)
#set heading(numbering: none)
#show link: set text(fill: rgb("245c91"))
#heading(level: 1)[#text("Koblitz carriers and direct relations: round 02")]

#text("2026-10-18. Parent commit 047074d9912d64842995f9d590d8c0babdd48042. This round follows the user's direction to generate experiments, track results, and supply relevant Lean proofs. The user assesses implications for faster IC. No timing or performance comparison was performed.")

#text("The concrete positive result is an evaluated n=3 transfer and a geometric relation solver on the Klein quartic. Of 56 degree-three quotient maps, 42 give nonzero transfers on the complete order-seven source group. Norms of 12,036 split line functions over F512 produce rank 64 on 65 closed-point factor-base columns modulo seven. After fixing the rational flex-divisor anchor, these relations recover all seven public source fixtures. A separate checker reconstructs the relations, recomputes rank with BigInt elimination, and checks 65 divisor-class certificates without interpolation solving.")

#image("routes.svg", width: 100%)

#heading(level: 2)[#text("Evidence and coverage")]

#text("The native ")#link("ledger.json")[#text("ledger")]#text(" records input and source hashes, requirement status, results and next experiments. The frozen ")#link("PROTOCOL.md")[#text("protocol")]#text(" contains amendments recorded before each added experiment. All raw outputs, failures and verification logs are in ")#link("runs.tar.xz")[#text("runs.tar.xz")]#text(", checked by ")#link("runs.tar.xz.sha256")[#text("runs.tar.xz.sha256")]#text(" and ")#link("runs.sha256")[#text("runs.sha256")]#text(". The ")#link("REPLAY.sh")[#text("replay script")]#text(" reproduces thirteen outputs byte for byte. ")#link("README.md")[#text("README.md")]#text(" gives extraction and verification commands.")

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
[#text("Experiment")],
[#text("Exact executed scope")],
[#text("Observed result")],
[#text("LC0")],
[#text("690 genus-5 Artin--Schreier double covers of five elliptic classes, declared rational odd pole partitions")],
[#text("No A5 or A3 target match")],
[#text("LC0b")],
[#text("32 genus-5 covers of E1, branch O plus one of four degree-three places, all trace-one parameters and constant twists")],
[#text("No A5 match; rational expressions and removable limits checked")],
[#text("LC1")],
[#text("All 1,048,575 nonzero bidegree-(3,4) forms on P1 x P1")],
[#text("No A5 divisibility candidate for either sign")],
[#text("LC1b")],
[#text("All 1,048,575 nonzero sections of 3C0+7F on the split Hirzebruch surface F2, C0 squared=-2")],
[#text("No A5 divisibility candidate for either sign")],
[#text("LC2")],
[#text("Five del Pezzo Frobenius types, 65,535 nonzero sextic sections each; 327,675 total")],
[#text("No A5 divisibility candidate for either sign")],
[#text("MR0")],
[#text("Orbit recurrences at n=3,5,7,131; actual small-subgroup point controls")],
[#text("At n=131: ranks 130,260,261,262 for one block, two blocks, anchor, supplied cross row")],
[#text("MR1")],
[#text("Five inert good primes, signed n=131 level 30,349,081; n=3,5 controls")],
[#text("n=131: all five necessary-condition gcds equal one; small controls pass")],
[#text("KT0/KT1")],
[#text("168 automorphisms; 56 quotient maps; seven source scalars, all fibers split over F512")],
[#text("42 nonzero maps, 14 zero maps; 393 principal-divisor checks pass")],
[#text("MR2")],
[#text("All 66 fully split F8 lines, nine factor-base columns")],
[#text("Rank 8; four fixtures recovered; three unsupported fibers retained")],
[#text("MR2b")],
[#text("All 262,657 F512 lines screened; 12,036 fully split lines, 65 columns")],
[#text("Rank 64; seven of seven fixtures recovered and checked")],
[#text("GL0/GL1")],
[#text("Both A5 signs, all elliptic cofactor traces; eight nonempty gluing-prime cases")],
[#text("Candidate primes 5,11,19; all eight Frobenius Hom spaces have dimension two")],
[#text("PF0")],
[#text("Thirteen structural lemmas in pinned Lean/mathlib, plus prior cyclic numerical lemma")],
[#text("Kernel builds and axiom audits; geometric bridges remain partial")],
)

#text("Genus-six forms include singular, reducible and nonreduced sections. The point-count/Newton/divisibility test is a necessary test for a smooth genus-six carrier. Since there are no divisibility candidates, none reaches a smoothness promotion. These searches do not prove that all genus-five or genus-six carriers are absent. LC2 covers Frobenius types 1+1+1+1, 2+1+1, 2+2, 3+1, and 4, using four general-position blowup points; types 5 and 3+2 remain open. No classification theorem upgrading these families to exhaustive genus-six coverage is claimed.")

#heading(level: 2)[#text("Curve, map, and subgroup identities")]

#text("The source control is E1: Y^2+X*Y=X^3+X^2+1 over F8, with its unique order-seven subgroup. E0, the other sign, is Y^2+X*Y=X^3+1. A_n denotes the full trace-zero factor of the restriction from F_(2^n) to F2; at n=5 it has dimension four. All target polynomials are constructed exactly, independently of the carrier enumeration. The large rung is the registered E0/F_(2^131) curve icv1-f2m131-tm22283658519494248867-115e0dc5, subgroup order 680564733841876926932320129493409985129. Its signed-character modular carrier and polynomial identity are preserved from ")#link("../../REPORT.md")[#text("round 01")]#text("; no large-rung point was targeted.")

#text("The Klein model is x^3*y+y^3*z+z^3*x=0. For a=x+y+z, b=x*y+y*z+z*x, c=x*y*z, the cyclic quotient is X=a*c/b^2, Y=1+a^3*c/b^3. At b=0 in the tested odd-degree fields, c=0 maps to (0,1) and c nonzero maps to the origin. There are no a=b=0 points in these fields; a global resolution at geometric exceptional points remains part of the written map argument. Conjugation by all 168 projective automorphisms yields 56 distinct maps. A separate verifier expands each recorded matrix against the quartic polynomial.")

#text("D=(0:1:0)-(1:0:0) has exact order seven. For each quotient phi, the transfer is the Frobenius trace of phi-pullback of (Q)-(O). Over F512 all cubic fibers split; the source still lies over F8. The 392 input/map combinations have explicit principal-function certificates expressing the transfer as an element of the cyclic group generated by D. Together with the witness for 7D, the separate checker verifies 393 principal divisors.")

#text("The Klein modular-curve interpretation and characteristic-two geometry are classical; see ")#link("https://library.slmath.org/books/Book35/files/elkies.pdf")[#text("Elkies, sections 3.3 and 4")]#text(". Our arithmetic claims concern exactly the displayed F2 model, including its descent, rather than treating all F2 twists as interchangeable. The explicit transfer and relation data in this round are native calculations.")

#heading(level: 2)[#text("Direct geometric relations and recovery")]

#text("MR0 uses inverse-Frobenius indexing, giving u_j-t*u_(j+1)+2*u_(j+2)=0. Homogeneous relations in separate orbit blocks leave independent scalar freedom. The extra cross-orbit row in MR0 is an explicit information control; it is not a generated modular relation. The Lean scaling lemmas establish the structural reason. This does not exclude other sources of modular relations.")

#text("MR2/MR2b instead generate actual principal divisors of Norm(ell/z). Every row comes from a line function, without target scalars or the KT class answers. The nullspace has one free scale, fixed by D. Certificates verify the factor-base logs after multiplication by two, removing the irrelevant two-part of Jac(C)(F2), whose order is 14. The transfer is seven-torsion, so the seven-part suffices. The first map with a nonzero solved base log is selected using the relation solution. Recovery receives point inputs, reconstructs fibers and traces, takes their solved logs and divides by the base log. Known fixture scalars are compared only after solving.")

#text("The F8 factor base has nine columns and recovers scalars 0,1,2,4. Scalars 3,5,6 have nonsplit cubic fibers there. MR2b expands the coordinate field and factor base; it recovers all scalars 0 through 6. This is a full finite control for that particular seven-element group. It supplies no extrapolated relation yield, descent coverage or cost at n=5 or n=131.")

#text("MR1 tests a narrower arithmetic condition. If character values epsilon,eta satisfy epsilon^D=eta^D=1 and epsilon+p*eta=0, then (-p)^D=1. For N=49*787^2, D=4,330,074, the five primes 3,5,13,17,19 each give gcd(r,(-p)^D-1)=1, with Bezout witnesses. Thus the tested character-pair congruence is absent. The n=3 control uses N=7^4 (2401), rather than a minimal conductor, and the n=5 control uses N=5929; both pass this screen. No theorem identifying every cuspidal or modular-unit relation with this character-pair test has been supplied. Interior relations remain open.")

#heading(level: 2)[#text("Gluing and formal verification")]

#text("The real-Weil resultants against elliptic cofactors supply primes 5,11,19. All eight companion-module cases have a two-dimensional Frobenius Hom space. There are 24,100,360 injective intertwiners at primes 5,11,19, respectively; full elliptic torsion field degrees are 24,10,72. These are finite-module calculations for the companion Z[pi] lattice. They do not construct an alternating pairing, a principal polarization or a Jacobian, and they do not enumerate every integral isogeny representative.")

#text("The ")#link("PROOF_MAP.md")[#text("proof map")]#text(" states all thirteen Lean declarations, their hypotheses, readable divisor and relation arguments, and missing bridges. The final kernel proofs use only propext, Classical.choice and Quot.sound. The degree-eight certificate IDC1h2d387a2f2db3e737 (")#link("identity-klein-lift.json")[#text("integer-lift certificate")]#text(") records Delta=F*F_reverse+2H; ")#link("identity-klein-expansion.json")[#text("exact expansion")]#text(" records the even residual coefficients. Its v1 checker works over the specified odd prime. The Lean quotient identity independently proves the characteristic-two reduction; the scopes are explicit, and the odd-prime certificate is not mislabeled as a characteristic-two proof.")

#heading(level: 2)[#text("Controls, failures, and limitations")]

#text("Walsh counts agree with 54 direct LC1 checks, 42 direct LC1b checks and 180 direct LC2 checks. Native unit tests also exhaust all small (1,2) forms over three fields and check elliptic group laws on all five base classes. LC0/LC0b have exact base-elliptic divisibility checks. No positive A5 curve was found or inserted as a control; the known Klein target and original polynomial identities remain the positive target controls.")

#text("The first native build, initial Lean elaborations, and a later cosmetic rename failed; the logs are preserved. A cache setup requested an obsolete mathlib path after downloading the cache and returned nonzero; later builds succeeded. The first all-scalar F8 output has unsupported fibers and a null first-nonzero-map metadata field; it is incomplete coverage, not evidence that every transfer vanishes. Its v3 rerun changes only schema/scope/field metadata. The first byte replay consequently failed on that version change; the revised replay compares v3 to v3. No frozen output was rewritten.")

#text("All generation and checks here were performed by the same agent; separate algorithms and solver-free certificate checks are available, but this is not independent external review. Full geometry in Lean and higher-n maps remain unresolved. No performance leaderboard row was promoted. The two canonical carrier summaries were updated with this round's structural scope; progress-timeline and timing graphs have no new measured quantity.")

#heading(level: 2)[#text("Requirement status and next experiments")]

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
[#text("Requirement")],
[#text("Status")],
[#text("Next discriminating experiment or proof")],
[#text("Low-genus carriers")],
[#text("Partial; specified families exhausted")],
[#text("Descend the two remaining del Pezzo Frobenius types 5 and 3+2, controlled against the five executed types")],
[#text("Direct modular relations")],
[#text("Partial; n=3 geometric solver verified")],
[#text("Evaluate interior principal-function relations on an n=5 modular factor; supply the Hecke/cusp bridge before extending MR1")],
[#text("Transfer and recovery")],
[#text("Complete on the specified n=3 subgroup; higher n open")],
[#text("Construct an n=5 map/projector and verify nonzero subgroup action and exceptional fibers")],
[#text("Gluing/polarization")],
[#text("Candidate modules verified; construction open")],
[#text("Enumerate Rosati-compatible integral lattices at 5,11,19, then check maximal isotropy, principal polarization and Jacobian realization")],
[#text("Lean proofs")],
[#text("Thirteen lemmas checked; full geometry partial")],
[#text("Formalize the divisor-certificate criterion and connect it to the explicit curve, Picard classes and instantiated recovery")],
[#text("IC efficiency assessment")],
[#text("Not attempted in this round")],
[#text("User determines the assessment and subsequent matched cost protocol")],
)

#text("These are generated follow-up experiments, not jobs running in the background. The shared unresolved record is ")#link("../../../../problems/koblitz-modular-carrier-transfer.md")[#text("the carrier-transfer problem")]#text(".")

#text("The final proof update also checks the Klein gradient's common-zero criterion, bringing the structural module to thirteen theorems. The first ledger draft was assembled while two verification logs were still growing; it is retained in the raw archive as an unverified draft. The canonical ledger was assembled after verification finished, and excludes its own open assembly receipt. The archive manifest covers every finalized receipt, including that draft.")

#heading(level: 1)[#text("Formal proof map and written arguments")]

#text("Toolchain Lean 4.34.1; mathlib commit d13f23b723b8a846827a245b89c10fc7d3f11612. The checked module is ")#link("../../lean/KoblitzRelations.lean")[#text("../../lean/KoblitzRelations.lean")]#text(". Its thirteen declarations have kernel proofs; transitive foundations are propext, Classical.choice, and Quot.sound only. No native evaluation, custom axioms, or admitted proofs occur in the final source. Initial failed builds are retained as failed evidence. lean4checker was unavailable.")

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
[#text("Report implication")],
[#text("Lean declaration")],
[#text("Exact scope and remaining bridge")],
[#text("Orbit recurrences survive scalar rescaling")],
[#text("recurrence_scale")],
[#text("Any field and index successor, assuming the displayed recurrence")],
[#text("Independent target block retains scaling freedom")],
[#text("separate_blocks_leave_target_scaling")],
[#text("Linear kernel statement, with distinctness supplied by the next theorem")],
[#text("Different scales of a nonzero target differ")],
[#text("nonzero_target_scales_distinct")],
[#text("Field module, nonzero vector; no claim about all modular correspondences")],
[#text("Prime-to-r push-pull degree gives injectivity")],
[#text("push_pull_injective_on_torsion")],
[#text("Additive homomorphisms, explicit Bezout witness, composition equals degree, r kills both inputs; constructing geometric pull/push is external")],
[#text("Zero eigenvalue character pair forces (-p)^D=1")],
[#text("eisenstein_zero_necessary")],
[#text("Commutative ring; both character values have Dth power 1 and epsilon+p*eta=0; identification with cusp/Hecke modules is external")],
[#text("Quadratic extension point-order factorization")],
[#text("quadratic_extension_factorization")],
[#text("Universal commutative-ring polynomial identity; Frobenius interpretation external")],
[#text("Flex divisor relations imply 7D=0")],
[#text("klein_flex_relations")],
[#text("Additive group, E=3D and 3E=2D; deriving divisors of x,y,z is written geometry")],
[#text("Klein quotient invariant identity")],
[#text("klein_invariant_identity")],
[#text("Universal polynomial identity in a commutative ring of characteristic 2, for all x,y,z; rational-map extension and degree remain geometric")],
[#text("Nontrivial additive characters sum to zero")],
[#text("additive_character_orthogonality")],
[#text("Finite additive group, nontrivial complex additive character; bit-trace/Walsh implementation correspondence checked by native controls")],
[#text("Three-term Frobenius trace is fixed")],
[#text("frobenius_trace_fixed")],
[#text("Additive endomorphism F, hypothesis F^3(x)=x; identifying F with curve Frobenius external")],
[#text("Frobenius intertwiner respects iterates")],
[#text("intertwiner_iterates")],
[#text("Arbitrary functions with intertwining identity; no integral lattice or polarization existence conclusion")],
[#text("Nonzero anchored transferred log recovers scalar")],
[#text("anchored_transfer_recovery")],
[#text("Field-linear transfer and logarithm, nonzero log of transfer(1); actual geometric maps and correctness of line-derived logs are outside Lean")],
[#text("Klein gradient has no common projective zero")],
[#text("klein_gradient_common_zero")],
[#text("Field, curve equation and displayed gradient equations; all coordinates must vanish. The projective Jacobian criterion is external")],
)

#text("The older ")#link("../../CyclicWindow.lean")[#text("../../CyclicWindow.lean")]#text(" proves the conditional numerical genus-window implication. This round adds no stronger formal geometric cyclic-cover theorem.")

#heading(level: 2)[#text("Written divisor certificate criterion")]

#text("The Klein quartic is a smooth plane curve: its three partial derivatives cannot vanish simultaneously at a projective point. With a zero coordinate, the gradient equations force all coordinates zero. Otherwise multiply the three gradient equations by x, y, z respectively. The three curve monomials are equal and nonzero; their sum in characteristic two is nonzero, a contradiction. It is therefore geometrically integral of genus three. A homogeneous form of degree m not divisible by its equation cuts a divisor of degree 4m.")

#text("For a degree-zero divisor D supported in the recorded field, the checker constructs the effective divisor H of a product of split lines. It checks that a nonzero degree-m numerator, reduced in the standard monomial basis modulo the curve equation (leading monomial x^3*y), vanishes to at least H+D at every point. The checked orders sum to 4m; hence equality holds and its quotient by the denominator has divisor D. Local orders use the implicit function theorem with a checked nonzero partial derivative. The separate checker evaluates the supplied coefficients and jets; it never solves the interpolation system. A rational divisor principal after finite field extension is already principal over F2: the associated Riemann--Roch space commutes with field extension and has an F2 generator. This is a written argument, not a formal Lean theorem.")

#text("The coordinate line divisors are (x)=3P2+P1, (y)=3P0+P2, (z)=3P1+P0, where P0=(1:0:0), P1=(0:1:0), P2=(0:0:1). Thus div(x*z^2/y^3)=7(P1-P0). The difference of two distinct rational points cannot be principal on a positive-genus smooth curve: its function would give a degree-one map to P1. Consequently D=P1-P0 has exact order seven. The native interpolation also checks failure of a function with divisor D, and supplies the witness for 7D.")

#heading(level: 2)[#text("Transfer and relation argument")]

#text("The quotient formulas in REPORT.md are invariant under cyclic coordinate permutation. The invariant identity shows the image lies on E1. The quotient has degree three: for a nonexceptional image, the symmetric invariants recover a cyclic orbit of three points; the independent cyclic quotient genus computation and explicit fibers provide additional checks. The native experiment evaluates the three-point fibers for all seven source points. Its trace map T=sum_(j=0)^2 F^j pull respects addition. Principal certificates directly show T(kG)=c*k*D, c nonzero for 42 of 56 maps. This establishes the subgroup action on the complete order-seven fixture group. It is separate from a general transfer at other n.")

#text("Each split line ell supplies the principal divisor of the norm of ell/z. Write the F2-rational divisor in closed-point orbits, and subtract its degree times P0. These give relation matrix rows without target labels. Row rank is checked using u64 reduced elimination and separate BigInt forward elimination. A single free scale is fixed by D. For each resulting factor-base log l_i, a separate certificate verifies 2*(sum_(P in orbit_i) P - deg(orbit_i)*P0 - l_i*D) is principal. Thus these logs are correct after projection to the seven-part. The source transfer is seven-torsion, so the projection loses no scalar information. The selected map has nonzero base log, and division by it recovers the source scalar. All seven resulting answers are checked by elliptic multiplication.")

#heading(level: 2)[#text("Open formal obligations")]

#text("A full proof still needs the actual function field and smooth projective Klein model, divisors/local orders, Riemann--Roch certificate criterion and base change, quotient/pullback/pushforward, the Picard seven-part, and the linearity bridge for the instantiated recovery theorem. The experiment does not encode these definitions by assuming its desired geometric conclusion. Modularity of the signed-character factor, the Hecke action on cuspidal divisors, polarized integral gluing and Jacobian realization remain separate unformalized bridges. GL1 proves finite companion-module calculations only.")
