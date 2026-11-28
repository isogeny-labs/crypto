import Mathlib
namespace FamilyAdmission
/-- An elliptic-product order cannot hide a prime larger than each factor. -/
theorem prime_product_bound (l a b bound : ℕ) (hl : l.Prime)
    (ha : 0 < a) (hb : 0 < b) (haB : a ≤ bound) (hbB : b ≤ bound)
    (hdiv : l ∣ a * a * b) : l ≤ bound := by
  rcases hl.dvd_mul.mp hdiv with haa | hbb
  · rcases hl.dvd_mul.mp haa with hla | hla
    · exact le_trans (Nat.le_of_dvd ha hla) haB
    · exact le_trans (Nat.le_of_dvd ha hla) haB
  · exact le_trans (Nat.le_of_dvd hb hbb) hbB
end FamilyAdmission
#print axioms FamilyAdmission.prime_product_bound
