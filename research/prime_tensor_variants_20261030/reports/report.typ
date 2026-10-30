#set page(paper: "a4", margin: 17mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: false, leading: .65em)
#set heading(numbering: none)
#let blue = rgb("#18283e")
#let amber = rgb("#fff1d6")
#let note(body) = block(fill: amber, inset: 9pt, radius: 4pt, width: 100%, body)
#show heading: set text(fill: blue)

#text(size: 20pt, weight: "bold", fill: blue)[Packet variants for prime-field ECDLP]
\ Exact checks and the prime-field ECDLP research boundary
\ 30 October 2026 UTC · research/prime_tensor_variants_20261030

#note[*The requested essentially new sub-exponential ECDLP algorithm remains unachieved.*
This work adapts an existing tensor method to three-adic arithmetic. It supplies
an independently reviewed written argument and finite native checks. It does
not improve the existing exponent, certify a general implementation or establish novelty.]

= What changed
The proposed order-three packet action gives real selected-coordinate locality.
Independent review found a missing target index: one source tile can cross to
two groups. Splitting each mode by target repairs the group factorization and
changes the mode bound from `9M/D` to `18M/D`. The retained incorrect formula is
an actual negative control, rather than an explanation inferred from timing.

#image("target-split.svg", width: 100%)

= Mathematical scope
The written adaptation contracts a bipartite Boolean-edge network over powers
of 3 in `poly(s+Dlen) 2^O(s/sqrt(log s))` bit work and polynomial space,
under the stated size, degree, precision and charged entry-evaluation bounds.
For 0/1 tensors, root precision `M+1` determines the integer count exactly.
This is the same exponent class as Appendix D.1 of the pinned October 25
manuscript. Its application to ECDLP still uses the separate published compact
hashed-factor-base reduction. This assumes a nonsingular curve over a prime
field of characteristic greater than 3 and known odd prime whole order; the
source treats a fixed-cofactor extension separately.

Sources and full proof: `sources.md`, `ternary-derivation.md`,
`reviews/ternary-global.md`; theorem-to-Lean map: `proof-map.md`.

#pagebreak()
= Exact development evidence
The protocols were frozen before native implementation. These are local
correctness checks, with no timing acceptance criteria or scientific isolation
claim. Tiny forced parameters do not admit the full asymptotic recursion.

#table(columns: (2fr, 1.1fr), inset: 5pt,
  table.header([Packet / local identity unit], [Recorded count]),
  [Full pair classes / orbits], [897 / 251],
  [Nonzero slot-locality checks], [22,278, all pass],
  [Explicit matching edges covered exactly once], [913,731],
  [F67 single-pair locality checks], [26,934, all pass],
  [Correct target-split local cases], [192 / 192],
)

#image("mode-mismatches.svg", width: 100%)
#text(size: 8pt)[Population: all 64 three-bit pin pairs at each modulus. Native source:
`packet-unit/mode-records/summary.json`; every case row is retained. The graph
plots recorded counts, not runtimes or asymptotic extrapolations.]

#table(columns: (2fr, 1.1fr), inset: 5pt,
  table.header([One active call + direct leaves], [Recorded count]),
  [Root cases at precisions 1 and 2], [128 / 128 pass],
  [Mode/direct/exact-chart evaluations], [19,200],
  [Actual two-edge child contractions], [1,555,200],
  [Explicit child assignments], [6,220,800],
  [Full chart coefficient comparisons], [19,200],
  [Exact and canonical-lift Hermite recoveries], [128 / 128 pass],
)

The unit uses `U^3-U-1` over the three-adic coefficient ring, 25 distinct
residue-field lifts, six active Hermite nodes, and child precisions 121 and
122. It explicitly sums all four assignments per two-edge child. A separate
six-bit direct sum and exact rational chart control the factorization.

Flow: all planned fixed cases completed; no case excluded. The first strict
one-level build failed on an unused helper; its diagnostic is retained.
Two final rebuilds replay byte-identically. Parent replays also pass.
Sources: `packet-unit/README.md`, `onelevel-unit/README.md`, both protocols,
all case rows, build logs and independent source reviews.

#pagebreak()
= Formalization and remaining gaps
Twenty base Lean theorems check the arbitrary-additive-group rotation/action,
selected-coordinate locality, uniform positive reached fibers and the fixed-point
hypothesis including F67. A versioned addition supplies eight more theorems:
actual integer valuations give the required nonzero digit and a valid bounded
slot coordinate. The 28 unique theorems pass explicit module builds,
transitive axiom audit, unmodified shared gate and official fresh kernel replay.
Only `propext`, `Classical.choice` and `Quot.sound` occur. Base failed attempts
and later probe diagnostics are preserved.

Pins: Lean v4.35.0; shared crypto-lean
`148f5dc389b341401e06d76abe7783c4ccf42f2e`; Mathlib
`cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`.
Exact hypotheses, commands and remaining gaps are in `lean/PROOF_MAP.md` and
`lean-digits/PROOF_MAP.md`. Kernel support, graph packing, three-adic precision,
target-split factorization, the recursive machine and complexity are not yet
Lean formalized. One active native call does not test multiple active levels.

= Other investigated shortcuts
- Stronger mode weights and taper changes do not improve the current
  enumeration bound by themselves. This is not an arbitrary-algorithm lower bound.
- A direct product deformation restores passive channels. A broader proposed
  code-occupancy barrier was false without extra premises; repeated-slot
  counterexamples are retained, and no general impossibility is claimed.
- A concrete scalar window verifier has `O(b^(20/19))` Boolean edges, but the
  cited contraction bound then gives no sub-exponential guarantee in `b`.
- A sharper heavy-vertex collision formula does not remove whole-vertex
  padding restrictions. Explicit Boolean code is not automatically an exact
  weighted multilinear-extension evaluator.
- The literal Fourier graph has large dense binary contraction width. That
  argument does not exclude implicit methods or a changed representation.

Evidence: `route-decisions.md` and all linked original/independent reviews.
These additional written arguments remain outside the completed Lean scope.

= Request status and next research obligation
Essential novelty, a complete original algorithm, general implementation audit,
exhaustive historical six-repository coverage and isolated increasing-size
measurements remain open. No leaderboard or benchmark result is promoted.
No performance graph changes: existing scoreboard/timeline and prior-study
visuals are left unchanged because no new admitted runtime or pipeline value
was measured. This continuation adds its own evidence diagram and count graph.

The selected next bet is a common correlated-packet kernel, described on the
next page. Exact compact scalar-predicate splitting remains an alternative.
Full requirement mapping: `requirements.md`. Reproduction: `README.md`,
`check.sh`, and the pinned Lean project.

= Primary references
#text(size: 8pt)[
Akshima et al., #emph[A Subexponential Time Algorithm for Elliptic Curve Discrete
Logarithm], v0.2, 29 October 2026; source commit
`1ee44e562b1b9a50ea339f3b8f81112b5a24b96f`, bochum-crypto/subexponential-ecdl.
\ #emph[The Exponential Time Hypothesis is False], 25 October 2026, Appendix D;
source commit `7ef5c0cc5ca4b0e072b7a050fca09ab24bc6fa3a`, openai/math.
\ Exact URLs, PDF hashes, scope and literature-search limits: `sources.md`.
]

#pagebreak()
= The one next bet selected from this round
Try a single deformation with compatible two-axis local factorizations at each
valuation level. The reason is concrete: origin recovery has the needed
precision already; it is the common geometry and charged mode density that
remain missing. This is a structural matrix problem independent of a secret
scalar. A change of coefficient prime alone does not supply it.
This lead uses the binary source kernel, separately from the ternary units.

#image("common-kernel.svg", width: 100%)

For a product deformation of bidegree `(d,d)`, restricting to `G(T)=F(T,T)`
gives degree at most `2d`. The source's Hermite argument recovers the origin
from diagonal matched-level samples at precision order `MD/u`. Thus mixed-level
samples are unnecessary for this step. Two independent derivations are retained.

The hoped-for two-index mode count `O(M/D^2)` per `(b1,b2)` would give the
conditional envelope `exp(O(M/u^(2/3)))`. However, imposing equality at all
retained axis bits leaves only `L^2/2^u` physical slots. Direct counting loses
the hoped-for density; a constructive finite-support moment reduction restores
the old mode-enumeration envelope, without proving a faster algorithm.

#note[Changing the packet geometry separately by level needs a proved common
deformation or congruent-jet identity. Shared origin, bounded degree and integral
charts alone do not suffice. No exponent improvement or new ECDLP algorithm is
established here.]

The written binomial counterfamily satisfies all three generic properties yet
has zero levelwise data and origin one. It charges negative nodes, every chart
coefficient, degree and denominator sizes. Nine additional checked Lean theorems
prove signed binomial parity, rational-polynomial evaluation, origin and node
divisibility. They do not prove full coefficientwise chart integrality or the
entire data-collision theorem. There are 37 named theorem declarations across
the study, rather than a formalized global algorithm.

Actual realization of the counterfamily by repacked Cauchy networks is unknown.
It refutes a generic data-only bridge, not every compatible kernel or a
code-aware algorithm. A valid new bridge must exploit and prove that additional
structure. If every required density, recursion and cost property closed, the
inherited prime-field reduction would have a conditional target exponent
`O(b/(log b)^(1/3))`. This is a research target, not a runtime bound.

Sources: `reviews/matched-level.md`, `reviews/matched-recovery-independent.md`,
`reviews/congruence-moments-independent.md`,
`reviews/levelwise-binomial-counterexample.md`, and `lean-binomial/PROOF_MAP.md`.
Concrete acceptance conditions: `problems/correlated-packet-common-kernel.md`
at the repository root. No timing or native implementation of this proposed
common kernel has been run.
