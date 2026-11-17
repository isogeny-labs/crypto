import Mathlib.Tactic.Abel

/-!
The additive-grid control has a full two-core after its rectangle rows are
included, but the rectangles follow from the defining sum rows.  The
extension theorem states the exact elimination fact: every assignment of
the seed columns has a unique extension to the derived columns.  The
residual version shows the defining rows are independent as equations.
-/

namespace P192ExtensionCover

variable {I J G : Type*} [AddCommGroup G]

theorem additive_grid_unique_extension (A : I → G) (B : J → G) :
    ∃! C : I → J → G, ∀ i j, C i j = A i + B j := by
  refine ⟨(fun i j => A i + B j), ?_, ?_⟩
  · intro i j
    rfl
  · intro C h
    funext i j
    exact h i j

theorem additive_grid_arbitrary_residual (A : I → G) (B : J → G)
    (R : I → J → G) :
    ∃! C : I → J → G,
      ∀ i j, C i j - A i - B j = R i j := by
  refine ⟨(fun i j => R i j + A i + B j), ?_, ?_⟩
  · intro i j
    simp only [sub_eq_add_neg]
    abel
  · intro C h
    funext i j
    have hij := h i j
    apply sub_eq_zero.mp
    calc
      C i j - (R i j + A i + B j) =
          (C i j - A i - B j) - R i j := by abel
      _ = 0 := sub_eq_zero.mpr hij

theorem additive_grid_rectangle (A : I → G) (B : J → G)
    (C : I → J → G) (h : ∀ i j, C i j = A i + B j)
    (i k : I) (j l : J) :
    C i j + C k l - C k j - C i l = 0 := by
  rw [h i j, h k l, h k j, h i l]
  abel

#print axioms additive_grid_unique_extension
#print axioms additive_grid_arbitrary_residual
#print axioms additive_grid_rectangle

end P192ExtensionCover
