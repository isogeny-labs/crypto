module
public import FiniteCocycles
public import Mathlib.GroupTheory.QuotientGroup.Defs

@[expose] public section
namespace ProcyclicGaloisCocycleAudit
noncomputable section

variable (K : Type*) [Field K] [Fintype K]
variable (M : Type*) [CommGroup M] [TopologicalSpace M] [DiscreteTopology M]
variable [TopologicalSpace (MulAut M)] [DiscreteTopology (MulAut M)]

/-- The actual principal continuous cocycle, in multiplicative notation. -/
def principalCocycle (ρ : Gamma K →ₜ* MulAut M) (m : M) : Cocycles K M ρ := by
  let c : C(Gamma K, M) := ⟨fun σ => ρ σ m / m,
    (continuous_of_discreteTopology :
      Continuous (fun a : MulAut M => a m / m)).comp ρ.continuous⟩
  refine ⟨c, (mem_cocycleSubgroup K M ρ c).mpr ?_⟩
  intro σ τ
  change ρ (σ * τ) m / m = (ρ σ m / m) * ρ σ (ρ τ m / m)
  simp only [map_mul, MulAut.mul_apply, map_div]
  rw [mul_comm (ρ σ m / m), div_mul_div_cancel]

def principalHom (ρ : Gamma K →ₜ* MulAut M) : M →* Cocycles K M ρ where
  toFun := principalCocycle K M ρ
  map_one' := by
    apply Subtype.ext
    apply ContinuousMap.ext
    intro σ
    change ρ σ 1 / 1 = 1
    simp
  map_mul' a b := by
    apply Subtype.ext
    apply ContinuousMap.ext
    intro σ
    change ρ σ (a * b) / (a * b) = (ρ σ a / a) * (ρ σ b / b)
    simp only [map_mul, div_eq_mul_inv, mul_inv_rev]
    ac_rfl

omit [Fintype K] in
theorem principal_apply (ρ : Gamma K →ₜ* MulAut M) (m : M) (σ : Gamma K) :
    (principalHom K M ρ m).val σ = ρ σ m / m := rfl

def differenceHom (ρ : Gamma K →ₜ* MulAut M) : M →* M where
  toFun m := ρ (frobenius K) m / m
  map_one' := by simp
  map_mul' a b := by
    simp only [map_mul, div_eq_mul_inv, mul_inv_rev]
    ac_rfl

/-- The standard quotient of all continuous crossed cocycles by all principal
cocycles. No restricted collection of representatives replaces this quotient. -/
abbrev CocycleClasses (ρ : Gamma K →ₜ* MulAut M) :=
  Cocycles K M ρ ⧸ (principalHom K M ρ).range

abbrev Coinvariants (ρ : Gamma K →ₜ* MulAut M) := M ⧸ (differenceHom K M ρ).range

/-- Evaluation descends to the standard class quotient without assuming finiteness. -/
def classEvaluationHom (ρ : Gamma K →ₜ* MulAut M) :
    CocycleClasses K M ρ →* Coinvariants K M ρ :=
  QuotientGroup.map _ _ (cocycleEvaluation K M ρ) (by
    rintro _ ⟨m, rfl⟩
    exact ⟨m, rfl⟩)

variable [Finite M]

theorem evaluation_principal (ρ : Gamma K →ₜ* MulAut M) (m : M) :
    cocycleEvaluationEquiv K M ρ (principalHom K M ρ m) = differenceHom K M ρ m := rfl

theorem evaluation_maps_principals (ρ : Gamma K →ₜ* MulAut M) :
    (principalHom K M ρ).range.map (cocycleEvaluationEquiv K M ρ).toMonoidHom =
      (differenceHom K M ρ).range := by
  ext m
  constructor
  · rintro ⟨c, ⟨a, rfl⟩, rfl⟩
    exact ⟨a, rfl⟩
  · rintro ⟨a, rfl⟩
    exact ⟨principalHom K M ρ a, ⟨a, rfl⟩, rfl⟩

/-- Evaluation gives the actual continuous-cocycle-class/coinvariant isomorphism. -/
def classEvaluationEquiv (ρ : Gamma K →ₜ* MulAut M) :
    CocycleClasses K M ρ ≃* Coinvariants K M ρ :=
  QuotientGroup.congr _ _ (cocycleEvaluationEquiv K M ρ) (evaluation_maps_principals K M ρ)

theorem class_evaluation_bijective (ρ : Gamma K →ₜ* MulAut M) :
    Function.Bijective (classEvaluationHom K M ρ) :=
  (classEvaluationEquiv K M ρ).bijective

theorem classEvaluation_mk (ρ : Gamma K →ₜ* MulAut M) (c : Cocycles K M ρ) :
    classEvaluationEquiv K M ρ (QuotientGroup.mk c) =
      QuotientGroup.mk (c.val (frobenius K)) := rfl

omit [Fintype K] [Finite M] in
/-- The exact representative relation is the actual principal-cocycle relation. -/
theorem classes_equal_iff (ρ : Gamma K →ₜ* MulAut M) (c d : Cocycles K M ρ) :
    (QuotientGroup.mk c : CocycleClasses K M ρ) = QuotientGroup.mk d ↔
      ∃ m : M, ∀ σ : Gamma K, c.val σ / d.val σ = ρ σ m / m := by
  rw [QuotientGroup.eq_iff_div_mem]
  constructor
  · rintro ⟨m, hm⟩
    refine ⟨m, fun σ => ?_⟩
    have hh := congrArg (fun a : Cocycles K M ρ => a.val σ) hm
    change ρ σ m / m = c.val σ / d.val σ at hh
    exact hh.symm
  · rintro ⟨m, hm⟩
    refine ⟨m, ?_⟩
    apply Subtype.ext
    apply ContinuousMap.ext
    intro σ
    exact (hm σ).symm

#print axioms principalCocycle
#print axioms evaluation_maps_principals
#print axioms classEvaluationEquiv
#print axioms classes_equal_iff
end
end ProcyclicGaloisCocycleAudit
