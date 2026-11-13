import PrescribedDegree

namespace SemaevReassociationAudit
open Polynomial S4DegreeAudit ResultantDegreeStepAudit RecursiveSemaevDegreeAudit
open ResultantInterchangeAudit RecursiveSemaevEvaluationAudit
noncomputable section
variable {R : Type*} [CommRing R]

theorem pairEval_C_eq_map (z : R) :
    pairEval (C : R →+* Polynomial R) (C z) X = mapRingHom (evalRingHom z) := by
  ext a <;> simp [pairEval]

theorem step_map_eval (D : ℕ) (p : Polynomial R) (b2 b4 b6 b8 z : R) :
    (resultantStep D p b2 b4 b6 b8).map (evalRingHom z) =
      Polynomial.resultant (lift p) (s3Bi b2 b4 b6 b8 z) D 2 := by
  have h := step_specialization p D b2 b4 b6 b8 z
  rw [pairEval_C_eq_map] at h
  exact h

/-- Exactly two existing resultantStep operations, introducing three variables. -/
def doubleStep (D : ℕ) (p : Polynomial R) (b2 b4 b6 b8 : R) :
    Polynomial (Polynomial (Polynomial R)) :=
  resultantStep (D*2) (resultantStep D p b2 b4 b6 b8) (C b2) (C b4) (C b6) (C b8)

def tripleEval (z x y : R) : Polynomial (Polynomial (Polynomial R)) →+* R :=
  pairEval (evalRingHom z) x y

theorem doubleStep_eval (D : ℕ) (p : Polynomial R) (b2 b4 b6 b8 x y z : R) :
    tripleEval z x y (doubleStep D p b2 b4 b6 b8) =
      leftAssoc D p b2 b4 b6 b8 x y z := by
  unfold doubleStep tripleEval
  rw [resultantStep_pairEval, step_map_eval]
  rw [evalHom_C z b2, evalHom_C z b4, evalHom_C z b6, evalHom_C z b8]
  rfl

theorem doubleStep_reassociation [IsDomain R] (D : ℕ) (p : Polynomial R)
    (hp : p.natDegree ≤ D) (b2 b4 b6 b8 x y z : R) :
    tripleEval z x y (doubleStep D p b2 b4 b6 b8) =
      rightAssoc D p b2 b4 b6 b8 x y z := by
  rw [doubleStep_eval]
  exact reassociation_prescribed p D hp b2 b4 b6 b8 x y z

variable {K : Type*} [Field K]

theorem canonical_doubleStep (W : WeierstrassCurve.Affine K) (n : ℕ) :
    recursivePolynomial W (n+2) = doubleStep (2^(n+1)) (recursivePolynomial W n)
      (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
      (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈) := by
  change recursivePolynomial W ((n+1)+1) = _
  rw [recursivePolynomial_succ, recursivePolynomial_succ]
  change resultantStep (2^((n+1)+1)) (resultantStep (2^(n+1)) (recursivePolynomial W n)
      (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
      (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈))
      (C (coefficientEmbedding n W.b₂)) (C (coefficientEmbedding n W.b₄))
      (C (coefficientEmbedding n W.b₆)) (C (coefficientEmbedding n W.b₈)) = _
  rw [pow_succ]
  rfl

def canonicalTriple (W : WeierstrassCurve.Affine K) (n : ℕ)
    (x y z : InputRing K n) : InputRing K n :=
  tripleEval z x y (recursivePolynomial W (n+2))

def canonicalBalanced (W : WeierstrassCurve.Affine K) (n : ℕ)
    (x y z : InputRing K n) : InputRing K n :=
  rightAssoc (2^(n+1)) (recursivePolynomial W n)
    (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
    (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈) x y z

theorem canonical_reassociation (W : WeierstrassCurve.Affine K) (n : ℕ)
    (x y z : InputRing K n) : canonicalTriple W n x y z = canonicalBalanced W n x y z := by
  unfold canonicalTriple canonicalBalanced
  rw [canonical_doubleStep]
  exact doubleStep_reassociation _ _ (recursivePolynomial_natDegree W n).le ..

theorem actual_S3_reassociation_certificate :
    (∀ (R : Type) [CommRing R] [IsDomain R] (D : ℕ) (p : Polynomial R),
      p.natDegree ≤ D → ∀ b2 b4 b6 b8 x y z : R,
      tripleEval z x y (doubleStep D p b2 b4 b6 b8) =
        rightAssoc D p b2 b4 b6 b8 x y z) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (n : ℕ)
      (x y z : InputRing K n), canonicalTriple W n x y z = canonicalBalanced W n x y z) := by
  constructor
  · intro R _ _ D p hp b2 b4 b6 b8 x y z
    exact doubleStep_reassociation D p hp b2 b4 b6 b8 x y z
  · intro K _ W n x y z
    exact canonical_reassociation W n x y z

#print axioms actual_S3_reassociation_certificate

end
end SemaevReassociationAudit
