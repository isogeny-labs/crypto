import Mathlib.Tactic
import Mathlib.Algebra.Order.Group.Int.Sum

namespace XedniFirstHit

/-- A target-independent prefix of `m` candidate points can have at most
`m` distinct target hits. -/
theorem prefix_hit_count_le (m n : ℕ) (candidate : Fin m → Fin n) :
    (Finset.univ.image candidate).card ≤ m := by
  simpa using Finset.card_image_le (s := Finset.univ) (f := candidate)

/-- Before `n` candidates, a sequential list has not covered all `n`
distinct target points. -/
theorem exists_unhit_before_n (m n : ℕ) (hm : m < n)
    (candidate : Fin m → Fin n) :
    ∃ target : Fin n, ∀ i : Fin m, candidate i ≠ target := by
  classical
  by_contra h
  push Not at h
  have hsurj : Function.Surjective candidate := by
    intro target
    obtain ⟨i, hi⟩ := h target
    exact ⟨i, hi⟩
  have hcard : n ≤ m := by
    simpa using Fintype.card_le_of_surjective candidate hsurj
  omega

/-- Twice the sum of the first `n` positive integers. -/
private theorem twice_sum_positive_range (n : ℕ) :
    2 * (∑ i ∈ Finset.range n, ((i : ℤ) + 1)) = (n : ℤ) * ((n : ℤ) + 1) := by
  induction n with
  | zero => simp
  | succ n ih =>
    rw [Finset.sum_range_succ]
    push_cast
    nlinarith

/-- For a fixed candidate order that first hits every one of `n` targets,
the mean first-hit index is at least `(n+1)/2`. The first-hit map is
injective because one candidate can first hit at most one target. -/
theorem first_hit_sum_floor (n : ℕ) (first : Fin n → ℤ)
    (hinj : Function.Injective first) (hpos : ∀ i, 1 ≤ first i) :
    (n : ℤ) * ((n : ℤ) + 1) ≤ 2 * ∑ i, first i := by
  let s : Finset ℤ := Finset.univ.image first
  have hcard : s.card = n := by
    simp [s, Finset.card_image_of_injective _ hinj]
  have hspos : ∀ x ∈ s, (1 : ℤ) ≤ x := by
    intro x hx
    obtain ⟨i, _, rfl⟩ := Finset.mem_image.mp hx
    exact hpos i
  have hfloor := Finset.sum_range_le_sum (s := s) (c := (1 : ℤ)) hspos
  have hsum : (∑ x ∈ s, x) = ∑ i : Fin n, first i := by
    simpa [s] using (Finset.sum_image (s := (Finset.univ : Finset (Fin n)))
      (g := first) (f := fun x : ℤ => x) (by
        intro a _ b _ hab
        exact hinj hab))
  rw [hcard, hsum] at hfloor
  have hfloor' : (∑ i ∈ Finset.range n, ((i : ℤ) + 1)) ≤ ∑ i : Fin n, first i := by
    simpa only [add_comm] using hfloor
  have hformula := twice_sum_positive_range n
  nlinarith

#print axioms prefix_hit_count_le
#print axioms exists_unhit_before_n
#print axioms first_hit_sum_floor

end XedniFirstHit
