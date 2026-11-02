# GAPS.md — completeness critic, ic_mega_pass_20261031 (2026-10-31)

Sources: report/REPORT.md, brief/BRIEF.md §3, brief/MAP_ideas_corpus.json (GAP: items), ideas/*.json (57 cards), verdicts/*.json + RANKING.json, literature/*.json (8 lanes), experiments/*/{README,AUDIT,RESULTS}. Keyword hits were checked in context, so incidental words ("theta" as an exponent, "tower" meaning PKM tower) do not count as coverage.

## A. Areas with no idea card

### A1. MAP_ideas_corpus.json GAP items with no card (keyword grep plus context check over ideas/*.json)
- Arakelov / arithmetic intersection theory: none
- Shimura varieties / quaternionic uniformisation: none ("Shimura" appears only as a prior-art citation in number_theory-2)
- Sato–Tate / Katz–Sarnak Frobenius statistics: none (wildcards-4 "random matrix" is about the Macaulay matrix, not Frobenius)
- Lang–Trotter / prime-order trace heuristics: none
- Cohen–Lenstra: none (the "class group" hits are ray class groups in number_theory-2 and the Diem citation in transfer_tori-2)
- Mazur torsion / Ribet / Eisenstein ideal: none
- BSD / Tate conjecture / L-values: none
- Complex-analytic uniformisation (periods, ℘, AGM, elliptic logs): none (complexity-5 "Weierstrass" means change of model)
- Garcia–Stichtenoth towers / DVI / asymptotically good AG codes: none
- Hilbert modular / RM / Humbert surfaces: none
- Siegel modular forms / Igusa invariants / theta constants / Rosenhain–Thomae: none
- Ekedahl–Oort / Newton polygon / p-rank / a-number: none
- Hasse invariant / Deuring lifting: none (citation only)
- Deligne–Lusztig characters as x-filters: none (marked THIN in the corpus and still empty)
- Kronecker limit formula / Chowla–Selberg / CM L-functions: none (number_theory-2 uses a Hecke character only as the Galois action)
- Large sieve / Burgess / Pólya–Vinogradov short sums: none. number_theory-4 covers only the Weil/Bombieri bilinear case; the sub-√p short-sum regime that KN-OPEN-91d4f6 needs has no card.
- Brauer–Siegel / Siegel zeros / class number formula: none
- del Pezzo / cubic surfaces / 27 lines: none
- Jacobi forms / theta with characteristics: none
- Sphere packings / Leech / Construction A / Barnes–Wall: none
- Finite geometry (ovoids, hyperovals, spreads as factor-base indexing): none
- Igusa local zeta functions of descended or digit systems: none
- Mahler measure / Mahler functions: none
- Fermat quotients / Wieferich: none
- Transcendence (Schanuel, linear forms in elliptic logarithms): none
- LTC / LDC / PCP / interactive-proof arithmetisation: none
- Mumford representation / Cantor arithmetic in a Prym or genus-2 destination: none (transfer_tori-2 cites Mumford for Pryms only)

### A2. GAP items with partial coverage only
- Honda–Tate / Weil polynomials: transfer_tori-1 (deferred, folklore) and transfer_tori-2 (killed)
- Drinfeld modules: number_theory-5 (deferred; closed by the degree argument)
- Model theory of finite fields: complexity-1 (deferred; CDM dichotomy)
- Explicit CFT / elliptic units: number_theory-2 (killed). Its own nongenericity clause ("unless a sub-|E| description of K_m via elliptic/Siegel units") was not pursued.
- Semi-normality / Betti tables: alg_geom-1 and alg_geom-2 (killed, argument only); comm_alg-2 (deferred)
- Hilbert irreducibility / specialisation: no card. The only hits are the word "specialised".
- Derived / homotopy frameworks: none (the hits are wildcards-3 "homotopy continuation" and incidental uses of "derived")

### A3. BRIEF §3 levers with no surviving or executed idea
- Lever 2 (sub-N² S_4 membership on prime curves; D_m-invariant coordinates on 2-torsion curves): only coding_comb-3, killed. No card attacks root finding directly.
- Lever 4 (PKM-tower plateau D=6,6,7,7; prediction d_F = 4+⌊(t−2)/4⌋): no dedicated card. ml_empirical-1 used the PKM data and was killed on design.
- Lever 5 (Frobenius-equivariant F4 on the barrel): only symmetry-1 arm B, which was cut.
- Lever 6 (non-subspace Frobenius-stable bases): μ_263, the Hamming ball (KN-OPEN-5e3dd0) and the cyclotomic-sparse A2 base each have no executed card. The cards touching it are wildcards-2, transfer_tori-4 and symmetry-3, all killed or deferred.
- Lever 10 (Miller cocycles, same-field Prym, Kani/Richelot second N-torsion): transfer_tori-3 was deferred. alg_geom-5 was killed, but its scope is degree-2 extensions only. Same-field Prym over F_p or prime-degree F_{2^n} has no card.
- Lever 11, function-field-section half: no card. The CM-canonical-lift half has number_theory-1 and number_theory-2, both killed.
- Lever 12 (learned decomposability, cost model, M7): ml_empirical-2 deferred, ml_empirical-4 deferred. Nothing ran.
- Lever 13 (ω ≤ 9/4 over GF(2^k) for hashed tensor ECDL): wildcards-6 measured flattening rank only. The ω question itself is unaddressed.

## B. Literature lanes that are thin, and the re-run queries

Counts from literature/*.json. "inwin" means date ≥ 2026-09-12.

| lane | entries | in window | new | queries | problem | re-run with |
|---|---|---|---|---|---|---|
| ml_cryptanalysis | 29 | 2 | 25 | 34 | Every entry is abstract-level. OpenReview and 2026/1688, 2026/2556 failed. HAWK figures are second-hand press. | arXiv listing scrape cs.CR+cs.LG 2026-09-15..10-31 with "Groebner/F4 learning", "SAT solver learned branching cryptanalysis", "transformer discrete logarithm", "learned solving degree"; retry OpenReview ICLR-2027 submissions via the API |
| adjacent_fields | 33 | 3 | 22 | 49 | It ran before brief/MAP_ideas_corpus.json existed (lane mtime 22:06, corpus 22:26), so none of the 34 GAP items were queried. Semantic Scholar returned 429. | One query per GAP item in §A1 joined with "discrete logarithm" or "elliptic curve point decomposition", e.g. "Deligne-Lusztig character sum elliptic", "Igusa zeta Weil descent", "Garcia-Stichtenoth tower index calculus", "large sieve elliptic curve x-coordinate short interval" |
| lattices_smallroots | 26 | 3 | 15 | 45 | ePrint PDFs are behind Anubis. Numeric bounds in 2026/1027, 244, 423, 2621 are unverified. cuso was not run. | Fetch the ePrint .pdf through the curl path that worked for eprint_recent; queries "Coppersmith symmetric polynomial invariant lattice", "Coppersmith multivariate sublattice non-triangular 2026", "summation polynomial small roots" |
| arxiv_recent | 20 | 20 | 8 | 33 | Smallest lane. Phrase search is incomplete. The cited "public preprints claiming sub-exponential prime-field ECDLP" and [Ope26] were not located. | Full listing scrape of math.NT, math.AG, cs.CR and math.AC for the window, filtered by abstract regex (elliptic & (discrete log | decomposition | summation | descent)); title search for the Ope26 polynomial-factoring preprint |
| program_self_review | 27 | 27 | 2 | 10 | Only 10 queries. The ml-cryptanalysis and cryptanalysis trees were not consulted. Four cited directories are missing locally. | Read /Volumes/SSD990-2/ml-cryptanalysis/csd/{semaev,fpn_degrees}.py results and /Users/adamburan/cryptanalysis; grep the full crypto tree for prime_ic_program_20261029 and prime_fall_degree_20261027 |

## C. Inconclusive experiments: the one change that makes each decisive

- **MP-comm_alg-1** (inconclusive). Run d=4 (not d=3, which is the tautological low_3=3n) at n=19..29 with a compiled M4RI eliminator, arms A vs D1 vs random_matched. Decisive test: whether 1 ∈ W_4 at any n, or the slope of e_4(n). The d=3 grid carries no d_F information.
- **MP-lattices-4** (inconclusive). State the small-root bound on x_i (or Y_i) and compute the reach of the D4-invariant sublattice with that bound in the programme's min-cut code at the matched l. The current model puts smallness on e_k(Y²) and π, which are not small. Without this the result cannot discriminate at all.
- **MP-symmetry-1** (inconclusive). Replace the null with one that breaks G-stability of the generator span without changing the system: per-block GL_l basis change on one block only. The theorem predicts c*_triv > c*_full for that null. Agreement on both arms means no power; disagreement on the null validates the harness.
- **MP-ml_empirical-3** (inconclusive). Replace the integer metric D* ∈ {2,3} with the continuous dimVV / linearization excess and run it at n=23 (the cell that did not run). Basis-overlap-aware permutation: winners sharing basis vectors were counted as independent.
- **MP-comm_alg-4** (inconclusive). Use a null that preserves support locality: the same variable-to-equation incidence as the chain, with random coefficients. Report Δ at N≥20. The current null destroys locality, so local over-determination is confounded with the block law.
- **MP-number_theory-4** (inconclusive, formally invalid at every k). Make the symmetric NULL-1 the primary null and calibrate the planted dose so it is detectable at k=14 before the run. Better still, move to char 2 (Tr(f(x)), F_2-affine buckets), which is the actual KN-OPEN-91d4f6 cell.
- **MP-lattices-3** (reported as refutes; should be treated as inconclusive). The prediction was "A3 > S5 at l=2 or 3", and l=3 was not run. S5 rises from 0.0025 to 0.0048 at l=3. Run l=3 to settle it.

## D. Kills on weak reasons that deserve a second look

- **MP-symmetry-5.** Reason: "for a 2-group G acting ... by variable permutations ... or any action fixing an F_2-point P — the constant 1 is never in Tr_G(V)". The kill's own scope excludes fixed-point-free affine actions, and the 2-torsion translation x→β/x (FGHR) is exactly that case. The symmetry most relevant to lattices-4 and symmetry-4 is therefore untested.
- **MP-wildcards-5.** Reason: "Stationary measure of an r-adding walk on a cyclic group is uniform". That bounds stationary bias only, not transient correlations or mixing time. The report itself records "Spectral-gap test not run". This is a kill by collision with KN-OPEN-010 plus a stationary argument; the gap test is cheap.
- **MP-coding_comb-1.** Report text: "Argument only; msolve not run". Reason: "Bilinearity is false". The ~5n+3l-variable key-equation presentation's d_F was never measured. Presentation sensitivity of d_F is exactly the open lever-1 question, so "same ideal" is not sufficient: d_F depends on the presentation (BRIEF §2).
- **MP-wildcards-3.** Reason: "each of the n=ml descended equations has Newton polytope = product of m scaled l-simplices (... dense within block since the subspace basis mixes all coordinates)". That assumes a dense basis. A sparse or polynomial-basis subspace (the same family that won in ml_empirical-3 arm D) may have a smaller mixed volume. Closed form only, no computation.
- **MP-ml_empirical-1.** Reason: "held-out n=31..45 cells are certified LOWER bounds d_F>=5 ... falsifier cannot separate". This is a design defect, not a kill of the idea. Re-propose with uncensored cells (exact d_F at n ≤ 29 from the comm_alg-1 compiled eliminator).
- **MP-complexity-2.** Reason: "conflicts grow ~3.5-4.3x per extra bit". Its scope is plain CDCL. Cutting planes, PC and Gauss-XOR solvers (cryptominisat5 XOR mode) were not tried.
- **MP-alg_geom-2.** Reason: "Weil restriction does not commute with regularity". That kills only the n-independence inference, not a measured comparison of reg(Sym^m E) with d_F. Argument only.
- **MP-number_theory-3.** Its scope excludes "selection using information beyond P's coordinates". Left open, not killed.

## E. Factual inconsistencies between REPORT.md and the files it cites

1. **Literature total.** REPORT §2 says "250 entries in total". The lane files sum to 230 (33+20+27+35+33+26+29+27). The novelty split it quotes (114+83+33) also sums to 230, so 250 is wrong.
2. **"brief/MAP_ideas_corpus.json is absent."** REPORT §7 repeats this from adjacent_fields without qualification. The file exists (mtime 2026-10-31 22:26; the lane ran at 22:06). The real gap is that the lane never used it (see §B).
3. **MP-lattices-3 outcome.** REPORT §1 and §4.8 list "refutes". The AUDIT says the prediction was "A3 > S5 by >10% at l=2 or 3", tested only at l≤2 ("l=3 untested"; "Not a closure of the S5 joint-lattice idea in general"). The bottom-line count (refutes 1) overstates this.
4. **MP-ml_empirical-6.** An experiment directory exists (rank_counting.*, trace_control.*) with no README, RESULTS or AUDIT, and it has refute and collision verdicts but no idea card. The report notes this but still says "10 executed experiments". ml_empirical-6 was also executed, or at least probed, and is excluded from the killed list (30) and the ranking.
5. **MP-lattices-6.** REPORT §1 lists "The S_4-symmetric orbit lattice at m=4 also gives 1/80" under lever 3 as holding. The AUDIT (quoted in §4.3) says that arm "uses x-degrees, not e-variables, so it does not test the D4 lever". §1 does not carry that qualifier.
6. **Kill-reason sourcing.** REPORT §5 says "Argument only" for coding_comb-1 and §7 lists 9 argument-only kills. wildcards-5 ("Spectral-gap test not run") and wildcards-3 ("Closed form only") are also computation-free on their decisive claim, so the list is incomplete.
7. **Brief statement.** BRIEF §3 lever 10 cites "five filed follow-ups 2026-10-27" for Prym. program_self_review found none, and they contradict the atlas. REPORT §7 notes this, but §3 and §6 give no Prym next step, leaving lever 10 orphaned.

## F. Priority of re-runs (cheapest decisive first)
1. MP-lattices-3 at l=3: minutes, settles C/E3.
2. MP-symmetry-5 restricted to the affine 2-torsion translation action: a Reynolds/transfer test is not available (|G|=2 is even), so the test is a direct certificate census. Under 1 hour.
3. MP-wildcards-5 spectral-gap test: under 1 hour.
4. adjacent_fields re-run over the GAP list: one lane agent.
5. MP-lattices-4 corrected smallness model: minutes to an hour.
6. MP-coding_comb-1 msolve d_F on the key-equation presentation at n ≤ 17: under 1 hour.
