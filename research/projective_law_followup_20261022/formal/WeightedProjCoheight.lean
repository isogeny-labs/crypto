import Mathlib.AlgebraicGeometry.Properties
import Mathlib.RingTheory.DiscreteValuationRing.TFAE

/-!
# Coheight of proved DVR points on the actual weighted projective curve

This generic bridge connects a proved stalk-level DVR to scheme-theoretic
coheight. Applying it to an irreducible quadratic support still requires
constructing that point on the weighted-Proj curve and proving its stalk DVR.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjCoheight

open AlgebraicGeometry Order

theorem coheight_one_of_dvr {X : Scheme} (z : X)
    [IsDomain (X.presheaf.stalk z)]
    [IsDiscreteValuationRing (X.presheaf.stalk z)] :
    coheight z = 1 := by
  have hdim : ringKrullDim (X.presheaf.stalk z) = 1 :=
    IsDiscreteValuationRing.ringKrullDim_eq_one _
  rw [ringKrullDim_stalk_eq_coheight] at hdim
  exact_mod_cast hdim

#print axioms coheight_one_of_dvr

end WeightedProjCoheight
