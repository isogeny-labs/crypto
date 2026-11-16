import AuxiliaryOrbits

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
variable {K : Type*} [Field K] [DecidableEq K]

def chosenFunctionUnit (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) : W.FunctionFieldˣ :=
  Units.mk0 (millerFunction W P n) (miller_function_nonzero W P n)

theorem chosen_unit_divisor (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ)
    (hn : n • P=0) :
    functionDivisor W (chosenFunctionUnit W P n) (chosenFunctionUnit W P n).ne_zero=
      torsionDivisor W P n := chosen_torsion_divisor W P n hn

/-- Diagonal scalar one is derived from the actual two- and three-point products. -/
theorem diagonal_scalar_one [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P : W.Point) (f : W.FunctionFieldˣ)
    (ht : (3 : ℕ) • P=0) (c : K) (hc : c≠0)
    (he : auxiliaryQuotient W hΔ P P f f=constantHom W c) : c=1 := by
  classical
  obtain ⟨S,hS⟩ := exists_translated_admissible W hΔ P P f f {0,P,-P}
  have h0 : AuxiliaryAdmissible W P P f f S := by
    simpa only [add_zero] using hS 0 (by simp)
  have h1 : AuxiliaryAdmissible W P P f f (S+P) := hS P (by simp)
  have h2 : AuxiliaryAdmissible W P P f f (S-P) := by
    simpa only [sub_eq_add_neg] using hS (-P) (by simp)
  have hv (R : W.Point) (hR : AuxiliaryAdmissible W P P f f R) :
      auxiliaryValue W P P f f R hR=Units.mk0 c hc :=
    auxiliary_value_of_scalar W hΔ P P f f R hR c hc he
  have hsquare := diagonal_negative_product W P S f h0
  rw [hv (-S) (diagonal_negative_admissible W P S f h0),hv S h0] at hsquare
  have hcube := diagonal_three_product W P S f ht h0 h1 h2
  rw [hv S h0,hv (S+P) h1,hv (S-P) h2,hsquare,one_mul] at hcube
  exact congrArg (fun z : Kˣ => (z : K)) hcube

/-- Every actual geometric three-torsion function has alternating auxiliary quotient. -/
theorem diagonal_auxiliary_one [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P : W.Point) (f : W.FunctionFieldˣ)
    (ht : (3 : ℕ) • P=0)
    (hDf : functionDivisor W f f.ne_zero=torsionDivisor W P 3) :
    auxiliaryQuotient W hΔ P P f f=1 := by
  obtain ⟨c,hc,he⟩ := auxiliary_scalar W hΔ P P 3 f.ne_zero f.ne_zero hDf hDf
  have hc1 := diagonal_scalar_one W hΔ P f ht c hc he
  simpa only [hc1,map_one] using he

theorem chosen_diagonal_one [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P : W.Point) (ht : (3 : ℕ) • P=0) :
    auxiliaryQuotient W hΔ P P (chosenFunctionUnit W P 3) (chosenFunctionUnit W P 3)=1 :=
  diagonal_auxiliary_one W hΔ P (chosenFunctionUnit W P 3) ht (chosen_unit_divisor W P 3 ht)

#print axioms diagonal_scalar_one
#print axioms diagonal_auxiliary_one
#print axioms chosen_diagonal_one
end
end WeilAuxiliaryRatioAudit
