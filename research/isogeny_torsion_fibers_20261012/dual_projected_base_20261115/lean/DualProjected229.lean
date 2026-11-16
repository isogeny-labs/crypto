import Init

set_option maxRecDepth 8192
set_option maxHeartbeats 0

namespace DualProjected229

/-- The inverse of the degree-three action on the order-229 scalar group. -/
theorem inverseThree : (153 * 3 : Fin 229) = 1 := by decide

theorem inverseThreeAction : ∀ y : Fin 229, 153 * (3 * y) = y := by decide

/-- Conditional dual transport on the order-229 scalar group. `forward` and
`dual` stand for maps between the source and destination subgroup coordinate
groups. The premises expose the map composition and scalar-linearity bridges
that must be checked for the concrete elliptic-curve isogenies. -/
theorem projectedTransport
    (forward dual : Fin 229 → Fin 229)
    (composition : ∀ y, forward (dual y) = 3 * y)
    (linearity : ∀ n x, forward (n * x) = n * forward x)
    (y : Fin 229) :
    forward (153 * dual y) = y := by
  rw [linearity, composition]
  exact inverseThreeAction y

#print axioms DualProjected229.inverseThree
#print axioms DualProjected229.inverseThreeAction
#print axioms DualProjected229.projectedTransport

end DualProjected229
