import Std.Tactic
namespace TripleOrbit

-- Degree N=3(r-1)/2. Every index at or above r reflects to below r.
-- This proves the integer bound only, not the polynomial reversal identity.
theorem reflected_index_below (r n N : Nat) (hr : 0 < r)
    (hdegree : 2 * N = 3 * (r - 1)) (hindex : r ≤ n) :
    N - n < r := by
  omega

#print axioms reflected_index_below
end TripleOrbit
