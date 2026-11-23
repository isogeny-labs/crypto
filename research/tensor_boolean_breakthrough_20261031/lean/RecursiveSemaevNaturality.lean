import DilationScaleSpecialization
import Mathlib.FieldTheory.IsAlgClosed.AlgebraicClosure

/-!
# Base-change naturality of the canonical recursive Semaev family

The geometric characterization is stated over algebraically closed fields.
This module supplies the missing coefficient-extension theorem needed to use
that characterization for point tuples originally defined over a finite base
field.
-/

namespace DilationSemaevBridge

noncomputable section

open Polynomial ResultantDegreeStepAudit RecursiveSemaevDegreeAudit
open RecursiveSemaevEvaluationAudit

universe u v

variable {K : Type u} {L : Type v} [Field K] [Field L]

/-- Map every coefficient through a tower of nested polynomial rings. -/
def inputMap (phi : K →+* L) :
    (n : ℕ) → InputRing K n →+* InputRing L n
  | 0 => Polynomial.mapRingHom (Polynomial.mapRingHom phi)
  | n + 1 => Polynomial.mapRingHom (inputMap phi n)

theorem inputMap_coefficientEmbedding (phi : K →+* L) (n : ℕ) (a : K) :
    inputMap phi n (coefficientEmbedding n a) =
      coefficientEmbedding n (phi a) := by
  induction n with
  | zero =>
      change (Polynomial.C (Polynomial.C a)).map
        (Polynomial.mapRingHom phi) = Polynomial.C (Polynomial.C (phi a))
      simp
  | succ n ih =>
      change (Polynomial.C (coefficientEmbedding n a)).map (inputMap phi n) =
        Polynomial.C (coefficientEmbedding n (phi a))
      simp [ih]

theorem firstLift_map {A : Type*} {B : Type*} [CommRing A] [CommRing B]
    (phi : A →+* B) (f : Polynomial A) :
    (firstLift f).map
        (Polynomial.mapRingHom (Polynomial.mapRingHom phi)) =
      firstLift (f.map phi) := by
  ext i j
  simp [firstLift, Polynomial.coeff_map, RingHom.comp_apply]

theorem secondLift_map {A : Type*} {B : Type*} [CommRing A] [CommRing B]
    (phi : A →+* B) (b2 b4 b6 b8 : A) :
    (secondLift b2 b4 b6 b8).map
        (Polynomial.mapRingHom (Polynomial.mapRingHom phi)) =
      secondLift (phi b2) (phi b4) (phi b6) (phi b8) := by
  simp [secondLift, S4DegreeAudit.quad, S4DegreeAudit.qaRing,
    S4DegreeAudit.qbRing, S4DegreeAudit.qcRing, map_ofNat]

set_option backward.isDefEq.respectTransparency false in
theorem resultantStep_map {A : Type*} {B : Type*} [CommRing A] [CommRing B]
    (phi : A →+* B) (D : ℕ) (f : Polynomial A) (b2 b4 b6 b8 : A) :
    (resultantStep D f b2 b4 b6 b8).map (Polynomial.mapRingHom phi) =
      resultantStep D (f.map phi) (phi b2) (phi b4) (phi b6) (phi b8) := by
  change (Polynomial.mapRingHom (Polynomial.mapRingHom phi))
      (Polynomial.resultant (firstLift f) (secondLift b2 b4 b6 b8) D 2) = _
  rw [← Polynomial.resultant_map_map, firstLift_map, secondLift_map]
  rfl

theorem basePolynomial_map (phi : K →+* L)
    (W : WeierstrassCurve.Affine K) :
    (basePolynomial W).map (inputMap phi 0) =
      basePolynomial (W.map phi) := by
  have hA : inputMap phi 0 (S4DegreeAudit.pairA (K := K)) =
      S4DegreeAudit.pairA (K := L) := by
    change (S4DegreeAudit.pairA (K := K)).map
      (Polynomial.mapRingHom phi) = S4DegreeAudit.pairA (K := L)
    simp [S4DegreeAudit.pairA, S4DegreeAudit.pairX, S4DegreeAudit.pairY]
  have hB : inputMap phi 0 (S4DegreeAudit.pairB W) =
      S4DegreeAudit.pairB (W.map phi) := by
    change (S4DegreeAudit.pairB W).map (Polynomial.mapRingHom phi) = _
    simp [S4DegreeAudit.pairB, S4DegreeAudit.pairX, S4DegreeAudit.pairY,
      S4DegreeAudit.liftPair]
  have hC : inputMap phi 0 (S4DegreeAudit.pairC W) =
      S4DegreeAudit.pairC (W.map phi) := by
    change (S4DegreeAudit.pairC W).map (Polynomial.mapRingHom phi) = _
    simp [S4DegreeAudit.pairC, S4DegreeAudit.pairX, S4DegreeAudit.pairY,
      S4DegreeAudit.liftPair]
  change (S4DegreeAudit.pairA (K := K)).map (Polynomial.mapRingHom phi) = _ at hA
  change (S4DegreeAudit.pairB W).map (Polynomial.mapRingHom phi) = _ at hB
  change (S4DegreeAudit.pairC W).map (Polynomial.mapRingHom phi) = _ at hC
  change (S4DegreeAudit.quad (S4DegreeAudit.pairA (K := K))
    (S4DegreeAudit.pairB W) (S4DegreeAudit.pairC W)).map
      (Polynomial.mapRingHom (Polynomial.mapRingHom phi)) =
    S4DegreeAudit.quad (S4DegreeAudit.pairA (K := L))
      (S4DegreeAudit.pairB (W.map phi)) (S4DegreeAudit.pairC (W.map phi))
  simp only [S4DegreeAudit.quad, Polynomial.map_add, Polynomial.map_mul,
    Polynomial.map_pow, Polynomial.map_X, Polynomial.map_C,
    Polynomial.coe_mapRingHom, hA, hB, hC]

set_option backward.isDefEq.respectTransparency false in
theorem recursivePolynomial_map (phi : K →+* L)
    (W : WeierstrassCurve.Affine K) :
    ∀ n : ℕ, (recursivePolynomial W n).map (inputMap phi n) =
      recursivePolynomial (W.map phi) n := by
  intro n
  induction n with
  | zero => exact basePolynomial_map phi W
  | succ n ih =>
      rw [recursivePolynomial_succ, recursivePolynomial_succ]
      change (resultantStep (2 ^ (n + 1)) (recursivePolynomial W n)
        (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
        (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈)).map
          (Polynomial.mapRingHom (inputMap phi n)) = _
      rw [resultantStep_map, ih]
      simp only [inputMap_coefficientEmbedding, WeierstrassCurve.map_b₂,
        WeierstrassCurve.map_b₄, WeierstrassCurve.map_b₆,
        WeierstrassCurve.map_b₈]

theorem eval₂RingHom_comp_map {A : Type*} {B : Type*} {C : Type*}
    [CommSemiring A] [CommSemiring B] [CommSemiring C]
    (f : A →+* B) (g : C →+* A) (x : B) :
    (Polynomial.eval₂RingHom f x).comp (Polynomial.mapRingHom g) =
      Polynomial.eval₂RingHom (f.comp g) x := by
  apply Polynomial.ringHom_ext
  · intro a
    simp [RingHom.comp_apply]
  · simp [RingHom.comp_apply]

set_option backward.isDefEq.respectTransparency false in
theorem inputEval_inputMap (phi : K →+* L) :
    ∀ (n : ℕ) (xs : Fin (n + 2) → L),
      (inputEval n xs).comp (inputMap phi n) = inputEvalTo phi n xs := by
  intro n
  induction n with
  | zero =>
      intro xs
      change (Polynomial.eval₂RingHom (Polynomial.evalRingHom (xs 0)) (xs 1)).comp
        (Polynomial.mapRingHom (Polynomial.mapRingHom phi)) = _
      rw [eval₂RingHom_comp_map]
      change Polynomial.eval₂RingHom
        ((Polynomial.eval₂RingHom (RingHom.id L) (xs 0)).comp
          (Polynomial.mapRingHom phi)) (xs 1) =
        Polynomial.eval₂RingHom (Polynomial.eval₂RingHom phi (xs 0)) (xs 1)
      rw [eval₂RingHom_comp_map]
      rfl
  | succ n ih =>
      intro xs
      change (Polynomial.eval₂RingHom (inputEval n (Fin.init xs))
        (xs (Fin.last (n + 2)))).comp
          (Polynomial.mapRingHom (inputMap phi n)) = _
      rw [eval₂RingHom_comp_map, ih]
      rfl

set_option backward.isDefEq.respectTransparency false in
theorem evalCanonical_baseChange (phi : K →+* L)
    (W : WeierstrassCurve.Affine K) (n : ℕ) (xs : Fin (n + 3) → L) :
    evalCanonicalTo phi W n xs = evalCanonical (W.map phi) n xs := by
  rw [evalCanonicalTo, evalCanonical, ← recursivePolynomial_map phi W n,
    Polynomial.eval₂_map, inputEval_inputMap]

/-- Scalar evaluation of the canonical family commutes with field
extension. -/
theorem map_evalCanonical (phi : K →+* L)
    (W : WeierstrassCurve.Affine K) (n : ℕ) (xs : Fin (n + 3) → K) :
    phi (evalCanonical W n xs) =
      evalCanonical (W.map phi) n (fun i => phi (xs i)) := by
  rw [← evalCanonicalTo_id W n xs,
    evalCanonicalTo_natural (RingHom.id K) phi,
    RingHom.comp_id, evalCanonical_baseChange]

end

end DilationSemaevBridge
