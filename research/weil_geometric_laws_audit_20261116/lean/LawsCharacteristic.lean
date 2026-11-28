import LawsInputs

namespace WeilGeometricLawsAudit
noncomputable section
open WeierstrassCurve.Affine
open WeilDivisorChoiceAudit
variable {K : Type*} [Field K]

/-- In characteristic p the actual p-th root-of-unity group of field points is trivial. -/
theorem characteristic_root_one (p : ℕ) [Fact p.Prime] [CharP K p]
    (u : Kˣ) (hu : u^p=1) : u=1 := by
  have hv : (u : K)^p=1 := by
    have h:=congrArg (fun z : Kˣ=>(z : K)) hu
    simpa only [Units.val_pow_eq_pow_val,Units.val_one] using h
  have hz : ((u : K)-1)^p=0 := by rw [sub_pow_char,hv,one_pow,sub_self]
  have he : (u : K)-1=0 := eq_zero_of_pow_eq_zero hz
  exact Units.ext (sub_eq_zero.mp he)

variable [DecidableEq K] [IsAlgClosed K]

theorem geometric_pairing_characteristic_trivial (p : ℕ) [Fact p.Prime] [CharP K p]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q : torsionPoints W (p : ℤ)) :
    geometricPairing W hΔ (p : ℤ) P Q=1 := by
  apply characteristic_root_one p
  simpa only [zpow_natCast] using geometric_pairing_root W hΔ (p : ℤ) P Q

#print axioms characteristic_root_one
#print axioms geometric_pairing_characteristic_trivial
end
end WeilGeometricLawsAudit
