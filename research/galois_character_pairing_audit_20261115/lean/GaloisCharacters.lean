module
public import GaloisUnits
public import PerfectEvaluation

@[expose] public section
namespace GaloisCharacterPairingAudit
noncomputable section
open ProcyclicGaloisCocycleAudit

variable (K : Type*) [Field K] [Fintype K]
variable (M : Type*) [CommGroup M]
variable [TopologicalSpace M] [DiscreteTopology M]
variable [TopologicalSpace (MulAut M)] [DiscreteTopology (MulAut M)]

/-- Actual algebraic-closure-valued characters equivariant for every actual
Galois element, the invariant group in the source's Cartier-dual description. -/
def galoisCharacters (ρ : Gamma K →ₜ* MulAut M) : Subgroup (M →* (Omega K)ˣ) where
  carrier := {χ | ∀ σ m, χ (ρ σ m) = unitGalois K σ (χ m)}
  one_mem' := by intro σ m; simp
  mul_mem' := by
    intro χ ψ hχ hψ σ m
    change ∀ σ m, χ (ρ σ m) = unitGalois K σ (χ m) at hχ
    change ∀ σ m, ψ (ρ σ m) = unitGalois K σ (ψ m) at hψ
    simp only [MonoidHom.mul_apply, hχ, hψ, map_mul]
  inv_mem' := by
    intro χ hχ σ m
    change ∀ σ m, χ (ρ σ m) = unitGalois K σ (χ m) at hχ
    simp only [MonoidHom.inv_apply, hχ, map_inv]

omit [Fintype K] [TopologicalSpace M] [DiscreteTopology M]
  [DiscreteTopology (MulAut M)] in
theorem galois_character_equivariant (ρ : Gamma K →ₜ* MulAut M)
    (χ : galoisCharacters K M ρ) (σ : Gamma K) (m : M) :
    χ.val (ρ σ m) = unitGalois K σ (χ.val m) := χ.property σ m

def characterStabilizer (χ : M →* Kˣ) : Subgroup (MulAut M) where
  carrier := {g | ∀ m, χ (g m) = χ m}
  one_mem' := by intro m; rfl
  mul_mem' := by
    intro g h hg hh m
    change ∀ m, χ (g m) = χ m at hg
    change ∀ m, χ (h m) = χ m at hh
    change χ (g (h m)) = χ m
    rw [hg, hh]
  inv_mem' := by
    intro g hg m
    change ∀ m, χ (g m) = χ m at hg
    simpa using (hg (g⁻¹ m)).symm

local instance : TopologicalSpace Kˣ := ⊥
local instance : DiscreteTopology Kˣ := ⟨rfl⟩

omit [TopologicalSpace M] [DiscreteTopology M] in
theorem invariant_all_galois (ρ : Gamma K →ₜ* MulAut M)
    (χ : FiniteTateDualityAudit.invariantCharacters (ρ (frobenius K)) K)
    (σ : Gamma K) (m : M) : χ.val (ρ σ m) = χ.val m := by
  have hf : Continuous (fun g : Gamma K => χ.val (ρ g m)) :=
    (continuous_of_discreteTopology :
      Continuous (fun a : MulAut M => χ.val (a m))).comp ρ.continuous
  have he : (fun g : Gamma K => χ.val (ρ g m)) ∘
      (fun n : ℤ => ProcyclicGaloisCocycleAudit.frobenius K ^ n) =
      (fun _ : Gamma K => χ.val m) ∘
      (fun n : ℤ => ProcyclicGaloisCocycleAudit.frobenius K ^ n) := by
    funext n
    have hm : ρ (frobenius K) ∈ characterStabilizer K M χ.val := χ.property
    have hn := Subgroup.zpow_mem (characterStabilizer K M χ.val) hm n
    change ∀ m, χ.val ((ρ (frobenius K) ^ n) m) = χ.val m at hn
    simpa only [Function.comp_apply, map_zpow] using hn m
  exact congrFun ((frobenius_powers_dense K).equalizer hf continuous_const he) σ

def extendInvariant (ρ : Gamma K →ₜ* MulAut M)
    (χ : FiniteTateDualityAudit.invariantCharacters (ρ (frobenius K)) K) :
    galoisCharacters K M ρ := by
  refine ⟨baseCharacterMap K M χ.val, ?_⟩
  intro σ m
  change baseUnitMap K (χ.val (ρ σ m)) = unitGalois K σ (baseUnitMap K (χ.val m))
  rw [unitGalois_base, invariant_all_galois]

def descendGalois (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1) (χ : galoisCharacters K M ρ) :
    FiniteTateDualityAudit.invariantCharacters (ρ (frobenius K)) K := by
  refine ⟨(baseCharacterEquiv K M hM).symm χ.val, ?_⟩
  intro m
  apply baseUnitMap_injective K
  calc
    baseUnitMap K ((baseCharacterEquiv K M hM).symm χ.val (ρ (frobenius K) m)) =
        χ.val (ρ (frobenius K) m) := character_descent_value K M hM χ.val _
    _ = unitGalois K (frobenius K) (χ.val m) := galois_character_equivariant K M ρ χ _ _
    _ = unitGalois K (frobenius K)
        (baseUnitMap K ((baseCharacterEquiv K M hM).symm χ.val m)) := by
      rw [character_descent_value]
    _ = baseUnitMap K ((baseCharacterEquiv K M hM).symm χ.val m) := unitGalois_base K _ _

def galoisCharacterEquiv (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1) :
    galoisCharacters K M ρ ≃* FiniteTateDualityAudit.invariantCharacters (ρ (frobenius K)) K where
  toFun := descendGalois K M ρ hM
  invFun := extendInvariant K M ρ
  left_inv χ := by
    apply Subtype.ext
    exact (baseCharacterEquiv K M hM).apply_symm_apply χ.val
  right_inv χ := by
    apply Subtype.ext
    exact (baseCharacterEquiv K M hM).symm_apply_apply χ.val
  map_mul' χ ψ := by
    apply Subtype.ext
    exact (baseCharacterEquiv K M hM).symm.map_mul χ.val ψ.val

theorem galoisCharacterEquiv_value (ρ : Gamma K →ₜ* MulAut M)
    (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1) (χ : galoisCharacters K M ρ) (m : M) :
    baseUnitMap K ((galoisCharacterEquiv K M ρ hM χ).val m) = χ.val m :=
  character_descent_value K M hM χ.val m

#print axioms invariant_all_galois
#print axioms galoisCharacterEquiv
#print axioms galoisCharacterEquiv_value
end
end GaloisCharacterPairingAudit
