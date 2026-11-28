module
public import CancellationDivisors

@[expose] public section
namespace BinaryMillerAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerPrincipalIdealAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit MillerBasechangeAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing TestField := (inferInstance : Field TestField).toCommRing
local instance : DecidableEq (AlgebraicClosure TestField) := Classical.decEq _

abbrev GeometricCurve := extendedCurve (L := AlgebraicClosure TestField) TestCurve

def geometricPoint : TestCurve.Point →+ GeometricCurve.Point :=
  pointBaseChange (L := AlgebraicClosure TestField) TestCurve

def geometricFunction : GeometricCurve.FunctionField :=
  fieldBaseChange (L := AlgebraicClosure TestField) TestCurve (millerFunction TestCurve basePoint 3)

theorem geometric_function_nonzero : geometricFunction ≠ 0 :=
  field_base_nonzero (L := AlgebraicClosure TestField) TestCurve
    (miller_function_nonzero TestCurve basePoint 3)

def geometricBadDivisor : GeometricCurve.Point →₀ ℤ :=
  Finsupp.single (geometricPoint polePoint) 1 - Finsupp.single (geometricPoint twoTorsionPoint) 1

def geometricSafeDivisor : GeometricCurve.Point →₀ ℤ :=
  Finsupp.single (geometricPoint safePoint) 1 - Finsupp.single (geometricPoint twoTorsionPoint) 1

theorem geometric_bad_point_image :
    geometricBadDivisor = Finsupp.mapDomain geometricPoint badDivisor := by
  simp only [badDivisor, geometricBadDivisor, Finsupp.mapDomain_sub, Finsupp.mapDomain_single]

theorem geometric_safe_point_image :
    geometricSafeDivisor = Finsupp.mapDomain geometricPoint safeDivisor := by
  simp only [safeDivisor, geometricSafeDivisor, Finsupp.mapDomain_sub, Finsupp.mapDomain_single]

theorem geometric_delta : GeometricCurve.Δ ≠ 0 :=
  extended_delta_nonzero TestCurve control_delta

theorem geometric_injective : Function.Injective geometricPoint :=
  point_base_injective (L := AlgebraicClosure TestField) TestCurve

theorem geometric_ne (P Q : TestCurve.Point) (h : P ≠ Q) : geometricPoint P ≠ geometricPoint Q :=
  fun he => h (geometric_injective he)

theorem geometric_ne_zero (P : TestCurve.Point) (h : P ≠ 0) : geometricPoint P ≠ 0 := by
  intro he
  exact h (geometric_injective (he.trans (map_zero geometricPoint).symm))

theorem geometric_base_three : (3 : ℕ) • geometricPoint basePoint = 0 := by
  rw [← map_nsmul, base_three, map_zero]

theorem geometric_bad_degree_zero : divisorDegree GeometricCurve geometricBadDivisor = 0 := by
  simp only [geometricBadDivisor, map_sub, degree_single, sub_self]

theorem geometric_safe_degree_zero : divisorDegree GeometricCurve geometricSafeDivisor = 0 := by
  simp only [geometricSafeDivisor, map_sub, degree_single, sub_self]

/-- The actual image function's full geometric divisor, without assuming general divisor base change. -/
theorem geometric_final_divisor :
    functionDivisor GeometricCurve geometricFunction geometric_function_nonzero =
      (3 : ℤ) • Finsupp.single (geometricPoint basePoint) 1 - (3 : ℤ) • Finsupp.single 0 1 := by
  change functionDivisor (extendedCurve (L := AlgebraicClosure TestField) TestCurve)
    (fieldBaseChange (L := AlgebraicClosure TestField) TestCurve (millerFunction TestCurve basePoint 3))
    (field_base_nonzero TestCurve (miller_function_nonzero TestCurve basePoint 3)) = _
  rw [constructed_geometric_divisor TestCurve control_delta]
  change millerDivisor GeometricCurve (geometricPoint basePoint) 3 = _
  rw [millerDivisor, geometric_base_three, sub_smul, one_smul]
  abel

theorem geometric_final_order_elsewhere (R : GeometricCurve.Point)
    (hP : R ≠ geometricPoint basePoint) (h0 : R ≠ 0) : pointOrder GeometricCurve geometricFunction R = 0 := by
  change functionDivisor GeometricCurve geometricFunction geometric_function_nonzero R = 0
  rw [geometric_final_divisor]
  simp only [Finsupp.sub_apply, Finsupp.smul_apply, Finsupp.single_eq_of_ne hP,
    Finsupp.single_eq_of_ne h0, smul_zero, sub_self]

theorem geometric_final_disjoint_bad : Disjoint geometricBadDivisor.support
    (functionDivisor GeometricCurve geometricFunction geometric_function_nonzero).support := by
  apply Finset.disjoint_left.mpr
  intro R hR hF
  have hc : R = geometricPoint polePoint ∨ R = geometricPoint twoTorsionPoint := support_pair hR
  have hz : pointOrder GeometricCurve geometricFunction R = 0 := by
    rcases hc with rfl | rfl
    · exact geometric_final_order_elsewhere _
        (geometric_ne polePoint basePoint pole_ne_base) (geometric_ne_zero polePoint pole_ne_zero)
    · exact geometric_final_order_elsewhere _
        (geometric_ne twoTorsionPoint basePoint two_ne_base) (geometric_ne_zero twoTorsionPoint two_ne_zero)
  have hn : pointOrder GeometricCurve geometricFunction R ≠ 0 := by
    simpa only [Finsupp.mem_support_iff, function_divisor_apply] using hF
  exact hn hz

theorem geometric_final_disjoint_safe : Disjoint geometricSafeDivisor.support
    (functionDivisor GeometricCurve geometricFunction geometric_function_nonzero).support := by
  apply Finset.disjoint_left.mpr
  intro R hR hF
  have hc : R = geometricPoint safePoint ∨ R = geometricPoint twoTorsionPoint := support_pair hR
  have hz : pointOrder GeometricCurve geometricFunction R = 0 := by
    rcases hc with rfl | rfl
    · exact geometric_final_order_elsewhere _
        (geometric_ne safePoint basePoint safe_ne_base) (geometric_ne_zero safePoint safe_ne_zero)
    · exact geometric_final_order_elsewhere _
        (geometric_ne twoTorsionPoint basePoint two_ne_base) (geometric_ne_zero twoTorsionPoint two_ne_zero)
  have hn : pointOrder GeometricCurve geometricFunction R ≠ 0 := by
    simpa only [Finsupp.mem_support_iff, function_divisor_apply] using hF
  exact hn hz

theorem geometric_double_base : (2 : ℕ) • geometricPoint basePoint = geometricPoint polePoint := by
  rw [← map_nsmul, double_base]

/-- The actual embedded intermediate function has a geometric pole at the actual mapped Q. -/
theorem geometric_intermediate_order_pole :
    pointOrder GeometricCurve
      (fieldBaseChange (L := AlgebraicClosure TestField) TestCurve (millerFunction TestCurve basePoint 2))
      (geometricPoint polePoint) = -1 := by
  change functionDivisor GeometricCurve
    (fieldBaseChange (L := AlgebraicClosure TestField) TestCurve (millerFunction TestCurve basePoint 2))
    (field_base_nonzero TestCurve (miller_function_nonzero TestCurve basePoint 2)) (geometricPoint polePoint) = -1
  rw [constructed_geometric_divisor TestCurve control_delta]
  change millerDivisor GeometricCurve (geometricPoint basePoint) 2 (geometricPoint polePoint) = -1
  rw [millerDivisor, geometric_double_base]
  simp only [Finsupp.sub_apply, Finsupp.smul_apply,
    Finsupp.single_eq_of_ne (geometric_ne polePoint basePoint pole_ne_base),
    Finsupp.single_eq_of_ne (geometric_ne_zero polePoint pole_ne_zero),
    Finsupp.single_eq_same, smul_zero, zero_sub, sub_zero]

#print axioms geometric_final_divisor
#print axioms geometric_final_disjoint_bad
#print axioms geometric_final_disjoint_safe
end
end BinaryMillerAudit
