import MillerProperties

namespace PairingMillerBaseDescentAudit
open PairingMillerCurveAudit PairingMillerTorsionAudit PairingMillerPropertiesAudit
  FrobeniusPointDescentAudit

abbrev baseCurve := BKActualThreeAudit.curve

noncomputable def inclusion : baseCurve.Point→+curve.Point :=
  WeierstrassCurve.Affine.Point.map (W':=baseCurve) (Algebra.ofId (ZMod 5) F25)

def BaseRational (R : curve.Point) : Prop := ∃ S : baseCurve.Point,inclusion S=R

theorem base_rational_iff_frobenius (R : curve.Point) :
    BaseRational R ↔ pointFrobenius baseCurve R=R :=
  (point_frobenius_fixed_iff baseCurve R).symm

theorem indexed_frobenius : ∀ u : Index,
    pointFrobenius baseCurve (indexedPoint u)=indexedPoint (u.1,-u.2) := by
  decide +kernel

theorem base_rational_indexed_iff (u : Index) : BaseRational (indexedPoint u) ↔ u.2=0 := by
  rw [base_rational_iff_frobenius,indexed_frobenius,indexedPoint_injective.eq_iff]
  have hneg : ∀ b : ZMod 3,-b=b ↔ b=0 := by decide +kernel
  constructor
  · intro h
    exact (hneg u.2).mp (congrArg Prod.snd h)
  · intro h
    exact Prod.ext rfl (by simp [h])

theorem actual_rational_torsion_line (U : threeTorsion) :
    BaseRational U.val ↔ ∃ a : ZMod 3,rationalPoint a=U := by
  obtain ⟨u,rfl⟩:=torsionHom_bijective.surjective U
  change BaseRational (indexedPoint u) ↔ ∃ a,rationalPoint a=torsionHom u
  rw [base_rational_indexed_iff]
  constructor
  · intro h
    refine ⟨u.1,?_⟩
    change torsionHom (u.1,0)=torsionHom u
    congr 1
    exact Prod.ext rfl h.symm
  · rintro ⟨a,ha⟩
    have hi : (a,0)=u := torsionHom_bijective.injective ha
    exact (congrArg Prod.snd hi).symm

theorem Q_not_base_rational : ¬BaseRational Q := by
  have hq : indexedPoint (0,1)=Q := by decide +kernel
  rw [←hq,base_rational_indexed_iff]
  decide +kernel

theorem rational_domain_nondegeneracy_false :
    ¬∀ U : threeTorsion,BaseRational U.val →
      (∀ V : threeTorsion,BaseRational V.val → weilValue U V=1) → U=0 := by
  intro h
  apply rationalPoint_one_nonzero
  apply h (rationalPoint 1)
  · exact (actual_rational_torsion_line _).mpr ⟨1,rfl⟩
  · intro V hV
    obtain ⟨b,rfl⟩:=(actual_rational_torsion_line V).mp hV
    exact (rational_restriction_trivial 1 b).1

abbrev BaseUnits := (ZMod 5)ˣ
def baseCubes : Subgroup BaseUnits := (powMonoidHom 3 : BaseUnits→*BaseUnits).range
abbrev BaseTarget := BaseUnits ⧸ baseCubes

theorem base_cubes_surjective : Function.Surjective (powMonoidHom 3 : BaseUnits→*BaseUnits) := by
  decide +kernel

theorem base_target_trivial (z : BaseTarget) : z=1 := by
  refine QuotientGroup.induction_on z (fun u=>?_)
  exact (QuotientGroup.eq_one_iff u).mpr (base_cubes_surjective u)

def baseP : baseCurve.Point := .some 0 1 (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
    SkepticalAudit.rationalCurve_nonsingular).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel)

theorem baseP_three : (3:ℕ) • baseP=0 := by decide +kernel
theorem baseP_nonzero : baseP≠0 := by decide +kernel
def baseThreeTorsion : AddSubgroup baseCurve.Point :=
  (nsmulAddMonoidHom 3 : baseCurve.Point→+baseCurve.Point).ker

theorem no_left_nondegenerate_base_target :
    ¬∃ e : baseThreeTorsion→baseCurve.Point→BaseTarget,
      ∀ U,(∀ V,e U V=1) → U=0 := by
  rintro ⟨e,he⟩
  have hp := he ⟨baseP,baseP_three⟩ (fun V=>base_target_trivial (e _ V))
  exact baseP_nonzero (congrArg Subtype.val hp)

#print axioms indexed_frobenius
#print axioms rational_domain_nondegeneracy_false
#print axioms no_left_nondegenerate_base_target
end PairingMillerBaseDescentAudit
