module
public import GeometricMiller

@[expose] public section
namespace MillerBasechangeAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
local instance : CommRing F3 := (inferInstance : Field F3).toCommRing
local instance : DecidableEq (AlgebraicClosure F3) := Classical.decEq _

def counterfeitMiller (P : rationalCurve.Point) (n : ℕ) : rationalCurve.FunctionField :=
  rationalFunction * millerFunction rationalCurve P n

theorem counterfeit_nonzero (P : rationalCurve.Point) (n : ℕ) : counterfeitMiller P n ≠ 0 :=
  mul_ne_zero rational_function_nonzero (miller_function_nonzero rationalCurve P n)

/-- The old rational-point-only condition admits counterfeits at every point and every n. -/
theorem counterfeit_weak_miller (P : rationalCurve.Point) (n : ℕ) :
    IsMillerFunction rationalCurve P n (counterfeitMiller P n) := by
  have hr : HasPointDivisor rationalCurve rationalFunction 0 :=
    ⟨rational_function_nonzero, fun Q => rational_point_order_zero Q⟩
  refine ⟨?_, infinity_monic_mul rationalCurve rational_function_nonzero
    (miller_function_nonzero rationalCurve P n) rational_function_monic
    (miller_infinity_monic rationalCurve P n)⟩
  simpa only [counterfeitMiller, zero_add] using
    HasPointDivisor.mul rationalCurve hr (miller_has_point_divisor rationalCurve P n)

theorem counterfeit_not_chosen (P : rationalCurve.Point) (n : ℕ) :
    counterfeitMiller P n ≠ millerFunction rationalCurve P n := by
  intro he
  have hc := congrArg (fun f : rationalCurve.FunctionField => f / millerFunction rationalCurve P n) he
  simp only [counterfeitMiller, mul_div_assoc,
    div_self (miller_function_nonzero rationalCurve P n), mul_one] at hc
  exact rational_function_not_one hc

theorem counterfeit_not_geometric (P : rationalCurve.Point) (n : ℕ) :
    ¬ GeometricMiller rationalCurve P n (counterfeitMiller P n) := by
  intro hf
  exact counterfeit_not_chosen P n (geometric_miller_unique rationalCurve P n hf)

theorem counterfeit_geometric_divisor_wrong (P : rationalCurve.Point) (n : ℕ) :
    functionDivisor (extendedCurve (L := AlgebraicClosure F3) rationalCurve)
        (fieldBaseChange (L := AlgebraicClosure F3) rationalCurve (counterfeitMiller P n))
        (field_base_nonzero rationalCurve (counterfeit_nonzero P n)) ≠
      millerDivisor (extendedCurve (L := AlgebraicClosure F3) rationalCurve)
        (pointBaseChange (L := AlgebraicClosure F3) rationalCurve P) n := by
  intro hd
  apply counterfeit_not_geometric P n
  refine ⟨rational_delta, ⟨⟨field_base_nonzero rationalCurve (counterfeit_nonzero P n), ?_⟩, ?_⟩⟩
  · intro Q
    exact congrArg (fun D => D Q) hd
  · exact (infinity_monic_basechange (L := AlgebraicClosure F3) rationalCurve
      (counterfeitMiller P n)).mpr (counterfeit_weak_miller P n).2

theorem counterfeit_zero (P : rationalCurve.Point) : counterfeitMiller P 0 = rationalFunction := by
  simp only [counterfeitMiller, millerFunction, mul_one]

theorem counterfeit_one (P : rationalCurve.Point) : counterfeitMiller P 1 = rationalFunction := by
  simp only [counterfeitMiller, miller_function_one, mul_one]

/-- The actual mapped prior ratio has a nonzero actual geometric divisor, despite rational divisor0. -/
theorem mapped_ratio_divisor_nonzero :
    functionDivisor (extendedCurve (L := AlgebraicClosure F3) rationalCurve)
        (fieldBaseChange (L := AlgebraicClosure F3) rationalCurve rationalFunction)
        (field_base_nonzero rationalCurve rational_function_nonzero) ≠ 0 := by
  intro hd
  have he := normalized_divisor_unique (extendedCurve (L := AlgebraicClosure F3) rationalCurve)
    (extended_delta_nonzero rationalCurve rational_delta)
    (field_base_nonzero rationalCurve rational_function_nonzero) one_ne_zero
    (hd.trans (function_divisor_one _).symm)
    ((infinity_monic_basechange rationalCurve rationalFunction).mpr rational_function_monic)
    (infinity_monic_one _)
  apply rational_function_not_one
  apply field_base_injective (L := AlgebraicClosure F3) rationalCurve
  simpa only [map_one] using he

#print axioms counterfeit_weak_miller
#print axioms counterfeit_not_geometric
#print axioms counterfeit_geometric_divisor_wrong
#print axioms mapped_ratio_divisor_nonzero
end
end MillerBasechangeAudit
