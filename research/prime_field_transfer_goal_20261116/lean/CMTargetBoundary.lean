import Mathlib.NumberTheory.Zsqrtd.Basic
import Mathlib.Data.ZMod.Basic
import Mathlib.Algebra.Polynomial.Basic
import Mathlib.Tactic.NormNum
import Mathlib.Tactic.Ring

/-! Literal CM parameter certificates. No point transport is postulated. -/
namespace CMTargetBoundary

def smallTwicePi : Zsqrtd (-3) := ⟨1, 5⟩
def smallAlpha : Zsqrtd (-3) := ⟨2, -5⟩

theorem small_twice_pi_norm : smallTwicePi.norm = 4 * 19 := by
  rw [Zsqrtd.norm_def]
  norm_num [smallTwicePi]

theorem small_alpha_norm : smallAlpha.norm = 79 := by
  rw [Zsqrtd.norm_def]
  norm_num [smallAlpha]

theorem small_product_norm : (smallTwicePi * smallAlpha).norm = (4 * 19) * 79 := by
  rw [Zsqrtd.norm_mul, small_twice_pi_norm, small_alpha_norm]

def largeTwicePi : Zsqrtd (-3) :=
  ⟨1, 1146303377458733895550677674601920593921⟩
def largeAlpha : Zsqrtd (-3) :=
  ⟨2, -1146303377458733895550677674601920593921⟩

theorem large_twice_pi_norm : largeTwicePi.norm =
    4 * 985508574879975417328988229747358102063188795599374534603903530670657036615681 := by
  rw [Zsqrtd.norm_def]
  norm_num [largeTwicePi]

theorem large_alpha_norm : largeAlpha.norm =
    3942034299519901669315952918989432408252755182397498138415614122682628146462727 := by
  rw [Zsqrtd.norm_def]
  norm_num [largeAlpha]

theorem target_orientation :
    (7 : ZMod 19)^2 + 7 + 1 = 0 ∧
    (11 : ZMod 19)^2 + 11 + 1 = 0 ∧
    (3 + 5 * 7 : ZMod 19) = 0 ∧
    (3 + 5 * 11 : ZMod 19) = 1 := by decide

theorem source_orientation :
    (55 : ZMod 79)^2 + 55 + 1 = 0 ∧
    (-3 - 10 * 55 : ZMod 79) = 0 := by decide

theorem parameter_norm_identity :
    1 + 3 * (2 * (Polynomial.X : Polynomial ℤ) - 1)^2 =
      4 * (3 * Polynomial.X^2 - 3 * Polynomial.X + 1) := by ring

theorem parameter_factor_identity :
    (3 * (Polynomial.X : Polynomial ℤ)^2 - 3 * Polynomial.X + 1) - 1 =
      3 * Polynomial.X * (Polynomial.X - 1) := by ring

end CMTargetBoundary
