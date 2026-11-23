import Std

/- Arithmetic dependencies of PRIME_LEFSCHETZ_ARGUMENT.md, section 2.
   This file does not define polynomial rings, Lefschetz modules, syzygies,
   first fall degree, last fall degree, or degree of regularity.
   All variables below range over natural numbers. -/
namespace N83LefschetzBoundary

-- r is coefficient degree, K the total primitive degree, k one block degree.
-- The equality avoids truncated subtraction concealing a negative r.
theorem exponent_within_string (ell d r K k : Nat)
    (hdegree : r + 4 + K = d) (hblock : k ≤ K)
    (hbound : 2 * d ≤ ell + 6) : r + 2 + 2 * k ≤ ell := by
  omega

theorem exponent_within_cap (ell d r K k : Nat)
    (hdegree : r + 4 + K = d) (hblock : k ≤ K)
    (hbound : 2 * d ≤ ell + 6) : r + 2 ≤ ell - 2 * k := by
  have := exponent_within_string ell d r K k hdegree hblock hbound
  omega

-- Every exponent e in q_j F_j is bounded by coefficient degree plus two.
-- Identifying actual polynomial exponents with this hypothesis remains open.
theorem monomial_exponent_within_cap (ell d r K k e : Nat)
    (hdegree : r + 4 + K = d) (hblock : k ≤ K)
    (hbound : 2 * d ≤ ell + 6) (hexponent : e ≤ r + 2) :
    e ≤ ell - 2 * k := by
  exact Nat.le_trans hexponent
    (exponent_within_cap ell d r K k hdegree hblock hbound)

theorem n83_through_degree16 (d r K k e : Nat)
    (hdegree : r + 4 + K = d) (hblock : k ≤ K)
    (hd : d ≤ 16) (hexponent : e ≤ r + 2) : e ≤ 27 - 2 * k := by
  apply monomial_exponent_within_cap 27 d r K k e hdegree hblock
  · omega
  · exact hexponent

theorem below_quartic_no_coefficient (d r K : Nat) (hd : d < 4) :
    r + 4 + K ≠ d := by
  omega

-- Odd width 2h+1: degree h+4, primitive degree h, coefficient degree zero.
-- Exponent two exceeds the cap one. This proves arithmetic sharpness only.
theorem odd_width_boundary (h : Nat) :
    0 + 4 + h = h + 4 ∧ 2 * h ≤ 2 * h + 1 ∧
    (2 * h + 1) - 2 * h = 1 ∧ 2 > (2 * h + 1) - 2 * h := by
  omega

theorem n83_degree17_boundary :
    0 + 4 + 13 = (17 : Nat) ∧ 27 - 2 * 13 = (1 : Nat) ∧
    2 > (27 : Nat) - 2 * 13 := by decide

-- Numerical prerequisite only: neither primality nor an SLP theorem.
theorem source_characteristic_above_width :
    (27 : Nat) < 4835703278761981908090881 := by decide

#print axioms exponent_within_string
#print axioms exponent_within_cap
#print axioms monomial_exponent_within_cap
#print axioms n83_through_degree16
#print axioms below_quartic_no_coefficient
#print axioms odd_width_boundary
#print axioms n83_degree17_boundary
#print axioms source_characteristic_above_width
end N83LefschetzBoundary
