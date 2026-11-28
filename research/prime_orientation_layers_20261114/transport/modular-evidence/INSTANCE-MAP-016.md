# Nonscalar-origin actual polynomial instance map

1. Structure above: Mathlib Polynomial Q, evaluation ring hom, natDegree; actual OrientationAnchor completed family over grid and actual Ports=Fin(2^a).
2. Laws used as stated: completed_origin_diagonal/offdiagonal, delta_in_grid, zero_in_grid, completed_at_node, completed_degree; Polynomial.eval_add/eval_mul/eval_C, natDegree_add_le/natDegree_mul_C_le; Fin.ext.
3. Object: actual scalar D=completed grid u 0 0 m and entrywise W'=completed + D*C(Sxy-delta_xy), for arbitrary explicit rational matrix S. No private syntax object is primary.
4. Assembly probe130: actual family contracts with matrix S as parameter, derived from named actual laws plus ring equality; no favorable degree/origin/node witnesses.
5. Missing instance facts: actual completed diagonal independent of port; actual D vanishes at nonzero grid nodes; new origin/node/diagonal/degree statements. The generic interpolation and ring laws already exist; no generic law is rederived. Pairing, U units, crossing content and recovery are OUTSIDE this probe.
6. Inheritance: evaluation through eval ring laws; origin via old diagonal/offdiagonal; grid hypotheses via delta_in_grid/zero_in_grid; degree via old completed_degree then mul_C/add inequalities. Diagonal independence unfolds only actual guard/correction/definition because there is no named actual diagonal-independence lemma; consumer is exact diagonal cancellation. No changed computational trust or limits.

Statements are study instance obligations/endpoints, not a generic ECDLP theorem or novel generic polynomial law. [pcc:unverified]
