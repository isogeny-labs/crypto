import TangentFrobenius

namespace ThreeTangentPairingAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General BKFrobeniusBridgeAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {F L K : Type*} [Field F] [Fintype F] [Field L] [Algebra F L]
  [DecidableEq F] [DecidableEq L] [IsAlgClosed L] [NeZero (3:L)]
  [Field K] [Fintype K] [Algebra F K] [Algebra K L] [IsScalarTower F K L] [DecidableEq K]

/-- Necessity uses no rational point-count divisibility hypothesis. -/
theorem full_three_torsion_forces_cyclotomic (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0)
    (k : ℕ) (hK : Fintype.card K=Fintype.card F^k)
    (h : ∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 →
      P∈Set.range (pointInclusion F K L W)) : 3 ∣ Fintype.card F^k-1 := by
  let b := actualGeometricBasis (L:=L) W hΔ
  have hh := actual_extension_forces_det_power W b k hK h
  rw [actual_general_three_determinant W hΔ b] at hh
  exact (cyclotomic_cast_iff _ k Fintype.card_pos).mp hh

#print axioms full_three_torsion_forces_cyclotomic
end
end ThreeTangentPairingAudit
