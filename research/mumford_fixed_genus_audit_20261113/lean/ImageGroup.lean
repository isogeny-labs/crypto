import OddFields
import Mathlib.Algebra.Group.Subgroup.Finite

namespace MumfordFixedGenusAudit
open Polynomial
noncomputable section
variable {F : Type*} [Field F]

def Image (ι : Base →+* F) := Set.range (extend ι)

def imageEquiv (ι : Base →+* F) : BasePair ≃ Image ι :=
  Equiv.ofInjective (extend ι) (extend_injective ι)

instance imageGroup (ι : Base →+* F) : AddCommGroup (Image ι) :=
  (imageEquiv ι).symm.addCommGroup

noncomputable instance imageFintype (ι : Base →+* F) : Fintype (Image ι) :=
  Fintype.ofEquiv BasePair (imageEquiv ι)

def imageIso (ι : Base →+* F) : BasePair ≃+ Image ι :=
  ((imageEquiv ι).symm.addEquiv).symm

theorem imageIso_coordinates (ι : Base →+* F) (P : BasePair) :
    (imageIso ι P).val=extend ι P := rfl

theorem image_addition_is_Cantor (ι : Base →+* F) (P Q : Image ι) :
    ZeroStep P.val Q.val (P+Q).val := by
  obtain ⟨A,rfl⟩ := (imageIso ι).surjective P
  obtain ⟨B,rfl⟩ := (imageIso ι).surjective Q
  rw [← map_add,imageIso_coordinates,imageIso_coordinates,imageIso_coordinates]
  exact extended_addition_is_Cantor ι A B

theorem image_Cantor_iff_addition (ι : Base →+* F) (P Q : Image ι) (R : Pair F) :
    ZeroStep P.val Q.val R ↔ R=(P+Q).val := by
  constructor
  · intro h
    exact zeroStep_unique _ _ _ _ h (image_addition_is_Cantor ι P Q)
  · rintro rfl
    exact image_addition_is_Cantor ι P Q

theorem image_card (ι : Base →+* F) : Nat.card (Image ι)=8 := by
  rw [← Nat.card_congr (imageEquiv ι),Nat.card_eq_fintype_card]
  exact MumfordSmoothnessAudit.reducedPair_cardinality

def cyclic (ι : Base →+* F) : AddSubgroup (Image ι) :=
  MumfordCantorGroupAudit.smallSubgroup.map (imageIso ι).toAddMonoidHom

theorem cyclic_is_zmultiples (ι : Base →+* F) :
    cyclic ι=AddSubgroup.zmultiples (imageIso ι (MumfordCantorAudit.point 1)) := by
  rw [cyclic,MumfordCantorGroupAudit.smallSubgroup_cyclic,AddMonoidHom.map_zmultiples]
  rfl

theorem cyclic_card (ι : Base →+* F) : Nat.card (cyclic ι)=2 := by
  rw [cyclic,AddSubgroup.card_map_of_injective (imageIso ι).injective,
    Nat.card_eq_fintype_card,MumfordCantorGroupAudit.smallSubgroup_cardinality]

theorem generator_order (ι : Base →+* F) :
    addOrderOf (imageIso ι (MumfordCantorAudit.point 1))=2 := by
  rw [(imageIso ι).addOrderOf_eq,MumfordCantorGroupAudit.point_one_order]

theorem cyclic_proper (ι : Base →+* F) : cyclic ι≠⊤ := by
  intro he
  have hm : imageIso ι (MumfordCantorAudit.point 7)∈cyclic ι := by
    rw [he]
    exact AddSubgroup.mem_top _
  obtain ⟨P,hP,hPe⟩ := hm
  have hp : P=MumfordCantorAudit.point 7 := (imageIso ι).injective hPe
  rw [hp] at hP
  change MumfordCantorAudit.point 7∈MumfordCantorGroupAudit.smallSubgroup at hP
  rw [MumfordCantorGroupAudit.smallSubgroup_membership] at hP
  exact (by decide +kernel : ¬ ((7 : Fin 8)=0 ∨ (7 : Fin 8)=1)) hP

theorem cyclic_all_smooth (ι : Base →+* F) (P : cyclic ι) : Smooth P.val.val := by
  obtain ⟨Q,hQ,hQP⟩ := P.property
  rw [← hQP]
  change (extend ι Q).u.Splits
  exact extend_splits ι Q (MumfordCantorGroupAudit.subgroup_all_smooth ⟨Q,hQ⟩)

def cyclicFraction (ι : Base →+* F) : ℚ :=
  (Nat.card {P : cyclic ι // Smooth P.val.val}:ℚ)/Nat.card (cyclic ι)

theorem cyclicFraction_one (ι : Base →+* F) : cyclicFraction ι=1 := by
  have h : Nat.card {P : cyclic ι // Smooth P.val.val}=Nat.card (cyclic ι) :=
    Nat.card_congr (Equiv.subtypeUnivEquiv (cyclic_all_smooth ι))
  rw [cyclicFraction,h,cyclic_card]
  norm_num

def ambientFraction (F : Type*) [Field F] : ℚ :=
  (Nat.card {P : Pair F // Smooth P}:ℚ)/Nat.card (Pair F)

theorem family_ambientFraction_lt_one (t : ℕ) : ambientFraction (FamilyField t)<1 := by
  have h := ambient_strict_count _ (actual_nonsmooth_point t)
  have hn : 0 < Nat.card (Pair (FamilyField t)) := lt_of_le_of_lt (Nat.zero_le _) h
  unfold ambientFraction
  apply (div_lt_one (by exact_mod_cast hn : (0:ℚ)<Nat.card (Pair (FamilyField t)))).mpr
  exact_mod_cast h

theorem fixed_genus_unequal_fractions (t : ℕ) :
    cyclicFraction (coefficientMap t)≠ambientFraction (FamilyField t) := by
  rw [cyclicFraction_one]
  exact ne_of_gt (family_ambientFraction_lt_one t)

#print axioms image_Cantor_iff_addition
#print axioms fixed_genus_unequal_fractions
end
end MumfordFixedGenusAudit
