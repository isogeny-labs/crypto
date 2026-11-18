module
public import HomographyBasics
public import Mathlib.FieldTheory.Finite.GaloisField
public import Mathlib.FieldTheory.Finite.Basic

@[expose] public section
namespace MomoseCubicHomographyAudit
noncomputable section
open Matrix

abbrev F9 := GaloisField 3 2
instance : Fintype F9 := Fintype.ofFinite _

theorem field9_card : Fintype.card F9 = 9 := by
  rw [← Nat.card_eq_fintype_card]
  exact (GaloisField.card 3 2 (by decide)).trans (by norm_num)

theorem field9_degree : Module.finrank (ZMod 3) F9 = 2 := GaloisField.finrank 3 (by decide)

theorem field9_square_two : ∃ a : F9, a^2 = 2 := by
  have hF : ringChar F9 ≠ 2 := by rw [ringChar.eq F9 3]; decide
  have h2 : (2 : F9) ≠ 0 := (CharP.cast_eq_zero_iff F9 3 2).not.mpr (by decide)
  have hs : IsSquare (2 : F9) := (FiniteField.isSquare_iff hF h2).mpr (by
    rw [field9_card]
    norm_num only [Nat.reduceSub, Nat.reduceDiv]
    have h : (15 : F9) = 0 := (CharP.cast_eq_zero_iff F9 3 15).mpr (by decide)
    linear_combination h)
  obtain ⟨a, ha⟩ := hs
  exact ⟨a, by simpa only [pow_two] using ha.symm⟩

theorem square_two_outside {a : F9} (ha : a^2 = 2) : outsideBase (ZMod 3) F9 a := by
  rintro ⟨b, rfl⟩
  have hb : b^2 = (2 : ZMod 3) := by
    apply (algebraMap (ZMod 3) F9).injective
    simpa only [map_pow, map_ofNat] using ha
  have hh : ∀ b : ZMod 3, b^2 ≠ 2 := by decide
  exact hh b hb

def quadraticSwap : GL2 (ZMod 3) :=
  Matrix.GeneralLinearGroup.mkOfDetNeZero !![0, 2; 1, 0] (by
    norm_num [det_fin_two]
    decide)

theorem quadraticSwap_not_projective_identity : Matrix.ProjGenLinGroup.mk quadraticSwap ≠ 1 := by
  intro h
  rw [Matrix.ProjGenLinGroup.mk_eq_one, Matrix.GeneralLinearGroup.center_eq_range_scalar] at h
  obtain ⟨u, hu⟩ := h
  have hh := congrArg (fun A : GL2 (ZMod 3) => A 0 1) hu
  norm_num [quadraticSwap, Matrix.GeneralLinearGroup.coe_scalar, Matrix.scalar_apply] at hh
  exact (by decide : (0 : ZMod 3) ≠ 2) hh

theorem quadraticSwap_fixes {a : F9} (ha : a^2 = 2) :
    fractional (ZMod 3) F9 quadraticSwap.val a = a := by
  have hz := square_two_outside ha
  apply (sends_iff_fractional (ZMod 3) F9 quadraticSwap.val a a
    (denominator_ne_zero (ZMod 3) F9 hz quadraticSwap)).mp
  simpa [sends, numerator, denominator, quadraticSwap, pow_two, map_ofNat] using ha.symm

/-- Actual quadratic-extension counterexample to deleting the cubic hypothesis. -/
theorem quadratic_stabilizer_control :
    Fintype.card F9 = 9 ∧ Module.finrank (ZMod 3) F9 = 2 ∧
    ∃ a : F9, outsideBase (ZMod 3) F9 a ∧
      fractional (ZMod 3) F9 quadraticSwap.val a = a ∧
      Matrix.ProjGenLinGroup.mk quadraticSwap ≠ 1 := by
  obtain ⟨a, ha⟩ := field9_square_two
  exact ⟨field9_card, field9_degree, a, square_two_outside ha,
    quadraticSwap_fixes ha, quadraticSwap_not_projective_identity⟩

theorem uniqueness_without_cubic_degree_false :
    ¬ (∀ a : F9, outsideBase (ZMod 3) F9 a → ∀ A : GL2 (ZMod 3),
      fractional (ZMod 3) F9 A.val a = a → Matrix.ProjGenLinGroup.mk A = 1) := by
  intro h
  obtain ⟨a, ha⟩ := field9_square_two
  exact quadraticSwap_not_projective_identity
    (h a (square_two_outside ha) quadraticSwap (quadraticSwap_fixes ha))

#print axioms field9_square_two
#print axioms quadraticSwap_not_projective_identity
#print axioms quadratic_stabilizer_control
#print axioms uniqueness_without_cubic_degree_false
end
end MomoseCubicHomographyAudit
