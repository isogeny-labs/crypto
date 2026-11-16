module
public import TranslationGeneric

@[expose] public section
namespace EllipticTranslationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerBasechangeAudit FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K]

/-- Elementwise form of the credited official algebraic-closure/minimal-polynomial argument. -/
theorem algebraic_eq_coefficient {L : Type*} [Field L] [Algebra K L] [IsAlgClosed K]
    {x : L} (hx : IsAlgebraic K x) : ∃ c : K, algebraMap K L c=x := by
  have hi := hx.isIntegral
  have hq : (minpoly K x).leadingCoeff=1 := minpoly.monic hi
  have hd : (minpoly K x).degree=1 :=
    IsAlgClosed.degree_eq_one_of_irreducible K (minpoly.irreducible hi)
  have he : aeval x (minpoly K x)=0 := minpoly.aeval K x
  rw [eq_X_add_C_of_degree_eq_one hd,hq,C_1,one_mul,aeval_add,aeval_X,aeval_C,
    add_eq_zero_iff_eq_neg] at he
  exact ⟨-(minpoly K x).coeff 0,(map_neg _ _).trans he.symm⟩

def genericX (W : WeierstrassCurve.Affine K) : (genericCurve W).Point → W.FunctionField
  | .zero => 0
  | .some x _ _ => x

def genericY (W : WeierstrassCurve.Affine K) : (genericCurve W).Point → W.FunctionField
  | .zero => 0
  | .some _ y _ => y

theorem generic_coordinates_nonsingular (W : WeierstrassCurve.Affine K)
    (P : (genericCurve W).Point) (hP : P ≠ 0) :
    (genericCurve W).Nonsingular (genericX W P) (genericY W P) := by
  cases P with
  | zero => exact (hP rfl).elim
  | some x y h => exact h

variable [IsAlgClosed K] [DecidableEq K]

/-- Over actual algebraically closed coefficients, an algebraic abscissa forces the
actual ordinate algebraic too; hence an actual finite point is a constant point. -/
theorem point_constant_of_algebraic_x (W : WeierstrassCurve.Affine K)
    (P : (genericCurve W).Point) (hP : P ≠ 0) (hx : IsAlgebraic K (genericX W P)) :
    ∃ R : W.Point, constantPoint W R=P := by
  cases P with
  | zero => exact (hP rfl).elim
  | some x y h =>
    change IsAlgebraic K x at hx
    obtain ⟨c,hc⟩ := algebraic_eq_coefficient hx
    rw [function_coefficient_map] at hc
    have hy : IsAlgebraic K y := by
      refine ⟨fiberPolynomial W c,fiber_polynomial_nonzero W c,?_⟩
      have he := ((genericCurve W).equation_iff x y).mp h.1
      rw [← hc] at he
      simp only [genericCurve,extendedCurve,WeierstrassCurve.map,function_coefficient_map] at he
      change y^2+constantHom W W.a₁*constantHom W c*y+constantHom W W.a₃*y =
        constantHom W c^3+constantHom W W.a₂*constantHom W c^2+
        constantHom W W.a₄*constantHom W c+constantHom W W.a₆ at he
      simp only [fiberPolynomial,map_sub,map_add,map_mul,map_pow,aeval_X,aeval_C,
        function_coefficient_map,map_add,map_mul,map_pow]
      linear_combination he
    obtain ⟨d,hd⟩ := algebraic_eq_coefficient hy
    rw [function_coefficient_map] at hd
    have hm : (genericCurve W).Nonsingular (constantHom W c) (constantHom W d) := by
      rw [hc,hd]
      exact h
    have hn : W.Nonsingular c d :=
      (W.map_nonsingular (constantHom W).injective c d).mp hm
    refine ⟨Point.some c d hn,?_⟩
    rw [constantPoint,point_base_some]
    simp only [Point.some.injEq]
    exact ⟨hc,hd⟩

/-- Transcendence is derived from actual generic-point addition, not supplied to the field lift. -/
theorem translated_x_transcendental (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) : Transcendental K (genericX W (translatedGeneric W hΔ Q)) := by
  intro hx
  obtain ⟨R,hR⟩ := point_constant_of_algebraic_x W _ (translated_generic_nonzero W hΔ Q) hx
  exact translated_generic_not_constant W hΔ Q R hR.symm

#print axioms point_constant_of_algebraic_x
#print axioms translated_x_transcendental
end
end EllipticTranslationAudit
