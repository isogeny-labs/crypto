module
public import ComplexH1
public import CocycleClasses
public import Mathlib.Algebra.Group.Equiv.TypeTags
public import Mathlib.LinearAlgebra.Isomorphisms

@[expose] public section
namespace ContinuousH1ComparisonAudit
noncomputable section
open ProcyclicGaloisCocycleAudit

universe u
variable (K : Type u) [Field K] [Fintype K]
variable (M : Type u) [CommGroup M] [TopologicalSpace M] [DiscreteTopology M]
variable [TopologicalSpace (MulAut M)] [DiscreteTopology (MulAut M)]

def coefficientRepresentation (ρ : Gamma K →ₜ* MulAut M) :
    ContRepresentation ℤ (Gamma K) (Additive M) where
  toMonoidHom.toFun σ := {
    toLinearMap := (ρ σ).toMonoidHom.toAdditive.toIntLinearMap
    cont := continuous_of_discreteTopology }
  toMonoidHom.map_one' := by
    apply ContinuousLinearMap.ext
    intro a
    change ρ 1 a.toMul = a.toMul
    rw [map_one]
    rfl
  toMonoidHom.map_mul' σ τ := by
    apply ContinuousLinearMap.ext
    intro a
    change ρ (σ * τ) a.toMul = ρ σ (ρ τ a.toMul)
    rw [map_mul]
    rfl

abbrev coefficientObject (ρ : Gamma K →ₜ* MulAut M) : TopRep ℤ (Gamma K) :=
  TopRep.of (coefficientRepresentation K M ρ)

omit [Fintype K] in
theorem coefficient_joint_continuous (ρ : Gamma K →ₜ* MulAut M) :
    Continuous (fun a : Gamma K × coefficientObject K M ρ =>
      (coefficientObject K M ρ).ρ a.1 a.2) := by
  have hf : Continuous (fun a : MulAut M × Additive M =>
      Additive.ofMul (a.1 a.2.toMul)) := continuous_of_discreteTopology
  exact hf.comp ((ρ.continuous.comp continuous_fst).prodMk continuous_snd)

abbrev GaloisCrossed (ρ : Gamma K →ₜ* MulAut M) := crossedCocycles (coefficientObject K M ρ)
abbrev GaloisCrossedClasses (ρ : Gamma K →ₜ* MulAut M) :=
  CrossedClasses (coefficientObject K M ρ) (coefficient_joint_continuous K M ρ)

def toOldCocycle (ρ : Gamma K →ₜ* MulAut M) (c : GaloisCrossed K M ρ) :
    Cocycles K M ρ := by
  refine ⟨⟨fun σ => (c.val σ).toMul, continuous_toMul.comp c.val.continuous⟩, ?_⟩
  intro σ τ
  exact c.property σ τ

def fromOldCocycle (ρ : Gamma K →ₜ* MulAut M) (c : Cocycles K M ρ) :
    GaloisCrossed K M ρ := by
  refine ⟨⟨fun σ => Additive.ofMul (c.val σ), continuous_ofMul.comp c.val.continuous⟩, ?_⟩
  intro σ τ
  exact cocycle_identity K M ρ c σ τ

def crossedOldEquiv (ρ : Gamma K →ₜ* MulAut M) :
    GaloisCrossed K M ρ ≃+ Additive (Cocycles K M ρ) where
  toFun c := Additive.ofMul (toOldCocycle K M ρ c)
  invFun c := fromOldCocycle K M ρ c.toMul
  left_inv c := by
    apply Subtype.ext
    apply ContinuousMap.ext
    intro σ
    rfl
  right_inv c := by
    change toOldCocycle K M ρ (fromOldCocycle K M ρ c.toMul) = c.toMul
    apply Subtype.ext
    apply ContinuousMap.ext
    intro σ
    rfl
  map_add' c d := by
    change toOldCocycle K M ρ (c + d) = toOldCocycle K M ρ c * toOldCocycle K M ρ d
    apply Subtype.ext
    apply ContinuousMap.ext
    intro σ
    rfl

omit [Fintype K] in
theorem crossed_old_principal (ρ : Gamma K →ₜ* MulAut M) (m : M) :
    crossedOldEquiv K M ρ (principalCrossed (coefficientObject K M ρ)
      (coefficient_joint_continuous K M ρ) (Additive.ofMul m)) =
        Additive.ofMul (principalHom K M ρ m) := by
  apply Subtype.ext
  apply ContinuousMap.ext
  intro σ
  rfl

def crossedToClasses (ρ : Gamma K →ₜ* MulAut M) :
    GaloisCrossed K M ρ →ₗ[ℤ] Additive (CocycleClasses K M ρ) :=
  (((QuotientGroup.mk' (principalHom K M ρ).range).toAdditive).comp
    (crossedOldEquiv K M ρ).toAddMonoidHom).toIntLinearMap

omit [Fintype K] in
theorem crossed_to_classes_surjective (ρ : Gamma K →ₜ* MulAut M) :
    Function.Surjective (crossedToClasses K M ρ) := by
  intro c
  obtain ⟨d, hd⟩ := QuotientGroup.mk'_surjective (principalHom K M ρ).range c.toMul
  refine ⟨(crossedOldEquiv K M ρ).symm (Additive.ofMul d), ?_⟩
  change Additive.ofMul (QuotientGroup.mk
    ((crossedOldEquiv K M ρ) ((crossedOldEquiv K M ρ).symm (Additive.ofMul d))).toMul) = c
  rw [AddEquiv.apply_symm_apply]
  exact congrArg Additive.ofMul hd

omit [Fintype K] in
theorem crossed_to_classes_kernel (ρ : Gamma K →ₜ* MulAut M) :
    (crossedToClasses K M ρ).ker =
      (principalCrossed (coefficientObject K M ρ) (coefficient_joint_continuous K M ρ)).range := by
  ext c
  constructor
  · intro hc
    change (QuotientGroup.mk (toOldCocycle K M ρ c) : CocycleClasses K M ρ) = 1 at hc
    obtain ⟨m, hm⟩ := (QuotientGroup.eq_one_iff (toOldCocycle K M ρ c)).mp hc
    refine ⟨Additive.ofMul m, ?_⟩
    apply (crossedOldEquiv K M ρ).injective
    exact (crossed_old_principal K M ρ m).trans (congrArg Additive.ofMul hm)
  · rintro ⟨a, rfl⟩
    change (QuotientGroup.mk (toOldCocycle K M ρ
      (principalCrossed (coefficientObject K M ρ) (coefficient_joint_continuous K M ρ) a)) :
        CocycleClasses K M ρ) = 1
    apply (QuotientGroup.eq_one_iff _).mpr
    exact ⟨a.toMul, rfl⟩

def galoisCrossedClassEquiv (ρ : Gamma K →ₜ* MulAut M) :
    GaloisCrossedClasses K M ρ ≃ₗ[ℤ] Additive (CocycleClasses K M ρ) :=
  (Submodule.quotEquivOfEq _ _ (crossed_to_classes_kernel K M ρ).symm).trans
    ((crossedToClasses K M ρ).quotKerEquivOfSurjective (crossed_to_classes_surjective K M ρ))

def actualGaloisH1Equiv (ρ : Gamma K →ₜ* MulAut M) :
    continuousCohomology 1 (coefficientObject K M ρ) ≃ₗ[ℤ] Additive (CocycleClasses K M ρ) :=
  (actualContinuousH1CrossedClasses (coefficientObject K M ρ)
    (coefficient_joint_continuous K M ρ)).trans (galoisCrossedClassEquiv K M ρ)

omit [Fintype K] in
theorem galoisCrossedClassEquiv_mk (ρ : Gamma K →ₜ* MulAut M) (c : GaloisCrossed K M ρ) :
    galoisCrossedClassEquiv K M ρ (Submodule.Quotient.mk c) =
      Additive.ofMul (QuotientGroup.mk (toOldCocycle K M ρ c)) := by
  rfl

omit [Fintype K] in
theorem actualGaloisH1_representative (ρ : Gamma K →ₜ* MulAut M)
    (f : KernelOne (coefficientObject K M ρ)) :
    actualGaloisH1Equiv K M ρ (homogeneousKernelClass (coefficientObject K M ρ) f) =
      Additive.ofMul (QuotientGroup.mk (toOldCocycle K M ρ
        (kernelCrossedEquiv (coefficientObject K M ρ) (coefficient_joint_continuous K M ρ) f))) := by
  change galoisCrossedClassEquiv K M ρ
    (actualContinuousH1CrossedClasses _ _ (homogeneousKernelClass _ f)) = _
  rw [homogeneous_crossed_class_formula]
  exact galoisCrossedClassEquiv_mk K M ρ _

variable [Finite M]

def actualGaloisH1CoinvariantEquiv (ρ : Gamma K →ₜ* MulAut M) :
    continuousCohomology 1 (coefficientObject K M ρ) ≃ₗ[ℤ] Additive (Coinvariants K M ρ) :=
  (actualGaloisH1Equiv K M ρ).trans
    (classEvaluationEquiv K M ρ).toAdditive.toIntLinearEquiv

#print axioms coefficient_joint_continuous
#print axioms crossed_to_classes_kernel
#print axioms actualGaloisH1Equiv
#print axioms actualGaloisH1CoinvariantEquiv
end
end ContinuousH1ComparisonAudit
