# Sources checked, 17 November 2026

This is a bounded primary-source and repository review. It does not establish
novelty or exhaust the literature. A search for 2025/2026 ECDLP index calculus
also returned quantum-annealing and quantum-circuit work; no classical
Curve25519 exponent conclusion was drawn from those search results.

| Primary source read | Passage and consequence for the proposal |
|---|---|
| [Petit--Kosters--Messeng, PKC2016](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf) | Sections 3--5, especially generalized root finding in section 5. Compositions and auxiliary isogenies are prior art; dedicated solving and asymptotic complexity remain separate questions. |
| [Kosters--Yeo, Notes on summation polynomials, v3](https://arxiv.org/pdf/1503.08001) | Introduction and first-fall/regularity discussion. A small first-fall degree does not provide a family-wide solving bound. Its conditional large-arity hardness statement is not a fixed-arity average-case Curve25519 exclusion. |
| [Diem, Compositio 2011](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/59B877810708C90F6287972486A5BF0C/S0010437X10005075a.pdf/on-the-discrete-logarithm-problem-in-elliptic-curves.pdf) | Introduction, theorem and decomposition construction: genuine extension/subfield hypotheses. Those are not present over a prime field. |
| [Huang--Kosters--Petit--Yeo--Yun, 2020](https://pure-oai.bham.ac.uk/ws/portalfiles/portal/97895891/Huang_et_al_Quasi_subfield_polynomials_Journal_of_Mathematical_Cryptology_2020.pdf) | Sections 3,4 and Appendix C: characteristic-power constraints, parameterized complexity, and existence limitations. The proposal does not transfer these bounds to p=2^255-19. |
| [Bhargava--Ghosh--Guo--Kumar--Umans, arXiv2205.00342v1](https://arxiv.org/html/2205.00342v1) | Introduction and result statements: explicit coefficient-vector input. Fast evaluation is not implicit eliminant construction or root discovery. |
| [Wagner, long version linked by author](https://people.eecs.berkeley.edu/~daw/papers/genbday.html) | Operations other than xor and relation to discrete logs, Theorem 2/Corollary 1. Both integer interval joins and quotient versions matter; do not reject all k-sum merely because the elliptic subgroup is prime. |
| [Snowden, Math679 lecture 2](https://websites.umich.edu/asnowden/teaching/2013/679/L02.html) | Divisors, Riemann--Roch and E(k)=Cl^0(E). Supports the target-bearing divisor construction and zero/pole degree warning, not an efficient norm-factorization algorithm. |
| [Coppersmith, author publication page](https://research.ibm.com/publications/small-solutions-to-polynomial-equations-and-low-exponent-rsa-vulnerabilities) | Author abstract distinguishes univariate modular and bivariate integer results. Full 1997 PDF fetch failed; no precise lattice threshold from that paper is asserted here. |

The primary Wagner PostScript was downloaded directly and converted with
ps2pdf/pdftotext outside the repository after browser tools rejected its content
type. Author URL: https://people.eecs.berkeley.edu/~daw/papers/genbday-long.ps.
Downloaded SHA256 c523a243cc92b3fc4c57d83db524c2a2003c91aad2ec5a643a70ac598961fba0.
Local decoded text was read in the two sections above; PDF/text conversions
are reading aids, not scientific execution or signed receipts. The IACR PDF
returned 403. A guessed eprint URL was inaccessible and not relied on.
Coppersmith PDF mirrors returned 403/404/502; the limitation is explicit.

Repository baseline: local f9d1dfef823df42672d70720bdf9fc958be39b15,
published equivalent tree at 2c9dbdf2b38a9a0f5d4a92c7327492dc865f4ab6;
current-main read 94479f7e3eb9de9ae59b764d2b2b50b3238f0a44.

- problems/curve25519-torus-four-point-decomposition.md: original gamma<1
  contract and missing geometric/distribution/solver bridges.
- reports/scaling-audit-v1/REPORT.md and lean-scaling/PROOF_MAP.md: saturation,
  earlier illustrative exponent points, current cardinality gap. No new Lean
  endpoint or gate execution is claimed.
- theory-v1/energy-lemma.md: average over an admissible base randomizer,
  four-sum only, not a fixed-base/six-sum useful-yield theorem.
- energy-test-v1/RANKFREE_REDUCTION.md: more than B common-column rows and
  controller-private tag selection; do not invent a full-rank requirement.
- root-reference-v1/PROTOCOL.md and reports/root-reference-v1/REPORT.md:
  triple enumeration, witness lifting and fixed-fixture scope.
- rowspace-audit-v1/{README.md,AFFINE_QUOTIENT_PLAN.md}: constant combinations
  and affine quotient proposals have narrow scopes; neither is a family bound.
- crates/ecdl-tensor-ic/{README.md,src/solver.rs}: counting-to-search and toy
  contractors already exist. No new generic tensor algorithm is claimed.
- examples/quasi_subfield_reach.rs: existing binary-extension search,
  not a new prime-field construction. The joint-smallroot-native paths were
  located only; they must be read before extending the secondary lattice lead.
- docs/ic/SUCCESS_CRITERIA.md: cold single-target interval and T2 gates.

KB unavailable: no loaded kb_status/search_knowledge/search_theorems tools;
scripts/research-kb status fails 127. Queries used in the external search:
prime-field index calculus, first-fall limitations, PKM2016 generalized root
finding, quasi-subfield polynomials, generalized birthday groups, and fast
multipoint evaluation. No absence or novelty conclusion follows from this
bounded search.

Publication continuation: PR4511 merged during this search. Only the new, unpublished proposal files were transferred to canonical main19eb53c9d993596192d8dfc45b0293e3862dc9e5. Current AGENTS e3fb17ddc5e31403ae962dba70d00de884c941ce1093a9fe5acd905aa5576493 introduced draft-first task tracking; its standalone tracking commit f9b497c423 was published to the new lane and draft PR4522 before further publication work. Prior literature/drafting had already occurred under the older instructions; no retroactive first-step or timestamp is claimed. Old published ledger events were not re-registered.
