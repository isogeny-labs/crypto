import GaloisFieldCase

namespace BKActualCertificateAudit
noncomputable section
open BKActualThreeAudit BKActualEmbeddingAudit FrobeniusPointDescentAudit
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance galoisDecidableEq (k : ℕ) : DecidableEq (GaloisField 5 k) :=
  Classical.decEq _
variable {L : Type*} [Field L] [Algebra F L] [DecidableEq L] [IsAlgClosed L]

theorem bk_actual_case_audit_certificate :
    curve.Δ≠0 ∧ Nat.card curve.Point=6 ∧ Nat.Prime 3 ∧
    Nat.Coprime 3 5 ∧ 3∣Nat.card curve.Point ∧ ¬3∣5-1 ∧
    (∀ k : ℕ,
      (∀ P : (geomCurve (L:=L)).Point,(3:ℕ) • P=0 →
        (pointFrobenius curve)^[k] P=P) ↔ 3∣5^k-1) ∧
    (∃ P : (geomCurve (L:=L)).Point,addOrderOf P=3 ∧
      pointFrobenius curve P= -P ∧ pointFrobenius curve P≠P ∧
      ¬∃ Q : curve.Point,
        WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId F L) Q=P) ∧
    (∀ (K : Type*) [Field K] [Fintype K] [Algebra F K] [Algebra K L]
      [IsScalarTower F K L] [DecidableEq K] (k : ℕ),Fintype.card K=5^k →
      ((∀ P : (geomCurve (L:=L)).Point,(3:ℕ) • P=0 →
        P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId K L))) ↔
          3∣5^k-1)) ∧
    (∀ k : ℕ,0<k → Nat.card (GaloisField 5 k)=5^k ∧
      ∃ ι : GaloisField 5 k→ₐ[F]L,
        (∀ P : (geomCurve (L:=L)).Point,(3:ℕ) • P=0 →
          P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=curve) ι)) ↔ 3∣5^k-1) := by
  refine ⟨SkepticalAudit.rationalCurve_nonsingular,
    TorsionBoundaryAudit.small_curve_cardinality,by decide,by decide,?_,by decide,?_,?_,?_,?_⟩
  · rw [TorsionBoundaryAudit.small_curve_cardinality]
    decide
  · exact actual_frobenius_criterion
  · exact ⟨BKActualWitnessAudit.P,BKActualWitnessAudit.P_order,
      BKActualWitnessAudit.P_frobenius_neg,BKActualWitnessAudit.P_not_fixed,
      BKActualWitnessAudit.P_not_rational⟩
  · intro K _ _ _ _ _ _ k hc
    exact actual_extension_torsion_criterion k hc
  · intro k hk
    exact ⟨GaloisField.card 5 k (Nat.ne_of_gt hk),
      BKActualGaloisFieldAudit.actual_galois_field_criterion k hk⟩

#print axioms bk_actual_case_audit_certificate
end
end BKActualCertificateAudit
