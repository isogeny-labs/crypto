module
public import EvaluationCertificate

@[expose] public section
namespace BinaryMillerAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerPrincipalIdealAudit MillerFinitePointAudit MillerNormValuationAudit
open MillerRecursionAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev TestField := MillerPrincipalIdealAudit.F
abbrev TestCurve := MillerPrincipalIdealAudit.controlCurve
local instance : CommRing TestField := (inferInstance : Field TestField).toCommRing

def basePoint : TestCurve.Point := .some 0 1 control_positive_nonsingular

theorem pole_nonsingular : TestCurve.Nonsingular (0 : TestField) 4 := by
  apply (equation_iff_nonsingular_of_Δ_ne_zero control_delta).mp
  rw [equation_iff]
  decide +kernel

def polePoint : TestCurve.Point := .some 0 4 pole_nonsingular

def twoTorsionPoint : TestCurve.Point := .some 4 0 control_two_nonsingular

def safePoint : TestCurve.Point := .some 2 2 control_negative_nonsingular

theorem base_three : (3 : ℕ) • basePoint = 0 := control_positive_three

theorem base_order_three : addOrderOf basePoint = 3 := by
  let : Fact (Nat.Prime 3) := ⟨by decide⟩
  exact addOrderOf_eq_prime base_three (Point.some_ne_zero control_positive_nonsingular)

theorem pole_is_negative : polePoint = -basePoint := by decide +kernel

theorem double_base : (2 : ℕ) • basePoint = polePoint := by decide +kernel

theorem pole_ne_base : polePoint ≠ basePoint := by decide +kernel

theorem two_ne_base : twoTorsionPoint ≠ basePoint := by decide +kernel

theorem safe_ne_base : safePoint ≠ basePoint := by decide +kernel

theorem pole_ne_zero : polePoint ≠ 0 := Point.some_ne_zero pole_nonsingular

theorem two_ne_zero : twoTorsionPoint ≠ 0 := Point.some_ne_zero control_two_nonsingular

theorem safe_ne_zero : safePoint ≠ 0 := Point.some_ne_zero control_negative_nonsingular

theorem pole_ne_two : polePoint ≠ twoTorsionPoint := by decide +kernel

theorem safe_ne_two : safePoint ≠ twoTorsionPoint := by decide +kernel

def badDivisor : TestCurve.Point →₀ ℤ :=
  Finsupp.single polePoint 1 - Finsupp.single twoTorsionPoint 1

def safeDivisor : TestCurve.Point →₀ ℤ :=
  Finsupp.single safePoint 1 - Finsupp.single twoTorsionPoint 1

theorem bad_degree_zero : divisorDegree TestCurve badDivisor = 0 := by
  simp only [badDivisor, map_sub, degree_single, sub_self]

theorem safe_degree_zero : divisorDegree TestCurve safeDivisor = 0 := by
  simp only [safeDivisor, map_sub, degree_single, sub_self]

/-- The actual chosen final function has its source three-torsion principal divisor. -/
theorem final_miller_divisor :
    functionDivisor TestCurve (millerFunction TestCurve basePoint 3)
      (miller_function_nonzero TestCurve basePoint 3) =
        (3 : ℤ) • Finsupp.single basePoint 1 - (3 : ℤ) • Finsupp.single 0 1 := by
  rw [function_divisor_miller, millerDivisor, base_three, sub_smul, one_smul]
  abel

theorem final_order_elsewhere (R : TestCurve.Point) (hP : R ≠ basePoint) (h0 : R ≠ 0) :
    pointOrder TestCurve (millerFunction TestCurve basePoint 3) R = 0 := by
  change functionDivisor TestCurve (millerFunction TestCurve basePoint 3)
    (miller_function_nonzero TestCurve basePoint 3) R = 0
  rw [final_miller_divisor]
  simp only [Finsupp.sub_apply, Finsupp.smul_apply, Finsupp.single_eq_of_ne hP,
    Finsupp.single_eq_of_ne h0, smul_zero, sub_self]

theorem support_pair {α : Type*} {A B R : α}
    (h : R ∈ (Finsupp.single A 1 - Finsupp.single B 1 : α →₀ ℤ).support) :
    R = A ∨ R = B := by
  classical
  by_cases ha : R = A
  · exact Or.inl ha
  by_cases hb : R = B
  · exact Or.inr hb
  exfalso
  apply (Finsupp.mem_support_iff.mp h)
  rw [Finsupp.sub_apply, Finsupp.single_eq_of_ne ha, Finsupp.single_eq_of_ne hb, sub_self]

theorem final_disjoint_bad :
    Disjoint badDivisor.support (functionDivisor TestCurve (millerFunction TestCurve basePoint 3)
      (miller_function_nonzero TestCurve basePoint 3)).support := by
  apply Finset.disjoint_left.mpr
  intro R hR hF
  have hc : R = polePoint ∨ R = twoTorsionPoint := support_pair hR
  have hz : pointOrder TestCurve (millerFunction TestCurve basePoint 3) R = 0 := by
    rcases hc with rfl | rfl
    · exact final_order_elsewhere polePoint pole_ne_base pole_ne_zero
    · exact final_order_elsewhere twoTorsionPoint two_ne_base two_ne_zero
  have hn : pointOrder TestCurve (millerFunction TestCurve basePoint 3) R ≠ 0 := by
    simpa only [Finsupp.mem_support_iff, function_divisor_apply] using hF
  exact hn hz

theorem final_disjoint_safe :
    Disjoint safeDivisor.support (functionDivisor TestCurve (millerFunction TestCurve basePoint 3)
      (miller_function_nonzero TestCurve basePoint 3)).support := by
  apply Finset.disjoint_left.mpr
  intro R hR hF
  have hc : R = safePoint ∨ R = twoTorsionPoint := support_pair hR
  have hz : pointOrder TestCurve (millerFunction TestCurve basePoint 3) R = 0 := by
    rcases hc with rfl | rfl
    · exact final_order_elsewhere safePoint safe_ne_base safe_ne_zero
    · exact final_order_elsewhere twoTorsionPoint two_ne_base two_ne_zero
  have hn : pointOrder TestCurve (millerFunction TestCurve basePoint 3) R ≠ 0 := by
    simpa only [Finsupp.mem_support_iff, function_divisor_apply] using hF
  exact hn hz

/-- The intermediate actual f2 has a pole at Q even though final f3 is a unit there. -/
theorem intermediate_order_pole :
    pointOrder TestCurve (millerFunction TestCurve basePoint 2) polePoint = -1 := by
  change functionDivisor TestCurve (millerFunction TestCurve basePoint 2)
    (miller_function_nonzero TestCurve basePoint 2) polePoint = -1
  rw [function_divisor_miller, millerDivisor, double_base]
  simp only [Finsupp.sub_apply, Finsupp.smul_apply, Finsupp.single_eq_of_ne pole_ne_base,
    Finsupp.single_eq_of_ne pole_ne_zero, Finsupp.single_eq_same, smul_zero, zero_sub, sub_zero]

#print axioms final_miller_divisor
#print axioms final_disjoint_bad
#print axioms final_disjoint_safe
#print axioms intermediate_order_pole
end
end BinaryMillerAudit
