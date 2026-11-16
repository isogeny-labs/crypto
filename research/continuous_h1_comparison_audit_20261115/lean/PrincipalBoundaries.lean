module
public import HomogeneousCocycles

@[expose] public section
namespace ContinuousH1ComparisonAudit
noncomputable section
open TopRep ContRepresentation CategoryTheory

variable {k G : Type*} [Ring k] [TopologicalSpace k]
variable [Group G] [TopologicalSpace G] [IsTopologicalGroup G]
variable (X : TopRep k G)

theorem invariant_zero_value (f : CochainsZero X) (g : G) :
    f.val g = X.ρ g (f.val 1) := by
  have he := congrArg (fun a : C(G, X) => a g) (f.property g)
  change X.ρ g (f.val (g⁻¹ * g)) = f.val g at he
  simpa only [inv_mul_cancel] using he.symm

def boundaryToKernel : CochainsZero X →L[k] KernelOne X :=
  ((homogeneousCochains X).d 0 1).hom.codRestrict _ (by
    intro f
    have he := congrArg (fun a : CochainsZero X ⟶ (homogeneousCochains X).X 2 => a.hom f)
      ((homogeneousCochains X).d_comp_d 0 1 2)
    exact he)

variable (hρ : Continuous (fun a : G × X => X.ρ a.1 a.2))

def principalCrossed : X →ₗ[k] crossedCocycles X where
  toFun a := by
    let c : C(G, X) := ⟨fun g => X.ρ g a - a,
      (hρ.comp (continuous_id.prodMk continuous_const)).sub continuous_const⟩
    refine ⟨c, ?_⟩
    intro g h
    change X.ρ (g * h) a - a = X.ρ g a - a + X.ρ g (X.ρ h a - a)
    rw [action_mul, map_sub]
    abel
  map_add' a b := by
    apply Subtype.ext
    apply ContinuousMap.ext
    intro g
    change X.ρ g (a + b) - (a + b) = (X.ρ g a - a) + (X.ρ g b - b)
    rw [map_add]
    abel
  map_smul' a b := by
    apply Subtype.ext
    apply ContinuousMap.ext
    intro g
    change X.ρ g (a • b) - a • b = a • (X.ρ g b - b)
    rw [map_smul, smul_sub]

abbrev CrossedClasses := crossedCocycles X ⧸ (principalCrossed X hρ).range

def zeroCochainFromValue (a : X) : CochainsZero X := by
  refine ⟨⟨fun g => X.ρ g a, hρ.comp (continuous_id.prodMk continuous_const)⟩, ?_⟩
  intro g
  apply ContinuousMap.ext
  intro h
  change X.ρ g (X.ρ (g⁻¹ * h) a) = X.ρ h a
  rw [← action_mul, mul_inv_cancel_left]

theorem boundary_crossed (f : CochainsZero X) :
    kernelCrossedEquiv X hρ (boundaryToKernel X f) = principalCrossed X hρ (f.val 1) := by
  apply Subtype.ext
  apply ContinuousMap.ext
  intro g
  change (((homogeneousCochains X).d 0 1).hom f).val 1 g = X.ρ g (f.val 1) - f.val 1
  rw [differential_zero_value X f 1 g, invariant_zero_value X f g]

theorem boundaries_map_to_all_principals :
    (boundaryToKernel X).range.map (kernelCrossedEquiv X hρ).toLinearMap =
      (principalCrossed X hρ).range := by
  ext c
  constructor
  · rintro ⟨f, ⟨a, rfl⟩, rfl⟩
    exact ⟨a.val 1, (boundary_crossed X hρ a).symm⟩
  · rintro ⟨a, rfl⟩
    refine ⟨boundaryToKernel X (zeroCochainFromValue X hρ a),
      ⟨zeroCochainFromValue X hρ a, rfl⟩, ?_⟩
    exact (boundary_crossed X hρ (zeroCochainFromValue X hρ a)).trans
      (congrArg (principalCrossed X hρ) (by
        change X.ρ 1 a = a
        simp))

def kernelQuotientCrossedClasses :
    (KernelOne X ⧸ (boundaryToKernel X).range) ≃ₗ[k] CrossedClasses X hρ :=
  Submodule.Quotient.equiv _ _ (kernelCrossedEquiv X hρ)
    (boundaries_map_to_all_principals X hρ)

#print axioms boundary_crossed
#print axioms boundaries_map_to_all_principals
#print axioms kernelQuotientCrossedClasses
end
end ContinuousH1ComparisonAudit
