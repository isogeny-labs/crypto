#set page(paper: "a4", margin: (x: 17mm, y: 15mm), numbering: "1")

#set text(font: "DejaVu Sans", size: 8.7pt)

#set par(leading: 0.38em)

#set heading(numbering: none)

#heading(level: 1)[#text("Round 15: one-prime CM carriers, theta descent and new coupled norms")]

#text("21 October 2026, America/Los_Angeles. Scientific parent1f733a2; execution baseline15c22cc67, including the user's merge of main.")


#text("The strongest new lead is a direct odd CM character of level7p. Exact Euler products and independent coset enumeration give candidate modular carriers of genus61 for n=5 and45589 for n=131. A geometric diamond quadratic twist supplies the required n=131 Koblitz sign. Separately, actual theta-class corrections specify a 3lambda target P242 for all four polarized quotients. Two new coupled norms have ordinary pushed degrees 460 and367. Full saturation, explicit proper equations and the new carrier maps remain open. No IC efficiency assessment is supplied.")


#image("routes.svg",width:100%)


#heading(level: 2)[#text("New modular carrier lead")]

#text("The previous ray twist of49a1 has level49p. Constructing the infinity-type1 character directly with an odd finite character at one split prime instead gives level7p. The sign condition makes gamma*eta(gamma) independent of the choice of principal-ideal generator. The modularity theorem, modular Jacobian factor and good reduction supply the written global bridge. CM15 then performs exact arithmetic in Z[alpha,zeta_n], including every coefficient-field conjugate and the complete Euler polynomial at2.")


#text(size: 7.2pt)[#table(columns: 6, inset: 3pt, stroke: 0.35pt + rgb("cbd5e1"),[#text("n")],[#text("auxiliary p")],[#text("level")],[#text("factor dimension")],[#text("character-cover genus")],[#text("untwisted Koblitz factors")],[#text("5")],[#text("11")],[#text("77")],[#text("8")],[#text("61")],[#text("A5(E1) × A5(E0)")],[#text("11")],[#text("23")],[#text("161")],[#text("20")],[#text("309")],[#text("A11(E0)²")],[#text("83")],[#text("499")],[#text("3493")],[#text("164")],[#text("54781")],[#text("A83(E1) × A83(E0)")],[#text("131")],[#text("263")],[#text("1841")],[#text("260")],[#text("45589")],[#text("A131(E1)²")])]

#text("The curve is the compact Gamma_H modular curve, H the kernel of the order2n nebentypus. CM15 independently enumerates its entire finite S,T coset permutations and checks S²=(ST)³=1 before counting genus. All four winners pass, within the frozen ten-million-coset cap. The coefficient-field assertion is separately certified by the full a2 Galois orbit, of degree2(n-1). Only the first qualifying prime in the bounded odd-k≤99 scan is selected; global genus minimality is not claimed.")


#text("The order2 diamond involution acts as -1 on every primitive conjugate. Its geometric F4/F2 twist exchanges E0 and E1 without changing genus. Thus the n=131 twisted carrier contains the desired A131(E0)² up to F2 isogeny, under the written modular/descent bridge. Its factor has dimension260; the desired trace-zero component has dimension130. The earlier genus378708949 carrier remains frozen historical evidence.")


#text("Retained LMFDB database queries independently confirm the level77 newform77.2.l.a, dimension8 and character order10, and its full local polynomial A5(E0)*A5(E1). The level539 even-twist control539.2.f.c has polynomial A5(E1)². Both are parsed and checked by CM15. The bounded query returns no matching131 row, which is missing coverage rather than a contradiction. Explicit Gamma_H models, diamond twist equations, correspondences and subgroup kernel checks have not been constructed.")


#pagebreak()

#heading(level: 2)[#text("Proper polarized target")]

#text("G15 uses the actual q5 source coefficient rotation, not the dual Kummer chart rotation. It verifies fifth power, Hermitian invariance and integral Frobenius binding. A kernel-sum argument gives the CM divisor bias ab*T for a+bw. All30 selected power-of2 norm controls per curve pass with inseparable multiplicities included.")


#text("The four Frobenius theta drifts are bits[7,2,7,2], corrected uniquely by [15,9,15,9]. All16 possible2-torsion drifts are checked. The supersingular companion has no nonzero geometric2-torsion. Corrected line bundles descend coherently from F32. Isotropic19-kernel theta lifts form a torsor under the actual Cartier character module. I-C is invertible for every graph; all361 drifts and361 character candidates are checked per graph, with a singular negative control. This proves rational theta descent under the written theta-group bridge, rather than assuming a rational lift from geometric isotropy alone.")


#text("The symmetric principal bundle Theta gives h0(3Theta)=243 and a very ample target P242, of degree29160. Pullback to B has87723 sections. The6lambda-to3lambda section-count ratio is exactly32. Actual section bases, theta-group action matrices, invariant F2 sections, proper coordinate ideals and their affine-chart identification remain open. This is finite correction data and a written bundle construction, not an implemented projective embedding or an IC runtime result.")


#heading(level: 2)[#text("New direct modular relations")]

#text("M15 recovers three Hecke norms as A(x)+B(x)y on E49:y²+xy=x³+x²+1 over F2. All1024 coefficients match;123904 input coefficients feed the cyclic121 norm of degree132. M16 exactly factors the quadratic norms, verifies products and Rabin conditions, resolves y residues and classifies every pushed closed point.")


#text(size: 7.2pt)[#table(columns: 6, inset: 3pt, stroke: 0.35pt + rgb("cbd5e1"),[#text("Function")],[#text("pole degree")],[#text("pushed cusp")],[#text("pushed SS")],[#text("pushed ordinary")],[#text("ordinary source degree under C5 bridge")],[#text("y0*yinf")],[#text("792")],[#text("0")],[#text("0")],[#text("792")],[#text("3960")],[#text("y0*yinf+1")],[#text("792")],[#text("12")],[#text("320")],[#text("460")],[#text("2300")],[#text("x0*yinf+1")],[#text("660")],[#text("3")],[#text("290")],[#text("367")],[#text("1835")])]

#text("The initial assumption that y0*yinf was a cusp unit failed. Its zeros are ordinary. The raw failed run and protocol amendment are retained. The corrected positive control proves exact norm multiplicativity: Norm(y0*yinf)=y^132*Norm(yinf), with all three F8 points y=0 explicitly disjoint from the actual cusps and supersingular locus. The asymmetric function reverses its reciprocal branch: large pole365, small245 and ten middle poles5 give660. Treating both large branches as365 would be incorrect. Both branch conventions are explicitly encoded and replayed.")


#text("The modularity/good-reduction/pole/Riemann–Roch identification uses the same written bridge as rounds13–14; finite q matching alone is not a global proof. New source fibers, projected images, complete source signed divisors and independent relation ranks are not yet computed. The product control is not promoted as a new independent coupled row.")


#text("A further exact identity links the coupled coordinates. Writing f=x+u, g=y+v, s=xu, r=yv and c=xv+uy gives c²-fgc+f²r+g²s=4sr, universally. In characteristic2 this is a direct relation among five modular functions. On f≠0 it expresses r rationally in the other four. Lean proves the integer identity. Whether this reduces source-fiber evaluation or yields useful extra principal rows is untested; algebraic dependence alone gives no relation-rank increment.")


#pagebreak()

#heading(level: 2)[#text("Full quotient saturation")]

#text("Q18 verifies the actual all-Q17 boundary models against the original full-coefficient programs. It forms a sound K-linear first-jet relaxation at h=t, omitting only coefficient-Frobenius derivative rows. Each graph is already inconsistent at the same original gate12: h*z=a², with base values z0=1 and a0=0. The exact contradiction certificate has one original row and coefficient bit0. Lean binds that gate and those base values to the retained literal programs and Q17 models; it also proves the universal coefficient obstruction.")


#text("This excludes h=t lifts at these points, including when all omitted Frobenius rows are restored. It does not exclude h=t² or more ramified arcs. The explicit h=t²,x=t,z=1 control passes. Consequently no boundary point is declared outside the projective closure, and full h-saturation is still uncomputed. Ramified lifting or a whole-ideal h-regularity certificate is the next discriminating test.")


#heading(level: 2)[#text("Evidence, failures and next acceptance tests")]

#text("The proof map separates universal Lean results, literal finite checks, native exact arithmetic and written geometry. Failed Rust builds, the rejected cusp-unit assumption, failed proof elaboration and the first underbound jet certificate remain in the raw archive. Only final passing proofs count. The new CM control test compares the exact norm with the independent level77 polynomial and rejects the wrong-sign alternative.")


#text("REPLAY.sh rebuilds12 new JSON outputs and three generated Lean modules, then checks five Lean-source-bound IDC records. Historical round14 source pins are checked against their immutable1f733a2 Git objects; round15 pins are checked against the current source tree. Frozen ledgers are preserved. The correctness suites and archive replay receipts are in VALIDATION.md. No wall-time comparison, solver benchmark or attack-cost row is promoted.")


#text("The immediate construction targets are a level77 F2 model with a verified n=5 factor map; actual3Theta section/descent matrices and a saturated coordinate ideal; ramified lifts of the Q17 models; and source fibers and images for the460/367 ordinary pushed supports. The recurring problem record retains each gap and its checkable acceptance criterion.")


#heading(level:1)[Source links]
#link("https://antsmath.org/ANTSXV/papers/ANTS-XV_molin-page.pdf")[Molin–Page, Theorem37] · #link("https://www.math.ru.nl/~bmoonen/BookAV/ThetaGr.pdf")[Moonen, Theta groups] · #link("https://www.math.uchicago.edu/~ngo/Shimura.pdf")[Genestier–Ngo, section2.2] · #link("https://www.lmfdb.org/ModularForm/GL2/Q/holomorphic/77/2/l/a/")[LMFDB77.2.l.a]
#text("Local evidence: CM.md, THETA.md, PROOF_MAP.md, ledger.json, runs.tar.xz, runs.sha256 and REPLAY.sh beside this PDF. Frozen failures and written global bridges remain explicit.")
