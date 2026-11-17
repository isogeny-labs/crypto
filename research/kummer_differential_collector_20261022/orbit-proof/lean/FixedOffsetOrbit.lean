import Std

/-!
This file isolates the logical part of the fixed-offset sidecar argument.
`step` is translation by the negative of one fixed jump.  The theorem does
not assume a Kummer variety, a cyclic group, or a particular representation.
The native checker separately constructs the exact cyclic orbits used by the
336 frozen rows.
-/

namespace KummerFixedOffset

def iterate {α : Type} (step : α → α) (start : α) : Nat → α
  | 0 => start
  | n + 1 => step (iterate step start n)

theorem closed_contains_iterates {α : Type} {step : α → α}
    {stored : α → Prop} {start : α}
    (startStored : stored start)
    (closed : ∀ x, stored x → stored (step x)) :
    ∀ n, stored (iterate step start n) := by
  intro n
  induction n with
  | zero => exact startStored
  | succ n ih => exact closed _ ih

structure ExactOrbit (α : Type) (step : α → α) (start : α) (size : Nat) where
  point : Fin size → α
  point_eq_iterate : ∀ i, point i = iterate step start i.val
  injective : Function.Injective point

/- If a nonempty fixed-offset state is closed under the required predecessor
   translation and that translation has an exact orbit of `size`, every one
   of the `size` distinct orbit offsets must be stored. -/
theorem fixed_offset_orbit_requirement {α : Type} {step : α → α}
    {stored : α → Prop} {start : α} {size : Nat}
    (startStored : stored start)
    (closed : ∀ x, stored x → stored (step x))
    (orbit : ExactOrbit α step start size) :
    (∀ i, stored (orbit.point i)) ∧ Function.Injective orbit.point := by
  constructor
  · intro i
    rw [orbit.point_eq_iterate i]
    exact closed_contains_iterates startStored closed i.val
  · exact orbit.injective

#print axioms closed_contains_iterates
#print axioms fixed_offset_orbit_requirement

end KummerFixedOffset
