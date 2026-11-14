module

public import Mathlib.Tactic

@[expose] public section
namespace TwoStepCollisionAudit
open scoped BigOperators
variable {I : Type*} [Fintype I] [DecidableEq I]

def secondMoment (p : I → ℝ) : ℝ := ∑ i, (p i)^2
def fourthMoment (p : I → ℝ) : ℝ := ∑ i, (p i)^4

/-- Source admissible degree-two labels: different last steps, and not
oppositely ordered copies of the same two steps. -/
def labelCoefficient (p : I → ℝ) : ℝ :=
  ∑ j : I, ∑ l : I, ∑ i : I, ∑ k : I,
    if j ≠ l ∧ ¬(k = j ∧ l = i) then p i * p j * p k * p l else 0

theorem off_diagonal_sum (f : I → ℝ) :
    (∑ i : I, ∑ j : I, if i = j then (0 : ℝ) else f i * f j) =
      (∑ i : I, f i)^2 - ∑ i : I, (f i)^2 := by
  have he (i j : I) : (if i = j then (0 : ℝ) else f i * f j) =
      f i * f j - (if i = j then (f i)^2 else 0) := by
    by_cases hij : i = j
    · subst j
      simp only [ite_true]
      ring
    · simp [hij]
  simp_rw [he]
  simp only [Finset.sum_sub_distrib, Finset.sum_ite_eq, Finset.mem_univ, ite_true,
    ← Finset.mul_sum, ← Finset.sum_mul]
  ring

theorem fixed_last_steps (p : I → ℝ) (hp : ∑ i, p i = 1) (j l : I) :
    (∑ i : I, ∑ k : I,
      if j ≠ l ∧ ¬(k = j ∧ l = i) then p i * p j * p k * p l else 0) =
      if j = l then 0 else p j * p l - (p j)^2 * (p l)^2 := by
  by_cases hjl : j = l
  · simp [hjl]
  · have he (i k : I) :
        (if j ≠ l ∧ ¬(k = j ∧ l = i) then p i * p j * p k * p l else 0) =
        p i * p j * p k * p l -
          (if l = i then (if k = j then p i * p j * p k * p l else 0) else 0) := by
      by_cases hli : l = i
      · by_cases hkj : k = j
        · have hb : ¬(j ≠ l ∧ ¬(k = j ∧ l = i)) := fun h => h.2 ⟨hkj, hli⟩
          rw [ite_eq_right hb, ite_eq_left hli, ite_eq_left hkj]
          ring
        · have hg : j ≠ l ∧ ¬(k = j ∧ l = i) := ⟨hjl, fun h => hkj h.1⟩
          rw [ite_eq_left hg, ite_eq_left hli, ite_eq_right hkj]
          ring
      · have hg : j ≠ l ∧ ¬(k = j ∧ l = i) := ⟨hjl, fun h => hli h.2⟩
        rw [ite_eq_left hg, ite_eq_right hli]
        ring
    have ht : (∑ i : I, ∑ k : I, p i * p j * p k * p l) = p j * p l := by
      have hm (i k : I) : p i * p j * p k * p l = (p j * p l) * (p i * p k) := by ring
      simp_rw [hm]
      simp only [← Finset.mul_sum, hp, mul_one]
    have hb : (∑ i : I, ∑ k : I,
        if l = i then (if k = j then p i * p j * p k * p l else 0) else 0) =
        p l * p j * p j * p l := by
      calc
        _ = ∑ i : I, if l = i then p i * p j * p j * p l else 0 := by
          apply Finset.sum_congr rfl
          intro i _
          by_cases hli : l = i <;> simp [hli]
        _ = _ := by simp
    simp_rw [he]
    simp only [Finset.sum_sub_distrib]
    rw [ht, hb, ite_eq_right hjl]
    ring

theorem degree_two_label_coefficient (p : I → ℝ) (hp : ∑ i, p i = 1) :
    labelCoefficient p =
      1 - secondMoment p - (secondMoment p)^2 + fourthMoment p := by
  unfold labelCoefficient
  simp_rw [fixed_last_steps p hp]
  have he (j l : I) :
      (if j = l then 0 else p j * p l - (p j)^2 * (p l)^2) =
      (if j = l then 0 else p j * p l) -
        (if j = l then 0 else (p j)^2 * (p l)^2) := by
    by_cases hjl : j = l <;> simp [hjl]
  simp_rw [he]
  simp only [Finset.sum_sub_distrib]
  rw [off_diagonal_sum p, off_diagonal_sum (fun i => (p i)^2), hp]
  unfold secondMoment fourthMoment
  have hs : (∑ i : I, ((p i)^2)^2) = ∑ i : I, (p i)^4 := by
    apply Finset.sum_congr rfl
    intro i _
    ring
  rw [hs]
  ring

#print axioms degree_two_label_coefficient
end TwoStepCollisionAudit
