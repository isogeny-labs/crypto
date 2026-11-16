module
public import FiniteQuotients
public import Mathlib.GroupTheory.SemidirectProduct
public import Mathlib.Topology.ContinuousMap.Algebra

@[expose] public section
namespace ProcyclicGaloisCocycleAudit
noncomputable section

variable (K : Type*) [Field K] [Fintype K]
variable (M : Type*) [CommGroup M] [TopologicalSpace M] [DiscreteTopology M]
variable [TopologicalSpace (MulAut M)] [DiscreteTopology (MulAut M)]

/-- All continuous crossed cocycles for the actual continuous action. -/
def cocycleSubgroup (ρ : Gamma K →ₜ* MulAut M) : Subgroup C(Gamma K, M) where
  carrier := {c | ∀ σ τ, c (σ * τ) = c σ * ρ σ (c τ)}
  one_mem' := by intro σ τ; simp
  mul_mem' := by
    intro c d hc hd σ τ
    change ∀ σ τ, c (σ * τ) = c σ * ρ σ (c τ) at hc
    change ∀ σ τ, d (σ * τ) = d σ * ρ σ (d τ) at hd
    simp only [ContinuousMap.mul_apply, map_mul]
    rw [hc, hd]
    ac_rfl
  inv_mem' := by
    intro c hc σ τ
    change ∀ σ τ, c (σ * τ) = c σ * ρ σ (c τ) at hc
    simp only [ContinuousMap.inv_apply, map_inv]
    rw [hc, mul_inv]

omit [Fintype K] [DiscreteTopology (MulAut M)] in
theorem mem_cocycleSubgroup (ρ : Gamma K →ₜ* MulAut M) (c : C(Gamma K, M)) :
    c ∈ cocycleSubgroup K M ρ ↔ ∀ σ τ, c (σ * τ) = c σ * ρ σ (c τ) := Iff.rfl

abbrev Cocycles (ρ : Gamma K →ₜ* MulAut M) := cocycleSubgroup K M ρ

omit [Fintype K] [DiscreteTopology (MulAut M)] in
theorem cocycle_identity (ρ : Gamma K →ₜ* MulAut M) (c : Cocycles K M ρ)
    (σ τ : Gamma K) : c.val (σ * τ) = c.val σ * ρ σ (c.val τ) :=
  (mem_cocycleSubgroup K M ρ c.val).mp c.property σ τ

omit [Fintype K] [DiscreteTopology (MulAut M)] in
theorem cocycle_at_one (ρ : Gamma K →ₜ* MulAut M) (c : Cocycles K M ρ) : c.val 1 = 1 := by
  have h := cocycle_identity K M ρ c 1 1
  have he : c.val 1 * 1 = c.val 1 * c.val 1 := by simpa using h
  exact (mul_left_cancel he).symm

abbrev CocycleExtension := M ⋊[MonoidHom.id (MulAut M)] MulAut M

local instance : TopologicalSpace (CocycleExtension M) := ⊥
local instance : DiscreteTopology (CocycleExtension M) := ⟨rfl⟩

def cocycleGraph (ρ : Gamma K →ₜ* MulAut M) (c : Cocycles K M ρ) :
    Gamma K →ₜ* CocycleExtension M where
  toFun σ := ⟨c.val σ, ρ σ⟩
  map_one' := SemidirectProduct.ext (cocycle_at_one K M ρ c) ρ.map_one
  map_mul' σ τ := SemidirectProduct.ext (cocycle_identity K M ρ c σ τ) (ρ.map_mul σ τ)
  continuous_toFun :=
    (continuous_of_discreteTopology :
      Continuous (fun a : M × MulAut M => (⟨a.1, a.2⟩ : CocycleExtension M))).comp
      (c.val.continuous.prodMk ρ.continuous)

theorem cocycle_eq_of_frobenius (ρ : Gamma K →ₜ* MulAut M)
    (c d : Cocycles K M ρ) (h : c.val (frobenius K) = d.val (frobenius K)) : c = d := by
  have hg : cocycleGraph K M ρ c = cocycleGraph K M ρ d :=
    hom_eq_of_frobenius K _ _ (by
      change (⟨c.val (frobenius K), ρ (frobenius K)⟩ : CocycleExtension M) =
        ⟨d.val (frobenius K), ρ (frobenius K)⟩
      exact SemidirectProduct.ext h rfl)
  apply Subtype.ext
  apply ContinuousMap.ext
  intro σ
  exact congrArg SemidirectProduct.left (congrArg (fun f => f σ) hg)

variable [Finite M]

/-- Existence is derived using an actual finite semidirect product; it does not
assume a procyclic cohomology formula or a prescribed cocycle. -/
theorem cocycle_with_value (ρ : Gamma K →ₜ* MulAut M) (m : M) :
    ∃ c : Cocycles K M ρ, c.val (frobenius K) = m := by
  let : Finite (MulAut M) :=
    Finite.of_injective (fun f : MulAut M => (f : M → M)) DFunLike.coe_injective
  let : Finite (CocycleExtension M) :=
    Finite.of_equiv (M × MulAut M) SemidirectProduct.equivProd.symm
  obtain ⟨f, hf⟩ := finite_target_extension (K := K) (CocycleExtension M) ⟨m, ρ (frobenius K)⟩
  let π : Gamma K →ₜ* MulAut M := {
    __ := (SemidirectProduct.rightHom : CocycleExtension M →* MulAut M).comp f.toMonoidHom
    continuous_toFun := (continuous_of_discreteTopology :
      Continuous (SemidirectProduct.right : CocycleExtension M → MulAut M)).comp f.continuous }
  have hπ : π = ρ := hom_eq_of_frobenius K π ρ (congrArg SemidirectProduct.right hf)
  have hr (σ : Gamma K) : (f σ).right = ρ σ := congrArg (fun a => a σ) hπ
  let c : C(Gamma K, M) := ⟨fun σ => (f σ).left,
    (continuous_of_discreteTopology :
      Continuous (SemidirectProduct.left : CocycleExtension M → M)).comp f.continuous⟩
  have hc : c ∈ cocycleSubgroup K M ρ := by
    apply (mem_cocycleSubgroup K M ρ c).mpr
    intro σ τ
    change (f (σ * τ)).left = (f σ).left * ρ σ ((f τ).left)
    rw [map_mul, SemidirectProduct.mul_left, hr]
    rfl
  exact ⟨⟨c, hc⟩, congrArg SemidirectProduct.left hf⟩

def cocycleEvaluation (ρ : Gamma K →ₜ* MulAut M) : Cocycles K M ρ →* M where
  toFun c := c.val (frobenius K)
  map_one' := rfl
  map_mul' _ _ := rfl

theorem cocycle_evaluation_bijective (ρ : Gamma K →ₜ* MulAut M) :
    Function.Bijective (cocycleEvaluation K M ρ) :=
  ⟨fun c d h => cocycle_eq_of_frobenius K M ρ c d h, cocycle_with_value K M ρ⟩

def cocycleEvaluationEquiv (ρ : Gamma K →ₜ* MulAut M) : Cocycles K M ρ ≃* M :=
  MulEquiv.ofBijective (cocycleEvaluation K M ρ) (cocycle_evaluation_bijective K M ρ)

#print axioms cocycle_eq_of_frobenius
#print axioms cocycle_with_value
#print axioms cocycleEvaluationEquiv
end
end ProcyclicGaloisCocycleAudit
