import RecursiveSemaevDegree
import DeclaredResultantRoots
import Mathlib.Data.Fin.Tuple.Basic

/-! Evaluation of the actual fixed-size resultant recursion. Declared
Sylvester sizes are preserved even when specialization lowers degrees. -/
namespace RecursiveSemaevEvaluationAudit
noncomputable section
open Polynomial S4DegreeAudit ResultantDegreeStepAudit RecursiveSemaevDegreeAudit
variable {K : Type*} [Field K]

section Naturality
variable {A B : Type*} [CommRing A] [CommRing B]

def pairEval (φ : A→+*B) (z t : B) : A[X][X]→+*B :=
  Polynomial.eval₂RingHom (Polynomial.eval₂RingHom φ z) t

theorem firstLift_pairEval (φ : A→+*B) (z t : B) (f : A[X]) :
    (firstLift f).map (pairEval φ z t)=f.map φ := by
  ext k
  simp [firstLift,Polynomial.coeff_map,pairEval,RingHom.comp_apply]

theorem secondLift_pairEval (φ : A→+*B) (z t : B) (b2 b4 b6 b8 : A) :
    (secondLift b2 b4 b6 b8).map (pairEval φ z t)=
      quad (qaRing z t) (qbRing (φ b2) (φ b4) (φ b6) z t)
        (qcRing (φ b4) (φ b6) (φ b8) z t) := by
  simp [secondLift,quad,qaRing,qbRing,qcRing,pairEval,map_ofNat]

theorem resultantStep_pairEval (φ : A→+*B) (z t : B) (D : ℕ)
    (f : A[X]) (b2 b4 b6 b8 : A) :
    pairEval φ z t (resultantStep D f b2 b4 b6 b8)=
      Polynomial.resultant (f.map φ)
        (quad (qaRing z t) (qbRing (φ b2) (φ b4) (φ b6) z t)
          (qcRing (φ b4) (φ b6) (φ b8) z t)) D 2 := by
  unfold resultantStep
  rw [←Polynomial.resultant_map_map (R:=A[X][X]) (S:=B)
    (firstLift f) (secondLift b2 b4 b6 b8) D 2 (pairEval φ z t)]
  rw [firstLift_pairEval,secondLift_pairEval]

end Naturality

set_option backward.isDefEq.respectTransparency false in
def inputEval : (n : ℕ) → (Fin (n+2)→K) → InputRing K n→+*K
  | 0,xs => Polynomial.eval₂RingHom (Polynomial.evalRingHom (xs 0)) (xs 1)
  | n+1,xs => Polynomial.eval₂RingHom (inputEval n (Fin.init xs)) (xs (Fin.last (n+2)))

def evalCanonical (W : WeierstrassCurve.Affine K) (n : ℕ) (xs : Fin (n+3)→K) : K :=
  Polynomial.eval₂ (inputEval n (Fin.init xs)) (xs (Fin.last (n+2)))
    (recursivePolynomial W n)

set_option backward.isDefEq.respectTransparency false in
theorem inputEval_scalar (n : ℕ) (xs : Fin (n+2)→K) (a : K) :
    inputEval n xs (coefficientEmbedding n a)=a := by
  induction n with
  | zero =>
    change Polynomial.eval₂ (Polynomial.evalRingHom (xs 0)) (xs 1) (C (C a))=a
    rw [Polynomial.eval₂_C]
    exact Polynomial.eval_C
  | succ n ih =>
    change Polynomial.eval₂ (inputEval n (Fin.init xs)) _
      (C (coefficientEmbedding n a))=a
    rw [Polynomial.eval₂_C]
    exact ih (Fin.init xs)

set_option backward.isDefEq.respectTransparency false in
theorem inputEval_snoc (n : ℕ) (xs : Fin (n+2)→K) (z : K) :
    inputEval (n+1) (Fin.snoc xs z)=Polynomial.eval₂RingHom (inputEval n xs) z := by
  simp only [inputEval,Fin.init_snoc,Fin.snoc_last]

theorem evalCanonical_snoc (W : WeierstrassCurve.Affine K) (n : ℕ)
    (xs : Fin (n+2)→K) (z : K) :
    evalCanonical W n (Fin.snoc xs z)=
      (recursivePolynomial W n).eval₂ (inputEval n xs) z := by
  simp only [evalCanonical,Fin.init_snoc,Fin.snoc_last]

set_option backward.isDefEq.respectTransparency false in
theorem canonical_successor_eval (W : WeierstrassCurve.Affine K) (n : ℕ)
    (xs : Fin (n+2)→K) (z t : K) :
    evalCanonical W (n+1) (Fin.snoc (Fin.snoc xs z) t)=
      Polynomial.resultant ((recursivePolynomial W n).map (inputEval n xs))
        (GeneralS4Audit.cubicInLast W z t) (2^(n+1)) 2 := by
  rw [evalCanonical_snoc,inputEval_snoc]
  change pairEval (inputEval n xs) z t
    (resultantStep (2^(n+1)) (recursivePolynomial W n)
      (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
      (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈))=_
  rw [resultantStep_pairEval,inputEval_scalar,inputEval_scalar,inputEval_scalar,inputEval_scalar]
  rfl

set_option backward.isDefEq.respectTransparency false in
theorem mapped_top_coefficient (W : WeierstrassCurve.Affine K) (n : ℕ)
    (xs : Fin (n+3)→K) :
    ((recursivePolynomial W (n+1)).map (inputEval (n+1) xs)).coeff (2^(n+2))=
      (evalCanonical W n xs)^2 := by
  rw [Polynomial.coeff_map,recursivePolynomial_succ_coeff,map_pow]
  rfl

theorem cubicInLast_degree_le (W : WeierstrassCurve.Affine K) (z t : K) :
    (GeneralS4Audit.cubicInLast W z t).natDegree≤2 := quad_natDegree_le ..

set_option backward.isDefEq.respectTransparency false in
theorem base_mapped (W : WeierstrassCurve.Affine K) (xs : Fin 2→K) :
    (basePolynomial W).map (inputEval 0 xs)=
      GeneralS4Audit.cubicInLast W (xs 0) (xs 1) := by
  change (quad (pairA (K:=K)) (pairB W) (pairC W)).map
    (Polynomial.eval₂RingHom (Polynomial.evalRingHom (xs 0)) (xs 1))=_
  simp [quad,pairA,pairB,pairC,pairX,pairY,liftPair,
    GeneralS4Audit.cubicInLast,GeneralS4Audit.quadratic,
    GeneralS4Audit.qa,GeneralS4Audit.qb,GeneralS4Audit.qc,map_ofNat]

theorem evalCanonical_three (W : WeierstrassCurve.Affine K) (xs : Fin 3→K) :
    evalCanonical W 0 xs=GeneralS3Audit.generalS3 W (xs 0) (xs 1) (xs 2) := by
  change (basePolynomial W).eval₂ (inputEval 0 (Fin.init xs)) (xs 2)=_
  rw [Polynomial.eval₂_eq_eval_map,base_mapped,GeneralS4Audit.cubicInLast_eval]
  rfl

theorem evalCanonical_four (W : WeierstrassCurve.Affine K) (xs : Fin 4→K) :
    evalCanonical W 1 xs=GeneralS4Audit.generalS4 W (xs 0) (xs 1) (xs 2) (xs 3) := by
  have hx : xs=Fin.snoc (Fin.snoc ![xs 0,xs 1] (xs 2)) (xs 3) := by
    funext i
    fin_cases i <;> rfl
  conv_lhs => rw [hx]
  rw [canonical_successor_eval W 0,recursivePolynomial_zero,base_mapped]
  rfl

set_option backward.isDefEq.respectTransparency false in
theorem canonical_two_point_recurrence [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (n : ℕ) (xs : Fin (n+3)→K) (z t : K) :
    evalCanonical W (n+2) (Fin.snoc (Fin.snoc xs z) t)=0 ↔
      (∃ u : K,evalCanonical W (n+1) (Fin.snoc xs u)=0 ∧
        GeneralS3Audit.generalS3 W z t u=0) ∨ (evalCanonical W n xs=0 ∧ z=t) := by
  rw [canonical_successor_eval W (n+1)]
  have hf : ((recursivePolynomial W (n+1)).map (inputEval (n+1) xs)).natDegree≤2^(n+2) :=
    (Polynomial.natDegree_map_le).trans (le_of_eq (recursivePolynomial_natDegree W (n+1)))
  have hr := DeclaredResultantRootsAudit.declared_root_iff
    ((recursivePolynomial W (n+1)).map (inputEval (n+1) xs))
    (GeneralS4Audit.cubicInLast W z t) (2^(n+2)) 2
    (pow_pos (by decide) _) (by decide) hf (cubicInLast_degree_le W z t)
  rw [mapped_top_coefficient] at hr
  have hc : (GeneralS4Audit.cubicInLast W z t).coeff 2=(z-t)^2 := by
    exact quad_coeff_two ..
  rw [hc] at hr
  simpa only [evalCanonical_snoc,Polynomial.eval₂_eq_eval_map,
    GeneralS4Audit.cubicInLast_eval,pow_eq_zero_iff (by decide : 2≠0),sub_eq_zero]
    using hr

#print axioms resultantStep_pairEval
#print axioms inputEval_scalar
#print axioms canonical_two_point_recurrence
#print axioms evalCanonical_three
#print axioms evalCanonical_four
end
end RecursiveSemaevEvaluationAudit
