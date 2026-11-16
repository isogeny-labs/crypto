module
public import Mathlib.RepresentationTheory.Homological.ContCohomology.Basic
public import Mathlib.Topology.Instances.Int

@[expose] public section
namespace ContinuousH1ComparisonAudit
noncomputable section
open TopRep ContRepresentation

variable {k G : Type*} [Ring k] [TopologicalSpace k]
variable [Group G] [TopologicalSpace G] [IsTopologicalGroup G]
variable (X : TopRep k G)

omit [TopologicalSpace G] [IsTopologicalGroup G] in
theorem action_mul (g h : G) (a : X) : X.ρ (g * h) a = X.ρ g (X.ρ h a) := by
  rw [map_mul]
  rfl

abbrev CochainsZero := (homogeneousCochains X).X 0
abbrev CochainsOne := (homogeneousCochains X).X 1
abbrev KernelOne := ((homogeneousCochains X).d 1 2).hom.ker

/-- All continuous crossed cocycles, with no restriction on representatives. -/
def crossedCocycles : Submodule k C(G, X) where
  carrier := {c | ∀ g h, c (g * h) = c g + X.ρ g (c h)}
  zero_mem' := by intro g h; simp
  add_mem' := by
    intro a b ha hb g h
    change ∀ g h, a (g * h) = a g + X.ρ g (a h) at ha
    change ∀ g h, b (g * h) = b g + X.ρ g (b h) at hb
    simp only [ContinuousMap.add_apply, ha, hb, map_add]
    abel
  smul_mem' := by
    intro a c hc g h
    change ∀ g h, c (g * h) = c g + X.ρ g (c h) at hc
    simp only [ContinuousMap.smul_apply, hc, smul_add, map_smul]

theorem differential_zero_value (f : CochainsZero X) (x y : G) :
    (((homogeneousCochains X).d 0 1).hom f).val x y = f.val y - f.val x := by
  rw [homogeneousCochains.d_apply]
  rfl

theorem differential_one_value (f : CochainsOne X) (x y z : G) :
    (((homogeneousCochains X).d 1 2).hom f).val x y z =
      f.val y z - (f.val x z - f.val x y) := by
  rw [homogeneousCochains.d_apply]
  rfl

theorem invariant_one_value (f : CochainsOne X) (t x y : G) :
    f.val (t * x) (t * y) = X.ρ t (f.val x y) := by
  have he := congrArg (fun a : C(G, C(G, X)) => a (t * x) (t * y)) (f.property t)
  change X.ρ t (f.val (t⁻¹ * (t * x)) (t⁻¹ * (t * y))) = _ at he
  simpa only [inv_mul_cancel_left] using he.symm

theorem kernel_triangle (f : KernelOne X) (x y z : G) :
    f.val.val x z = f.val.val x y + f.val.val y z := by
  have he := congrArg (fun a : (homogeneousCochains X).X 2 => a.val x y z) f.property
  have hh : f.val.val y z - (f.val.val x z - f.val.val x y) = 0 :=
    (differential_one_value X f.val x y z).symm.trans he
  exact (sub_eq_iff_eq_add.mp (sub_eq_zero.mp hh).symm).trans (add_comm _ _)

def kernelToCrossed : KernelOne X →ₗ[k] crossedCocycles X where
  toFun f := by
    refine ⟨f.val.val 1, ?_⟩
    intro g h
    change f.val.val 1 (g * h) = f.val.val 1 g + X.ρ g (f.val.val 1 h)
    rw [kernel_triangle X f 1 g (g * h)]
    congr 1
    simpa only [mul_one] using invariant_one_value X f.val g 1 h
  map_add' _ _ := rfl
  map_smul' _ _ := rfl

variable (hρ : Continuous (fun a : G × X => X.ρ a.1 a.2))

/-- Actual continuous homogeneous cochain obtained by currying the joint map. -/
def crossedHomogeneousMap (c : crossedCocycles X) : C(G, C(G, X)) :=
  ContinuousMap.curry ⟨fun p : G × G => X.ρ p.1 (c.val (p.1⁻¹ * p.2)),
    hρ.comp (continuous_fst.prodMk (c.val.continuous.comp
      (continuous_fst.inv.mul continuous_snd)))⟩

theorem crossedHomogeneous_value (c : crossedCocycles X) (x y : G) :
    crossedHomogeneousMap X hρ c x y = X.ρ x (c.val (x⁻¹ * y)) := rfl

def crossedToCochain (c : crossedCocycles X) : CochainsOne X := by
  refine ⟨crossedHomogeneousMap X hρ c, ?_⟩
  intro t
  apply ContinuousMap.ext
  intro x
  apply ContinuousMap.ext
  intro y
  change X.ρ t (X.ρ (t⁻¹ * x) (c.val ((t⁻¹ * x)⁻¹ * (t⁻¹ * y)))) =
    X.ρ x (c.val (x⁻¹ * y))
  rw [← action_mul]
  simp only [mul_inv_rev, inv_inv, mul_assoc, mul_inv_cancel_left]

def crossedToKernel (c : crossedCocycles X) : KernelOne X := by
  refine ⟨crossedToCochain X hρ c, ?_⟩
  apply Subtype.ext
  apply ContinuousMap.ext
  intro x
  apply ContinuousMap.ext
  intro y
  apply ContinuousMap.ext
  intro z
  change (((homogeneousCochains X).d 1 2).hom (crossedToCochain X hρ c)).val x y z = 0
  rw [differential_one_value X (crossedToCochain X hρ c) x y z]
  change X.ρ y (c.val (y⁻¹ * z)) -
    (X.ρ x (c.val (x⁻¹ * z)) - X.ρ x (c.val (x⁻¹ * y))) = 0
  have hc := c.property (x⁻¹ * y) (y⁻¹ * z)
  change c.val ((x⁻¹ * y) * (y⁻¹ * z)) =
    c.val (x⁻¹ * y) + X.ρ (x⁻¹ * y) (c.val (y⁻¹ * z)) at hc
  simp only [mul_assoc, mul_inv_cancel_left] at hc
  rw [hc, map_add]
  have he (a : X) : X.ρ x (X.ρ (x⁻¹ * y) a) = X.ρ y a := by
    rw [← action_mul, mul_inv_cancel_left]
  rw [he]
  abel

def kernelCrossedEquiv : KernelOne X ≃ₗ[k] crossedCocycles X where
  toFun := kernelToCrossed X
  invFun := crossedToKernel X hρ
  left_inv f := by
    apply Subtype.ext
    apply Subtype.ext
    apply ContinuousMap.ext
    intro x
    apply ContinuousMap.ext
    intro y
    change X.ρ x (f.val.val 1 (x⁻¹ * y)) = f.val.val x y
    simpa only [mul_one, mul_inv_cancel_left] using
      (invariant_one_value X f.val x 1 (x⁻¹ * y)).symm
  right_inv c := by
    apply Subtype.ext
    apply ContinuousMap.ext
    intro g
    change X.ρ 1 (c.val (1⁻¹ * g)) = c.val g
    simp
  map_add' := (kernelToCrossed X).map_add
  map_smul' := (kernelToCrossed X).map_smul

#print axioms kernel_triangle
#print axioms kernelCrossedEquiv
end
end ContinuousH1ComparisonAudit
