import FixedFields

namespace BKGeneralFrobeniusAudit.SourceNineteen
noncomputable section
private instance : Fact (Nat.Prime 19) := ⟨by decide⟩

/-- Original BK1998 Remarks1 example; its values are checked, not assumed. -/
def curve : WeierstrassCurve.Affine (ZMod 19) := ⟨0,0,0,1,6⟩

theorem curve_nonsingular : curve.Δ ≠ 0 := by decide +kernel

theorem equation_iff (x y : ZMod 19) :
    curve.Equation x y ↔ y^2 = x^3 + x + 6 := by
  rw [WeierstrassCurve.Affine.equation_iff]
  simp [curve]

def generator : curve.Point := .some 0 5 (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_nonsingular).mp
  rw [equation_iff]
  decide +kernel)

theorem generator_controls : (18 : ℕ) • generator = 0 ∧
    (9 : ℕ) • generator ≠ 0 ∧ (6 : ℕ) • generator ≠ 0 := by
  decide +kernel

theorem generator_order : addOrderOf generator = 18 := by
  apply addOrderOf_eq_of_nsmul_and_div_prime_nsmul (by decide : 0 < 18)
    generator_controls.1
  intro p hp hd
  have hb : p ≤ 18 := Nat.le_of_dvd (by decide : 0 < 18) hd
  have hh : ∀ a : Fin 19, a.val.Prime → a.val ∣ 18 → a.val = 2 ∨ a.val = 3 := by
    decide +kernel
  obtain rfl | rfl := hh ⟨p,by omega⟩ hp hd
  · exact generator_controls.2.1
  · exact generator_controls.2.2

private def nonsingularDecidable (xy : ZMod 19 × ZMod 19) :
    Decidable (curve.Nonsingular xy.1 xy.2) :=
  decidable_of_iff (xy.2^2 = xy.1^3 + xy.1 + 6) (by
    rw [← WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_nonsingular]
    exact (equation_iff xy.1 xy.2).symm)

theorem rational_cardinality : Nat.card curve.Point = 18 := by
  rw [Nat.card_congr curve.nonsingularPointEquiv]
  change Nat.card (Option {xy : ZMod 19 × ZMod 19 // curve.Nonsingular xy.1 xy.2}) = 18
  let : DecidablePred (fun xy : ZMod 19 × ZMod 19 => curve.Nonsingular xy.1 xy.2) :=
    nonsingularDecidable
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem rational_cyclic : IsAddCyclic curve.Point := by
  let : Finite curve.Point := BKLinearAudit.actual_rational_point_finite _
  apply isAddCyclic_of_addOrderOf_eq_card generator
  rw [generator_order, rational_cardinality]

theorem rational_three_torsion_card : Nat.card (Torsion curve 3) = 3 := by
  let : DecidablePred (fun xy : ZMod 19 × ZMod 19 => curve.Nonsingular xy.1 xy.2) :=
    nonsingularDecidable
  let : Fintype curve.Point := Fintype.ofEquiv
    (Option {xy : ZMod 19 × ZMod 19 // curve.Nonsingular xy.1 xy.2})
    curve.nonsingularPointEquiv.symm
  let : Fintype (Torsion curve 3) := inferInstanceAs
    (Fintype {P : curve.Point // (3 : ℕ) • P = 0})
  rw [Nat.card_eq_fintype_card]
  decide +kernel

#print axioms generator_order
#print axioms rational_cardinality
#print axioms rational_cyclic
#print axioms rational_three_torsion_card
end
end BKGeneralFrobeniusAudit.SourceNineteen
