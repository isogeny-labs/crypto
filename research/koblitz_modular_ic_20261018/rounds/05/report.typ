#set page(paper: "a4", margin: (x: 19mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.55em)
#set heading(numbering: none)
#show link: set text(fill: rgb("245c91"))
#heading(level: 1)[#text("Koblitz carriers: reverse transfer and a level-dependent divisor")]

#text("Round 05, 2026-10-19. Parent 75b902661. All arithmetic is native Rust. No timings or IC efficiency assessment are made.")

#text("The n=5 modular map has a written integral reverse correspondence whose composition is [32000], a unit at 11. Five determinant-5 forms now have explicit polarized equivalences and canonical quotient equations. The function X(P)+1 gives a complete written principal divisor beyond the old four-fiber span. Its new closed-point images remain unevaluated.")

#image("routes.svg", width: 100%)

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Requested track")],
[#text("Result")],
[#text("Remaining gap")],
),
[#text("Quotient equations")],
[#text("Five explicit unimodular equivalences; transported diagonal graphs use the known five F2 equations")],
[#text("Nine frozen first graphs fail the natural-kernel test; determinant-19 equations open")],
[#text("Reverse transfer / saturation")],
[#text("Integral polarized adjoint, composition 32000; split injection on 11-primary torsion")],
[#text("Reverse fibers and reduced Jacobian divisor not evaluated; global optimal quotient not computed")],
[#text("Relations beyond rank 4")],
[#text("Complete written divisor and one symbolic projected row: rank 5 on 588 columns")],
[#text("44 new closed-point coordinates/images unevaluated; evaluated matrix stays rank 4 on 544")],
[#text("Lean")],
[#text("18 new named theorems, including five explicit inverse/pairing certificates")],
[#text("Moduli, adjunction, local valuation and divisor geometry remain written proofs")],
)

#heading(level: 2)[#text("Reverse correspondence with the actual polarization")]

#text("Let C be the determinant-normalized full-eleven-level curve over X0(49), of genus 2841, used in round 04. Its twelve maps phi_a to E49=X0(49) each have degree 660. The deck group PSL2(F11) has order 660 and acts transitively on the 132 ordered distinct pairs of lines. The Hecke sum is sum phi_a=[4]pi on Jacobians. These facts force the Gram matrix G=640*I12+20*ones12: diagonal 660, off-diagonal 20.")

#text("For either signed five-coordinate construction, put S=5*I5-ones5 and U_i=phi_(2^i) plus or minus phi_(-2^i), with the alternating sign in the odd case. In the first-four-coordinate model of A5, let F be the first four rows of S*U. The induced target polarization is H=I4+ones4, of degree 25. It must be retained in the adjoint calculation:")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("F * G * transpose(F) * H = 32000 * I4.", block: true)]

#text("Consequently r=lambda_C^(-1)*Psi^dual*lambda_A is an integral homomorphism A5 -> Jac(C), and Psi*r=[32000]. Since 32000-2909*11=1, r is a right inverse on A5[11] and injects its order-11 F2 subgroup. On all 11-primary torsion and the 11-adic Tate module, normalize r by the inverse of 32000. This gives a split summand, with no denominator at 11. Native Bezout certificates cover 11^k for k=1 through 8; the general conclusion uses coprimality. This says nothing about optimality or the kernel at other primes.")

#text("There is an explicit divisor-correspondence formula. For a geometric trace-zero tuple (P_i), sum the ten signed pullbacks phi_(+/-2^i)^*([5*epsilon_i*P_i]-[O]), with epsilon_i=1 in the even case and (-1)^i in the odd case. This represents r(P). The total positive and negative degrees are each at most 6600. The formula and its composition are established; its fibers have not been split into coordinates or reduced in a Jacobian implementation. The full geometry is written in PROOF_MAP.md; Lean proves the matrix identities and the group implications under the stated composition, rather than assuming a modular scheme library exists.")

#heading(level: 2)[#text("Quotient equations and the graph distinction")]

#text("Search all 390625 vectors in [-2,2]^8 in the cyclic O_-7 algebra. Of these, 410 give unimodular matrices. Each of the five distinct determinant-5 forms has ten hits. The records contain an explicit U with U^dagger*H_natural*U=H_candidate and determinant +/-1. Generated Lean certificates check both integral inverse identities and the alternating pairing identity for each form.")

#text("Thus q_U(a,Q)=U(a)-diag(Q) gives a principally polarized quotient onto Res_(F32/F2)(E). Its kernel is (U^(-1)diag(T),T), T in E[5]. The target equations are exactly the five F2 polynomials in ten affine variables from round 04; these are genuinely the same target, with a new explicit source map. The identity and mixed projective charts retain their round-04 scope. This gives a construction for both signs of every determinant-5 form.")

#text("The frozen first anti-isometries are checked separately. After U, their four torsion blocks agree, but only one of ten common blocks is +/-I. That hit has trace -1 and is the already natural graph. The nine other blocks are not silently replaced by a diagonal graph. Their equations, the trace+1 companion/CM symplectic identification, and all determinant-19 mixed ordinary/supersingular quotient equations remain open.")

#heading(level: 2)[#text("The first relation beyond the four base fibers")]

#text("The former function x(P)+1 is denoted X(P)+1 here, reserving lowercase x for a smooth supersingular deformation. Its zero polynomial is")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("b^15+b^13+b^12+b^10+b^6+b^2+1.", block: true)]

#text("It is irreducible over F2; exhaustive division by all possible proper small factors independently checks this. A root defines F_(2^15), with modulus bits 46149. The a=0 curve has 33000 points and trace -231. On the a=1 twist, (1,11254) has order 11; its trace is +231 and order 32538. The first run failed because a division-polynomial x root need not have a rational y on the chosen twist. That failure is preserved.")

#text("The discriminant -77711 is a 7-adic unit. The companion Frobenius on P1(Z/49) is therefore justified by the maximal unramified order, rather than guessed from the characteristic polynomial. Modulo 49 its matrix is [[0,13],[1,35]]. It has four orbits of length 14. The normalized eleven complements transform by r -> -r under Frobenius^15. Their product gives 44 closed zeros of degree 210, each simple: total degree 9240. None lies in the old factor base, whose closed degrees were 6 and 30.")

#text("The poles are now resolved as a complete geometric stratum. In the supersingular deformation y^2+s*x*y+y=x^3, normalize the ordinary curve by x=s^2*X+1/s, y=s^3*Y+1/s^3. Then")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("X=(s*x+1)/s^3,\nY^2+X*Y=X^3+s^(-3)*X^2+(1+s^3)/s^12.", block: true)]

#text("Every finite nonzero eleven-torsion section has pole order 3 in X. There are 56*660/12=3080 geometric supersingular points of C. Hence the full pole degree is also 9240. Monicity of psi11 in X excludes other poles at finite b, including the cusps. The deformation identity is checked by an integer identity certificate and by Lean; versality, torsion lifting and valuation are written geometric steps.")

#text("Every eleven-isogenous target of a supersingular input remains supersingular. The modular base has six such points, all in E49(F4), killed by 8. Native enumeration and maintained-library scalar multiplication check all six. The closed pole divisor's projected images lie in A5(F2), of order 11, and are killed by 8, so they vanish. Lean checks this coprime-torsion implication. The 44 zero columns therefore give the symbolic projected principal row (1,...,1).")

#text("Append those 44 disjoint columns to the old 544 and extend the old 102 rows by zeros. This one new row raises rank from 4 to 5 on 588 columns, checked by both finite-field elimination and separate BigInt elimination. The 44 new individual column images are not evaluated. The previously evaluated matrix remains rank 4/544. A geometric divisor certificate is not reported as a completed point-image replay or as scalar recovery.")

#heading(level: 2)[#text("Evidence, failures and next experiments")]

#text("")#link("PROTOCOL.md")[#text("Protocol")]#text(", ")#link("ledger.json")[#text("ledger")]#text(", ")#link("PROOF_MAP.md")[#text("proof map")]#text(", ")#link("VALIDATION.md")[#text("validation")]#text(" and ")#link("REPLAY.sh")[#text("replay")]#text(" record the bounds and requirement statuses. Raw inputs, unsuccessful runs and receipts are in ")#link("runs.tar.xz")[#text("runs.tar.xz")]#text(". The prior round-04 replay still matches all seven outputs byte for byte. No prior archive is rewritten.")

#text("The initial twist failure, overly narrow maximal-order screen, nine graph misses, missing Lake module declaration and failed Lean simplifications are retained. M6's rejected row and rank-4 outcome are historical outputs; M7 is a separately frozen experiment after an explicit protocol amendment.")

#text("Next: evaluate the 44 degree-210 closed-point images and check the new row directly in both target groups; split and reduce a reverse pullback divisor for a nonzero order-11 point; construct coordinates for one specified nonidentity gluing block or a determinant-19 quotient. No n=131 reverse transfer, individual modular descent, IC cost comparison or background execution is supplied by this round.")

#heading(level: 1)[#text("Formal proof map and written arguments")]

#text("All 18 new named theorems in KoblitzRound05 and KoblitzRound05Quotients are kernel checked and transitively audited. The matrix decisions use decide +kernel, not native_decide. Failed build logs may contain error-recovery sorryAx messages; those attempts are not proofs. The accepted modules have no such axiom. No full algebraic-geometry formalization is claimed.")

#table(columns: 3, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Claim")],
[#text("Lean coverage")],
[#text("Additional evidence or written bridge")],
),
[#text("Off-diagonal Gram entry 20")],
[#text("hecke_off_diagonal")],
[#text("Equal degrees, deck action and Hecke sum below")],
[#text("Polarized composition 32000, both signs")],
[#text("even/odd_polarized_composition; augmentation_square")],
[#text("Actual target polarization and integral adjunction below")],
[#text("11-primary reverse splitting")],
[#text("scalar_bezout; scalar_on_eleven_torsion; reverse_injective_on_eleven; reverse_right_inverse_on_eleven; bezout_reverse_right_inverse")],
[#text("Existence and F2 descent of the correspondence below")],
[#text("Five canonical quotient maps")],
[#text("polarized_equivalence_0 through _4")],
[#text("Integral CM endomorphisms, cyclic shift and natural quotient geometry")],
[#text("Ordinary zero support")],
[#text("missing_zero_degree checks degree arithmetic only")],
[#text("Exhaustive factor check, point count, maximal-order argument and finite orbit partitions")],
[#text("Supersingular poles")],
[#text("deformation_integral_lift; deformation_binary_identity; supersingular_image_zero")],
[#text("Fine-level local parameter, torsion lifting and automorphism count")],
[#text("Symbolic rank 5/588")],
[#text("No additional Lean rank theorem")],
[#text("Native and separate BigInt elimination; new point images not evaluated")],
)

#heading(level: 2)[#text("1. Why the Gram matrix is geometric")]

#text("Work geometrically first. Full determinant-fixed eleven-level structure modulo simultaneous sign has 660 bases. The map pi:C->X0(49) has degree 660. For a line a in P1(F11), phi_a divides the universal elliptic curve by that cyclic eleven-subgroup and transports its cyclic49 subgroup. The intermediate curve X0(539) has two degree-12 degeneracies and the remaining map from C has degree 55. Thus every phi_a has degree 660. These are morphisms of smooth proper modular curves, with good reduction at 2; translations from cusp basepoints do not change their Jacobian maps.")

#text("The deck group permutes the twelve maps by its action on lines. It is two-transitive: after sending one line to infinity, the upper triangular unipotents act transitively on the other eleven. Native enumeration independently finds 132 ordered-pair images. Hence every off-diagonal phi_a*phi_b^dagger is the same endomorphism beta of E49. Interchanging a,b also gives beta=beta^dagger. E49 is ordinary with imaginary quadratic endomorphism algebra, whose Rosati-fixed integral endomorphisms are Z; therefore beta is an integer scalar. The diagonal is [660] by degree.")

#text("The twelve maps sum to T11*pi, and T11=[a11]=[4] on the genus-one Jacobian X0(49). The minimal equation gives eight F11 points, so a11=4; native enumeration checks this and round 04 checks all supported finite Hecke sums. Pair the sum with its adjoint:")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("12*660+132*beta = 4^2*660, hence beta=20.", block: true)]

#text("The push-pull/adjoint interpretation is the standard ")#link("https://public.websites.umich.edu/~asnowden/teaching/2013/679/L16.html")[#text("Hecke correspondence construction")]#text(". The specific curve equations come from ")#link("https://arxiv.org/pdf/1110.6801")[#text("Kilford–McMurdy, section 2")]#text(". The Gram derivation above is ours, rather than a quoted identity.")

#heading(level: 2)[#text("2. Target descent, adjoint and saturation")]

#text("On the determinant-normalized F2 model, Frobenius sends a paired basis (P,Q) to (P^2,Q^2/2). Consequently phi_a(z)^2=phi_(2a)(Frob z). With i taken modulo five, the sums at a=2^i and -2^i transform by cyclic shift. The alternating differences transform by minus cyclic shift, because 2^5=-1 mod11. Augmentation S therefore descends the even tuple to A5(E1), and the odd tuple to its quadratic twist A5(E0). This is an algebraic moduli identity for every point, rather than an inference from the finite round-04 orbit checks.")

#text("Over the algebraic closure, A is sum-zero in E49^5. Its coordinates are (p1,p2,p3,p4,-sum p_i). Restricting the product polarization gives H=I4+ones4; this integral polarization has degree 25. If Psi:J(C)->A is the induced homomorphism, its polarized adjoint is r=lambda_C^(-1)*Psi^dual*lambda_A. The source polarization is principal, so r is integral even though lambda_A is not principal. In coordinates r has coefficients transpose(F)*H. Direct matrix multiplication gives Psi*r=[32000] for both signs. Lean checks all sixteen entries in each case.")

#text("Since gcd(32000,11)=1, this composition gives a left inverse for r over Z11. Thus r(T11 A) is a direct summand and is saturated at 11. On A[11] the scalar is already 1. On A[11^k], use a Bezout inverse of 32000 modulo 11^k; the generic Lean lemma checks this implication for any supplied Bezout certificate. The formula uses no division by 11. No claim is made that the integral quotient is optimal, that its entire kernel is connected, or that a divisor-class evaluator has been implemented.")

#text("Expanding transpose(F)*H on a five-coordinate sum-zero point gives 5*U^transpose*(P_i). Pulling back the degree-zero divisors of these ten points gives the reverse-divisor formula in REPORT.md. Pullback commutes with Frobenius, so this formula descends. For the odd sign, use the F4 isomorphism to E0 already given in round 04. Each nonconstant pullback has degree 660, yielding the 6600 bound for each sign part.")

#text("The two polarized-adjoint-norm identity certificates bind the universal quadratic consequences w^t*G*w=32000*v^t*H*v to this Lean source. Their issuer checks 64 evaluation points and the extra checker 128. The IDs are IDC1hf4cb17403b4831b1 (even) and IDC1h258cf4d7d3daf45d (odd). The matrix theorem, rather than a direct formal quadratic-form rewrite, is the attached Lean statement. Neither certificate proves the modular scheme or adjunction facts by polynomial testing.")

#heading(level: 2)[#text("3. Canonical quotient equations versus frozen graphs")]

#text("Let w^2+w+2=0 and O=Z[w]. On E0 take w=Frob2, and on E1 take w=-Frob2. These are actual integral endomorphisms over F2. The cyclic shift sigma on the five Weil-restriction factors commutes with O. Thus every U=sum_(i=0)^3 (a_i+b_i*w)*sigma^i in the records is an algebraic endomorphism of A over F2. Determinant +/-1 gives an integral O-matrix inverse, hence an automorphism. The five generated Lean certificates verify both inverses and the alternating polarization pullback on the eight-dimensional integral lattice; the Hermitian identities are also checked entrywise by a separate native path.")

#text("Composing the natural quotient with U gives q_U and its explicit kernel as stated in the report. This proves equations for that graph. A first frozen anti-isometry with a different common block C is a different geometric obligation. Checking only determinants, finite isotropy or the existence of a related canonical graph does not identify its quotient. The ten explicit block comparisons retain that distinction. In particular, changing from a trace+1 companion basis to the CM basis requires a symplectic torsion identification, not a silent integer sign change.")

#heading(level: 2)[#text("4. Ordinary zeros and Frobenius labels")]

#text("The native bivariate recursion checks psi11 is monic of degree 60 in X. At X=1 its b polynomial is irreducible degree 15. Its derivative is b^12*(b+1)^2, coprime to it, so all zeros are simple on the ordinary moduli curve. Forgetting cyclic49 and the normalized complement is etale there. There are 56 cyclic49 subgroups and eleven complements modulo simultaneous sign, giving 15*56*11=9240 geometric zeros.")

#text("For the point-rational twist, Frobenius^15 has polynomial T^2-231*T+32768. Discriminant -77711 is a 7-adic unit, so Z7[pi] is the maximal unramified quadratic order. A stable rank-one lattice over this DVR is free. Consequently its action on E[49] is conjugate to the companion matrix; this would require extra information if the order were nonmaximal. The lattice/Frobenius framework is standard in ")#link("https://www.numdam.org/article/ASENS_1969_4_2_4_521_0.pdf")[#text("Waterhouse")]#text(".")

#text("The enumerated four projective cycles have length 14. On E[11], the eigenvalues are 1 and -1, distinct; choose Q in the -1 eigenspace and rescale its pairing. Frobenius^15 sends Q+rP, after determinant normalization, to Q-rP. Enumerating the 616 product labels gives 44 orbits of length 14 over the b root, hence closed degree 210 over F2. The checker verifies every transition and the full partition. It does not produce coordinates for their cyclic49 subgroups or projected images.")

#heading(level: 2)[#text("5. The complete pole stratum and the symbolic row")]

#text("Use the smooth deformation E_s:y^2+s*x*y+y=x^3 at the unique supersingular elliptic curve in characteristic two. Its discriminant is 1+s^3. The Hasse coefficient a1=s is a uniformizer on a fine-level moduli chart. One can see its nonzero tangent directly: an admissible coordinate change sends a1 to a1/u in characteristic two, so it cannot remove a nonzero first-order a1 deformation. The moduli stack is smooth of dimension one. Prime-to-two level structures are etale over that chart, so s remains a parameter on C.")

#text("Nonzero eleven-torsion sections specialize to finite points of E_0, since E[11] is etale. The normalization x=s^2*X+1/s, y=s^3*Y+1/s^3 gives X=(s*x+1)/s^3, whose numerator is a unit. Thus X(P)+1 has pole order exactly three at every supersingular point. The integral polynomial identity, including its explicit correction multiple of 2, and the binary specialization are proved in Lean and bound to identity-supersingular-deformation-lift.json, ID IDC1h88fa2cda16d08555. The valuation argument and moduli-chart smoothness are written, not formalized.")

#text("The supersingular curve has 24 automorphisms; see the primary account by ")#link("https://arxiv.org/pdf/1004.3695")[#text("Zapponi")]#text(". Modulo +/-1 the group has order 12. It acts freely on full11 bases: if an automorphism fixes E[11] pointwise, its difference from identity has degree at least 121 unless zero, whereas a difference of two automorphisms has degree at most 4. The same argument applies when it fixes a basis up to simultaneous sign. Therefore there are 56*660/12=3080 geometric supersingular points on C. Their multiplicity-three poles exhaust degree 9240. Monicity in X implies integrality over F2[b], so no other poles exist at finite b. This proves the full divisor, not merely equality of its total degrees.")

#text("All phi_a targets of this stratum are supersingular modular base points. The explicit E49 model has six such points, all F4-rational and killed by 8, checked by the maintained binary library. The closed pole-divisor trace under the five-coordinate point map is therefore killed by 8. It lies in A5(F2), killed by 11, so it is zero. The generic coprime implication is a new Lean theorem. Principal divisors map to zero under the induced Jacobian homomorphism. Thus the 44 zero columns, each of multiplicity one, satisfy one projected relation, with pole columns omitted because their projected images vanish.")

#text("Their support is disjoint from all old columns, so a row with 44 ones there is independent of the old rank-four matrix extended by zeros. Two native arithmetic paths check rank five. This is a symbolic relation with a written geometric certificate. New point coordinates and individual images have not been replayed; the evaluated matrix is still rank 4/544.")
