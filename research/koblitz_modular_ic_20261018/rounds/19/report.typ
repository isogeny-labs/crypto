#set page(paper: "a4", margin: (x: 16mm, y: 14mm), numbering: "1")

#set text(font: "DejaVu Sans", size: 10.5pt)

#set par(leading: 0.38em)

#set heading(numbering: none)

#heading(level:1)[#text("Round19: controlled poles, mixed sections and coupled elimination")]

#text("Client date 23 October 2026; native execution 23 October UTC. Scientific parent a2b8f27be5fb3faac9362430ef43245b3bce819d. Earlier evidence is frozen. This report continues the explicit integral level77 map, mixed F2 section construction and four Q28 boundary models. No IC cost or efficiency assessment is made, and no rank or divisor row is promoted.")


#text("The new evidence excludes the registered controlled-pole map spaces and the four Q28 models, constructs 41 mixed sections with the correct scalar-cover polarization, and identifies a primitive orbit obstruction that the refined cover clears. The original map, canonical 243 sections and whole quotient saturation remain open.")


#image("routes.svg",width:100%)

#text(size:8.5pt)[#table(columns:3,inset:3pt,stroke:0.35pt + rgb("cbd5e1"),[#text("track")],[#text("observed or derived result")],[#text("requirement status")],[#text("CM23/24 controlled poles")],[#text("335,544,320 x numerator/sign cases;64 constants;no nonconstant finite fit")],[#text("bounded search complete;integral factor map open")],[#text("G22/G25 mixed sections")],[#text("six auxiliary controls and41 correctly polarized sections;32 tuples;48 scalar-kernel noninvariance witnesses")],[#text("correct polarization constructed for graphs 0/2;canonical class and kernels open")],[#text("G23/24 polarization")],[#text("390,625 integral rows per form give no match;exact modulo 5 obstruction excludes every integral orbit-transfer sum for H and3H")],[#text("specific construction route excluded;general mixed sections open")],[#text("Q29--32 saturation")],[#text("four Q28 and four Q30 exclusions;266 pure-source resultants;four Q32 models retained")],[#text("source elimination steps complete;whole saturation open")],[#text("Lean")],[#text("79 new named results")],[#text("formalized implications and finite bindings;geometric/native limits explicit")])]

#pagebreak()

#heading(level:2)[#text("Controlled-pole functions on the level77 chart")]

#text("CM23 searches x=N/q² and y=M/q³ for q in 1,b,c,1+c,b+(1+c)c. N is affine in b,c,lambda,nu over F32; M ranges over the 15 quadratic monomials. Every family examines 32^5 numerators for each of the two elliptic signs. All y solutions are obtained by F2 linear algebra on 75 coefficient bits, using ten original incidence points and eight independent F1024 holdouts. The JSON records skipped zero denominators; no affine solution space exceeds the enumeration cap.")


#text(size:8.5pt)[#table(columns:4,inset:3pt,stroke:0.35pt + rgb("cbd5e1"),[#text("denominator")],[#text("valid x cases a=0")],[#text("valid x cases a=1")],[#text("surviving maps")],[#text("1")],[#text("1935")],[#text("11")],[#text("43+21 constants")],[#text("b")],[#text("1934")],[#text("6")],[#text("none")],[#text("c")],[#text("1978")],[#text("4")],[#text("none")],[#text("1+c")],[#text("1991")],[#text("5")],[#text("none")],[#text("b+(1+c)c")],[#text("1918")],[#text("4")],[#text("none")])]

#text("The coefficient spaces therefore contain no nonconstant map satisfying these necessary actual-point constraints. Lean independently recomputes all 64 constant residues. Native controls check the identity map and its elliptic conjugate on an a=0 chart, and compare complete linear solution spaces with brute force. The search is not a proof that general rational maps do not exist.")


#text("If D is the sum of the coordinate pole divisors, x and y have poles bounded by D+2(q)_0 and 2D+3(q)_0. Exact supports and multiplicities have not been computed. Thus these are controlled divisor bounds, while a complete prescribed cuspidal Riemann--Roch space and the proper model remain missing. The next map construction needs those spaces and a full function-field identity, then a checked 11-torsion image and divisor map. CM.md retains the exact field, basis, clearing identity and scope.")


#pagebreak()

#heading(level:2)[#text("Correctly polarized mixed sections and kernel limits")]

#text("G22 first constructs six auxiliary orbit sections and checks 768 torsion translations. Its ordinary polarization has dimension 80 before scalar pullback. Including the actual degree 57 companion gives full dimension 67371968236781250000, not divisible by 361. That excludes this auxiliary product from the final degree 361 gluing descent. The companion-strengthened record is retained alongside the initial ordinary-only record.")


#text("G23 finds no match for either actual Hermitian H in the complete[-2,2]^8 integral orbit-row box. G24 proves the primitive obstruction modulo 5: every matrix transfer sum(R^j)^t M R^j kills the fixed vector(1,1,1,1), whereas both H and 3H do not. It excludes every integral orbit-Gram sum for those primitive forms, with no coefficient cap. The scalar 105 pullback is divisible by 5 and is outside that obstruction.")


#text("G25 uses precisely that distinction. The first two rows of the actual C can be divided by 5. Their five rotations, with the other two C rows, and weights(45,315,63,171) have exact Hermitian Gram sum 33075 H, which is3*105² H. This gives an explicit effective F2 mixed divisor with the correct scalar-cover polarization for graphs 0/2. Four Lean certificates check its Gram sum, rotation cycles, weights and actual C source binding.")


#text("Elementary symmetric orbit coefficients of both x and y give 41 sections of this bundle. All 16 tuples per sign pass rotation and twisted Frobenius. Every one of the 24 tested scalar-kernel translates per sign changes a section ratio, giving 48 exact noninvariance witnesses. The sections also have a forced fixed divisor with weights(42,312,60,168). They are not a basepoint-free system; linear independence is not claimed.")


#text("The next obligations are canonical symmetric Pic2 class matching, use of the full weighted factor-section spaces, and actual kernel theta actions. A correct polarization alone does not supply those. The canonical 243 target sections, the other two graphs and proper quotient embeddings remain open. THETA.md gives the formulas, the corrected companion accounting and the Riemann--Roch/descent references.")


#pagebreak()

#heading(level:2)[#text("Coupled source elimination and the remaining boundary")]

#text("Q28's supported pure-x nodes already have the maximum degree9, so the proposed higher-degree univariate comparison fails. The actual y*S coordinate has higher pole order. Q29 uses the elliptic norm of V=y*S, with deg S=8, and takes its determinant in the algebra P(x)=U, deg P=9. Each graph yields 191 terms, exact zero after elliptic substitution and a unique total-degree19 term U^19. Its projective homogenization forces U=0 on the reduced boundary, rejecting each actual Q28 point with U=1.")


#text("Q30 keeps every previous constraint and returns models on[120,122]. Q31 compares all actual lower-degree pure-x source polynomials with P. The66,66,67,67 distinct resultants cover70,70,71,71 source aliases. Every exact univariate substitution is zero; the monic boundary term forces the lower source coordinate to vanish. All four Q30 models fail.")


#text("Q32 retains all original Q13--Q17 constraints, the five cube/square repairs and every new radical consequence. It returns four complete checked uncapped models on[123,124,126,127], in9 visits each. They are unclassified. A complete simultaneous source arc must include all inverse witnesses, Kummer roots and coefficient-Frobenius components. A partial leading-pole calculation is insufficient. Alternatively another actual source separator is needed. Whole saturated-ideal equality and proper polarized embeddings remain open.")


#text("These eliminations use no coefficient-Frobenius valuation assumption. The norm and resultant identities hold on the actual affine source, so their homogenizations belong to its h-saturation under the source bridge. No minimal h-power certificate or equality of whole ideals is asserted. SATURATION.md contains the complete written implications and proof limits.")


#pagebreak()

#heading(level:2)[#text("Verification, failures and reproducibility")]

#text("The proof map lists 79 new named Lean results. They cover the universal norm, weighted map, determinant, monic-boundary, orbit and transfer implications; actual finite source/leading-term/model certificates; and constant-map residues. The final transitive audit and full test receipts are recorded in ledger.json and VALIDATION.md. Only propext, Classical.choice and Quot.sound are allowed; no custom claim axiom, sorry or trusted native evaluation is accepted.")


#text("The complete searches and large-field substitutions remain native Rust. Full modular, abelian, field, divisor and scheme geometry is not formalized. In particular the concrete native norm matrix is not connected to its evaluation vector in Lean, and the full search exhaustion is not a kernel proof. Written geometry and native certificates are not relabelled as formal results.")


#text("Retained failures include a Rust JSON macro error, incorrect graph2 cubic gate extraction, initial Lean unit/noncomputable/notation errors and a large-expression elaboration limit. The equivalent monomial-list proof passes; the original failed source and interrupted duplicate compile are retained. The first root suite fails an unrelated F4 cost-count threshold (19 rather than 20); the isolated test and full rerun pass. No research conclusion uses that first failure. G22's auxiliary-degree obstruction, G23's bounded null and every new saturation survivor remain visible.")


#text("REPLAY.sh reruns 15 commands, compares 15 JSON outputs and seven generated Lean modules byte for byte, and checks three source-bound identities with 128 additional parsed checks each. Deterministic raw and validation archives include fresh extraction, member hashes and identical repacking. The final source pins are checked separately from frozen Round18 pins. The SVG and 150dpi PNG use recorded values, and every PDF page is inspected. These checks provide reproducibility, not an independent-host audit or an IC efficiency result.")


#heading(level:2)[Evidence and sources]
#text("Companion files: CM.md, THETA.md, SATURATION.md, PROOF_MAP.md, SOURCES.md, ledger.json, VALIDATION.md and REPLAY.sh. Primary sources read: Moonen, The cohomology of line bundles, 9.11–9.14 (Riemann–Roch and isogeny pullback); Moonen, The theta group of a line bundle, 8.10–8.12 (descent and the arbitrary-field caveat). Native arithmetic, written geometric bridges and kernel-checked implications are distinguished throughout.")
