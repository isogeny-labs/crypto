import Mathlib.Algebra.MvPolynomial.Eval
import Mathlib.RingTheory.Ideal.Quotient.Operations
import Mathlib.LinearAlgebra.FreeModule.Finite.Matrix
import Mathlib.RingTheory.Length
import Mathlib.Tactic

/-! Actual affine solutions and coordinate-quotient maps, including nilpotents. -/
namespace CoordinateLengthAudit
noncomputable section
open MvPolynomial

variable {K L σ : Type*} [Field K] [Field L] [Algebra K L]

def Solution (I : Ideal (MvPolynomial σ K)) (L : Type*) [Field L] [Algebra K L] :=
  { x : σ → L // ∀ p, p ∈ I → aeval x p = 0 }

abbrev CoordinateAlgebra (I : Ideal (MvPolynomial σ K)) := MvPolynomial σ K ⧸ I

theorem hom_evaluation (I : Ideal (MvPolynomial σ K))
    (f : CoordinateAlgebra I →ₐ[K] L) (p : MvPolynomial σ K) :
    aeval (fun i => f (Ideal.Quotient.mkₐ K I (X i))) p =
      f (Ideal.Quotient.mkₐ K I p) := by
  exact (AlgHom.congr_fun (aeval_unique (f.comp (Ideal.Quotient.mkₐ K I))) p).symm

def homSolution (I : Ideal (MvPolynomial σ K)) (f : CoordinateAlgebra I →ₐ[K] L) :
    Solution I L :=
  ⟨fun i => f (Ideal.Quotient.mkₐ K I (X i)), by
    intro p hp
    rw [hom_evaluation]
    have hz : Ideal.Quotient.mkₐ K I p = 0 := Ideal.Quotient.eq_zero_iff_mem.mpr hp
    rw [hz,map_zero]⟩

def solutionHom (I : Ideal (MvPolynomial σ K)) (x : Solution I L) :
    CoordinateAlgebra I →ₐ[K] L :=
  Ideal.Quotient.liftₐ I (aeval x.1) x.2

@[simp] theorem solutionHom_mk (I : Ideal (MvPolynomial σ K)) (x : Solution I L)
    (p : MvPolynomial σ K) :
    solutionHom I x (Ideal.Quotient.mkₐ K I p) = aeval x.1 p := by
  rfl

def solutionEquivHom (I : Ideal (MvPolynomial σ K)) :
    Solution I L ≃ (CoordinateAlgebra I →ₐ[K] L) where
  toFun := solutionHom I
  invFun := homSolution I
  left_inv x := by
    apply Subtype.ext
    funext i
    change solutionHom I x (Ideal.Quotient.mkₐ K I (X i)) = x.1 i
    rw [solutionHom_mk,aeval_X]
  right_inv f := by
    apply Ideal.Quotient.algHom_ext K
    apply MvPolynomial.algHom_ext
    intro i
    change solutionHom I (homSolution I f) (Ideal.Quotient.mkₐ K I (X i)) =
      f (Ideal.Quotient.mkₐ K I (X i))
    rw [solutionHom_mk, aeval_X]
    rfl

theorem solutions_finite (I : Ideal (MvPolynomial σ K))
    [Module.Finite K (CoordinateAlgebra I)] : Finite (Solution I L) :=
  Finite.of_equiv (CoordinateAlgebra I →ₐ[K] L) (solutionEquivHom I).symm

theorem support_le_dimension (I : Ideal (MvPolynomial σ K))
    [Module.Finite K (CoordinateAlgebra I)] :
    Nat.card (Solution I L) ≤ Module.finrank K (CoordinateAlgebra I) := by
  rw [Nat.card_congr (solutionEquivHom I)]
  exact card_algHom_le_finrank K (CoordinateAlgebra I) L

theorem support_le_scalar_length (I : Ideal (MvPolynomial σ K))
    [Module.Finite K (CoordinateAlgebra I)] :
    (Nat.card (Solution I L) : ℕ∞) ≤ Module.length K (CoordinateAlgebra I) := by
  rw [Module.length_eq_finrank]
  exact_mod_cast support_le_dimension (L := L) I

end
end CoordinateLengthAudit
