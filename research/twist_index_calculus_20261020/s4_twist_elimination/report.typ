#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Exact twist S4 structure and a complete prefix relation collector")

#text("23 October 2026. This continues the requested investigation of twist symmetry in index calculus, including mixed-component bases, unique relations, matrix rank and complete cost. The concrete result here is a relation-finder correctness repair on all seven frozen processed prefixes. Complete cold index-calculus cost and a new general complexity result remain open.")


== #text("Requirements and evidence")

#text(size: 6.8pt)[#table(columns: 2, inset: 4pt, text("Requirement"), text("Status and evidence"), text("Exact summation-polynomial structure"), text("Verified over the integers; 49-term and 18-term symmetric identities, zero coefficient remainders; coefficients and certificate IDs (results/structure.json)"), text("Faithful formal proof"), text("Both universal commutative-ring identities compiled and passed the axiom audit in pinned Lean 4.34.1; source (proof/TwistS4Identity.lean), audit (proof/axiom-audit.log)"), text("Twist-specific descended equations"), text("All 46,200 finite signed peels agree coefficient by coefficient with independent interpolation; frozen profile (fixtures/profile-v1.json)"), text("Complete roots on the retained discriminator"), text("Eight exact F4 root sets match exhaustive Fp^3 enumeration; record (results/eight-systems.json)"), text("Unique signed relations on the original prefixes"), text("All seven candidate sets match an independent group-pair oracle at every query, in two identical runs; summary (results/prefixes.json)"), text("Matrix rank and recovery"), text("Seven full-rank matrices; all target and physical factor logarithms verified in the curve group"), text("Complete cold cost and strongest external solver"), text("Not established here. Current F4 counter covers row reduction only; no priced pipeline, timing, fastest-solver or rho claim"))]

#image("overview.svg", width: 100%)

== #text("Exact problem and frozen inputs")

#text("The three curves are the registered models E: y^2=x^3+b0*t^3 over Fp[t]/(t^4-d), with d the least nonsquare:")


#text(size: 6.8pt)[#table(columns: 6, inset: 4pt, text("p"), text("b0"), text("Prime subgroup order"), text("Physical base"), text("Folded columns"), text("Seeds"), text("37"), text("2"), text("1,873,633"), text("48"), text("4"), text("0"), text("73"), text("5"), text("28,408,033"), text("96"), text("8"), text("0, 1, 2"), text("97"), text("5"), text("88,538,353"), text("72"), text("6"), text("0, 1, 2"))]

#text("The base consists of the original nonzero line-0 and line-2 abscissae, with exactly the original signs, coordinates, endomorphism orbit map and residual order. The exact generator, target and physical coordinates are in the seven original quartic_twist_k1_scale_p*_seed*_v1.json inputs in the parent directory. Their SHA-256 values are pinned in the profile (fixtures/profile-v1.json) and checked before collection. Targets are public synthetic fixtures; the candidate constructs residuals from public G,Q,a,b and verifies its result by group multiplication. It never reads the stored factor logarithms or the public known answer to produce relations or solve the matrix.")


#text("The profile's whole-file hash is 756f5b72404c2c208f477f8cc2eeb20e8b9094a02250a1b5571776bb9d853f8a. The exact derivation, full symmetric reduction, eight-system pilot and prefix integration were frozen separately before their respective runs: 509df89c5e, 3ff04c056, 40fbae5a7, e6fc49a7d.")


== #text("Algebra: what was actually proved")

#text("For y^2=x^3+B, write the third summation polynomial as a quadratic in Z:")


#raw("c0(x,y) = (xy)^2 - 4B(x+y)\nc1(x,y) = -2((x+y)xy + 2B)\nc2(x,y) = (x-y)^2\nS3(x,y,Z) = c2 Z^2 + c1 Z + c0.", block: true)

#text("For a_i=c_i(x1,x2) and b_i=c_i(x3,X), define its quadratic resultant")


#raw("R = (a2*b0-a0*b2)^2 - (a2*b1-a1*b2)*(a1*b0-a0*b1).", block: true)

#text("Identity 1. Substituting e1=x1+x2+x3, e2=x1*x2+x1*x3+x2*x3, e3=x1*x2*x3 in the recorded 49-term polynomial H gives R identically.")


#text("Identity 2. With s1,s2,s3,s4 the four elementary symmetric polynomials of x1,x2,x3,X, the following 18-term expression also equals R:")


#raw(" -64*s4^3 - 512*s3*B^3 + 192*s3^2*B^2 - 24*s3^3*B + s3^4\n +256*s2*s4*B^2 + 32*s2*s3*s4*B - 8*s2*s3^2*s4 + 16*s2^2*s4^2\n -64*s2^3*B^2 - 192*s1*s4^2*B + 256*s1*s2*B^3\n +32*s1*s2*s3*B^2 - 8*s1*s2*s3^2*B + 32*s1*s2^2*s4*B\n -192*s1^2*s4*B^2 + 16*s1^2*s2^2*B^2 - 64*s1^3*B^3.", block: true)

#text("The native derivation expands the resultant into 191 integer monomials. Lexicographic symmetric reduction subtracts products of the elementary polynomials until the remainder is exactly zero, then reconstructs the original coefficients as a second assertion. All arithmetic uses checked i128. These are integer polynomial identities, so substitution into a commutative ring preserves equality. Lean independently unfolds the same resultant and elementary polynomials and proves both equalities with grind; it does not assume the coefficient derivation, the identities, or elliptic-curve correctness as axioms. The formal statement concerns these polynomial identities, not the completeness of an elliptic-curve decomposition algorithm.")


#text("The corresponding checked identity records are IDC1h4ec979d586d661e5 (results/identity.json) and IDC1h5e063e62d1ec50f7 (results/full-identity.json). Each uses 32 sealed points and 32 further domain-separated points; adding 1 to either identity is rejected. These evaluations supplement the exact derivation and kernel proof.")


=== #text("Proof map")

#text(size: 6.8pt)[#table(columns: 3, inset: 4pt, text("Report statement"), text("Lean declaration"), text("Exact hypotheses / audit"), text("Identity 1"), text("TwistS4Identity.symmetrised_resultant"), text("Arbitrary R : Type with Lean.Grind.CommRing R; no characteristic restriction; standard foundations only"), text("Identity 2"), text("TwistS4Identity.fully_symmetrised_resultant"), text("Same; standard foundations only"))]

#text("The audit lists only propext, Classical.choice, Quot.sound: two theorems, zero failing, zero compiler-trusted. No sorry, custom axioms or native_decide is used. Fresh lean4checker verification was not run.")


== #text("Descending the exact formula: a negative structural result")

#text("After a signed line-2 peel, substitute the actual remaining abscissa X and B=b0*t^3 into H, and reduce powers modulo t^4=d. The unknown e1,e2,e3 remain in Fp. An independent interpolator reconstructs the same polynomial from direct S3-resultant values. Every coefficient of all four components agreed on 46,200 systems across all seven prefixes, including the previously failed p73 system.")


#text("Every system had component degree profile [4,4,4,4], including after linear row reduction of the coefficient vectors. Thus this census found no component equation of lower degree from that operation. It does not exclude nonlinear elimination, syzygies, a different invariant base or other twist models. The compact 18-term formula is ordinary full symmetrization; by itself it does not establish a new relation-yield or solving-cost advantage.")


== #text("Complete root pilot and all-prefix relation comparison")

#text("F4 used grevlex, degree bound 12, no staircase stop and one Rayon thread. All eight retained fixtures were also enumerated over the full Fp^3. Their complete rational root sets agreed. In p73/seed1/query18/peel54, sign +1, the only roots are (7,61,52) and (66,61,21); splitting their cubics and checking signs in the original curve group recovers both missing relations. The other seven pilot systems have no rational roots. A corrupted coefficient fails the fixture hash before any solve.")


#text("The collector then used F4 on every finite signed line-2 peel in the seven original prefixes. Only after candidate collection for a run did it construct the independent signed L0-pair oracle. It compared every query's full signed relation set, rebuilt the anchored matrix in sorted relation order, and verified the resulting target and all physical factor logarithms by group multiplication. It continued through the original prefix even if the repaired solver reached full rank earlier.")


#text(size: 6.8pt)[#table(columns: 7, inset: 4pt, text("p/seed"), text("Queries retained"), text("S4 systems"), text("Old / F4 / exact signed relations"), text("Rank"), text("First full-rank query (zero based)"), text("F4 row-reduction Fp multiplications"), text("37/0"), text("8"), text("192"), text("4 / 4 / 4"), text("4/4"), text("7"), text("22,281,500"), text("73/0"), text("15"), text("1,440"), text("8 / 8 / 8"), text("8/8"), text("14"), text("173,193,847"), text("73/1"), text("20"), text("1,920"), text("9 / 11 / 11"), text("8/8"), text("18"), text("230,561,246"), text("73/2"), text("43"), text("4,128"), text("15 / 15 / 15"), text("8/8"), text("42"), text("496,014,079"), text("97/0"), text("310"), text("22,320"), text("16 / 16 / 16"), text("6/6"), text("309"), text("2,704,729,336"), text("97/1"), text("75"), text("5,400"), text("6 / 6 / 6"), text("6/6"), text("74"), text("654,195,561"), text("97/2"), text("150"), text("10,800"), text("6 / 6 / 6"), text("6/6"), text("149"), text("1,309,492,405"))]

#text("All seven runs: zero missing/extra signed relations, zero undetermined systems, full rank, recovered 271, and all physical factor logs verified. The complete first and A/A output trees are byte-identical. For p73/seed1 the first full-rank query moved from 19 to 18. This is a measured repair of a missed-relation failure; the algebraic supply of relations did not increase. The new collector was not applied to the 21 thinned bases or the unused suffixes of the six larger frozen streams in this round.")


== #text("Cost boundary and remaining research")

#text("The table counts only multiplications covered by F4's row-reduction counter. Input normalization, polynomial interreduction, substitution, root finding, inversion work outside that counter, exact coefficient specialization, sign verification, base/model setup, orbit validation, matrix work, allocation and hashing are not all included. The oracle is separate audit work. Stored factor coordinates and public orbit metadata are replay inputs, so this is not a cold pipeline run. Consequently the table cannot be divided by the old complete-ish IC counter or matched rho counter to claim a speedup. A/A here checks deterministic results and counts; it is not an isolated timing comparison or five-round ABAB study.")


#text("The original requested complete cost remains open: implement complete phase counters, run the candidate through cold setup and recovery, calibrate the 0/0.1/1 GAE band and compare the strongest same-target twist24 rho. Compare the identical polynomial systems and outputs with msolve/Magma before any fastest-solver claim. Neither external executable was available on this host in this round.")


#text("A separate hypothesis is to add endomorphism-sum images of the existing factors. For example, (1-psi)(P) has known log coefficient 1-lambda_cubic, so its physical point can be added without a new log column. Existing automorphism images are already saturated by the current orbit fold. New images may change mixed relation occupancy and rank, but generating, deduplicating and solving with them must be charged. No relation-yield or complexity advantage for this hypothesis has been measured here.")


#text("Ordinary curve symmetries and quasi-homogeneous polynomial solving are established prior work: Faugère–Gaudry–Huot–Renault, Journal of Cryptology 2014 (https://researchportal.ip-paris.fr/en/publications/using-symmetries-in-the-index-calculus-for-elliptic-curves-discre/). Torsion-based equivariant maps and compact symmetrized summation polynomials are also prior work: Faugère–Huot–Joux–Renault–Vitse, EUROCRYPT 2014 (https://www.iacr.org/archive/eurocrypt2014/84410158/84410158.pdf). This round makes no novelty claim for invariant coordinates or the 18-term identity.")


== #text("Execution, reproducibility and presentation")

#text("Execution receipt (EXECUTION.md) retains all failures, including the invalid temporary-storage proof run, the audit PATH failure and both source-bundle failures. Run reports (reports/README.md) include each launched job with a diagram or recorded-value graph and an inspected PDF. The final native job was 20261023-025542-twist-s4-prefix-all-48c275, c7i.large Spot, one Rayon thread, 8 GiB address-space cap and 1,200 s watchdog per collector. It passed 48 native tests. The local mandatory library gate passed 4,488 tests with 119 existing ignores; both touched examples passed (1 and 3 tests).")


#text("Extract the immutable raw job trees with tar -xJf runs.tar.xz, and check the archive with sha256sum -c runs.tar.xz.sha256. The summaries, frozen profile, certificate records and formal proof are also kept loose so their cited paths resolve. Each raw tree retains actual executed source, command, environment, host/rusage records and terminal status.")


#text("Native reproduction after extracting inputs into the repository layout:")


#raw("cargo test --locked --release --manifest-path native/Cargo.toml\nRAYON_NUM_THREADS=1 cargo run --locked --release --manifest-path native/Cargo.toml -- \\\n  --prefixes fixtures/profile-v1.json \\\n  756f5b72404c2c208f477f8cc2eeb20e8b9094a02250a1b5571776bb9d853f8a \\\n  .. /absolute/write-once/output", block: true)

#text("Run compute through the repository isolation wrapper or its ephemeral runner; bundle inputs (bundle-inputs.txt), prefix command (run-prefixes.sh) and fail-closed proof command (run-proof.sh) preserve the executed setup.")


#text("Canonical graph check: the existing scaling, thinning and peeled-orbit reuse graphs retain their historical measurements. This new result has its own overview and report links. No leaderboard A–C value or priced benchmark changes, because this round did not measure the required cost interval. PDF (report.pdf).")

