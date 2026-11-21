import Mathlib.Data.ZMod.Basic
import Mathlib.Algebra.Group.Hom.Defs
import Mathlib.Tactic

namespace XedniFixedLiftIndex

variable {A : Type*} [AddGroup A]

instance : Fact (Nat.Prime 43) := ⟨by norm_num⟩

/-- Multiplication of a fixed rational lift reduces to multiplication of
its finite-field image. The map is the good-reduction homomorphism supplied
as an explicit hypothesis. -/
theorem reduction_of_multiple (reduce : A →+ ZMod 43) (Q : A) (k : ℕ) :
    reduce (k • Q) = k • reduce Q := by
  simp

/-- In a fixed nonzero `F₄₃` generator coordinate, each reduced target has
one scalar index. This is the index that a multiple-of-`Q` lift must use. -/
theorem unique_scalar_index (G T : ZMod 43) (hG : G ≠ 0) :
    ∃! k : ZMod 43, T + k * G = 0 := by
  refine ⟨-T / G, ?_, ?_⟩
  · field_simp [hG]
    ring
  · intro k hk
    have hw : T + (-T / G) * G = 0 := by
      field_simp [hG]
      ring
    have hm : k * G = (-T / G) * G :=
      add_left_cancel (hk.trans hw.symm)
    exact (mul_right_cancel₀ hG hm)

#print axioms reduction_of_multiple
#print axioms unique_scalar_index

end XedniFixedLiftIndex
