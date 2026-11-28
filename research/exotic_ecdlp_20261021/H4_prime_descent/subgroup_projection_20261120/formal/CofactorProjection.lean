import Init

/-! An additive cofactor map transports every original three-point relation. -/

namespace H4CofactorProjection

theorem additive_map_preserves_three_sum
    {G H : Type} (addG : G → G → G) (addH : H → H → H)
    (f : G → H)
    (hadd : ∀ x y, f (addG x y) = addH (f x) (f y))
    (a b c R : G)
    (h : addG (addG a b) c = R) :
    addH (addH (f a) (f b)) (f c) = f R := by
  calc
    addH (addH (f a) (f b)) (f c) =
        addH (f (addG a b)) (f c) := by rw [hadd a b]
    _ = f (addG (addG a b) c) := (hadd (addG a b) c).symm
    _ = f R := by rw [h]

#print axioms H4CofactorProjection.additive_map_preserves_three_sum

end H4CofactorProjection
