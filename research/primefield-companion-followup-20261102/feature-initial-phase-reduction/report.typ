#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Initial mode constraints by quotient powering
T-1910 · Optional prime-field recovery initializer · 2026-11-08

Build the phase domain before mode0: construct Dₙ(W) by Dickson doubling, take the squarefree phase support and impose Vⁿ=u canonically. Binary powering of (X−1)ʰ modulo Xᵐ−V then supplies all mode sections. Filter mode0 and reuse the signed sections in the smaller quotient. Successful powered initialization does not invoke the mode coefficient reader.

#image("flow.svg",width:100%)

*Phase capacity.* For normalized character +1, the maximum squarefree W degree is floor(n/2)+1; for −1 it is n. The preflight checks twice m times that degree. Inversion pairs of V roots establish the degree formula, including fixed points V=±1. Rejected capacity uses the existing seeded streaming constructor. The slot bound covers logical section output, not peak memory.

*Work retained.* n=(r−1)/(4m). Dickson polynomials still reach degree n, and recursion depth is not total arithmetic complexity. Squarefree gcds, phase filtering, quotient products/reductions, allocations and scalar inverses remain. Sections are computed before mode0 can discard a branch. Source extraction, public-radius normalization, replay and point verification remain charged.

*Representation constraint.* Fixing the initial sign leaves L=(r−1)/(2m) m-th-power images. Excluding the possible endpoint leaves L−δ. Mode0 has at most m+1 sign-sum values, so some mode0-only fiber contains at least ceil((L−δ)/(m+1)) images. This bounds the degree of an explicit polynomial for that fiber, not typical retention or the full multi-mode recovery. It motivates combined constraints before large intermediate polynomials.

*Sources.* NIST DLMF18.5.1 and the predecessor reciprocal, canonical and powered-section modules. KB unavailable; exact-source fallback recorded. See DERIVATION.md for assumptions and the written proof. No new full Lean runtime refinement is claimed.


*Qualification.* 15,264 support comparisons and 2,529 public target/capacity cases pass with equal outcomes, routes and recorded source statistics. There are 132 Dickson equalities, 24 phase-degree checks and 12 exact/one-below capacity boundaries. 2,131 powered initializations and 4,160 fallbacks are coverage totals, not timings. Both revisions pass inherited regression. 120 copied sources and 134 dependencies are hash-bound. PCC-20261108-b7a0ed is unsigned producer evidence; independent review pending.

*Provenance and next construction.* The earlier 2mn-cap revision and changed sources are preserved. A shared-scope refusal was initially followed by an erroneous planning-log append; later shared writes waited for reservation. Next implement the derived half-degree squarefree factors for positive phases, avoiding their degree-n Dickson polynomial and squarefree gcd. These formulas are documented, not included as current savings.
