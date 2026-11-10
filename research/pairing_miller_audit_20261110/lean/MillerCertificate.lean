import WeilTarget
import TangentCertificate

namespace PairingMillerCertificateAudit
open PairingMillerCurveAudit PairingMillerExpressionsAudit PairingMillerTorsionAudit
  PairingMillerTableAudit PairingMillerPropertiesAudit PairingMillerBaseDescentAudit
  PairingMillerTateQuotientAudit PairingMillerUnitsTargetAudit PairingMillerTateClassAudit
  PairingMillerWeilTargetAudit PairingMillerTangentAudit

def torsionP : threeTorsion := ⟨P,P_three⟩
def torsionQ : threeTorsion := ⟨Q,Q_three⟩

theorem generator_root_values : rootValue (weilUnits torsionP torsionQ)=omega ∧
    rootValue (reducedUnits torsionP (torsionToRight torsionQ))=omega^2 := by
  constructor
  · rw [weil_units_value]
    exact weil_generator_values.1
  · rw [reduced_value,quotient_value_on_torsion]
    exact tate_generator_values.1

/-- Actual finite-domain Miller computations and direct base-domain
counterexamples. The polynomial intersection certificate is included;
canonical divisor/valuation identification is not asserted as a premise. -/
theorem concrete_miller_audit_certificate :
    Fintype.card F25=25 ∧ curve.Δ≠0 ∧
    Nat.card threeTorsion=9 ∧ Nat.card Right=9 ∧
    Nat.card ExtTarget=3 ∧ Nat.card cubeRoots=3 ∧
    (Nat.Coprime 3 5 ∧ ¬3∣5-1 ∧ 3∣25-1) ∧
    (∀ u v : Index,
      weilMiller (indexedPoint u) (indexedPoint v)=omega^(determinant u v).val ∧
      reducedTateMiller (indexedPoint u) (indexedPoint v)=omega^(2*determinant u v).val) ∧
    (∀ U V W : threeTorsion,
      weilUnits (U+V) W=weilUnits U W*weilUnits V W ∧
      weilUnits U (V+W)=weilUnits U V*weilUnits U W) ∧
    (∀ U V : threeTorsion,∀ z w : Right,
      tateClass (U+V) z=tateClass U z*tateClass V z ∧
      tateClass U (z+w)=tateClass U z*tateClass U w) ∧
    (∀ U : threeTorsion,(∀ V,weilUnits U V=1) → U=0) ∧
    (∀ V : threeTorsion,(∀ U,weilUnits U V=1) → V=0) ∧
    (∀ U : threeTorsion,(∀ z,tateClass U z=1) → U=0) ∧
    (∀ z : Right,(∀ U,tateClass U z=1) → z=0) ∧
    (∀ U : threeTorsion,BaseRational U.val ↔ ∃ a : ZMod 3,rationalPoint a=U) ∧
    ¬BaseRational Q ∧
    (¬∀ U : threeTorsion,BaseRational U.val →
      (∀ V : threeTorsion,BaseRational V.val → weilUnits U V=1) → U=0) ∧
    (∀ z : BaseTarget,z=1) ∧
    (¬∃ e : baseThreeTorsion→baseCurve.Point→BaseTarget,
      ∀ U,(∀ V,e U V=1) → U=0) ∧
    (∀ U : threeTorsion,InflectionCertificate U.val) ∧
    rootValue (weilUnits torsionP torsionQ)=omega ∧
    rootValue (reducedUnits torsionP (torsionToRight torsionQ))=omega^2 := by
  refine ⟨field_cardinality,curve_nonsingular,actual_torsion_cardinality,actual_right_cardinality,
    actual_target_cardinalities.2,actual_target_cardinalities.1,embedding_two_conditions,
    ?_,?_,?_,weil_units_left_nondegenerate,weil_units_right_nondegenerate,
    class_left_nondegenerate,class_right_nondegenerate,actual_rational_torsion_line,
    Q_not_base_rational,base_restriction_nondegeneracy_false,base_target_trivial,
    no_left_nondegenerate_base_target,every_torsion_inflection_certificate,
    generator_root_values.1,generator_root_values.2⟩
  · exact fun u v=>⟨weil_table u v,tate_table u v⟩
  · exact fun U V W=>⟨weil_units_left_bilinear U V W,weil_units_right_bilinear U V W⟩
  · exact fun U V z w=>⟨class_left_bilinear U V z,class_right_bilinear U z w⟩

#print axioms concrete_miller_audit_certificate
end PairingMillerCertificateAudit
