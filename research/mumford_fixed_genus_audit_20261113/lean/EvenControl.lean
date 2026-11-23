import ImageGroup

namespace MumfordFixedGenusAudit
open Polynomial
noncomputable section

abbrev EvenField (t : ℕ) := GaloisField 3 (2*t+2)
noncomputable instance evenFintype (t : ℕ) : Fintype (EvenField t) := Fintype.ofFinite _
def evenMap (t : ℕ) : Base →+* EvenField t := algebraMap _ _

theorem even_card (t : ℕ) : Nat.card (EvenField t)=3^(2*t+2) :=
  GaloisField.card 3 _ (by omega)

theorem even_power_mod_four (t : ℕ) : (3^(2*t+2))%4=1 := by
  have he : 2*t+2=2*(t+1) := by omega
  rw [he,pow_mul,Nat.pow_mod]
  norm_num

theorem even_square_neg_one (t : ℕ) : IsSquare (-1 : EvenField t) := by
  apply FiniteField.isSquare_neg_one_iff.mpr
  rw [← Nat.card_eq_fintype_card,even_card,even_power_mod_four]
  decide +kernel

theorem quadratic_splits_of_square_neg_one {F : Type*} [Field F]
    (h : IsSquare (-1 : F)) : (X^2+1 : F[X]).Splits := by
  obtain ⟨x,hx⟩ := h
  have hx2 : x^2= -1 := by simpa only [pow_two] using hx.symm
  have hh := congrArg (C : F →+* F[X]) hx2
  simp only [map_pow,map_neg,map_one] at hh
  have he : (X-C x)*(X+C x)=(X^2+1 : F[X]) := by
    calc
      (X-C x)*(X+C x)=X^2-(C x)^2 := by ring
      _ = X^2+1 := by rw [hh]; ring
  rw [← he]
  exact (Splits.X_sub_C x).mul (Splits.X_add_C x)

theorem even_point_smooth (t : ℕ) :
    Smooth (extend (evenMap t) (MumfordCantorAudit.point 7)) := by
  have hc : (extend (evenMap t) (MumfordCantorAudit.point 7)).u=X^2+1 := by
    simp [extend,MumfordCantorAudit.point,MumfordSmoothnessAudit.candidatePair,
      MumfordSmoothnessAudit.representative,MumfordSmoothnessAudit.uPolynomial]
  unfold Smooth
  rw [hc]
  exact quadratic_splits_of_square_neg_one (even_square_neg_one t)

theorem nonsmooth_preserved_under_every_extension_false :
    ¬ (∀ (F : Type) [Field F] [Finite F] (ι : Base →+* F),
      ¬Smooth (extend ι (MumfordCantorAudit.point 7))) := by
  intro h
  exact h (EvenField 0) (evenMap 0) (even_point_smooth 0)

theorem actual_generator_coordinates {F : Type*} [Field F] (ι : Base →+* F) :
    (imageIso ι (MumfordCantorAudit.point 1)).val.u=X ∧
      (imageIso ι (MumfordCantorAudit.point 1)).val.v=0 := by
  rw [imageIso_coordinates]
  change (MumfordCantorAudit.point 1).u.map ι=X ∧ (MumfordCantorAudit.point 1).v.map ι=0
  rw [MumfordCantorGroupAudit.generator_coordinates.1,
    MumfordCantorGroupAudit.generator_coordinates.2,Polynomial.map_X,Polynomial.map_zero]
  exact ⟨rfl,rfl⟩

#print axioms nonsmooth_preserved_under_every_extension_false
end
end MumfordFixedGenusAudit
