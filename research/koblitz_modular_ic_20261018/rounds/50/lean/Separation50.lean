import Mathlib.Algebra.Ring.Hom.Basic
namespace Separation50
/-- Two evaluations agreeing on a subring but separating x exclude x from
that subring's image. -/
theorem evaluations_exclude_image
    {A B C : Type*} [Semiring A] [Semiring B] [Semiring C]
    (i : A →+* B) (e0 e1 : B →+* C)
    (hcomp : e0.comp i = e1.comp i) (x : B) (hsep : e0 x ≠ e1 x) :
    x ∉ Set.range i := by
  intro hx
  rcases hx with ⟨a, ha⟩
  apply hsep
  rw [← ha]
  exact DFunLike.congr_fun hcomp a
end Separation50
