module

public import Mathlib.FieldTheory.Finite.Basic
public import Mathlib.Tactic

@[expose] public section

namespace CubicPermutationCountAudit
variable {K : Type*} [Field K] [Fintype K]

def cubeRoot (x : K) : K := x ^ ((2 * Fintype.card K - 1) / 3)

theorem cube_exponent (hq : Fintype.card K % 3 = 2) :
    3 * ((2 * Fintype.card K - 1) / 3) = (Fintype.card K - 1) * 2 + 1 := by
  have hqpos : 1 < Fintype.card K := Fintype.one_lt_card
  omega

theorem cubeRoot_cube (hq : Fintype.card K % 3 = 2) (x : K) :
    cubeRoot (x ^ 3) = x := by
  have he := cube_exponent hq
  have hepos : 0 < (2 * Fintype.card K - 1) / 3 := by omega
  by_cases hx : x = 0
  · subst x
    simp [cubeRoot, hepos.ne']
  · rw [cubeRoot, ← pow_mul, he, pow_add, pow_mul,
      FiniteField.pow_card_sub_one_eq_one x hx]
    simp

theorem cube_cubeRoot (hq : Fintype.card K % 3 = 2) (x : K) :
    cubeRoot x ^ 3 = x := by
  simpa only [cubeRoot, ← pow_mul, Nat.mul_comm] using cubeRoot_cube hq x

noncomputable def cubeEquiv (hq : Fintype.card K % 3 = 2) : K ≃ K where
  toFun x := x ^ 3
  invFun := cubeRoot
  left_inv := cubeRoot_cube hq
  right_inv := cube_cubeRoot hq

theorem cube_bijective (hq : Fintype.card K % 3 = 2) :
    Function.Bijective (fun x : K => x ^ 3) :=
  (cubeEquiv hq).bijective

#print axioms cubeRoot_cube
#print axioms cube_cubeRoot
#print axioms cube_bijective
end CubicPermutationCountAudit
