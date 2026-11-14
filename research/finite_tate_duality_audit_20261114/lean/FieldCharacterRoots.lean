module

public import Mathlib.GroupTheory.FiniteAbelian.Duality
public import Mathlib.FieldTheory.Finite.Basic

public section
namespace FiniteTateDualityAudit
noncomputable section
variable (K : Type*) [Field K] [Fintype K]

/-- Derive the root supply from the actual cyclic finite field unit group. -/
theorem field_card_roots : HasEnoughRootsOfUnity K (Fintype.card K-1) := by
  classical
  have : NeZero (Fintype.card K-1) := ⟨Nat.sub_ne_zero_of_lt Fintype.one_lt_card⟩
  obtain ⟨z,hz⟩ := IsCyclic.exists_generator (α:=Kˣ)
  have ho : orderOf z=Fintype.card K-1 := by
    rw [orderOf_eq_card_of_forall_mem_zpowers hz,Nat.card_eq_fintype_card,Fintype.card_units]
  refine ⟨⟨(z:K),?_⟩,rootsOfUnity.isCyclic K _⟩
  rw [IsPrimitiveRoot.coe_units_iff,IsPrimitiveRoot.iff_orderOf]
  exact ho

/-- The source annihilation condition supplies enough roots for every finite
abelian source group; no primitive root or duality is assumed as input. -/
theorem roots_of_annihilation {G : Type*} [CommGroup G]
    (h : ∀ g : G,g^(Fintype.card K-1)=1) :
    HasEnoughRootsOfUnity K (Monoid.exponent G) := by
  have : HasEnoughRootsOfUnity K (Fintype.card K-1) := field_card_roots K
  have : NeZero (Fintype.card K-1) := ⟨Nat.sub_ne_zero_of_lt Fintype.one_lt_card⟩
  exact HasEnoughRootsOfUnity.of_dvd K (Monoid.exponent_dvd_of_forall_pow_eq_one h)

#print axioms field_card_roots
#print axioms roots_of_annihilation
end
end FiniteTateDualityAudit
