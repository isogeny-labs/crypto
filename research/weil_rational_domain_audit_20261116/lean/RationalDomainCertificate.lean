import RationalDomainControls

namespace WeilRationalDomainAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit

def ActualCyclicPullbackLaw : Prop:=
  ∀ (K G : Type) (_ : Field K) (_ : DecidableEq K) (_ : IsAlgClosed K)
    (_ : AddGroup G) (_ : IsAddCyclic G) (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (m : ℤ) (f : G →+ torsionPoints W m) (P Q : G),geometricPairing W hΔ m (f P) (f Q)=1

def ActualNineteenControls : Prop:=
  Curve.Δ≠0 ∧ Nat.card Curve.Point=18 ∧
    addOrderOf BKGeneralFrobeniusAudit.SourceNineteen.generator=18 ∧
    Nat.Coprime 3 19 ∧ 3∣19-1 ∧ 9∣Nat.card Curve.Point ∧
    Nat.card RationalThree=3 ∧ Nat.card GeometricThree=9 ∧ rationalThreeWitness≠0

def ActualRationalThreeCollapse : Prop:=
  ∀ P Q : RationalThree,geometricPairing GeometricCurve geometric_delta 3
    (rationalThreeInclusion P) (rationalThreeInclusion Q)=1

def ActualGeometricOutsidePartner : Prop:=
  rationalThreeInclusion rationalThreeWitness≠0 ∧
    ∃ Q : GeometricThree,
      geometricPairing GeometricCurve geometric_delta 3 (rationalThreeInclusion rationalThreeWitness) Q≠1 ∧
        Q∉Set.range rationalThreeInclusion

def ActualFullThreeExtensionCriterion : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : Fintype K) (_ : Algebra (ZMod 19) K)
    (_ : Algebra K Closure) (_ : IsScalarTower (ZMod 19) K Closure) (_ : DecidableEq K)
    (k : ℕ),Fintype.card K=19^k →
      ((∀ P : GeometricCurve.Point,(3 : ℕ) • P=0 →
        P∈Set.range (Point.map (W':=Curve) (Algebra.ofId K Closure))) ↔ 3∣k)

def ActualTateThreeDomainFailure : Prop:=
  (∀ P : RationalThree,torsionToTateRight Curve 3 P=0) ∧
    toTateRight Curve 3 BKGeneralFrobeniusAudit.SourceNineteen.generator≠0

def ActualTateTwoPositive : Prop:=
  Function.Bijective (torsionToTateRight Curve 2) ∧
    Nonempty (RationalTwo ≃+ TateRight Curve 2) ∧ torsionToTateRight Curve 2 rationalTwoWitness≠0

theorem actual_rational_domain_certificate :
    ActualCyclicPullbackLaw ∧ ActualNineteenControls ∧ ActualRationalThreeCollapse ∧
      ActualGeometricOutsidePartner ∧ ActualFullThreeExtensionCriterion ∧
      ActualTateThreeDomainFailure ∧ ActualTateTwoPositive ∧
      ¬WrongRationalThreeNondegeneracy ∧ ¬WrongTateThreeIdentification := by
  refine ⟨?_,?_,rational_pairing_one,⟨included_witness_ne_zero,geometric_partner_outside⟩,?_,
    ⟨tate_three_natural_map_zero,tate_three_generator_class_ne_zero⟩,
    ⟨⟨tate_two_natural_map_injective,tate_two_natural_map_surjective⟩,
      ⟨actualTateTwoEquiv⟩,tate_two_witness_class_ne_zero⟩,
    wrong_rational_three_nondegeneracy_false,wrong_tate_three_identification_false⟩
  · intro K G _ _ _ _ _ W hΔ m f P Q
    exact actual_cyclic_pullback_trivial W hΔ m f P Q
  · refine ⟨BKGeneralFrobeniusAudit.SourceNineteen.curve_nonsingular,
      BKGeneralFrobeniusAudit.SourceNineteen.rational_cardinality,
      BKGeneralFrobeniusAudit.SourceNineteen.generator_order,by decide,by decide,?_,
      rational_three_card,geometric_three_card,rational_three_witness_ne_zero⟩
    rw [BKGeneralFrobeniusAudit.SourceNineteen.rational_cardinality]
    decide
  · intro K _ _ _ _ _ _ k hc
    exact BKGeneralFrobeniusAudit.SourceNineteen.source_all_extensions k hc

#print axioms actual_rational_domain_certificate
end
end WeilRationalDomainAudit
