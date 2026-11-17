#set page(paper: "a4", margin: (x: 15mm, y: 14mm))
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.6em)
#set heading(numbering: none)
= Symmetric PKM: independent rank and recovery certificates
2026-10-30 · G1–G6 continuation · Native checker and proof development

*Question:* can symmetric PKM produce useful elliptic-curve relations cheaply enough to improve a complete cold single-target discrete logarithm? *Answer remains unestablished.* This continuation independently checks the useful-rank and target-recovery records. It supplies no scientific panel, measured gain, GAE result or complexity improvement.

#table(columns: (0.5fr, 1.4fr, 3fr), inset: 5pt,
  [*Goal*], [*Status*], [*Still required*],
  [G1], [Partial], [Frozen multi-curve m=4 complete-output qualification and controls.],
  [G2], [Partial], [Full work counts, failed-process records, residual pricing and admitted measurements.],
  [G3], [Partial], [Equal-domain presentation ablation, qualified external solver and A/A/null controls.],
  [G4], [Unrun], [Frozen complete baseline and one held-out measured improvement.],
  [G5], [Unrun], [Predeclared size/depth panel, retained censoring and held-outs.],
  [G6], [Partial], [Qualified native execution/resume, matched cold references and Rust/polynomial/cost bridges.],
)

== Native contracts
Replay uses a separate rightmost-pivot row space with integer Euclidean inverses and original-row provenance. It checks coefficient and augmented rank, useful yield after each query, and canonical final RREF membership. Recovery returns sparse original-row weights isolating the target and its right-hand side. Unknown results return a kernel vector that changes the target coordinate by one while preserving every retained scalar equation. Both witnesses are checked directly against original rows. Ambiguity concerns these equations, not multiple logarithms of a curve target.

Producer/setup/relation/timing source is unchanged from 94e243dcc; replay no longer calls producer Elimination. Inputs and producer output remain v1; replay certificate is pkm.toy-dlp-replay/v2. Shared setup and relation encoding remain explicit. The existing separate ordinary affine reference checks point equations and final kP=Q. Rust parsing/arithmetic is not formally verified.

The public cold-control fixture is E/F17: y²=x³+2x+2, full group order 19, P=(5,1), Q=(6,3). Scalar rows are over F19, not F17. The producer remains bounded to p<=257 and prime full-curve order; wider scalar-module unit tests do not expand curve support. No secret target/base logarithms enter its interface.

== Verified scope
Three standalone native tests pass, including all 729 two-by-three F3 matrices, exhaustive vector membership, row permutations, near32-bit residues and forged certificate refusals. Strict scoped clippy passes. All five new Lean contracts build; fresh project audit passes 18 declarations with zero trusted exceptions. Final official module kernel replay passes against imports, not --fresh. Earlier failed elaboration, lint failure and 16-declaration audit are retained separately. Integrated example/library validation is running from source commit faade165a; no result is claimed before termination.

#pagebreak()
#image("scalar-certificates.svg", width: 100%)

== Formal scope and accounting
Mathlib's cyclic isomorphism turns actual point relations into scalar rows under exact cardinality and generation hypotheses. Prime exact cardinality and a nonzero generator discharge generation. The recovery theorem distributes a certified original-row combination; the ambiguity theorem uses a target-normalized kernel. Only propext, Classical.choice and Quot.sound occur. Exact statements and proof obligations are in proof-map.md; Rust/Mathlib curve representation, polynomial correspondence and complete solving/extraction-cost bounds remain open.

Replay work is outside the producer diagnostic cold clock and is not silently priced as free. Existing residual work and startup/serialization gaps remain. Comparison admission stays false. No timing comparison was performed, so no benchmark or performance-graph value is added.

== Sources and blockers
PKM 2016 Algorithm 1 and §3.2 already include permutation symmetry, relation dependence and linear algebra; primary source: people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf. Mathlib revision d13f23b723b8a846827a245b89c10fc7d3f11612 supplies zmodAddEquivOfGenerator and the prime-cardinality subgroup dichotomy. Local references: PKM_GOALS.md, tower review §3.3–3.7, existing linalg.rs and ThreeLeads row contracts. This is implementation/certification progress with standard algebra, not a new cheap relation algorithm or global novelty claim.

No lab/artifact identity or source-bound qualified native runtime is configured. The unchanged checkpoint gate refuses missing transport. No panel, lease, funding, IAM, quota or pool change occurred. The full-panel card remains blocked/unfunded at its historical pin. Existing canonical IC/curve/performance graphs remain unchanged: there is no admitted numerical result or new isogeny edge.

Evidence: continuation-20261030/validation/proof-final-receipt.json and exact source bindings; final native results must be added separately. Original inputs and receipts stay immutable. Reviewed merge 94e against both parents and restored 91 lost log paragraphs in 17 complete original blocks. Software merge does not complete any scientific goal.
