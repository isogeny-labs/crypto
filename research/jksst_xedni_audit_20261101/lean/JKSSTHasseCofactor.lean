import JKSSTCoefficientProbability

/-!
# From a Hasse bound and subgroup cofactor to a `p/q` comparison

The probability audit naturally gives a denominator equal to the subgroup
order `q`.  Replacing it by the base-field parameter `p` requires an explicit
comparison.  This file derives one from two arithmetic hypotheses:

* a Hasse-style deviation bound for the curve order `N`; and
* an exact cofactor equation `N = H*q`.

No elliptic-curve API is assumed: the Hasse inequality and cofactor equation
are visible inputs.  The conservative bound valid from `p ≥ 4` is

`p ≤ 4*N = (4*H)*q`.

Tighter elementary variants `p ≤ 3*N` for `p ≥ 9` and `p ≤ 2*N` for
`p ≥ 16` are also proved.
-/

namespace JKSSTHasseCofactor

/-- Numerical form of Hasse's deviation bound
`|N - (p+1)| ≤ 2*sqrt(p)`. -/
def HasseBound (p N : ℕ) : Prop :=
  |(N : ℝ) - ((p : ℝ) + 1)| ≤ 2 * Real.sqrt (p : ℝ)

/-- The lower half of the Hasse interval. -/
theorem order_lower_bound_of_hasse
    {p N : ℕ} (hasse : HasseBound p N) :
    (p : ℝ) + 1 - 2 * Real.sqrt (p : ℝ) ≤ (N : ℝ) := by
  have lower := (abs_le.mp hasse).1
  linarith

/-- For `p ≥ 4`, the Hasse lower endpoint is at least `p/4`. -/
theorem quarter_base_le_hasse_lower_endpoint
    {p : ℕ} (hp : 4 ≤ p) :
    (p : ℝ) / 4 ≤
      (p : ℝ) + 1 - 2 * Real.sqrt (p : ℝ) := by
  let x := Real.sqrt (p : ℝ)
  have p_nonneg : (0 : ℝ) ≤ (p : ℝ) := by positivity
  have x_nonneg : 0 ≤ x := Real.sqrt_nonneg _
  have x_sq : x ^ 2 = (p : ℝ) := by
    change Real.sqrt (p : ℝ) ^ 2 = (p : ℝ)
    exact Real.sq_sqrt p_nonneg
  have hp_real : (4 : ℝ) ≤ (p : ℝ) := by exact_mod_cast hp
  have two_le_x : (2 : ℝ) ≤ x := by nlinarith
  have factor_nonneg : 0 ≤ (3 * x - 2) * (x - 2) :=
    mul_nonneg (by linarith) (by linarith)
  dsimp [x] at x_sq ⊢
  nlinarith

/-- Hasse alone gives `p ≤ 4*N` once `p ≥ 4`. -/
theorem base_le_four_mul_order
    {p N : ℕ} (hp : 4 ≤ p) (hasse : HasseBound p N) :
    p ≤ 4 * N := by
  have quarter := quarter_base_le_hasse_lower_endpoint hp
  have lower := order_lower_bound_of_hasse hasse
  have real_bound : (p : ℝ) ≤ 4 * (N : ℝ) := by
    nlinarith
  exact_mod_cast real_bound

/-- Combining Hasse with `N=H*q` supplies the exact size-comparison premise
needed by the probability theorem, with factor `4*H`. -/
theorem base_le_four_cofactor_mul_subgroup
    {p N H q : ℕ} (hp : 4 ≤ p) (hasse : HasseBound p N)
    (cofactor : N = H * q) :
    p ≤ (4 * H) * q := by
  calc
    p ≤ 4 * N := base_le_four_mul_order hp hasse
    _ = (4 * H) * q := by rw [cofactor]; ring

/-- For `p ≥ 9`, the constant `4` improves to `3`. -/
theorem base_le_three_mul_order
    {p N : ℕ} (hp : 9 ≤ p) (hasse : HasseBound p N) :
    p ≤ 3 * N := by
  let x := Real.sqrt (p : ℝ)
  have p_nonneg : (0 : ℝ) ≤ (p : ℝ) := by positivity
  have x_nonneg : 0 ≤ x := Real.sqrt_nonneg _
  have x_sq : x ^ 2 = (p : ℝ) := by
    change Real.sqrt (p : ℝ) ^ 2 = (p : ℝ)
    exact Real.sq_sqrt p_nonneg
  have hp_real : (9 : ℝ) ≤ (p : ℝ) := by exact_mod_cast hp
  have three_le_x : (3 : ℝ) ≤ x := by nlinarith
  have extra_nonneg : 0 ≤ 2 * x * (x - 3) := by positivity
  have lower := order_lower_bound_of_hasse hasse
  dsimp [x] at x_sq
  have real_bound : (p : ℝ) ≤ 3 * (N : ℝ) := by
    nlinarith
  exact_mod_cast real_bound

theorem base_le_three_cofactor_mul_subgroup
    {p N H q : ℕ} (hp : 9 ≤ p) (hasse : HasseBound p N)
    (cofactor : N = H * q) :
    p ≤ (3 * H) * q := by
  calc
    p ≤ 3 * N := base_le_three_mul_order hp hasse
    _ = (3 * H) * q := by rw [cofactor]; ring

/-- For `p ≥ 16`, the constant improves further to `2`. -/
theorem base_le_two_mul_order
    {p N : ℕ} (hp : 16 ≤ p) (hasse : HasseBound p N) :
    p ≤ 2 * N := by
  let x := Real.sqrt (p : ℝ)
  have p_nonneg : (0 : ℝ) ≤ (p : ℝ) := by positivity
  have x_nonneg : 0 ≤ x := Real.sqrt_nonneg _
  have x_sq : x ^ 2 = (p : ℝ) := by
    change Real.sqrt (p : ℝ) ^ 2 = (p : ℝ)
    exact Real.sq_sqrt p_nonneg
  have hp_real : (16 : ℝ) ≤ (p : ℝ) := by exact_mod_cast hp
  have four_le_x : (4 : ℝ) ≤ x := by nlinarith
  have extra_nonneg : 0 ≤ x * (x - 4) := by positivity
  have lower := order_lower_bound_of_hasse hasse
  dsimp [x] at x_sq
  have real_bound : (p : ℝ) ≤ 2 * (N : ℝ) := by
    nlinarith
  exact_mod_cast real_bound

theorem base_le_two_cofactor_mul_subgroup
    {p N H q : ℕ} (hp : 16 ≤ p) (hasse : HasseBound p N)
    (cofactor : N = H * q) :
    p ≤ (2 * H) * q := by
  calc
    p ≤ 2 * N := base_le_two_mul_order hp hasse
    _ = (2 * H) * q := by rw [cofactor]; ring

/-! ## Composition with the exact useful-relation probability bound -/

open JKSSTCoefficientProbability JKSSTProbability

variable {F ι : Type*} [Field F] [Fintype F] [Fintype ι]

/-- If `|F|` is the relevant prime subgroup order, `H*|F|` is the curve
order, and Hasse holds, the finite useful-event theorem obtains a certified
`/p` bound with numerator factor `4*H`.

The interpretation `|F| = q = ord(P)` is a separate prime-order cyclic-group
instantiation; this theorem only composes the displayed numeric hypotheses.
-/
theorem bounded_coefficient_success_from_hasse
    (x : F) (C : ℕ) (hC : C < ringChar F)
    (success : Finset ((ι → F) × (ι → F)))
    (hsuccess : success ⊆ badUsefulSamples x
      (reducedCandidates (F := F) (ι := ι) C))
    (p H : ℕ) (hp : 4 ≤ p)
    (hasse : HasseBound p (H * Fintype.card F)) :
    success.card * p ≤
      ((4 * H) * ((2 * C + 1) ^ Fintype.card ι - 1)) *
        (Fintype.card F ^ Fintype.card ι) ^ 2 := by
  apply bounded_coefficient_success_over_base_with_size_factor
    x C hC success hsuccess p (4 * H) (by omega)
  exact base_le_four_cofactor_mul_subgroup hp hasse rfl

end JKSSTHasseCofactor
