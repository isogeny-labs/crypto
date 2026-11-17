import ActualGroup

namespace PairingKOneAudit

def threeMultiples : AddSubgroup curve.Point :=
  (nsmulAddMonoidHom 3 : curve.Point →+ curve.Point).range
abbrev Right := curve.Point ⧸ threeMultiples
def toRight : curve.Point →+ Right := QuotientAddGroup.mk' threeMultiples
def torsionToRight : threeTorsion →+ Right := toRight.comp threeTorsion.subtype

def reduce9 : ZMod 9 →+* ZMod 3 := ZMod.castHom (by decide : 3 ∣ 9) (ZMod 3)

noncomputable def residueHom : curve.Point →+ ZMod 3 :=
  reduce9.toAddMonoidHom.comp indexEquiv.symm.toAddMonoidHom

theorem residue_on_index (i : ZMod 9) : residueHom (indexedPoint i)=reduce9 i := by
  change reduce9 (indexEquiv.symm (indexEquiv i))=reduce9 i
  rw [AddEquiv.symm_apply_apply]

theorem reduce9_zero_iff : ∀ j : ZMod 9, reduce9 j=0 ↔ ∃ k : ZMod 9,3*k=j := by
  decide +kernel

theorem residue_surjective : Function.Surjective residueHom := by
  intro i
  obtain ⟨j,hj⟩ := ZMod.castHom_surjective (by decide : 3 ∣ 9) i
  exact ⟨indexedPoint j,(residue_on_index j).trans hj⟩

theorem residue_kernel : threeMultiples=residueHom.ker := by
  apply AddSubgroup.ext
  intro R
  constructor
  · rintro ⟨Q,rfl⟩
    change residueHom ((3:ℕ) • Q)=0
    rw [map_nsmul,nsmul_eq_mul]
    have h3 : ((3:ℕ):ZMod 3)=0 := by decide +kernel
    rw [h3,zero_mul]
  · intro h
    obtain ⟨j,rfl⟩ := indexHom_surjective R
    have hr : reduce9 j=0 := by
      change residueHom (indexedPoint j)=0 at h
      rwa [residue_on_index] at h
    obtain ⟨k,hk⟩ := (reduce9_zero_iff j).mp hr
    refine ⟨indexHom k,?_⟩
    change (3:ℕ) • indexHom k=indexHom j
    rw [←map_nsmul,nsmul_eq_mul]
    simpa only [Nat.cast_ofNat] using congrArg indexHom hk

noncomputable def rightEquiv : Right ≃+ ZMod 3 :=
  QuotientAddGroup.liftEquiv threeMultiples residue_surjective residue_kernel

theorem right_on_index (i : ZMod 9) : rightEquiv (toRight (indexedPoint i))=reduce9 i :=
  residue_on_index i

theorem right_on_G : rightEquiv (toRight G)=1 := by
  have hi : indexedPoint (1:ZMod 9)=G := by
    simp only [indexedPoint,show (1:ZMod 9).val=1 by decide +kernel,one_nsmul]
  have hr : reduce9 (1:ZMod 9)=1 := by decide +kernel
  simpa only [hi,hr] using right_on_index 1

theorem actual_right_cardinality : Nat.card Right=3 := by
  rw [Nat.card_congr rightEquiv.toEquiv,Nat.card_eq_fintype_card]
  exact ZMod.card 3

theorem G_class_ne_zero : toRight G≠0 := by
  intro h
  have hh := congrArg rightEquiv h
  rw [right_on_G,map_zero] at hh
  exact (show (1:ZMod 3)≠0 by decide) hh

theorem torsionToRight_zero (U : threeTorsion) : torsionToRight U=0 := by
  obtain ⟨i,rfl⟩ := torsionHom_bijective.surjective U
  apply (QuotientAddGroup.eq_zero_iff _).mpr
  refine ⟨i.val • G,?_⟩
  change (3:ℕ) • (i.val • G)=i.val • ((3:ℕ) • G)
  exact smul_comm 3 i.val G

theorem torsion_representatives_not_surjective : ¬Function.Surjective torsionToRight := by
  intro h
  obtain ⟨U,hU⟩ := h (toRight G)
  exact G_class_ne_zero (hU.symm.trans (torsionToRight_zero U))

theorem pullback_not_left_nondegenerate {M : Type*} [One M]
    (b : threeTorsion → Right → M) (hb : ∀ U,b U 0=1) :
    ¬(∀ U : threeTorsion,(∀ V : threeTorsion,b U (torsionToRight V)=1) → U=0) := by
  intro h
  apply torsionHom_one_ne_zero
  apply h
  intro V
  rw [torsionToRight_zero,hb]

#print axioms residue_kernel
#print axioms actual_right_cardinality
#print axioms torsionToRight_zero
#print axioms pullback_not_left_nondegenerate
end PairingKOneAudit
