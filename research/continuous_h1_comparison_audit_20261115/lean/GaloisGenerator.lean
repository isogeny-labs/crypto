module
public import Mathlib.FieldTheory.AbsoluteGaloisGroup
public import Mathlib.FieldTheory.Galois.Profinite
public import Mathlib.FieldTheory.Finite.Extension
public import Mathlib.Topology.Algebra.Group.ClosedSubgroup
public import Mathlib.Topology.DenseEmbedding
public import Mathlib.Tactic

@[expose] public section
namespace ProcyclicGaloisCocycleAudit
noncomputable section
open Polynomial

variable (K : Type*) [Field K] [Fintype K]

abbrev Omega := AlgebraicClosure K
abbrev Gamma := Omega K ≃ₐ[K] Omega K

/-- Explicit identity with Mathlib's wrapped absolute Galois group. This avoids
mixing its derived group instance with the underlying automorphism instance. -/
def absoluteGaloisEquiv : Gamma K ≃ₜ* Field.absoluteGaloisGroup K where
  toFun σ := σ
  invFun σ := σ
  left_inv _ := rfl
  right_inv _ := rfl
  map_mul' _ _ := rfl
  continuous_toFun := continuous_id
  continuous_invFun := continuous_id

/-- The actual arithmetic q-power Frobenius of the actual algebraic closure. -/
def frobenius : Gamma K := FiniteField.frobeniusAlgEquivOfAlgebraic K (Omega K)

def asAlgEquiv (σ : Gamma K) : Omega K ≃ₐ[K] Omega K := σ

theorem frobenius_apply (a : Omega K) : asAlgEquiv K (frobenius K) a = a ^ Fintype.card K := rfl

theorem frobenius_fixed_iff (a : Omega K) :
    asAlgEquiv K (frobenius K) a = a ↔ ∃ b : K, algebraMap K (Omega K) b = a := by
  constructor
  · intro h
    have hs : (X ^ Fintype.card K - X : K[X]).Splits := by
      rw [Polynomial.splits_iff_card_roots, FiniteField.roots_X_pow_card_sub_X,
        ← Finset.card_def, Finset.card_univ,
        FiniteField.X_pow_card_sub_X_natDegree_eq K Fintype.one_lt_card]
    apply hs.mem_range_of_isRoot (FiniteField.X_pow_card_sub_X_ne_zero K Fintype.one_lt_card)
    simpa [Polynomial.IsRoot, sub_eq_zero, frobenius_apply] using h
  · rintro ⟨b, rfl⟩
    exact (asAlgEquiv K (frobenius K)).commutes b

def frobeniusClosure : ClosedSubgroup (Gamma K) where
  toSubgroup := (Subgroup.zpowers (frobenius K)).topologicalClosure
  isClosed' := Subgroup.isClosed_topologicalClosure _

theorem frobenius_closure_fixedField :
    IntermediateField.fixedField (frobeniusClosure K).toSubgroup = ⊥ := by
  apply le_antisymm
  · intro a ha
    have hf : frobenius K ∈ (frobeniusClosure K).toSubgroup :=
      Subgroup.le_topologicalClosure _ (Subgroup.mem_zpowers _)
    rw [IntermediateField.mem_fixedField_iff] at ha
    have hx := ha (asAlgEquiv K (frobenius K)) hf
    exact IntermediateField.mem_bot.mpr ((frobenius_fixed_iff K a).mp hx)
  · exact bot_le

theorem frobenius_closure_eq_top : (frobeniusClosure K).toSubgroup = ⊤ := by
  have h := InfiniteGalois.fixingSubgroup_fixedField (frobeniusClosure K)
  rw [frobenius_closure_fixedField, IntermediateField.fixingSubgroup_bot] at h
  exact h.symm

/-- Density is proved from actual fixed fields and the Krull Galois correspondence. -/
theorem frobenius_powers_dense : DenseRange (fun n : ℤ => frobenius K ^ n) := by
  have hs : Set.range (fun n : ℤ => frobenius K ^ n) =
      (Subgroup.zpowers (frobenius K) : Set (Gamma K)) := by
    ext g
    simp [Subgroup.mem_zpowers_iff]
  change Dense (Set.range (fun n : ℤ => frobenius K ^ n))
  rw [hs, dense_iff_closure_eq]
  change ((frobeniusClosure K).toSubgroup : Set (Gamma K)) = Set.univ
  rw [frobenius_closure_eq_top]
  rfl

/-- Uniqueness of continuous homomorphisms retains the Hausdorff target guard. -/
theorem hom_eq_of_frobenius {H : Type*} [Group H] [TopologicalSpace H] [T2Space H]
    (f g : Gamma K →ₜ* H) (h : f (frobenius K) = g (frobenius K)) : f = g := by
  have hh : (f : Gamma K → H) = g :=
    (frobenius_powers_dense K).equalizer f.continuous g.continuous (by
      funext n
      simp only [Function.comp_apply, map_zpow, h])
  exact DFunLike.ext' hh

#print axioms frobenius_fixed_iff
#print axioms frobenius_powers_dense
#print axioms hom_eq_of_frobenius
end
end ProcyclicGaloisCocycleAudit
