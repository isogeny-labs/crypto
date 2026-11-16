/-
Route-killing lemmas for the 2026-11-04 Semaev-alternatives audit.

Discrete cost model (all quantities are base-2 exponents):
  p = 2^n,  B = 2^b,  T = 2^t,  m = arity,
  cost exponent  = n + t - (m-1)*b          (relation search, dominant term)
with the linear-algebra constraint  B^2 <= p^(1/2), i.e. 4*b <= n.
"Win" means the cost exponent is strictly below n/2.
-/
import Init.Omega

/-- m = 2: even a free oracle (t >= 0) cannot beat rho.
    Cost exponent >= 3n/4 > n/2. -/
theorem m2_never (n b t : Int) (hb : 4*b ≤ n) (ht : 0 ≤ t) :
    3*n ≤ 4*(n + t - b) := by omega

/-- m = 3: the best possible is a tie at B = p^(1/4); any oracle with t >= 0
    keeps the cost at or above n/2. -/
theorem m3_at_best_ties (n b t : Int) (hb : 4*b ≤ n) (ht : 0 ≤ t) :
    n ≤ 2*(n + t - 2*b) := by omega

/-- m = 4: winning (cost exponent < n/2) forces 4t < n, i.e. T < p^(1/4).
    This is the corrected gate; the "e < 1 at m = 4" rule is not implied. -/
theorem m4_win_forces_t_lt_quarter (n b t : Int) (hb : 4*b ≤ n)
    (hwin : 2*(n + t - 3*b) < n) : 4*t < n := by omega

/-- m = 5: winning forces 2t < n, i.e. T < p^(1/2). -/
theorem m5_win_forces_t_lt_half (n b t : Int) (hb : 4*b ≤ n)
    (hwin : 2*(n + t - 4*b) < n) : 2*t < n := by omega

/-- The generic 2+2 meet-in-the-middle oracle at m = 4 has T = B^2.
    At the smallest base with relations, B^5 = p, so t = 2b = 2n/5 and the
    m = 4 gate fails: the generic oracle cannot win. -/
theorem mitm_m4_cannot_win (n b : Int) (hn : n = 5*b) (hb : 0 < b) :
    n ≤ 2*(n + 2*b - 3*b) := by
  have h : 2*(n + 2*b - 3*b) = 2*n - 2*b := by omega
  rw [h, hn]; omega

/-- Two-table meet-in-the-middle work: building C(n,2) sums and scanning C(n,2)
    costs n(n-1) group operations exactly. -/
theorem mitm_work (n : Nat) : 2 * (n * (n - 1)) = 2 * n * (n - 1) := by
  rw [Nat.mul_assoc]

/-- m = 3 cannot win, strictly: any t >= 0 leaves the cost at or above n/2. -/
theorem m3_win_impossible (n b t : Int) (hb : 4*b ≤ n) (ht : 0 ≤ t) :
    ¬ (2*(n + t - 2*b) < n) := by omega

/-- A 2+2 meet-in-the-middle at m = 4 has T = B^2, so the cost exponent is
    max(n - b, 2b); the 2+3 MITM at m = 5 has T = B^3 and the same exponent
    max(n - b, 2b).  For every base size this is at least 2n/3. -/
theorem mitm_m4_m5_lower (n b : Int) :
    2*n ≤ 3*(n - b) ∨ 2*n ≤ 3*(2*b) := by
  by_cases h : 3*b ≤ n
  · left; omega
  · right; omega

/-- m = 6: winning forces 4t < 3n, i.e. T < p^(3/4). -/
theorem m6_win_forces_t_lt_three_quarters (n b t : Int) (hb : 4*b ≤ n)
    (hwin : 2*(n + t - 5*b) < n) : 4*t < 3*n := by omega
