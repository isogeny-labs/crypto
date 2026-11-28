import CubicInterchange
import FullSymmetry

namespace SemaevGeneralSplitAudit
open Polynomial SemaevSymmetryAudit ResultantDegreeStepAudit
open RecursiveSemaevTransportAudit
noncomputable section
variable {R S : Type*} [CommRing R] [CommRing S]

/-- Explicit outer-first evaluation under an arbitrary coefficient map. -/
def towerEvaluate (f : R →+* S) : (n : ℕ) → (Fin n → S) → Tower R n →+* S
  | 0, _ => f
  | n+1, xs => eval₂RingHom (towerEvaluate f n (fun i => xs i.succ)) (xs 0)

theorem towerEvaluate_cons (f : R →+* S) (n : ℕ) (xs : Fin n → S) (z : S) :
    towerEvaluate f (n+1) (Fin.cases z xs)=eval₂RingHom (towerEvaluate f n xs) z := rfl

theorem towerEvaluate_scalar (f : R →+* S) (n : ℕ) (xs : Fin n → S) (a : R) :
    towerEvaluate f n xs (towerScalar n a)=f a := by
  induction n with
  | zero => rfl
  | succ n ih =>
      change Polynomial.eval₂ _ _ (C (towerScalar n a))=f a
      rw [Polynomial.eval₂_C]
      exact ih (fun i => xs i.succ)

theorem towerEvaluate_variable (f : R →+* S) (n : ℕ) (xs : Fin n → S) (i : Fin n) :
    towerEvaluate f n xs (towerVariable n i)=xs i := by
  induction n with
  | zero => exact Fin.elim0 i
  | succ n ih =>
      refine Fin.cases ?_ (fun j => ?_) i
      · change Polynomial.eval₂ _ _ X=xs 0
        exact Polynomial.eval₂_X ..
      · change Polynomial.eval₂ _ _ (C (towerVariable n j))=xs j.succ
        rw [Polynomial.eval₂_C]
        exact ih (fun i => xs i.succ) j

theorem towerEvaluate_equiv (f : R →+* S) (n : ℕ) (xs : Fin n → S)
    (p : MvPolynomial (Fin n) R) :
    towerEvaluate f n xs (towerEquiv R n p)=MvPolynomial.eval₂ f xs p := by
  have h : (towerEvaluate f n xs).comp (towerEquiv R n).toRingHom=
      MvPolynomial.eval₂Hom f xs := by
    apply MvPolynomial.ringHom_ext
    · intro a
      change towerEvaluate f n xs (towerEquiv R n (MvPolynomial.C a))=
        MvPolynomial.eval₂ f xs (MvPolynomial.C a)
      rw [towerEquiv_C,towerEvaluate_scalar,MvPolynomial.eval₂_C]
    · intro i
      change towerEvaluate f n xs (towerEquiv R n (MvPolynomial.X i))=
        MvPolynomial.eval₂ f xs (MvPolynomial.X i)
      rw [towerEquiv_X,towerEvaluate_variable,MvPolynomial.eval₂_X]
  exact congrArg (fun h : MvPolynomial (Fin n) R →+* S => h p) h

theorem eval₂Hom_as_comp (f : R →+* S) (z : S) :
    eval₂RingHom f z=(evalRingHom z).comp (mapRingHom f) := by
  ext a <;> simp

variable {K : Type*} [Field K]

def sourceKernel (W : WeierstrassCurve.Affine K) (n : ℕ) (f : K →+* S)
    (xs : Fin (n+2) → S) : Polynomial S :=
  (towerCanonical W n).map (towerEvaluate f (n+2) xs)

def sourceValue (W : WeierstrassCurve.Affine K) (n : ℕ) (f : K →+* S)
    (xs : Fin (n+3) → S) : S := towerEvaluate f (n+3) xs (towerCanonical W n)

theorem sourceKernel_degree_le (W : WeierstrassCurve.Affine K) (n : ℕ) (f : K →+* S)
    (xs : Fin (n+2) → S) : (sourceKernel W n f xs).natDegree≤2^(n+1) := by
  exact natDegree_map_le.trans (towerCanonical_degree W n).le

theorem sourceKernel_eval (W : WeierstrassCurve.Affine K) (n : ℕ) (f : K →+* S)
    (xs : Fin (n+2) → S) (z : S) :
    (sourceKernel W n f xs).eval z=sourceValue W n f (Fin.cases z xs) := by
  unfold sourceKernel sourceValue
  rw [towerEvaluate_cons]
  exact Polynomial.eval_map ..

theorem sourceValue_as_mv (W : WeierstrassCurve.Affine K) (n : ℕ) (f : K →+* S)
    (xs : Fin (n+3) → S) :
    sourceValue W n f xs=MvPolynomial.eval₂ f xs (canonicalMv W n) := by
  have h := towerEvaluate_equiv f (n+3) xs (canonicalMv W n)
  simpa only [sourceValue,canonicalMv,RingEquiv.apply_symm_apply] using h

theorem sourceValue_permute (W : WeierstrassCurve.Affine K) (n : ℕ) (f : K →+* S)
    (xs : Fin (n+3) → S) (σ : Equiv.Perm (Fin (n+3))) :
    sourceValue W n f (xs ∘ σ)=sourceValue W n f xs := by
  rw [sourceValue_as_mv,sourceValue_as_mv]
  have h := congrArg (MvPolynomial.eval₂ f xs) (canonicalMv_isSymmetric W n σ)
  simpa only [MvPolynomial.eval₂_rename] using h

theorem sourceKernel_succ (W : WeierstrassCurve.Affine K) (n : ℕ) (f : K →+* S)
    (xs : Fin (n+2) → S) (z : S) :
    sourceKernel W (n+1) f (Fin.cases z xs)=
      cubicConvolution (2^(n+1)) (sourceKernel W n f xs)
        (f W.b₂) (f W.b₄) (f W.b₆) (f W.b₈) z := by
  let α : Tower K (n+2) →+* S := towerEvaluate f (n+2) xs
  have hs (a : K) : α (towerScalar (n+2) a)=f a := towerEvaluate_scalar f (n+2) xs a
  have h := resultantStep_map α (2^(n+1)) (towerCanonical W n)
    (towerScalar (n+2) W.b₂) (towerScalar (n+2) W.b₄)
    (towerScalar (n+2) W.b₆) (towerScalar (n+2) W.b₈)
  rw [hs,hs,hs,hs] at h
  calc
    sourceKernel W (n+1) f (Fin.cases z xs)=
        ((resultantStep (2^(n+1)) (towerCanonical W n)
          (towerScalar (n+2) W.b₂) (towerScalar (n+2) W.b₄)
          (towerScalar (n+2) W.b₆) (towerScalar (n+2) W.b₈)).map (mapRingHom α)).map
          (evalRingHom z) := by
      unfold sourceKernel
      rw [towerCanonical_succ,towerEvaluate_cons,Polynomial.map_map,← eval₂Hom_as_comp]
    _ = (resultantStep (2^(n+1)) (sourceKernel W n f xs)
          (f W.b₂) (f W.b₄) (f W.b₆) (f W.b₈)).map (evalRingHom z) := by
      exact congrArg (fun p : Polynomial (Polynomial S) => p.map (evalRingHom z)) h
    _ = cubicConvolution (2^(n+1)) (sourceKernel W n f xs)
          (f W.b₂) (f W.b₄) (f W.b₆) (f W.b₈) z :=
      (cubicConvolution_as_step (R:=S) (2^(n+1)) (sourceKernel W n f xs)
        (f W.b₂) (f W.b₄) (f W.b₆) (f W.b₈) z).symm

#print axioms sourceKernel_succ
#print axioms sourceValue_permute
end
end SemaevGeneralSplitAudit
