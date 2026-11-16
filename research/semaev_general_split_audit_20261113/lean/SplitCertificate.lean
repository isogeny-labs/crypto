import LastVariable
import DegreeDropControl

namespace SemaevGeneralSplitAudit
open Polynomial SemaevSymmetryAudit SemaevReassociationAudit ResultantInterchangeAudit
open S4DegreeAudit
noncomputable section

theorem source_index_coverage (N k : ℕ) (hN : 4≤N) (hk : 1≤k) (hmax : k≤N-3) :
    ∃ m n : ℕ,N=(m+2)+(n+2) ∧ k=n+1 := by
  exact ⟨N-k-3,k-1,by omega,by omega⟩

/-- All original permitted source splits, with literal canonical polynomials,
source variable ordering and proved natural dimensions; specialization and
negative controls are separate explicit conjuncts. -/
theorem actual_all_source_splits_certificate :
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (m n : ℕ),
      (leftLastFactor W m n).natDegree=2^(m+1) ∧
      (rightLastFactor W m n).natDegree=2^(n+1) ∧
      Polynomial.resultant (leftLastFactor W m n) (rightLastFactor W m n)
        (leftLastFactor W m n).natDegree (rightLastFactor W m n).natDegree=
        MvPolynomial.rename (arityCast ((m+2)+(n+2)) (blockArityProof m n))
          (canonicalMv W (((m+2)+(n+2))-3))) ∧
    (∀ N k : ℕ,4≤N → 1≤k → k≤N-3 → ∃ m n : ℕ,N=(m+2)+(n+2) ∧ k=n+1) ∧
    (∀ (K S : Type) [Field K] [CommRing S] [IsDomain S]
      (W : WeierstrassCurve.Affine K) (f : K →+* S)
      (xs ys : List S) (hx : 2≤xs.length) (hy : 2≤ys.length),
      Polynomial.resultant (listKernel W f xs hx) (listKernel W f ys hy)
        (kernelSize xs) (kernelSize ys)=joinedValue W f xs ys hx hy) ∧
    (leftNatural controlP controlQ controlR 1≠rightNatural controlP controlQ controlR 1 ∧
      leftFixed controlP controlQ controlR 1 1=rightFixed controlP controlQ controlR 1 1) ∧
    s4Ring (0 : ℚ) 0 0 1 0 1 2 3 ≠ s4Ring (0 : ℚ) 0 0 1 0 2 1 3 ∧
    ¬ ∃ W : WeierstrassCurve ℚ,W.b₂=0 ∧ W.b₄=0 ∧ W.b₆=0 ∧ W.b₈=1 := by
  refine ⟨?_,source_index_coverage,?_,natural_interchange_counterexample,
    unrestricted_b_S4_counterexample,control_b_not_a_curve⟩
  · intro K _ W m n
    refine ⟨?_,?_,source_order_natural_split W m n⟩
    · rw [leftLastFactor_eq,leftFactor_degree]
    · rw [rightLastFactor_eq,rightFactor_degree]
  · intro K S _ _ _ W f xs ys hx hy
    exact all_list_splits W f ys xs hx hy

#print axioms actual_all_source_splits_certificate
end
end SemaevGeneralSplitAudit
