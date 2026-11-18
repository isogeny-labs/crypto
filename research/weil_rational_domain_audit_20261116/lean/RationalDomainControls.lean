import RationalDomainTate

namespace WeilRationalDomainAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit
abbrev RationalTwo:=torsionPoints Curve 2

def rationalTwoWitness : RationalTwo:=
  ⟨(9 : ℕ) • BKGeneralFrobeniusAudit.SourceNineteen.generator,by
    have ht : (2 : ℕ) • ((9 : ℕ) • BKGeneralFrobeniusAudit.SourceNineteen.generator)=0 := by
      rw [smul_smul]
      exact BKGeneralFrobeniusAudit.SourceNineteen.generator_controls.1
    exact (natCast_zsmul ((9 : ℕ) • BKGeneralFrobeniusAudit.SourceNineteen.generator) 2).trans ht⟩

theorem rational_two_witness_ne_zero : rationalTwoWitness≠0 := by
  intro h
  exact BKGeneralFrobeniusAudit.SourceNineteen.generator_controls.2.1
    (congrArg (fun X : RationalTwo=>X.val) h)

theorem tate_two_natural_map_injective : Function.Injective (torsionToTateRight Curve 2) := by
  apply (injective_iff_map_eq_zero (torsionToTateRight Curve 2)).mpr
  intro P hP
  obtain ⟨R,hR⟩:=(QuotientAddGroup.eq_zero_iff P.val).mp hP
  change (2 : ℕ) • R=P.val at hR
  have h9 : (9 : ℕ) • P.val=0 := by
    rw [←hR,smul_smul]
    exact eighteen_kills R
  have h2 : (2 : ℕ) • P.val=0 := (natCast_zsmul P.val 2).symm.trans P.property
  have h8 : (8 : ℕ) • P.val=0 := by
    have h:=congrArg (fun X : Curve.Point=>(4 : ℕ) • X) h2
    norm_num only [smul_smul,nsmul_zero] at h
    exact h
  have hz : P.val=0 := by
    rw [show (9 : ℕ)=8+1 by decide,add_nsmul,h8,one_nsmul,zero_add] at h9
    exact h9
  exact Subtype.ext hz

theorem tate_two_natural_map_surjective : Function.Surjective (torsionToTateRight Curve 2) := by
  intro z
  refine QuotientAddGroup.induction_on z (fun R=>?_)
  have h2 : (2 : ℤ) • ((9 : ℕ) • R)=0 := by
    have ht : (2 : ℕ) • ((9 : ℕ) • R)=0 := by
      rw [smul_smul]
      exact eighteen_kills R
    exact (natCast_zsmul ((9 : ℕ) • R) 2).trans ht
  refine ⟨⟨(9 : ℕ) • R,h2⟩,?_⟩
  change toTateRight Curve 2 ((9 : ℕ) • R)=toTateRight Curve 2 R
  rw [map_nsmul]
  have h8 : (8 : ℕ) • toTateRight Curve 2 R=0 := by
    have h:=congrArg (fun X : TateRight Curve 2=>(4 : ℕ) • X) (quotient_killed Curve 2 R)
    norm_num only [smul_smul,nsmul_zero] at h
    exact h
  rw [show (9 : ℕ)=8+1 by decide,add_nsmul,h8,one_nsmul,zero_add]

def actualTateTwoEquiv : RationalTwo ≃+ TateRight Curve 2:=
  AddEquiv.ofBijective (torsionToTateRight Curve 2)
    ⟨tate_two_natural_map_injective,tate_two_natural_map_surjective⟩

theorem tate_two_witness_class_ne_zero : torsionToTateRight Curve 2 rationalTwoWitness≠0 := by
  intro h
  exact rational_two_witness_ne_zero
    (tate_two_natural_map_injective (h.trans (map_zero (torsionToTateRight Curve 2)).symm))

#print axioms tate_two_natural_map_injective
#print axioms tate_two_natural_map_surjective
#print axioms tate_two_witness_class_ne_zero
end
end WeilRationalDomainAudit
