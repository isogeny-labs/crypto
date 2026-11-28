import S4Cross

namespace SemaevReassociationAudit
open Polynomial S4DegreeAudit RecursiveSemaevDegreeAudit
noncomputable section
variable {R : Type*} [CommRing R]

theorem actual_S4_rotate (W : WeierstrassCurve R) (x y z t : R) :
    s4Ring W.b₂ W.b₄ W.b₆ W.b₈ x y z t =
      s4Ring W.b₂ W.b₄ W.b₆ W.b₈ z x y t := by
  rw [actual_S4_middle_swap W x y z t, s4Ring_first_swap]

theorem actual_lastPolynomial_rotate (W : WeierstrassCurve R) (x y z : R) :
    lastPolynomial W.b₂ W.b₄ W.b₆ W.b₈ x y z =
      lastPolynomial W.b₂ W.b₄ W.b₆ W.b₈ z x y := by
  rw [lastPolynomial_eq_s4Ring, lastPolynomial_eq_s4Ring]
  have h := actual_S4_rotate (W.map (C : R →+* Polynomial R)) (C x) (C y) (C z) X
  simpa only [WeierstrassCurve.map_b₂, WeierstrassCurve.map_b₄,
    WeierstrassCurve.map_b₆, WeierstrassCurve.map_b₈] using h

theorem unrestricted_b_S4_counterexample :
    s4Ring (0 : ℚ) 0 0 1 0 1 2 3 ≠ s4Ring (0 : ℚ) 0 0 1 0 2 1 3 := by
  norm_num [s4Ring, quadratic_resultant_ring, resForm, qaRing, qbRing, qcRing]

theorem control_b_not_a_curve :
    ¬ ∃ W : WeierstrassCurve ℚ, W.b₂ = 0 ∧ W.b₄ = 0 ∧ W.b₆ = 0 ∧ W.b₈ = 1 := by
  rintro ⟨W,h2,h4,h6,h8⟩
  have h := W.b_relation
  rw [h2,h4,h6,h8] at h
  norm_num at h

def sourceBalanced (D : ℕ) (p : Polynomial R) (W : WeierstrassCurve R) :
    Polynomial (Polynomial (Polynomial R)) :=
  rightAssoc D (p.map tripleLift) (tripleLift W.b₂) (tripleLift W.b₄)
    (tripleLift W.b₆) (tripleLift W.b₈) formalZ formalX formalY

theorem balanced_source_order (D : ℕ) (p : Polynomial R) (W : WeierstrassCurve R) :
    balancedPolynomial D p W.b₂ W.b₄ W.b₆ W.b₈ = sourceBalanced D p W := by
  unfold balancedPolynomial sourceBalanced rightAssoc
  have h := actual_lastPolynomial_rotate (W.map tripleLift)
    (formalX (R := R)) (formalY (R := R)) (formalZ (R := R))
  simp only [WeierstrassCurve.map_b₂, WeierstrassCurve.map_b₄,
    WeierstrassCurve.map_b₆, WeierstrassCurve.map_b₈] at h
  rw [h]

theorem source_order_polynomial_reassociation [IsDomain R] (D : ℕ) (p : Polynomial R)
    (hp : p.natDegree ≤ D) (W : WeierstrassCurve R) :
    doubleStep D p W.b₂ W.b₄ W.b₆ W.b₈ = sourceBalanced D p W := by
  rw [doubleStep_polynomial_reassociation D p hp, balanced_source_order]

variable {K : Type*} [Field K]

theorem canonical_source_order_reassociation (W : WeierstrassCurve.Affine K) (n : ℕ) :
    recursivePolynomial W (n+2) = sourceBalanced (2^(n+1)) (recursivePolynomial W n)
      (W.map (coefficientEmbedding n)) := by
  rw [canonical_polynomial_reassociation]
  change balancedPolynomial (R := InputRing K n) (2^(n+1)) (recursivePolynomial W n)
    (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
    (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈) = _
  have h := balanced_source_order (2^(n+1)) (recursivePolynomial W n)
    (W.map (coefficientEmbedding n))
  rw [WeierstrassCurve.map_b₂, WeierstrassCurve.map_b₄,
    WeierstrassCurve.map_b₆, WeierstrassCurve.map_b₈] at h
  exact h

theorem actual_source_order_reassociation_with_control :
    (∀ (R : Type) [CommRing R] [IsDomain R] (D : ℕ) (p : Polynomial R),
      p.natDegree ≤ D → ∀ W : WeierstrassCurve R,
      doubleStep D p W.b₂ W.b₄ W.b₆ W.b₈ = sourceBalanced D p W) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (n : ℕ),
      recursivePolynomial W (n+2) = sourceBalanced (2^(n+1)) (recursivePolynomial W n)
        (W.map (coefficientEmbedding n))) ∧
    s4Ring (0 : ℚ) 0 0 1 0 1 2 3 ≠ s4Ring (0 : ℚ) 0 0 1 0 2 1 3 := by
  constructor
  · intro R _ _ D p hp W
    exact source_order_polynomial_reassociation D p hp W
  · constructor
    · intro K _ W n
      exact canonical_source_order_reassociation W n
    · exact unrestricted_b_S4_counterexample

#print axioms actual_source_order_reassociation_with_control

end
end SemaevReassociationAudit
