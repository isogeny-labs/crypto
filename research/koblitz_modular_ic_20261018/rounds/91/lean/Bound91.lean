import Std.Tactic

namespace Bound91

theorem a_bound (k i j e : Int) (hk0 : 0 <= k) (hk : k <= 37)
    (hi0 : 0 <= i) (hi : i <= 15) (hj0 : 0 <= j) (hj : j <= 1)
    (he : 0 <= e) : -61 * e <= (37 - k - 4 * i - j) * e := by
  have h : -61 <= 37 - k - 4 * i - j := by omega
  exact Int.mul_le_mul_of_nonneg_right h he

theorem b_bound (k i j e : Int) (hk0 : 0 <= k) (hk : k <= 37)
    (hi0 : 0 <= i) (hi : i <= 15) (hj0 : 0 <= j) (hj : j <= 1)
    (he : 0 <= e) : -17 * e <= (37 - k - i - 2 * j) * e := by
  have h : -17 <= 37 - k - i - 2 * j := by omega
  exact Int.mul_le_mul_of_nonneg_right h he

end Bound91
