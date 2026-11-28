import ActualPointMaps

namespace BKFullBindingAudit
noncomputable section
open BKActualThreeAudit FrobeniusPointDescentAudit

private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance fullGaloisDecidableEq (k : ℕ) : DecidableEq (GaloisField 5 k) :=
  Classical.decEq _

/- This deliberately writes the field telescope in the theorem itself.
   The type uses no named-argument assignment or absolute-value delimiter.
   Its content is the unchanged actual F₅ certificate, via explicit maps. -/
theorem full_bk_certificate :
    ∀ (L : Type*) [Field L] [Algebra F L] [DecidableEq L] [IsAlgClosed L],
    curve.Δ ≠ 0 ∧ Nat.card curve.Point = 6 ∧ Nat.Prime 3 ∧
    Nat.Coprime 3 5 ∧ 3 ∣ Nat.card curve.Point ∧ ¬3 ∣ 5 - 1 ∧
    (∀ k : ℕ,
      (∀ P : (curve.baseChange L).Point, (3 : ℕ) • P = 0 →
        (pointFrobenius curve)^[k] P = P) ↔ 3 ∣ 5 ^ k - 1) ∧
    (∃ P : (curve.baseChange L).Point, addOrderOf P = 3 ∧
      pointFrobenius curve P = -P ∧ pointFrobenius curve P ≠ P ∧
      ¬∃ Q : curve.Point, rationalPointMap L Q = P) ∧
    (∀ (K : Type*) [Field K] [Fintype K] [Algebra F K] [Algebra K L]
      [IsScalarTower F K L] [DecidableEq K] (k : ℕ), Fintype.card K = 5 ^ k →
      ((∀ P : (curve.baseChange L).Point, (3 : ℕ) • P = 0 →
        P ∈ Set.range (extensionPointMap L K)) ↔ 3 ∣ 5 ^ k - 1)) ∧
    (∀ k : ℕ, 0 < k → Nat.card (GaloisField 5 k) = 5 ^ k ∧
      ∃ ι : GaloisField 5 k →ₐ[F] L,
        (∀ P : (curve.baseChange L).Point, (3 : ℕ) • P = 0 →
          P ∈ Set.range (galoisPointMap L k ι)) ↔ 3 ∣ 5 ^ k - 1) := by
  intro L _ _ _ _
  obtain ⟨hn, hc, hp, hcop, hd, hex, hf, hw, he, hg⟩ :=
    BKActualCertificateAudit.bk_actual_case_audit_certificate (L := L)
  refine ⟨hn, hc, hp, hcop, hd, hex, hf, hw, ?_, ?_⟩
  · intro K _ _ _ _ _ _ k hk
    exact he K k hk
  · intro k hk
    exact hg k hk

#print axioms full_bk_certificate

end
end BKFullBindingAudit
