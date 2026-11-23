import RecursiveSemaevEvaluation
import SemaevPointLists

/-! All-arity geometric meaning of the actual canonical resultant family.
The points, coordinate witnesses and group sums are Mathlib curve points. -/
namespace RecursiveSemaevGeometryAudit
noncomputable section
open Polynomial SemaevS3 GeneralS3Audit GeneralS4Audit
open RecursiveSemaevEvaluationAudit SemaevPointListsAudit
open scoped BigOperators
variable {K : Type*} [Field K] [DecidableEq K]

theorem manyPoints_three_iff (W : WeierstrassCurve.Affine K) (xs : Fin 3→K) :
    ManyPoints W xs ↔ ∃ P Q R : W.Point,pointX P=some (xs 0) ∧
      pointX Q=some (xs 1) ∧ pointX R=some (xs 2) ∧ P+Q+R=0 := by
  constructor
  · rintro ⟨P,hP,hs⟩
    exact ⟨P 0,P 1,P 2,hP 0,hP 1,hP 2,
      by simpa [Fin.sum_univ_succ,add_assoc] using hs⟩
  · rintro ⟨P,Q,R,hP,hQ,hR,hs⟩
    refine ⟨![P,Q,R],?_,?_⟩
    · intro i
      fin_cases i
      · exact hP
      · exact hQ
      · exact hR
    · simpa [Fin.sum_univ_succ,add_assoc] using hs

theorem manyPoints_four_iff (W : WeierstrassCurve.Affine K) (xs : Fin 4→K) :
    ManyPoints W xs ↔ fourPoints W (xs 0) (xs 1) (xs 2) (xs 3) := by
  constructor
  · rintro ⟨P,hP,hs⟩
    exact ⟨P 0,P 1,P 2,P 3,hP 0,hP 1,hP 2,hP 3,
      by simpa [Fin.sum_univ_succ,add_assoc] using hs⟩
  · rintro ⟨P,Q,R,S,hP,hQ,hR,hS,hs⟩
    refine ⟨![P,Q,R,S],?_,?_⟩
    · intro i
      fin_cases i
      · exact hP
      · exact hQ
      · exact hR
      · exact hS
    · simpa [Fin.sum_univ_succ,add_assoc] using hs

omit [Field K] [DecidableEq K] in
theorem tuple_two_snoc {n : ℕ} (xs : Fin (n+2)→K) :
    xs=Fin.snoc (Fin.snoc (Fin.init (Fin.init xs))
      (xs (Fin.last n).castSucc)) (xs (Fin.last (n+1))) := by
  funext i
  refine Fin.lastCases ?_ (fun j=>?_) i
  · simp
  · refine Fin.lastCases ?_ (fun l=>?_) j
    · simp
    · simp only [Fin.snoc_castSucc]
      rfl

theorem canonical_zero_iff [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℕ) :
    ∀ xs : Fin (n+3)→K,evalCanonical W n xs=0 ↔ ManyPoints W xs := by
  induction n using Nat.strong_induction_on with
  | h n ih =>
    rcases n with _ | n
    · intro xs
      rw [evalCanonical_three,manyPoints_three_iff]
      exact generalS3_zero_iff_exists_points W hΔ _ _ _
    rcases n with _ | n
    · intro xs
      rw [evalCanonical_four,manyPoints_four_iff]
      exact generalS4_zero_iff_exists_points W hΔ _ _ _ _
    · intro xs
      rw [tuple_two_snoc xs,canonical_two_point_recurrence,
        two_point_recurrence W hΔ]
      simp only [ih (n+1) (by omega),ih n (by omega)]

theorem all_arity_geometric_audit_certificate :
    ∀ (F : Type*) [Field F] [DecidableEq F] [IsAlgClosed F]
      (W : WeierstrassCurve.Affine F),W.Δ≠0 →
      ∀ (n : ℕ) (xs : Fin (n+3)→F),
        evalCanonical W n xs=0 ↔
          ∃ P : Fin (n+3)→W.Point,
            (∀ i,pointX (P i)=some (xs i)) ∧ (∑ i,P i)=0 := by
  intro F _ _ _ W hΔ n xs
  exact canonical_zero_iff W hΔ n xs

private instance primeFiveControl : Fact (Nat.Prime 5) := ⟨by decide⟩

theorem rational_zero_no_decomposition :
    evalCanonical SkepticalAudit.rationalCurve 0 ![3,1,4]=0 ∧
      ¬ ManyPoints SkepticalAudit.rationalCurve ![3,1,4] := by
  constructor
  · rw [evalCanonical_three]
    decide +kernel
  · rintro ⟨P,hP,_⟩
    exact SkepticalAudit.no_rational_point_above_three ⟨P 0,hP 0⟩

theorem rational_only_allarity_false :
    ¬ ∀ (n : ℕ) (xs : Fin (n+3)→ZMod 5),
      evalCanonical SkepticalAudit.rationalCurve n xs=0 →
        ManyPoints SkepticalAudit.rationalCurve xs := by
  intro h
  exact rational_zero_no_decomposition.2
    (h 0 ![3,1,4] rational_zero_no_decomposition.1)

theorem geometric_domain_audit_certificate :
    (∀ (F : Type*) [Field F] [DecidableEq F] [IsAlgClosed F]
      (W : WeierstrassCurve.Affine F),W.Δ≠0 →
      ∀ (n : ℕ) (xs : Fin (n+3)→F),
        evalCanonical W n xs=0 ↔
          ∃ P : Fin (n+3)→W.Point,
            (∀ i,pointX (P i)=some (xs i)) ∧ (∑ i,P i)=0) ∧
    (SkepticalAudit.rationalCurve.Δ≠0 ∧
      ¬ ∀ (n : ℕ) (xs : Fin (n+3)→ZMod 5),
        evalCanonical SkepticalAudit.rationalCurve n xs=0 →
          ManyPoints SkepticalAudit.rationalCurve xs) := by
  exact ⟨all_arity_geometric_audit_certificate,
    SkepticalAudit.rationalCurve_nonsingular,rational_only_allarity_false⟩

#print axioms canonical_zero_iff
#print axioms all_arity_geometric_audit_certificate
#print axioms rational_only_allarity_false
#print axioms geometric_domain_audit_certificate
end
end RecursiveSemaevGeometryAudit
