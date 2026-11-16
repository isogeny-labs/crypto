module
public import CyclotomicDegree

@[expose] public section
namespace PrimeTraceWeakBoundAudit
noncomputable section
open scoped BigOperators

theorem squareFiberCard_eq_filter {K : Type*} [Field K] [Fintype K] [DecidableEq K] (z : K) :
    squareFiberCard z = (Finset.univ.filter (fun y : K => y ^ 2 = z)).card := by
  simp only [squareFiberCard, SquareFiber, Nat.card_eq_fintype_card, Fintype.card_subtype]

local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : Fact (Nat.Prime 7) := ⟨by decide⟩

def lowerThree : WeierstrassCurve.Affine (ZMod 3) := ⟨0, 0, 0, -1, -1⟩
def upperThree : WeierstrassCurve.Affine (ZMod 3) := ⟨0, 0, 0, -1, 1⟩
def twoControl : WeierstrassCurve.Affine (ZMod 5) := ⟨0, 0, 0, 0, 1⟩
def eightControl : WeierstrassCurve.Affine (ZMod 7) := ⟨0, 0, 0, 1, 0⟩

theorem lower_three_delta : lowerThree.Δ ≠ 0 := by decide +kernel
theorem upper_three_delta : upperThree.Δ ≠ 0 := by decide +kernel
theorem two_control_delta : twoControl.Δ ≠ 0 := by decide +kernel
theorem eight_control_delta : eightControl.Δ ≠ 0 := by decide +kernel

/-- These known small-field examples are credited to the earlier char3/count
cohorts; here they control deletion of the new q>3 hypothesis. -/
theorem lower_three_count : Nat.card lowerThree.Point = 1 := by
  rw [actual_point_fiber_count lowerThree lower_three_delta (by decide +kernel)]
  simp_rw [squareFiberCard_eq_filter]
  simp only [completedCubic, Polynomial.eval_add, Polynomial.eval_mul, Polynomial.eval_pow,
    Polynomial.eval_C, Polynomial.eval_X]
  decide +kernel

theorem upper_three_count : Nat.card upperThree.Point = 7 := by
  rw [actual_point_fiber_count upperThree upper_three_delta (by decide +kernel)]
  simp_rw [squareFiberCard_eq_filter]
  simp only [completedCubic, Polynomial.eval_add, Polynomial.eval_mul, Polynomial.eval_pow,
    Polynomial.eval_C, Polynomial.eval_X]
  decide +kernel

theorem two_control_count : Nat.card twoControl.Point = 6 := by
  rw [actual_point_fiber_count twoControl two_control_delta (by decide +kernel)]
  simp_rw [squareFiberCard_eq_filter]
  simp only [completedCubic, Polynomial.eval_add, Polynomial.eval_mul, Polynomial.eval_pow,
    Polynomial.eval_C, Polynomial.eval_X]
  decide +kernel

theorem eight_control_count : Nat.card eightControl.Point = 8 := by
  rw [actual_point_fiber_count eightControl eight_control_delta (by decide +kernel)]
  simp_rw [squareFiberCard_eq_filter]
  simp only [completedCubic, Polynomial.eval_add, Polynomial.eval_mul, Polynomial.eval_pow,
    Polynomial.eval_C, Polynomial.eval_X]
  decide +kernel

theorem three_source_controls :
    sourceSupersingular 3 lowerThree ∧ sourceSupersingular 3 upperThree ∧
    actualTrace lowerThree = 3 ∧ actualTrace upperThree = -3 := by
  norm_num [sourceSupersingular, actualTrace, ZMod.card, lower_three_count, upper_three_count]

theorem three_euler_constant :
    ∀ x : ZMod 3, (Polynomial.eval x (completedCubic upperThree)) ^ ((3 - 1) / 2) = 1 := by
  simp only [completedCubic, Polynomial.eval_add, Polynomial.eval_mul, Polynomial.eval_pow,
    Polynomial.eval_C, Polynomial.eval_X]
  decide +kernel

theorem two_control_source : sourceSupersingular 5 twoControl := by
  norm_num [sourceSupersingular, actualTrace, ZMod.card, two_control_count]

theorem eight_control_source : sourceSupersingular 7 eightControl := by
  norm_num [sourceSupersingular, actualTrace, ZMod.card, eight_control_count]

def twoPoint : twoControl.Point := .some 4 0 (by
  rw [← twoControl.equation_iff_nonsingular_of_Δ_ne_zero two_control_delta,
    WeierstrassCurve.Affine.equation_iff]
  decide +kernel)

theorem two_point_order : addOrderOf twoPoint = 2 := by
  apply addOrderOf_eq_prime
  · rw [two_nsmul]
    exact WeierstrassCurve.Affine.Point.add_self_of_Y_eq (by decide +kernel)
  · decide +kernel

/-- Credited actual composite-order point from CompositeTorsion; the new
degree control below uses the actual roots-of-unity field. -/
def eightPoint : eightControl.Point := .some 3 3 (by
  rw [← eightControl.equation_iff_nonsingular_of_Δ_ne_zero eight_control_delta,
    WeierstrassCurve.Affine.equation_iff]
  decide +kernel)

theorem eight_point_order : addOrderOf eightPoint = 8 := by
  exact addOrderOf_eq_prime_pow (p := 2) (n := 2)
    (by decide +kernel) (by decide +kernel)

theorem eight_control_actual_degree : Module.finrank (ZMod 7) (rootsField 7 8) = 2 :=
  roots_field_degree_two 7 8 (by decide) (by decide)

theorem two_control_actual_degree : Module.finrank (ZMod 5) (rootsField 5 2) = 1 :=
  roots_two_field_degree 5

theorem literal_prime_field_degree_two_false :
    ¬ Module.finrank (ZMod 5) (rootsField 5 (addOrderOf twoPoint)) = 2 := by
  rw [two_point_order, two_control_actual_degree]
  decide

#print axioms three_source_controls
#print axioms two_point_order
#print axioms eight_point_order
#print axioms literal_prime_field_degree_two_false
end
end PrimeTraceWeakBoundAudit
