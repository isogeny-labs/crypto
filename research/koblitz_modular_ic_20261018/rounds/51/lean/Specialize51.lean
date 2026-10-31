import Mathlib.RingTheory.Ideal.Basic
import Mathlib.Tactic.Ring

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

/-- The exact line/curve intersection polynomial used by the native packet test. -/
theorem line_intersection_identity {K : Type*} [CommRing K]
    (A b l n r : K) (h2 : (2 : K) = 0) :
    (l*r+n)^2+A*r*(l*r+n)+b*(l*r+n)+r^3+b*r^2 =
      r^3+(b+l^2+A*l)*r^2+(A*n+b*l)*r+n^2+b*n := by
  calc
    _ = (r^3+(b+l^2+A*l)*r^2+(A*n+b*l)*r+n^2+b*n)
        +(2:K)*(l*r*n) := by ring
    _ = _ := by rw [h2, zero_mul, add_zero]

end Specialize51
