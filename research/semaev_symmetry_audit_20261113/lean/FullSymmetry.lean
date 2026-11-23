import AdjacentCanonical
import TowerVariables

namespace SemaevSymmetryAudit
open Polynomial S4DegreeAudit RecursiveSemaevDegreeAudit SemaevReassociationAudit
noncomputable section
variable {R : Type*} [CommRing R]

theorem symmetric_of_fin_adjacent (n : ℕ) (p : MvPolynomial (Fin (n+1)) R)
    (h : ∀ i : Fin n, MvPolynomial.rename (Equiv.swap i.castSucc i.succ) p = p) :
    p.IsSymmetric := by
  let H : Submonoid (Equiv.Perm (Fin (n+1))) := {
    carrier := {σ | MvPolynomial.rename σ p = p}
    one_mem' := by
      change MvPolynomial.rename id p = p
      exact MvPolynomial.rename_id_apply p
    mul_mem' := by
      intro σ τ hσ hτ
      change MvPolynomial.rename σ p = p at hσ
      change MvPolynomial.rename τ p = p at hτ
      change MvPolynomial.rename (σ ∘ τ) p = p
      rw [← MvPolynomial.rename_rename, hτ, hσ] }
  have hc : Submonoid.closure (Set.range fun i : Fin n => Equiv.swap i.castSucc i.succ) ≤ H := by
    apply Submonoid.closure_le.mpr
    rintro σ ⟨i,rfl⟩
    exact h i
  have ht : (⊤ : Submonoid (Equiv.Perm (Fin (n+1)))) ≤ H := by
    rw [← Equiv.Perm.mclosure_swap_castSucc_succ n]
    exact hc
  intro σ
  exact ht (show σ ∈ (⊤ : Submonoid (Equiv.Perm (Fin (n+1)))) from trivial)

def towerPermutation (n : ℕ) (σ : Equiv.Perm (Fin n)) : Tower R n →+* Tower R n :=
  (towerEquiv R n).toRingHom.comp ((MvPolynomial.rename σ).toRingHom.comp
    (towerEquiv R n).symm.toRingHom)

theorem towerPermutation_variable (n : ℕ) (σ : Equiv.Perm (Fin n)) (i : Fin n) :
    towerPermutation n σ (towerVariable n i : Tower R n) = towerVariable n (σ i) := by
  change towerEquiv R n (MvPolynomial.rename σ
    ((towerEquiv R n).symm (towerVariable n i))) = _
  rw [← towerEquiv_X n i]
  rw [RingEquiv.symm_apply_apply, MvPolynomial.rename_X, towerEquiv_X]

theorem towerPermutation_scalar (n : ℕ) (σ : Equiv.Perm (Fin n)) (a : R) :
    towerPermutation n σ (towerScalar n a : Tower R n) = towerScalar n a := by
  change towerEquiv R n (MvPolynomial.rename σ
    ((towerEquiv R n).symm (towerScalar n a))) = _
  rw [← towerEquiv_C n a]
  rw [RingEquiv.symm_apply_apply, MvPolynomial.rename_C, towerEquiv_C]

variable {K : Type*} [Field K]

def canonicalMv (W : WeierstrassCurve.Affine K) (n : ℕ) : MvPolynomial (Fin (n+3)) K :=
  (towerEquiv K (n+3)).symm (towerCanonical W n)

theorem canonicalMv_adjacent (W : WeierstrassCurve.Affine K) (n : ℕ) (i : Fin (n+2)) :
    MvPolynomial.rename (adjacentPerm (n+1) i) (canonicalMv W n) = canonicalMv W n := by
  apply (towerEquiv K (n+3)).injective
  rw [towerEquiv_adjacent]
  simp only [canonicalMv, RingEquiv.apply_symm_apply]
  exact towerCanonical_adjacent W n i

theorem canonicalMv_isSymmetric (W : WeierstrassCurve.Affine K) (n : ℕ) :
    (canonicalMv W n).IsSymmetric := by
  exact symmetric_of_fin_adjacent (n+2) _ (fun i => canonicalMv_adjacent W n i)

theorem towerCanonical_permutation (W : WeierstrassCurve.Affine K) (n : ℕ)
    (σ : Equiv.Perm (Fin (n+3))) :
    towerPermutation (n+3) σ (towerCanonical W n) = towerCanonical W n := by
  change towerEquiv K (n+3) (MvPolynomial.rename σ (canonicalMv W n)) = _
  rw [canonicalMv_isSymmetric W n σ]
  exact (towerEquiv K (n+3)).apply_symm_apply _

def fullInputEquiv (K : Type*) [Field K] (n : ℕ) :
    Polynomial (InputRing K n) ≃+* Tower K (n+3) := Polynomial.mapEquiv (inputEquiv K n)

theorem fullInputEquiv_scalar (n : ℕ) (a : K) :
    fullInputEquiv K n (C (coefficientEmbedding n a)) = towerScalar (n+3) a := by
  change (C (coefficientEmbedding n a)).map (inputEquiv K n).toRingHom =
    C (towerScalar (n+2) a)
  rw [Polynomial.map_C]
  exact congrArg C (inputEquiv_scalar n a)

def inputVariable (K : Type*) [Field K] (n : ℕ) (i : Fin (n+3)) : Polynomial (InputRing K n) :=
  (fullInputEquiv K n).symm (towerVariable (n+3) i)

def permuteCanonical (K : Type*) [Field K] (n : ℕ) (σ : Equiv.Perm (Fin (n+3))) :
    Polynomial (InputRing K n) →+* Polynomial (InputRing K n) :=
  (fullInputEquiv K n).symm.toRingHom.comp ((towerPermutation (n+3) σ).comp
    (fullInputEquiv K n).toRingHom)

theorem inputVariable_outer (n : ℕ) : inputVariable K n 0 = X := by
  unfold inputVariable
  rw [towerVariable_zero]
  change (X : Polynomial (Tower K (n+2))).map (inputEquiv K n).symm.toRingHom = X
  exact Polynomial.map_X _

theorem canonicalMv_actual (W : WeierstrassCurve.Affine K) (n : ℕ) :
    towerEquiv K (n+3) (canonicalMv W n) = fullInputEquiv K n (recursivePolynomial W n) := by
  change towerEquiv K (n+3) ((towerEquiv K (n+3)).symm (towerCanonical W n)) = _
  rw [RingEquiv.apply_symm_apply]
  rfl

theorem permuteCanonical_variable (n : ℕ) (σ : Equiv.Perm (Fin (n+3))) (i : Fin (n+3)) :
    permuteCanonical K n σ (inputVariable K n i) = inputVariable K n (σ i) := by
  change (fullInputEquiv K n).symm (towerPermutation (n+3) σ
    ((fullInputEquiv K n) ((fullInputEquiv K n).symm (towerVariable (n+3) i)))) = _
  rw [RingEquiv.apply_symm_apply, towerPermutation_variable]
  rfl

theorem permuteCanonical_actual (W : WeierstrassCurve.Affine K) (n : ℕ)
    (σ : Equiv.Perm (Fin (n+3))) :
    permuteCanonical K n σ (recursivePolynomial W n) = recursivePolynomial W n := by
  change (fullInputEquiv K n).symm (towerPermutation (n+3) σ
    ((fullInputEquiv K n) (recursivePolynomial W n))) = _
  have h : fullInputEquiv K n (recursivePolynomial W n) = towerCanonical W n := rfl
  rw [h, towerCanonical_permutation]
  exact (fullInputEquiv K n).symm_apply_apply _

theorem permuteCanonical_scalar (n : ℕ) (σ : Equiv.Perm (Fin (n+3))) (a : K) :
    permuteCanonical K n σ (C (coefficientEmbedding n a)) = C (coefficientEmbedding n a) := by
  change (fullInputEquiv K n).symm (towerPermutation (n+3) σ
    ((fullInputEquiv K n) (C (coefficientEmbedding n a)))) = _
  rw [fullInputEquiv_scalar, towerPermutation_scalar, ← fullInputEquiv_scalar n a]
  exact (fullInputEquiv K n).symm_apply_apply _

theorem actual_all_input_symmetry_with_control :
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (n : ℕ),
      (canonicalMv W n).IsSymmetric ∧
      towerEquiv K (n+3) (canonicalMv W n) = fullInputEquiv K n (recursivePolynomial W n) ∧
      (∀ σ : Equiv.Perm (Fin (n+3)),
        permuteCanonical K n σ (recursivePolynomial W n) = recursivePolynomial W n) ∧
      (∀ (σ : Equiv.Perm (Fin (n+3))) (i : Fin (n+3)),
        permuteCanonical K n σ (inputVariable K n i) = inputVariable K n (σ i)) ∧
      (∀ (σ : Equiv.Perm (Fin (n+3))) (a : K),
        permuteCanonical K n σ (C (coefficientEmbedding n a)) = C (coefficientEmbedding n a)) ∧
      inputVariable K n 0 = X) ∧
    s4Ring (0 : ℚ) 0 0 1 0 1 2 3 ≠ s4Ring (0 : ℚ) 0 0 1 0 2 1 3 := by
  constructor
  · intro K _ W n
    exact ⟨canonicalMv_isSymmetric W n, canonicalMv_actual W n,
      permuteCanonical_actual W n, permuteCanonical_variable n, permuteCanonical_scalar n,
      inputVariable_outer n⟩
  · exact unrestricted_b_S4_counterexample

#print axioms actual_all_input_symmetry_with_control

end
end SemaevSymmetryAudit
