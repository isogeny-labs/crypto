import Mathlib.Data.Fin.Basic
import Mathlib.Data.Fintype.Prod

set_option maxRecDepth 100000
set_option maxHeartbeats 6000000

namespace TwistKernelSymmetry

/- A concrete finite companion module over F7. These kernel-checked finite
   statements do not supply the elliptic-curve/torsion-basis bridge. -/
abbrev V := Fin 7 × Fin 7
def step (v : V) : V := (4 * v.2, v.1 + v.2)
def iterate : Nat → V → V
  | 0, v => v
  | n+1, v => step (iterate n v)

theorem eighth_scalar : ∀ v : V,
    iterate 8 v = (3*v.1, 3*v.2) := by decide

theorem forty_eighth_identity : ∀ v : V,
    iterate 48 v = v := by decide

theorem nonzero_point_period : ∀ v : V, v ≠ (0,0) →
    ∀ k : Fin 48, k ≠ 0 → iterate k.val v ≠ v := by decide

def sameLine (v w : V) : Prop :=
  ∃ c : Fin 7, c ≠ 0 ∧ w = (c*v.1, c*v.2)
instance (v w : V) : Decidable (sameLine v w) := inferInstanceAs
  (Decidable (∃ c : Fin 7, c ≠ 0 ∧ w = (c*v.1, c*v.2)))

theorem projective_period : ∀ v : V, v ≠ (0,0) →
    sameLine v (iterate 8 v) ∧
    ∀ k : Fin 8, k ≠ 0 → ¬sameLine v (iterate k.val v) := by decide

end TwistKernelSymmetry
