import RationalDomainNineteen

namespace WeilRationalDomainAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit

def actualMultiples {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (n : ℕ) : AddSubgroup W.Point:=
  (nsmulAddMonoidHom n : W.Point →+ W.Point).range

abbrev TateRight {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (n : ℕ):=W.Point ⧸ actualMultiples W n

def toTateRight {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (n : ℕ) : W.Point →+ TateRight W n:=
  QuotientAddGroup.mk' (actualMultiples W n)

def torsionToTateRight {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (n : ℕ) : torsionPoints W (n : ℤ) →+ TateRight W n:=
  (toTateRight W n).comp (torsionPoints W (n : ℤ)).subtype

theorem quotient_killed {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (n : ℕ) (R : W.Point) :
    n • toTateRight W n R=0 := by
  rw [←map_nsmul]
  exact (QuotientAddGroup.eq_zero_iff _).mpr ⟨R,rfl⟩

theorem eighteen_kills (R : Curve.Point) : (18 : ℕ) • R=0 := by
  have h : Nat.card Curve.Point • R=0:=card_nsmul_eq_zero'
  simpa only [BKGeneralFrobeniusAudit.SourceNineteen.rational_cardinality] using h

theorem rational_three_divisible (P : RationalThree) :
    ∃ R : Curve.Point,(3 : ℕ) • R=P.val := by
  let : IsAddCyclic Curve.Point:=BKGeneralFrobeniusAudit.SourceNineteen.rational_cyclic
  obtain ⟨g,hg⟩:=exists_zsmul_surjective Curve.Point
  have ho : addOrderOf g=18 :=
    (addOrderOf_eq_card_of_forall_mem_zmultiples (fun R=>hg R)).trans
      BKGeneralFrobeniusAudit.SourceNineteen.rational_cardinality
  obtain ⟨z,hz⟩:=hg P.val
  change z • g=P.val at hz
  have hk : (3*z) • g=0 := by
    rw [mul_zsmul,hz]
    exact P.property
  have hd : (18 : ℤ)∣3*z := by
    have h:=addOrderOf_dvd_iff_zsmul_eq_zero.mpr hk
    simpa only [ho,Nat.cast_ofNat] using h
  obtain ⟨w,hw⟩:=hd
  have hzw : (3 : ℤ)*(2*w)=z := by omega
  refine ⟨(2*w) • g,?_⟩
  rw [←natCast_zsmul,smul_smul]
  change ((3 : ℤ)*(2*w)) • g=P.val
  rw [hzw,hz]

theorem tate_three_natural_map_zero (P : RationalThree) : torsionToTateRight Curve 3 P=0 := by
  obtain ⟨R,hR⟩:=rational_three_divisible P
  exact (QuotientAddGroup.eq_zero_iff P.val).mpr ⟨R,hR⟩

theorem tate_three_generator_class_ne_zero :
    toTateRight Curve 3 BKGeneralFrobeniusAudit.SourceNineteen.generator≠0 := by
  intro h
  obtain ⟨R,hR⟩:=(QuotientAddGroup.eq_zero_iff BKGeneralFrobeniusAudit.SourceNineteen.generator).mp h
  change (3 : ℕ) • R=BKGeneralFrobeniusAudit.SourceNineteen.generator at hR
  have hz : (6 : ℕ) • BKGeneralFrobeniusAudit.SourceNineteen.generator=0 := by
    rw [←hR,smul_smul]
    exact eighteen_kills R
  exact BKGeneralFrobeniusAudit.SourceNineteen.generator_controls.2.2 hz

def WrongTateThreeIdentification : Prop:=Function.Injective (torsionToTateRight Curve 3)

theorem wrong_tate_three_identification_false : ¬WrongTateThreeIdentification := by
  intro h
  exact rational_three_witness_ne_zero
    (h ((tate_three_natural_map_zero rationalThreeWitness).trans (map_zero (torsionToTateRight Curve 3)).symm))

#print axioms rational_three_divisible
#print axioms tate_three_natural_map_zero
#print axioms tate_three_generator_class_ne_zero
#print axioms wrong_tate_three_identification_false
end
end WeilRationalDomainAudit
