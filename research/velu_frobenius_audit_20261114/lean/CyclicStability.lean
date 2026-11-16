module

public import Mathlib.Tactic

@[expose] public section
namespace VeluFrobeniusAudit
noncomputable section
variable {G : Type*} [AddCommGroup G]

theorem cyclic_stable_iff_scalar (F : G →+ G) (P : G) :
    (∀ Q ∈ AddSubgroup.zmultiples P, F Q ∈ AddSubgroup.zmultiples P) ↔
      ∃ k : ℤ, k • P = F P := by
  constructor
  · intro h
    exact AddSubgroup.mem_zmultiples_iff.mp (h P (AddSubgroup.mem_zmultiples P))
  · rintro ⟨k, hk⟩ Q hQ
    obtain ⟨j, rfl⟩ := AddSubgroup.mem_zmultiples_iff.mp hQ
    apply AddSubgroup.mem_zmultiples_iff.mpr
    refine ⟨j * k, ?_⟩
    rw [map_zsmul, ← hk, mul_zsmul]

theorem order_three_members (P : G) (hP : addOrderOf P = 3) (Q : G) :
    Q ∈ AddSubgroup.zmultiples P → Q = 0 ∨ Q = P ∨ Q = -P := by
  rintro ⟨k, rfl⟩
  have h3 : (3 : ℕ) • P = 0 := by rw [← hP]; exact addOrderOf_nsmul_eq_zero P
  have hd : P + P = -P := by
    apply add_eq_zero_iff_eq_neg.mp
    simpa only [show (3 : ℕ) = 2 + 1 by rfl, add_nsmul, two_nsmul, one_nsmul] using h3
  change k • P = 0 ∨ k • P = P ∨ k • P = -P
  rw [← mod_addOrderOf_zsmul P k, hP]
  have h0 := Int.emod_nonneg k (show (3 : ℤ) ≠ 0 by decide)
  have hhi := Int.emod_lt_of_pos k (show (0 : ℤ) < 3 by decide)
  have hc : k % 3 = 0 ∨ k % 3 = 1 ∨ k % 3 = 2 := by omega
  rcases hc with h | h | h
  · left; simp [h]
  · right; left; simp [h]
  · right; right; simpa [h, two_zsmul] using hd

theorem mixed_point_outside_cyclic (F : G →+ G) (A B : G)
    (hA3 : (3 : ℕ) • A = 0) (hB3 : (3 : ℕ) • B = 0)
    (hA0 : A ≠ 0) (hB0 : B ≠ 0) (hP0 : A + B ≠ 0)
    (hFA : F A = A) (hFB : F B = -B) :
    addOrderOf (A + B) = 3 ∧ F (A + B) ∉ AddSubgroup.zmultiples (A + B) := by
  let _ : Fact (Nat.Prime 3) := ⟨by decide +kernel⟩
  have hP3 : (3 : ℕ) • (A + B) = 0 := by rw [nsmul_add, hA3, hB3, zero_add]
  have ho := addOrderOf_eq_prime hP3 hP0
  refine ⟨ho, ?_⟩
  intro hm
  rcases order_three_members (A + B) ho (F (A + B)) hm with h | h | h
  · have hfix : F (F (A + B)) = A + B := by simp [hFA, hFB]
    rw [h, map_zero] at hfix
    exact hP0 hfix.symm
  · have hc : -B = B := add_left_cancel (by simpa only [map_add, hFA, hFB] using h)
    have h2 : (2 : ℕ) • B = 0 := by
      rw [two_nsmul]
      exact (congrArg (fun X => B + X) hc.symm).trans (add_neg_cancel B)
    have hz := hB3
    rw [show (3 : ℕ) = 2 + 1 by rfl, add_nsmul, h2, zero_add, one_nsmul] at hz
    exact hB0 hz
  · have hc : A = -A := add_right_cancel (by
      simpa only [map_add, hFA, hFB, neg_add] using h)
    have h2 : (2 : ℕ) • A = 0 := by
      rw [two_nsmul]
      exact (congrArg (fun X => A + X) hc).trans (add_neg_cancel A)
    have hz := hA3
    rw [show (3 : ℕ) = 2 + 1 by rfl, add_nsmul, h2, zero_add, one_nsmul] at hz
    exact hA0 hz

#print axioms cyclic_stable_iff_scalar
#print axioms mixed_point_outside_cyclic
end
end VeluFrobeniusAudit
