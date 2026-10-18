#set page(paper: "a4", margin: (x: 17mm, y: 17mm), numbering: "1", header: align(right, text(size: 8pt, fill: rgb("666666"))[Three ECDLP leads · 2026-10-18]))
#set text(font: "DejaVu Sans", size: 10.3pt)
#set par(leading: .6em, spacing: .85em)
#set heading(numbering: none)
#show link: set text(fill: rgb("235789"))
#show heading.where(level: 1): set text(size: 19pt)
#show heading.where(level: 2): set text(size: 13pt)
#show heading.where(level: 3): set text(size: 10.5pt)
#show table.cell: set text(size: 8.7pt)
#heading(level: 1)[#text("Audit of three ECDLP index-calculus leads")]

#text("2026-10-18. Correctness, literature and evidence audit. No new timing or end-to-end speedup claim. Repository base: ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "1e2bf0715a2a927afc92e6730d6e65b59f15ab9e")#text(".")


#text("The trace obstruction in lead 1 is sound for ordinary binary curves in the stated normalized model. Its consequences need correction: it is not specific to quadratic extensions, it does not imply a 3/8 limit, and it rejects no target in an odd-order subgroup. Lead 2's coverage result leaves the solving question open, but later repository experiments already test several solvers with negative results. Lead 3 has a plausible established transfer framework; its first-cycle extraction and numerical ranking require the original code.")


#text("The exact producers of the supplied 21-curve, 18-cell and 1,260-run studies were not identified in the three named local repositories. Their quoted statistics are ")#strong[#text("unverified original claims")]#text(", not results reproduced here. The fresh checks below are separate, fully specified small-instance diagnostics. See ")#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/problems/three-leads-original-evidence-unlocated.md")[#text("the unresolved evidence record")]#text(".")


#heading(level: 2)[#text("Claim-by-claim disposition")]

#table(columns: (1fr, 2fr), inset: 4pt, stroke: .3pt + rgb("cccccc"), table.header([#text("Supplied claim")], [#text("Disposition and required qualification")]), [#text("Odd characteristic, n=2..5: coverage matches random")],
[#text("Possible empirical finding for the original population; not a universal identity. Fresh tiny-field counterexample below.")],
[#text("Weil descent creates no extra relations")],
[#text("Coverage alone cannot establish this as an algorithmic conclusion. Tuple counts fix mean multiplicity; descent can change solving cost.")],
[#text("Binary n=2 has a trace obstruction")],
[#text("Confirmed under ordinary normalized-model hypotheses; published in Kosters–Yeo, Proposition 4.2 and Remark 4.8.")],
[#text("Coverage tends exactly to 3/8, z=-4.7, checked on 21 curves")],
[#text("Original data and significance calculation unavailable. Half-group theorem alone is insufficient; descended-coefficient families behave differently.")],
[#text("About 4.7% fewer relations per target")],
[#text("Correct arithmetic for the relative reduction in successful-target probability from 1-exp(-1/2) to 3/8; not a reduction in mean total decomposition count at fixed base size.")],
[#text("Free trace test skips impossible targets")],
[#text("Cheap full-group filter; no rejection of targets already in an odd-order subgroup. Basis and target distribution must be stated.")],
[#text("n=2 costs 2.7–66 times rho; root step about 1,000M")],
[#text("Original implementation and pricing cells unavailable. Neither figure is a lower bound for all n=2 methods.")],
[#text("PKM: 18 cells, at least 23 primes, all abs(z)<=1.84")],
[#text("Original raw records unavailable. Failure to reject equality is not an equivalence test.")],
[#text("PKM advantage would have to come from solving")],
[#text("Sound distinction for matched actual base sizes. Later solver evidence narrows the open question; it does not prove impossibility.")],
[#text("Genus-2 curve and map tau passed all checks")],
[#text("Construction has prior art; exact executable map, kernels, models and subgroup projection remain unverified.")],
[#text("1,260 runs through 40 bits, all kP checked")],
[#text("Unverified original census. Correct recovered scalars would establish sampled output correctness, not completeness or unbiased costs.")],
[#text("IC .27–.37p versus rho .31–.44p")],
[#text("Unverified original counters; needs actual subgroup order, walk and trial definitions, cycle signs and full costs.")],
[#text("Solves at first large-prime cycle")],
[#text("False as a general graph assertion. A usable signed dependence and nonzero target coefficient are necessary. Original implementation may already enforce them.")],
[#text("Independent trials rule out rho hiding in a walk")],
[#text("Useful control for walk dependence; birthday-order collisions remain possible, and uniform-trial generation has a cost.")],
[#text("41M+I versus 12M+I")],
[#text("Formula/backend-dependent claim; field, coordinate, squaring and inversion units must match.")],
[#text("Ratio .51–2.83; rho wins 10/12 cells at 40 bits")],
[#text("If accurately computed, the band crosses 1 and the overall ranking remains unresolved. Twelve selected prices are scenarios, not probabilities.")])

#heading(level: 2)[#text("1. Subfield decomposition")]

#heading(level: 3)[#text("Trace theorem, model and target subgroup")]

#text("Let K=F_(2^(mn)), k=F_(2^m), and use the ordinary model ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "E: y^2 + x*y = x^3 + a*x^2 + b")#text(", with b nonzero. Define chi(O)=0 and chi(P)=Tr_(K/F_2)(x(P)+a) for affine P. Kosters–Yeo identify this homomorphism's kernel with 2E(K). In a general Weierstrass model the coefficient normalization matters: their expression is Tr((x+a2)/a1^2), with a1 nonzero. This argument does not cover a1=0. ")#link("https://arxiv.org/html/1503.08001")[#text("Kosters–Yeo, Proposition 4.2 and Remark 4.8")]#text(".")


#text("For even n, every x in k has absolute trace zero because its relative trace to k is n*x=0. Therefore all factor-base points have the same chi value, c=Tr(a). A sum of r such points has chi=r*c in F_2; in particular, every even-length sum lies in 2E(K). This includes n=4 with four summands. The field trace of an x coordinate is different from the elliptic-curve point trace P+P^q.")


#text("Every odd-order subgroup G is contained in 2E(K): doubling is invertible on G. Thus chi(aP+bQ)=0 whenever P,Q belong to G. The full-group statement “half the targets are impossible” cannot be used as a half-target saving for this usual DLP sampling law. If full-group targets really are sampled, the filter can save decomposition calls. Its bit operations and basis conversion still belong in a complete cost ledger.")


#text("In the particularly important case where G=2E(K) has odd order, full-group coverage 3/8 would mean coverage 3/4 on uniformly sampled G targets, because every reachable point is in G. This is a conditional consequence of the proposed 3/8 value, not a new proof of that value. For a proper subgroup of 2E the distribution requires its own analysis. A random-base comparison must match both subgroup projection and target population; the 4.7% full-group difference does not give the attack's relation yield.")


#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/TraceKernel.lean")[#text("TraceKernel.lean")]#text(" formally proves the same-coset and odd-order consequences ")#strong[#text("conditional on a Boolean homomorphism")]#text(". The ")#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/lean-gate-v2/research_three_leads_audit_20261018_TraceKernel_lean/audit.log")[#text("successful proof gate")]#text(" audits seven declarations, including three generated recursive equations, with zero failing or trusted declarations. The elliptic-curve homomorphism, its kernel and the field-trace bridge are cited mathematics and finite checks, not formalized by this file.")


#heading(level: 3)[#text("What the coverage numbers mean")]

#text("The usual independent occupancy approximation is ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "1-exp(-lambda)")#text(", with lambda approximately B^n/(n!*N) for signed point-base size B and group population N. At B approximately q and n=2, lambda is about 1/2 and the model gives 0.39346934. This is a heuristic occupancy model, not the exact law for every random sign-closed x-set. The half-group theorem gives a 1/2 upper bound; it does not select 3/8. The quoted z=-4.7 needs its actual null variance and sampling unit; correlated targets on one curve cannot automatically be counted as independent evidence across curves. A discrepancy from a Poisson formula alone is not a calibrated test against matched random factor bases.")


#text("Summation-polynomial zeros are defined using points over the algebraic closure. Especially at higher arity, x coordinates in the desired field do not alone certify usable rational-point relations. Check every lift and the actual signed group sum. The fresh coverage censuses use rational points and group sums directly; they do not count unverified polynomial zeros as relations. This distinction is explicit in Kosters–Yeo, Proposition 2.1 and Remark 4.8.")


#text("Coverage and relation multiplicity are different. If D(R) counts all ordered decompositions over a fixed base, then exactly ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "sum_R D(R)=B^n")#text(". For unordered tuples, use the chosen distinctness and repetition convention instead. A different sumset redistributes the same tuple population. A lower fraction of successful targets therefore does not imply fewer mean relations when every decomposition is returned. The approximately 4.7% statement applies to a first-success-per-target strategy under the quoted occupancy comparison.")


#text("There is also exploitable algebra in binary n=2. Write t=x(R), u=x1+x2 and v=x1*x2. The normalized S3 equation is ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "v^2 + t*v + t^2*u^2 + b = 0")#text(". For t outside k, put T=t+t^q, N=t*t^q and b=b0+b1*t. Splitting coefficients gives ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "v=T*u^2+b1")#text(" and, with w=u^2, ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "T^2*w^2+N*w+b1^2+b0=0")#text(". Solve this quadratic over k, take square roots, split X^2+uX+v, then verify rational point lifts and the group sum. Identity and other exceptional inputs need explicit handling. This reduction is an algebraic derivation, not a measured implementation or a formal identity certificate. The repository's v1 identity-certificate field is odd characteristic; it must not be used to certify a characteristic-two calculation.")


#text("This explains why generic nonidentity targets have at most two candidate unordered x-pairs; it also motivates a specialized solver rather than treating 1,000M as intrinsic. It does ")#strong[#text("not")]#text(" prove the asymptotic distribution of the candidates' splitting and lifting events. The exact 3/8 limit remains a separate counting obligation. Related linearized binary decomposition predates this audit; see ")#link("https://eprint.iacr.org/2016/003")[#text("Courtois 2016/003")]#text(" and the local ")#link("https://github.com/aburan28/cryptanalysis/tree/80868b829255312a5e41defb7d5b3a2d580ad90f/experiments/linearized-half-decomposition")[#text("linearized-half-decomposition study")]#text(". An abstract's decomposition bound is not an end-to-end sub-rho result.")


#heading(level: 3)[#text("Fresh binary checks")]

#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/binary_check.rs")[#text("binary_check.rs")]#text(" uses independent polynomial-basis arithmetic and complete point/sumset enumeration. It checks 35 curves with b outside k, four descended-b controls and two degree-four-extension controls: ")#strong[#text("41 curves")]#text(", not a replay of the original 21. All arithmetic, trace, doubled-image, odd-order and group-sum assertions passed. The smallest quadratic cases also compare S3 with actual group decompositions. An additional assertion pass checks the coefficient-splitting derivation above on those cases and reproduces the same coverage CSV.")


#table(columns: (1fr, 1fr, 1fr, 1fr), inset: 4pt, stroke: .3pt + rgb("cccccc"), table.header([#text("Base q")], [#text("Generic curves")], [#text("Mean two-sum coverage")], [#text("Observed range")]), [#text("8")],
[#text("7")],
[#text(".360750")],
[#text(".296875–.439394")],
[#text("16")],
[#text("7")],
[#text(".370724")],
[#text(".327206–.425620")],
[#text("32")],
[#text("7")],
[#text(".359843")],
[#text(".354167–.365927")],
[#text("64")],
[#text("7")],
[#text(".364233")],
[#text(".358832–.370784")],
[#text("128")],
[#text("7")],
[#text(".370035")],
[#text(".368251–.371986")])

#text("These are deterministic tiny instances, consistent with a generic 3/8 hypothesis at larger q but not a proof or significance test. At q=16, K=GF(256) with modulus 0x11b and b=1, a=0 gives N=288, B=31 and coverage 144/288. Changing a to polynomial-basis integer 32 gives N=226, B=1 and coverage 1/226. The analogous q=64 controls give 1/2 and 1/4050. Both traces still obey the same subgroup theorem. They demonstrate why curve-family and actual-base-size hypotheses are essential; they do not refute a suitably restricted generic asymptotic claim.")


#text("The degree-four controls have 9/240 and 117/4096 four-sum coverage, both contained in 2E. Their CSV ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "poisson_model")#text(" field is a two-sum diagnostic and must not be compared with these four-sum coverages. Full coefficients and denominators are in ")#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/binary_results_v2.csv")[#text("binary_results_v2.csv")]#text(".")


#figure(image("figures/binary_coverage.svg", width: 100%), caption: [#text("Complete two-sum enumeration on 39 quadratic-extension binary curves. The two degree-four controls are excluded. Descended-b controls show why trace alone does not fix coverage.")])

#heading(level: 3)[#text("Fresh odd-characteristic checks")]

#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/odd_check.rs")[#text("odd_check.rs")]#text(" checks 24 curves over F_(p^n), p=5,7, n=2,3,4,5, with complete n-sumsets and 32 matched controls per curve. Controls preserve sign-orbit count and the same rational two-torsion points. All field, inverse, curve and emitted-sum checks passed.")


#text("One explicit diagnostic is p=5, n=5, modulus X^5+4X+1, curve ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "y^2=x^3+(2+2X)*x+(1+4X)")#text(". Its full group has 3,065 points and its subfield base has six points. The five-sumset contains 128 points; the 32 controls have mean 145.65625. This rules out universal exact random-equivalence. It does not invalidate an original large-field empirical result. The approximately 12.2 control-standard-deviation gap is descriptive, not a calibrated p-value or a predeclared significance test. The raw control counts are retained in ")#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/odd_results_v2.csv")[#text("odd_results_v2.csv")]#text(".")


#text("No extra coverage is needed for extension-field algorithms to improve asymptotics: decomposition solving and linear algebra determine total cost. ")#link("https://eprint.iacr.org/2004/073")[#text("Gaudry's primary paper")]#text(" treats index calculus on abelian varieties with small dimension. Random-like relation yield is compatible with such algorithms.")


#heading(level: 2)[#text("2. Prime-field tower bases")]

#text("The original ")#link("https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf")[#text("Petit–Kosters–Messeng paper")]#text(" builds low-degree membership chains, including multiplicative-group cosets and auxiliary elliptic-curve isogeny fibers. Dickson split and nonsplit torus variants are related extensions; a Kummer/Dickson census does not exhaust the original auxiliary-isogeny construction. The auxiliary curve's group law is not automatically the target curve's law. The paper's total cost includes relation collection, solving and linear algebra; low degrees of individual equations do not settle that cost.")


#text("The supplied abs(z)<=1.84 statement establishes, at most, no detected coverage discrepancy at the original test's resolution. Reproduce it with exact usable signed bases, the same torsion convention, matched target populations, independent prime/curve clusters and raw per-cell variances. An equivalence conclusion needs a declared effect margin and confidence intervals or power analysis. Eighteen non-significant cells alone are insufficient.")


#text("The fresh prime diagnostic has 18 cases: p=257,769,1153, depths 2,3,4, Kummer and split Dickson. Every fiber is directly enumerated; Kummer roots are independently generated from the multiplicative group, and Dickson roots are checked using u*zeta+1/(u*zeta). Each case has 32 matched sign-orbit controls. This checks implementations and small-field coverage, not the supplied 18-cell multi-prime design and not a solver. The first critical-fiber run, D_d(x)=2, is preserved separately from the second run's generic fibers; source v1 and both outputs remain intact.")


#heading(level: 3)[#text("Later repository evidence")]

#table(columns: (1fr, 1fr, 1fr), inset: 4pt, stroke: .3pt + rgb("cccccc"), table.header([#text("Source")], [#text("What its retained evidence says")], [#text("Limit")]), [#link("https://github.com/aburan28/crypto/tree/3536d16ae185feef0ff90a0c53162e16e58f9d6f/research/pkm_dickson_torus_20261015")[#text("Dickson-torus study, 2026-10-15")]],
[#text("127 tower rows across five primes; solving degrees 3..7; reported exhaustive fiber agreement; split and torus degree plateaus match on their common tested cells.")],
[#text("Archived producer and cross-check receipts inspected; expensive solver runs not independently repeated here. Finite plateaus prove neither bounded nor unbounded asymptotic degree.")],
[#link("https://github.com/aburan28/crypto/tree/3536d16ae185feef0ff90a0c53162e16e58f9d6f/research/pkm_tower_ecbench_20261013")[#text("PKM ecbench study, 2026-10-13")]],
[#text("Reports 896 runs, including 672 measured and 224 warmups, all verified; complete root-finding IC costs about 29–141 times its rho-with-negation reference on 20–28-bit prime curves.")],
[#text("A root-finding/table implementation, not an ideal algebraic PKM solver. Historical cost records, not fresh timings.")],
[#link("https://github.com/aburan28/crypto-autoresearcher/blob/8b77d405e297f275a15cba425cdebb09fcc799c3/problems/pkm-isogeny-chain-factor-base-no-fast-decomposition.md")[#text("Auxiliary-isogeny-chain obstruction")]],
[#text("EXP-ECDLP-f49f75 tests Singular chain/dense/random encodings; EXP-ECDLP-502ae2 adds msolve F4. The reported chain arm is slower in the jointly completed toy cells, with timeouts retained.")],
[#text("Different family from the supplied Kummer/Dickson census; m<=3 and small depths. No general solver impossibility or P-256 result.")])

#text("Thus “solving was not measured” can describe the supplied coverage study, but not the repositories' entire PKM program. The remaining useful lead is a solver or representation that beats same-fiber dense and random controls on a fixed, independently checked workload. Another coverage census cannot resolve that question.")


#text("For literature follow-up, ")#link("https://eprint.iacr.org/2017/609")[#text("Amadori–Pintore–Sala 2017/609")]#text(" uses a dynamically constructed base with known scalar labels so that one nontrivial decomposition relation suffices. ")#strong[#text("Correction to the earlier chat response:")]#text(" this is not a generic reusable Gröbner-basis computation. ")#link("https://doi.org/10.1007/978-3-030-00434-7_19")[#text("Kudo et al., CANS 2018")]#text(" examines hybrid/symmetry improvements; only the official author abstract was available here. The primary ")#link("https://nutmic2019.imj-prg.fr/confpapers/SemaevECDLP.pdf")[#text("Yokoyama et al. 2019 conference preprint")]#text(" analyzes naive Semaev systems under statistical assumptions. Its scope must not be expanded into an unconditional lower bound on every tower solver. The later journal version is ")#link("https://doi.org/10.1515/jmc-2019-0029")[#text("JMC 14, 460–485")]#text(".")


#figure(image("figures/coverage_controls.svg", width: 100%), caption: [#text("Structured coverage versus matched-control mean. Fields are deliberately small; each point is a diagnostic, not a significance test or a replay of the original census.")])

#heading(level: 2)[#text("3. Quadratic-extension transfer and genus-2 index calculus")]

#text("The transfer framework has substantial prior art. Scholten's construction and the explicit maps in ")#link("https://doi.org/10.1112/S1461157014000394")[#text("Bernstein–Lange, Hyper-and-elliptic-curve cryptography")]#text(" relate suitable elliptic curves over F_(p^2) to genus-2 Jacobians over F_p. Their explicit isogenies compose to multiplication by 2. This supplies a useful order-preservation criterion on odd-order subgroups for those maps, not validation of the supplied tau implementation on every curve.")


#text("An executable audit needs the actual curve family, nonsingularity and genus checks, map direction and formulas, exceptional inputs, identity, homomorphism tests, kernel degree and subgroup order. A sextic model with two points at infinity must not silently enter formulas requiring a monic quintic and one point at infinity. The local ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "genus2_fp.rs")#text(" module supports the latter model; it is not the missing tau producer. Large-prime divisors and relation coefficients also need valid subgroup projection.")


#heading(level: 3)[#text("A graph cycle is not necessarily a target relation")]

#text("An edge can give a signed equation among two unknown large-prime logs and a known combination A+B*k. Following a cycle gives ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "z = epsilon*z + A + B*k")#text(". With epsilon=+1, a nonzero invertible B can reveal k. With epsilon=-1 over an odd-order coefficient field, it only determines 2z in terms of k; another dependence or an anchor is needed. A zero target coefficient also prevents extraction.")


#text("For example, the triangle equations x+y=a1+b1*k, y+z=a2+b2*k, z+x=a3+b3*k have an invertible 3-by-3 large-prime coefficient matrix in odd characteristic. They admit a solution for every k and therefore do not determine k. ")#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/cycle_check.rs")[#text("cycle_check.rs")]#text(" verifies this exactly modulo 101 for all 101 target values and includes a four-edge positive control with nonzero elimination coefficient recovering k=37.")


#text("This is an implementation audit criterion, not evidence that the unlocated original code is wrong. The issue is already explicit in ")#link("https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf")[#text("Gaudry–Thomé–Thériault's original double-large-prime paper, section 3.1")]#text(". Any claimed first-cycle solver must show its signs, coefficients and stopping rule, and retain fruitless cycles and retries in the totals.")


#figure(image("figures/cycle_dependence.svg", width: 100%), caption: [#text("Exact signed-cycle control: a triangle can constrain only the large-prime variables, while a square can eliminate them. Extraction still requires a nonzero target coefficient.")])

#heading(level: 3)[#text("Counts, costs and ranking")]

#link("https://arxiv.org/abs/math/0606607")[#text("Diem's primary analysis")]#text(" gives the fixed-genus class-group algorithm's expected soft-O(q^(2-2/g)) bound under its stated group/input hypotheses. At g=2 the exponent is 1. For an elliptic DLP with subgroup order r proportional to p^2, rho also has order p. The supplied counts therefore concern constants, not a demonstrated exponent improvement.")


#text("Independent random trials can help test whether a walk's dependence causes an observed stopping count. They cannot remove the birthday phenomenon from a first-cycle algorithm. Generating a fresh uniform known-label divisor may require scalar multiplications; equating one such trial with one incremental Jacobian addition would be invalid.")


#text("The quoted 41M+I and 12M+I need common base-field units. An F_(p^2) multiplication is not an F_p multiplication. Count squarings, extension reduction, batched inversions, coefficient updates, polynomial factoring, factor-base construction, graph/hash operations, memory and map costs. Coordinate-dependent formulas also change the price. The ")#link("https://www.hyperelliptic.org/EFD/g1p/auto-shortw.html")[#text("Explicit-Formulas Database")]#text(" provides examples of this dependence; its formulas are not measurements of these two implementations.")


#text("The correct ratio is the complete charged IC total divided by complete charged rho total on the same curve/subgroup, targets, hardware and memory policy. Separate one-target online time from setup; if setup is amortized over many targets, give rho the same preprocessing allowance. Use the strongest applicable rho walk, with negation, automorphisms and collision-detection overhead accounted for. Replace p by actual sqrt(r) where appropriate; cofactors affect constants materially. For missing native-operation prices, apply the repository's 0, 0.1 and 1 group-add-equivalent sensitivity charges to both arms. A band that changes the winner remains unresolved.")


#text("If the original .51–2.83 sensitivity band is reproduced, its ranking really is unresolved across that band. The statement “rho cheaper in 10 of 12 pricing cells” does not assign an 83% probability. A measured 2.8-fold gap would rank those implementations on that host and workload. It would not establish a universal genus-2 loss.")


#heading(level: 2)[#text("Reproduction and evidence boundaries")]

#text("The ")#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/PROTOCOL.md")[#text("protocol")]#text(" precedes the diagnostic runs and records all three amendments. The native checkers use no production cryptography module, no recovered-secret oracle, no cloud workers and no timing interpretation. Field-element encodings, irreducible polynomials, curve coefficients, group sizes, usable base sizes and null counts are recorded. The exact diagnostics are not production curve identities or registered benchmark instances.")


#text("Run ")#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/verify.sh")[#text("verify.sh")]#text(" from this directory with Rust and the pinned Lean toolchain installed. It rebuilds all three current native checkers, compares their deterministic output with the retained CSVs, then runs the repository's Lean proof gate. No production runtime code changed, so the standalone checks are the relevant native verification; the full crate's unrelated solver tests are not claimed as run. The final assertion replay is retained in ")#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/verification-v2.log")[#text("verification-v2.log")]#text(".")


#text("Preserved failures and revisions: the first Lean elaboration omitted explicitly included hypotheses and failed; ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "lean-check.log")#text(" retains that failure. The corrected proof passes in ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "lean-check-v2.log")#text(" and the gate. The gate's first audit invocation lacked a default toolchain; ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "lean-gate/")#text(" retains that infrastructure failure and ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "lean-gate-v2/")#text(" retains the successful invocation with ELAN_TOOLCHAIN pinned. The cycle checker initially required explicit integer types at compilation, then passed. The first prime run's critical Dickson fibers are preserved with ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "odd_check_v1.rs")#text("; the second uses generic fibers. None is silently substituted for the supplied experiment.")


#text("The report's graphs read retained CSV values only. See ")#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/figures/")[#text("figures")]#text(", the editable ")#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/render.py")[#text("render.py")]#text(", and ")#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/research/three_leads_audit_20261018/report.pdf")[#text("report.pdf")]#text(". The copies in ")#link("https://github.com/aburan28/crypto/blob/codex/three-leads-audit-20261018/benchmarks/20261018-three-leads-audit/")[#text("benchmarks")]#text(" are browsing artifacts. Historical speed ratios remain source-reported; there is no newly admitted timing, performance gain or frontier point.")


#text("Canonical graphs checked for applicability: ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "docs/index-calculus-scoreboard.html")#text(", ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "docs/ic/progress-timeline.json")#text(" and ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "docs/ic/leaderboard.json")#text(". They remain unchanged because these correctness diagnostics supply no qualified IC/rho comparison. The open source-provenance record is indexed under ")#text(font: "DejaVu Sans Mono", size: 8.1pt, "problems/")#text(".")


#text("To finish the numerical audit, recover the original source revision, raw targets and seeds, curve/subgroup identities, exact base and tuple conventions, pricing table, complete counters and executable tau. Then recompute the original 21-curve, 18-cell and 1,260-run summaries, including failures and retries, before admitting their numerical claims. The present audit establishes the scoped mathematical corrections and fresh finite checks; the original numerical replay remains open.")
