module
public import LevelControls

@[expose] public section
namespace MillerLevelCompatibilityAudit
noncomputable section
open WeierstrassCurve.Affine
open BinaryMillerAudit MillerPrincipalIdealAudit MillerFinitePointAudit MillerRecursionAudit
open MillerBasechangeAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing TestField := (inferInstance : Field TestField).toCommRing
local instance : DecidableEq (AlgebraicClosure TestField) := Classical.decEq _

def levelGeometricFunction (P : TestCurve.Point) (n : ℕ) : GeometricCurve.FunctionField :=
  fieldBaseChange (L := AlgebraicClosure TestField) TestCurve (millerFunction TestCurve P n)

theorem level_geometric_nonzero (P : TestCurve.Point) (n : ℕ) : levelGeometricFunction P n ≠ 0 :=
  field_base_nonzero TestCurve (miller_function_nonzero TestCurve P n)

def geometricDifference (A B : TestCurve.Point) : GeometricCurve.Point →₀ ℤ :=
  Finsupp.single (geometricPoint A) 1 - Finsupp.single (geometricPoint B) 1

theorem geometric_difference_image (A B : TestCurve.Point) :
    geometricDifference A B = Finsupp.mapDomain geometricPoint (Finsupp.single A (1 : ℤ) - Finsupp.single B 1) := by
  simp only [geometricDifference, Finsupp.mapDomain_sub, Finsupp.mapDomain_single]

theorem geometric_difference_degree (A B : TestCurve.Point) :
    divisorDegree GeometricCurve (geometricDifference A B) = 0 := by
  simp only [geometricDifference, map_sub, degree_single, sub_self]

theorem geometric_order_away (P : TestCurve.Point) (n : ℕ) (A : TestCurve.Point)
    (hP : A ≠ P) (hn : A ≠ n • P) (h0 : A ≠ 0) :
    pointOrder GeometricCurve (levelGeometricFunction P n) (geometricPoint A) = 0 := by
  change functionDivisor GeometricCurve (levelGeometricFunction P n) (level_geometric_nonzero P n)
    (geometricPoint A) = 0
  have hd : functionDivisor GeometricCurve (levelGeometricFunction P n) (level_geometric_nonzero P n) =
      millerDivisor GeometricCurve (geometricPoint P) n :=
    constructed_geometric_divisor TestCurve control_delta P n
  rw [hd]
  change millerDivisor GeometricCurve (geometricPoint P) n (geometricPoint A) = 0
  have hnp : geometricPoint A ≠ n • geometricPoint P := by
    rw [← map_nsmul]
    exact geometric_ne A (n • P) hn
  rw [millerDivisor]
  simp only [Finsupp.sub_apply, Finsupp.smul_apply,
    Finsupp.single_eq_of_ne (geometric_ne A P hP), Finsupp.single_eq_of_ne hnp,
    Finsupp.single_eq_of_ne (geometric_ne_zero A h0), smul_zero, sub_zero]

theorem geometric_final_domain_away (P : TestCurve.Point) (n : ℕ) (A B : TestCurve.Point)
    (hAP : A ≠ P) (hBP : B ≠ P) (hAn : A ≠ n • P) (hBn : B ≠ n • P)
    (hA0 : A ≠ 0) (hB0 : B ≠ 0) :
    Disjoint (geometricDifference A B).support
      (functionDivisor GeometricCurve (levelGeometricFunction P n) (level_geometric_nonzero P n)).support := by
  apply Finset.disjoint_left.mpr
  intro R hR hF
  have hz : functionDivisor GeometricCurve (levelGeometricFunction P n) (level_geometric_nonzero P n) R = 0 := by
    rcases support_pair hR with h | h
    · rw [h]
      exact geometric_order_away P n A hAP hAn hA0
    · rw [h]
      exact geometric_order_away P n B hBP hBn hB0
  exact (Finsupp.mem_support_iff.mp hF) hz

theorem positive_geometric_domain_two : Disjoint (geometricDifference basePoint safePoint).support
    (functionDivisor GeometricCurve (levelGeometricFunction twoTorsionPoint 2) (level_geometric_nonzero twoTorsionPoint 2)).support := by
  apply geometric_final_domain_away
  all_goals decide +kernel

theorem positive_geometric_domain_four : Disjoint (geometricDifference basePoint safePoint).support
    (functionDivisor GeometricCurve (levelGeometricFunction twoTorsionPoint 4) (level_geometric_nonzero twoTorsionPoint 4)).support := by
  apply geometric_final_domain_away
  all_goals decide +kernel

theorem negative_geometric_domain_two : Disjoint (geometricDifference point23 twoTorsionPoint).support
    (functionDivisor GeometricCurve (levelGeometricFunction basePoint 2) (level_geometric_nonzero basePoint 2)).support := by
  apply geometric_final_domain_away
  all_goals decide +kernel

theorem negative_geometric_domain_four : Disjoint (geometricDifference point23 twoTorsionPoint).support
    (functionDivisor GeometricCurve (levelGeometricFunction basePoint 4) (level_geometric_nonzero basePoint 4)).support := by
  apply geometric_final_domain_away
  all_goals decide +kernel

#print axioms geometric_final_domain_away
#print axioms negative_geometric_domain_two
end
end MillerLevelCompatibilityAudit
