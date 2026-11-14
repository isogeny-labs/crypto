import CanonicalBridge

namespace SemaevReassociationAudit
open Polynomial S4DegreeAudit ResultantDegreeStepAudit RecursiveSemaevDegreeAudit
open ResultantInterchangeAudit RecursiveSemaevEvaluationAudit
noncomputable section
variable {R S : Type*} [CommRing R] [CommRing S]

def tripleEvalWith (f : R →+* S) (z x y : S) :
    Polynomial (Polynomial (Polynomial R)) →+* S := pairEval (eval₂RingHom f z) x y

theorem pairEval_with_C (f : R →+* S) (z : S) :
    pairEval (C.comp f) (C z) X = mapRingHom (eval₂RingHom f z) := by
  ext a <;> simp [pairEval, RingHom.comp_apply]

theorem step_map_eval_with (f : R →+* S) (D : ℕ) (p : Polynomial R)
    (b2 b4 b6 b8 : R) (z : S) :
    (resultantStep D p b2 b4 b6 b8).map (eval₂RingHom f z) =
      Polynomial.resultant (lift (p.map f))
        (s3Bi (f b2) (f b4) (f b6) (f b8) z) D 2 := by
  have h := resultantStep_pairEval (C.comp f) (C z) X D p b2 b4 b6 b8
  rw [pairEval_with_C] at h
  change (resultantStep D p b2 b4 b6 b8).map (eval₂RingHom f z) = _ at h
  change (resultantStep D p b2 b4 b6 b8).map (eval₂RingHom f z) =
    Polynomial.resultant (p.map (C.comp f)) (s3Bi (f b2) (f b4) (f b6) (f b8) z) D 2 at h
  rw [lift, Polynomial.map_map]
  exact h

theorem eval₂Hom_C (f : R →+* S) (z : S) (a : R) :
    (eval₂RingHom f z) (C a) = f a := by simp

theorem doubleStep_eval_with (f : R →+* S) (D : ℕ) (p : Polynomial R)
    (b2 b4 b6 b8 : R) (x y z : S) :
    tripleEvalWith f z x y (doubleStep D p b2 b4 b6 b8) =
      leftAssoc D (p.map f) (f b2) (f b4) (f b6) (f b8) x y z := by
  unfold tripleEvalWith doubleStep
  rw [resultantStep_pairEval, step_map_eval_with]
  rw [eval₂Hom_C f z b2, eval₂Hom_C f z b4, eval₂Hom_C f z b6, eval₂Hom_C f z b8]
  rfl

def tripleLift : R →+* Polynomial (Polynomial (Polynomial R)) := C.comp scalarLift
def formalZ : Polynomial (Polynomial (Polynomial R)) := C (C X)
def formalX : Polynomial (Polynomial (Polynomial R)) := C X
def formalY : Polynomial (Polynomial (Polynomial R)) := X

theorem generic_tripleEval_identity :
    tripleEvalWith (tripleLift : R →+* Polynomial (Polynomial (Polynomial R)))
      formalZ formalX formalY = RingHom.id (Polynomial (Polynomial (Polynomial R))) := by
  ext a <;> simp [tripleEvalWith, pairEval, tripleLift, scalarLift,
    formalZ, formalX, formalY, RingHom.comp_apply]

def balancedPolynomial (D : ℕ) (p : Polynomial R) (b2 b4 b6 b8 : R) :
    Polynomial (Polynomial (Polynomial R)) :=
  rightAssoc D (p.map tripleLift) (tripleLift b2) (tripleLift b4)
    (tripleLift b6) (tripleLift b8) formalX formalY formalZ

theorem generic_balanced_identity [IsDomain R] (D : ℕ) (p : Polynomial R)
    (hp : p.natDegree ≤ D) (b2 b4 b6 b8 : R) :
    leftAssoc D (p.map tripleLift) (tripleLift b2) (tripleLift b4)
      (tripleLift b6) (tripleLift b8) formalX formalY formalZ =
      balancedPolynomial D p b2 b4 b6 b8 := by
  unfold balancedPolynomial
  apply reassociation_prescribed
  exact natDegree_map_le.trans hp

/-- Equality of the actual formal polynomials, not only their values or zero sets. -/
theorem doubleStep_polynomial_reassociation [IsDomain R] (D : ℕ) (p : Polynomial R)
    (hp : p.natDegree ≤ D) (b2 b4 b6 b8 : R) :
    doubleStep D p b2 b4 b6 b8 = balancedPolynomial D p b2 b4 b6 b8 := by
  have h := doubleStep_eval_with (R := R) (S := Polynomial (Polynomial (Polynomial R)))
    (tripleLift (R := R)) D p b2 b4 b6 b8
    (formalX (R := R)) (formalY (R := R)) (formalZ (R := R))
  rw [generic_tripleEval_identity, RingHom.id_apply] at h
  rw [h]
  exact generic_balanced_identity D p hp b2 b4 b6 b8

variable {K : Type*} [Field K]

theorem canonical_polynomial_reassociation (W : WeierstrassCurve.Affine K) (n : ℕ) :
    recursivePolynomial W (n+2) = balancedPolynomial (2^(n+1)) (recursivePolynomial W n)
      (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
      (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈) := by
  rw [canonical_doubleStep]
  exact doubleStep_polynomial_reassociation _ _ (recursivePolynomial_natDegree W n).le ..

theorem actual_polynomial_reassociation_certificate :
    (∀ (R : Type) [CommRing R] [IsDomain R] (D : ℕ) (p : Polynomial R),
      p.natDegree ≤ D → ∀ b2 b4 b6 b8 : R,
      doubleStep D p b2 b4 b6 b8 = balancedPolynomial D p b2 b4 b6 b8) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (n : ℕ),
      recursivePolynomial W (n+2) = balancedPolynomial (2^(n+1)) (recursivePolynomial W n)
        (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
        (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈)) := by
  constructor
  · intro R _ _ D p hp b2 b4 b6 b8
    exact doubleStep_polynomial_reassociation D p hp b2 b4 b6 b8
  · intro K _ W n
    exact canonical_polynomial_reassociation W n

#print axioms actual_polynomial_reassociation_certificate

end
end SemaevReassociationAudit
