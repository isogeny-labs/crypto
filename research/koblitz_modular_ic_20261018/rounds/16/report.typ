#set page(paper: "a4", margin: (x: 16mm, y: 14mm), numbering: "1")

#set text(font: "DejaVu Sans", size: 8.6pt)

#set par(leading: 0.35em)

#set heading(numbering: none)

#heading(level: 1)[#text("Round16: affine CM carrier, cover bases and actual saturation separators")]

#text("Client date2026-10-21 America/Los_Angeles; execution date2026-10-22 UTC. Parent26b0d6f82349d35bf15b049f345f3ab2ebaa57f4. This is exact mathematical construction research on public family models. No IC timing, speed comparison or efficiency verdict is supplied. Global modular/abelian identifications remain written bridges alongside the finite native and Lean certificates.")


#text("The main new construction is an explicit F2 affine incidence for the genus61 level77 modular carrier, plus its diamond quadratic twist. It passes nonvacuous independent torsion controls. Two other tracks advance: explicit diagonal finite-cover section bases for all four polarized quotients, and a depth6 quartic separator rejecting all four old projective boundary models. The simple Frobenius sign-projector route fails integrality at11.")


#image("routes.svg",width:100%)

#heading(level: 2)[#text("Level77 equations and independent controls")]

#text("CM17 uses a Tate curve with a specified point of order11 and a line through an oriented7-torsion triple {Q,2Q,4Q}. Exact Add/Mul/Inverse circuits over F2 are exported for the original carrier and the diamond twist. Each inverse has its denominator retained as an equation. The written modular stabilizer identifies the smooth compact normalization with Round15's Gamma_H carrier of genus61. The proper normalization and Koblitz-factor correspondence have not been constructed.")


#text("The affine inputs are b,c,lambda,nu. The equations enforce5P11=-6P11, a separable cubic line intersection and doubling closure of its three points. The involution(lambda,nu)↦(lambda+1+c,nu+b) gives the diamond action. An explicit substitution over F4 derives the twist, including its changed elliptic coefficients and doubling numerator. Lean proves the polynomial substitutions, involution and the scalar implication8Q=Q⇒7Q=0.")


#text(size:7.2pt)[#table(columns:4,inset:3pt,stroke:0.35pt + rgb("cbd5e1"),[#text("base field")],[#text("order11 Tate bases")],[#text("original incidence")],[#text("twisted incidence")],[#text("F2")],[#text("0")],[#text("0")],[#text("0")],[#text("F4")],[#text("0")],[#text("0")],[#text("0")],[#text("F8")],[#text("0")],[#text("0")],[#text("0")],[#text("F16")],[#text("20")],[#text("0")],[#text("0")],[#text("F32")],[#text("15")],[#text("10")],[#text("10")])]

#text("For every base through F32, a separate generic group-law enumeration over F_(q³) recovers the actual7-torsion QR triples and tests q-Frobenius stability. Both original and twisted sets agree exactly with the equation circuits. The positive F32 control avoids the initial vacuous small-field checks. The2b conservative-operation-cap failure is retained; its8b repeat finishes with charge2415610624. That charge is an upper bound, not a measured arithmetic count or IC performance result. Details and all preconditions are in CM.md and the raw CM17 JSON.")


#pagebreak()

#heading(level: 2)[#text("Integral sign separation: a concrete negative result")]

#text("CM16 checks the two actual degree8 n=5 Koblitz trace-zero polynomials. Their resultant is29721861554176=2^24*11^6. Their primitive integer Bezout denominator is704=64*11. Their gcd modulo11 has degree6; other control primes3,5,7,13,19,23,29,31 give gcd1. Both polynomials have value11 at X=1.")


#text("Lean proves the actual integer Bezout identity and the obstruction to an integral coprimality projector modulo11. This blocks that sign-separation method on the rational11-torsion. It does not rule out other integral correspondences or an injective subgroup map. A future level77 transfer must compute its actual11-primary kernel, rather than assume rational isogeny splitting preserves the subgroup.")


#heading(level: 2)[#text("Explicit cover section bases")]

#text("G16 diagonalizes each actual determinant19 Hermitian polarization by an integral CM matrix B and checks B†HB=D with exact rationals. Selection is bounded to24 Gram–Schmidt coordinate orders, with no global optimality claim. Generated Lean checks all four actual integral Riemann-form congruences and section dimensions. These covers use the F32 splitting coordinates; F2 descent of the cover map is still open.")


#text(size:7.2pt)[#table(columns:5,inset:3pt,stroke:0.35pt + rgb("cbd5e1"),[#text("graphs")],[#text("diagonal entries D")],[#text("cover degree")],[#text("separable")],[#text("tensor sections")],[#text("0,2")],[#text("3,21,105,285")],[#text("99225")],[#text("yes")],[#text("8704314675")],[#text("1,3")],[#text("4,8,112,266")],[#text("50176")],[#text("no")],[#text("4401589248")])]

#text("The exact factor bases use1,x^i,x^i*y and, for a nontrivial Pic2 bias, (y+1)/x. The JSON stores each factor list and a lossless Cartesian-product descriptor, without materializing billions of sections. Dimensions equal 87723 times the cover degree. The odd cover on graphs0/2 is separable and a unit at11; the other two need scheme-kernel descent. Explicit trivializations against the original mixed divisor, theta-action matrices, the243 target sections and proper equations remain open. THETA.md records the precise divisor classes and remaining descent step.")


#pagebreak()

#heading(level: 2)[#text("Saturation: four old models excluded, four new survivors")]

#text("Q19 proves any putative arc through an old Q17 model needs ramification index divisible by6, and checks a precision96 base elliptic arc at index6. That arc covers only the initial elliptic subcircuit. Q20 propagates actual gate valuations and finds node80 would have value-1/3 in every graph. Its15-assignment witness uses no coefficient-Frobenius gate.")


#text("Q21 replaces the valuation argument with a polynomial ideal certificate. For actual source gates U=z19,V=z80 and x=input0, normalized source relations give h²F1,h²F2 in I, where F1 is cubic and F2 quartic. An explicit adjugate identity yields h⁶R in I. Thus R lies in the h-saturation, while R at the old boundary is c3³!=0. Lean proves the universal integer residual, source-depth implications, ideal membership and nonzero boundary evaluation; generated finite certificates bind the actual gates and scalar coefficients. Under the exported native field/circuit interpretation all four old Q17 points are outside the projective closure, for any ramification.")


#text("Adding the valid radical boundary consequence node19=0 and retaining all prior constraints gives four new checked survivors in four search visits each: unit nodes26,28,31,34,35 for graphs0/1 and25,27,30,33,34 for graphs2/3. Lean checks these finite models. They are not declared false points. Full saturation, proper boundary identification and closed quotient embeddings remain open. SATURATION.md gives the equations and scope.")


#pagebreak()

#heading(level: 2)[#text("Validation, retained failures and next experiments")]

#text("Seven new JSON outputs and five generated Lean modules are subject to byte-for-byte replay. There are63 new named Lean results, with a full project transitive axiom audit. Five identity.certificate/v1 records bind polynomial claims to their Lean source. PROOF_MAP.md distinguishes the kernel statements from written geometry and native implementation semantics. VALIDATION.md and ARCHIVE_VALIDATION.txt give the completed receipts.")


#text("Failed modular-gcd, compilation, Lean elaboration/unfolding and capped runs are retained with their corrections. The first CM17 controls were vacuous; the later exact positive controls supersede that validation, not its record. The old-head CI workflow budget violation is repaired by reducing its cap to25 minutes while retaining every test. Missing Chrome in the dashboard job and an unrelated sea_point_count Lean CLI failure are preserved as external CI failures; they are not patched around here.")


#text("Next experiments: normalize the level77 incidence and compute an integral correspondence with its11-primary kernel; descend the odd-cover tensor basis by explicit kernel actions; derive a source-bound separator or an actual arc for each new Q22 survivor. Round15's new norm source fibers and projected images are still unresolved. Round14's complete eight signed divisors and principal/deck nonzero-image ranks15/15 and18/17 remain frozen. No new principal-row rank is claimed.")


#text("Identity records: quartic separator IDC1h84c81bd7c932e9cf; line residual IDC1hea27af76151d3ffe; diamond curve twist IDC1h351b81f320ac1be8; doubling twist IDC1hd3e28c12fbda6d71; actual CM Bezout IDC1hf18c6163f50a7516. The full transitive audit checks468 project theorems,0 failing and0 trusted;63 new named results are counted separately.")


#heading(level:2)[Evidence links]
#text("Local files beside this PDF: CM.md, THETA.md, SATURATION.md, PROOF_MAP.md, ledger.json, runs.sha256, runs.tar.xz and REPLAY.sh. Global modular bridges follow Round15 CM.md; the present round supplies explicit affine circuits and partial kernel proofs, not a proper factor-map evaluator.")
