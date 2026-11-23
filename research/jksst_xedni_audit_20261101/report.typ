#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 9.25pt)
#set heading(numbering: "1.")

= JKSST xedni analysis: skeptical Lean audit

#text(fill: rgb("52637e"))[Kernel-checked formal audit, 2026-11-01]

== Result

The Lean suite proves finite packing, cyclic extraction, exact event counts,
and formal replacements for probabilistic and runtime steps needed to make
the JKSST xedni analysis precise. It also gives counterexamples to several
overbroad readings. The
paper's final conclusion remains conditional on a family-uniform selected-lift
height/discriminant comparison, Lang or a direct height gap, good reduction
and subgroup correspondence, useful-relation selection, a bounded cofactor,
and a repeated-trial hazard theorem.

#figure(
  image("claim-chain.svg", width: 100%),
  caption: [Claim-chain status. Green boxes are formal implications; amber
  boxes are explicit arithmetic premises; red boxes are missing algorithm
  interfaces; purple records kernel-checked counterexamples.],
)

== Source correspondence

JKSST Theorem 4.1 starts "under certain plausible assumptions." Lemma 4.1
assumes `log |D| >= C1 max h-hat(Pi)` and Lang's conjecture. Its finite-field
argument calls `(2 C2)^r / p` an approximate probability. The audit replaces
that step with exact finite-field cardinalities and keeps subgroup order `q`
distinct from base-field prime `p`.

The actual canonical-height construction is tied to Mathlib's rational
Weierstrass point group. Its all-multiples counterexample does not claim that
xedni's selected lifts contain those multiples. The formal result therefore
corrects an unconditional reading without asserting that xedni succeeds.

#pagebreak()

== Decisive results

- Exact coefficient cube: `(2 C + 1)^r`, with the zero vector removed.
- Same rank, minimum, and radius can give different exact lattice-ball counts.
- Finite repair: `#S <= T (1 + 2R/delta)^k` with every hypothesis visible.
- Fixed ordinary relation probability: `1/q`; fixed useful relation:
  `(q-1)/q^2`.
- A bounded relation does not automatically select a useful denominator.
- Actual exact-order curve-point encoding supports correct log extraction.
- Hasse plus `#E=Hq` yields a conditional `p <= 4Hq`, not `p approximately q`.
- A uniform finite-horizon hazard recurrence yields a literal truncated
  `Omega(p)` lower bound; a one-shot event bound is not that recurrence.

== Trust and limits

The project builds under Lean/Mathlib 4.34.1. The axiom ledger reports only
`propext`, `Classical.choice`, and `Quot.sound`; there are no project axioms,
`sorry`, `admit`, or `native_decide` proofs. Passing this gate certifies the
formal declarations, not the still-missing correspondence to the lift
selection and adaptive algorithm.

Primary source: M. J. Jacobson, N. Koblitz, J. H. Silverman, A. Stein, and
E. Teske, "Analysis of the Xedni Calculus Attack," DCC 20 (2000), 41--64,
DOI 10.1023/A:1008312401197. Full source and theorem-level mapping are in
`README.md` and `PROOF_MAP.md` beside this report.

== Open end-to-end obligations

- Specify the lift-selection algorithm and prove a family-uniform comparison
  with a global minimal discriminant.
- Prove Lang for that family, or replace it with an unconditional direct
  canonical-height gap.
- Instantiate finite generation, rank, separation, torsion fibers, and the
  exact finite packing hypotheses for the selected lifts.
- Connect good reduction to the sampled prime-order subgroup and prove the
  sampling correspondence.
- Show every algorithmic success selects a bounded scalar relation and, for
  the tighter event, a nonzero denominator.
- Prove a uniform cofactor bound before replacing subgroup order `q` by
  base-field prime `p`.
- Derive the history-uniform conditional-hazard recurrence for the adaptive
  algorithm; a one-shot relation rate does not supply it.
- If the cubic dimension step is used, prove full row rank for the actual ten
  evaluation conditions.
