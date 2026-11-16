import Mathlib.Tactic.Abel

/-! Formal group identities for the signed divisor calculation. No native
arrays, source geometry, Picard implementation or factor map is imported. -/
namespace SignedDivisor37
variable {A : Type*} [AddCommGroup A]

def weighted (x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 : A) : A :=
  4 • x0 + x1 - x2 + x3 - x4 - 4 • x5 - x6 + x7 - x8 + x9

theorem paired_weight_identity (x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 : A) :
    weighted x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 =
      4 • (x0 - x5) + (x1 - x6) - (x2 - x7) + (x3 - x8) - (x4 - x9) := by
  unfold weighted
  abel

theorem common_padding_cancels (q x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 : A) :
    weighted (x0 + q) (x1 + q) (x2 + q) (x3 + q) (x4 + q)
      (x5 + q) (x6 + q) (x7 + q) (x8 + q) (x9 + q) =
    weighted x0 x1 x2 x3 x4 x5 x6 x7 x8 x9 := by
  unfold weighted
  abel

end SignedDivisor37
