import Std.Tactic

/-! Conditional algebraic components. This file does not formalize elliptic
curves, Frobenius/Verschiebung, the Boolean polynomial representation, solver
completeness or a complexity improvement. Native exact certificates supply
the separate fixed-instance representation checks. -/
namespace ICThreeLeads
open Lean.Grind

theorem quadratic_resultant_bezout {F : Type} [CommRing F]
    (a b c d e f z : F) :
    (a*f-c*d)^2 - (a*e-b*d)*(b*f-c*e) =
      (d*(a*e-b*d)*z + e*(a*e-b*d) - d*(a*f-c*d)) * (a*z^2+b*z+c) +
      (a*(a*f-c*d) - a*(a*e-b*d)*z - b*(a*e-b*d)) * (d*z^2+e*z+f) := by
  grind

theorem complementary_quotient_identity {F : Type} [CommRing F]
    (s f v : F) : (s*f^3)*(s*v^3) = s^2*(f*v)^3 := by
  grind

theorem complementary_cofactor_4232 {F : Type} [CommRing F]
    (f v : F) (h : f*v = 2) : (23*f^3)*(23*v^3) = 4232 := by
  grind

def combination {F : Type} [CommRing F] : List (F × F) → F
  | [] => 0
  | (a,b)::xs => a*b + combination xs

theorem certified_combination_vanishes {F : Type} [CommRing F]
    (xs : List (F × F)) (hz : ∀ p, p ∈ xs → p.2 = 0) :
    combination xs = 0 := by
  induction xs with
  | nil => rfl
  | cons p xs ih =>
    have hp : p.2 = 0 := hz p (by simp)
    have ht : ∀ q, q ∈ xs → q.2 = 0 := by
      intro q hq
      exact hz q (by simp [hq])
    simp [combination, hp, ih ht]
    grind

theorem replayed_consequence_vanishes {F : Type} [CommRing F]
    (row : F) (xs : List (F × F))
    (hreplay : row = combination xs) (hz : ∀ p, p ∈ xs → p.2 = 0) :
    row = 0 := by
  rw [hreplay]
  exact certified_combination_vanishes xs hz

/- Exact conditional bridge for the finite kernel certificate. Coverage is for
the independently specified h-multiplication kernel, not an assumed phi kernel.
The geometric Frobenius relation and coverage are separate obligations. -/
theorem complementary_map_kernel_certificate {A : Type}
    (phi psi hMul : A → A) (zero : A) (kernel : List A)
    (hcomp : ∀ x, psi (phi x) = hMul x)
    (hzero : psi zero = zero)
    (hcover : ∀ x, hMul x = zero ↔ x ∈ kernel)
    (hkill : ∀ x, x ∈ kernel → phi x = zero) (x : A) :
    phi x = zero ↔ x ∈ kernel := by
  constructor
  · intro hx
    apply (hcover x).mp
    rw [← hcomp x, hx, hzero]
  · exact hkill x

theorem subgroup_injective_from_complementarity {A : Type}
    (phi psi hMul : A → A) (domain : A → Prop)
    (hcomp : ∀ x, psi (phi x) = hMul x)
    (hinj : ∀ x y, domain x → domain y → hMul x = hMul y → x = y)
    (x y : A) (hx : domain x) (hy : domain y) (heq : phi x = phi y) :
    x = y := by
  apply hinj x y hx hy
  have h := congrArg psi heq
  simpa only [hcomp] using h

#print axioms quadratic_resultant_bezout
#print axioms complementary_quotient_identity
#print axioms complementary_cofactor_4232
#print axioms certified_combination_vanishes
#print axioms replayed_consequence_vanishes
#print axioms complementary_map_kernel_certificate
#print axioms subgroup_injective_from_complementarity
end ICThreeLeads
