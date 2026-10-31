#set page(paper: "a4", margin: (x: 19mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.55em)
#set heading(numbering: none)
#show link: set text(fill: rgb("245c91"))
#heading(level: 1)[#text("Round 06 — column images, reverse divisor coordinates and polarized quotients")]

#text("Client date 2026-10-18; host date 2026-10-19 UTC. Parent round05 is preserved. This is exact construction and verification, with no IC timing experiment.")

#text("All44 degree210 columns now have evaluated images: zero for both signs. The evaluated principal matrix has rank5 on588 columns, but removing the new zero-image columns restores rank4. The nonzero-image submatrices remain rank4 on510/520 columns. Thus this particular additional row gives no new constraint on the old nonzero-image columns.")

#image("routes.svg", width: 100%)

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Track")],
[#text("Verified result")],
[#text("Remaining obligation")],
),
[#text("Degree210 images")],
[#text("All44 closed traces evaluated for each sign; all616 geometric tuples are nonzero; all44 relative14-term traces already vanish")],
[#text("Relations distinguishing second7 choices")],
[#text("Polarized quotients")],
[#text("All10 frozen determinant5 graphs realized with explicit torsion basis binding; three split models, effective F2 descent and positive unimodular polarizations")],
[#text("Determinant19 mixed quotients; expanded F2 projective equations; Jacobian identification")],
[#text("Even reverse divisor")],
[#text("All20 pullbacks in explicit coordinates; positive and negative degree6600;7200 point-algebra entries of degree1 or11; F2 descent checked")],
[#text("Independent pointwise forward norms, reduced Jacobian arithmetic and odd reverse divisor")],
[#text("Lean")],
[#text("14 new named theorems,15 audited declarations including a generated equation; no nonstandard axioms")],
[#text("Global modular/Picard and divisor geometry remain written")],
)

#heading(level: 2)[#text("Why the new images vanish")]

#text("The pinned evaluation field is F2^210 with polynomial x^210+x^7+1. The F30 and F32 embeddings are retained explicitly. Native enumeration constructs56 cyclic49 chains and all12 transported eleven-isogeny targets for each chain, checks commuting7/11 squares, and partitions616 paired labels into44 closed degree210 orbits. The first trace implementation sums210 Frobenius conjugates; a separate path sums14 actual orbit members and then15 conjugates. Both return zero in every column for both signs. All616 Frobenius15 equivariance tests pass for each sign;672 field products also agree with independent schoolbook reduction.")

#text("Grouping the chains by their first7 kernel gives eight fibers of seven. For each of the12 target maps, every fiber sum is the same order2 cusp T=(0,1):96 exact checks. The degree7 map t0=x^4/y explains this constant sum. Even pair sums give2T=0 and odd differences giveT−T=0 before augmentation. The proof map separates the written fiber argument from the Lean cancellation and ring identities. This is a structural explanation for this family’s vanishing, not a universal obstruction to modular relations.")

#heading(level: 2)[#text("Quotient equations for every determinant5 graph")]

#text("Over F32 each model is E_a^5, with five projective equations")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("Y_i^2 Z_i + X_i Y_i Z_i = X_i^3 + a X_i^2 Z_i + Z_i^3.", block: true)]

#text("For k=0,4,8 its F2 form is specified by coefficient Frobenius followed by the explicit order5 automorphism")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("T_k(Y1,Y2,Y3,Y4,Z)=(-Y4,Y1−Y4,Y2−Y4,Y3−Y4,Z+pi^k Y4).", block: true)]

#text("Here Y1,…,Z denote elliptic points, rather than the homogeneous coordinates in the preceding equation. The quotient sends (a1,…,a5,Q), with sum a_i=0, to (a1−a5,…,a4−a5,pi^k a5−epsilon Q), after the retained integral U. The explicit Hermitian polarization has upper block I4+alpha J4, alpha=(2^k−1)/5, last column conjugate(pi^k), and last entry5. Its determinant is1, its leading minors are positive, and its quotient pullback and descent identities hold exactly.")

#text("The trace+1 companion basis is now bound explicitly to a CM basis by [[3,3],[4,1]] modulo5. Actual five-torsion coordinates realize that binding. All250 points in the ten frozen kernels pass the quotient equations; a separate maintained-library path checks them again. All7260 additions among the F2-rational inputs pass. These are split equations with effective F2 descent, not an expanded F2 ideal or a claim that the varieties are Jacobians. The earlier nine failed direct-natural-graph matches remain preserved; the new basis binding and nontrivial pi powers supply the missing constructions.")

#heading(level: 2)[#text("The complete even reverse divisor as explicit point algebras")]

#text("Choose the first nonzero order11 point P0 of E49(F32). The twelve reverse eleven-neighbors over P0 all have full eleven-torsion x coordinates over F2^120, with polynomial x^120+x^4+x^3+x+1. This corrects the initial prediction of degree11 complements, whose failed assertion is retained. The full paired fiber contains660 points, with all pairings verified.")

#text("Over −P0 the corresponding neighbors have only the five known rational x roots. The attempted split fiber fails as recorded. A Weil-pairing resolvent in a rank110 quadratic coordinate algebra instead produces five irreducible degree11 components per neighbor. Their x minimal polynomials and y coordinate polynomials specify all660 geometric points without enumerating F2^1320. Product, irreducibility, curve, order11 and pairing checks all pass.")

#text("For Pi=Frob^i P0, 5Pi=Frob^(i+1)P0 and6Pi=−Frob^(i+1)P0. The translated representative [6Pi]−[Pi] gives the required [5Pi]−[O] class using ordinary fibers. Frobenius conjugation and explicit determinant1 basis changes produce all20 pullbacks for the ten signed lines. The result has600 degree11 entries in its positive part and6600 degree1 entries in its negative part, each of geometric degree6600. No cancellation or Jacobian reduction is claimed. Both seed descents and the full20-term Frobenius permutation are checked. A separate parser rechecks all7200 coordinate entries and their requested line.")

#text("Its forward image agrees with the source generator using the already proved Gram correspondence:32000P=P on order11. This is a correspondence-based check with evaluated target points. An independent pointwise forward norm of the entire divisor is still unimplemented. The odd reverse divisor and general genus2841 Jacobian reduction also remain open.")

#heading(level: 2)[#text("Evidence, failures and next experiments")]

#text("The ")#link("ledger.json")[#text("ledger")]#text(", ")#link("PROTOCOL.md")[#text("protocol and amendments")]#text(", ")#link("PROOF_MAP.md")[#text("proof map")]#text(", ")#link("VALIDATION.md")[#text("validation")]#text(" and ")#link("REPLAY.sh")[#text("replay script")]#text(" record exact scope and requirements. The immutable archive retains two compiler failures and both unsuccessful fiber assumptions. Fresh replay reproduces24 JSON outputs and the generated Lean matrix source byte for byte. Prior rounds04 and05 also replay unchanged. Construction and checks are by the same agent; the separate paths share arithmetic primitives. There is no external independent audit.")

#text("The next discriminating experiments are target-function norms on the degree11 reverse components; functions separating second7 choices for nonzero new column images; and an explicit mixed determinant19 quotient. Each remains an open requirement. The user assesses their IC significance; no scalar recovery, solver cost or n=131 transfer is established here.")

#text("Primary formulas and context: ")#link("https://arxiv.org/pdf/1110.6801")[#text("Kilford–McMurdy, X0(49) equations")]#text(", ")#link("https://www.hyperelliptic.org/HEHCC/chapters/chap04.pdf")[#text("Miller algorithms in the Handbook of Elliptic and Hyperelliptic Curve Cryptography")]#text(", and ")#link("https://tsapps.nist.gov/publication/get_pdf.cfm?pub_id=917341")[#text("binary-curve arithmetic reference")]#text(". The new constructions and finite calculations above are repository results.")

#heading(level: 1)[#text("Formal proof map and written arguments")]

#text("The final Lean build and transitive audit cover14 named theorems plus step.eq_1. Only propext, Classical.choice and Quot.sound occur. Explicit matrix decisions use decide +kernel. Numerical identity checks supplement the kernel proofs; they do not formalize geometry.")

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Claim")],
[#text("Kernel proof")],
[#text("Written or executable bridge")],
),
[#text("Effective descent")],
[#text("descent_order_five")],
[#text("Galois descent of projective E^5 and its maps")],
[#text("Quotient polarization")],
[#text("polarization_lift, polarization_pullback; polarized_descent_0,_4,_8")],
[#text("CM endomorphisms, positivity and principal polarization")],
[#text("Split graph kernel")],
[#text("split_kernel")],
[#text("Complete250-point check and explicit basis binding")],
[#text("Degree7 cancellation")],
[#text("seven_even_cancel, seven_odd_cancel; degree_seven_equation")],
[#text("Modular forgetful map, local orders and all96 fiber checks")],
[#text("Ordinary reverse representative")],
[#text("reverse_ordinary_translation")],
[#text("Elliptic Picard identification and pullback")],
[#text("Coordinate-algebra inversion")],
[#text("quadratic_coordinate_norm")],
[#text("Separable degree11 factors and executable Miller checks")],
[#text("Reverse degree and Frobenius wrap")],
[#text("reverse_fiber_degree; fifth_line_wrap")],
[#text("Full20-fiber descent and point-algebra verification")],
)

#heading(level: 2)[#text("1. Quotient and polarization")]

#text("Use O=Z[w], w²+w+2=0, with its conjugation as Rosati. On E0 let w=pi and on E1 let w=−pi; pi is the actual coordinate Frobenius. All k used here are multiples of4, so pi^k=w^k for both signs. The round05 U matrices are integral automorphisms over F2 carrying each determinant5 polarization to H=I4+J4. Work first in these coordinates.")

#text("Define q_k on A×E by Y_i=a_i−a5 and Z=pi^k a5−epsilon Q. Its kernel is a1=…=a5=u,5u=0,Q=epsilon pi^k u, so it has25 geometric points and is separable. It is therefore an isogeny. The CM Hermitian matrix L in Q4 is integral. Its positive leading minors and determinant1 give a positive principal polarization on the product E^5. The native Hermitian calculation and the three integral10×10 Lean certificates verify q†Lq=diag(H,5), T†LT=L and T^5=I. The universal bilinear identity records the residual5(1+5alpha−p q)sa sb; it vanishes because Norm(pi^k)=2^k=1+5alpha. Changing epsilon is absorbed by the elliptic automorphism Q↦epsilon Q, which preserves its polarization.")

#text("The source cyclic factor descent induces exactly T_k on q coordinates. Its coefficients are F2 endomorphisms, hence it commutes with coefficient Frobenius. The order5 cocycle gives effective descent from F32 to F2 for this projective abelian variety, its polarization and quotient map. This supplies equations as a split projective model plus descent datum. Expanding a single F2 projective ideal is a distinct unfinished presentation.")

#text("The frozen finite graphs previously had no bound geometric companion basis. For trace+1 we explicitly use Tplus=[[3,3],[4,1]] mod5, determinant1; it intertwines companion Frobenius with −w on E1[5]. For trace−1 use the declared CM basis directly. Q4 retains actual CM basis points and all 25 points of each graph, and checks the common block after the frozen U. Every graph matches epsilon pi^(−k), k=0,4,8. Thus every specified graph with this explicit binding is the kernel of the given q_k∘(U,id). This does not retroactively make the earlier unbound basis canonical. No determinant19 graph or Jacobian realization follows from this argument.")

#heading(level: 2)[#text("2. The degree7 fiber sum")]

#text("Write the eta model as")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("y²+x y(x²+x+1)+x(x6+x5+x4+x3+x2+x+1)=0.", block: true)]

#text("Substitute t=x4/y and clear denominators. The exact Lean ring identity gives the monic degree7 equation for x printed in the report. Generically y=x4/t, so [F2(E49):F2(t)]≤7. On the ordinary elliptic model at T=(0,1), set v=Y+1. The elliptic equation becomes")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("x³+x²+x+x v+v²=0.", block: true)]

#text("Its derivative with respect to x at T is1. Hence v is a local parameter, ord_T(x)=2, and the eta y=x²+(x²+x+1)v has order1. Therefore ord_T(t)=7. The degree bound is attained: t has degree7 and zero divisor 7[T]. Every generic fiber is linearly equivalent to7[T]; under the elliptic Picard identification its seven points sum to7T=T. This uses the standard X0(49)→X0(7) forgetful identification of t from the cited eta formulas. It is written geometry, not a finite-test inference.")

#text("The 56 cyclic49 choices group into eight seven-element fibers over a fixed first7 kernel. Quotienting by eleven commutes with this forgetful map, so each of the12 target maps also has fiber sum T. All96 sums are checked in exact F2^210 coordinates. The14-member Frobenius orbits traverse two such seven-element fibers, with the normalized complement labels carried along. The direct14-term sum also verifies this for every column. For the signed tuple, the seven sums cancel pairwise as2T or T−T; augmentation and the subsequent trace preserve zero. Lean proves these cancellation implications assuming the two fiber-sum identities. It does not formalize the modular forgetful map or the local divisor.")

#heading(level: 2)[#text("3. Reverse fiber coordinates and the full divisor")]

#text("The ordinary reverse11 correspondence has12 base neighbors; each carries 55 determinant-normalized paired bases modulo simultaneous sign. The old P line is fixed by the dual kernel, while the pairing chooses its complement. These ordinary fibers are etale; all source49 kernels and backward targets are explicitly transported and checked. The positive seed is represented by660 F2^120 points.")

#text("For the negative seed let K=F2^120 and h=psi11/psi_known, degree55. In K[X,Y]/(h,Y²+XY−f(X)) the generic Q has order11. This algebra is etale because11 is prime to2 and X is nonzero on its support. Inversion uses the norm of u+vY, namely u²+uvX+v²f. The exact conditional characteristic2 identity is proved in Lean. The Miller recursion computes e(G,Q)=u+vY using an auxiliary point avoiding its poles. The recorded auxiliary points, units and pairing evaluations are executable checks.")

#text("For c=1,…,5, put z=zeta^(c^−1). The polynomial gcd of h with e+e^−1−(z+z^−1) has degree11. On this component set Y=(z+u)/v. Each factor is irreducible over K, their product is h, and the resulting point satisfies the curve equation, [11]Q=0 and e(cG,Q)=zeta. Choosing c modulo sign exhausts paired bases modulo simultaneous sign. Thus the 60 recorded components specify all660 geometric points, rather than an unresolved extension search. The separate checker recomputes these equations and pairings in degree11 quotient fields.")

#text("On the order11 source generator Pi, coordinate Frobenius acts as[5], so6Pi=−Frob Pi. In Picard0(E), [6Pi]−[Pi] represents5Pi and thus may replace [5Pi]−[O] in the round05 adjoint formula. This avoids cusp fibers while preserving the divisor class. Frobenius on paired bases is (P,Q)↦(P²,Q²/2). The determinant1 change")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("(P,Q)↦(−aQ,a^−1 P+Q)", block: true)]

#text("makes the line Pnew+aQnew equal the old P line. Applying these maps to both seed formats gives every one of the20 requested pullback fibers. The stored coordinates satisfy their curves, torsion, pairing and line conditions; the separate parser checks all7200 curve pairs and line conditions. Seed Frobenius5 permutations and the20-term permutation prove that the complete signed divisor descends to F2. There are ten positive and ten negative fibers, each of degree660.")

#text("The forward check invokes the round05 written correspondence identity Psi r=[32000] and evaluates its target-point expression. It is not an independent norm computation from the7200 entries. Full Picard reduction, pointwise forward norms and the odd reverse divisor remain unimplemented.")

#heading(level: 2)[#text("4. Identity certificates and formal scope")]

#text("identity-split-polarization-lift.json binds the universal bilinear residual to polarization_lift; identity-degree-seven-equation.json binds the exact cleared equation to degree_seven_equation. Each issuer checks64 points and an extra checker checks128, with direct Lean source hashes. The native record assembler binds all raw outputs and the relevant source files. Global descent effectivity, positivity-to-polarization, modular moduli, Picard identification and completeness of divisors are the written bridges above. No theorem assumes a new research claim as a custom axiom.")
