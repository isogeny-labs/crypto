module
public import BasechangeControls

@[expose] public section
namespace MillerBasechangeAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit MillerRecursionAudit FunctionDivisorFinitenessAudit
local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
local instance : CommRing F3 := (inferInstance : Field F3).toCommRing
local instance : DecidableEq (AlgebraicClosure F3) := Classical.decEq _

/-- Positional spelling of the same actual model; avoids named-argument tokens in the bound header. -/
abbrev curveOver {K : Type*} [Field K] (W : WeierstrassCurve.Affine K)
    (L : Type*) [Field L] [Algebra K L] : WeierstrassCurve.Affine L := extendedCurve (L := L) W

abbrev functionEmbedding {K : Type*} [Field K] (W : WeierstrassCurve.Affine K)
    (L : Type*) [Field L] [Algebra K L] : W.FunctionField →+* (curveOver W L).FunctionField :=
  fieldBaseChange (L := L) W

abbrev pointEmbedding {K : Type*} [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (L : Type*) [Field L] [Algebra K L] [DecidableEq L] : W.Point →+ (curveOver W L).Point :=
  pointBaseChange (L := L) W

theorem miller_basechange_certificate :
    (∀ (K L : Type) [Field K] [Field L] [Algebra K L] (W : WeierstrassCurve.Affine K),
      Function.Injective (functionEmbedding W L) ∧
      (∀ a : W.CoordinateRing, Algebra.norm L[X]
        (WeierstrassCurve.Affine.CoordinateRing.map W (algebraMap K L) a) =
          (Algebra.norm K[X] a).map (algebraMap K L)) ∧
      (∀ f : W.FunctionField, functionValuation (curveOver W L)
          (functionEmbedding W L f) = functionValuation W f ∧
        (InfinityMonic (curveOver W L) (functionEmbedding W L f) ↔ InfinityMonic W f))) ∧
    (∀ (K L : Type) [Field K] [Field L] [Algebra K L] [DecidableEq K] [DecidableEq L]
      (W : WeierstrassCurve.Affine K), Function.Injective (pointEmbedding W L) ∧
      ∀ (P : W.Point) (n : ℕ), functionEmbedding W L (millerFunction W P n) =
        millerFunction (curveOver W L) (pointEmbedding W L P) n) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K) (_ : W.Δ ≠ 0)
      (P : W.Point) (n : ℕ), GeometricMiller W P n (millerFunction W P n) ∧
        ∀ f : W.FunctionField, GeometricMiller W P n f → f = millerFunction W P n) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K) (_ : W.Δ ≠ 0)
      (P : W.Point) (i j : ℕ), millerFunction W P (i + j) =
        millerFunction W P i * millerFunction W P j * stepFunction W (i • P) (j • P)) ∧
    rationalCurve.Δ ≠ 0 ∧
    (∀ (P : rationalCurve.Point) (n : ℕ), IsMillerFunction rationalCurve P n (counterfeitMiller P n) ∧
      counterfeitMiller P n ≠ millerFunction rationalCurve P n ∧
      ¬ GeometricMiller rationalCurve P n (counterfeitMiller P n) ∧
      functionDivisor (curveOver rationalCurve (AlgebraicClosure F3))
          (functionEmbedding rationalCurve (AlgebraicClosure F3) (counterfeitMiller P n))
          (field_base_nonzero rationalCurve (counterfeit_nonzero P n)) ≠
        millerDivisor (curveOver rationalCurve (AlgebraicClosure F3))
          (pointEmbedding rationalCurve (AlgebraicClosure F3) P) n) ∧
    functionDivisor (curveOver rationalCurve (AlgebraicClosure F3))
        (functionEmbedding rationalCurve (AlgebraicClosure F3) rationalFunction)
        (field_base_nonzero rationalCurve rational_function_nonzero) ≠ 0 ∧
    (∀ P : rationalCurve.Point, counterfeitMiller P 0 = rationalFunction ∧
      counterfeitMiller P 1 = rationalFunction) := by
  refine ⟨?_, ?_, ?_, ?_, rational_delta, ?_, mapped_ratio_divisor_nonzero, ?_⟩
  · intro K L _ _ _ W
    refine ⟨field_base_injective W, coordinate_norm_basechange W, ?_⟩
    intro f
    exact ⟨function_valuation_basechange W f, infinity_monic_basechange W f⟩
  · intro K L _ _ _ _ _ W
    exact ⟨point_base_injective W, miller_function_basechange W⟩
  · intro K _ _ W hΔ P n
    exact ⟨constructed_geometric_miller W hΔ P n, fun _ hf => geometric_miller_unique W P n hf⟩
  · intro K _ _ W hΔ P i j
    exact chosen_miller_addition_all_fields W hΔ P i j
  · intro P n
    exact ⟨counterfeit_weak_miller P n, counterfeit_not_chosen P n,
      counterfeit_not_geometric P n, counterfeit_geometric_divisor_wrong P n⟩
  · intro P
    exact ⟨counterfeit_zero P, counterfeit_one P⟩

#print axioms miller_basechange_certificate
end
end MillerBasechangeAudit
