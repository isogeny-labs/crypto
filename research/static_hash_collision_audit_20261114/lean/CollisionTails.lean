import Survival
import FirstCollision
import Mathlib.Data.Fintype.Fin

namespace StaticHashCollisionAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit RhoFirstCollisionAudit

theorem least_collision_tail_iff (p : ℕ) [NeZero p] (f : ZMod p → ZMod p)
    (x : ZMod p) (t : ℕ) :
    t < firstCollision p f x ↔
      Function.Injective (fun i : Fin (t + 1) => f^[i.val] x) := by
  constructor
  · intro ht i j he
    apply Fin.ext
    by_contra hne
    rcases lt_or_gt_of_ne hne with hij | hji
    · have hj : j.val < firstCollision p f x := by have := j.isLt; omega
      exact no_earlier_collision p f x j.val hj ⟨⟨i.val, hij⟩, he⟩
    · have hi : i.val < firstCollision p f x := by have := i.isLt; omega
      exact no_earlier_collision p f x i.val hi ⟨⟨j.val, hji⟩, he.symm⟩
  · intro hi
    by_contra hn
    have hT : firstCollision p f x ≤ t := Nat.le_of_not_gt hn
    obtain ⟨j, hj⟩ := firstCollision_spec p f x
    have he := hi (a₁ := ⟨j.val, by have := j.isLt; omega⟩)
      (a₂ := ⟨firstCollision p f x, by omega⟩) hj
    have hv := congrArg Fin.val he
    change j.val = firstCollision p f x at hv
    have := j.isLt
    omega

theorem first_collision_tail_mass {I : Type*} [Fintype I] [DecidableEq I]
    (p : ℕ) [NeZero p] (q : I → ℝ) (hq : ∑ i, q i = 1)
    (U : ZMod p → I → ZMod p) (x : ZMod p) (t : ℕ) :
    expectation q (fun h : Hash (ZMod p) I =>
      if t < firstCollision p (staticStep U h) x then (1 : ℝ) else 0) =
      iidSurvival q U x t := by
  classical
  simp only [least_collision_tail_iff]
  exact static_survival_eq_iid q hq U x t

theorem bounded_tail_sum (p n : ℕ) (hn : n ≤ p) :
    (n : ℝ) = ∑ t : Fin p, if t.val < n then (1 : ℝ) else 0 := by
  have hc : (Finset.univ.filter (fun t : Fin p => t.val < n)).card = n := by
    rw [Fin.card_filter_val_lt, Nat.min_eq_right hn]
  calc
    _ = ((Finset.univ.filter (fun t : Fin p => t.val < n)).card : ℝ) := by rw [hc]
    _ = _ := by
      rw [Finset.card_filter, Nat.cast_sum]
      apply Finset.sum_congr rfl
      intro t _
      split_ifs <;> norm_num

/-- Exact expectation of the actual least repeat; a finite sum of actual iid
self-avoidance probabilities, rather than an independent-pair approximation. -/
theorem first_collision_expectation {I : Type*} [Fintype I] [DecidableEq I]
    (p : ℕ) [NeZero p] (q : I → ℝ) (hq : ∑ i, q i = 1)
    (U : ZMod p → I → ZMod p) (x : ZMod p) :
    expectation q (fun h : Hash (ZMod p) I =>
      (firstCollision p (staticStep U h) x : ℝ)) =
      ∑ t : Fin p, iidSurvival q U x t.val := by
  have he (h : Hash (ZMod p) I) := bounded_tail_sum p
    (firstCollision p (staticStep U h) x) (firstCollision_le p _ x)
  simp_rw [he]
  rw [expectation_sum]
  apply Finset.sum_congr rfl
  intro t _
  exact first_collision_tail_mass p q hq U x t.val

#print axioms least_collision_tail_iff
#print axioms first_collision_expectation
end
end StaticHashCollisionAudit
