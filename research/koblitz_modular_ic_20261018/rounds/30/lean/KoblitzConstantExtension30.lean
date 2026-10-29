import Mathlib.Tactic.Abel
import Mathlib.Data.Fin.Basic

/-! The finite-component augmentation distinction. Nonzero vectors below do
not assert nonprincipal divisors or nonzero Jacobian images. -/
namespace KoblitzConstantExtension30

def componentDifference (s : Fin 5) : Fin 5 → ℤ :=
  fun i => (if i=0 then 1 else 0) - (if i=s then 1 else 0)

theorem individual_component_nonzero (s : Fin 5) (hs : s ≠ 0) :
    componentDifference s ≠ 0 := by
  intro h
  have he := congrFun h 0
  simp [componentDifference, Ne.symm hs] at he

theorem complete_orbit_difference_zero {A : Type*} [AddCommGroup A]
    (a b c d e : A) :
    (a-b)+(b-c)+(c-d)+(d-e)+(e-a)=0 := by abel

end KoblitzConstantExtension30
