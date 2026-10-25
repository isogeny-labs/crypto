# Prior work and provenance

This is implementation and a bounded diagnostic for three proposed leads,
not an exhaustive novelty search or a fastest-implementation comparison.

- The repository's earlier exact finite-base study derives two-torsion
  quotient coordinates, exceptional fibres and support/lifting conditions:
  [EXACT_FACTOR_BASE_RESULTS.md](../sat_factor_base_review_20260908/continuation-01/EXACT_FACTOR_BASE_RESULTS.md).
  Its four-orbit membership predicate also explicitly contrasts a product
  equation with a guarded selector formulation and leaves coupled solver
  cost unmeasured. Its old measurements and sources remain immutable.
- The completed [chart-complement census](../factor_base_chart_complements_20261019/README.md)
  supplies the exact input point sets and the B027+B033 candidate. Its
  finite support was not evidence of successful F4 cost. This study adds
  actual bounded solver calls; it does not rewrite that census.
- Faugère, Huot, Joux, Renault and Vitse, *Symmetrized Summation
  Polynomials: Using Small Order Torsion Points to Speed Up Elliptic Curve
  Index Calculus*, EUROCRYPT2014,
  https://www.iacr.org/archive/eurocrypt2014/84410158/84410158.pdf .
  Related primary literature for torsion symmetries, cited through the prior
  report. The earlier direct PDF request returned403; this implementation
  does not claim a fresh full-text replication or use its timing as a
  matched external reference.
- Explicit-Formulas Database binary Weierstrass formulas:
  https://www.hyperelliptic.org/EFD/g12o/auto-shortw.html . The earlier
  quotient report supplies the algebraic derivation and citations; this
  run checks the map and returned sums with native arithmetic.

The previous quasi-subfield search
`research/ic_next10_20261014/08-quasi-subfield/README.md` already records
negative results for its specified tested classes and a trace-rejection
explanation. This study does not recast cheap rejection as cheap successful
decomposition or repeat that search under a new name.

For future fastest-solver claims the named external references remain
Magma F4/F5, msolve/FGb and applicable Singular engines on identical
exported inputs and the same host. None is installed here at setup;
external comparison is not run and no fastest claim is admitted.
