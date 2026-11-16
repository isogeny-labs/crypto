import RationalDomainCyclic

namespace WeilRationalDomainAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit
private instance : Fact (Nat.Prime 19):=⟨by decide⟩
abbrev Curve:=BKGeneralFrobeniusAudit.SourceNineteen.curve
abbrev Closure:=BKGeneralFrobeniusAudit.SourceNineteen.Closure
private instance : DecidableEq Closure:=Classical.decEq _
private instance : CharP Closure 19:=charP_of_injective_algebraMap' (ZMod 19) 19
private instance : NeZero (3 : Closure):=⟨by
  intro h
  have he:=(CharP.cast_eq_zero_iff Closure 19 3).mp h
  norm_num at he⟩
abbrev GeometricCurve:=Curve.baseChange Closure
abbrev RationalThree:=torsionPoints Curve 3
abbrev GeometricThree:=torsionPoints GeometricCurve 3

theorem geometric_delta : GeometricCurve.Δ≠0 :=
  BKGeneralFrobeniusAudit.SourceNineteen.closure_nonsingular

def rationalThreeInclusion : RationalThree →+ GeometricThree:=integerTorsionInclusion Curve 3

theorem rational_three_inclusion_injective : Function.Injective rationalThreeInclusion :=
  integer_torsion_inclusion_injective Curve 3

def rationalThreeWitness : RationalThree:=
  ⟨(6 : ℕ) • BKGeneralFrobeniusAudit.SourceNineteen.generator,by
    have ht : (3 : ℕ) • ((6 : ℕ) • BKGeneralFrobeniusAudit.SourceNineteen.generator)=0 := by
      rw [smul_smul]
      exact BKGeneralFrobeniusAudit.SourceNineteen.generator_controls.1
    exact (natCast_zsmul ((6 : ℕ) • BKGeneralFrobeniusAudit.SourceNineteen.generator) 3).trans ht⟩

theorem rational_three_witness_ne_zero : rationalThreeWitness≠0 := by
  intro h
  exact BKGeneralFrobeniusAudit.SourceNineteen.generator_controls.2.2
    (congrArg (fun X : RationalThree=>X.val) h)

theorem included_witness_ne_zero : rationalThreeInclusion rationalThreeWitness≠0 := by
  intro h
  exact rational_three_witness_ne_zero
    (rational_three_inclusion_injective (h.trans (map_zero rationalThreeInclusion).symm))

theorem rational_pairing_one (P Q : RationalThree) :
    geometricPairing GeometricCurve geometric_delta 3
      (rationalThreeInclusion P) (rationalThreeInclusion Q)=1 := by
  let : IsAddCyclic Curve.Point:=BKGeneralFrobeniusAudit.SourceNineteen.rational_cyclic
  exact actual_cyclic_pullback_trivial GeometricCurve geometric_delta 3 rationalThreeInclusion P Q

theorem geometric_partner_outside :
    ∃ Q : GeometricThree,
      geometricPairing GeometricCurve geometric_delta 3 (rationalThreeInclusion rationalThreeWitness) Q≠1 ∧
        Q∉Set.range rationalThreeInclusion := by
  have he : ∃ Q : GeometricThree,
      geometricPairing GeometricCurve geometric_delta 3 (rationalThreeInclusion rationalThreeWitness) Q≠1 := by
    by_contra hh
    push Not at hh
    exact included_witness_ne_zero
      (geometric_pairing_three_left_nondegenerate GeometricCurve geometric_delta _ hh)
  obtain ⟨Q,hQ⟩:=he
  refine ⟨Q,hQ,?_⟩
  rintro ⟨R,hr⟩
  apply hQ
  rw [←hr]
  exact rational_pairing_one _ _

theorem rational_three_card : Nat.card RationalThree=3 :=
  (Nat.card_congr (natIntegerTorsionEquiv Curve 3).toEquiv).symm.trans
    BKGeneralFrobeniusAudit.SourceNineteen.rational_three_torsion_card

theorem geometric_three_card : Nat.card GeometricThree=9 :=
  (Nat.card_congr (natIntegerTorsionEquiv GeometricCurve 3).toEquiv).symm.trans
    BKGeneralFrobeniusAudit.SourceNineteen.geometric_three_card

def WrongRationalThreeNondegeneracy : Prop:=
  ∀ P : RationalThree,
    (∀ Q : RationalThree,geometricPairing GeometricCurve geometric_delta 3
      (rationalThreeInclusion P) (rationalThreeInclusion Q)=1)→P=0

theorem wrong_rational_three_nondegeneracy_false : ¬WrongRationalThreeNondegeneracy := by
  intro h
  exact rational_three_witness_ne_zero (h rationalThreeWitness (rational_pairing_one _))

#print axioms rational_pairing_one
#print axioms geometric_partner_outside
#print axioms wrong_rational_three_nondegeneracy_false
end
end WeilRationalDomainAudit
