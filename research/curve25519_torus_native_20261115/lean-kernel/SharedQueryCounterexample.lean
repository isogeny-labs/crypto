import Mathlib.Data.Matrix.Mul
import Mathlib.Data.ZMod.Basic

set_option autoImplicit false
open Matrix
namespace Curve25519SharedQuery

example (b : ZMod 5) : b + -b = 0 := add_neg_cancel b

def rows : Matrix (Fin 2) (Fin 1) (ZMod 5) := fun _ _ => 1
def witness : Fin 2 → ZMod 5 := fun i => if i = 0 then 1 else -1

theorem shared_query_counterexample : ∀ b : ZMod 5,
    witness ≠ 0 ∧ witness ᵥ* rows = 0 ∧ witness ⬝ᵥ (fun _ => b) = 0 := by
  decide

#print axioms shared_query_counterexample
end Curve25519SharedQuery
