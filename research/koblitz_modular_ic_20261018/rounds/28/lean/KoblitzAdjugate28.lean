import Mathlib.Algebra.BigOperators.GroupWithZero.Finset
import Mathlib.Tactic.FieldSimp

/-! Finite-factor adjugate specialization. The identification with an actual
Hecke norm and unramified fiber remains a written geometric bridge. -/
namespace KoblitzAdjugate28
open scoped BigOperators
variable {ι K : Type*} [Fintype ι] [DecidableEq ι] [Field K]

def cofactor (f : ι → K) (i : ι) : K :=
  ∏ j ∈ Finset.univ.erase i, f j

def weightedAdjugate (f g : ι → K) : K :=
  ∑ i : ι, g i * cofactor f i

theorem specialize_at_zero (f g : ι → K) (i : ι) (hi : f i = 0) :
    weightedAdjugate f g = g i * cofactor f i := by
  unfold weightedAdjugate
  apply Finset.sum_eq_single i
  · intro j hj hji
    have hmem : i ∈ Finset.univ.erase j :=
      Finset.mem_erase.mpr ⟨Ne.symm hji, Finset.mem_univ i⟩
    have hz : cofactor f j = 0 := Finset.prod_eq_zero hmem hi
    rw [hz, mul_zero]
  · intro hnot
    exact False.elim (hnot (Finset.mem_univ i))

theorem cofactor_nonzero (f : ι → K) (i : ι)
    (hothers : ∀ j, j ≠ i → f j ≠ 0) : cofactor f i ≠ 0 := by
  unfold cofactor
  apply Finset.prod_ne_zero_iff.mpr
  intro j hj
  exact hothers j (Finset.mem_erase.mp hj).1

theorem recover_weighted_value (f g : ι → K) (i : ι)
    (hi : f i = 0) (hothers : ∀ j, j ≠ i → f j ≠ 0) :
    weightedAdjugate f g / weightedAdjugate f (fun _ => 1) = g i := by
  rw [specialize_at_zero f g i hi,
      specialize_at_zero f (fun _ => 1) i hi, one_mul]
  have hn := cofactor_nonzero f i hothers
  field_simp

end KoblitzAdjugate28
