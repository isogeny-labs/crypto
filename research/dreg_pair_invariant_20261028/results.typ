#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= The summand-swap invariant does not refute below degree 4 at ℓ = 6; degree 4 at ℓ = 7 is unresolved

*Study* `research/dreg_pair_invariant_20261028/`, pre-registered. *Class:*
stage diagnostic. *Verdict:* H_none in both cells. At (22, 6) that is measured. At (21, 7)
it follows the registered convention for an unmeasured degree 4, and that
cell is unresolved.

== Question

At m = 3 every cubic monomial of the chained descent comes from `x₁x₂e₁`.
In the swap invariants `s = x₁+x₂` and `p = x₁x₂`, the first link becomes
the quadratic `s²e₁² + p e₁ + p² + b`.

Does adding `p` as unknowns let the MutantXL closure refute at a lower
degree? Is any gain due to the invariant, or only to quadratisation?
Quadratisation alone is tested by a matched control, `r = x₁e₁`.

== Result

#figure(image("reports/run-20261028T0551Z/invariant.svg", width: 100%))

#table(columns: 4,
  [*cell*], [*B (plain)*], [*Q (invariant)*], [*R (control)*],
  [(22, 6), 4 draws], [refutes at 4], [> 3], [> 3],
  [(21, 7), 4 draws], [> 4], [> 3], [> 3],
)

- Q's degree 4 is unmeasured. Its dense matrix needs 523,686–637,393
  columns, against B's 102,091 and 124,314.
- So at (22, 6) the invariant arm refutes no earlier than B, at 5.1–5.5
  times the columns.
- At (21, 7), B does not refute at 4 (C(42, ≤ 5) = 974,982 columns at degree
  5). If Q refuted at 4, with 637,393 columns, it would be lower and
  smaller. That is not measured, so this cell is open.
- At degree 3, quadratisation alone (R) does not help either.
- Controls valid: exact identity check, B reproduces its records, and no
  satisfiable draw is refuted.

== Reading

- *Where the plain chain refutes at 4 (ℓ = 6), the pair invariant does not
  help* (measured).
- *Where it does not refute at 4 (ℓ = 7), it is unresolved:* degree 4 of Q
  at (21, 7) decides it and needs a large-memory host.
- At m = 4 the second cubic link has no summand symmetry.
- *Unregistered pointer:* at (22, 6), Q closes more of its degree-3 matrix
  (0.41–0.44) than B (0.37–0.38) or R (0.28), but not enough to refute.
- *No row moves and none beats rho.* There is no improvement over the
  baseline yet.
- *Untried successors:* the invariants as a replacement for the summand
  coordinates, so that N falls; the orbit-aware split rule.
