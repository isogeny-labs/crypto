import Std

/- Integer and modular coefficient lemmas. The bridge to an elliptic-curve
   subgroup and the correspondence with Rust are tested, not formalized. -/
namespace ApsCoefficients

def signedOnes : List Bool → Int
  | [] => 0
  | true :: xs => 1 + signedOnes xs
  | false :: xs => -1 + signedOnes xs

theorem signedOnes_parity (xs : List Bool) :
    signedOnes xs % 2 = (xs.length : Int) % 2 := by
  induction xs with
  | nil => decide
  | cons b xs ih =>
    cases b <;> simp only [signedOnes, List.length_cons] <;> omega

theorem odd_signedOnes_nonzero (xs : List Bool)
    (odd : (xs.length : Int) % 2 = 1) : signedOnes xs ≠ 0 := by
  have parity := signedOnes_parity xs
  omega

theorem signedOnes_bounds (xs : List Bool) :
    -(xs.length : Int) ≤ signedOnes xs ∧ signedOnes xs ≤ (xs.length : Int) := by
  induction xs with
  | nil => decide
  | cons b xs ih =>
    cases b <;> simp only [signedOnes, List.length_cons] <;> omega

def signedPowers : List Bool → Int
  | [] => 0
  | true :: xs => 1 + 2 * signedPowers xs
  | false :: xs => -1 + 2 * signedPowers xs

theorem nonempty_signedPowers_odd (b : Bool) (xs : List Bool) :
    signedPowers (b :: xs) % 2 = 1 := by
  cases b <;> simp only [signedPowers] <;> omega

theorem nonempty_signedPowers_nonzero (b : Bool) (xs : List Bool) :
    signedPowers (b :: xs) ≠ 0 := by
  have parity := nonempty_signedPowers_odd b xs
  omega

theorem bounded_nonzero_mod (b n : Int) (lo : -n < b) (hi : b < n)
    (nonzero : b ≠ 0) : b % n ≠ 0 := by
  by_cases nonnegative : 0 ≤ b
  · have reduced := Int.emod_eq_of_lt nonnegative hi
    omega
  · have lower : 0 ≤ b + n := by omega
    have upper : b + n < n := by omega
    have reduced := Int.emod_eq_of_lt lower upper
    rw [Int.add_emod_right] at reduced
    omega

theorem odd_signedOnes_nonzero_mod (xs : List Bool) (n : Int)
    (odd : (xs.length : Int) % 2 = 1) (bound : (xs.length : Int) < n) :
    signedOnes xs % n ≠ 0 := by
  have bounds := signedOnes_bounds xs
  exact bounded_nonzero_mod _ n (by omega) (by omega) (odd_signedOnes_nonzero xs odd)

theorem signedPowers_nonzero_mod (b : Bool) (xs : List Bool) (n : Int)
    (lo : -n < signedPowers (b :: xs)) (hi : signedPowers (b :: xs) < n) :
    signedPowers (b :: xs) % n ≠ 0 :=
  bounded_nonzero_mod _ n lo hi (nonempty_signedPowers_nonzero b xs)

#print axioms signedOnes_parity
#print axioms odd_signedOnes_nonzero
#print axioms signedOnes_bounds
#print axioms nonempty_signedPowers_odd
#print axioms nonempty_signedPowers_nonzero
#print axioms bounded_nonzero_mod
#print axioms odd_signedOnes_nonzero_mod
#print axioms signedPowers_nonzero_mod
end ApsCoefficients
