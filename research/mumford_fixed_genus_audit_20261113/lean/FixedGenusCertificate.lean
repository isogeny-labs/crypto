import EvenControl

namespace MumfordFixedGenusAudit
open Polynomial
noncomputable section

theorem unbounded_fixed_genus_counterexamples (B : ℕ) :
    ∃ t : ℕ,B < Nat.card (FamilyField t) ∧
      cyclicFraction (coefficientMap t)≠ambientFraction (FamilyField t) := by
  obtain ⟨t,ht⟩ := fields_unbounded B
  exact ⟨t,ht,fixed_genus_unequal_fractions t⟩

theorem universal_fixed_genus_equality_false :
    ¬ (∀ t : ℕ,cyclicFraction (coefficientMap t)=ambientFraction (FamilyField t)) := by
  intro h
  exact fixed_genus_unequal_fractions 0 (h 0)

/-- Exact fixed-genus arithmetic counterfamily, with actual curve/field and
Cantor outputs, full ambient-pair counting, concrete cyclic subgroup and
the boundary control. No ambient geometric group is assumed. -/
theorem fixed_genus_counterfamily_certificate :
    (∀ t : ℕ,Nat.card (FamilyField t)=3^(2*t+1) ∧
      (curve (FamilyField t)).natDegree=5 ∧ (curve (FamilyField t)).Monic ∧
      (curve (FamilyField t)).Separable ∧
      Nat.card (Image (coefficientMap t))=8 ∧ Nat.card (cyclic (coefficientMap t))=2 ∧
      cyclic (coefficientMap t)=AddSubgroup.zmultiples
        (imageIso (coefficientMap t) (MumfordCantorAudit.point 1)) ∧
      cyclic (coefficientMap t)≠⊤ ∧
      addOrderOf (imageIso (coefficientMap t) (MumfordCantorAudit.point 1))=2 ∧
      (imageIso (coefficientMap t) (MumfordCantorAudit.point 1)).val.u=X ∧
      (imageIso (coefficientMap t) (MumfordCantorAudit.point 1)).val.v=0 ∧
      cyclicFraction (coefficientMap t)=1 ∧ ambientFraction (FamilyField t)<1 ∧
      cyclicFraction (coefficientMap t)≠ambientFraction (FamilyField t) ∧
      ¬Smooth (extend (coefficientMap t) (MumfordCantorAudit.point 7))) ∧
    (∀ (t : ℕ) (P Q : Image (coefficientMap t)) (R : Pair (FamilyField t)),
      ZeroStep P.val Q.val R ↔ R=(P+Q).val) ∧
    (∀ B : ℕ,∃ t : ℕ,B < Nat.card (FamilyField t) ∧
      cyclicFraction (coefficientMap t)≠ambientFraction (FamilyField t)) ∧
    ¬ (∀ t : ℕ,cyclicFraction (coefficientMap t)=ambientFraction (FamilyField t)) ∧
    (∀ t : ℕ,Smooth (extend (evenMap t) (MumfordCantorAudit.point 7))) ∧
    ¬ (∀ (F : Type) [Field F] [Finite F] (ι : Base →+* F),
      ¬Smooth (extend ι (MumfordCantorAudit.point 7))) := by
  refine ⟨?_,?_,unbounded_fixed_genus_counterexamples,universal_fixed_genus_equality_false,
    even_point_smooth,nonsmooth_preserved_under_every_extension_false⟩
  · intro t
    obtain ⟨hm,hd,hs⟩ := curve_properties (coefficientMap t)
    obtain ⟨hgu,hgv⟩ := actual_generator_coordinates (coefficientMap t)
    exact ⟨family_card t,hd,hm,hs,image_card _,cyclic_card _,cyclic_is_zmultiples _,
      cyclic_proper _,generator_order _,hgu,hgv,cyclicFraction_one _,
      family_ambientFraction_lt_one t,fixed_genus_unequal_fractions t,actual_nonsmooth_point t⟩
  · intro t P Q R
    exact image_Cantor_iff_addition _ P Q R

#print axioms fixed_genus_counterfamily_certificate
end
end MumfordFixedGenusAudit
