import Mathlib
namespace RepeatedBase
/-- If g divides u², numerator precision modulo u³ determines its quotient modulo u. -/
theorem quotient_precision {R : Type*} [CommRing R] [IsDomain R]
    (u g v qa qb t : R) (hg : g ≠ 0)
    (hdiv : u * u = g * v)
    (hnum : g * qa - g * qb = u * u * u * t) :
    ∃ s : R, qa - qb = u * s := by
  refine ⟨v * t, ?_⟩
  apply mul_left_cancel₀ hg
  calc
    g * (qa - qb) = g * qa - g * qb := by ring
    _ = u * u * u * t := hnum
    _ = (g * v) * u * t := by rw [hdiv]
    _ = g * (u * (v * t)) := by ring
end RepeatedBase
#print axioms RepeatedBase.quotient_precision
