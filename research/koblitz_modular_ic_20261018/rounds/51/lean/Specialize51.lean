import Mathlib.RingTheory.Ideal.Basic

namespace Specialize51

/-- Cancel a certified denominator unit in an ideal membership statement. -/
theorem mem_of_unit_mul_mem {R : Type*} [CommRing R] (I : Ideal R)
    {d x : R} (hd : IsUnit d) (hx : d * x ∈ I) : x ∈ I := by
  obtain ⟨u, hu⟩ := hd
  have h := I.mul_mem_left (↑(u⁻¹) : R) hx
  simpa [← hu, ← mul_assoc] using h

/-- The two cleared certificate products give these inclusions after any
specialization where d is a unit. Actual source arrays are external inputs. -/
theorem ideals_eq_of_unit_scaled_inclusions {R : Type*} [CommRing R]
    (I J : Ideal R) (d : R) (hd : IsUnit d)
    (hIJ : ∀ x, x ∈ I → d * x ∈ J)
    (hJI : ∀ x, x ∈ J → d^2 * x ∈ I) : I = J := by
  apply le_antisymm
  · intro x hx
    exact mem_of_unit_mul_mem J hd (hIJ x hx)
  · intro x hx
    exact mem_of_unit_mul_mem I (hd.pow 2) (hJI x hx)

/-- The native packet test checks three doublings. This is its group consequence;
actual point coordinates and the group-law identification remain external. -/
theorem seven_nsmul_zero_of_eight {G : Type*} [AddGroup G] (P : G)
    (h : (8 : ℕ) • P = P) : (7 : ℕ) • P = 0 := by
  have hcancel : (7 : ℕ) • P + P = 0 + P := by
    calc
      (7 : ℕ) • P + P = (7 + 1 : ℕ) • P := by simp only [add_nsmul, one_nsmul]
      _ = P := h
      _ = 0 + P := by simp only [zero_add]
  exact add_right_cancel hcancel

end Specialize51
