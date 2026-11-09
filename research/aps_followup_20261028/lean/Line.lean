import Lean

/-! Core-only coefficient and saturation checks. The EC chord-law and the
Rust polynomial builder are not formalized by this file. -/
namespace APSFollowup

variable {R : Type} [Lean.Grind.CommRing R]

def lineCubic (a b t c x : R) : R :=
  x ^ 3 - t ^ 2 * x ^ 2 + (a - 2 * t * c) * x + b - c ^ 2

theorem line_norm (a b t c x : R) :
    x ^ 3 + a * x + b - (t * x + c) ^ 2 = lineCubic a b t c x := by
  unfold lineCubic
  grind

theorem line_root_on_curve (a b t c x : R) (h : lineCubic a b t c x = 0) :
    (t * x + c) ^ 2 = x ^ 3 + a * x + b := by
  unfold lineCubic at h
  grind

theorem signed_line_same_cubic (a b t c x : R) :
    lineCubic a b (-t) (-c) x = lineCubic a b t c x := by
  unfold lineCubic
  grind

theorem orbit_label_sum (u lam : R) (h : lam ^ 2 + lam + 1 = 0) :
    u + lam * u + lam ^ 2 * u = 0 := by
  grind

theorem horizontal_cubic (b c x : R) :
    lineCubic 0 b 0 c x = x ^ 3 + b - c ^ 2 := by
  unfold lineCubic
  grind

-- Explicitly conditional on nontriviality: over the zero ring, inverse
-- equations would not imply nonzero t.
theorem saturation_excludes_zero (t z : R) (hne : (0 : R) ≠ 1)
    (h : t * z - 1 = 0) : t ≠ 0 := by
  intro ht
  grind

theorem saturation_keeps_inverse (t z : R) (h : t * z = 1) :
    t * z - 1 = 0 := by
  grind

#print axioms line_norm
#print axioms line_root_on_curve
#print axioms signed_line_same_cubic
#print axioms orbit_label_sum
#print axioms horizontal_cubic
#print axioms saturation_excludes_zero
#print axioms saturation_keeps_inverse

end APSFollowup
