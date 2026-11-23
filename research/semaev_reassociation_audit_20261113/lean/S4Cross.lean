import LiteralPolynomial

namespace SemaevReassociationAudit
open Polynomial S4DegreeAudit
noncomputable section
variable {R : Type*} [CommRing R]

theorem actual_S4_middle_swap (W : WeierstrassCurve R) (x y z t : R) :
    s4Ring W.b₂ W.b₄ W.b₆ W.b₈ x y z t =
      s4Ring W.b₂ W.b₄ W.b₆ W.b₈ x z y t := by
  simp only [s4Ring, quadratic_resultant_ring, resForm, qaRing, qbRing, qcRing,
    WeierstrassCurve.b₂, WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]
  ring

#print axioms actual_S4_middle_swap

end
end SemaevReassociationAudit
