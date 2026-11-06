import Mathlib.Tactic

namespace TwistChord
variable {K : Type*} [Field K]

def quad (s p x : K) := x^2 - s*x + p
def cubic (c₁ c₂ c₃ x : K) := x^3-c₁*x^2+c₂*x-c₃
def remU (s p c₁ c₂ : K) := s^2-p-c₁*s+c₂
def remV (s p c₁ c₃ : K) := (c₁-s)*p-c₃

theorem curve_line (A B a b l x : K) (h : b^2=a^3+A*a+B) :
    x^3+A*x+B-(l*(x-a)-b)^2 =
      (x-a)*quad (l^2-a) (a*l^2+2*b*l+a^2+A) x := by
  unfold quad
  linear_combination -h

theorem cubic_remainder (s p c₁ c₂ c₃ x : K) :
    cubic c₁ c₂ c₃ x = (x+s-c₁)*quad s p x +
      remU s p c₁ c₂*x + remV s p c₁ c₃ := by
  unfold cubic quad remU remV
  ring

theorem vanishing_remainder_sound (s p c₁ c₂ c₃ x : K)
    (hq : quad s p x=0) (hu : remU s p c₁ c₂=0)
    (hv : remV s p c₁ c₃=0) : cubic c₁ c₂ c₃ x=0 := by
  rw [cubic_remainder, hq, hu, hv]
  ring

theorem distinct_roots_complete (s p c₁ c₂ c₃ x y : K)
    (hxy : x ≠ y) (hx : quad s p x=0) (hy : quad s p y=0)
    (fx : cubic c₁ c₂ c₃ x=0) (fy : cubic c₁ c₂ c₃ y=0) :
    remU s p c₁ c₂=0 ∧ remV s p c₁ c₃=0 := by
  have ex : remU s p c₁ c₂*x+remV s p c₁ c₃=0 := by
    linear_combination fx - (x+s-c₁)*hx - cubic_remainder s p c₁ c₂ c₃ x
  have ey : remU s p c₁ c₂*y+remV s p c₁ c₃=0 := by
    linear_combination fy - (y+s-c₁)*hy - cubic_remainder s p c₁ c₂ c₃ y
  have prod : remU s p c₁ c₂*(x-y)=0 := by linear_combination ex-ey
  have hu : remU s p c₁ c₂=0 := (mul_eq_zero.mp prod).resolve_right (sub_ne_zero.mpr hxy)
  exact ⟨hu, by simpa [hu] using ex⟩

theorem diagonal_quadratic (s p : K) (h2 : (2:K)≠0) :
    quad s p (s/2)=0 ↔ s^2-4*p=0 := by
  unfold quad
  constructor
  · intro h
    linear_combination -4*h
  · intro h
    field_simp
    linear_combination -h

#print axioms curve_line
#print axioms cubic_remainder
#print axioms vanishing_remainder_sound
#print axioms distinct_roots_complete
#print axioms diagonal_quadratic
end TwistChord
