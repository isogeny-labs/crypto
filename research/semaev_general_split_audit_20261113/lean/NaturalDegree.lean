import UniversalSplit

namespace SemaevGeneralSplitAudit
open Polynomial SemaevSymmetryAudit
noncomputable section
variable {K : Type*} [Field K]

theorem towerEvaluate_independent (N T : ℕ) (g : Fin N → Fin T)
    (p : MvPolynomial (Fin N) K) :
    towerEvaluate MvPolynomial.C N (fun i => MvPolynomial.X (g i)) (towerEquiv K N p)=
      MvPolynomial.rename g p := by
  rw [towerEvaluate_equiv]
  calc
    MvPolynomial.eval₂ MvPolynomial.C (fun i => MvPolynomial.X (g i)) p=
      MvPolynomial.eval₂ MvPolynomial.C MvPolynomial.X (MvPolynomial.rename g p) := by
      simpa only [Function.comp_def] using
        (MvPolynomial.eval₂_rename MvPolynomial.C g MvPolynomial.X p).symm
    _ = MvPolynomial.rename g p := MvPolynomial.eval₂_eta _

theorem towerEvaluate_independent_injective (N T : ℕ) (g : Fin N → Fin T)
    (hg : Function.Injective g) :
    Function.Injective (towerEvaluate (MvPolynomial.C : K →+* MvPolynomial (Fin T) K)
      N (fun i => MvPolynomial.X (g i))) := by
  intro p q h
  apply (towerEquiv K N).symm.injective
  apply MvPolynomial.rename_injective g hg
  rw [← towerEvaluate_independent,← towerEvaluate_independent]
  simpa only [RingEquiv.apply_symm_apply] using h

theorem sourceKernel_independent_degree (W : WeierstrassCurve.Affine K) (n T : ℕ)
    (g : Fin (n+2) → Fin T) (hg : Function.Injective g) :
    (sourceKernel W n MvPolynomial.C (fun i => MvPolynomial.X (g i))).natDegree=2^(n+1) := by
  rw [sourceKernel,natDegree_map_eq_of_injective (towerEvaluate_independent_injective _ _ g hg),
    towerCanonical_degree]

theorem leftFactor_degree (W : WeierstrassCurve.Affine K) (m n : ℕ) :
    (leftFactor W m n).natDegree=2^(m+1) :=
  sourceKernel_independent_degree W m _ (Fin.castAdd (n+2)) (Fin.castAdd_injective _ _)

theorem rightFactor_degree (W : WeierstrassCurve.Affine K) (m n : ℕ) :
    (rightFactor W m n).natDegree=2^(n+1) :=
  sourceKernel_independent_degree W n _ (Fin.natAdd (m+2)) (Fin.natAdd_injective _ _)

/-- The prescribed dimensions are proved to be the true degrees of the actual
independent-variable factors. No degree premise is imported into the claim. -/
theorem universal_natural_split (W : WeierstrassCurve.Affine K) (m n : ℕ) :
    Polynomial.resultant (leftFactor W m n) (rightFactor W m n)
      (leftFactor W m n).natDegree (rightFactor W m n).natDegree=
      arityPolynomial W ((m+2)+(n+2)) (blockArityProof m n) := by
  rw [leftFactor_degree,rightFactor_degree,universal_prescribed_split]

#print axioms universal_natural_split
end
end SemaevGeneralSplitAudit
