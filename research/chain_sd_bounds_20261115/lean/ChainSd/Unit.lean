import Mathlib

/-!
# Unitness of the admissible family

Curve `E_a : y² = x³ + a x` over `F_p`, target abscissa `t`.  If `2^(k+1) ∣ p - 1` then every
element of `μ_N` (`N = 2^k`) is a square in `F_p`.  The theorem `S3_ne_zero_of_cm` shows that
for `t` a non-square with `t³ + a t` a nonzero square, the third Semaev polynomial
`S₃(x₁, x₂, t)` (for `B = 0`) does not vanish at any pair of nonzero squares `x₁, x₂`; hence
it vanishes nowhere on the grid `μ_N × μ_N`, and the chain system is a unit ideal (the
Nullstellensatz/CRT bridge is in the report).  `family_S3_ne_zero` specialises to the explicit
family `a = n²(n-1)`, `t = n`, `n` a non-square.

The proof is elementary: over an algebraic closure `K` take `y_i` with `y_i² = x_i³ + a x_i`.
If `S₃ = 0` then `t` is the abscissa `addX` of `(x₁,y₁) ± (x₂,y₂)` (`S3_factor`).  Mathlib's
`equation_negAdd` gives the ordinate `y₃` of that point with `y₃² = t³ + a t`, a nonzero square
of `F_p`, so `y₃` is rational and nonzero; the products `y₁y₃`, `y₂y₃` are rational, hence `y₁,
y₂` are rational, and the identity `x₁ x₂ · addX · (x₁-x₂)² = (x₁ y₂ - x₂ y₁)²` makes `t` a square.
-/

namespace ChainSd

open WeierstrassCurve.Affine

/-- The third Semaev polynomial of `y² = x³ + a x + b` with `b = 0`, in the target variable `t`. -/
def S3 {R : Type*} [CommRing R] (a t x1 x2 : R) : R :=
  (x1 - x2) ^ 2 * t ^ 2 - 2 * ((x1 + x2) * (x1 * x2 + a)) * t + (x1 * x2 - a) ^ 2

theorem map_S3 {R S : Type*} [CommRing R] [CommRing S] (f : R →+* S) (a t x1 x2 : R) :
    f (S3 a t x1 x2) = S3 (f a) (f t) (f x1) (f x2) := by
  simp [S3, map_ofNat]

/-- The curve `y² = x³ + a x`. -/
def curveCM {R : Type*} [CommRing R] (a : R) : WeierstrassCurve.Affine R :=
  { a₁ := 0, a₂ := 0, a₃ := 0, a₄ := a, a₆ := 0 }

theorem equation_curveCM {R : Type*} [CommRing R] (a x y : R) :
    (curveCM a).Equation x y ↔ y ^ 2 = x ^ 3 + a * x := by
  rw [equation_iff]
  simp [curveCM]

theorem negY_curveCM {R : Type*} [CommRing R] (a x y : R) : (curveCM a).negY x y = -y := by
  simp [negY, curveCM]

theorem addX_curveCM {R : Type*} [CommRing R] (a x1 x2 l : R) :
    (curveCM a).addX x1 x2 l = l ^ 2 - x1 - x2 := by
  simp [addX, curveCM]

theorem negAddY_curveCM {R : Type*} [CommRing R] (a x1 x2 y1 l : R) :
    (curveCM a).negAddY x1 x2 y1 l = l * (l ^ 2 - x1 - x2 - x1) + y1 := by
  simp [negAddY, addX, curveCM]

section Identities

variable {K : Type*} [Field K]

/-- `(x₁-x₂)² S₃(x₁,x₂,c)` factors through the two abscissae `(y₁ ∓ y₂)²/(x₁-x₂)² - x₁ - x₂`. -/
theorem S3_factor (a c x1 x2 y1 y2 : K) (h1 : y1 ^ 2 = x1 ^ 3 + a * x1)
    (h2 : y2 ^ 2 = x2 ^ 3 + a * x2) :
    (x1 - x2) ^ 2 * S3 a c x1 x2 =
      (c * (x1 - x2) ^ 2 - ((y1 - y2) ^ 2 - (x1 + x2) * (x1 - x2) ^ 2)) *
        (c * (x1 - x2) ^ 2 - ((y1 + y2) ^ 2 - (x1 + x2) * (x1 - x2) ^ 2)) := by
  have key : (c * (x1 - x2) ^ 2 - ((y1 - y2) ^ 2 - (x1 + x2) * (x1 - x2) ^ 2)) *
        (c * (x1 - x2) ^ 2 - ((y1 + y2) ^ 2 - (x1 + x2) * (x1 - x2) ^ 2)) =
      (c * (x1 - x2) ^ 2 + (x1 + x2) * (x1 - x2) ^ 2) ^ 2
        - 2 * (c * (x1 - x2) ^ 2 + (x1 + x2) * (x1 - x2) ^ 2) * (y1 ^ 2 + y2 ^ 2)
        + (y1 ^ 2 - y2 ^ 2) ^ 2 := by ring
  rw [key, h1, h2]
  unfold S3
  ring

/-- The 2-descent identity for `y² = x³ + a x`:
`x₁ x₂ · x(P₁+P₂) · (x₁-x₂)² = (x₁ y₂ - x₂ y₁)²`, written without division. -/
theorem descent_identity (a x1 x2 y1 y2 : K) (h1 : y1 ^ 2 = x1 ^ 3 + a * x1)
    (h2 : y2 ^ 2 = x2 ^ 3 + a * x2) :
    x1 * x2 * ((y1 - y2) ^ 2 - (x1 + x2) * (x1 - x2) ^ 2) = (x1 * y2 - x2 * y1) ^ 2 := by
  linear_combination (x1 * x2 - x2 ^ 2) * h1 + (x1 * x2 - x1 ^ 2) * h2

/-- Doubling: `(3x²+a)² - 8x(x³+ax) = (x²-a)²`. -/
theorem doubling_identity (a x : K) :
    (3 * x ^ 2 + a) ^ 2 - 8 * x * (x ^ 3 + a * x) = (x ^ 2 - a) ^ 2 := by ring

theorem S3_diag (a c x : K) : S3 a c x x = -4 * (x ^ 3 + a * x) * c + (x ^ 2 - a) ^ 2 := by
  unfold S3; ring

end Identities

section Rational

variable {F K : Type*} [Field F] [Field K] [Algebra F K]

/-- `y` is rational if it lies in the image of `F`. -/
def IsRat (F : Type*) {K : Type*} [Field F] [Field K] [Algebra F K] (y : K) : Prop :=
  ∃ r : F, algebraMap F K r = y

theorem isRat_algebraMap (r : F) : IsRat F (algebraMap F K r) := ⟨r, rfl⟩

theorem IsRat.add {y z : K} (hy : IsRat F y) (hz : IsRat F z) : IsRat F (y + z) := by
  obtain ⟨r, rfl⟩ := hy; obtain ⟨s, rfl⟩ := hz; exact ⟨r + s, by simp⟩

theorem IsRat.sub {y z : K} (hy : IsRat F y) (hz : IsRat F z) : IsRat F (y - z) := by
  obtain ⟨r, rfl⟩ := hy; obtain ⟨s, rfl⟩ := hz; exact ⟨r - s, by simp⟩

theorem IsRat.mul {y z : K} (hy : IsRat F y) (hz : IsRat F z) : IsRat F (y * z) := by
  obtain ⟨r, rfl⟩ := hy; obtain ⟨s, rfl⟩ := hz; exact ⟨r * s, by simp⟩

theorem IsRat.div {y z : K} (hy : IsRat F y) (hz : IsRat F z) : IsRat F (y / z) := by
  obtain ⟨r, rfl⟩ := hy; obtain ⟨s, rfl⟩ := hz; exact ⟨r / s, by simp⟩

theorem IsRat.neg {y : K} (hy : IsRat F y) : IsRat F (-y) := by
  obtain ⟨r, rfl⟩ := hy; exact ⟨-r, by simp⟩

theorem IsRat.pow {y : K} (hy : IsRat F y) (n : ℕ) : IsRat F (y ^ n) := by
  obtain ⟨r, rfl⟩ := hy; exact ⟨r ^ n, by simp⟩

theorem isRat_two : IsRat F (2 : K) := ⟨2, map_ofNat _ 2⟩

theorem isRat_three : IsRat F (3 : K) := ⟨3, map_ofNat _ 3⟩

/-- If `y z` and `z ≠ 0` are rational, so is `y`. -/
theorem IsRat.of_mul_right {y z : K} (hz : IsRat F z) (hz0 : z ≠ 0)
    (hyz : IsRat F (y * z)) : IsRat F y := by
  have : y = (y * z) / z := by rw [mul_div_assoc, div_self hz0, mul_one]
  rw [this]; exact hyz.div hz

end Rational

section Main

variable {p : ℕ} [hp : Fact p.Prime]

theorem two_ne_zero_of_ne_two (hp2 : p ≠ 2) : (2 : ZMod p) ≠ 0 := by
  intro h
  have h2 : ((2 : ℕ) : ZMod p) = 0 := by rw [Nat.cast_ofNat]; exact h
  rw [ZMod.natCast_eq_zero_iff] at h2
  exact hp2 ((Nat.prime_dvd_prime_iff_eq hp.out Nat.prime_two).mp h2)

/-- `t` is a square whenever `x₁ x₂ t (x₁-x₂)² = w²` with `x₁, x₂` nonzero squares and `x₁ ≠ x₂`. -/
theorem isSquare_of_descent {x1 x2 t w : ZMod p} (hx1 : IsSquare x1) (hx2 : IsSquare x2)
    (h1 : x1 ≠ 0) (h2 : x2 ≠ 0) (hD : x1 ≠ x2) (h : x1 * x2 * t * (x1 - x2) ^ 2 = w ^ 2) :
    IsSquare t := by
  obtain ⟨u1, rfl⟩ := hx1
  obtain ⟨u2, rfl⟩ := hx2
  have hu1 : u1 ≠ 0 := fun h0 => h1 (by simp [h0])
  have hu2 : u2 ≠ 0 := fun h0 => h2 (by simp [h0])
  have hD' : u1 * u1 - u2 * u2 ≠ 0 := sub_ne_zero.mpr hD
  have hden : u1 * u2 * (u1 * u1 - u2 * u2) ≠ 0 := mul_ne_zero (mul_ne_zero hu1 hu2) hD'
  refine ⟨w / (u1 * u2 * (u1 * u1 - u2 * u2)), ?_⟩
  rw [div_mul_div_comm, eq_div_iff (mul_ne_zero hden hden)]
  linear_combination h

/-- **Generic case.** Over a field `K ⊇ F_p`, suppose `t = addX` of two points with abscissae
`x₁ ≠ x₂` (nonzero squares) on `y² = x³ + a x`.  If `t³ + a t` is a nonzero square of `F_p`,
then `t` is a square. -/
theorem isSquare_of_addX {K : Type*} [Field K] [Algebra (ZMod p) K] (hp2 : p ≠ 2)
    (a t x1 x2 : ZMod p) (y1 y2 : K)
    (hy1 : y1 ^ 2 = algebraMap (ZMod p) K (x1 ^ 3 + a * x1))
    (hy2 : y2 ^ 2 = algebraMap (ZMod p) K (x2 ^ 3 + a * x2))
    (hD : x1 ≠ x2) (hx1 : IsSquare x1) (hx2 : IsSquare x2) (h1 : x1 ≠ 0) (h2 : x2 ≠ 0)
    (hft : IsSquare (t ^ 3 + a * t)) (hft0 : t ^ 3 + a * t ≠ 0)
    (hX : algebraMap (ZMod p) K t * (algebraMap (ZMod p) K x1 - algebraMap (ZMod p) K x2) ^ 2 =
      (y1 - y2) ^ 2 - (algebraMap (ZMod p) K x1 + algebraMap (ZMod p) K x2) *
        (algebraMap (ZMod p) K x1 - algebraMap (ZMod p) K x2) ^ 2) :
    IsSquare t := by
  classical
  set ι := algebraMap (ZMod p) K with hι
  have hinj : Function.Injective ι := (algebraMap (ZMod p) K).injective
  have hDK : ι x1 - ι x2 ≠ 0 := by
    intro h
    apply hD
    have h' : ι (x1 - x2) = ι 0 := by rw [map_sub, h, map_zero]
    exact sub_eq_zero.mp (hinj h')
  have h2K : (2 : K) ≠ 0 := by
    have e : (2 : K) = ι 2 := (map_ofNat ι 2).symm
    rw [e]; intro h
    have h' : ι 2 = ι 0 := by rw [h, map_zero]
    exact two_ne_zero_of_ne_two hp2 (hinj h')
  -- the curve over K and the two points
  set W := curveCM (ι a) with hW
  have hP1 : W.Equation (ι x1) y1 := by
    rw [hW, equation_curveCM, hy1]; simp
  have hP2 : W.Equation (ι x2) y2 := by
    rw [hW, equation_curveCM, hy2]; simp
  have hxy : ¬(ι x1 = ι x2 ∧ y1 = W.negY (ι x2) y2) := fun h => hD (hinj h.1)
  -- slope and sum abscissa
  have hslope : W.slope (ι x1) (ι x2) y1 y2 = (y1 - y2) / (ι x1 - ι x2) :=
    slope_of_X_ne (fun h => hD (hinj h))
  set l := (y1 - y2) / (ι x1 - ι x2) with hl
  have hsq : (y1 - y2) ^ 2 = (ι t + (ι x1 + ι x2)) * (ι x1 - ι x2) ^ 2 := by
    linear_combination (-1 : K) * hX
  have hl2 : l ^ 2 = ι t + (ι x1 + ι x2) := by
    rw [hl, div_pow, hsq, mul_div_assoc, div_self (pow_ne_zero 2 hDK), mul_one]
  have haddX : W.addX (ι x1) (ι x2) l = ι t := by
    rw [hW, addX_curveCM, hl2]; ring
  -- the ordinate of the sum point satisfies the equation
  have hE := equation_negAdd hP1 hP2 hxy
  rw [hslope, haddX, hW, equation_curveCM] at hE
  set y3 := (curveCM (ι a)).negAddY (ι x1) (ι x2) y1 l with hy3
  have hy3sq : y3 ^ 2 = ι (t ^ 3 + a * t) := by rw [hE]; simp
  -- y3 is rational and nonzero
  obtain ⟨s, hs⟩ := hft
  have hs0 : s ≠ 0 := by rintro rfl; simp at hs; exact hft0 hs
  have hy3' : y3 = ι s ∨ y3 = -ι s := by
    have : y3 ^ 2 = (ι s) ^ 2 := by rw [hy3sq, hs]; simp [sq]
    exact sq_eq_sq_iff_eq_or_eq_neg.mp this
  have hy3rat : IsRat (ZMod p) y3 := by
    rcases hy3' with h | h
    · rw [h]; exact isRat_algebraMap s
    · rw [h]; exact (isRat_algebraMap s).neg
  have hy30 : y3 ≠ 0 := by
    rcases hy3' with h | h <;> rw [h] <;> intro h0
    · exact hs0 (hinj (by rw [h0, map_zero]))
    · exact hs0 (hinj (by rw [neg_eq_zero.mp h0, map_zero]))
  -- y1 y2 is rational
  have hprod : IsRat (ZMod p) (y1 * y2) := by
    have e : y1 * y2 = (y1 ^ 2 + y2 ^ 2 - (y1 - y2) ^ 2) / 2 := by
      rw [eq_div_iff h2K]; ring
    rw [e, hy1, hy2, hsq]
    refine IsRat.div ?_ isRat_two
    refine IsRat.sub ((isRat_algebraMap _).add (isRat_algebraMap _)) ?_
    exact IsRat.mul ((isRat_algebraMap t).add ((isRat_algebraMap x1).add (isRat_algebraMap x2)))
      (((isRat_algebraMap x1).sub (isRat_algebraMap x2)).pow 2)
  -- explicit form of y3
  have hy3eq : y3 = (y1 - y2) * ((ι t - ι x1) / (ι x1 - ι x2)) + y1 := by
    rw [hy3, negAddY_curveCM]
    have e : l ^ 2 - ι x1 - ι x2 - ι x1 = ι t - ι x1 := by rw [hl2]; ring
    rw [e, hl]; ring
  have hα : IsRat (ZMod p) ((ι t - ι x1) / (ι x1 - ι x2)) :=
    ((isRat_algebraMap t).sub (isRat_algebraMap x1)).div
      ((isRat_algebraMap x1).sub (isRat_algebraMap x2))
  -- y1 and y2 are rational
  have hy1rat : IsRat (ZMod p) y1 := by
    refine IsRat.of_mul_right hy3rat hy30 ?_
    have e : y1 * y3 = (y1 ^ 2 - y1 * y2) * ((ι t - ι x1) / (ι x1 - ι x2)) + y1 ^ 2 := by
      rw [hy3eq]; ring
    rw [e, hy1]
    exact (((isRat_algebraMap _).sub hprod).mul hα).add (isRat_algebraMap _)
  have hy2rat : IsRat (ZMod p) y2 := by
    refine IsRat.of_mul_right hy3rat hy30 ?_
    have e : y2 * y3 = (y1 * y2 - y2 ^ 2) * ((ι t - ι x1) / (ι x1 - ι x2)) + y1 * y2 := by
      rw [hy3eq]; ring
    rw [e, hy2]
    exact ((hprod.sub (isRat_algebraMap _)).mul hα).add hprod
  obtain ⟨r1, hr1⟩ := hy1rat
  obtain ⟨r2, hr2⟩ := hy2rat
  -- the descent identity, pulled back to F_p
  have hdesc := descent_identity (ι a) (ι x1) (ι x2) y1 y2 (by rw [hy1]; simp) (by rw [hy2]; simp)
  rw [← hr1, ← hr2] at hdesc hsq
  have hF : x1 * x2 * t * (x1 - x2) ^ 2 = (x1 * r2 - x2 * r1) ^ 2 := by
    apply hinj
    simp only [map_mul, map_pow, map_sub]
    rw [← hdesc, hsq]
    ring
  exact isSquare_of_descent hx1 hx2 h1 h2 hD hF

/-- **Doubling case.** -/
theorem isSquare_of_doubling {K : Type*} [Field K] [Algebra (ZMod p) K] [IsAlgClosed K]
    (hp2 : p ≠ 2) (a t x : ZMod p) (ha : a ≠ 0) (hx : IsSquare x) (hx0 : x ≠ 0)
    (hft : IsSquare (t ^ 3 + a * t)) (hft0 : t ^ 3 + a * t ≠ 0)
    (hS : S3 a t x x = 0) : IsSquare t := by
  classical
  set ι := algebraMap (ZMod p) K with hι
  have hinj : Function.Injective ι := (algebraMap (ZMod p) K).injective
  have h2 : (2 : ZMod p) ≠ 0 := two_ne_zero_of_ne_two hp2
  have h2K : (2 : K) ≠ 0 := by
    have e : (2 : K) = ι 2 := (map_ofNat ι 2).symm
    rw [e]; intro h
    have h' : ι 2 = ι 0 := by rw [h, map_zero]
    exact h2 (hinj h')
  rw [S3_diag] at hS
  -- f(x) ≠ 0
  have hfx : x ^ 3 + a * x ≠ 0 := by
    intro h0
    have hsq : (x ^ 2 - a) ^ 2 = 0 := by rw [h0] at hS; linear_combination hS
    have hxa : x ^ 2 = a := by
      have := pow_eq_zero_iff (n := 2) (by norm_num) |>.mp hsq
      linear_combination this
    have e : x ^ 3 + a * x = 2 * a * x := by rw [← hxa]; ring
    rw [e] at h0
    rcases mul_eq_zero.mp h0 with h | h
    · rcases mul_eq_zero.mp h with h' | h'
      · exact h2 h'
      · exact ha h'
    · exact hx0 h
  have h4 : (4 : ZMod p) * (x ^ 3 + a * x) ≠ 0 := by
    apply mul_ne_zero _ hfx
    have e : (4 : ZMod p) = 2 * 2 := by norm_num
    rw [e]; exact mul_ne_zero h2 h2
  have ht : t = (x ^ 2 - a) ^ 2 / (4 * (x ^ 3 + a * x)) := by
    rw [eq_div_iff h4]
    linear_combination (-1 : ZMod p) * hS
  -- a point over K with abscissa x
  obtain ⟨y, hy⟩ : ∃ y : K, y ^ 2 = ι (x ^ 3 + a * x) := by
    obtain ⟨z, hz⟩ := IsAlgClosed.exists_eq_mul_self (ι (x ^ 3 + a * x))
    exact ⟨z, by rw [sq, ← hz]⟩
  have hfxK : ι (x ^ 3 + a * x) ≠ 0 := by
    intro h; apply hfx; apply hinj; rw [h, map_zero]
  have hy0 : y ≠ 0 := by
    intro h0; apply hfxK; rw [← hy, h0]; norm_num
  set W := curveCM (ι a) with hW
  have hP : W.Equation (ι x) y := by rw [hW, equation_curveCM, hy]; simp
  have hyneg : y ≠ W.negY (ι x) y := by
    rw [hW, negY_curveCM]; intro h
    have e : 2 * y = 0 := by linear_combination h
    rcases mul_eq_zero.mp e with h' | h'
    · exact h2K h'
    · exact hy0 h'
  have hxy : ¬(ι x = ι x ∧ y = W.negY (ι x) y) := fun h => hyneg h.2
  have hslope : W.slope (ι x) (ι x) y y = (3 * (ι x) ^ 2 + ι a) / (2 * y) := by
    rw [slope_of_Y_ne rfl hyneg, hW, negY_curveCM]
    have e1 : (curveCM (ι a)).a₁ = 0 := rfl
    have e2 : (curveCM (ι a)).a₂ = 0 := rfl
    have e4 : (curveCM (ι a)).a₄ = ι a := rfl
    rw [e1, e2, e4]
    congr 1 <;> ring
  set l := (3 * (ι x) ^ 2 + ι a) / (2 * y) with hl
  have hl2 : l ^ 2 - ι x - ι x = ι t := by
    rw [hl, ht, div_pow, mul_pow, hy, map_div₀, map_pow, map_sub, map_pow, map_mul, map_ofNat]
    have h4K : (4 : K) * ι (x ^ 3 + a * x) ≠ 0 := by
      apply mul_ne_zero _ hfxK
      rw [show (4 : K) = 2 * 2 by norm_num]; exact mul_ne_zero h2K h2K
    have h2sq : (2 : K) ^ 2 * ι (x ^ 3 + a * x) ≠ 0 := mul_ne_zero (pow_ne_zero 2 h2K) hfxK
    rw [div_sub' h2sq, div_sub' h2sq, div_eq_div_iff h2sq h4K]
    have e : ι (x ^ 3 + a * x) = (ι x) ^ 3 + ι a * ι x := by simp
    rw [e]
    ring
  have haddX : W.addX (ι x) (ι x) l = ι t := by rw [hW, addX_curveCM, hl2]
  have hE := equation_negAdd hP hP hxy
  rw [hslope, haddX, hW, equation_curveCM] at hE
  set y3 := (curveCM (ι a)).negAddY (ι x) (ι x) y l with hy3
  have hy3sq : y3 ^ 2 = ι (t ^ 3 + a * t) := by rw [hE]; simp
  obtain ⟨s, hs⟩ := hft
  have hs0 : s ≠ 0 := by rintro rfl; simp at hs; exact hft0 hs
  have hy3' : y3 = ι s ∨ y3 = -ι s := by
    have : y3 ^ 2 = (ι s) ^ 2 := by rw [hy3sq, hs]; simp [sq]
    exact sq_eq_sq_iff_eq_or_eq_neg.mp this
  have hy3rat : IsRat (ZMod p) y3 := by
    rcases hy3' with h | h
    · rw [h]; exact isRat_algebraMap s
    · rw [h]; exact (isRat_algebraMap s).neg
  have hy30 : y3 ≠ 0 := by
    rcases hy3' with h | h <;> rw [h] <;> intro h0
    · exact hs0 (hinj (by rw [h0, map_zero]))
    · exact hs0 (hinj (by rw [neg_eq_zero.mp h0, map_zero]))
  -- y y3 is rational, hence y is rational
  have hy3eq : y3 = l * (ι t - ι x) + y := by
    rw [hy3, negAddY_curveCM, hl2]
  have hyrat : IsRat (ZMod p) y := by
    refine IsRat.of_mul_right hy3rat hy30 ?_
    have hyl : y * l = (3 * (ι x) ^ 2 + ι a) / 2 := by
      rw [hl]; field_simp
    have e : y * y3 = (3 * (ι x) ^ 2 + ι a) / 2 * (ι t - ι x) + y ^ 2 := by
      rw [hy3eq, ← hyl]; ring
    rw [e, hy]
    refine IsRat.add (IsRat.mul (IsRat.div ?_ isRat_two) ((isRat_algebraMap t).sub (isRat_algebraMap x))) (isRat_algebraMap _)
    exact (isRat_three.mul ((isRat_algebraMap x).pow 2)).add (isRat_algebraMap a)
  obtain ⟨r, hr⟩ := hyrat
  -- f(x) = r² is a square, so t = (x²-a)²/(4 f(x)) is a square
  have hfr : x ^ 3 + a * x = r ^ 2 := by
    apply hinj; rw [map_pow, ← hy, ← hr]
  have hr0 : r ≠ 0 := by rintro rfl; simp at hfr; exact hfx hfr
  rw [ht, hfr]
  obtain ⟨u, rfl⟩ := hx
  refine ⟨((u * u) ^ 2 - a) / (2 * r), ?_⟩
  rw [div_mul_div_comm]
  congr 1 <;> ring

/-- **Unitness.** For `y² = x³ + a x`, a non-square `t` whose `t³ + a t` is a nonzero square, and
nonzero squares `x₁, x₂`, the Semaev polynomial `S₃(x₁, x₂, t)` is nonzero. -/
theorem S3_ne_zero_of_cm (hp2 : p ≠ 2) (a t x1 x2 : ZMod p) (ha : a ≠ 0) (ht : ¬IsSquare t)
    (hft : IsSquare (t ^ 3 + a * t)) (hft0 : t ^ 3 + a * t ≠ 0)
    (hx1 : IsSquare x1) (hx2 : IsSquare x2) (h1 : x1 ≠ 0) (h2 : x2 ≠ 0) :
    S3 a t x1 x2 ≠ 0 := by
  intro hS
  apply ht
  by_cases hD : x1 = x2
  · subst hD
    exact isSquare_of_doubling (K := AlgebraicClosure (ZMod p)) hp2 a t x1 ha hx1 h1 hft hft0 hS
  · let K := AlgebraicClosure (ZMod p)
    let ι := algebraMap (ZMod p) K
    obtain ⟨y1, hy1⟩ : ∃ y : K, y ^ 2 = ι (x1 ^ 3 + a * x1) := by
      obtain ⟨z, hz⟩ := IsAlgClosed.exists_eq_mul_self (ι (x1 ^ 3 + a * x1))
      exact ⟨z, by rw [sq, ← hz]⟩
    obtain ⟨y2, hy2⟩ : ∃ y : K, y ^ 2 = ι (x2 ^ 3 + a * x2) := by
      obtain ⟨z, hz⟩ := IsAlgClosed.exists_eq_mul_self (ι (x2 ^ 3 + a * x2))
      exact ⟨z, by rw [sq, ← hz]⟩
    have hfac := S3_factor (ι a) (ι t) (ι x1) (ι x2) y1 y2 (by rw [hy1]; simp) (by rw [hy2]; simp)
    have hS' : S3 (ι a) (ι t) (ι x1) (ι x2) = 0 := by rw [← map_S3, hS, map_zero]
    rw [hS', mul_zero] at hfac
    rcases mul_eq_zero.mp hfac.symm with h | h
    · -- t = addX for (x1,y1), (x2,y2)
      apply isSquare_of_addX (K := K) hp2 a t x1 x2 y1 y2 hy1 hy2 hD hx1 hx2 h1 h2 hft hft0
      linear_combination h
    · -- t = addX for (x1,y1), (x2,-y2)
      apply isSquare_of_addX (K := K) hp2 a t x1 x2 y1 (-y2) hy1 (by rw [neg_sq]; exact hy2)
        hD hx1 hx2 h1 h2 hft hft0
      linear_combination h

/-- Every element of `μ_{2^k}` is a square in `F_p` when `2^(k+1) ∣ p - 1`. -/
theorem isSquare_of_pow_eq_one {k : ℕ} (hk : 2 ^ (k + 1) ∣ p - 1) {z : ZMod p}
    (hz : z ^ (2 ^ k) = 1) : IsSquare z := by
  have hz0 : z ≠ 0 := by rintro rfl; simp at hz
  rw [ZMod.euler_criterion p hz0]
  obtain ⟨m, hm⟩ := hk
  have hp1 : 1 ≤ p := hp.out.one_lt.le
  have hpe : p = 2 ^ (k + 1) * m + 1 := (Nat.sub_eq_iff_eq_add hp1).mp hm
  have hp2 : p / 2 = 2 ^ k * m := by
    have e : 2 ^ (k + 1) * m + 1 = 2 * (2 ^ k * m) + 1 := by ring
    rw [hpe, e, Nat.mul_add_div (by norm_num : 0 < 2)]
    norm_num
  rw [hp2, pow_mul, hz, one_pow]

/-- `2^(k+1) ∣ p - 1` forces `p ≠ 2`. -/
theorem ne_two_of_dvd {k : ℕ} (hk : 2 ^ (k + 1) ∣ p - 1) : p ≠ 2 := by
  rintro rfl
  have h := Nat.le_of_dvd (by norm_num) hk
  have h2 := Nat.one_lt_two_pow (Nat.succ_ne_zero k)
  omega

/-- **The admissible family is unit.** For a non-square `n`, the curve `y² = x³ + n²(n-1) x` with
target `t = n` has `S₃(x₁, x₂, n) ≠ 0` for all `x₁, x₂ ∈ μ_{2^k}`, when `2^(k+1) ∣ p - 1`. -/
theorem family_S3_ne_zero {k : ℕ} (hk : 2 ^ (k + 1) ∣ p - 1) (n : ZMod p) (hn : ¬IsSquare n)
    (x1 x2 : ZMod p) (h1 : x1 ^ (2 ^ k) = 1) (h2 : x2 ^ (2 ^ k) = 1) :
    S3 (n ^ 2 * (n - 1)) n x1 x2 ≠ 0 := by
  have hn0 : n ≠ 0 := by rintro rfl; exact hn ⟨0, by simp⟩
  have hn1 : n ≠ 1 := by rintro rfl; exact hn ⟨1, by simp⟩
  have ha : n ^ 2 * (n - 1) ≠ 0 := mul_ne_zero (pow_ne_zero 2 hn0) (sub_ne_zero.mpr hn1)
  have hf : n ^ 3 + n ^ 2 * (n - 1) * n = (n ^ 2) ^ 2 := by ring
  have hx10 : x1 ≠ 0 := by rintro rfl; simp at h1
  have hx20 : x2 ≠ 0 := by rintro rfl; simp at h2
  refine S3_ne_zero_of_cm (ne_two_of_dvd hk) _ n x1 x2 ha hn ?_ ?_
    (isSquare_of_pow_eq_one hk h1) (isSquare_of_pow_eq_one hk h2) hx10 hx20
  · rw [hf]; exact ⟨n ^ 2, sq _⟩
  · rw [hf]; exact pow_ne_zero 2 (pow_ne_zero 2 hn0)

end Main

end ChainSd
