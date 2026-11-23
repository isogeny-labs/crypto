import CryptoLean.Polynomial.ConstructibleNormalForm
import CryptoResearch.FallDegree.LastFall

/-! Apply the new generic law to the pre-existing V and lastFallDegree.
The tower's NF construction and its contracts are explicitly NOT proved here. -/
namespace VietaSpeedAudit
open MvPolynomial CryptoResearch.FallDegree
variable {σ k : Type*} [Field k]

theorem lastFallDegree_le_of_normalForm
    (F : Set (MvPolynomial σ k)) (T : ℕ)
    (nf : MvPolynomial σ k →ₗ[k] MvPolynomial σ k)
    (hdeg : ∀ p, (nf p).totalDegree < T)
    (hone : (1 : MvPolynomial σ k) - nf 1 ∈ V F T)
    (hact : ∀ p i, nf p * X i - nf (nf p * X i) ∈ V F T)
    (hcompat : ∀ p i, nf (nf p * X i) = nf (p * X i))
    (hker : ∀ p ∈ Ideal.span F, nf p = 0) : lastFallDegree F ≤ T := by
  apply Nat.sInf_le
  intro p hp
  have h := CryptoLean.Polynomial.normalForm_constructible_bound
    (V F) (fun _ _ hij => V_mono F hij)
    (fun {_} {_ _} hf hd => mul_mem_V hf hd) T nf hdeg hone hact hcompat p
  simpa only [hker p hp, sub_zero] using h

#print axioms lastFallDegree_le_of_normalForm
end VietaSpeedAudit
