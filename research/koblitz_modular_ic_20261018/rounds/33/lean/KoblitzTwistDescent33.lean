import Mathlib.Algebra.Module.LinearMap.Basic
import Mathlib.Algebra.CharP.Basic
import Mathlib.Tactic.Abel

/-! Universal characteristic-two descent operators. Native matrices and proper
RR/pole geometry are not imported as Lean certificates. -/
namespace KoblitzTwistDescent33
theorem add_self_char_two (K : Type*) {V : Type*} [Field K] [CharP K 2]
    [AddCommGroup V] [Module K V] (x : V) : x+x=0 := by
  have h2 : (2:K)=0 := CharP.cast_eq_zero K 2
  calc
    x+x = (2:K) • x := (two_smul K x).symm
    _ = 0 := by rw [h2, zero_smul]

variable {K V : Type*} [Field K] [CharP K 2] [AddCommGroup V] [Module K V]

def descend (N : V →ₗ[K] V) (t : K) (x : V) : V := x+t • N x

theorem descent_involutive (N : V →ₗ[K] V) (hN : ∀ x, N (N x)=0)
    (t : K) (x : V) : descend N t (descend N t x)=x := by
  simp only [descend, map_add, map_smul, hN, smul_zero, add_zero]
  rw [add_assoc, add_self_char_two K, add_zero]

theorem descent_cocycle (N : V →ₗ[K] V) (hN : ∀ x, N (N x)=0)
    (t : K) (x : V) :
    descend N (t+1) x + N (descend N (t+1) x) = descend N t x := by
  simp only [descend, map_add, map_smul, hN, smul_zero, add_zero, add_smul, one_smul]
  calc
    _ = (x+t • N x)+(N x+N x) := by abel
    _ = x+t • N x := by rw [add_self_char_two K,add_zero]

theorem descent_bijective (N : V →ₗ[K] V) (hN : ∀ x, N (N x)=0) (t : K) :
    Function.Bijective (descend N t) := by
  constructor
  · intro x y h
    have hh := congrArg (descend N t) h
    simpa only [descent_involutive N hN t] using hh
  · intro y
    exact ⟨descend N t y, descent_involutive N hN t y⟩

end KoblitzTwistDescent33
