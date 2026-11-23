import NewestThree
import Mathlib.Algebra.MvPolynomial.Equiv
import Mathlib.RingTheory.MvPolynomial.Symmetric.Defs
import Mathlib.GroupTheory.Perm.Sign

namespace SemaevSymmetryAudit
open Polynomial SemaevReassociationAudit RecursiveSemaevDegreeAudit
noncomputable section
variable {R : Type*} [CommRing R]

def twoEquiv (R : Type*) [CommRing R] :
    MvPolynomial (Fin 2) R ≃ₐ[R] Polynomial (Polynomial R) :=
  (MvPolynomial.finSuccEquiv R 1).trans
    (Polynomial.mapAlgEquiv (MvPolynomial.uniqueAlgEquiv R (Fin 1)))

def threeEquiv (R : Type*) [CommRing R] : MvPolynomial (Fin 3) R ≃ₐ[R] Triple R :=
  (MvPolynomial.finSuccEquiv R 2).trans (Polynomial.mapAlgEquiv (twoEquiv R))

theorem twoEquiv_X_zero : twoEquiv R (MvPolynomial.X 0) = X := by
  simp [twoEquiv, MvPolynomial.finSuccEquiv_X_zero]

theorem twoEquiv_X_one : twoEquiv R (MvPolynomial.X 1) = C X := by
  change (Polynomial.mapAlgEquiv (MvPolynomial.uniqueAlgEquiv R (Fin 1)))
    ((MvPolynomial.finSuccEquiv R 1) (MvPolynomial.X (0 : Fin 1).succ)) = _
  rw [MvPolynomial.finSuccEquiv_X_succ]
  simp [MvPolynomial.uniqueAlgEquiv]

theorem threeEquiv_C (a : R) : threeEquiv R (MvPolynomial.C a) = tripleLift a := by
  simp [threeEquiv, twoEquiv, tripleLift, scalarLift,
    MvPolynomial.finSuccEquiv_apply, MvPolynomial.uniqueAlgEquiv]

theorem threeEquiv_X_zero : threeEquiv R (MvPolynomial.X 0) = formalY := by
  simp [threeEquiv, twoEquiv, formalY, MvPolynomial.finSuccEquiv_apply]

theorem threeEquiv_X_one : threeEquiv R (MvPolynomial.X 1) = formalX := by
  change (Polynomial.mapAlgEquiv (twoEquiv R))
    ((MvPolynomial.finSuccEquiv R 2) (MvPolynomial.X (0 : Fin 2).succ)) = _
  rw [MvPolynomial.finSuccEquiv_X_succ]
  simpa [formalX] using congrArg Polynomial.C (twoEquiv_X_zero (R := R))

theorem threeEquiv_X_two : threeEquiv R (MvPolynomial.X 2) = formalZ := by
  change (Polynomial.mapAlgEquiv (twoEquiv R))
    ((MvPolynomial.finSuccEquiv R 2) (MvPolynomial.X (1 : Fin 2).succ)) = _
  rw [MvPolynomial.finSuccEquiv_X_succ]
  simpa [formalZ] using congrArg Polynomial.C (twoEquiv_X_one (R := R))

theorem threeEquiv_outer (p : MvPolynomial (Fin 3) R) :
    threeEquiv R (MvPolynomial.rename (Equiv.swap (0 : Fin 3) 1) p) =
      outerSwap (threeEquiv R p) := by
  have hh : (threeEquiv R).toRingHom.comp
      (MvPolynomial.rename (Equiv.swap (0 : Fin 3) 1)).toRingHom =
      outerSwap.comp (threeEquiv R).toRingHom := by
    apply MvPolynomial.ringHom_ext
    · intro a
      change threeEquiv R (MvPolynomial.rename _ (MvPolynomial.C a)) =
        outerSwap (threeEquiv R (MvPolynomial.C a))
      rw [MvPolynomial.rename_C, threeEquiv_C, outerSwap_scalar]
    · intro i
      change threeEquiv R (MvPolynomial.rename _ (MvPolynomial.X i)) =
        outerSwap (threeEquiv R (MvPolynomial.X i))
      rw [MvPolynomial.rename_X]
      fin_cases i <;>
        norm_num [Equiv.swap_apply_def, threeEquiv_X_zero, threeEquiv_X_one,
          threeEquiv_X_two, outerSwap_X, outerSwap_Y, outerSwap_Z]
  exact congrArg (fun f => f p) hh

theorem threeEquiv_inner (p : MvPolynomial (Fin 3) R) :
    threeEquiv R (MvPolynomial.rename (Equiv.swap (1 : Fin 3) 2) p) =
      innerSwap (threeEquiv R p) := by
  have hh : (threeEquiv R).toRingHom.comp
      (MvPolynomial.rename (Equiv.swap (1 : Fin 3) 2)).toRingHom =
      innerSwap.comp (threeEquiv R).toRingHom := by
    apply MvPolynomial.ringHom_ext
    · intro a
      change threeEquiv R (MvPolynomial.rename _ (MvPolynomial.C a)) =
        innerSwap (threeEquiv R (MvPolynomial.C a))
      rw [MvPolynomial.rename_C, threeEquiv_C, innerSwap_scalar]
    · intro i
      change threeEquiv R (MvPolynomial.rename _ (MvPolynomial.X i)) =
        innerSwap (threeEquiv R (MvPolynomial.X i))
      rw [MvPolynomial.rename_X]
      fin_cases i <;>
        norm_num [Equiv.swap_apply_def, threeEquiv_X_zero, threeEquiv_X_one,
          threeEquiv_X_two, innerSwap_X, innerSwap_Y, innerSwap_Z]
  exact congrArg (fun f => f p) hh

theorem symmetric_of_adjacent (p : MvPolynomial (Fin 3) R)
    (h01 : MvPolynomial.rename (Equiv.swap (0 : Fin 3) 1) p = p)
    (h12 : MvPolynomial.rename (Equiv.swap (1 : Fin 3) 2) p = p) : p.IsSymmetric := by
  have h02 : MvPolynomial.rename (Equiv.swap (0 : Fin 3) 2) p = p := by
    have hp : Equiv.swap (0 : Fin 3) 2 =
        Equiv.swap 0 1 * Equiv.swap 1 2 * Equiv.swap 0 1 := by decide +kernel
    simp only [hp, Equiv.Perm.coe_mul, ← MvPolynomial.rename_rename, h01, h12]
  have h10 : MvPolynomial.rename (Equiv.swap (1 : Fin 3) 0) p = p := by
    rw [Equiv.swap_comm]; exact h01
  have h20 : MvPolynomial.rename (Equiv.swap (2 : Fin 3) 0) p = p := by
    rw [Equiv.swap_comm]; exact h02
  have h21 : MvPolynomial.rename (Equiv.swap (2 : Fin 3) 1) p = p := by
    rw [Equiv.swap_comm]; exact h12
  have hs (i j : Fin 3) : MvPolynomial.rename (Equiv.swap i j) p = p := by
    fin_cases i <;> fin_cases j
    all_goals solve
      | exact h01 | exact h02 | exact h10 | exact h12 | exact h20 | exact h21 | simp
  intro σ
  refine σ.swap_induction_on (by simp) ?_
  intro π i j hij ih
  simpa only [Equiv.Perm.coe_mul, ← MvPolynomial.rename_rename, ih] using hs i j

def threeMv (p : Triple R) : MvPolynomial (Fin 3) R := (threeEquiv R).symm p

theorem threeMv_symmetric (p : Triple R) (ho : outerSwap p = p) (hi : innerSwap p = p) :
    (threeMv p).IsSymmetric := by
  apply symmetric_of_adjacent
  · apply (threeEquiv R).injective
    rw [threeEquiv_outer]
    simpa only [threeMv, AlgEquiv.apply_symm_apply] using ho
  · apply (threeEquiv R).injective
    rw [threeEquiv_inner]
    simpa only [threeMv, AlgEquiv.apply_symm_apply] using hi

def permuteNewest (σ : Equiv.Perm (Fin 3)) : Triple R →+* Triple R :=
  (threeEquiv R).toRingHom.comp ((MvPolynomial.rename σ).toRingHom.comp
    (threeEquiv R).symm.toRingHom)

variable {K : Type*} [Field K]

theorem canonical_newest_three_symmetric (W : WeierstrassCurve.Affine K) (n : ℕ) :
    (threeMv (recursivePolynomial W (n+2) : Triple (InputRing K n))).IsSymmetric := by
  exact threeMv_symmetric _ (canonical_outerSwap W n) (canonical_innerSwap W n)

theorem canonical_newest_three_permutation (W : WeierstrassCurve.Affine K) (n : ℕ)
    (σ : Equiv.Perm (Fin 3)) :
    permuteNewest (R := InputRing K n) σ (recursivePolynomial W (n+2)) =
      recursivePolynomial W (n+2) := by
  change threeEquiv (InputRing K n)
    (MvPolynomial.rename σ (threeMv (recursivePolynomial W (n+2) : Triple (InputRing K n)))) = _
  rw [canonical_newest_three_symmetric W n σ]
  exact (threeEquiv (InputRing K n)).apply_symm_apply _

#print axioms canonical_newest_three_permutation

end
end SemaevSymmetryAudit
