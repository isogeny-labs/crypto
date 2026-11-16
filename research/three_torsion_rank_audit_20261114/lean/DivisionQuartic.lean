import Mathlib.AlgebraicGeometry.EllipticCurve.DivisionPolynomial.Degree
import Mathlib.FieldTheory.IsAlgClosed.Basic
import Mathlib.Tactic

namespace ThreeTorsionRankAudit
open Polynomial
noncomputable section
variable {K : Type*} [Field K]

def short (a b : K) : WeierstrassCurve.Affine K := ⟨0,0,0,a,b⟩
def psi (a b : K) : K[X] := (short a b).Ψ₃
def curveValue (a b x : K) : K := x^3+a*x+b

theorem curve_equation (a b x y : K) : (short a b).Equation x y ↔ y^2=curveValue a b x := by
  simp [short,curveValue,WeierstrassCurve.Affine.equation_iff]

theorem psi_formula (a b : K) :
    psi a b = 3*X^4+6*C a*X^2+12*C b*X-C (a^2) := by
  simp [psi,short,WeierstrassCurve.Ψ₃,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
    WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  norm_num only [map_ofNat]
  ring

theorem psi_eval (a b x : K) :
    (psi a b).eval x = 3*x^4+6*a*x^2+12*b*x-a^2 := by
  simp [psi_formula]

theorem psi_value_identity (a b x : K) :
    (psi a b).eval x = 12*x*curveValue a b x-(3*x^2+a)^2 := by
  rw [psi_eval]
  unfold curveValue
  ring

theorem psi_root_curve_value_nonzero (a b : K) (hΔ : (short a b).Δ ≠ 0)
    {x : K} (hx : (psi a b).IsRoot x) : curveValue a b x ≠ 0 := by
  intro hf
  have hv := psi_value_identity a b x
  have hx' : (psi a b).eval x = 0 := hx
  have hsq : (3*x^2+a)^2=0 := by rw [hx',hf] at hv; linear_combination hv
  have hd : 3*x^2+a=0 := sq_eq_zero_iff.mp hsq
  have heq : (short a b).Equation x 0 := (curve_equation a b x 0).mpr (by simp [hf])
  have hn := (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp heq
  have hs := (WeierstrassCurve.Affine.nonsingular_iff' x 0).mp hn
  simpa [short,hd] using hs.2

theorem psi_derivative (a b : K) :
    (psi a b).derivative = C 12*(X^3+C a*X+C b) := by
  rw [psi_formula]
  simp [Polynomial.derivative_pow]
  norm_num only [map_ofNat]
  ring

theorem psi_degree [NeZero (3 : K)] (a b : K) : (psi a b).natDegree=4 :=
  (short a b).natDegree_Ψ₃ (NeZero.ne _)

theorem psi_separable [IsAlgClosed K] [NeZero (2 : K)] [NeZero (3 : K)]
    (a b : K) (hΔ : (short a b).Δ ≠ 0) : (psi a b).Separable := by
  rw [Polynomial.separable_def,
    Polynomial.isCoprime_iff_aeval_ne_zero_of_isAlgClosed K K (psi a b) (psi a b).derivative]
  intro x
  change (psi a b).eval x ≠ 0 ∨ (psi a b).derivative.eval x ≠ 0
  by_cases hx : (psi a b).eval x = 0
  · right
    have hf := psi_root_curve_value_nonzero a b hΔ hx
    have h12 : (12 : K) ≠ 0 := by
      have he : (12 : K)=2^2*3 := by norm_num
      rw [he]
      exact mul_ne_zero (pow_ne_zero _ (NeZero.ne _)) (NeZero.ne _)
    rw [psi_derivative]
    simpa [curveValue] using mul_ne_zero h12 hf
  · exact Or.inl hx

end
end ThreeTorsionRankAudit
