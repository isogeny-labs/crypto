import Mathlib.Tactic

namespace XedniSquareModulo

/-- An integer square has a square residue modulo every positive modulus. -/
theorem square_has_modular_root (c C : ℤ) (m : ℕ) (hm : 0 < m)
    (hC : c^2 = C) :
    ∃ z : ℕ, z < m ∧ (((z : ℤ)^2 - C) % m = 0) := by
  let z : ℕ := (c % m).toNat
  have hznonneg : 0 ≤ c % m := Int.emod_nonneg _ (by omega)
  have hzlt : c % m < m := Int.emod_lt_of_pos _ (by omega)
  have hzcast : (z : ℤ) = c % m := by dsimp [z]; omega
  have hzbound : z < m := by omega
  let k : ℤ := c / m
  have hc : c = (z : ℤ) + (m : ℤ) * k := by
    calc
      c = c % m + (m : ℤ) * (c / m) := (Int.emod_add_mul_ediv c m).symm
      _ = (z : ℤ) + (m : ℤ) * k := by rw [hzcast]
  refine ⟨z, hzbound, ?_⟩
  calc
    ((z : ℤ)^2 - C) % m = ((z : ℤ)^2 - c^2) % m := by rw [hC]
    _ = ((z : ℤ)^2 - ((z : ℤ) + (m : ℤ) * k)^2) % m := by rw [hc]
    _ = ((m : ℤ) * (-2 * (z : ℤ) * k - (m : ℤ) * k^2)) % m := by ring_nf
    _ = 0 := by simp

#print axioms square_has_modular_root

end XedniSquareModulo
