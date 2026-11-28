import Mathlib.GroupTheory.OrderOfElement
import Mathlib.NumberTheory.Divisors

/-! Conditional finite-group interpretation of the scalar certificates.
The concrete source Picard group and native arrays are not imported. -/
set_option maxRecDepth 100000

namespace ClassOrder37
variable {A : Type*} [AddGroup A]

theorem source_order_cases (x : A) (h : 40931 • x = 0)
    (h11 : 11 • x ≠ 0) (h61 : 61 • x ≠ 0) (h3721 : 3721 • x ≠ 0) :
    addOrderOf x = 671 ∨ addOrderOf x = 40931 := by
  have hd : addOrderOf x ∣ 40931 := addOrderOf_dvd_of_nsmul_eq_zero h
  have hm : addOrderOf x ∈ Nat.divisors 40931 := Nat.mem_divisors.mpr ⟨hd, by decide⟩
  have divisors : Nat.divisors 40931 = {1, 11, 61, 671, 3721, 40931} := by decide
  rw [divisors] at hm
  simp only [Finset.mem_insert, Finset.mem_singleton] at hm
  have hn1 : addOrderOf x ≠ 1 := by
    intro hx
    have hz := addOrderOf_nsmul_eq_zero x
    rw [hx, one_nsmul] at hz
    exact h11 (by simp [hz])
  have hn11 : addOrderOf x ≠ 11 := by
    intro hx
    exact h11 (by simpa only [hx] using addOrderOf_nsmul_eq_zero x)
  have hn61 : addOrderOf x ≠ 61 := by
    intro hx
    exact h61 (by simpa only [hx] using addOrderOf_nsmul_eq_zero x)
  have hn3721 : addOrderOf x ≠ 3721 := by
    intro hx
    exact h3721 (by simpa only [hx] using addOrderOf_nsmul_eq_zero x)
  rcases hm with h1 | h2 | h3 | h4 | h5 | h6
  · exact False.elim (hn1 h1)
  · exact False.elim (hn11 h2)
  · exact False.elim (hn61 h3)
  · exact Or.inl h4
  · exact False.elim (hn3721 h5)
  · exact Or.inr h6

theorem order_671_of_annihilation (x : A) (h : 40931 • x = 0)
    (h11 : 11 • x ≠ 0) (h61 : 61 • x ≠ 0) (h3721 : 3721 • x ≠ 0)
    (h671 : 671 • x = 0) : addOrderOf x = 671 := by
  rcases source_order_cases x h h11 h61 h3721 with horder | horder
  · exact horder
  · have hd : addOrderOf x ∣ 671 := addOrderOf_dvd_of_nsmul_eq_zero h671
    rw [horder] at hd
    exact False.elim ((by decide : ¬40931 ∣ 671) hd)

theorem order_40931_of_nonannihilation (x : A) (h : 40931 • x = 0)
    (h11 : 11 • x ≠ 0) (h61 : 61 • x ≠ 0) (h3721 : 3721 • x ≠ 0)
    (h671 : 671 • x ≠ 0) : addOrderOf x = 40931 := by
  rcases source_order_cases x h h11 h61 h3721 with horder | horder
  · exact False.elim (h671 (by simpa only [horder] using addOrderOf_nsmul_eq_zero x))
  · exact horder

theorem scalar_to_eleven_iff (x : A) (horder : addOrderOf x = 671) (k : ℕ) :
    11 • (k • x) = 0 ↔ 61 ∣ k := by
  rw [← mul_nsmul, ← addOrderOf_dvd_iff_nsmul_eq_zero, horder]
  change 11 * 61 ∣ k * 11 ↔ 61 ∣ k
  rw [Nat.mul_comm k 11]
  exact Nat.mul_dvd_mul_iff_left (by decide : 0 < 11)

theorem finite_projector_idempotent (x : A) (h : 671 • x = 0) :
    122 • (122 • x) = 122 • x := by
  rw [← mul_nsmul]
  have hnum : 122 * 122 = 122 + 671 * 22 := by decide
  rw [hnum, add_nsmul, mul_nsmul, h, nsmul_zero, add_zero]

end ClassOrder37
