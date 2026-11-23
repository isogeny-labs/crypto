import MixedNondegenerate

namespace WeilMixedLevelAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

theorem geometric_pairing_nsmul_left (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (m : ℤ) (P Q : torsionPoints W m) (n : ℕ) :
    geometricPairing W hΔ m (n • P) Q=(geometricPairing W hΔ m P Q)^n := by
  induction n with
  | zero => rw [zero_nsmul,geometric_pairing_zero_left,pow_zero]
  | succ n ih => rw [succ_nsmul,geometric_pairing_add_left,ih,pow_succ]

/-- Characteristic torsion is in the actual radical even at a larger ambient level. -/
theorem characteristic_torsion_radical (p : ℕ) [Fact p.Prime] [CharP K p]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (m : ℤ)
    (P : torsionPoints W m) (hp : (p : ℕ) • P=0) (Q : torsionPoints W m) :
    geometricPairing W hΔ m P Q=1 := by
  apply characteristic_root_one p
  have he:=geometric_pairing_nsmul_left W hΔ m P Q p
  rw [hp,geometric_pairing_zero_left] at he
  exact he.symm

#print axioms geometric_pairing_nsmul_left
#print axioms characteristic_torsion_radical
end
end WeilMixedLevelAudit
