import AnomalousPrime
import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point

namespace AnomalousCurveDataAudit
open AnomalousPrimeAudit
instance : Fact (Nat.Prime p) := ⟨p_prime⟩
abbrev F := ZMod p

def printedCurve : WeierstrassCurve.Affine F :=
  ⟨0,0,0,25706413842211054102700238164133538302169176474,
    203362936548826936673264444982866339953265530166⟩
def formulaCurve : WeierstrassCurve.Affine F :=
  ⟨0,0,0,425706413842211054102700238164133538302169176474,
    203362936548826936673264444982866339953265530166⟩

theorem printed_discriminant_ne_zero : printedCurve.Δ≠0 := by
  simp only [printedCurve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,
    WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  unfold F p
  reduce_mod_char; decide +kernel

theorem formula_discriminant_ne_zero : formulaCurve.Δ≠0 := by
  simp only [formulaCurve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,
    WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  unfold F p
  reduce_mod_char; decide +kernel

instance : printedCurve.IsElliptic :=
  ⟨isUnit_iff_ne_zero.mpr printed_discriminant_ne_zero⟩
instance : formulaCurve.IsElliptic :=
  ⟨isUnit_iff_ne_zero.mpr formula_discriminant_ne_zero⟩

theorem printed_j : printedCurve.j=
    (681693274333382938233427061391435351997384499942 : F) := by
  rw [WeierstrassCurve.j_eq]
  apply (inv_mul_eq_iff_eq_mul₀ printed_discriminant_ne_zero).mpr
  simp only [printedCurve,WeierstrassCurve.Δ,WeierstrassCurve.c₄,
    WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  unfold F p
  reduce_mod_char

theorem formula_j : formulaCurve.j=(-32768 : F) := by
  rw [WeierstrassCurve.j_eq]
  apply (inv_mul_eq_iff_eq_mul₀ formula_discriminant_ne_zero).mpr
  simp only [formulaCurve,WeierstrassCurve.Δ,WeierstrassCurve.c₄,
    WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  unfold F p
  reduce_mod_char

theorem printed_j_not_claimed : printedCurve.j≠(-32768 : F) := by
  rw [printed_j]
  unfold F p
  reduce_mod_char; decide +kernel

theorem printed_point_equation : printedCurve.Equation
    (25 : F) (37304648684346883938862473354554031475866783037 : F) := by
  rw [WeierstrassCurve.Affine.equation_iff]
  simp only [printedCurve]
  unfold F p
  reduce_mod_char

noncomputable def printedPoint : printedCurve.Point :=
  .some 25 37304648684346883938862473354554031475866783037
    ((WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      printed_discriminant_ne_zero).mp printed_point_equation)

#print axioms printed_j_not_claimed
#print axioms formula_j
end AnomalousCurveDataAudit
