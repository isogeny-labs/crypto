module
public import ConstantDivisors
public import Mathlib.FieldTheory.IsAlgClosed.Basic

@[expose] public section
namespace GeometricPrincipalDivisorAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K]

theorem fiber_coefficient_two (W : WeierstrassCurve.Affine K) (x : K) :
    (fiberPolynomial W x).coeff 2 = 1 := by
  simp only [fiberPolynomial, coeff_sub, coeff_add, coeff_C_mul, coeff_X_pow, coeff_X, coeff_C]
  norm_num

theorem fiber_degree_nonzero (W : WeierstrassCurve.Affine K) (x : K) :
    (fiberPolynomial W x).degree ≠ 0 := by
  intro hd
  have hz := coeff_eq_zero_of_degree_lt (p := fiberPolynomial W x) (n := 2)
    (by rw [hd]; norm_num)
  rw [fiber_coefficient_two] at hz
  exact one_ne_zero hz

/-- Actual points over every x-coordinate, with nonsingularity derived from the actual discriminant. -/
theorem geometric_fiber [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (x : K) : ∃ y, W.Nonsingular x y := by
  obtain ⟨y, hy⟩ := IsAlgClosed.exists_root (fiberPolynomial W x) (fiber_degree_nonzero W x)
  exact ⟨y, (equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp ((equation_fiber_root W x y).mpr hy)⟩

#print axioms geometric_fiber
end
end GeometricPrincipalDivisorAudit
