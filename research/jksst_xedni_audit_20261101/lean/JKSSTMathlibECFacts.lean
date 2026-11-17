import Mathlib.NumberTheory.Height.EllipticCurve
import Mathlib.NumberTheory.Height.NumberField
import Mathlib.AlgebraicGeometry.EllipticCurve.Reduction

/-!
# Elliptic-curve facts currently available in Mathlib

This file records the strongest directly reusable arithmetic infrastructure
found during the JKSST audit.  Mathlib supplies the naïve height, its
approximate parallelogram law, Northcott finiteness, finiteness of rational
torsion under Northcott hypotheses, and local minimal Weierstrass models over
discrete valuation rings.

Mathlib does not itself package the Néron--Tate canonical height; this project
constructs it from the first two ingredients in `JKSSTCanonicalHeight`.
Mathlib also does not expose the Mordell--Weil finite-generation/lattice
realization needed by the packing theorem or a global minimal-discriminant
comparison for the xedni lift family.
-/

namespace JKSSTMathlibECFacts

open Height

namespace RationalHeight

variable {F : Type*} [Field F] [AdmissibleAbsValues F]
  {W : WeierstrassCurve.Affine F}

/-- The actual Mathlib naïve height has finite bounded sublevel sets whenever
the base field has the corresponding Northcott property. -/
theorem finite_naive_height_ball
    [Northcott (logHeight₁ (K := F))] (B : ℝ) :
    {P : W.Point | P.naiveHeight ≤ B}.Finite :=
  WeierstrassCurve.Affine.finite_naiveHeight_le W B

/-- Mathlib's proved approximate parallelogram law for the naïve height. -/
theorem approximate_parallelogram
    [DecidableEq F] [W.IsElliptic] :
    ∃ C, ∀ P Q : W.Point,
      |(P + Q).naiveHeight + (P - Q).naiveHeight -
        2 * (P.naiveHeight + Q.naiveHeight)| ≤ C :=
  WeierstrassCurve.Affine.approx_parallelogram_law W

/-- Under Northcott and ellipticity hypotheses, the rational torsion subgroup
is finite. -/
theorem finite_rational_torsion
    [Northcott (logHeight₁ (K := F))] [DecidableEq F] [W.IsElliptic] :
    Finite (AddCommGroup.torsion W.Point) :=
  WeierstrassCurve.Affine.finite_torsion

end RationalHeight

namespace OverRat

variable {W : WeierstrassCurve.Affine ℚ}

/-- Specialization of Northcott finiteness to an actual rational
Weierstrass curve. -/
theorem finite_naive_height_ball (B : ℝ) :
    {P : W.Point | P.naiveHeight ≤ B}.Finite :=
  RationalHeight.finite_naive_height_ball B

/-- The torsion subgroup of an elliptic curve over `ℚ` is finite in the
actual Mathlib point group. -/
theorem finite_torsion [W.IsElliptic] :
    Finite (AddCommGroup.torsion W.Point) :=
  RationalHeight.finite_rational_torsion

end OverRat

namespace LocalMinimalModel

variable (R : Type*) [CommRing R] [IsDomain R] [IsDiscreteValuationRing R]
  {K : Type*} [Field K] [Algebra R K] [IsFractionRing R K]

/-- Every Weierstrass curve over the fraction field has an isomorphic model
minimal for the chosen discrete valuation.  This is local minimality, not a
global minimal discriminant over `ℤ`. -/
theorem exists_local_minimal_model (W : WeierstrassCurve K) :
    ∃ C : WeierstrassCurve.VariableChange K,
      WeierstrassCurve.IsMinimal R (C • W) :=
  WeierstrassCurve.exists_isMinimal R W

end LocalMinimalModel

end JKSSTMathlibECFacts
