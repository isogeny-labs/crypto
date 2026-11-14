module

public import Mathlib.Topology.Algebra.ContinuousMonoidHom
public import Mathlib.Topology.Compactness.Compact
public import Mathlib.GroupTheory.OrderOfElement

public section
namespace ContinuousCocycleAudit
noncomputable section
variable {P H : Type*} [Group P] [Group H]
variable [TopologicalSpace P] [TopologicalSpace H] [CompactSpace P]
variable [DiscreteTopology H] [IsMulTorsionFree H]

/-- A continuous map from a compact group has finite image in a discrete
codomain. A torsion-free group has no nonidentity finite-order image element. -/
theorem compact_continuous_hom_value (c : P →ₜ* H) (g : P) : c g=1 := by
  have hr : (Set.range c).Finite := (isCompact_range c.continuous).finite_of_discrete
  have : Finite c.toMonoidHom.range := hr.to_subtype
  have ht := isOfFinOrder_of_finite (⟨c g,⟨g,rfl⟩⟩ : c.toMonoidHom.range)
  apply IsOfFinOrder.eq_one'
  rw [isOfFinOrder_iff_pow_eq_one] at ht ⊢
  obtain ⟨n,hn,hpow⟩ := ht
  exact ⟨n,hn,congrArg Subtype.val hpow⟩

theorem compact_continuous_hom_trivial (c : P →ₜ* H) : c=1 := by
  ext g
  exact compact_continuous_hom_value c g

#print axioms compact_continuous_hom_value
#print axioms compact_continuous_hom_trivial
end
end ContinuousCocycleAudit
