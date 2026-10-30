import WeightedOverlapEmbedding
import WeightedProjCover
import Common22TwoChartGlue

/-! Scheme comparison between the two separately glued charts and weighted `Proj`.
The global isomorphism remains open until the chart overlap maps and gluing
universal property are related without extra hypotheses. -/

set_option autoImplicit false
set_option maxHeartbeats 400000
noncomputable section

namespace WeightedProjGlobalGlue

open AlgebraicGeometry CategoryTheory WeightedRingGraded WeightedOverlapFull
  WeightedOverlapSpecialized Common22WeightedProjQuotient

variable {K : Type} [Field K]

def chartIntoProj (a3 a2 a1 a0 : K) :
    (i : Bool) → Common22TwoChartGlue.chart a3 a2 a1 a0 i ⟶
      Proj (degreeSubmodule a3 a2 a1 a0)
  | false => (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv ≫
      (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0)).ι
  | true => (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv ≫
      (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0)).ι

#print axioms chartIntoProj

end WeightedProjGlobalGlue
