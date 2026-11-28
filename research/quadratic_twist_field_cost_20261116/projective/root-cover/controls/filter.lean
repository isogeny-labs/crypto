import Mathlib.Tactic

namespace TwistRootCover
variable {K : Type*} [Field K]

def cubic (a₃ a₂ a₁ a₀ x : K) : K := a₃*x^3+a₂*x^2+a₁*x+a₀
def cover (a₃ a₂ a₁ a₀ b₃ b₂ b₁ b₀ x : K) : K :=
  (b₃*a₂-a₃*b₂)*x^2+(b₃*a₁-a₃*b₁)*x+(b₃*a₀-a₃*b₀)

theorem original_input_identity (a₃ a₂ a₁ a₀ b₃ b₂ b₁ b₀ x : K) :
    b₃*cubic a₃ a₂ a₁ a₀ x-a₃*cubic b₃ b₂ b₁ b₀ x =
      cover a₃ a₂ a₁ a₀ b₃ b₂ b₁ b₀ x := by
  unfold cubic cover
  ring

theorem common_root_is_cover_root (a₃ a₂ a₁ a₀ b₃ b₂ b₁ b₀ x : K)
    (ha : cubic a₃ a₂ a₁ a₀ x = 0) (hb : cubic b₃ b₂ b₁ b₀ x = 0) :
    cover a₃ a₂ a₁ a₀ b₃ b₂ b₁ b₀ x = 0 := by
  rw [← original_input_identity, ha, hb]
  ring

theorem filtered_cover_exact (a₃ a₂ a₁ a₀ b₃ b₂ b₁ b₀ x : K) (ha₃ : a₃ ≠ 0) :
    (cubic a₃ a₂ a₁ a₀ x = 0 ∧ cubic b₃ b₂ b₁ b₀ x = 0) ↔
      (cover a₃ a₂ a₁ a₀ b₃ b₂ b₁ b₀ x = 1 ∧ cubic a₃ a₂ a₁ a₀ x = 0) := by
  constructor
  · rintro ⟨ha,hb⟩
    exact ⟨common_root_is_cover_root _ _ _ _ _ _ _ _ _ ha hb,ha⟩
  · rintro ⟨hr,ha⟩
    refine ⟨ha,?_⟩
    have h := original_input_identity a₃ a₂ a₁ a₀ b₃ b₂ b₁ b₀ x
    rw [ha,hr] at h
    have hz : a₃*cubic b₃ b₂ b₁ b₀ x = 0 := by linear_combination -h
    exact (mul_eq_zero.mp hz).resolve_left ha₃

theorem discriminant_identity (a b c x : K) :
    (2*a*x+b)^2-(b^2-4*a*c) = 4*a*(a*x^2+b*x+c) := by ring

theorem quadratic_root_formula (a b c r : K) (ha : a ≠ 0) (h₂ : (2:K) ≠ 0)
    (hr : r^2=b^2-4*a*c) :
    a*((-b+r)/(2*a))^2+b*((-b+r)/(2*a))+c=0 := by
  field_simp [ha,h₂]
  linear_combination hr

theorem quadratic_roots_complete (a b c r x : K) (ha : a ≠ 0) (h₂ : (2:K) ≠ 0)
    (hr : r^2=b^2-4*a*c) :
    a*x^2+b*x+c=0 ↔ x=(-b+r)/(2*a) ∨ x=(-b-r)/(2*a) := by
  have hden : (2:K)*a ≠ 0 := mul_ne_zero h₂ ha
  constructor
  · intro hx
    have hp : (2*a*x+b-r)*(2*a*x+b+r)=0 := by
      linear_combination 4*a*hx-hr
    rcases mul_eq_zero.mp hp with hl|hright
    · left
      apply (eq_div_iff hden).2
      linear_combination hl
    · right
      apply (eq_div_iff hden).2
      linear_combination hright
  · rintro (hx|hx)
    · rw [hx]
      exact quadratic_root_formula a b c r ha h₂ hr
    · rw [hx]
      have hm : (-r)^2=b^2-4*a*c := by simpa using hr
      simpa [sub_eq_add_neg] using quadratic_root_formula a b c (-r) ha h₂ hm

end TwistRootCover

#print axioms TwistRootCover.original_input_identity
#print axioms TwistRootCover.common_root_is_cover_root
#print axioms TwistRootCover.filtered_cover_exact
#print axioms TwistRootCover.discriminant_identity
#print axioms TwistRootCover.quadratic_root_formula
#print axioms TwistRootCover.quadratic_roots_complete
