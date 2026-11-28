import RecursiveSemaevGeometry

/-!
# Simultaneous-scale specialization of the canonical Semaev family

This module supplies the coefficient-ring naturality needed to turn the
existing nested canonical family into an honest univariate polynomial.  It
does not assert that the specialized polynomial is nonzero.
-/

namespace DilationSemaevBridge

noncomputable section

open Polynomial SemaevS3 RecursiveSemaevDegreeAudit RecursiveSemaevEvaluationAudit
open RecursiveSemaevGeometryAudit SemaevPointListsAudit
open scoped BigOperators

universe u v w

variable {K : Type u} [Field K]

/-- Evaluate every nested input of `InputRing K n` in an arbitrary
commutative target ring.  The existing `inputEval` is the target-`K` special
case. -/
def inputEvalTo {R : Type v} [CommRing R] (phi : K →+* R) :
    (n : ℕ) → (Fin (n + 2) → R) → InputRing K n →+* R
  | 0, xs =>
      Polynomial.eval₂RingHom (Polynomial.eval₂RingHom phi (xs 0)) (xs 1)
  | n + 1, xs =>
      Polynomial.eval₂RingHom (inputEvalTo phi n (Fin.init xs))
        (xs (Fin.last (n + 2)))

/-- Evaluation of the canonical Semaev polynomial in an arbitrary
commutative target ring. -/
def evalCanonicalTo {R : Type v} [CommRing R] (phi : K →+* R)
    (W : WeierstrassCurve.Affine K) (n : ℕ) (xs : Fin (n + 3) → R) : R :=
  Polynomial.eval₂ (inputEvalTo phi n (Fin.init xs))
    (xs (Fin.last (n + 2))) (recursivePolynomial W n)

theorem eval₂RingHom_comp {A : Type*} {R : Type v} {S : Type w}
    [CommSemiring A] [CommSemiring R] [CommSemiring S] (phi : A →+* R)
    (psi : R →+* S) (x : R) :
    psi.comp (Polynomial.eval₂RingHom phi x) =
      Polynomial.eval₂RingHom (psi.comp phi) (psi x) := by
  apply Polynomial.ringHom_ext
  · intro a
    simp [RingHom.comp_apply]
  · simp [RingHom.comp_apply]

set_option backward.isDefEq.respectTransparency false in
theorem inputEvalTo_natural {R : Type v} {S : Type w}
    [CommRing R] [CommRing S] (phi : K →+* R) (psi : R →+* S) :
    ∀ (n : ℕ) (xs : Fin (n + 2) → R),
      psi.comp (inputEvalTo phi n xs) =
        inputEvalTo (psi.comp phi) n (fun i => psi (xs i)) := by
  intro n
  induction n with
  | zero =>
      intro xs
      change psi.comp
        (Polynomial.eval₂RingHom (Polynomial.eval₂RingHom phi (xs 0)) (xs 1)) = _
      rw [eval₂RingHom_comp, eval₂RingHom_comp]
      rfl
  | succ n ih =>
      intro xs
      change psi.comp (Polynomial.eval₂RingHom
        (inputEvalTo phi n (Fin.init xs)) (xs (Fin.last (n + 2)))) = _
      rw [eval₂RingHom_comp, ih]
      rfl

set_option backward.isDefEq.respectTransparency false in
theorem inputEvalTo_id :
    ∀ (n : ℕ) (xs : Fin (n + 2) → K),
      inputEvalTo (RingHom.id K) n xs = inputEval n xs := by
  intro n
  induction n with
  | zero =>
      intro xs
      rfl
  | succ n ih =>
      intro xs
      simp only [inputEvalTo, inputEval]
      rw [ih]

theorem evalCanonicalTo_id (W : WeierstrassCurve.Affine K) (n : ℕ)
    (xs : Fin (n + 3) → K) :
    evalCanonicalTo (RingHom.id K) W n xs = evalCanonical W n xs := by
  simp only [evalCanonicalTo, evalCanonical, inputEvalTo_id]

set_option backward.isDefEq.respectTransparency false in
theorem evalCanonicalTo_natural {R : Type v} {S : Type w}
    [CommRing R] [CommRing S] (phi : K →+* R) (psi : R →+* S)
    (W : WeierstrassCurve.Affine K) (n : ℕ) (xs : Fin (n + 3) → R) :
    psi (evalCanonicalTo phi W n xs) =
      evalCanonicalTo (psi.comp phi) W n (fun i => psi (xs i)) := by
  simp only [evalCanonicalTo]
  rw [Polynomial.hom_eval₂, inputEvalTo_natural]
  rfl

/-- The actual simultaneous-scale specialization
`s ↦ S_(n+3)(s*t₀, ..., s*t_(n+2))`. -/
def simultaneousScalePolynomial (W : WeierstrassCurve.Affine K) (n : ℕ)
    (t : Fin (n + 3) → K) : Polynomial K :=
  evalCanonicalTo Polynomial.C W n (fun i => Polynomial.C (t i) * Polynomial.X)

theorem simultaneousScalePolynomial_eval (W : WeierstrassCurve.Affine K)
    (n : ℕ) (t : Fin (n + 3) → K) (s : K) :
    (simultaneousScalePolynomial W n t).eval s =
      evalCanonical W n (fun i => s * t i) := by
  change (Polynomial.evalRingHom s)
      (evalCanonicalTo Polynomial.C W n
        (fun i => Polynomial.C (t i) * Polynomial.X)) = _
  rw [evalCanonicalTo_natural]
  have hcomp : (Polynomial.evalRingHom s).comp
      (Polynomial.C : K →+* Polynomial K) = RingHom.id K := by
    exact Polynomial.eval₂RingHom_comp_C (RingHom.id K) s
  rw [hcomp, evalCanonicalTo_id]
  congr 1
  funext i
  change (Polynomial.C (t i) * Polynomial.X).eval s = s * t i
  rw [Polynomial.eval_mul, Polynomial.eval_C, Polynomial.eval_X]
  exact mul_comm _ _

/-- Over an algebraically closed coefficient field, an actual zero-sum tuple
with the simultaneously scaled x-coordinates gives a root of the specialized
univariate polynomial. -/
theorem simultaneousScalePolynomial_vanishes_of_points [DecidableEq K]
    [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (hDelta : W.Δ ≠ 0)
    (n : ℕ) (t : Fin (n + 3) → K) (s : K)
    (P : Fin (n + 3) → W.Point)
    (hX : ∀ i, pointX (P i) = some (s * t i))
    (hSum : ∑ i, P i = 0) :
    (simultaneousScalePolynomial W n t).eval s = 0 := by
  rw [simultaneousScalePolynomial_eval]
  apply (canonical_zero_iff W hDelta n _).2
  exact ⟨P, hX, hSum⟩

end

end DilationSemaevBridge
