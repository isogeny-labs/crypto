module

public import CubePermutation
public import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point

@[expose] public section

namespace CubicPermutationCountAudit
noncomputable section
variable {K : Type*} [Field K]

def curve (b : K) : WeierstrassCurve.Affine K := ⟨0, 0, 0, 0, b⟩

theorem curve_delta (b : K) : (curve b).Δ = -(2 ^ 4 * 3 ^ 3 * b ^ 2) := by
  simp only [curve, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
    WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]
  ring

theorem curve_nonsingular (h2 : (2 : K) ≠ 0) (h3 : (3 : K) ≠ 0)
    (b : K) (hb : b ≠ 0) : (curve b).Δ ≠ 0 := by
  rw [curve_delta]
  exact neg_ne_zero.mpr (mul_ne_zero
    (mul_ne_zero (pow_ne_zero 4 h2) (pow_ne_zero 3 h3)) (pow_ne_zero 2 hb))

theorem curve_equation (b x y : K) :
    (curve b).Equation x y ↔ y ^ 2 = x ^ 3 + b := by
  rw [WeierstrassCurve.Affine.equation_iff]
  simp [curve]

variable [Fintype K]

noncomputable def coordinatesEquiv (hq : Fintype.card K % 3 = 2)
    (b : K) (hΔ : (curve b).Δ ≠ 0) :
    {xy : K × K // (curve b).Nonsingular xy.1 xy.2} ≃ K where
  toFun p := p.val.2
  invFun y := ⟨(cubeRoot (y ^ 2 - b), y),
    ((curve b).equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp (by
      rw [curve_equation, cube_cubeRoot hq]
      ring)⟩
  left_inv p := by
    apply Subtype.ext
    apply Prod.ext
    · have he := (curve_equation b p.val.1 p.val.2).mp
        (((curve b).equation_iff_nonsingular_of_Δ_ne_zero hΔ).mpr p.property)
      have hh : p.val.2 ^ 2 - b = p.val.1 ^ 3 := by rw [he]; ring
      change cubeRoot (p.val.2 ^ 2 - b) = p.val.1
      rw [hh, cubeRoot_cube hq]
    · rfl
  right_inv y := rfl

noncomputable def actualPointEquiv (hq : Fintype.card K % 3 = 2)
    (b : K) (hΔ : (curve b).Δ ≠ 0) : (curve b).Point ≃ Option K :=
  (curve b).nonsingularPointEquiv.trans (coordinatesEquiv hq b hΔ).optionCongr

theorem actual_point_count (hq : Fintype.card K % 3 = 2)
    (h2 : (2 : K) ≠ 0) (h3 : (3 : K) ≠ 0) (b : K) (hb : b ≠ 0) :
    Nat.card (curve b).Point = Fintype.card K + 1 := by
  rw [Nat.card_congr (actualPointEquiv hq b (curve_nonsingular h2 h3 b hb)),
    Nat.card_eq_fintype_card, Fintype.card_option]

#print axioms curve_nonsingular
#print axioms actual_point_count
end
end CubicPermutationCountAudit
