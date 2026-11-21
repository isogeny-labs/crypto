import Mathlib.Tactic

namespace XedniFreshTarget

/-- Once the reduced relation `R = λ G` is known, the two-generator
candidate and its one-generator finite control are equal at every label.
The elliptic-curve-to-scalar-coordinate bridge is a separate obligation. -/
theorem scalar_control_equivalence (q : ℕ) (a b lam G R : ZMod q)
    (hR : R = lam * G) :
    a * G + b * R = (a + lam * b) * G := by
  rw [hR]
  ring

#print axioms scalar_control_equivalence

end XedniFreshTarget
