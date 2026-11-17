import ResultantDegreeStep
import Mathlib.Algebra.Category.Ring.Basic

/-! Canonical recursive polynomials with independent variables.
Stage n has arity n+3. Only the last input's degree is asserted here;
symmetry, the other inputs and geometric zero-sum equivalence are separate.
The declared Sylvester sizes are never reset after specialization. -/
namespace RecursiveSemaevDegreeAudit
noncomputable section
open Polynomial S4DegreeAudit ResultantDegreeStepAudit
universe u
variable {K : Type u} [Field K]

/-- All inputs except the last one: n+2 independent polynomial variables. -/
def InputRing (K : Type u) [Field K] : ℕ→CommRingCat.{u}
  | 0 => CommRingCat.of (Polynomial (Polynomial K))
  | n+1 => CommRingCat.of (Polynomial (InputRing K n))

instance inputRingIsDomain (n : ℕ) : IsDomain (InputRing K n) := by
  induction n with
  | zero => exact inferInstanceAs (IsDomain (Polynomial (Polynomial K)))
  | succ n ih =>
    let := ih
    exact inferInstanceAs (IsDomain (Polynomial (InputRing K n)))

def coefficientEmbedding : (n : ℕ)→K→+*InputRing K n
  | 0 => (C : K[X]→+*K[X][X]).comp (C : K→+*K[X])
  | n+1 => (C : InputRing K n→+*Polynomial (InputRing K n)).comp
      (coefficientEmbedding n)

def basePolynomial (W : WeierstrassCurve.Affine K) : Polynomial (InputRing K 0) :=
  quad (pairA (K:=K)) (pairB W) (pairC W)

theorem quad_eval {R : Type*} [CommRing R] (a b c t : R) :
    (quad a b c).eval t=a*t^2+b*t+c := by simp [quad]

set_option backward.isDefEq.respectTransparency false in
theorem basePolynomial_eval (W : WeierstrassCurve.Affine K) (t : InputRing K 0) :
    (basePolynomial W).eval t=
      s3Ring (liftPair W.b₂) (liftPair W.b₄) (liftPair W.b₆) (liftPair W.b₈)
        pairX pairY t := by
  exact quad_eval (R:=InputRing K 0) (pairA (K:=K)) (pairB W) (pairC W) t

set_option backward.isDefEq.respectTransparency false in
theorem basePolynomial_natDegree (W : WeierstrassCurve.Affine K) :
    (basePolynomial W).natDegree=2 := by
  change (quad (pairA (K:=K)) (pairB W) (pairC W)).natDegree=2
  apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero (quad_natDegree_le ..)
  rw [quad_coeff_two]
  exact pairA_nonzero

/-- Actual canonical recursion, over an increasing polynomial coefficient
ring. At each stage the old last input becomes the elimination variable;
the new last two inputs are independent Z,T from resultantStep. -/
def recursivePolynomial (W : WeierstrassCurve.Affine K) :
    (n : ℕ)→Polynomial (InputRing K n)
  | 0 => basePolynomial W
  | n+1 => resultantStep (2^(n+1)) (recursivePolynomial W n)
      (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
      (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈)

theorem recursivePolynomial_zero (W : WeierstrassCurve.Affine K) :
    recursivePolynomial W 0=basePolynomial W := rfl

theorem recursivePolynomial_succ (W : WeierstrassCurve.Affine K) (n : ℕ) :
    recursivePolynomial W (n+1)=
      resultantStep (2^(n+1)) (recursivePolynomial W n)
        (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
        (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈) := rfl

set_option backward.isDefEq.respectTransparency false in
theorem recursivePolynomial_natDegree (W : WeierstrassCurve.Affine K) (n : ℕ) :
    (recursivePolynomial W n).natDegree=2^(n+1) := by
  induction n with
  | zero => simpa only [recursivePolynomial,zero_add,pow_one] using basePolynomial_natDegree W
  | succ n ih =>
    have hn : recursivePolynomial W n≠0 := by
      intro hz
      have hp : 0<2^(n+1) := pow_pos (by decide) _
      rw [hz,Polynomial.natDegree_zero] at ih
      omega
    rw [recursivePolynomial_succ]
    change (resultantStep (2^(n+1)) (recursivePolynomial W n)
      (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
      (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈) :
      Polynomial (Polynomial (InputRing K n))).natDegree=2^(n+1+1)
    rw [resultantStep_natDegree _ _ _ _ _ _ (le_of_eq ih) hn]
    exact (pow_succ 2 (n+1)).symm

theorem recursivePolynomial_nonzero (W : WeierstrassCurve.Affine K) (n : ℕ) :
    recursivePolynomial W n≠0 := by
  intro hz
  have hd := recursivePolynomial_natDegree W n
  have hp : 0<2^(n+1) := pow_pos (by decide) _
  rw [hz,Polynomial.natDegree_zero] at hd
  omega

/-- For actual arity m>=3, the last input has degree 2^(m-2). -/
theorem all_arities_last_input_degree (W : WeierstrassCurve.Affine K)
    (m : ℕ) (hm : 3≤m) :
    (recursivePolynomial W (m-3)).natDegree=2^(m-2) := by
  rw [recursivePolynomial_natDegree]
  congr 1
  omega

theorem firstLift_quad {R : Type*} [CommRing R] (a b c : R) :
    firstLift (quad a b c)=quad (C (C a)) (C (C b)) (C (C c)) := by
  simp [firstLift,quad,RingHom.comp_apply]

set_option backward.isDefEq.respectTransparency false in
theorem recursivePolynomial_one_eq_canonicalS4 (W : WeierstrassCurve.Affine K) :
    recursivePolynomial W 1=fullySymbolicLast W := by
  change resultantStep 2 (quad (pairA (K:=K)) (pairB W) (pairC W))
    (liftPair W.b₂) (liftPair W.b₄) (liftPair W.b₆) (liftPair W.b₈)=fullySymbolicLast W
  rw [resultantStep,firstLift_quad]
  rfl

set_option backward.isDefEq.respectTransparency false in
theorem recursivePolynomial_succ_coeff (W : WeierstrassCurve.Affine K) (n : ℕ) :
    (recursivePolynomial W (n+1)).coeff (2^(n+2))=(recursivePolynomial W n)^2 := by
  change (resultantStep (2^(n+1)) (recursivePolynomial W n)
    (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
    (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈)).coeff (2^(n+2))=_
  rw [show n+2=(n+1)+1 by omega,pow_succ]
  exact resultantStep_coeff_top _ _ _ _ _ _ (le_of_eq (recursivePolynomial_natDegree W n))

/-- Explicit binding of the actual family and proved recurrence, rather
than a hypothesis asserting an abstract degree-doubling sequence. -/
theorem recursive_semaev_degree_audit_certificate (W : WeierstrassCurve.Affine K) :
    recursivePolynomial W 1=fullySymbolicLast W ∧
    (∀ n, recursivePolynomial W (n+1)=
      resultantStep (2^(n+1)) (recursivePolynomial W n)
        (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
        (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈)) ∧
    (∀ n, (recursivePolynomial W (n+1)).coeff (2^(n+2))=(recursivePolynomial W n)^2) ∧
    (∀ (m : ℕ), 3≤m → (recursivePolynomial W (m-3)).natDegree=2^(m-2)) := by
  exact ⟨recursivePolynomial_one_eq_canonicalS4 W,recursivePolynomial_succ W,
    recursivePolynomial_succ_coeff W,all_arities_last_input_degree W⟩

#print axioms recursivePolynomial_natDegree
#print axioms all_arities_last_input_degree
#print axioms recursive_semaev_degree_audit_certificate
end
end RecursiveSemaevDegreeAudit
