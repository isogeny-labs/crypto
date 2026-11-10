import ShortPointCount
import MissingFiber
import CardinalityControls

namespace AnomalousCardinalityAudit
open AnomalousPrimeAudit AnomalousCurveDataAudit AnomalousPointTraceAudit
open ShortPointCountAudit AnomalousMissingFiberAudit

theorem actual_point_cardinality : Nat.card formulaCurve.Point=p := by
  have hd : p∣Nat.card formulaCurve.Point := by
    have h := addOrderOf_dvd_natCard G
    simpa only [G_order] using h
  have hb : Nat.card formulaCurve.Point<2*p := by
    have h := cardinality_lt_twice_field
      (425706413842211054102700238164133538302169176474:F) constantB (0:F)
      (by simpa using missing_zero_fiber)
    simpa [shortCurve,formulaCurve,constantB,F,ZMod.card] using h
  let : Finite (WithZero {xy : F × F // formulaCurve.Nonsingular xy.1 xy.2}) :=
    inferInstanceAs (Finite (Option {xy : F × F // formulaCurve.Nonsingular xy.1 xy.2}))
  let : Finite formulaCurve.Point := Finite.of_injective formulaCurve.nonsingularPointEquiv
    formulaCurve.nonsingularPointEquiv.injective
  exact multiple_below_twice_eq p (Nat.card formulaCurve.Point) Nat.card_pos hd hb

theorem anomalous_curve_cardinality_certificate :
    Nat.Prime p ∧ 2^159≤p ∧ p<2^160 ∧ formulaCurve.Δ≠0 ∧
    Nat.card formulaCurve.Point=p ∧ addOrderOf G=p ∧ knownScalar • G=Q ∧
    ¬ (∀ (q : ℕ) [Fact (Nat.Prime q)] (E : WeierstrassCurve.Affine (ZMod q))
      (P : E.Point), E.Δ≠0 → addOrderOf P=q → Nat.card E.Point=q) := by
  exact ⟨p_prime,p_bit_bounds.1,p_bit_bounds.2,formula_discriminant_ne_zero,
    actual_point_cardinality,G_order,known_answer_relation,
    CardinalityControlAudit.unrestricted_point_order_forces_cardinality_false⟩

#print axioms actual_point_cardinality
#print axioms anomalous_curve_cardinality_certificate
end AnomalousCardinalityAudit
