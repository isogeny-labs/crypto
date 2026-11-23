import Mathlib
import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point

namespace XedniReductionLeanBridge

local instance : Fact (Nat.Prime 47) := ⟨by norm_num⟩

def FixedCurve : WeierstrassCurve.Affine ℚ := ⟨0, -47, 0, 751, -3997⟩

def ReducedCurve : WeierstrassCurve.Affine (ZMod 47) := ⟨0, 0, 0, -1, -2⟩

theorem rational_Q_on_curve : FixedCurve.Nonsingular 17 10 := by
  rw [FixedCurve.nonsingular_iff']
  constructor
  · exact (FixedCurve.equation_iff 17 10).2 (by norm_num [FixedCurve])
  · right
    norm_num [FixedCurve]

theorem reduced_target_on_curve : ReducedCurve.Nonsingular 10 1 := by
  rw [ReducedCurve.nonsingular_iff']
  constructor
  · exact (ReducedCurve.equation_iff 10 1).2 (by decide)
  · right
    decide

private theorem cube_denominator_numerator (a : ℤ) (b : ℕ) (x y : ℚ)
    (hb : 0 < b)
    (hx : x = (a : ℚ) / (b : ℚ) ^ 2)
    (hcurve : y ^ 2 = x ^ 3 - 47 * x ^ 2 + 751 * x - 3997) :
    ∃ c : ℤ, y * (b : ℚ) ^ 3 = c := by
  have hbq : (b : ℚ) ≠ 0 := by exact_mod_cast (Nat.ne_of_gt hb)
  have hsq : (y * (b : ℚ) ^ 3) ^ 2 =
      ((a ^ 3 - 47 * a ^ 2 * (b : ℤ) ^ 2 +
        751 * a * (b : ℤ) ^ 4 - 3997 * (b : ℤ) ^ 6 : ℤ) : ℚ) := by
    calc
      _ = y ^ 2 * (b : ℚ) ^ 6 := by ring
      _ = _ := by
        rw [hcurve, hx]
        push_cast
        field_simp [hbq]
  have hint : IsIntegral ℤ ((y * (b : ℚ) ^ 3) ^ 2) := by
    rw [hsq]
    exact isIntegral_intCast _
  obtain ⟨c, hc⟩ :=
    IsIntegrallyClosed.exists_algebraMap_eq_of_isIntegral_pow
      (R := ℤ) (K := ℚ) (x := y * (b : ℚ) ^ 3) (by omega : 0 < 2) hint
  exact ⟨c, hc.symm⟩

theorem integral_model_equation (a c : ℤ) (b : ℕ) (x y : ℚ)
    (hb : 0 < b)
    (hx : x = (a : ℚ) / (b : ℚ) ^ 2)
    (hy : y * (b : ℚ) ^ 3 = c)
    (hcurve : y ^ 2 = x ^ 3 - 47 * x ^ 2 + 751 * x - 3997) :
    c ^ 2 = a ^ 3 - 47 * a ^ 2 * (b : ℤ) ^ 2 +
      751 * a * (b : ℤ) ^ 4 - 3997 * (b : ℤ) ^ 6 := by
  have hbq : (b : ℚ) ≠ 0 := by exact_mod_cast (Nat.ne_of_gt hb)
  have hsq : (y * (b : ℚ) ^ 3) ^ 2 =
      ((a ^ 3 - 47 * a ^ 2 * (b : ℤ) ^ 2 +
        751 * a * (b : ℤ) ^ 4 - 3997 * (b : ℤ) ^ 6 : ℤ) : ℚ) := by
    calc
      _ = y ^ 2 * (b : ℚ) ^ 6 := by ring
      _ = _ := by
        rw [hcurve, hx]
        push_cast
        field_simp [hbq]
  rw [hy] at hsq
  exact_mod_cast hsq

def reducedX (a : ℤ) (b : ℕ) : ZMod 47 :=
  (a : ZMod 47) / (b : ZMod 47) ^ 2

def reducedY (c : ℤ) (b : ℕ) : ZMod 47 :=
  (c : ZMod 47) / (b : ZMod 47) ^ 3

theorem reduced_equation (a c : ℤ) (b : ℕ)
    (hb : (b : ZMod 47) ≠ 0)
    (hmodel : c ^ 2 = a ^ 3 - 47 * a ^ 2 * (b : ℤ) ^ 2 +
      751 * a * (b : ℤ) ^ 4 - 3997 * (b : ℤ) ^ 6) :
    (reducedY c b) ^ 2 = (reducedX a b) ^ 3 - reducedX a b - 2 := by
  have hcast := congrArg (fun z : ℤ => (z : ZMod 47)) hmodel
  push_cast at hcast
  have h47 : (47 : ZMod 47) = 0 := by decide
  have h751 : (751 : ZMod 47) = -1 := by decide
  have h3997 : (3997 : ZMod 47) = 2 := by decide
  rw [h47, h751, h3997] at hcast
  dsimp [reducedX, reducedY]
  field_simp [hb]
  linear_combination hcast

theorem reduced_discriminant_ne_zero : ReducedCurve.Δ ≠ 0 := by
  decide

theorem reduced_point_nonsingular (a c : ℤ) (b : ℕ)
    (hb : (b : ZMod 47) ≠ 0)
    (hmodel : c ^ 2 = a ^ 3 - 47 * a ^ 2 * (b : ℤ) ^ 2 +
      751 * a * (b : ℤ) ^ 4 - 3997 * (b : ℤ) ^ 6) :
    ReducedCurve.Nonsingular (reducedX a b) (reducedY c b) := by
  apply (ReducedCurve.equation_iff_nonsingular_of_Δ_ne_zero
    reduced_discriminant_ne_zero).mp
  apply (ReducedCurve.equation_iff _ _).2
  have heq := reduced_equation a c b hb hmodel
  simpa [ReducedCurve, sub_eq_add_neg] using heq

theorem target_coordinate_congruences (a c : ℤ) (b : ℕ)
    (hb : (b : ZMod 47) ≠ 0)
    (hx : reducedX a b = 10)
    (hy : reducedY c b = 1) :
    (a : ZMod 47) = 10 * (b : ZMod 47) ^ 2 ∧
    (c : ZMod 47) = (b : ZMod 47) ^ 3 := by
  constructor
  · dsimp [reducedX] at hx
    exact (div_eq_iff (pow_ne_zero 2 hb)).mp hx
  · dsimp [reducedY] at hy
    have h := (div_eq_iff (pow_ne_zero 3 hb)).mp hy
    simpa using h

theorem rational_point_reduces (a : ℤ) (b : ℕ) (x y : ℚ)
    (hP : FixedCurve.Nonsingular x y)
    (hb : 0 < b) (hunit : (b : ZMod 47) ≠ 0)
    (hx : x = (a : ℚ) / (b : ℚ) ^ 2) :
    ∃ c : ℤ, y * (b : ℚ) ^ 3 = c ∧
      ReducedCurve.Nonsingular (reducedX a b) (reducedY c b) := by
  have hcurve : y ^ 2 = x ^ 3 - 47 * x ^ 2 + 751 * x - 3997 := by
    have heq := (FixedCurve.equation_iff x y).mp hP.1
    simpa [FixedCurve, sub_eq_add_neg] using heq
  obtain ⟨c, hc⟩ := cube_denominator_numerator a b x y hb hx hcurve
  refine ⟨c, hc, ?_⟩
  exact reduced_point_nonsingular a c b hunit
    (integral_model_equation a c b x y hb hx hc hcurve)

#print axioms cube_denominator_numerator
#print axioms rational_Q_on_curve
#print axioms reduced_target_on_curve
#print axioms integral_model_equation
#print axioms reduced_equation
#print axioms reduced_discriminant_ne_zero
#print axioms reduced_point_nonsingular
#print axioms target_coordinate_congruences
#print axioms rational_point_reduces

end XedniReductionLeanBridge
