import Inflection

namespace PairingKOneAudit
noncomputable section
abbrev GeometricField := TorsionBoundaryAudit.K
local instance geometricDecidableEq : DecidableEq GeometricField := Classical.decEq _
abbrev geometricCurve := curve.baseChange GeometricField
def basePointMap : curve.Point →+ geometricCurve.Point :=
  WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId F GeometricField)

theorem nonrational_geometric_torsion : ∃ P : geometricCurve.Point,
    (3:ℕ) • P=0 ∧ P ∉ Set.range basePointMap := by
  obtain ⟨_,_,_,_,_,P,hP,hn⟩ := TorsionBoundaryAudit.bk_excluded_boundary_counterexample
  exact ⟨P,hP,hn⟩

theorem embedding_degree_one : TorsionBoundaryAudit.embeddingDegree 7 3=1 := by
  change orderOf (7:ZMod 3)=1
  rw [show (7:ZMod 3)=1 by decide +kernel,orderOf_one]

theorem actual_k1_quotient_miller_certificate :
    (curve.Δ≠0 ∧ Nat.card curve.Point=9 ∧ addOrderOf G=9 ∧
      Nat.Prime 3 ∧ Nat.Coprime 3 7 ∧ 3 ∣ 7-1 ∧ 3^2 ∣ Nat.card curve.Point ∧
      TorsionBoundaryAudit.embeddingDegree 7 3=1) ∧
    (Nat.card threeTorsion=3 ∧ Nat.card Right=3 ∧ Nat.card UnitTarget=3 ∧
      Nat.card cubeRoots=3 ∧ torsionHom 1≠0 ∧ toRight G≠0 ∧
      ∀ U : threeTorsion,torsionToRight U=0) ∧
    (∀ (U : threeTorsion) (R : curve.Point),
      tateValue U (toRight R)=reducedTateMiller U.val R) ∧
    (∀ (U : threeTorsion) (R S : curve.Point) (h : goodAux U.val R S),
      (QuotientGroup.mk' baseCubes) (rawAuxUnit U R S h)=tateClass U (toRight R)) ∧
    (∀ (U V : threeTorsion) (S : curve.Point),
      WeilAuxSupport U.val V.val S → weilAtAux U.val V.val S=1) ∧
    (∀ (U V : threeTorsion) (z : Right),
      tateClass (U+V) z=tateClass U z*tateClass V z) ∧
    (∀ (U : threeTorsion) (z w : Right),
      tateClass U (z+w)=tateClass U z*tateClass U w) ∧
    (∀ U : threeTorsion,(∀ z : Right,tateClass U z=1) → U=0) ∧
    (∀ z : Right,(∀ U : threeTorsion,tateClass U z=1) → z=0) ∧
    (∀ (U V : threeTorsion) (z : Right),
      reducedUnits (U+V) z=reducedUnits U z*reducedUnits V z) ∧
    (∀ (U : threeTorsion) (z w : Right),
      reducedUnits U (z+w)=reducedUnits U z*reducedUnits U w) ∧
    (∀ U : threeTorsion,(∀ z : Right,reducedUnits U z=1) → U=0) ∧
    (∀ z : Right,(∀ U : threeTorsion,reducedUnits U z=1) → z=0) ∧
    (∀ U V : threeTorsion,weilValue U V=1 ∧ tateClass U (torsionToRight V)=1) ∧
    (¬∀ U : threeTorsion,(∀ V : threeTorsion,weilValue U V=1) → U=0) ∧
    (¬∀ U : threeTorsion,(∀ V : threeTorsion,tateClass U (torsionToRight V)=1) → U=0) ∧
    (∀ (A : Type) [AddCommGroup A] (b : threeTorsion →+ threeTorsion →+ A),
      (∀ U : threeTorsion,b U U=0) →
      (∀ U V : threeTorsion,b U V=0) ∧
      (¬∀ U : threeTorsion,(∀ V : threeTorsion,b U V=0) → U=0)) ∧
    (∀ U : threeTorsion,InflectionCertificate U.val) ∧
    (∃ P : geometricCurve.Point,(3:ℕ) • P=0 ∧ P ∉ Set.range basePointMap) ∧
    IsPrimitiveRoot (2:F) 3 ∧ reducedTateMiller ((3:ℕ) • G) G=2 := by
  refine ⟨?_,?_,tateValue_actual,raw_class_all_valid_auxiliaries,weil_all_valid_auxiliary_actual,
    class_left_bilinear,class_right_bilinear,class_left_nondegenerate,class_right_nondegenerate,
    reduced_left_bilinear,reduced_right_bilinear,reduced_left_nondegenerate,reduced_right_nondegenerate,
    ?_,weil_not_nondegenerate,class_restricted_not_nondegenerate,?_,actual_inflection_certificate,
    nonrational_geometric_torsion,two_primitive_cube_root,tate_generator_value⟩
  · refine ⟨TorsionBoundaryAudit.bkCurve_nonsingular,TorsionBoundaryAudit.bkCurve_cardinality,
      G_order,by decide,by decide,by decide,?_,embedding_degree_one⟩
    rw [TorsionBoundaryAudit.bkCurve_cardinality]
    decide
  · exact ⟨actual_rational_torsion_cardinality,actual_right_cardinality,
      actual_target_cardinalities.2,actual_target_cardinalities.1,
      torsionHom_one_ne_zero,G_class_ne_zero,torsionToRight_zero⟩
  · intro U V
    exact ⟨weil_all_one U V,class_restricted_trivial U V⟩
  · intro A _ b ha
    exact ⟨alternating_pairing_trivial b ha,alternating_pairing_not_nondegenerate b ha⟩

#print axioms actual_k1_quotient_miller_certificate

end
end PairingKOneAudit
