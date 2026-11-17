import Mathlib.Tactic.Ring

/- Algebraic kernel only: this file does not certify finite-table construction,
   memory cost, the Rust implementation, or an end-to-end performance claim. -/
namespace HecTableOracle

variable {K : Type*} [CommRing K]

theorem cube_coordinates (a b t n : K) (ht : t ^ 2 = n) :
    (a + b * t) ^ 3 =
      (a ^ 3 + 3 * n * a * b ^ 2) + (3 * a ^ 2 * b + n * b ^ 3) * t := by
  calc
    (a + b * t) ^ 3 =
      (a ^ 3 + 3 * a * b ^ 2 * t ^ 2) +
      (3 * a ^ 2 * b + b ^ 3 * t ^ 2) * t := by ring
    _ = _ := by rw [ht]; ring

theorem projective_cube (b t n : K) (ht : t ^ 2 = n) :
    (1 + b * t) ^ 3 = (1 + 3 * n * b ^ 2) + (3 * b + n * b ^ 3) * t := by
  simpa using cube_coordinates (1 : K) b t n ht

theorem lift_projective_cube (u a c d s : K)
    (hu : u ^ 3 = d * a) (hc : c ^ 3 * d = s) :
    (c * u) ^ 3 = s * a := by
  calc
    (c * u) ^ 3 = c ^ 3 * u ^ 3 := by ring
    _ = (c ^ 3 * d) * a := by rw [hu]; ring
    _ = s * a := by rw [hc]

theorem cardano_root (u v p q : K)
    (hp : 3 * u * v = -p) (hq : u ^ 3 + v ^ 3 = -q) :
    (u + v) ^ 3 + p * (u + v) + q = 0 := by
  calc
    (u + v) ^ 3 + p * (u + v) + q =
      (u ^ 3 + v ^ 3) + (3 * u * v + p) * (u + v) + q := by ring
    _ = 0 := by rw [hp, hq]; ring

theorem imaginary_cardano_factorization (x u v k n p q : K)
    (hk : k ^ 2 = -3 * n)
    (hp : p = -3 * (u ^ 2 - n * v ^ 2))
    (hq : q = -2 * (u ^ 3 + 3 * n * u * v ^ 2)) :
    (x - 2 * u) * (x + u - k * v) * (x + u + k * v) = x ^ 3 + p * x + q := by
  calc
    (x - 2 * u) * (x + u - k * v) * (x + u + k * v) =
      x ^ 3 + (-3 * u ^ 2 - k ^ 2 * v ^ 2) * x +
        (-2 * u ^ 3 + 2 * u * k ^ 2 * v ^ 2) := by ring
    _ = _ := by rw [hk, hp, hq]; ring

theorem depress_cubic (x s b c d : K) (h : b = 3 * s) :
    (x - s) ^ 3 + b * (x - s) ^ 2 + c * (x - s) + d =
      x ^ 3 + (c - 3 * s ^ 2) * x + (2 * s ^ 3 - c * s + d) := by
  rw [h]; ring

theorem depress_quartic (x s b c d e : K) (h : b = 4 * s) :
    (x - s) ^ 4 + b * (x - s) ^ 3 + c * (x - s) ^ 2 + d * (x - s) + e =
      x ^ 4 + (c - 6 * s ^ 2) * x ^ 2 + (d - 2 * c * s + 8 * s ^ 3) * x +
      (e - d * s + c * s ^ 2 - 3 * s ^ 4) := by
  rw [h]; ring

theorem ferrari_factorization (x w a b p q r : K)
    (hp : a + b - w ^ 2 = p) (hq : w * (b - a) = q) (hr : a * b = r) :
    (x ^ 2 + w * x + a) * (x ^ 2 - w * x + b) = x ^ 4 + p * x ^ 2 + q * x + r := by
  calc
    (x ^ 2 + w * x + a) * (x ^ 2 - w * x + b) =
      x ^ 4 + (a + b - w ^ 2) * x ^ 2 + (w * (b - a)) * x + a * b := by ring
    _ = _ := by rw [hp, hq, hr]

#print axioms projective_cube
#print axioms lift_projective_cube
#print axioms imaginary_cardano_factorization
#print axioms ferrari_factorization
end HecTableOracle
