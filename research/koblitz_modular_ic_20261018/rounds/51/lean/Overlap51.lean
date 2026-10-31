import Mathlib.Tactic.Ring
namespace Overlap51

theorem first_square_identity {K : Type*} [CommRing K] (z v : K)
    (h2 : (2 : K) = 0) :
    (z+v)^2+z*(1+z+z^2+z*v)=v^2+z^2*v+z+z^3 := by
  calc
    _ = (v^2+z^2*v+z+z^3)+(2:K)*(z^2+z*v) := by ring
    _ = _ := by rw [h2, zero_mul, add_zero]

theorem second_square_identity {K : Type*} [CommRing K] (z v : K)
    (h2 : (2 : K) = 0) :
    (z+v+z^2)^2+z*(1+z+z^2+z^3+z*v)=v^2+z^2*v+z+z^3 := by
  calc
    _ = (v^2+z^2*v+z+z^3)+(2:K)*(z^2+z^4+z*v+z^3+z^2*v) := by ring
    _ = _ := by rw [h2, zero_mul, add_zero]

theorem overlap_norm_identity {K : Type*} [CommRing K] (z v : K)
    (h2 : (2 : K) = 0) :
    (z+v)*(z+v+z^2)+z*(z+1)=v^2+z^2*v+z+z^3 := by
  calc
    _ = (v^2+z^2*v+z+z^3)+(2:K)*(z^2+z*v) := by ring
    _ = _ := by rw [h2, zero_mul, add_zero]

/-- The native overlap identity supplies hz after passage to the curve;
hs is the defining inverse relation on its principal open. -/
theorem localized_base_inverse {K : Type*} [CommRing K] (z q h s : K)
    (hz : z*q=h^2) (hs : h*s=1) : z*(q*s^2)=1 := by
  calc
    _ = (z*q)*s^2 := by ring
    _ = (h*s)^2 := by rw [hz]; ring
    _ = 1 := by rw [hs]; ring
end Overlap51
