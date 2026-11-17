import JordanException
import RationalTorsion

namespace BKLinearAudit
open Matrix Polynomial
noncomputable section

/-- Direct negation retains determinant and nonzero fixed-vector hypotheses;
only the source condition q ≠ 1 is deleted. -/
theorem dropping_distinct_eigenvalues_false :
    ¬ (∀ (K : Type) [Field K] (A : M2 K) (q : K), A.det=q →
      ∀ v : Fin 2 → K, v≠0 → A*ᵥv=v → ∀ k : ℕ,A^k=1 ↔ q^k=1) := by
  have hp : Nat.Prime 2 := by decide +kernel
  have : Fact (Nat.Prime 2) := ⟨hp⟩
  intro h
  apply (jordan_prime_boundary 2 hp).2.2.2
  exact h (ZMod 2) (jordan 1) 1 (jordan_det 1) ![1,0]
    first_vector_ne_zero (jordan_fixed_vector 1)

/-- The entire source linear step and its exact excluded Jordan boundary.
Generic elliptic torsion rank, Frobenius determinant and descent are not asserted. -/
theorem qualified_BK_linear_certificate :
    (∀ (K : Type) [Field K] [Finite K] [DecidableEq K]
      (W : WeierstrassCurve.Affine K) (r : ℕ),r.Prime → r∣Nat.card W.Point →
        ∃ P : W.Point,P≠0 ∧ addOrderOf P=r ∧ r • P=0) ∧
    (∀ (K : Type) [Field K] (A : M2 K) (q : K), q≠1 → A.det=q →
      ∀ v : Fin 2 → K, v≠0 → A*ᵥv=v →
        A.trace=1+q ∧ A.charpoly=(X-C 1)*(X-C q) ∧
        ∀ k : ℕ,A^k=1 ↔ q^k=1) ∧
    (∀ r : ℕ,r.Prime →
      (∀ k : ℕ,(1:ZMod r)^k=1 ∧ (jordan (1:ZMod r)^k=1 ↔ r∣k)) ∧
      (jordan (1:ZMod r)).det=1 ∧
      ((![1,0] : Fin 2 → ZMod r)≠0 ∧ jordan (1:ZMod r)*ᵥ ![1,0]= ![1,0]) ∧
      ¬ (∀ k : ℕ,jordan (1:ZMod r)^k=1 ↔ (1:ZMod r)^k=1)) ∧
    ¬ (∀ (K : Type) [Field K] (A : M2 K) (q : K), A.det=q →
      ∀ v : Fin 2 → K, v≠0 → A*ᵥv=v → ∀ k : ℕ,A^k=1 ↔ q^k=1) := by
  refine ⟨?_,?_,jordan_prime_boundary,dropping_distinct_eigenvalues_false⟩
  · intro K _ _ _ W r hr hdiv
    exact actual_rational_prime_torsion W r hr hdiv
  · intro K _ A q hq hdet v hv hfix
    exact ⟨trace_from_fixed_vector A q hdet v hv hfix,
      charpoly_from_fixed_vector A q hdet v hv hfix,
      power_identity_iff A q hq hdet v hv hfix⟩

#print axioms qualified_BK_linear_certificate
end
end BKLinearAudit
