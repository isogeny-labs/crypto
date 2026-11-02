# ECDLP index-calculus mega pass, 2026-10-31: report

Programme owner's copy. All paths are relative to `research/ic_mega_pass_20261031/` unless absolute. Every number below is copied from a file in this directory, and its path is cited. Class vocabulary follows BRIEF §0 rule 2: *advance*, *engineering*, *relabelling*, *accounting*, plus *theory*, which the idea cards use. This report makes no security claims. All evidence is toy scale (n ≤ 45 binary, p ≤ 2^256 lattice validation, p ≤ 2^22 spectral).

## 1. Bottom line

- **No advance.** None of the 57 idea cards (`ideas/*.json`) and none of the 10 executed experiments changes an exponent or moves any construction below the CGK line B·D² ≪ N.
- **Audited experiment outcomes** (corrected outcome from each `experiments/*/AUDIT.md`; the original claim in brackets):
  - supports (3): MP-lattices-6 [supports], MP-lattices-2 [supports], MP-symmetry-4 [supports].
  - refutes (1): MP-lattices-3 [refutes].
  - inconclusive (6): MP-comm_alg-1 [supports], MP-lattices-4 [supports], MP-symmetry-1 [supports], MP-ml_empirical-3 [refutes], MP-comm_alg-4 [refutes], MP-number_theory-4 [inconclusive].
  - All 10 audits returned `partially_valid`. Five READMEs overclaimed their outcome.
- **What is new and holds within its scope:**
  1. *Lever 3, accounting.*
     - The (p,f)^l shift-lattice determinant exponent over downset shapes reproduces 1/6, 1/24 and 1/80 for boxes at m=2,3,4.
     - The S_4-symmetric orbit lattice at m=4 also gives 1/80 (0.0124997), about 13.5× short of the 0.169 needed (`experiments/MP-lattices-6/RESULTS_closedform.json`).
     - Row-deletion and threshold sublattices of the JM box with all columns kept never beat the full lattice for S3 (max(B'−A)=0.000, l≤5; `experiments/MP-lattices-2`).
     - Joint-tree lattices with full-size internal nodes give δ=0 at l≤2 (`experiments/MP-lattices-3`).
     - The D4-symmetrised S5 lattice is still *not* validly tested. The MP-lattices-4 model puts small-root bounds on non-small invariants (AUDIT issue 3).
  2. *Lever 1, accounting/relabelling.*
     - The d=3 falls of the descended S_3 system (low_3 = 3n) are reproduced by any F_2-rank-n surrogate tensor, not only by Galois-stable ones.
     - They come from l²=l and match the treatise's Lemma deg3. Nothing was measured on d_F growth (`experiments/MP-comm_alg-1/AUDIT.md`).
  3. *Lever 7, theory/relabelling.*
     - The all-lift subspace census at n=11, l=4 shows a deficit of about 0.92× against the hypergeometric null.
     - The audit's fresh shuffles show the deficit is specific to the true Z. The README's attribution to the pair-preserving shuffle was refuted (`experiments/MP-symmetry-4/AUDIT.md`).
     - There is no excess, and no growth from n=11 to n=13.
- **Killed this pass:** 30 ideas. Section 5 gives each one's obstruction so they are not re-proposed.
- **Deferred:** 17, including merges (section 3.11).
- **Next (section 6):**
  1. Compiled GF(2) eliminator for the d=4 Macaulay profile at n=19..29 (lever 1).
  2. A correctly modelled D4 lattice in which smallness is stated on x_i, cross-checked against the programme's min-cut code (lever 3).
  3. A capacity/LP converse for the lattice ceiling.
  4. A properly calibrated char-2 version of the x-filter spectral test (KN-OPEN-91d4f6).

## 2. Literature 2026-09-15 .. 2026-10-31

The sources are 8 lane files, `literature/{eprint_recent,arxiv_recent,groebner_complexity,lattices_smallroots,adjacent_fields,ffdlp_transfer,ml_cryptanalysis,program_self_review}.json`, with 250 entries in total. Each entry's novelty label is one of: `new_to_programme` 114, `already_in_atlas` 83, `unclear` 33. Only `new_to_programme` entries are listed below, deduplicated by id. Relevance is the first clause of the lane agent's `relevance` field, and every summary is abstract-level unless the lane file says otherwise.

Lane headline findings (quoted from the files):
- No ePrint paper in the window mentions summation polynomials, Semaev, Weil descent, GHS, cover attacks, first/last fall degree or Koblitz index calculus (`literature/eprint_recent.json`).
- No arXiv paper in the window proposes a new index-calculus or point-decomposition algorithm for ECDLP (`literature/arxiv_recent.json`).
- The Drinfeld-module DLP is broken by F_q-linearity of the scalar action (Scanlon 2001), not by index calculus. Brief item 14 is mis-worded (`literature/ffdlp_transfer.json`).
- Three items are new templates for the programme:
  - Corrigan-Gibbs–Henzinger–Wu structured GGM (EUROCRYPT 2026) and Hhan 2024, as formal versions of rule 5.
  - Kopparty–Kothary–Shankar (2026-10-24) polynomial-approximation bounds, a template wall for lever-8 x-filters.
  - de Boer–Pellet-Mary–Wesolowski, as a rigour template (`literature/ffdlp_transfer.json`).
- The self-review lane flags three brief statements as stale (`literature/program_self_review.json`):
  - The D4 *Macaulay* solve ran on 2026-10-13/15. Only the D4-encoded *lattice* is unbuilt.
  - Cells (4,4) and (4,6) at S=−9 ran.
  - The A2 yield clause ran and was yield-neutral.

### 2a. In window (latest date in entry >= 2026-09-12; 09-12..09-14 borderline; latest revision date used for re-revised ePrints) -- 35 papers

| id | title | date | relevance (one line) | source |
|---|---|---|---|---|
| https://www.researchsquare.com/article/rs-11336985/v1 | Grokking the Discrete Logarithm: Scaling Across Model Capacity and Mechanistic Analysis | 2026-10-31 | Within the 'past few weeks' window. Still p=113 memorisation; | `literature/ml_cryptanalysis.json` |
| /Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/brief/BRIEF.md ... | Drinfeld-module DLP analogy (BRIEF sec.3 item 14): the structural reason index calculus breaks the Drinfeld ... | 2026-10-31 | Names the obstruction precisely (BRIEF rule 3): an explicit statement of which property (a norm/degree function compatible with the module action and with a smooth-element census) E(F_q) lacks, and a check that tau-adic ... | `literature/program_self_review.json` |
| https://eprint.iacr.org/2026/2062 | Hamming Ideals and Grobner Bases for ISD-like Syndrome Decoding | 2026-10-28 | Lever 6 of the brief: non-subspace Frobenius-stable bases at prime n, where the Hamming-ball base has 'a bounded-degree Hamming-ideal membership (KN-OPEN-5e3dd0)'. | `literature/eprint_recent.json` |
| https://eprint.iacr.org/2026/2667 | Faster complete formulas and their application to curves with efficient endomorphisms | 2026-10-28 | Step-cost engineering for rho on j=0 prime curves (LEAD-RHO-cheaper-step-primitives open_unexamined; | `literature/eprint_recent.json` |
| https://arxiv.org/abs/2610.34946 | Sharp effective Nullstellensatz for projective Cohen–Macaulay schemes | 2026-10-26 | Lever 1 proof route (degree-c pseudo-solutions / closure certificates): Nullstellensatz degree bounds are upper bounds on the degree at which 1 (or a radical power) is reached, i.e. | `literature/arxiv_recent.json` |
| https://eprint.iacr.org/2026/2621 | Bounds on polynomial approximations of the discrete logarithm function | 2026-10-26 | Directly about the programme's 'no low-degree membership / only algebraic handle over F_p is x as an integer' wall (brief section 2) and the x-functional-filter lever (KN-OPEN-91d4f6): it quantifies how badly any ... | `literature/eprint_recent.json` |
| https://eprint.iacr.org/2026/2641 | A Memory-efficient Implementation of Wiedemann XL | 2026-10-26 | Enabler for the decisive unmeasured cells (m,l)=(4,4),(4,5),(4,6) and larger n in the descended S_5 grid, where memory of the Macaulay matrix is the binding constraint; | `literature/eprint_recent.json` |
| https://eprint.iacr.org/2026/2641 ; https://eprint.iacr.org/2026/1382 | XL engineering 2026: A Memory-efficient Implementation of Wiedemann XL (ePrint 2026/2641); Concrete ... | 2026-10-26 | Lever 1: none. Lever 5: the on-the-fly Macaulay row generation is the memory model an equivariant XL/F4 over F_{2^130} would need (character blocks are too large to store densely); | `literature/groebner_complexity.json` |
| https://arxiv.org/abs/2609.22018 ; https://arxiv.org/abs/2610.21488 | The supersingular isogeny problem in time and memory p^{1/3+o(1)}, unconditionally; Smoothness of shortest ... | 2026-10-25 | Newest structural results in the lane (within 2026-09-15..10-31). | `literature/ffdlp_transfer.json` |
| https://arxiv.org/abs/2610.33452 | Bounds on polynomial approximations of the discrete logarithm function | 2026-10-24 | Direct negative-result template for lever 8 (x-functional filters, KN-OPEN-91d4f6) and for coordinate-statistic hunts: any algebraic functional of low degree correlated with the logarithm is bounded by a Johnson-type ... | `literature/arxiv_recent.json` |
| https://eprint.iacr.org/2026/2621 ; https://arxiv.org/abs/2610.33452 | Bounds on polynomial approximations of the discrete logarithm function | 2026-10-24 | Cross-lane obstruction (brief §3 levers 8 and 12): any 'x-functional filter' or learned predictor that is a low-degree polynomial in the field representation agrees with the logarithm on O(sqrt(Dp)) inputs, i.e. | `literature/lattices_smallroots.json` |
| https://arxiv.org/abs/2610.32508 | Deterministic Root-Finding over Finite Fields with Elliptic Curves | 2026-10-23 | Lever 2 (root finding beyond two summands) is about root-finding structure for S_4 membership; | `literature/arxiv_recent.json` |
| https://eprint.iacr.org/2026/2560 | Graphs and cycles in curve-based collision walks | 2026-10-22 | Rho/multi-target bookkeeping; ECDLP is only the comparison baseline. | `literature/ffdlp_transfer.json` |
| https://arxiv.org/abs/2610.16350 | Generalizing Green's Syzygy Theorems to Symmetric Products of Curves | 2026-10-20 | Semaev's S_{m+1} is the equation of the locus in Sym^m(E) x E (or the image of Sym^m(E) under the x-coordinate map) cut out by the addition map; | `literature/arxiv_recent.json` |
| https://eprint.iacr.org/2026/2541 | Provable Lattice Reduction in Nearly the Time of QR Factorization | 2026-10-20 | Engineering for any lattice experiment in the programme (Semaev small-root lattices, EC-HNP). | `literature/eprint_recent.json` |
| https://eprint.iacr.org/2026/2556 | Analysis of an AI based code distinguisher | 2026-10-19 | In the past-few-weeks window; the one 2026 paper auditing what algebraic structure a transformer distinguisher actually exploits in a public-key setting. | `literature/ml_cryptanalysis.json` |
| https://eprint.iacr.org/2026/2109 | Attacking CRT-RSA with Small Exponents via Unravelled Linearization | 2026-10-18 | Lattice lever 3, engineering side: unravelled linearization halves lattice dimension without changing the asymptotic bound. | `literature/eprint_recent.json` |
| https://arxiv.org/abs/2610.19069 | Elliptic curves attached to Jacobians with imaginary multiplication | 2026-10-13 | Lever 10 (same-field genus-g embeddings): this is an embedding of H^1(E) into the middle cohomology H^g(A), not into H^1 — so E is not an isogeny factor of A and the relation is a Galois-representation / motivic ... | `literature/arxiv_recent.json` |
| https://eprint.iacr.org/2026/2181 ; https://eprint.iacr.org/2026/2444 | Attacking UOV-based Signatures with Schur--Macaulay Matrices (and: Extending the Schur--Macaulay Framework ... | 2026-10-11 | Lever 1: a new characteristic-2-specific source of extra low-degree equations (divided powers, i.e. | `literature/groebner_complexity.json` |
| arXiv:2610.14496 | When Do Restricted Sumsets Cover a Finite Abelian Group? | 2026-10-10 | This is literally the COVERAGE question the monodromy closure leaves free (brief s.2: 'coverage (which targets decompose at all)'): with factor base F of size B in prime-order E(F_q), every target decomposes as a sum of ... | `literature/adjacent_fields.json` |
| https://eprint.iacr.org/2026/408 | Smoothing the degree of regularity for polynomial systems | 2026-10-07 | Directly a cost-model refinement for the decisive solving-degree cells (m,l) of the Weil-descended Semaev system: if the system is solvable from a sub-matrix between degrees d_F and d_F + 1, the cost table in ... | `literature/adjacent_fields.json` |
| https://arxiv.org/abs/2609.27023 | A Szemerédi–Trotter Theorem in Arbitrary Fields | 2026-10-07 | Optimal point-line incidence bounds in F_p^2 are the quantitative tool behind sum-product/additive-energy statements that the programme's x-functional filter question (KN-OPEN-91d4f6) and coordinate-statistic hunts ... | `literature/arxiv_recent.json` |
| /Users/adamburan/ic-work/crypto/docs/ic/RESEARCH_IC_NOVEL_DIRECTIONS_20261007.md ... | Group-algebra lift N1 (exact S3 identity in F_2[Z/(2n+1)]) - never run anywhere; no record in crypto ... | 2026-10-07 | A cheap decisive negative/positive: zero exact lifts closes the only 'non-generic approach' filed in the novel-directions note and removes it from every future list; | `literature/program_self_review.json` |
| https://eprint.iacr.org/2026/1688 | Hell's Bells: A Neural Network Pipeline for Ternary Fast Matrix Multiplication Algorithms | 2026-10-06 | Only paper found touching lever 13 (hashed-factor-base ECDL with tensor contraction, conditional on an omega <= 9/4 question over GF(2^k)). | `literature/ml_cryptanalysis.json` |
| https://eprint.iacr.org/2026/2364 | Comparative Evaluation of Open-Source Gröbner Basis Implementations for Small-Scale AES Polynomial Systems ... | 2026-10-05 | Lever 1/5: engineering baseline confirming msolve as the best open F4 over GF(2) with field equations in 2026, i.e. | `literature/groebner_complexity.json` |
| https://eprint.iacr.org/2026/197 | Derivative-Based Evaluation of Multivariate Polynomials over Product Hamming Balls | 2026-09-29 | An enumeration oracle for a Hamming-ball factor base: evaluating a descended summation polynomial on every base element costs O(d) per point. | `literature/eprint_recent.json` |
| https://eprint.iacr.org/2026/040 ; https://eprint.iacr.org/2026/197 | Polynomial Evaluation on Many Inputs with Bounded Hamming Weight over GF(2) (ePrint 2026/040) and ... | 2026-09-29 | Lever 1: none. Lever 6/8 (flagged, not this lane): fast evaluation over Hamming balls is exactly the primitive a Hamming-ball factor base oracle (enumerate low-weight x and test membership) would use; | `literature/groebner_complexity.json` |
| https://www.mqchallenge.org/ ; ... | Fukuoka MQ Challenge: 2025-2026 hall-of-fame entries (Type II n=38 Verbel 2026-09-28 XL+Block-Wiedemann; Type ... | 2026-09-28 | Lever 1: Boolean (Type I) records have stalled at n=76 since 2023, i.e. | `literature/groebner_complexity.json` |
| https://eprint.iacr.org/2026/2181 | Attacking UOV-based Signatures with Schur--Macaulay Matrices | 2026-09-26 | A char-2 mechanism for producing low-degree equations that are 'pseudo-solutions' (vanish on the variety, not in the ideal at that degree) is exactly the shape of lever 1's suggested proof/computation route (degree-c ... | `literature/eprint_recent.json` |
| https://eprint.iacr.org/2026/1969 | Efficient Polynomial System Solving via Dixon Resultants: Applications to AO Primitives | 2026-09-21 | Summation-polynomial decomposition instances (S_{m+1} restricted to a subspace, or prime-field S_4/S_5 in few variables of degree 2^{m-1}) are the low-variable/high-degree regime where the paper reports Dixon resultants ... | `literature/eprint_recent.json` |
| https://eprint.iacr.org/2026/1027 | Computing Asymptotic Bounds for the Automated Coppersmith Method via Linear Programming | 2026-09-18 | Lever 3: a small-root method for S_5 at delta ~ 1/5 against the Jochemsz-May reach 1/80. | `literature/eprint_recent.json` |
| arXiv:2607.24251 | Decision trees, Frobenius traces, and Weierstrass coefficients of elliptic curves | 2026-09-18 | Best 2026 example of the exact-rule-mining pattern the sibling repo uses (csd/mine.py: constant/equality/offset rules, zero-violation check). | `literature/ml_cryptanalysis.json` |
| arXiv:2609.14433 | Probably correct row echelon form in the F4 algorithm | 2026-09-13 | Engineering: a certified probabilistic REF is relevant to the Frobenius-equivariant F4 on the full barrel system (lever 5) where the dense character blocks dominate; | `literature/adjacent_fields.json` |
| https://arxiv.org/abs/2609.14052 | Symmetric Models for Syndrome Decoding | 2026-09-12 | Lever 1 methodology: a proven upper bound on the solving degree of a structured Boolean system obtained by analysing S^top (homogeneous top parts) — a template for proving solving-degree bounds for Weil-descended ... | `literature/arxiv_recent.json` |
| https://eprint.iacr.org/2025/2044 | New Asymptotic Results on Predicting Polynomial Congruential Generators | 2026-09-12 | First automated machinery where the number of modular equations is a free parameter. | `literature/lattices_smallroots.json` |

### 2b. New to the programme but dated before the window (71 papers; background surfaced by the sweep)

| id | title | date | relevance (one line) | source |
|---|---|---|---|---|
| arXiv:2609.02764 | Finding a Shortest Vector and More in 2^(n/2+o(n)) Time using q-ary Coset Difference Tree | 2026-09-02 | A fresh example of Wagner-style merging where the filter is a coset structure (L/qL) rather than low bits, and the aggregated statistic (a Gaussian gradient) survives summation - i.e. | `literature/adjacent_fields.json` |
| arXiv:2609.01484 | A General Construction of Codes from Drinfeld Modules | 2026-09-01 | Illustrates again that in the Drinfeld world decoding is linear-algebraic (skew CRT, Welch-Berlekamp) because morphisms are F_q-linear. | `literature/adjacent_fields.json` |
| https://eprint.iacr.org/2026/001 | The Cokernel Pairing | 2026-09-01 | A new (2026) pairing object, but its non-degeneracy needs full rational m-torsion; | `literature/ffdlp_transfer.json` |
| https://arxiv.org/abs/2609.26127 ; https://arxiv.org/abs/2609.27886 | Resultant of an equivariant polynomial system with respect to a direct product of r symmetric groups / ... ... | 2026-08-31 | Lever 3 and the symmetry inventory: S_{m+1} is S_m-equivariant and on 2-torsion curves has the larger D_4 / G(2,n)-type symmetry (the never-run D4-symmetrised S_5 lattice; | `literature/arxiv_recent.json` |
| https://arxiv.org/abs/2608.29977 | A New Algebraic Algorithm for LWE | 2026-08-30 | Lever 1: methodological interest only (a direct, assumption-free complexity analysis of a structured system of the Arora-Ge type); | `literature/groebner_complexity.json` |
| https://arxiv.org/abs/2602.02972 ; https://arxiv.org/abs/2512.14722 | Learning Fast Monomial Orders for Gröbner Basis Computations; HATSolver: Learning Gröbner Bases with ... | 2026-08-28 | Lever 1: none directly; the programme's own result that d_F depends on the presentation (monomial order / coordinates) is exactly what a learned order selector would exploit, so a learned-order experiment on descended ... | `literature/groebner_complexity.json` |
| arXiv:2602.02972 | Learning Fast Monomial Orders for Gröbner Basis Computations | 2026-08-28 | Closest existing template for a learned presentation choice on lever 1: the Weil-descended S_{m+1} restricted to an l-dim subspace has a free choice of variable order / F_2-basis of V and of monomial order; | `literature/ml_cryptanalysis.json` |
| arXiv:2606.17399 | The Discrete-Log Clock: How a Transformer Learns Modular Multiplication | 2026-08-27 | Mechanistically the network *uses* a discrete-log table it memorised for p=113; | `literature/ml_cryptanalysis.json` |
| arXiv:2608.21986 | AI Grinding for Fun and Cryptanalysis | 2026-08-22 | The 'normal basis splits degree-63 into 7x degree-9' item is the one number-theoretic flavour: a presentation/subfield-structure exploit, which is exactly the door the programme has closed at prime n=131 (no proper ... | `literature/ml_cryptanalysis.json` |
| https://eprint.iacr.org/2026/1704 | A decrementally-improved algorithm for Boolean MQ | 2026-08-16 | Lever 1: none (generic Boolean MQ; exponent class unchanged, ~0.69n). | `literature/groebner_complexity.json` |
| https://eprint.iacr.org/2026/1658 | Upper bounds for failure probabilities of reductions from low density subset sum problems to lattice problems ... | 2026-08-11 | Same inapplicability as the previous entry (no Z-embedding of E(F_p)). | `literature/lattices_smallroots.json` |
| arXiv:2505.23696 | Computational Algebra with Attention: Transformer Oracles for Border Basis Algorithms | 2026-08-11 | The 'oracle that skips reductions with correctness preserved' pattern is the honest shape for a learned F4 component: it cannot change the solving degree, only skip zero rows. | `literature/ml_cryptanalysis.json` |
| https://arxiv.org/abs/2309.03855 | The complexity of solving a system of equations of the same degree | 2026-08-10 | Lever 1: refines the Macaulay bound for same-degree systems and, with field equations over F_2 (q=2), gives B = 2-t+(n-k), still linear in the number of variables; | `literature/groebner_complexity.json` |
| arXiv:2606.07321 | Letting Homogeneity Entropy Select S-Pairs in Buchberger's Algorithm | 2026-08-05 | Shows hand-designed non-degree pair selection is live again; | `literature/ml_cryptanalysis.json` |
| https://hawk-sign.info/ (withdrawal, verified) ; ... | Anthropic: key-recovery attack on HAWK-256 found by Claude Mythos Preview (HAWK withdrawn from NIST ... | 2026-07-29 | The strongest 2025-26 'AI mathematician on lattices' datum and the only one with a confirmed real-world consequence. | `literature/ml_cryptanalysis.json` |
| arXiv:2607.18538 | CryptanalysisBench: Can LLMs do Cryptanalysis? | 2026-07-29 | Calibrates what agent swarms (like this one) can and cannot do: wins are on design flaws of symmetric/KEM candidates, none on ECDLP/number-theoretic hardness. | `literature/ml_cryptanalysis.json` |
| https://arxiv.org/abs/2607.09163 | Beyond F5 and GVW: The Proper-Cover Algorithm for Fast Ideal Basis Computation | 2026-07-10 | Lever 1: signature-based solvers expose the syzygy cascade (LEAD-ICE-syzygy-cascade-signatures); | `literature/groebner_complexity.json` |
| arXiv:2606.18895 | Symmetric and Isotypic Hilbert Series for Symmetric Ideals | 2026-06-17 | An isotypic Hilbert series is a per-character count of the Macaulay matrix's rows/columns; | `literature/adjacent_fields.json` |
| arXiv:2606.15301 | Discovering Lattice Reduction Strategies via Self-Play (DeltaStar) | 2026-06-13 | Only lever-3-adjacent ML result found (small-root method for S_5 at delta~1/5 needs a better lattice step, not a faster LLL). | `literature/ml_cryptanalysis.json` |
| arXiv:2606.15036 | Transformers Learn the Mestre-Nagao Heuristic | 2026-06-13 | Shows interpretability can read a known analytic-number-theory functional off a trained model. | `literature/ml_cryptanalysis.json` |
| https://eprint.iacr.org/2025/1351 | On the Regularity of the Generalized Birthday Problem | 2026-05-30 | The relation-collection stage of IC is a many-solution k-SUM over E(F_q) with k = m+1 small and lists of size B; | `literature/adjacent_fields.json` |
| arXiv:2510.03828 | Additive Rigidity for x-Coordinates of Rational Points on Elliptic Curves | 2026-05-20 | Characteristic zero; nothing mod p. For the programme it is the clean statement of the heights wall from the additive side: the x-coordinates of a lift to E(Q) are additively unstructured (minimal additive energy), so ... | `literature/adjacent_fields.json` |
| https://arxiv.org/abs/2605.15002 | Extending CDCL to disjunctions of parity equations (Xorcle) | 2026-05-14 | Lever 1: SAT solvers are enumerators on PDP (treatise prog_binary.tex: 'SAT, SMT and WDSat are enumerators'); | `literature/groebner_complexity.json` |
| https://arxiv.org/abs/2505.10246 | An Algorithm for Computing the Leading Monomials of a Minimal Gröbner Basis of Generic Sequences | 2026-05-11 | Lever 1: a 'generic leading-monomial skeleton' is exactly the null against which structured descended systems deviate; | `literature/groebner_complexity.json` |
| https://arxiv.org/abs/2601.05026 | A data structure for monomial ideals with applications to signature Gröbner bases | 2026-05-11 | Lever 5: engineering for signature-based F5-type runs; | `literature/groebner_complexity.json` |
| https://arxiv.org/abs/2604.04656 | Subset Balancing and Generalized Subset Sum via Lattices | 2026-04-24 | Lagarias-Odlyzko lane item. Not applicable to E(F_p) relation collection: these reductions need a Z-linear embedding of the target group (integers, or F_q[t]); | `literature/lattices_smallroots.json` |
| arXiv:2509.14754 | Variables Ordering Optimization in Boolean Characteristic Set Method Using Simulated Annealing and Machine ... | 2026-04-18 | The only 2025-26 paper that does learned variable ordering for Boolean systems of cryptographic origin, which is exactly the shape of the descended S_{m+1} over F_2 (lever 1 cells). | `literature/ml_cryptanalysis.json` |
| arXiv:2604.03903 | Improving ML Attacks on LWE with Data Repetition and Stepwise Regression | 2026-04-05 | Defines the state of the art of 'transformer attack on a hard problem'. | `literature/ml_cryptanalysis.json` |
| https://arxiv.org/abs/2603.16378 | A complexity analysis of the F4 Gröbner basis algorithm with tracer data | 2026-03-17 | Lever 1: it is a generic-case result; the descended systems are far from generic (d_ff = 2), so the formula does not transfer, but it is the right null for 'how much cheaper than generic is the structured system'. | `literature/groebner_complexity.json` |
| arXiv:2603.09680 | Murmurations: a case study in AI-assisted mathematics | 2026-03-10 | The canonical success of 'ML finds a number-theoretic law' is an *averaged* statistic over families, invisible on a single curve. | `literature/ml_cryptanalysis.json` |
| arXiv:2603.07176 | Learning to Rank the Initial Branching Order of SAT Solvers | 2026-03-07 | Negative evidence for the simplest learned-branching idea on structured hard instances; | `literature/ml_cryptanalysis.json` |
| arXiv:2603.06483 | Uniform sum-product phenomenon for algebraic groups and Bremner's conjecture | 2026-03-06 | Over C only; nothing mod p. The relevant statement for index calculus is the shape: 'x-coordinates of points of E in a set with small additive doubling cannot be numerous' - i.e. | `literature/adjacent_fields.json` |
| https://eprint.iacr.org/2026/384 | The Structured Generic-Group Model | 2026-02-24 | Formalises the brief's rule 5 from a different direction: the programme's certificate is B.D^2 << N (CGK preprocessing); | `literature/ffdlp_transfer.json` |
| https://eprint.iacr.org/2026/360 | Improved preprocessing for the Crossbred algorithm and application to the MQ problem | 2026-02-22 | Lever 1: none. Confirms no WDSat/Weil-descent follow-up from the WDSat group in 2026 (their current line is Crossbred/MQOM). | `literature/groebner_complexity.json` |
| https://arxiv.org/abs/2512.01588 | Rigorous methods for computational number theory | 2026-02-19 | Structural ingredient: rerandomisation within a class that lands in a dense, recognisable, factorable family. | `literature/ffdlp_transfer.json` |
| https://eprint.iacr.org/2026/244 | Revisit Unravelled Linearization with Ehrhart (quasi-)Polynomial | 2026-02-13 | Directly the W10 §7 class 1 (non-full-rank / unravelled lattices) for which the programme has no ceiling and no tool. | `literature/lattices_smallroots.json` |
| arXiv:2602.10402 | Critical Numbers for Restricted Sumsets: Rigidity and Collapse in Finite Abelian Groups | 2026-02-11 | Companion to 2610.14496. MDS-ness of the elliptic code on evaluation set P is equivalent (via Cheng's Lemma 3 below) to 'no k distinct points of P sum to the fixed divisor point'; | `literature/adjacent_fields.json` |
| arXiv:2602.03803 | Computing submodules of points of general Drinfeld modules over finite fields | 2026-02-03 | Shows the whole 'point group' of a Drinfeld module is computed as a module via normal forms of an F_q-linear operator - the structural statement behind the easy DLP. | `literature/adjacent_fields.json` |
| arXiv:2601.02162 | A computational approach to Drinfeld modules | 2026-01-05 | Entry point for the function-field side of lever 14; | `literature/adjacent_fields.json` |
| arXiv:2512.12483 | Mage: Cracking Elliptic Curve Cryptography with Cross-Axis Transformers | 2026-01-01 | The only paper found that literally tries 'transformer vs ECDLP'; | `literature/ml_cryptanalysis.json` |
| https://arxiv.org/abs/2512.19076 ; https://eprint.iacr.org/2025/1004 | On Factoring and Power Divisor Problems via Rank-3 Lattices and the Second Vector | 2025-12-22 | A lattice + generic-collision hybrid in which the lattice supplies a quadratic relation and BSGS finishes; | `literature/lattices_smallroots.json` |
| arXiv:2510.08797 | TAPAS: Datasets for Learning the Learning with Errors Problem | 2025-12-19 | Model for what a public 'ECDLP index-calculus dataset' would look like (descended systems + certified degrees + priced costs); | `literature/ml_cryptanalysis.json` |
| arXiv:2512.14722 | HATSolver: Learning Groebner Bases with Hierarchical Attention Transformers | 2025-12-09 | Lauter/Perret entering Gröbner learning signals an intended cryptanalytic application (MQ/HFE-type). | `literature/ml_cryptanalysis.json` |
| https://neurips.cc/virtual/2025/poster/121790 | Bridging Crypto with ML-Based Solvers: The SAT Formulation and Benchmarks (SAT4CryptoBench, BASIN) | 2025-12 | ANF-native learning is the right representation for Weil-descended Semaev systems (they are ANF). | `literature/ml_cryptanalysis.json` |
| https://arxiv.org/abs/2511.10823 | Beating Meet-in-the-Middle for Subset Balancing Problems | 2025-11-13 | Representation-technique lane item. Still requires modular filtering of partial sums (a quotient of the ambient group), which E(F_p) of prime order lacks; | `literature/lattices_smallroots.json` |
| https://eprint.iacr.org/2025/1002 | Cool + Cruel = Dual, and New Benchmarks for Sparse LWE | 2025-11-04 | The equal-advice audit of the LWE world: an ML-adjacent 'win' reclassified as a known algorithm with a bad baseline. | `literature/ml_cryptanalysis.json` |
| arXiv:2510.02162 | NoMod: A Non-modular Attack on Module Learning With Errors | 2025-10-02 | Again linear-after-preprocessing sparse-secret regime. | `literature/ml_cryptanalysis.json` |
| arXiv:2509.21773 | Dual and Covering Radii of Extended Algebraic Geometry Codes | 2025-09-26 | Minor: confirms the two-valued covering radius dichotomy (n-k vs n-k+1) that underlies Cheng's Lemma 3 for long elliptic codes, i.e. | `literature/adjacent_fields.json` |
| https://pmc.ncbi.nlm.nih.gov/articles/PMC12325772/ | A note on the analysis of Herrmann-May lattices for small exponent RSA | 2025-08-05 | Second warning (with 2026/244) that unravelled-linearisation asymptotics are fragile; | `literature/lattices_smallroots.json` |
| https://arxiv.org/abs/2507.08990 | Computability of Equivariant Gröbner bases | 2025-07-11 | Lever 5: NEGATIVE relevance recorded for completeness: the 2024-2026 'equivariant Groebner basis' literature (Hillar-Krone-Leykin lineage) is about infinitely many variables and decidability, not about speeding up F4 ... | `literature/groebner_complexity.json` |
| arXiv:2506.00429 | Combinatorial t-Designs from Finite Abelian Groups and Their Applications to Elliptic Curve Codes | 2025-05-31 | A 1-design property for 'k-subsets summing to b' is exactly 'every base point lies in the same number of decompositions of b' - uniformity of the relation graph's column degrees, which the programme's rank-accounting ... | `literature/adjacent_fields.json` |
| arXiv:2505.16053 | Learning from Algorithm Feedback: One-Shot SAT Solver Guidance with GNNs (RLAF) | 2025-05-21 | Direct enabler for the atlas lead LEAD-ICE-sat-xor-pdp-oracle (CNF-XOR point-decomposition oracle). | `literature/ml_cryptanalysis.json` |
| https://arxiv.org/abs/2402.07852 | The Complexity of Algebraic Algorithms for LWE | 2025-03-29 | Lever 1: methodological template for proving generic coordinates for a structured cryptographic system (the LWE proof uses the shape of the top-degree parts). | `literature/groebner_complexity.json` |
| arXiv:2502.10357 | Learning Euler Factors of Elliptic Curves | 2025-02-14 | Context for the murmuration/trace-prediction lineage; | `literature/ml_cryptanalysis.json` |
| https://arxiv.org/abs/2501.17383 | Initial ideals of generic ideals and variations of Moreno-Socías conjecture | 2025-01-29 | Lever 1: Moreno-Socías underlies both the Kouba-Neiger-Safey El Din F4 bound and the Sakata-Takagi leading-monomial algorithm; | `literature/groebner_complexity.json` |
| arXiv:2410.06856 | On Wagner's k-Tree Algorithm Over Integers | 2024-10-10 | Sharpens the programme's Wagner wall. Z_m has no small QUOTIENT either when m is prime; | `literature/adjacent_fields.json` |
| https://arxiv.org/abs/2203.06970 | Computing a group action from the class field theory of imaginary hyperelliptic function fields | 2024-03-12 | The full function-field mirror of 'class group acts on curves' exists and is broken by linearity, not by index calculus on the class group. | `literature/ffdlp_transfer.json` |
| https://arxiv.org/abs/1008.1284 | Ideal forms of Coppersmith's theorem and Guruswami-Sudan list decoding | 2023-20 | T3: on F_{2^n} = F_2[t]/(f) the factor base {deg_t x < ell} is literally the small-root set, so S_{m+1} membership is a function-field Coppersmith problem with exact reduction and (univariate) rigour. | `literature/lattices_smallroots.json` |
| https://arxiv.org/abs/2111.14180 | Two variable polynomial congruences and capacity theory | 2023-20 | The only framework that bounds ALL auxiliary-polynomial Coppersmith methods, not just a shift-polynomial family; | `literature/lattices_smallroots.json` |
| https://arxiv.org/abs/2304.13485 | An upper bound for the solving degree in terms of the degree of regularity | 2023-04-26 | Lever 1: makes d_reg (Hilbert-series computable from the top-degree parts F^top) an upper bound on d_F whenever d_reg is finite, i.e. | `literature/groebner_complexity.json` |
| https://arxiv.org/abs/2304.05009 | Lattices in function fields and applications | 2023-04-11 | Cross-lane number theory for the binary-curve factor base: 'points of a curve in a low-dimensional F_2-subspace of F_{2^n}' is exactly the Weil-descent factor base, and these are upper bounds on how many such points ... | `literature/lattices_smallroots.json` |
| arXiv:2301.09287 | The k-XORSAT threshold revisited (full-rank threshold of sparse random matrices over finite fields) | 2023-01-23 | The IC relation matrix is exactly a sparse random matrix over F_r (or F_2 after taking the kernel mod 2) with k = m+1 nonzeros per row and B columns; | `literature/adjacent_fields.json` |
| arXiv:2207.12584 | On Deep Holes of Elliptic Curve Codes | 2022-07-26 | Deep-hole classification = classification of UNDECOMPOSABLE targets over the factor base, i.e. | `literature/adjacent_fields.json` |
| https://eprint.iacr.org/2022/438 | Computing isogenies between finite Drinfeld modules | 2022-04 | Structural ingredient: the morphism space is a finite-dimensional F_q-vector space with a linear defining equation. | `literature/ffdlp_transfer.json` |
| arXiv:1912.05266 | Symmetric ideals, Specht polynomials and solutions to symmetric systems of equations | 2021-02-16 | The Semaev system is S_{m+1}-symmetric (and (Z/2)^{m-1} x S_m after fixing the target). | `literature/adjacent_fields.json` |
| https://eprint.iacr.org/2019/1329 | Drinfeld modules may not be for isogeny based cryptography (talk title: 'Drinfeld Modules are not for Isogeny ... | 2020-03-20 | Answers lever 14 ('what structural difference makes E(F_q) resist?') crisply: the Drinfeld group law and scalar action are F_q-linear in the ambient coordinates, so the whole problem lives inside linear algebra over F_q; | `literature/adjacent_fields.json` |
| https://inria.hal.science/hal-02427655 | Solving ECDLP over F_p with Pre-computation via Representation Technique | 2020 | The only representation-technique (Howgrave-Graham-Joux lineage) result on prime-field ECDLP after Delaplace-May 2019. | `literature/lattices_smallroots.json` |
| https://eprint.iacr.org/2018/795 | Solving ECDLP via List Decoding | 2018-09-02 | The only prior attempt found to run Cheng's dictionary forward as an algorithm; | `literature/adjacent_fields.json` |
| arXiv:cs/0507026 | Hard Problems of Algebraic Geometry Codes | 2005-07-08 | Lemma 3 is an EXACT dictionary: the point decomposition problem 'is target Q a sum of k distinct factor-base points' equals bounded-distance decoding of the elliptic code on the factor base at radius n-k (one below ... | `literature/adjacent_fields.json` |
| J. Cryptology 14 (2001) 225-230 (bibliographic confirmation via CRAS 2003 note ... | Public key cryptosystems based on Drinfeld modules are insecure | 2001 | Primary source for the Drinfeld DLP being easy; | `literature/adjacent_fields.json` |
| https://math.berkeley.edu/~scanlon/papers/dmc7aug00.pdf | Public key cryptosystems based on Drinfeld modules are insecure | 2000-08-07 | Answers brief section 3 item 14 ('the DLP on Drinfeld modules is broken by index calculus; | `literature/ffdlp_transfer.json` |

## 3. New routes (surviving ideas, ranked per verdicts/RANKING.json)

Each route lists its proposer statement and wall (from `ideas/*.json`) and three skeptic verdicts quoted and truncated (from `verdicts/`). The final classification after experiment is in section 4.

### 3.1 MP-comm_alg-1 -- Tensor normal form: the m=2 Weil-descended S_3 system is a Boolean bilinear system whose 3-tensor is the restricted multiplication tensor of V; d_F is a functional of that tensor, not of the curve or ...

- **Statement.** Verified in this pass (ffd_semaev_core.build_system, n=7,9,10, 9 instances): every descended equation has only monomials a_j*b_l, a_j, b_l, 1 (zero a_j*a_l or b_l*b_m or cubic terms). Equivalently the system is the K-equation a*b + Phi(a) + Phi(b) = kappa with a,b in V, Phi = z*G(sigma(.)) F_2-linear, i.e. n Boolean equations a^T M_i b + ell_i(a,b) = c_i where (M_i) are the coordinate bilinear forms of mu_V: V x V -> K, (a,b) -> ab/z. Hypothesis: the last fall degree d_F is determined by the 3-tensor T_V in F_2^{k x k x n} (plus the affine part), and the deviation of d_F (4-5) from the generic bilinear value (FSS: k+2 = ceil(n/2)+2, which coincides with the treatise's proved upper bound) is ...
- **Wall bypassed.** TREAT-first-fall-degree-constant (scope: no presentation has a proved last-fall lower bound growing with n) and KN-OPEN-8f3d27 / open problem 12. Also goes around TREAT-digit-presentation-growth's 'curve-free null' by making the null explicit (the tensor), rather than reproducing a law.
- **Proposer class.** `theory`; novelty: novelty_unverified (searched: WebSearch 'polynomial calculus degree lower bound bilinear Boolean system Weil descent tensor rank Groebner last fall degree' -> ...
- **Absorbs.** MP-wildcards-4 (Random-matrix / k-XORSAT core prediction of Macaulay rank deficiency at degree d for descended S_3), MP-comm_alg-5 (Exact F_2-Nullstellensatz degree of the descended system via trace interpolants: 1 = Tr(G*u) on the cube with G a ...)
- **Skeptic verdicts.**
  - *prior-art and closure collision*: `survives_with_fix` (conf 0.62) -- "STRUCTURAL CLAIM IS KNOWN, HYPOTHESIS IS NOT. (1) 'Only a_j*b_l, a_j, b_l, 1 monomials' is a corollary of the programme's own treatise falldegree.tex Thm reform (S = z^2(w^2+w+l0*theta), w = X1X2/z + G(s^2)+G(a6/z^2), squaring is F_2-linear so the only nonlinear part is the product X1X2 of two F_2-linear forms in (a,b)); span F = {Tr(delta w)} + F_2 l0. Mechanism (Frobenius collapse of (x1x2)^2) is Semaev S_3 char-2 ..." Fix: "Drop the 'normal form' as a finding (cite falldegree.tex Thm reform and Thm upper proof, FSS 2011, Kosters-Yeo 2015). Keep only: Conjecture 'd_F = Phi(T_V) for rank/structure of the descended 3-tensor' plus the surrogate-tensor experiment, with arm A baseline ..." (`verdicts/MP-comm_alg-1.collision.json`)
  - *experiment design*: `survives_with_fix` (conf 0.7) -- "Arm structure (true tensor vs dense vs low-rank surrogate, same affine part, same Boolean field eqs) is a valid null/control layout and the metric (closure degree d_F, dim W_d) is structural, not wall time. Four defects as written. (1) Regime overlap: d_F(arm A) is about 3-4 at n<=17 (treatise certifies >=5 only at n=33-45), and a random Boolean quadratic system with ~N equations in N=2k=12-18 variables has ..." Fix: "Replace degree-capped refutation by exact rank profiles that are cheap and extend to large n. For d in {3,4} compute dim W_d (span of m*f, deg m<=d-2, reduced mod v^2+v, plus iterated closure W_d=W_d+ closure under the treatise rule) and the excess ..." (`verdicts/MP-comm_alg-1.design.json`)
  - *mathematical refutation*: `survives_with_fix` (conf 0.6) -- "(1) Bilinear normal form is correct but trivial, not new: binary S_3 = x1^2x2^2+x1^2x3^2+x2^2x3^2+x1x2x3+a6 with x1,x2 in F_2-subspace V; Frobenius is F_2-linear so (ab)^2, a^2, b^2 are linear images of ab, a, b; system = L(mu_V(a,b)) + Phi(a)+Phi(b) = kappa. This is the Frobenius-collapse (THM_SEMAEV_FALL1 Lemma 1 / HKY) restated; relabelling. (2) 'd_F is a functional of the tensor (+affine part+kappa)' is ..." Fix: "Weakened surviving form: state the bilinear normal form as a known restatement; test the causal low-rank hypothesis only with Galois-stable rank-n controls (mu_W for other subspaces / twisted multiplication tensors L1∘mu∘(L2xL3)) versus random dense (k,k,n) ..." (`verdicts/MP-comm_alg-1.refute.json`)

### 3.2 MP-lattices-4 -- FGHR Y-coordinate small-\|Y\| base on 2-torsion prime-field curves: the only archimedean base compatible with the (Z/2)^3 x S_4 reduction, run as a weighted invariant lattice (brief lever 3) with the ...

- **Statement.** On E: y^2 = x(x^2+Ax+B) over F_p with T=(0,0) and f'(x_T) = c^2, FGHR's coordinate Y = (x-x_T-c)/(x-x_T+c) satisfies Y(P+T) = -Y(P), Y(-P) = Y(P) (verified this session at p=1000003, 200/200 points). Hence the base F_Y = {P : \|Y(P)\| < p^delta} (lift to (-p/2,p/2)) is closed under the even-sign group (Z/2)^3 acting on the four summands' Y_i, the 8 translates of a decomposition are 8 roots in the box, and H = S_5.prod(1-Y_i)^8 is a polynomial in sigma_k = e_k(Y^2) (k=1,2,3) and pi = Y1Y2Y3Y4 of weighted degree 8 on 105 monomials. Claim A (engineering): the Coppersmith lattice on the invariant monomials has dimension 105 at l=1 (vs 6561 for the plain S_5 box) and 1/8 of the box dimension at ...
- **Wall bypassed.** TREAT-symmetry-inventory ('D4 on S5 computed on paper, never run') and brief section 3 lever 3; the Lattice-reach wall's scope item 'the D4-symmetrised S_5 lattice at m=4 (never run)'.
- **Proposer class.** `engineering`; novelty: novelty_unverified (no search for 'Coppersmith lattice in FGHR invariant coordinates'; the lattices lane found nothing on summation polynomials + Coppersmith ...
- **Skeptic verdicts.**
  - *prior-art and closure collision*: `survives_with_fix` (conf 0.6) -- "Partial collisions. (1) The Y=(x-x0-c)/(x-x0+c) coordinate, Y(P+T)=-Y(P), the 105-monomial D4 rewrite sigma^a sigma^b sigma^c pi^d with 2(a+b+c)+d<=8 and weights (2,2,2,1) are already implemented and run in crypto/src/cryptanalysis/d4_weighted_s5.rs and gaudry_quartic_d4 (candidate C8, PR #1785, weighted PR #1895; research/d4_symmetrised_s5_20261015), as a Macaulay/Groebner solve over F_{p^4} (Bezout 4096->512). So ..." Fix: "Drop Claim A as a novelty claim and Claim B as a new result; reframe as a single clean run of the missing D4 Coppersmith-lattice cell (Y-box, weighted polytope) to confirm the W10 1/32 cap, cite d4_weighted_s5.rs for the 105-monomial rewrite instead of ..." (`verdicts/MP-lattices-4.collision.json`)
  - *experiment design*: `survives_with_fix` (conf 0.7) -- "Steps 1,2,4 are sound, cheap, structural and scale-free: PARI resultant identity check with wrong-c negative control; exact closure/min-cut on the weighted polytope 2(a+b+c)+d<=8 (105 monomials, verified by count) giving delta as a function of l and asymptotic in p, so it does speak to p~2^130. Step 3 (finite-p planted LLL) is a toy artefact and probably unrunnable: (a) at p~2^32-2^40, p^delta for delta 0.01..0.06 ..." Fix: "1) Keep step 1 (S_5 on B=c^2 curve, p~2^32; assert S_5(Y) invariant under even sign flips, odd flip maps to xR+T; check D4-invariant rewrite in e1,e2,e3(Y^2),pi has exactly 105 monomials; negative control wrong c and a non-2-torsion curve). 2) Step 2 as the ..." (`verdicts/MP-lattices-4.design.json`)
  - *mathematical refutation*: `survives_with_fix` (conf 0.8) -- "Identity holds (x(P+T)=B/x gives Y->-Y). As a reach lever it is refuted by a dimension count: the even-sign (Z/2)^3 x S_4 action preserves the box \|Y_i\|<p^delta and the Coppersmith shift set, so the invariant sublattice cuts dim and log det by the same factor 8 (up to lower-order boundary terms); the asymptotic Howgrave-Graham condition det^(1/dim) < p^m is unchanged, so reach stays at the x-encoding value and ..." Fix: "Keep as engineering: constant-factor ~8x lattice dimension reduction for the 2-torsion Y-base at fixed delta; reach unchanged (<=1/32). Drop Claim B or restate as conjecture restricted to group-invariant quotients." (`verdicts/MP-lattices-4.refute.json`)

### 3.3 MP-lattices-6 -- Adelic-capacity threshold of the S_3 congruence as the strategy-independent small-root ceiling (Chinburg-Hemenway-Heninger-Scherr route), computed numerically from exact full lattices

- **Statement.** By Rumely/Cantor capacity theory, auxiliary polynomials small on an adelic set exist iff its (sectional) capacity is < 1; for E = [-X,X]^m (archimedean) x {x in Z_p^m : f(x) = 0 mod p} the capacity is the limit over degree L of det(L_M)^{1/(\|M\|.L)}-type quantities where L_M is the FULL lattice of integer polynomials with support M = {total degree <= L} vanishing mod p^l on the p-adic set (symbolic power), with the sup norm on the box in place of the coefficient norm (Chebyshev constant of a box, a constant factor per monomial). Claim: the threshold delta_cap at which capacity crosses 1 for S_3 is 1/6 (the JM/closure value), not 1/4 (charging ceiling) or 1/2 (counting), so Coppersmith-type ...
- **Wall bypassed.** TREAT-lattice-reach: converts the scope limit 'non-Coppersmith small-root solvers / sublattice strategies not covered' into a theorem (if the threshold is 1/6) or exhibits the regime where a finite zero locus exists above 1/6 (if not).
- **Proposer class.** `theory`; novelty: novelty_unverified (lattices lane read both capacity papers in full; no capacity computation for a nonlinear bivariate congruence found; not separately ...
- **Absorbs.** MP-lattices-1 (Hilbert-function (leading-monomial) ceiling for every full-rank lattice inside (p,f)^l on ANY monomial set, with exact ...)
- **Skeptic verdicts.**
  - *prior-art and closure collision*: `survives_with_fix` (conf 0.55) -- "Partial collisions: (1) MAP_closed 'Lattice reach' / LEAD-ICP-coppersmith-smallroot-semaev already proves a strategy-independent ceiling for any Coppersmith lattice from (p,f)^t on a convex monomial set: (n! vol Newt(S_3))^(-1/n) = 0.3293 (3 unknowns) / 0.3536 (2 unknowns), attained 11/11; unequal-bound optimiser finds nothing above 1/2; direct S_4/S_5 caps 1/21.8, 1/70.8; ceiling 1/32 (W10). The idea's 1/6 for S_3 ..." Fix: "Restate as: prove/test whether the W10 ceiling (0.3536 per S_3 block, 2 unknowns; 1/32 and 1/70.8 for S_4/S_5) is also the capacity threshold, reporting delta in W10's normalisation and explaining any gap to 1/6; cite 1605.08065 as univariate precedent and ..." (`verdicts/MP-lattices-6.collision.json`)
  - *experiment design*: `survives_with_fix` (conf 0.8) -- "Stated falsifier as written cannot reach its own predictions. (1) Total-degree<=L basis with f of total degree 4: closed-form determinant exponent (verified in scratch, l=40) gives sup delta=0.1218 -> 0.125 (1/8) limit, not 1/6. Bidegree-box basis [0,M]^2 with lead monomial x^2y^2 gives 0.1628 at l=40 -> 1/6 exactly (and box m=3 -> 0.0402->1/24, m=4 -> 0.01196->1/80, reproducing Jochemsz-May). So 1/6 is the ..." Fix: "Replace with an exponent-arithmetic experiment (python3 fractions/numpy; no primes, no LLL for the main result; <1 CPU-min). A. Closed form: for monomial set S (convex exponent set), degree-compatible lead monomial of f, delta(S,l) = (l - e/N)/avg_deg with e ..." (`verdicts/MP-lattices-6.design.json`)
  - *mathematical refutation*: `survives_with_fix` (conf 0.7) -- "Headline claim refuted in scope; a weakened theorem target survives. (a) 'Strategy-independent small-root ceiling' is false as stated: S_3(x1,x2,x3) is degree 2 in each variable, so fixing x1 over \|x1\|<=X and solving the quadratic in x2 mod p (Tonelli-Shanks) finds every root in the box at cost ~X*polylog p = p^delta for any delta<=1. Capacity bounds only polynomial-time auxiliary-polynomial (AP) methods, not all ..." Fix: "Restate as: conditional on the multivariable capacity converse, delta_cap(S_k) is an upper bound on the reach of any single-auxiliary-polynomial construction (shift lattices, sublattices and symmetrised lattices included). It is not a ceiling on small-root ..." (`verdicts/MP-lattices-6.refute.json`)

### 3.4 MP-symmetry-1 -- Trivial-character (Reynolds) Macaulay refutation for odd-order presentation stabilisers: certificate census for the (4,l) cells at 1/\|G\|^2 memory over F_2

- **Statement.** Let F be a Boolean presentation (in B = F_2[x]/(x^2-x)) of a point-decomposition instance that is invariant under a finite group G of ODD order acting by permuting variables: (i) the Frobenius cyclic group C_n on the barrel formulation with free target shift (summand i in sigma^{k_i}(W) with one-hot selectors, target sigma^j(x_R) with one-hot selector), (ii) the cyclic group Z/3 permuting the three summand coordinate blocks of the UNCHAINED Weil-descended S_4 with x_1,x_2,x_3 in one subspace V, (iii) their product C_n x Z/3 (order 3n, odd). Then for every c: 1 in W_c(F) if and only if 1 in R(W_c(F)), where R = sum_{g in G} g is the Reynolds operator (\|G\| is invertible in F_2) and R(W_c) ...
- **Wall bypassed.** TREAT-symmetry-inventory ('symmetry cannot change an exponent; it CAN make the decisive solving-degree cells affordable') - we accept the first clause and target the second. LEAD-ICB-frobenius-search-symmetry (closed_obstructed): 'Frobenius is an equivariance between conjugate instances, not a per-instance symmetry; equivariant elimination regressed 0.29-0.59x on production systems; Frobenius-equivariant F4 on the full barrel remains an open ENABLER'. LEAD-ICE-boolean-f4-f5-f6-engines ...
- **Proposer class.** `engineering`; novelty: novelty_unverified (searched 2026-10-31 WebSearch: 'Faugere Svartz Groebner bases ideals invariant commutative group non-modular trivial character block ...
- **Absorbs.** MP-symmetry-2 (Isotypic localisation of degree falls and pseudo-solutions: per-character Macaulay deficits (Z/3 over F_4 for binary ...)
- **Skeptic verdicts.**
  - *prior-art and closure collision*: `survives_with_fix` (conf 0.6) -- "The mechanism (Reynolds averaging over a non-modular group; a Nullstellensatz/Macaulay certificate can be averaged, so refutation lives in the invariant block) is textbook invariant theory and published: Faugere-Svartz ISSAC 2013 (non-modular commutative group Groebner bases, orbit/character-block Macaulay matrices), Gatermann-Parrilo 2004 (symmetry reduction of SOS/Positivstellensatz certificates, arXiv ..." Fix: "Drop any novelty claim; cite Faugere-Svartz/Gatermann-Parrilo and A2-11 as source of the trivial-block reduction. Reposition as an engineering measurement: orbit-sum Macaulay census of c* for (4,l) cells with G odd. First verify on every instance that the ..." (`verdicts/MP-symmetry-1.collision.json`)
  - *experiment design*: `survives_with_fix` (conf 0.7) -- "Core claim (1 in W_c iff 1 in R(W_c), \|G\| odd, W_c G-stable) is a theorem given G-stability, so Arm A can only test implementation/stability of the presentation, not a conjecture; the prototype already shows AGREEMENT on n=7,l=3 and n=11,l=4. As written the falsifier has these defects. (1) No negative control: nothing could make 'agreement' fail except a bug, so 100% agreement is uninformative. Need a ..." Fix: "Redesign. ARM A0 (correctness+control, cheap): l=4 only (12 vars), n in {11,13,17}, 8+ UNSAT targets/cell (filter by 2^12 enumeration, drop SAT), full vs orbit-sum for c=4..12, record c*_full, c*_triv, rank_full, rank_triv, cols, rows. Also assert G-stability ..." (`verdicts/MP-symmetry-1.design.json`)
  - *mathematical refutation*: `survives_with_fix` (conf 0.75) -- "Core equivalence is a theorem: \|G\| odd => 1/\|G\| in F_2, R=(1/\|G\|)sum g is an F_2-linear projector onto B^G; if span(generators) is G-stable then W_c is G-stable, R(W_c)=W_c cap B^G, and 1 in B^G, so 1 in W_c iff 1 in R(W_c). No counterexample possible under the stability hypothesis; it fails only if the generator span is not G-stable (e.g. per-block different bases, or barrel selectors not permuted ..." Fix: "Restate: for G-stable generator spans with \|G\| odd, refutation degree c* is computed exactly in the trivial isotypic block; implement row orbit reduction (rows = R(m f) over pair-orbit reps) to reach ~1/\|G\|^2 memory; restrict claimed benefit to Z/3 ..." (`verdicts/MP-symmetry-1.refute.json`)

### 3.5 MP-ml_empirical-3 -- Learned/annealed subspace (factor-base) selection minimising measured d_F, not yield

- **Statement.** Search over F_2-subspaces V of dimension l (parametrised by a basis in GL(n,F_2)/stabiliser) with simulated annealing + surrogate model whose objective is the measured solving degree and Macaulay matrix size of the descended S_3 system, beyond the gated log-linear yield model; test whether non-random subspaces exist with d_F strictly below the random-subspace distribution at fixed n,l.
- **Wall bypassed.** No-subfield closure leaves non-invariant subspaces open with cost = solving degree; monodromy closure leaves solving difficulty free.
- **Proposer class.** `advance`; novelty: novelty_unverified
- **Skeptic verdicts.**
  - *prior-art and closure collision*: `survives_with_fix` (conf 0.6) -- "Partial collision, not full. Closed: LEAD-ICB-factor-base-subspace-choice (exhausted_at_scale; KR-IC-73db3f; named-property subspaces vs random, gains 1.18x at r=2^13 -> 1.000x at r=2^39, all constants vanishing with size; revisit only if a property of V predicts N_var/yield with an effect growing in n>=31). LEAD-ICB-learned-cost-model-factor-base-selection (open_next_step): already picks the subspace at fixed ..." Fix: "Reframe as extending the existing learned-cost-model lead: (1) state that the comparator is the hand rule/pool-best from LEAD-ICB-learned-cost-model, not just random median; (2) the uncovered part is search beyond a fixed pool (annealing over basis, tail of ..." (`verdicts/MP-ml_empirical-3.collision.json`)
  - *experiment design*: `survives_with_fix` (conf 0.8) -- "As written the falsifier does not discriminate. (1) No matched-budget null: 'random median' vs annealed top-5 conflates optimisation with regression to the mean/winner's curse; need best-of-600 random arm and a shuffled-objective anneal arm. (2) Metric is an F4 max-degree/matrix-size from msolve on one or few targets: a proxy, noisy per target, not the certified last fall degree; matrix size is a different quantity ..." Fix: "Redesign: (a) Metric = certified d_last/e0 and closure_dims/ideal_dims gap from prebuilt hky_last_fall_degree (cell --n N --family random --target rand\|sat --draws K --seed S --cmax C), secondary = msolve -v 2 -g 2 max F4 deg and largest matrix on SAME ..." (`verdicts/MP-ml_empirical-3.design.json`)
  - *mathematical refutation*: `survives_with_fix` (conf 0.7) -- "(1) Basis optimisation is vacuous: for fixed subspace V, change of F_2-basis is an invertible linear change of the descent variables; the ideal's Hilbert series / degree of regularity is invariant (solving degree under generic-coordinate GB is too), so only V (a point of Gr(l,n)(F_2)) matters, plus affine translates; residual measured differences across bases are msolve-ordering noise, not structure. (2) Further ..." Fix: "Drop basis search; anneal over Gr(l,n)(F_2)/Frobenius with affine translate; metric = matrix size / Hilbert-series deviation (finer than integer max degree); require identified V to belong to an explicit family definable for all n (e.g. kernel of a fixed ..." (`verdicts/MP-ml_empirical-3.refute.json`)

### 3.6 MP-comm_alg-4 -- Multigraded block law for chained systems: d_F is set by the smallest variable blocks (the m factor-base blocks of size l), not by the n-sized auxiliary blocks; test by artificially restricting the ...

- **Statement.** The chained descended system (m summands, t-1 intermediates y) is multihomogeneous of multidegree <= (1,...,1) in the blocks (a^{(1)},...,a^{(m)}, y^{(1)},...,y^{(t-1)}) up to Frobenius-collapsed lower terms (the e_2^2 term of S_3 is linear after Boolean reduction; only x_1 x_2 y is genuinely trilinear). For bilinear systems the generic degree of regularity is min(n_x,n_y)+2 (FSS 2011), and the treatise's proved k+2 at m=2 is this bound. Conjecture: for the chained system the solving degree is controlled by the smallest block sizes, d_F <= f(l, m) with f linear in l and INDEPENDENT of the y-block size n, which explains the measured flatness in n at fixed (m,l) and the measured growth ...
- **Wall bypassed.** TREAT-first-fall-degree-constant scope ('measured D at m=3 fits ceil(l/2)+4; m=4 fits l+4; cells (4,4),(4,5),(4,6) unmeasured') and LEAD-ICE-descent-closure-degree-at-scale (open_blocked: does closure degree stay at 4 as l and n grow) and LEAD-ICE-semaev-chain-cost-model (closed: charged cost is topology-blind; revisit clause: 'a solver whose DEGREE depends on topology').
- **Proposer class.** `theory`; novelty: novelty_unverified (not searched; multigraded regularity for Weil-descended chained Semaev systems is suggested in prog_openproblems.tex item 1 as an unrun ...
- **Skeptic verdicts.**
  - *prior-art and closure collision*: `survives_with_fix` (conf 0.6) -- "Multihomogeneous block structure of Weil-descent systems is published (Petit-Quisquater 2012 AC12 'block structure (multihomogeneous)' and their Dreg analysis; Faugere-Safey El Din-Spaenlehauer 2011 bilinear d_reg<=min(n_x,n_y)+1 per Thm 6.1, thesis +2 on polynomial degrees; Spaenlehauer 2012 multihomogeneous F5; Faugere-Spaenlehauer-Svartz/1805.03577 mixed GB). Programme also lists multigraded regularity only as an ..." Fix: "Do not claim multihomogeneous framing as new; cite Petit-Quisquater 2012, FSS 2011, Spaenlehauer 2012, Salizzoni 2304.13485. Keep as new only the y-block-restriction diagnostic and the block-size law for the trilinear chained S_3. Note FSS bound is ..." (`verdicts/MP-comm_alg-4.collision.json`)
  - *experiment design*: `survives_with_fix` (conf 0.8) -- "Runnable with python3+msolve (INSTRUMENTS s4: 24-var Boolean S_4 in 0.2 s; 29 vars at F4 degree ~6 plausible but 96 runs x <10 s is unverified, borderline for 20 CPU-min; no pilot run was done by this skeptic). Metric (max msolve F4 step degree) is structural, not wall time: OK. Defects: (1) NO NULL ARM. Shrinking y to l' dims removes n-l' Boolean unknowns while keeping 2n equations, so it raises over-determination; ..." Fix: "Redesign as a block-dimension grid with matched nulls. (a) Parametrise blocks (d1,d2,d3,dy): restrict ANY block to a random subspace (x-blocks too, not only y), keep field F_{2^n} and 2n equations fixed. (b) Two paired contrasts per cell: C1 = equal total ..." (`verdicts/MP-comm_alg-4.design.json`)
  - *mathematical refutation*: `survives_with_fix` (conf 0.7) -- "(1) Upper-bound half is trivial, not a multigraded insight: after Weil descent over F_2, Frobenius makes y^2 F_2-linear in the y-bits, so every chained S_3 equation has degree <=1 in the y-block; given the factor-base bits a (3l Boolean vars) the system is linear in y. Multiplying by all Boolean monomials in a of degree <= 3l spans all functions of a, so linearization yields d_F <= 3l+2 (more sharply <= deg_a + ..." Fix: "Drop the claim that multigraded structure 'explains' n-flatness (it follows from linearity in y-bits after Frobenius collapse: d_F <= 3l+2 trivially). Reframe as: (a) measure d_F(l') to test whether the y-block dimension enters below the trivial bound; (b) ..." (`verdicts/MP-comm_alg-4.refute.json`)

### 3.7 MP-lattices-2 -- Non-full-rank 'helpful-shift' sublattices and second-reduced-vector scoring: the one Coppersmith class with no ceiling, tested at finite p on S_3 and S_4

- **Statement.** Take the Jochemsz-May box lattice for S_3 (m=2) at multiplicity l=2..4 over a 40-56-bit prime, delete every shift p^{l-k} x^gamma f^k whose diagonal entry exceeds p^l (lambda.beta > k, the May 'unhelpful' rows), keep all columns, and LLL the resulting rank-deficient basis (Boneh-Durfee 'geometrically progressive' / Herrmann-May unravelled style). Score success on the first m NON-trivial reduced vectors (Gao-Feng-Hu-Pan second-vector rule, excluding the symmetry-induced trivial vectors of the x1<->x2 symmetric S_3 lattice). Hypothesis to falsify: the sublattice's Howgrave-Graham-rigorous reach for S_3 stays <= 1/6 and its heuristic reach <= the measured 0.12-0.16; the same at m=3 stays <= ...
- **Wall bypassed.** TREAT-lattice-reach, W10 section 7 class 1 ('non-full-rank and sublattice strategies: we have no general ceiling for this class').
- **Proposer class.** `accounting`; novelty: novelty_unverified (WebSearch 2026-10-31 'helpful vectors sublattice Coppersmith non-full-rank geometrically progressive beyond Boneh-Durfee general criterion' ...
- **Skeptic verdicts.**
  - *prior-art and closure collision*: `survives_with_fix` (conf 0.6) -- "Mechanism (drop unhelpful shifts to get a non-full-rank sublattice, Boneh-Durfee 1999 / Herrmann-May 2008 / Ducas-Nguyen-style focus groups; second-vector scoring per Gao-Feng-Hu-Pan 2512.19076) is textbook and published for RSA/factoring. No search hit applies it to Semaev summation polynomials (2 extended WebSearch queries: only hit is programme's own PR #1691 W10 negative, 1/80 vs ceiling 1/32). Programme ..." Fix: "Reframe as the explicit test of W10 section 7 class 1 / MP-lattices-1 gap (non-full-rank, helpful-only), cite Boneh-Durfee/Herrmann-May/Ducas focus groups as prior art for mechanism, drop novelty claim for the mechanism and claim only first application to ..." (`verdicts/MP-lattices-2.collision.json`)
  - *experiment design*: `survives_with_fix` (conf 0.8) -- "Runnable and structural (HG norm test and root recovery over Z, no wall time), but as written it cannot discriminate. (1) Finite-size confound: at l=2..4 (dim 25/49/81) the achievable delta is far below the asymptotic JM limit (arm A itself only reaches HG 0.10 against 1/6), so 'never above 0.12' is guaranteed by small l. prediction_if_true is satisfied trivially and says nothing about the l->infinity ceiling. The ..." Fix: "Redesign in three layers, all structural. LAYER 1 (the actual discriminator, no LLL needed): for each l up to 12-16 (S_3 dims up to ~900, S_4 up to l=4-5) compute exactly the log-volume of the helpful-only sublattice (log sqrt det of Gram of retained rows, in ..." (`verdicts/MP-lattices-2.design.json`)
  - *mathematical refutation*: `survives_with_fix` (conf 0.6) -- "No computation run (low-effort pass). No theorem found that closes non-full-rank sublattices: W10/LM ceilings assume det=prod over M, so class 1 is formally open. Nongenericity holds: LLL reads x as integers, so CGK does not apply. Points against it: (1) by Herrmann-May-style analysis, deleting unhelpful rows can only raise reach toward the full-rank optimum of a better-chosen shift set M'. Every sublattice spanned ..." Fix: "Restrict the claim to sublattices whose Gram determinant is not a diagonal product (geometrically progressive, non-triangular, as in Boneh-Durfee). For triangular row-deletion, treat the reach as bounded by the W10 optimum over all M. Test only the ..." (`verdicts/MP-lattices-2.refute.json`)

### 3.8 MP-lattices-3 -- Joint addition-tree lattice with full-size internal nodes used only at multiplicity >= 2 (W10 class 3 / T5), priced by exact low-multiplicity enumeration

- **Statement.** Replace the dense degree-8 S_5 by the three sparse equations g1 = S_3(x1,x2,u), g2 = S_3(x3,x4,v), g3 = S_3(u,v,z) (degree 2 per variable, 13 terms each) with \|x_i\| < p^delta and the internal nodes u, v of full size (lambda_u = lambda_v = 1). W10 dismissed this because 'every monomial containing u, v already exceeds the modulus'; that argument only kills multiplicity-1 use. At multiplicity l >= 2 a shift p^{l-k} x^gamma u^i v^j g1^{k1} g2^{k2} g3^{k3} (k = k1+k2+k3) with top monomial of size p^{i+j+delta\|gamma\|+...} is helpful whenever i + j + delta.(x-degree) < k, exactly as Boneh-Durfee use y of size e^{1/2}. Hypothesis: the joint lattice's reach delta_l at l = 2,3 exceeds S_5's ...
- **Wall bypassed.** TREAT-lattice-reach / LEAD-ICP-coppersmith-smallroot-semaev: scope explicitly excludes 'joint three-block lattices for the (4,2) system' and 'multi-ideal lattices'. Also the W10 T5 dismissal, which is incomplete as stated.
- **Proposer class.** `accounting`; novelty: novelty_unverified (not searched beyond the lane's queries; the specific observation that W10's full-size-monomial dismissal covers only multiplicity 1 is this ...
- **Skeptic verdicts.**
  - *prior-art and closure collision*: `survives_with_fix` (conf 0.6) -- "Joint multi-block lattice is explicitly listed as NOT covered and as a revisit_when trigger in TREAT-lattice-reach (MAP_closed.json l.106-110, l.886-897) and LEAD-ICP-coppersmith-smallroot-semaev.yaml (revisit_when: joint lattice for three-block (4,2) system, 3 eqs in 6 unknowns, reaching more than per-block sum). Not in ideas corpus or lead atlas as a run item. Partial collision: the lead card's mechanism already ..." Fix: "Restate hypothesis as accounting test: joint lattice variables u,v are NOT windowed (bound ~p), so compare against both (a) the closed per-block sum with windowed internal nodes and (b) S_5 direct (reach 1/70.8 per MAP_closed). Add control that the ..." (`verdicts/MP-lattices-3.collision.json`)
  - *experiment_design*: `survives_with_fix` (conf 0.7) -- "Core metric (max-weight-closure delta_l, a structural exponent) is counted not timed, and delta_l is p-independent so it transfers to 130-bit p as an exponent statement. But as written: (1) baseline numbers are mislabelled: lit2026_w10_coppersmith_bound (prebuilt, run here) gives S5 box-8 delta_l = 0.000010, 0.002455, 0.004804, 0.006334 for l=1,2,3,4; 0.0144 is S4 at l=2 and 0.0063 is S5 at l=4, so the >10% ..." Fix: "Redesign (python3 + scipy.sparse.csgraph.maximum_flow, numpy, optional cypari2; no new tools): ARMS: (A0) control reproduction by the SAME code: univariate Coppersmith d=2,3,5 (1/3,2/5; 1/5,1/4; 1/9,1/7), BD f=1+x(A+y) closure 0.25 at l=1..4, S3 and S4 and S5 ..." (`verdicts/MP-lattices-3.design.json`)
  - *mathematical refutation*: `survives_with_fix` (conf 0.55) -- "No computation run (effort budget); argument only. (1) Determinant accounting: u,v have bound p, so any shift whose leading monomial carries u^i v^j costs p^{i+j} against at most p^k saved by g^k. S_3(u,v,z)=(u-v)^2z^2-2((u+v)(uv+A)+2B)z+(uv-A)^2-4B(u+v) has u,v-free monomials only {z,1}; g1,g2 have u- resp. v-degree 2 in every monomial carrying x1^2x2^2. A triangular basis must introduce every monomial of g1^k1 ..." Fix: "Weakened claim: joint addition-tree lattice reproduces S_5 asymptotic delta (=1/(m(m+1)2^{m-2})) and may give only a finite-l dimension reduction or <10% delta_l change at l=2,3; classify as accounting/engineering. Falsifier LP (l<=3) still worth running to ..." (`verdicts/MP-lattices-3.refute.json`)

### 3.9 MP-symmetry-4 -- Twist-avoiding subspaces at prime degree are the F_2-subspaces on which the inverse-function component Tr(beta/x) is affine: a 2-torsion-invariant Boolean function and the Carlet sum-freedom / ...

- **Statement.** For E: y^2 + xy = x^3 + a x^2 + b over F_{2^n}, x != 0 lifts to E(F_{2^n}) iff Tr(x + a + b/x^2) = 0, i.e. iff f(x) := Tr(x) + Tr(beta/x) = Tr(a) with beta = sqrt(b) (Tr(b/x^2) = Tr(beta/x)). A subspace V is twist-avoiding (all-lift: every nonzero x in V lifts to E, none to the twist) iff f is constant on V \ {0}, iff the component function h(x) = Tr(beta/x) of the multiplicative inverse coincides on V \ {0} with the affine function Tr(x) + Tr(a). The function f is invariant under the 2-torsion translation x -> beta/x (x(P+T) = beta/x(P)), so the all-lift property is a property of the orbit of V under this involution and of the 'linear structure' of the inverse function on V. This ...
- **Wall bypassed.** TREAT-twist-no-relations ('UNRESOLVED: twist-avoiding factor-base subspaces at prime degree'; brief sec. 3 item 7). Not a bypass of the twist wall (mixed sums never rational, exact); it addresses the one unresolved clause with an external theory instead of a census.
- **Proposer class.** `relabelling`; novelty: novelty_unverified (searched 2026-10-31 WebSearch: 'inverse function x^{-1} over F_{2^n} affine on subspaces component function Tr(beta/x) restricted to linear ...
- **Skeptic verdicts.**
  - *prior-art and closure collision*: `survives_with_fix` (conf 0.6) -- "Trace criterion (Artin-Schreier; x lifts iff Tr(x)+Tr(b/x^2)=Tr(a); x(P+T)=beta/x) is textbook point-compression/halving, not novel. Lever 7 is explicitly OPEN in BRIEF s3 item 7 and TREAT-twist-no-relations scope_limits (UNRESOLVED: twist-avoiding subspaces at prime degree; revisit_when useful twist-avoiding subspaces at crypto prime degree), so no closure covers it. Programme already ran the census: all-lift ..." Fix: "Drop n=11 replication except as a regression check against 0.921x; run n=13 (and 17 if affordable) to test whether the ratio trends with n; test sigma_V=0 / hyperplane-in-beta and beta/x-closure as the new content; frame as evidence for lever 7, not as a ..." (`verdicts/MP-symmetry-4.collision.json`)
  - *experiment design*: `survives_with_fix` (conf 0.8) -- "Arm (1) identity check is sound but tautological (Tr(b/x^2)=Tr(beta/x)); it is a unit test, not a discriminator. Arm (2) is under-powered at l=4: null 2^-15=3.05e-5 x 2e5 samples = ~6 expected hits, so a 1.5x excess (9 vs 6, sd~2.5) can never reach p<1e-3; only l=3 (~1560 hits, sd~39) discriminates. Arm (4) falsifier 'sigma_V=0 at dim>=3' is not a discriminator: sigma_V = c1/c0 of the subspace polynomial, so ..." Fix: "Keep the n in {11,13}, K_0 + 8 random b, but redesign: (A) Replace uniform sampling with exact counts of all-lift subspaces by DFS over Z={x!=0: Tr(x)+Tr(beta/x)=Tr(a)} (\|Z\|~2^{n-1}): enumerate l=2 exactly (n=11 and 13), then extend to l=3,4 with ..." (`verdicts/MP-symmetry-4.design.json`)
  - *mathematical refutation*: `survives_with_fix` (conf 0.8) -- "Identity is correct (z=y/x gives z^2+z=x+a+b/x^2; Tr(b/x^2)=Tr(beta/x)); verified 0 mismatches over all x in F_{2^11}^* on 9 curves. The derived sigma condition has a sign error: summing Tr(beta/x)=Tr(x)+Tr(a) over V\0 gives Tr(beta*sigma_V)=Tr(sum x)+(2^l-1)Tr(a)=Tr(a), not 0. Computed: on all-lift V found, Tr(beta*sigma_V)=Tr(a) held in 100% of cases, including curves with Tr(a)=1. Sum-freedom results (Carlet, ..." Fix: "Correct the necessary condition to Tr(beta*sigma_V)=Tr(a); for fixed V the admissible (a,beta) lie on an affine hyperplane. Drop the 'closure by sum-freedom citation' claim and the sigma_V=0 falsifier. Weakened surviving form: an exact trace reformulation of ..." (`verdicts/MP-symmetry-4.refute.json`)

### 3.10 MP-number_theory-4 -- Bilinear elliptic character-sum bound (Ahmadi-Shparlinski-Voloch type) as a proof that degree-d x-functional filters have O(1) gain

- **Statement.** For A,B subsets of E(F_p) and an additive character psi composed with a degree-d rational function f, \|sum_{P in A,Q in B} psi(f(x(P+Q)))\| <= C d sqrt(\|A\|\|B\|) p^{1/2}... ; plug into the bucket-gain functional of KN-OPEN-91d4f6 to show single-bucket gain <= 1 + O(d p^{1/2}/sqrt(\|A\|\|B\|)), i.e. o(1) once \|A\|\|B\| >> d^2 p.
- **Wall bypassed.** Wagner/k-list wall open cell KN-OPEN-91d4f6 (x-functional filters).
- **Proposer class.** `theory`; novelty: novelty_unverified (not searched)
- **Absorbs.** MP-coding_comb-2 (Elliptic Johnson-bound closure of polynomial x-filters for k-tree joins (EC analogue of Kopparty-Kothary-Shankar ...)
- **Skeptic verdicts.**
  - *prior-art and closure collision*: `refuted` (conf 0.85) -- "The claimed bound is the textbook Fourier/Cauchy-Schwarz bilinear estimate: \|sum_{P in A,Q in B} psi(f(x(P+Q)))\| <= max_chi \|sum_R psi(f(x(R))) chi(R)\| * sqrt(\|A\|\|B\|) <= O(d sqrt(p)) sqrt(\|A\|\|B\|) (Kohel-Shparlinski 2000 mixed EC sums; Shparlinski FFA 2008 bilinear sums nontrivial for \|U\|>q^{1/2+eps}; Ahmadi-Shparlinski Proc. Edinburgh 2010 and JNT 2014). Relative error d sqrt(p)/sqrt(\|A\|\|B\|) is ..." Fix: "Only uncovered part: a bilinear bound for \|A\|\|B\|<p (sets p^{1/5..2/5}) with theta>=2/5 for structured/log-independent A,B, or the char-2 F_2-linear-functional version of KN-OPEN-91d4f6; the proposed Weil-type bound does not provide either." (`verdicts/MP-number_theory-4.collision.json`)
  - *experiment design*: `survives_with_fix` (conf 0.8) -- "As written the falsifier does not discriminate. (1) Regime mismatch: THM_INCBARRIER2 P5.3 says Theorem F needs BIL(theta>=1/4) at coefficient-set sizes < p^{1/2}; Ahmadi-Shparlinski-type bounds already operate at >= p^{1/2+eps}. The toy uses \|A\|=\|B\|=p^0.55 (>p^{1/2}), the already-known regime, so a pass says nothing about KN-OPEN-91d4f6. At \|A\|\|B\|<p the stated bound 1+O(d sqrt(p)/sqrt(\|A\|\|B\|)) is vacuous ..." Fix: "Replace bilinear sampling with an exact spectral test. For each p in {2^14,2^16,2^18,2^20,2^22} (2^24 if memory allows, ~128MB/array), pick random E/F_p with prime (or large-prime-factor) order N, generate all kG, k=0..N-1, vectorised: baby table iG ..." (`verdicts/MP-number_theory-4.design.json`)
  - *mathematical refutation*: `survives_with_fix` (conf 0.75) -- "(1) Scope mismatch: KN-OPEN-91d4f6 is the CHARACTERISTIC-2 cell (binary curves, G(M)=M max pi_c(d)); the proposal states a prime-field bound with interval bucket [0,p/256) and toy p=2^20..2^24, which is not F_p (2^k not prime) and not the open cell. (2) Claimed rate unproven: the provable bilinear bound by Cauchy-Schwarz + Weil (x(P+Q), x(P+Q') have distinct poles -Q,-Q', so f(x∘tau_Q)-f(x∘tau_Q') nonconstant of ..." Fix: "Weakened, provable statement: for E/F_p (p prime), f in F_p[x] of degree d>=1, any A,B subset E(F_p): \|sum_{P in A,Q in B} psi(f(x(P+Q)))\| <= sqrt(\|A\|\|B\|p) + (4d)^{1/2} p^{1/4} \|A\|^{1/2}\|B\|; single-bucket gain for interval of length p/M <= 1 + O(M ..." (`verdicts/MP-number_theory-4.refute.json`)

**Final classification after audit** (section 4):

| idea | classification |
|---|---|
| comm_alg-1 | relabelling |
| lattices-4 | engineering at most, unestablished |
| lattices-6 | accounting |
| symmetry-1 | engineering |
| ml_empirical-3 | engineering/relabelling |
| comm_alg-4 | accounting |
| lattices-2 | accounting |
| lattices-3 | accounting |
| symmetry-4 | relabelling |
| number_theory-4 | accounting |

None is an advance.

### 3.11 Deferred (survived skeptics, not run this pass)

| id | title | skeptic verdicts | reason deferred (`verdicts/RANKING.json`) |
|---|---|---|---|
| MP-alg_geom-3 | Abel-Jacobi linear-fibre presentation: decomposition as splitting of a norm form over a P^{m-1} fibre, with ... | coll=survives_with_fix, desi=survives_with_fix, refu=refuted | Two lenses expect relabelling (HKYY auxiliary-variable presentation); ~5n bit-variable blow-up gives at most constant factor at n=131; 720-run grid barely fits budget. Revisit only if comm_alg-1/4 show presentation-sensitive d_F. |
| MP-comm_alg-2 | Split-algebra normal form of the top-form ideal: over K-bar the quadratic top forms of the descended S_3 are ... | coll=survives_with_fix, desi=survives_with_fix, refu=survives_with_fix | Both predictions measured false (0 syzygies, empty zero set); residual Hilbert-function comparison is covered by comm_alg-1 rank-profile arms; theory with no lever movement. |
| MP-comm_alg-5 | Exact F_2-Nullstellensatz degree of the descended system via trace interpolants: 1 = Tr(G*u) on the cube with ... | coll=survives_with_fix, desi=survives_with_fix, refu=survives_with_fix | Merged into MP-comm_alg-1 (static d_NS arm). |
| MP-wildcards-4 | Random-matrix / k-XORSAT core prediction of Macaulay rank deficiency at degree d for descended S_3 | coll=survives_with_fix, desi=survives_with_fix, refu=survives_with_fix | Merged into MP-comm_alg-1 (semi-regular/random_matched excess-defect arm). |
| MP-lattices-1 | Hilbert-function (leading-monomial) ceiling for every full-rank lattice inside (p,f)^l on ANY monomial set, ... | coll=survives_with_fix, desi=survives_with_fix, refu=survives_with_fix | Merged into MP-lattices-6 (non-convex shapes and dim V_k census inside the sup-over-shapes run); 1/32 ceiling is a 5-line theorem. |
| MP-symmetry-2 | Isotypic localisation of degree falls and pseudo-solutions: per-character Macaulay deficits (Z/3 over F_4 for ... | coll=survives_with_fix, desi=survives_with_fix, refu=survives_with_fix | Arm P/H2 refuted by Chevalley theorem; Arm Z merged into MP-symmetry-1. |
| MP-coding_comb-2 | Elliptic Johnson-bound closure of polynomial x-filters for k-tree joins (EC analogue of ... | coll=survives_with_fix, desi=survives_with_fix, refu=survives_with_fix | Merged into MP-number_theory-4 (same character-sum mechanism, lever 8). |
| MP-lattices-5 | Function-field Coppersmith over F_2[t] as a RIGOROUS reach certificate for the subspace factor base: exact ... | coll=survives_with_fix, desi=survives_with_fix, refu=survives_with_fix | Relabels closed JM reach in F_2[t]; restricted Macaulay sub-strategy bounded by solving degree; verification item only, low leverage. |
| MP-symmetry-3 | CRT structure of the group-algebra lift closes N1 and the mu_{2n+1} exponent-shift symmetry: exact lifts are ... | coll=survives_with_fix, desi=survives_with_fix, refu=survives | N1 closure stands by CRT algebra; remaining ~1/2 co-lift fraction is low-leverage confirmation; run as 5-min unit check if spare budget. |
| MP-number_theory-5 | Drinfeld linearity defect: measure the minimal F_2-linear (additive-polynomial) approximation of tau-adic ... | coll=refuted, desi=survives_with_fix, refu=survives_with_fix | Degree argument closes the hypothesis (agreement <= 2^k+N(alpha)); record lever-14 scalar-action clause closed by theorem; optional <1-min gp symbolic arm, no experiment slot needed. |
| MP-complexity-1 | Definable-factor-base dichotomy: any bounded-complexity first-order factor base over F_p has size O(1) or ... | coll=refuted, desi=survives_with_fix, refu=survives_with_fix | Published CDM dichotomy; closure under KN-OPEN-020 as theory note; no experiment needed. |
| MP-transfer_tori-1 | Simplicity certificate: Res_{F_{2^n}/F_2}E has no non-product sub-abelian varieties or Prym quotients over ... | coll=refuted, desi=survives_with_fix, refu=survives_with_fix | Folklore; corrected statement (n prime, n!=7) checkable by gp in seconds; record as verification note, not ranked slot. |
| MP-transfer_tori-3 | Miller cocycle telescopes to an elliptic-net ratio: U(kP) = c * W(k)/W(k-1) up to N-th roots, so ... | coll=refuted, desi=survives_with_fix, refu=survives_with_fix | Accounting closure already supported by refute run (0 identity failures, null fits); duplicates filed LEAD-TRF-miller-cocycle-nonhom-transfer H1. |
| MP-transfer_tori-4 | CEILIDH/torus factor bases collapse to inverted-affine subspace bases: x = u+1/u with u in T_2 parametrised ... | coll=refuted, desi=survives_with_fix, refu=survives | Relabelling verified 400/400; presentation control could be folded into comm_alg-4/ml_empirical-3 later; low leverage. |
| MP-ml_empirical-2 | Cheap pre-solve decomposability predictor with counted-operation accounting | coll=survives_with_fix, desi=survives_with_fix, refu=survives_with_fix | Expected relabelling of point-count prior (n_valid AUC 0.569); constant factor only; CERTBIN already covers early abort. |
| MP-ml_empirical-4 | Rank-aware relation collection via learned column-coverage policy (GNN-free baseline first) | coll=refuted, desi=survives_with_fix, refu=survives_with_fix | Selection policy refuted; targeted generation is known prior art; folds into LEAD-ICE-relation-graph-rank-large-prime. |
| MP-ml_empirical-5 | Learned variable order / XOR-branching for WDSat on descended systems, priced in propagations | coll=survives_with_fix, desi=survives_with_fix, refu=survives_with_fix | Engineering; slope pinned by size-l backdoor; enumerator-floor identity predicts null; low leverage. |

## 4. Experiments

Each experiment directory holds PROTOCOL.md (pre-registered unless noted), README.md (the experimenter's own claim) and AUDIT.md (an independent rerun). Each block below gives the *audited* outcome. Each audit reran the code (`reran: true`).

### 4.1 MP-comm_alg-1: tensor rank profile of descended S_3 (lever 1)
- **Protocol.**
  - Exact bit-packed Boolean closure W_d in Python.
  - Arms: A (mu_V), A' (twisted), B (dense random), C1/C2 (Galois-stable mu_W, twisted mu(W1×W2)), and D_R (random F_2-rank R ∈ {n,2n,4n}). All arms share the affine part.
  - d=3 at n=9..45 odd, 20 draws per n. d=4 at n=9..17, 10 draws.
- **Numbers.**
  - At d=3, arms A, C1 and C2 give low_3 = 3n in every draw. B, D2 and D4 give low_3 = n. Fit of the excess: slope 2.000 ± 6e-16.
  - D1 gives about 3n+2..15, slope 1.80 ± 0.27.
  - At d=4, n=17: low_4 is about 985 for A, C and D1, against 323 for B, D2 and D4.
  - CPU 1070 s.
- **Audit.** `partially_valid`, corrected outcome **inconclusive**.
  - D1 never exceeds 5σ from A (max |z|=4.38). That contradicts "Galois structure is the cause"; the cause is rank n over an extension, via l²=l.
  - The low_3 = 3n result is near-tautological, is already Lemma deg3, and has zero variance, so the n=131 extrapolation (262) is an identity, not a fit.
  - The n=15 C2 anomaly (133–137 vs 45) is real and unexplained.
  - At d=4, n≤15 is saturation. The random_matched null, static d_NS and the msolve cross-check were cut.
- **Class.** Relabelling of the Frobenius collapse. No d_F information.
- Path: `experiments/MP-comm_alg-1/{README,AUDIT}.md`, `RESULTS.json`, `raw/`.

### 4.2 MP-lattices-4: D4-symmetrised S5 lattice (lever 3)
- **Protocol.**
  - GP identity check on y²=x(x²+5x+c²): 50 trials × 16 sign masks, with a wrong-c control.
  - Exact JM componentwise-lead reach at l=1..5 for the x-box, S4 e(x) and the D4 polytope 2(a+b+c)+d≤8l.
  - A Monte Carlo asymptote (seed 7).
- **Numbers.**
  - Identity check: 800/800 pass, wrong-c fails 50/50.
  - Reach for xbox / S4e / D4: 9.5e-6 / 1.26e-4 / 5.95e-4 at l=1, and 0.00737 / 0.00780 / 0.00824 at l=5.
  - D4/S4e ratio: 4.71, 1.26, 1.12, 1.08, 1.06.
  - Asymptote 0.01250 for all three arms (ratio 1.0003, = 1/80).
  - Dimension at l=5: D4 19481, S4e 135751, box 2825761.
  - About 4 CPU-min.
- **Audit.** `partially_valid`, corrected outcome **inconclusive**.
  - The pre-registered 5%/10% rule is met at no l. The README substituted an unregistered asymptotic criterion.
  - The identity control is tautological (it is the Möbius x→c²/x).
  - Modelling flaw: the small-root bound applies to x_i, but the invariants e_k(Y²) and π are not small.
  - Claim B is equal by construction.
  - JM componentwise reach is orders below the programme's min-cut baselines (xbox l=1: 9.5e-6 vs 0.005253).
  - Only a 5–7× (→8×) dimension cut is visible, and it is unvalidated.
  - fplll validation and the exact log-det were not run.
- **Class.** Engineering at most, unestablished. Nothing shows reach above 1/80.
- Path: `experiments/MP-lattices-4/{README,AUDIT}.md`.

### 4.3 MP-lattices-6: determinant-exponent ceiling over monomial shapes (lever 3)
- **Protocol.**
  - Closed form for δ(S,l) = Σk(s)/Σdeg(s) of the (p,f)^l family, with Richardson extrapolation from l=100/200/400.
  - Shapes: simplex, box+t, weighted simplices, downset hill-climb, S_4-symmetric orbit.
  - No primes and no LLL.
- **Numbers.**
  - Controls: 1/d to <4e-6. Box m=2/3/4 gives 0.1666656 / 0.0416661 / 0.0124997.
  - Total-degree simplex 0.12499. Weighted simplices 0.11110.
  - m=4 symmetric orbit 0.0124997, against the 0.169 needed (≈13.5× short).
  - Runtime 52 s.
- **Audit.** `partially_valid`, corrected outcome **supports**, narrowly: accounting only, m=2 monomial-downset family.
  - The audit's exact staircase optimum beats the box at finite l (l=30: 0.16158 vs 0.16120; l=80: 0.16469 vs 0.16460), so the README's "no gain over box" is false at finite l. The gap is consistent with the same 1/6 limit.
  - The controls and the null arm are tautological.
  - The m=4 orbit lattice uses x-degrees, not e-variables, so it does **not** test the D4 lever.
- **Class.** Accounting. B·D² unchanged.
- Path: `experiments/MP-lattices-6/{README,AUDIT}.md`, `RESULTS.json`, `RESULTS_closedform.json`.

### 4.4 MP-symmetry-1: Z/3 orbit-sum (trivial-block) Macaulay refutation (lever 1 enabler)
- **Protocol.**
  - A0: l=4, n ∈ {11,13,17}, 8 UNSAT targets each (exhaustive ground truth).
  - A-null: independent V_i, n=11, 4 targets.
  - Incremental F_2 echelon.
- **Numbers.**
  - c*_full = c*_triv = 9 on 24/24 instances. Span is G-stable on 24/24.
  - rank_triv/rank_full = 0.343 / 0.325 / 0.335 (memory about 1/9).
  - Word-XOR ratio 22.8–24.7.
  - The A-null agrees 4/4 at c*=10.
  - About 18 CPU-min.
- **Audit.** `partially_valid`, corrected outcome **inconclusive**.
  - The null agreed 100%. The protocol lists that as a falsification clause (no power).
  - The null is not matched: it changes the system.
  - Seeds deviate from the protocol.
  - The regime is saturated, with c* constant at 9.
  - The XOR ratio is from the Python echelon only.
  - The result is a textbook identity (Faugère–Svartz, Gatermann–Parrilo).
- **Class.** Engineering, about 1/9 memory, unvalidated in F4/M4RI.
- Path: `experiments/MP-symmetry-1/{README,AUDIT}.md`.

### 4.5 MP-ml_empirical-3: annealing over Gr(l,n) for low refutation degree (lever 1)
- **Protocol.** Four equal-budget arms (random-best, anneal, anneal on a shuffled objective, polynomial-basis family) plus 40 random-V nulls. Metric: the certified Boolean refutation degree D* on 20 fresh targets.
- **Numbers.**
  - n=19, l=5, B=200: A 2.73, B 2.78, C 2.82, D 2.03. Null p5/med/p95 = 2.45 / 2.775 / 2.90. Permutation p = 0.83.
  - n=17, l=5, B=100: A 2.85, B 2.49, C 2.89, D 2.16. Permutation p = 0.021.
  - Runtime 295 + 524 s.
- **Audit.** `partially_valid`, corrected outcome **inconclusive** (leaning refutes for the annealing arm).
  - The n=17 p=0.021 is invalid: the winners share 3–4 of 5 basis vectors.
  - D* takes integer values {2,3}, so the experiment has low power.
  - The n=23 cell did not run.
  - The poly-basis advantage (dimVV 9) is the pre-existing linearization-excess lead.
- **Class.** Engineering/relabelling.
- Path: `experiments/MP-ml_empirical-3/{README,AUDIT}.md`.

### 4.6 MP-comm_alg-4: block law vs count law for chained m=3 descended S3 (lever 1)
- **Protocol.**
  - 13 block cells × 4 seeds over F_2^13 (26 equations).
  - Metric: msolve 0.9.5 maximum F4 step degree D, cross-checked by exact XL at N≤14.
  - A random_matched null.
- **Numbers.**
  - C2 at l=3: D = 2, 3, 4 at N = 12, 15, 22. At l=4: D = 3, 3, 4 at N = 16, 19, 25.
  - C1 at N=18: tie, 3 vs 3.
  - Null D = 3–5. Δ = −1 to −2.
  - 51/52 instances UNSAT.
  - 769 s wall time.
- **Audit.** `partially_valid`, corrected outcome **inconclusive**.
  - The second equation set touches only dy+d3 variables and carries 13 equations, so local over-determination sets D. That confounds the comparison of the two laws.
  - The random_matched null destroys support locality, so it is not a valid matched null.
  - The null is missing at N≥20.
- **Class.** Accounting. The multihomogeneous framing is prior art (FSS 2011).
- Path: `experiments/MP-comm_alg-4/{README,AUDIT}.md`.

### 4.7 MP-lattices-2: non-triangular sublattices of the JM box (W10 class 1)
- **Protocol.**
  - Layer 1: exact Gram determinant of row subsets for S3 at l=1..5, plus a closed form to l=16.
  - Layer 2: qflll on planted instances at p = nextprime(2^256), 4 per cell.
  - Layer 3: controls.
- **Numbers.**
  - Arm A δ* = .0493 / .0955 / .1154 / .1265 / .1335 at l=1..5. Arm B = 0 / 0 / .045 / .071 / .078.
  - max(B'−A) = 0.000.
  - A closed form gives δ*(16) = .1553, fit δ_inf = 0.1629.
  - S4 arm A fit 0.037.
  - Layer 2: A at l=3 reaches 0.12 HG / 0.13 heuristic. B and C reach 0 at every grid point.
  - Unplanted: 0 false recoveries. About 8 CPU-min.
- **Audit.** `partially_valid`, corrected outcome **supports**, restricted to JM-box S3 threshold row-deletion with columns kept (exact l≤5, LLL l≤3).
  - The result is near-tautological by Cauchy–Binet.
  - Layer 2's zeros sit below the grid.
  - The heuristic criterion is oracle-assisted.
  - The S4 evidence is empty, so "do not trigger S5" is unsupported.
- **Class.** Accounting. Partial closure of W10 §7 class 1.
- Path: `experiments/MP-lattices-2/{README,AUDIT}.md`.

### 4.8 MP-lattices-3: joint-tree lattice with full-size internal nodes (lever 3)
- **Protocol.** scipy max-flow LM-diagonal closure. Box ≤2l per variable over (x1..x4,u,v). Arms with multiplier degree 0, 1 and 2l. A0 controls.
- **Numbers.**
  - δ_l = 0 for every arm at l=1,2 under all 6 term orders.
  - S5 box-8 reference: 0.000010, 0.002455, 0.004804, 0.006334 at l=1..4. δ_needed = 0.169–0.2.
  - Sanity runs reproduce S3 at 0.05555 and 0.1.
  - The A0 controls match lit2026_w10 (BD 0.25, S3 1/18, 1/10, 0.11905).
  - About 6 CPU-min.
- **Audit.** `partially_valid`, corrected outcome **refutes**, scoped to accounting, triangular LM selection, l≤2, box ≤2l.
  - The audit's generator-subset and box-4/6 checks still give 0.
  - l=3,4 were not run, and S5 keeps rising there, so this is no asymptotic closure.
- **Class.** Accounting.
- Path: `experiments/MP-lattices-3/{README,AUDIT}.md`.

### 4.9 MP-symmetry-4: all-lift subspace census and l*(n) (lever 7)
- **Protocol.**
  - 9 curves per prime n.
  - Exact DFS counts at n=11 (l=2,3,4) and n=13 (l=2,3), plus SIS at n=13.
  - Nulls: hypergeometric and pair-preserving shuffle. A planted control.
  - l*(n) for n=7..19.
- **Numbers.**
  - n=11 obs/null: 0.9999 (l=2), 0.996 (l=3), 0.9227 (l=4).
  - n=13: l=2 1.0000, exact l=3 0.9991, SIS l=4 0.966 ± 0.032.
  - The planted control is detected (15285 → 17529).
  - Identity check: 0 mismatches over 45 curves.
  - l* true / shuffled / formula at n=19: 5 / 5 / 6.
  - 1333 s.
- **Audit.** `partially_valid`, corrected outcome **supports** (the no-excess falsifier did not fire).
  - The README claim that the shuffle reproduces part of the deficit is refuted. Pooled shuffles give 71717 against a null of 71094 and an observed 67103, so the deficit is specific to the true Z.
  - The Poisson z-values (−7.3, −35) are invalid. Per-curve deficits are 1–2 sd, with the same sign on all 9 curves.
  - The σ check is a tautology.
  - Arm D deviated from the protocol and has no power at n≥17.
- **Class.** Relabelling. Lever 7 is not closed: l ≤ 5 is far from the l ≈ n/2 needed at n=131.
- Path: `experiments/MP-symmetry-4/{README,AUDIT}.md`.

### 4.10 MP-number_theory-4: spectral test of polynomial x-filters (lever 8, prime field)
- **Protocol.**
  - p = nextprime(2^k), k=14..22.
  - rfft sup-norm of the bucket indicator for hill-climbed f of degree 1..6.
  - Arms: REAL, NULL-1 (x permuted), NULL-2 (iid), held-out curve, planted 5% control, and a θ sweep at k=20.
- **Numbers.**
  - REAL/NULL-2 = 1.53, 1.32, 1.50, 1.31, 1.42 for k=14..22.
  - The post-hoc symmetric null gives REAL/NULL-1sym 0.92–1.14.
  - The planted control was detected only at k=22 (2.08×).
  - θ-sweep adversarial: 3.68 / 2.87 / 1.93 / 1.455, identical on NULL-1 (selection bias).
  - 763 + 60 s.
- **Audit.** `partially_valid`, corrected outcome **inconclusive**, and formally invalid at every k.
  - The planted control was below the noise floor by construction.
  - NULL-2 is mismatched: real F is symmetric, which gives a ≈1.4× artifact.
  - The search is essentially random sampling (16–40 evaluations).
  - The statistic is not the KN-OPEN-91d4f6 sum-gain filter.
  - Prime field only.
- **Class.** Accounting. KN-OPEN-91d4f6 (char 2) is untouched.
- Path: `experiments/MP-number_theory-4/{README,AUDIT}.md`.

## 5. Killed ideas (do not re-propose without a new mechanism)

Verdict files are `verdicts/<id>.{collision,design,refute}.json`. The scope column says where each kill does *not* apply, where the verdict states it.

| id | title | killing reason (obstruction; scope) |
|---|---|---|
| MP-alg_geom-1 | Branch-locus saturation of descended decomposition scheme | The system is zero-dimensional and radical with field equations, so I:D^∞ = I on generic targets. About 2^{l+1−n} branch-locus solutions per target. Rabinowitsch raises the degree. Class: relabelling or engineering slowdown. |
| MP-alg_geom-2 | Curve vs descent split of solving degree via Sym^m(E) regularity | The two regularities live in different rings, and Weil restriction does not preserve regularity. reg(I+J) needs Tor-vanishing, which fails because the system is non-transversal. The bound points the wrong way, and d_F is not CM regularity. Class: accounting. |
| MP-alg_geom-4 | Alternative elliptic fibrations of Km(E×E) as coordinates | The bidegree-(1,1) parameters are e1/e2 symmetrisation (Gaudry/FPPR). A birational change keeps scheme length (Bézout), and a factor base on t gives no P1+P2=R relations. Class: relabelling. |
| MP-alg_geom-5 | Richelot gluing E/F_{p^2} with Frobenius conjugate | Scholten 2003. Genus-2 IC over F_p costs O~(p), the same as rho, and B·D²~p³ ≫ N. Falls under the GHS row. Scope: degree-2 extension fields only. |
| MP-comm_alg-3 | Single-generator K-closure reduction of open problem 12 | u² = σ(u) in char 2 gives V^K_d = W_d ⊗ K for d≥4, so d_K = d_F. No cross-conjugate falls. The claimed n-fold saving is false. Class: relabelling. |
| MP-number_theory-1 | Elliptic-Teichmüller mod-4 lift factor base | Witt-carry equations are ideal-redundant. ord_n(2) = 10, 12, 8, 18, 11 at n = 11, 13, 17, 19, 23, so no Frobenius-stable V with l=3..5 exists. d_F ≥ subspace d_F. Class: relabelling (HPST/GG). |
| MP-number_theory-2 | Koblitz ECDLP as Artin-symbol problem in ray class field of Q(√−7) | CM reciprocity gives degree (ℓ−1)/2 exactly. bnrinit gives ray_no = φ(m)/2 at n=5..13. Prior art: IDEA-018/211. Class: theory/relabelling. |
| MP-number_theory-3 | Hecke-correspondence branch-selection on X_0(l) | Each branch is a homomorphism (transport conservation). Selection is a subset filter with yield ratio 1+O(p^{−1/2}). Scope: does not cover selection using information beyond P's coordinates. |
| MP-additive_comb-1 | Two-valued Kummer bucket gain | An exact filter gives a congruence on Z_r/±1, so it is injective or constant at prime r. The approximate case gives G2 = 1+O(M^c q^{−1/2}). |
| MP-additive_comb-2 | BGK certification of chord ceiling for subgroup x-bases | Scale error: a certificate needs ε > 1−δ, which is beyond Weil. BGK is the wrong tool. Counterexample on j=0: T3 = 552 vs μ̃ = 58.7. |
| MP-additive_comb-3 | Group-Fourier spectrum / Bohr-set yield variance | Phase obstruction: χ(x) is DL information. max\|ĥ\|/√B = 3.21, 3.49, 3.75 vs √ln r = 3.33, 3.53, 3.72, which is generic. |
| MP-additive_comb-4 | Pre-reduction size of S_3 symmetric functions as level-2 filter | Collides with LEAD-ICP-coppersmith-smallroot-semaev. Coefficients a,b ~ p make reconstruction fail: success 0.0 at 2^24, 0.011 at 2^32. Class: relabelling. |
| MP-additive_comb-5 | Szemerédi–Trotter bilinear bound for affine x-filters | The buckets are a parallel pencil, so ST gives no saving. Toy max/mean 1.013–1.129 is Poisson. Class: accounting. |
| MP-coding_comb-1 | PDP as BDD of elliptic code (key-equation presentation) | Riemann–Roch rederives S_4. The system is not bilinear in effect: about 5n+3l variables with d ~ Θ(n). Argument only; msolve not run. |
| MP-coding_comb-3 | Integer-lift S_4 carry filter on prime-field interval bases | Identity: the carry is determined by the residues, so the pruning ratio is 1. MI is within 2σ of the shuffled null for q = 3, 5, 7. Scope: carry-mod-q filters on S_m. |
| MP-coding_comb-4 | LSH near-decomposition with V-adapted complement | The Weil/Bombieri bias goes to 0. Max deficit 0.047, 0.037, 0.0099 at n = 17, 19, 23, which is noise. |
| MP-coding_comb-5 | t-design bases and k-XORSAT threshold | Over Z/r, the coupon-collector bound means uniform is optimal. Uniform needs mean 2541 relations vs skewed 4587 (B=1024). Class: accounting. |
| MP-symmetry-5 | Transfer-image refutation for 2-group symmetry | ev_P(Tr w) = \|G\|·ev_P(w) = 0 in char 2, so 1 ∉ Tr_G(V). Scope: does not cover fixed-point-free affine actions (2-torsion translation). |
| MP-complexity-2 | Carry-level CDCL/XOR decomposition for interval bases | Conflicts grow 3.5–4.3× per bit (c ≈ 1.94 UNSAT), worse than 2^k. Scope: plain CDCL only, not PC, cutting planes or lattice encodings. |
| MP-complexity-3 | Degree-d-predicate GGM simulation theorem | Fails at d=1: Legendre patterns identify x. Unique fraction 0.9996 at p = 1048573, k = 32. Collides with Legendre-PRF key recovery. |
| MP-complexity-4 | Boolean-spectrum (LMN) certificate for bit oracles | The Reed–Muller distance forces t ≥ n−2k. ANF degree 15 (n=16) and 18 (n=18); all \|z\| < 3. |
| MP-complexity-5 | Encoding-orbit invariance x→u²x+r | Collides with LEAD-STR-class-representative-ic-selection (c). The Weil bound gives max/λ → 1. The excess falls as about p^−0.73. Class: accounting. |
| MP-transfer_tori-2 | Prym-factor carrier over F_2, genus in (n−1, 2(n−1)] | Expected hits about 2^{3g−d(2g−d)/4}, below 1 for d>12. Exponent about −3835 at d=130. No explicit isogeny. Scope: forced-factor covers fall under the GHS closure. |
| MP-transfer_tori-5 | Upward embedding E(F_p) → E(F_{p^n}) with trace back-projection | Same as LEAD-ICP-extension-field-embedding / ECDLP-IDEA-009. Tr(F_V) is essentially all of E(F_p), so B ~ r. Scope: not composite-field targets. |
| MP-ml_empirical-1 | Symbolic-regression law for d_F | Held-out cells are censored (d_F ≥ 5), log2 n spans only 3.70–5.49, so the law is unidentifiable. Class: accounting. |
| MP-wildcards-1 | BP / cavity predictor of solvability phase | About 2^{2l−n} ≤ 1 solutions. BP stays paramagnetic, with bias ≤ 2^{−rank/2}. Decimation amounts to hybrid guessing. Argument only. |
| MP-wildcards-2 | Automaton τ-NAF factor bases on Koblitz curves | Generic: membership needs the scalar. 0 annihilators at n = 23, 29, the same as random. The 66 at the 300-subset was a sort artifact. |
| MP-wildcards-3 | Tropical / BKK mixed-volume count | MV/Bézout ~ l^{−(m−1)/2}, which is only polynomial decay. Closed form only. Class: accounting/engineering. |
| MP-wildcards-5 | Transfer-operator spectrum of walk through x mod V | Same as KN-OPEN-010 / LEAD-EXO rows. All 32 hit ratios are within 2.4σ of 1 (n = 17..23). Spectral-gap test not run. |
| MP-wildcards-6 | Low-rank tensor-network contraction of hashed relation count | The flattening rank is Θ(2^{2l}): 69, 252, 1018 at l = 4, 5, 6 (n=11). Class: constant factor. |

## 6. Ranked next steps (with cost)

Costs are taken from the README/AUDIT `next_step` fields and the measured runtimes above. Each step's class is the expected one, given in advance.

| # | step | lever | why | cost (est., from measured runtimes) | expected class |
|---|---|---|---|---|---|
| 1 | Compiled M4RI-style GF(2) eliminator. Run low_4 and the 1-in-W_4 test for arms A, C, D1 vs B at n=19..29, with the random_matched null and static d_NS that were cut from 4.1 restored. | 1 | This is the only direct d_F-growth probe in the pass, and the Python closure capped out at n=17 (1070 s). | Engineering days, then hours CPU | theory |
| 2 | Rebuild the D4-symmetrised S5 lattice with smallness stated on x_i (not the invariants). Cross-check it in the programme's min-cut closure code and the lit2026 binary. Run one fplll validation at l=1, dim 105, p ~ 2^256, ≥8 seeds, null xR. | 3 | Lever 3 is still unrun in valid form (4.2, AUDIT issue 3). | Minutes to an hour CPU | engineering expected (≈8× dimension cut, reach ≤ 1/80) |
| 3 | Capacity/LP converse: a sectional p-adic interpolation determinant at small p for polynomials outside (p,f)^l, plus an LP over k(s) for multi-ideal and non-downset sets. | 3 | Turns the 4.3 accounting ceiling into a statement about all auxiliary polynomials. | <1 CPU-hour | accounting |
| 4 | Char-2 version of the x-filter spectral test (Tr(f(x)), F_2-affine buckets, binary curves). Make the symmetric NULL-1 the primary null, calibrate the planted dose-response before the run, and raise the climber budget (k=24). | 8 (KN-OPEN-91d4f6) | 4.10 was formally invalid and prime-field only. | About 1–2 CPU-hours | accounting |
| 5 | Importance-sampled all-lift counts at l=5,6 for n=17,19, with ≥7 shuffles per curve to attribute the 0.92× deficit. Seek a proved upper bound on l*(n). | 7 | The deficit is real and specific to the true Z (4.9 audit). | About 1 CPU-hour | theory/relabelling |
| 6 | Symmetry-1 arm A1: l=5,6, trivial block only, c≤10, n=11,13. Use a matched non-symmetry gmap control on the same system. | 1/5 | Makes the (4,5) and (4,6) cells affordable at about 1/9 memory. | <1 CPU-hour | engineering |
| 7 | Block law: n=17–19 with N ≈ 2n, l=5,6, exact refutation degree for both chain and null. Use a null that keeps support locality. | 1 | 4.6 was confounded by local over-determination. | Hours | accounting |
| 8 | Anneal on a continuous structural score (dimVV / linearization excess) at n=23, 29, 31. If it does not beat the poly-basis family, fold into LEAD-ICE-linearization-excess-fb-prior and file the annealing arm as exhausted. | 1 | 4.5 was low-power. | Hours | engineering |
| 9 | Optional: Herrmann–May unravelled linearization for S3, and S4 at l=2 (729-dim float QR). Otherwise mark W10 §7 class 1 empirically closed for row-deletion sublattices. | 3 | Scope gap in 4.7. | <1 CPU-hour | accounting |
| 10 | Theory notes, no experiment: MP-complexity-1 (CDM dichotomy), MP-number_theory-5 (degree theorem), MP-transfer_tori-1 (gp check, seconds), MP-symmetry-3 (5-min unit check). | various | These close deferred items on paper. | Minutes | accounting |

**Status after next-4 run (2026-11-01).** Steps 1-4 were run and repaired within audit items 1-4. All four re-audits came back partially_valid, and no lever was demonstrated. N1 replicates d_F=4 through n=25, with the run unfinished. N2 is engineering only, with the bracket log2 X in (3.58, 7). N3 gives the 1/32 ceiling only conditionally on the unproved premise P. N4 is inconclusive, with no growth seen. Details, audited numbers and open items are in [NEXT4_REPORT.md](NEXT4_REPORT.md).

## 7. Honesty appendix

**Novelty.** Every surviving idea card carries `novelty_unverified`. The searches behind each are listed in its `novelty_status` field (`ideas/*.json`). No experiment README claims verified novelty. The collision skeptics found prior art for the core statements of MP-comm_alg-1 (treatise Thm reform, Frobenius collapse, FSS 2011, Kosters–Yeo), MP-symmetry-1 (Faugère–Svartz, Gatermann–Parrilo) and MP-comm_alg-4 (FSS 2011, Spaenlehauer 2012).

**What was not run or not searched.**
- *Experiments.* Many protocol arms were cut for budget. The full lists are in each AUDIT.md. Main cuts:
  - comm_alg-1: random_matched null, static d_NS, semi-regular corank, msolve cross-check.
  - lattices-4: fplll validation, exact log-det, l=6 (killed).
  - lattices-6: D4 e-variable region, squarefree check.
  - symmetry-1: A1 (l=5) and barrel arm B.
  - ml_empirical-3: the n=23 cell.
  - comm_alg-4: null at N≥20.
  - lattices-2: S4 l≥2.
  - lattices-3: l=3,4, Howell det.
  - number_theory-4: the coding_comb-2 even-character arm.
- *Kills by argument only.* Several kills rest on argument with no computation: alg_geom-1, alg_geom-2, alg_geom-4, coding_comb-1, number_theory-1, number_theory-2, number_theory-3, wildcards-1, wildcards-3. Each verdict says so.
- *ML-empirical-6.* `verdicts/MP-ml_empirical-6.{collision,refute}.json` (refute: `refuted`) and `experiments/MP-ml_empirical-6/` exist, but there is no idea card for it in `ideas/ml_empirical.json` and it is absent from `verdicts/RANKING.json`. It is not counted in the 57.
- *Sage.* Not used anywhere (BRIEF rule 7).

**Literature access failures** (from each lane file's `access_problems`):
- *eprint_recent.* WebFetch got HTTP 429 on about 20 abstract pages, all recovered by curl. PDFs were not read. 17 curl'd search pages parsed to 0 rows; coverage relies on the full days/60 listing (1298 entries). 2026/2014 (received 2026-09-14) was omitted.
- *arxiv_recent.*
  - PDFs were read via pdftotext, first 7 pages only. 2609.32065 (Voloch, Prym) was read at abstract level only.
  - Three API 'published' dates are inconsistent with their IDs.
  - Two borderline items fall 2–3 days before the window.
  - Phrase search is incomplete.
  - Referenced "public preprints claiming classical sub-exponential prime-field ECDLP" and an OpenAI preprint [Ope26] on polynomial factoring were **not located or verified**.
- *groebner_complexity.* The treatise bib key `iterated2023` = Steiner arXiv:2310.03637 is inferred, not verified. Partial HTML reads (Steiner §8, Gorla–Trebiani, Gaggero–Gorla). msolve release years are unverified. Li–Sun F4 has no paper. Several targeted searches returned nothing; that is reported as "not found", not as non-existence.
- *lattices_smallroots.* ePrint PDFs (2026/1027, 244, 423, 2025/1812, 2024/1577, 2026/2621) and HAL are behind Anubis, and Springer/ScienceDirect are behind login walls. The numerical bounds in those papers are **unverified** (abstract or snippet level). cuso was not run (Sage hangs).
- *adjacent_fields.*
  - ePrint 2018/795 and 2019/1329 got 403, and the Joux–Narayanan mechanism is the agent's own reconstruction.
  - Semantic Scholar returned 429, and Google Scholar was unreachable.
  - `brief/MAP_ideas_corpus.json` is absent.
  - No paper was found on BKK/tropical Semaev, or on newer E(F_p) sum-product estimates.
- *ffdlp_transfer.* Joux–Narayanan was abstract-only. Leudière–Spaenlehauer is blocked. Diem 2013 is from memory. Kleinjung–Wesolowski was read to 100k of 142k chars. The arXiv API returned the first 40 hits only.
- *ml_cryptanalysis.* OpenReview, researchsquare, ePrint 2026/1688 and 2026/2556 (429) and blog.cr.yp.to failed. The HAWK figures are second-hand press. Every entry is abstract-level.
- *program_self_review.* The local full tree lags the treatise. Several cited directories are absent (prime_ic_program_20261029, prime_fall_degree_20261027, lift_dense_factor_base_20261027, gaudry_k4_char2_redteam_20261026). The "five filed follow-ups 2026-10-27" on Prym embeddings were not found and contradict the atlas. Idea card IDEA-20261017-e3a6f1 is missing. The ml-cryptanalysis and cryptanalysis trees were not consulted.

**Brief corrections surfaced.** Three corrections come from `literature/program_self_review.json`, `literature/lattices_smallroots.json` and `literature/ffdlp_transfer.json`:
- The D4 Macaulay solve ran. Only the D4 *lattice* is unbuilt.
- Cells (4,4) and (4,6) at S=−9 ran. (4,5), the S=0 cells, l=7–8 and the FGHR arm did not.
- The Drinfeld DLP break is linearity, not index calculus.

**Numbers not independently re-derived by this writer.** All figures are transcribed from the README/AUDIT/RESULTS/verdict files cited. Literature relevance lines are first clauses of lane-agent text, not checked against the papers.
