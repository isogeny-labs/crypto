module
public import LevelValues

@[expose] public section
namespace MillerLevelCompatibilityAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open BinaryMillerAudit MillerPrincipalIdealAudit MillerFinitePointAudit MillerNormValuationAudit
open MillerRecursionAudit MillerBasechangeAudit FunctionDivisorFinitenessAudit
open GeometricPrincipalDivisorAudit FinitePointEvaluationAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing TestField := (inferInstance : Field TestField).toCommRing

theorem point23_nonsingular : TestCurve.Nonsingular (2 : TestField) 3 := by
  apply (equation_iff_nonsingular_of_Δ_ne_zero control_delta).mp
  rw [equation_iff]
  decide +kernel

def point23 : TestCurve.Point := .some 2 3 point23_nonsingular

def positiveDivisor : TestCurve.Point →₀ ℤ := Finsupp.single basePoint 1 - Finsupp.single safePoint 1
def negativeDivisor : TestCurve.Point →₀ ℤ := Finsupp.single point23 1 - Finsupp.single twoTorsionPoint 1

theorem positive_degree_zero : divisorDegree TestCurve positiveDivisor = 0 := by
  simp only [positiveDivisor, map_sub, degree_single, sub_self]

theorem negative_degree_zero : divisorDegree TestCurve negativeDivisor = 0 := by
  simp only [negativeDivisor, map_sub, degree_single, sub_self]

theorem two_point_double : (2 : ℕ) • twoTorsionPoint = 0 := by decide +kernel

theorem two_point_order : addOrderOf twoTorsionPoint = 2 := by
  let : Fact (Nat.Prime 2) := ⟨by decide⟩
  exact addOrderOf_eq_prime two_point_double two_ne_zero

theorem base_not_two_torsion : (2 : ℕ) • basePoint ≠ 0 := by decide +kernel

theorem order_away (P : TestCurve.Point) (n : ℕ) (A : TestCurve.Point)
    (hP : A ≠ P) (hn : A ≠ n • P) (h0 : A ≠ 0) :
    pointOrder TestCurve (millerFunction TestCurve P n) A = 0 := by
  change functionDivisor TestCurve (millerFunction TestCurve P n)
    (miller_function_nonzero TestCurve P n) A = 0
  rw [function_divisor_miller, millerDivisor]
  simp only [Finsupp.sub_apply, Finsupp.smul_apply, Finsupp.single_eq_of_ne hP,
    Finsupp.single_eq_of_ne hn, Finsupp.single_eq_of_ne h0, smul_zero, sub_zero]

theorem final_domain_away (P : TestCurve.Point) (n : ℕ) (A B : TestCurve.Point)
    (hAP : A ≠ P) (hBP : B ≠ P) (hAn : A ≠ n • P) (hBn : B ≠ n • P)
    (hA0 : A ≠ 0) (hB0 : B ≠ 0) :
    Disjoint (Finsupp.single A (1 : ℤ) - Finsupp.single B 1).support
      (functionDivisor TestCurve (millerUnit TestCurve P n) (millerUnit TestCurve P n).ne_zero).support := by
  apply Finset.disjoint_left.mpr
  intro R hR hF
  have hz : functionDivisor TestCurve (millerUnit TestCurve P n) (millerUnit TestCurve P n).ne_zero R = 0 := by
    rcases support_pair hR with h | h
    · rw [h]
      exact order_away P n A hAP hAn hA0
    · rw [h]
      exact order_away P n B hBP hBn hB0
  exact (Finsupp.mem_support_iff.mp hF) hz

theorem positive_domain_two : Disjoint positiveDivisor.support
    (functionDivisor TestCurve (millerUnit TestCurve twoTorsionPoint 2) (millerUnit TestCurve twoTorsionPoint 2).ne_zero).support := by
  apply final_domain_away
  all_goals decide +kernel

theorem positive_domain_four : Disjoint positiveDivisor.support
    (functionDivisor TestCurve (millerUnit TestCurve twoTorsionPoint 4) (millerUnit TestCurve twoTorsionPoint 4).ne_zero).support := by
  apply final_domain_away
  all_goals decide +kernel

theorem negative_domain_two : Disjoint negativeDivisor.support
    (functionDivisor TestCurve (millerUnit TestCurve basePoint 2) (millerUnit TestCurve basePoint 2).ne_zero).support := by
  apply final_domain_away
  all_goals decide +kernel

theorem negative_domain_four : Disjoint negativeDivisor.support
    (functionDivisor TestCurve (millerUnit TestCurve basePoint 4) (millerUnit TestCurve basePoint 4).ne_zero).support := by
  apply final_domain_away
  all_goals decide +kernel

theorem positive_function_two : millerFunction TestCurve twoTorsionPoint 2 = verticalFunction TestCurve 4 := by
  change ((1 : TestCurve.FunctionField) * 1) * stepFunction TestCurve twoTorsionPoint twoTorsionPoint = _
  change (1 * 1) * (if (4 : TestField) = 4 ∧ (0 : TestField) = TestCurve.negY 4 0 then
    verticalFunction TestCurve 4 else _) = _
  have h : (4 : TestField) = 4 ∧ (0 : TestField) = TestCurve.negY 4 0 := by decide +kernel
  rw [ite_eq_left h, one_mul, one_mul]

theorem positive_function_four : millerFunction TestCurve twoTorsionPoint 4 = (verticalFunction TestCurve 4) ^ 2 := by
  have he := miller_multiple TestCurve control_delta twoTorsionPoint 2 two_point_double 2
  simpa only [positive_function_two] using he

theorem negative_function_four : millerFunction TestCurve basePoint 4 =
    MillerNormValuationAudit.lineFunction TestCurve 0 1 0 := by
  rw [millerFunction, base_three]
  change millerFunction TestCurve basePoint 3 * 1 = _
  rw [mul_one, third_miller_line]

theorem negative_four_order : pointOrder TestCurve (millerFunction TestCurve basePoint 4) polePoint = 0 := by
  apply order_away
  all_goals decide +kernel

theorem negative_square_order : pointOrder TestCurve (millerFunction TestCurve basePoint 2 ^ 2) polePoint = -2 := by
  rw [pow_two, point_order_mul TestCurve (miller_function_nonzero TestCurve basePoint 2)
    (miller_function_nonzero TestCurve basePoint 2), intermediate_order_pole]
  norm_num

theorem negative_function_power_failure :
    millerFunction TestCurve basePoint 4 ≠ millerFunction TestCurve basePoint 2 ^ 2 := by
  intro he
  have ho := congrArg (fun f => pointOrder TestCurve f polePoint) he
  rw [negative_four_order, negative_square_order] at ho
  norm_num at ho

#print axioms positive_domain_two
#print axioms negative_domain_four
#print axioms negative_function_power_failure
end
end MillerLevelCompatibilityAudit
