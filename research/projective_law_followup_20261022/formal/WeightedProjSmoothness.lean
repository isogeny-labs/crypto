import WeightedProjCover
import Mathlib.AlgebraicGeometry.Morphisms.Smooth

/-!
# Smoothness reduction to the two verified weighted-Proj charts

This establishes the scheme-level cover reduction. Smoothness of either
coordinate-ring presentation remains a separate hypothesis; the two
Jacobian ideal certificates alone do not construct smooth local presentations.
-/

set_option autoImplicit false
noncomputable section
namespace WeightedProjSmoothness
open AlgebraicGeometry CategoryTheory Common22WeightedProjQuotient
  WeightedRingGraded WeightedProjCover
variable {K : Type*} [Field K]

private def xOpen (a3 a2 a1 a0 : K) :
    (Proj (degreeSubmodule a3 a2 a1 a0)).Opens :=
  Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (gen a3 a2 a1 a0 0)

private def zOpen (a3 a2 a1 a0 : K) :
    (Proj (degreeSubmodule a3 a2 a1 a0)).Opens :=
  Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (gen a3 a2 a1 a0 2)

/-- The actual weighted Proj is smooth over a target whenever both of its
basic-open restrictions are smooth. This is a reduction, not an assertion
that the quotient chart rings have yet been proved smooth. -/
theorem smooth_of_two_basic_opens (a3 a2 a1 a0 : K) (Y : Scheme)
    (f : Proj (degreeSubmodule a3 a2 a1 a0) ⟶ Y)
    (hx : Smooth ((xOpen a3 a2 a1 a0).ι ≫ f))
    (hz : Smooth ((zOpen a3 a2 a1 a0).ι ≫ f)) : Smooth f := by
  let U : Bool → (Proj (degreeSubmodule a3 a2 a1 a0)).Opens :=
    fun b => if b then xOpen a3 a2 a1 a0 else zOpen a3 a2 a1 a0
  have hcover : iSup U = ⊤ := by
    simpa [U, xOpen, zOpen, iSup_bool_eq, sup_comm] using
      (WeightedProjCover.two_basic_opens_cover a3 a2 a1 a0)
  letI : IsZariskiLocalAtSource (@Smooth) :=
    HasRingHomProperty.instIsZariskiLocalAtSource (Q := RingHom.Smooth)
  apply IsZariskiLocalAtSource.of_iSup_eq_top U hcover
  intro b
  cases b
  · simpa [U] using hz
  · simpa [U] using hx

#print axioms WeightedProjSmoothness.smooth_of_two_basic_opens
end WeightedProjSmoothness
