#set page(paper: "a4", margin: (x: 18mm, y: 16mm), numbering: "1")

#set text(font: "DejaVu Sans", size: 9pt)

#set par(leading: 0.42em)

#set heading(numbering: none)

#heading(level: 1)[#text("Round 13: ordinary degree500/580 supports and saturation beyond shared factors")]

#text("20 October 2026, America/Los_Angeles. Parent `7fde2b9498ec30ea2e111f1b178ae000e465331d`.")


#text("Both requested ordinary components now have complete closed supports, simple multiplicities and zero images in both retained projections, under the written modular norm, pairing and descent bridges. The antidiagonal consists of two degree250 points; product-plus-one consists of four degree145 points. The exact source towers are exported and reparsed. Full quotient saturation and proper polarized F2 embeddings remain open. No IC timing or efficiency assessment is supplied.")


#image("routes.svg",width:100%)


#heading(level: 2)[#text("Requirement-to-evidence status")]

#table(columns: 3, inset: 4pt, stroke: 0.35pt + rgb("cbd5e1"),[#text("Requested item")],[#text("Status")],[#text("Evidence and remaining gap")],[#text("Ordinary antidiagonal500")],[#text("Complete under written bridge")],[#text("O13–O18; 2 simple closed points of degree250; both projected images zero")],[#text("Ordinary product-plus-one580")],[#text("Complete under written bridge")],[#text("O13–O18; 4 simple closed points of degree145; both projected images zero")],[#text("Full quotient saturation")],[#text("Partial")],[#text("Four stages of actual false points; certified Q13/Q14/Q15 repairs; complete saturated ideals not computed")],[#text("Proper polarized closed embeddings")],[#text("Partial")],[#text("Existing affine F2 models and Kummer atlas retained; complete F2 projective boundaries and polarization still open")],[#text("Relevant Lean proofs")],[#text("Partial")],[#text("63 named structural, actual-circuit and bit-checker results; global geometry and implementation semantics remain written")],[#text("Complete coupled principal rows")],[#text("Partial")],[#text("Requested ordinary components exhausted; whole row assembly and five other ordinary components remain open")])

#heading(level: 2)[#text("Exact Hecke norm functions")]

#text("Forgetting the scalar full-level basis gives the cyclic121 Hecke correspondence of degree132. The remaining source cover has degree5. On the retained minimal X0(49) model y²+xy=x³+x²+1, we recover the norms of x0+xinf and x0*xinf+1 as A(x)+B(x)*y with pole bounds504 and528. The complete coefficient polynomials are in O13. Quadratic norms to F2(x) have degrees504 and528.")


#text("Native binary series use640 output and77440 input coefficients. The 121-fold branch norm is a tower of two11-by-11 multiplication determinants. Controls include norm(1+t), Euler-product norm identities at all640 coefficients, multiplicativity, direct F1024 roots of unity, and the minimal elliptic equation. Both recovered functions match all640 coefficients, including135 and111 positive coefficients. Global equality also requires the written modular/Riemann–Roch argument in PROOF_MAP; finite matching alone is not asserted to prove it.")


#text("Exact binary factorization reconstructs both norm polynomials. Every irreducible factor passes Rabin conditions. Resolving y residues, common factors and ramification gives the complete pushed zero partitions:")


#table(columns: 5, inset: 4pt, stroke: 0.35pt + rgb("cbd5e1"),[#text("Function")],[#text("Cusp degree")],[#text("SS degree")],[#text("Ordinary degree")],[#text("Total")],[#text("x0+xinf norm")],[#text("84")],[#text("320")],[#text("100")],[#text("504")],[#text("x0*xinf+1 norm")],[#text("0")],[#text("412")],[#text("116")],[#text("528")])

#text("Multiplication by the scalar cover degree5 reproduces the independent round12 ordinary totals500 and580 and SS totals1600 and2060.")


#heading(level: 2)[#text("Explicit ordinary supports and five-sheet lift")]

#text("The pushed antidiagonal has two degree50 points on the same binary x-polynomial `7b52a5ad2a56f`, with y residues `d77b9d278afc` and `d77b9d278afe`. The product-plus-one support has four degree29 points:")


#table(columns: 2, inset: 4pt, stroke: 0.35pt + rgb("cbd5e1"),[#text("Binary x-polynomial")],[#text("Binary y-residues")],[#text("2524a953")],[#text("14e29a61, 14e29a63")],[#text("32a54929")],[#text("5dd8635, 5dd8637")])

#text("All six multiplicities are1. O15 finds exactly one compatible middle level49 triple for each point. For product-plus-one the target is P+T; the alternative T-P has common-j counts[0,2,0] and no compatible triple. These are exact residue-field calculations, not sampled rational points.")


#text("O16 factors the degree24 and60 division polynomials, reconstructs actual 7/11 cyclic kernels, and verifies four commuting squares for every point. The first11-kernel polynomial is irreducible of degree5 over each base field. Via the written fixed-Weil-pairing bridge, its x-coordinate algebra is the entire full-level scalar fiber. Thus the source degrees are [250,250] and[145,145,145,145], all simple. Simplicity of the pushed norm and the divisor norm formula exclude additional intermediate points. The etale degree5 cover then exhausts the ordinary source support.")


#text("O18 separately parses the exports, reconstructs both pushed divisors, checks their elliptic equations, kernel memberships, doubling invariance, all24 commuting squares, and irreducibility of all six degree5 extensions. It shares arithmetic primitives with the producer; no external CAS or independent implementation audit is claimed. The three base Rabin bit-checks also have kernel-evaluated Lean certificates. A formal theorem connecting that checker to irreducibility remains open.")


#heading(level: 2)[#text("Both projected images vanish structurally")]

#text("Each new closed source point is the entire scalar deck fiber over its pushed point. The order5 deck group permutes the five even tuple entries. The parity correction in the odd tuple gives the identical unsigned permutation. The actual25 cases are kernel checked. Summing an augmented entry 5*v_i-sum(v_j) over the deck group is zero integrally; summing over the base Frobenius orbit and applying the retained additive transfer and Frobenius projections still gives zero.")


#text("This evaluates all six closed divisor images in both signs. Pointwise projection coordinates were not newly computed; O17 records that flag explicitly. There are no new nonzero columns. Retained ranks stay12 on 1308 evaluated columns and nonzero-image rank11 on1165/1160 columns. No new whole principal row or rank promotion is performed.")


#heading(level: 2)[#text("Saturation: actual repairs and counterpoints")]

#text("All searches retain every original equation and interpret scalar models in the full360-coefficient ring. Nonbinary constant relations impose all coefficient equations. Bounds are frozen before arithmetic in the protocol.")


#table(columns: 4, inset: 4pt, stroke: 0.35pt + rgb("cbd5e1"),[#text("Family")],[#text("Graphs0/1 each")],[#text("Graphs2/3 each")],[#text("Outcome")],[#text("Inverse cancellation Q13")],[#text("23")],[#text("22")],[#text("A counterpoint passes all constants and violates a new quadric")],[#text("Shared-factor Q13")],[#text("3826")],[#text("3774")],[#text("A counterpoint passes every inverse repair and violates a new quadric")],[#text("Three-leaf associative linear")],[#text("0")],[#text("0")],[#text("Empty family; no search query performed")],[#text("Four-leaf two-product Q14")],[#text("4")],[#text("4")],[#text("A counterpoint passes all Q13 repairs and violates Q14")],[#text("Square of a linear form Q15")],[#text("5")],[#text("5")],[#text("A counterpoint passes all Q13/Q14 repairs and violates Q15")])

#text("Q14 examines53956/52326 output pairs per graph, below its250000 cap. Q15 examines35 square gates per graph:26 relations are trivial, four need unavailable atom squares, and five are emitted. No64-atom cap is hit. Their conservative h-membership depth is at most5. All360 scalar linear equations of every Q15 relation are imposed. After all five repairs, nonzero restricted boundary points still survive on every graph. Those last points alone do not prove failure of saturation; a further violated certified relation or a complete elimination is needed.")


#text("The32 actual-circuit Lean certificates verify the displayed intermediate counterpoints and their violated relations. Universal membership and characteristic-two square identities are checked separately. None of these bounded families is relabeled full h-saturation. The missing exit is a complete saturated projective presentation with verified charts, gluing and proper polarization. The existing six-chart Kummer atlas over K from round08 does not discharge this F2 projective boundary obligation.")


#heading(level: 2)[#text("Failures, controls and reproducibility")]

#text("The first residue-field inverse implementation initialized its exponent loop with a instead of1. Exact multiplication rejected it immediately. The corrected implementation passes exhaustive F16 inverse and Artin–Schreier controls; the failed log is retained. The first full-fiber run verifies one degree250 point, then reaches the frozen100-million reduction cap. AmendmentB raises only that command to one billion before repetition. The accepted fiber construction uses321809763 binary reduction steps. No timing or speed comparison is inferred.")


#text("Failed Rust syntax/import and Lean elaborations are retained. Literal array-based circuit certificates consumed tens of GB during kernel reduction; their incomplete/interrupted attempts are preserved. The final representation records the same sparse point and every original gate and quadric, with kernel evaluation. The proof map distinguishes checked statements from native semantics and global geometry.")


#text("Validation receipts, exact source/raw hashes and IDC IDs are in the ledger and deterministic archive. Replay reconstructs all15 JSON/checkpoint outputs and all three generated Lean modules byte for byte. The PDF and SVG are reviewed together. Canonical frontier pages and the construction problem are updated; historical cost and rank charts retain their data because this round supplies no new cost measurement or rank.")


#text("The next discriminating work is full projective saturation beyond these families, actual assembly/comparison of the two now-exhausted principal rows, and ordinary supports of degrees1760,1880,2760,1370,2180. Their acceptance criteria remain all closed points, local multiplicities, projected images and exact pole accounting. Global norm/moduli/pairing formalization remains a separate proof obligation.")


#heading(level:1)[Evidence paths and review]
#text("Raw outputs are in the deterministic runs.tar.xz archive; runs.sha256 pins every member. REPLAY.sh reconstructs all outputs. PROOF_MAP.md states each formal theorem and every written bridge. ledger.json contains source hashes, IDC IDs, requirement status and audit totals. ARCHIVE_VALIDATION.txt records extraction, replay and deterministic repack.")
#text("Editable visual: render.py and routes.svg. This document is rendered entirely from REPORT.md plus the same native JSON records used for routes.svg. No mathematical search or arithmetic is performed by this layout script.")
