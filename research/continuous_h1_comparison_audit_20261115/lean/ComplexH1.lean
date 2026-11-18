module
public import PrincipalBoundaries

@[expose] public section
namespace ContinuousH1ComparisonAudit
noncomputable section
open TopRep ContRepresentation CategoryTheory CategoryTheory.Limits

variable {k : Type*} [Ring k] [TopologicalSpace k]

def toDifferentialKernel (S : ShortComplex (TopModuleCat k)) :
    S.X₁ →L[k] S.g.hom.ker :=
  S.f.hom.codRestrict _ (fun x => by
    have he := congrArg (fun a : S.X₁ ⟶ S.X₃ => a.hom x) S.zero
    exact he)

def actualHomologyData (S : ShortComplex (TopModuleCat k)) : S.LeftHomologyData :=
  ⟨_, _, _, _, _, TopModuleCat.isLimitKer _, by simp, TopModuleCat.isColimitCoker _⟩

/-- Actual categorical homology is identified using actual kernel/cokernel
universal properties, preserving the existing homology object. -/
def actualHomologyKernelQuotient (S : ShortComplex (TopModuleCat k)) :
    S.homology ≃ₗ[k] (S.g.hom.ker ⧸ (toDifferentialKernel S).range) :=
  (actualHomologyData S).homologyIso.toContinuousLinearEquiv.toLinearEquiv

/-- This representative is mapped by the actual categorical homology projection. -/
def actualKernelClass (S : ShortComplex (TopModuleCat k)) (f : S.g.hom.ker) : S.homology :=
  S.homologyπ ((actualHomologyData S).cyclesIso.inv f)

theorem actualKernelClass_formula (S : ShortComplex (TopModuleCat k)) (f : S.g.hom.ker) :
    actualHomologyKernelQuotient S (actualKernelClass S f) = Submodule.Quotient.mk f := by
  let D := actualHomologyData S
  have he := congrArg (fun a : D.K ⟶ S.homology => a f) D.π_comp_homologyIso_inv
  have hx : D.homologyIso.inv (D.π f) = actualKernelClass S f := he
  change D.homologyIso.toContinuousLinearEquiv (actualKernelClass S f) = _
  rw [← hx]
  exact D.homologyIso.toContinuousLinearEquiv.apply_symm_apply (D.π f)

variable {G : Type*} [Group G] [TopologicalSpace G] [IsTopologicalGroup G]
variable (X : TopRep k G)

def actualContinuousH1KernelQuotient :
    continuousCohomology 1 X ≃ₗ[k] (KernelOne X ⧸ (boundaryToKernel X).range) :=
  (((homogeneousCochains X).homologyIsoSc' 0 1 2 (by simp) (by simp)).toContinuousLinearEquiv.toLinearEquiv).trans
    (actualHomologyKernelQuotient ((homogeneousCochains X).sc' 0 1 2))

def homogeneousKernelClass (f : KernelOne X) : continuousCohomology 1 X :=
  ((homogeneousCochains X).homologyIsoSc' 0 1 2 (by simp) (by simp)).inv
    (actualKernelClass ((homogeneousCochains X).sc' 0 1 2) f)

theorem homogeneousKernelClass_formula (f : KernelOne X) :
    actualContinuousH1KernelQuotient X (homogeneousKernelClass X f) =
      Submodule.Quotient.mk f := by
  change actualHomologyKernelQuotient _
    (((homogeneousCochains X).homologyIsoSc' 0 1 2 (by simp) (by simp)).toContinuousLinearEquiv
      (((homogeneousCochains X).homologyIsoSc' 0 1 2 (by simp) (by simp)).toContinuousLinearEquiv.symm
        (actualKernelClass _ f))) = _
  rw [ContinuousLinearEquiv.apply_symm_apply]
  exact actualKernelClass_formula ((homogeneousCochains X).sc' 0 1 2) f

def actualContinuousH1CrossedClasses
    (hρ : Continuous (fun a : G × X => X.ρ a.1 a.2)) :
    continuousCohomology 1 X ≃ₗ[k] CrossedClasses X hρ :=
  (actualContinuousH1KernelQuotient X).trans (kernelQuotientCrossedClasses X hρ)

theorem homogeneous_crossed_class_formula
    (hρ : Continuous (fun a : G × X => X.ρ a.1 a.2)) (f : KernelOne X) :
    actualContinuousH1CrossedClasses X hρ (homogeneousKernelClass X f) =
      Submodule.Quotient.mk (kernelCrossedEquiv X hρ f) := by
  change kernelQuotientCrossedClasses X hρ
    (actualContinuousH1KernelQuotient X (homogeneousKernelClass X f)) = _
  rw [homogeneousKernelClass_formula]
  rfl

#print axioms actualHomologyKernelQuotient
#print axioms actualContinuousH1CrossedClasses
#print axioms homogeneous_crossed_class_formula
end
end ContinuousH1ComparisonAudit
