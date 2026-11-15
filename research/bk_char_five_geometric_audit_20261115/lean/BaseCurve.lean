module
public import FiveCoordinates
public import Mathlib.FieldTheory.Finite.Basic

@[expose] public section
namespace BKCharFiveGeometricAudit
noncomputable section
open WeierstrassCurve.Affine
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev F := ZMod 5
local instance : CommRing F := (inferInstance : Field F).toCommRing

theorem base_delta : (curve F).Δ ≠ 0 := by decide +kernel

def arithmeticBaseEquiv :
    {xy : F × F // (curve F).Nonsingular xy.1 xy.2} ≃
      {xy : F × F // xy.2 ^ 2 = xy.1 ^ 3 + 3 * xy.1 + 2} :=
  Equiv.subtypeEquivProp (by
    funext xy
    apply propext
    rw [← WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero base_delta]
    simp [curve, WeierstrassCurve.Affine.equation_iff])

theorem base_count : Nat.card (curve F).Point = 5 := by
  rw [Nat.card_congr (curve F).nonsingularPointEquiv]
  change Nat.card (Option {xy : F × F // (curve F).Nonsingular xy.1 xy.2}) = 5
  rw [Nat.card_congr arithmeticBaseEquiv.optionCongr, Nat.card_eq_fintype_card]
  decide +kernel

theorem all_base_points_five (P : (curve F).Point) : (5 : ℕ) • P = 0 := by
  simpa only [base_count] using (card_nsmul_eq_zero' (x := P))

def basePoint (i s : Bool) : (curve F).Point :=
  .some (if i then 2 else 1) (if s then -1 else 1)
    ((WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero base_delta).mp (by
      cases i <;> cases s <;> rw [WeierstrassCurve.Affine.equation_iff]
        <;> decide +kernel))

theorem base_point_nonzero (i s : Bool) : basePoint i s ≠ 0 := Point.some_ne_zero _

theorem base_point_order (i s : Bool) : addOrderOf (basePoint i s) = 5 := by
  have hd : addOrderOf (basePoint i s) ∣ 5 :=
    addOrderOf_dvd_iff_nsmul_eq_zero.mpr (all_base_points_five _)
  rcases (show Nat.Prime 5 by decide).eq_one_or_self_of_dvd _ hd with h | h
  · exact False.elim (base_point_nonzero i s (AddMonoid.addOrderOf_eq_one_iff.mp h))
  · exact h

#print axioms base_delta
#print axioms base_count
#print axioms all_base_points_five
#print axioms base_point_order
end
end BKCharFiveGeometricAudit
