import PrimeRecovery
import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point

namespace SubgroupExtractionAudit

private instance : Fact (Nat.Prime 29) := ⟨by decide⟩
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩

abbrev Base := ZMod 29
def curve : WeierstrassCurve.Affine Base := ⟨0,0,0,0,1⟩
theorem curve_discriminant : curve.Δ ≠ 0 := by decide +kernel
theorem curve_equation (x y : Base) : curve.Equation x y ↔ y^2=x^3+1 := by
  simp [curve, WeierstrassCurve.Affine.equation_iff]

def P : curve.Point := .some 4 6 (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_discriminant).mp
  rw [curve_equation]
  decide +kernel)

theorem P_five : (5 : ℕ) • P = 0 := by decide +kernel
theorem P_nonzero : P ≠ 0 := by decide +kernel
theorem P_order : addOrderOf P = 5 := addOrderOf_eq_prime P_five P_nonzero

def EquationCode (xy : Base × Base) : Prop := xy.2^2=xy.1^3+1
instance : DecidablePred EquationCode := fun xy => inferInstanceAs (Decidable (xy.2^2=xy.1^3+1))
abbrev PointCode := Option {xy : Base × Base // EquationCode xy}

def pointEquivCode : curve.Point ≃ PointCode where
  toFun
    | 0 => none
    | .some x y h => some ⟨(x,y),(curve_equation x y).mp h.1⟩
  invFun
    | none => 0
    | some ⟨(x,y),h⟩ => .some x y
      ((WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_discriminant).mp
        ((curve_equation x y).mpr h))
  left_inv := by rintro (_ | _) <;> rfl
  right_inv := by rintro (_ | ⟨⟨x,y⟩,h⟩) <;> rfl

instance : Finite curve.Point := Finite.of_equiv PointCode pointEquivCode.symm
theorem ambient_card : Nat.card curve.Point = 30 := by
  rw [Nat.card_congr pointEquivCode]
  calc Nat.card PointCode = Fintype.card PointCode := Nat.card_eq_fintype_card
       _ = 30 := by decide +kernel

/-- Nonzero modulo the actual ambient order, but no inverse there. -/
theorem ambient_bad_beta : (5 : ZMod (Nat.card curve.Point)) ≠ 0 ∧
    ¬ IsUnit (5 : ZMod (Nat.card curve.Point)) := by
  rw [ambient_card]
  constructor
  · decide +kernel
  · simpa using (ZMod.isUnit_iff_coprime 5 30).not.mpr (by decide)

theorem bad_relation : (0 : ℕ) • P + (5 : ℕ) • P = 0 := by simpa using P_five

theorem naive_bad_recovery_fails :
    ((-(0 : ZMod 30)*(5 : ZMod 30)⁻¹).val : ℕ) • P ≠ P := by
  simpa using P_nonzero.symm

theorem universal_ambient_extraction_false :
    ¬ (∀ a b x : ℕ, a • P + b • (x • P) = 0 → (b : ZMod 30) ≠ 0 →
      ((-(a : ZMod 30)*(b : ZMod 30)⁻¹).val : ℕ) • P = x • P) := by
  intro h
  apply naive_bad_recovery_fails
  simpa using h 0 5 1 (by simpa using bad_relation) (by decide +kernel)

/-- Even a useful relation need not permit division in the ambient coefficient ring. -/
theorem useful_relation : (3 : ℕ) • P + (2 : ℕ) • P = 0 := by decide +kernel

theorem useful_beta_nonunit_ambient : ¬ IsUnit (2 : ZMod 30) := by
  simpa using (ZMod.isUnit_iff_coprime 2 30).not.mpr (by decide)

theorem useful_beta_nonzero_subgroup : (2 : ZMod 5) ≠ 0 := by decide +kernel

theorem correct_subgroup_recovery : (1 : ZMod 5) = -(3 : ZMod 5)/(2 : ZMod 5) := by
  apply recover_mod_prime 5 P P_order 3 2 1
  · simpa using useful_relation
  · exact useful_beta_nonzero_subgroup

theorem ambient_unit_control : IsUnit (1 : ZMod 30) ∧
    (-(29 : ZMod 30)*(1 : ZMod 30)⁻¹).val = 1 := by
  constructor
  · exact isUnit_one
  · decide +kernel

end SubgroupExtractionAudit
