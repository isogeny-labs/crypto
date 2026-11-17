# Proof map

This table maps the audit's central readable claims to exact Lean
declarations. "Conditional" means every listed bridge remains a theorem
hypothesis; it does not mean the xedni algorithm satisfies it.

| Claim | Lean declaration | Status and boundary |
|---|---|---|
| Exact integer coefficient cube count | `JKSSTHeightCount.card_integerBox` | Proved exactly |
| Exact nonzero coefficient count | `JKSSTCoefficientBox.card_nonzeroCoefficientBox` | Proved exactly |
| Coarse lattice data do not determine exact Euclidean count | `JKSSTCountNonDeterminacy.same_euclidean_minimum_different_radius_one_count` | Explicit counterexample |
| Coarse lattice data do not determine exact sup-norm count | `JKSSTCountNonDeterminacy.same_minimum_different_radius_one_count` | Explicit counterexample |
| Finite packing with torsion/fiber factor | `JKSSTPacking.card_le_fiberBound_mul_nat_pow` | Proved from displayed radius/separation hypotheses |
| Deleting an asymptotic error does not give a finite upper bound | `JKSSTAsymptotic.bigO_remainder_does_not_imply_main_upper_bound` | Explicit counterexample |
| Certified height ratio gives bounded group relation | `JKSSTConditionalHeightCore.bounded_relation_of_certified_height_ratio` | Conditional; abstract Mordell--Weil instantiation remains |
| Actual canonical/Neron--Tate square law | `WeierstrassCurve.Affine.Point.neronTateHeight_nsmul` | Proved on Mathlib point group |
| Height zero iff torsion | `WeierstrassCurve.Affine.Point.neronTateHeight_eq_zero_iff_mem_torsion` | Proved under Northcott |
| No all-multiples bound by fixed actual model discriminant | `JKSSTActualHeightCounterexample.no_uniform_bound_by_actual_model_discriminant` | Counterexample schema given a nontorsion rational point; not a selected-lift theorem |
| Quadraticity alone gives no Lang constant | `JKSSTLang.no_uniform_lower_bound_from_quadraticity` | Explicit scaled-lattice countermodel |
| Model discriminant has weight twelve | `JKSSTWeierstrassScaling.scaled_shortModel_discriminant` | Proved with bundled Mathlib variable change |
| Model-discriminant log is unbounded under rescaling | `JKSSTModelDiscriminant.scaled_logAbs_discriminant_unbounded` | Proved for nonzero short discriminant |
| Ten independent cubic conditions force `r<=9` | `JKSSTConcreteCubic.fullRowRank_points_on_nonzero_cubic_le_nine` | Conditional on full row rank; not arbitrary points |
| Exact-order point gives injective scalar encoding | `JKSSTCyclicECDLP.orderScalarMap_injective` | Proved for displayed `addOrderOf P=q` |
| Useful group relation extracts exact logarithm | `JKSSTCyclicECDLP.extract_log_of_group_relation` | Proved for prime `q` and nonzero denominator |
| Actual finite-curve specialization | `JKSSTEllipticCyclic.extract_log_on_curve` | Proved; rational-lift reduction remains missing |
| Fixed useful relation event cardinality | `JKSSTProbability.card_usefulEvent` | Exact `(q-1)/q^2` event probability |
| Fixed ordinary scalar relation bound | `JKSSTSuccessComposition.ordinary_success_bound_of_group_relation_certificates` | Conditional on scalar-reduction certificate; no usefulness required |
| Useful success-union bound | `JKSSTSuccessComposition.exact_success_bound_of_group_relation_certificates` | Requires an additional useful-selection certificate |
| Usefulness is not automatic | `JKSSTSuccessComposition.no_useful_reduction_at_zero_sample` | Explicit zero-sample counterexample |
| Composite coefficient need not give `1/q` | `JKSSTCompositeModulus.relation_probability_is_one_half` | Explicit `ZMod 4` counterexample |
| Hasse/cofactor numeric `/p` bridge | `JKSSTHasseCofactor.base_le_four_cofactor_mul_subgroup` | Hasse and `N=H*q` are explicit premises |
| Probability bound composed with Hasse | `JKSSTHasseCofactor.bounded_coefficient_success_from_hasse` | Uses `N=H*#F` by substitution; subgroup interpretation remains separate |
| No uniform all-`p` inverse bound for fixed positive probability | `JKSSTProbability.fixed_positive_probability_not_bigO_inverse` | Exact quantified contradiction |
| Fixed positive probability is not `O(1/p)` | `JKSSTProbability.fixed_positive_probability_not_bigO_inverse_nat` | Literal Mathlib `atTop` theorem |
| Finite-horizon hazard gives linear wait | `JKSSTExpectedTrials.inverse_order_hazard_gives_linear_wait` | Conditional recurrence through the stated horizon |
| Literal truncated-runtime `Omega(p)` | `JKSSTOmegaRuntime.inverse_order_hazard_isOmega` | Fixed `K`, normalized nonnegative mass, positive waiting time, and explicit uniform recurrence; not actual adaptive xedni yet |

## Semantic-fidelity checkpoints

- The project does not call Lang's conjecture proved or disproved.
- The actual height counterexample assumes a supplied nontorsion rational
  point; it does not exhibit a concrete positive-rank curve.
- The exact individual-event probability and the union **upper bound** are
  distinct; overlapping relation events are not asserted disjoint.
- `q` is the scalar subgroup order. Replacing it by `p` is never implicit.
- The `Omega(p)` theorem concerns the explicitly defined truncated finite
  process. It now assumes a probability mass function and requires a
  history-uniform conditional recurrence, not merely a one-shot rate.
- Passing the Lean axiom gate proves the formal statements, not their missing
  correspondence to the xedni implementation.
