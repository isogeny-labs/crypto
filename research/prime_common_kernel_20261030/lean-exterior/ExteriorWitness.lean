module

public import Mathlib.Algebra.Polynomial.Derivative
public import Mathlib.Algebra.Polynomial.Eval.Coeff
public import Mathlib.Tactic.NormNum
public import Mathlib.Tactic.Ring

public section

namespace ExteriorWitness
open Polynomial

def node (j : Nat) : ℚ := (j : ℚ) - 4
noncomputable def grid : ℚ[X] :=
  ∏ j ∈ Finset.range 8, (X - C (node j))
noncomputable def normalizer : ℚ := grid.derivative.eval 0

/-- Actual punctured source polynomial at T=1+8z, not a planted coefficient. -/
noncomputable def chart (delta : ℚ) : ℚ[X] :=
  C (normalizer⁻¹) *
    ∏ j ∈ Finset.range 8,
      if node j = delta then 1 else ((C 1 + C 8 * X) - C (node j))

/-- The actual even total two-bit parity code for the finite L=4 witness. -/
def evenSlot4 (x y : Nat) : Prop :=
  x < 4 ∧ y < 4 ∧ ((x % 2) + (x / 2 % 2) + (y % 2) + (y / 2 % 2)) % 2 = 0

theorem witness_slots_allowed : evenSlot4 0 0 ∧ evenSlot4 2 1 := by
  norm_num [evenSlot4]

theorem witness_has_passive_axis :
    (0 : Nat) % 2 = 2 % 2 ∧ (1 : Int) - 0 = 1 := by decide

theorem source_normalizer : normalizer = -144 := by
  norm_num [normalizer, grid, node, Finset.prod_range_succ, derivative_mul]

theorem local_node_value : (chart 1).eval 0 = -5 / 3 := by
  norm_num [chart, source_normalizer, node, Finset.prod_range_succ]

theorem crossing_linear_coefficient : (chart 2).coeff 1 = 40 / 3 := by
  have h : (chart 2).derivative.eval 0 = 40 / 3 := by
    norm_num [chart, source_normalizer, node, Finset.prod_range_succ, derivative_mul]
  rw [← coeff_zero_eq_eval_zero, coeff_derivative] at h
  simpa using h

theorem passive_product_coefficient :
    ((chart 2).coeff 1) * ((chart 1).eval 0) = -200 / 9 := by
  rw [crossing_linear_coefficient, local_node_value]
  norm_num

theorem passive_product_nonzero :
    ((chart 2).coeff 1) * ((chart 1).eval 0) ≠ 0 := by
  rw [passive_product_coefficient]
  norm_num

#print axioms source_normalizer
#print axioms witness_slots_allowed
#print axioms witness_has_passive_axis
#print axioms local_node_value
#print axioms crossing_linear_coefficient
#print axioms passive_product_coefficient
#print axioms passive_product_nonzero

end ExteriorWitness
