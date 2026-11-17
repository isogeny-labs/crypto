import Mathlib.Data.Nat.Choose.Basic

/- Counts of all monomials of total degree at most the indicated bound.
These are ambient spaces, not measured F4 widths or runtime estimates. -/
namespace FourTrackEnvelope
 theorem linear_control : Nat.choose (7+7) 7 = 3432 := by decide
 theorem xor_recoding : Nat.choose (7+6) 7 = 1716 := by decide
 theorem one_hot : Nat.choose (9+5) 9 = 2002 := by decide
 theorem dense_control : Nat.choose (2+9) 2 = 55 := by decide
end FourTrackEnvelope
