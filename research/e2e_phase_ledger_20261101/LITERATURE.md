# Literature gate

Reviewed before protocol freeze on 2026-11-01.

## Primary sources

- J.-C. Faugere, "A new efficient algorithm for computing Groebner bases
  (F4)," ISSAC 1999, DOI 10.1145/309831.309887. This fixes the meaning of F4
  as batched sparse linear algebra; it does not license using only an
  elimination counter as an end-to-end cost.
- J.-C. Faugere, "A new efficient algorithm for computing Groebner bases
  without reduction to zero (F5)," ISSAC 2002, DOI
  10.1145/780506.780516. This motivates measuring the F5 criterion's saved
  rows together with its symbolic and matrix costs.
- I. Semaev, "Summation polynomials and the discrete logarithm problem on
  elliptic curves," IACR ePrint 2004/031. This supplies the summation-
  polynomial relation framework and makes relation finding only one stage of
  an index-calculus attack.
- P. Gaudry, "Index calculus for abelian varieties of small dimension and the
  elliptic curve discrete logarithm problem," Journal of Symbolic
  Computation 44(12), 2009. This motivates separating factor-base,
  relation-collection, and linear-algebra costs.
- J.-C. Faugere, L. Perret, C. Petit, and G. Renault, "Improving the
  complexity of index calculus algorithms in elliptic curves over binary
  fields," EUROCRYPT 2012. This is the closest binary-field algebraic attack
  context and motivates reporting degree/matrix growth without treating a
  toy-field result as a 130-bit estimate.

## Repository evidence used only for calibration

- `research/f4_f5_f6_cross_family_20261011/`: existing F4/F5/F6 engine and
  native-unit measurements. These are in-sample engineering evidence, not
  confirmatory data for this study.
- `research/ecbench_e2e_frontier_20261031/`: current ecbench phase, work,
  pricing-band, isolation, and replay contracts.
- `research/nonlinear_factor_labels_20261002/`: the nonlinear-label line of
  inquiry that motivated testing algebraic backends, but not a novelty proof.

## Scope consequence

The literature supports the decomposition of the experiment and the choice
of baselines. It does not support the claim that a backend or hyperparameter
combination is unprecedented. This experiment can identify a new result
within the registered implementation and toy cell; any "never done before"
claim requires a separate, broader novelty search and should remain qualified.
