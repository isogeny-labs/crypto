import FrobeniusField

namespace IsogenyFactorizationGuardAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit MillerBasechangeAudit MillerNormValuationAudit
noncomputable instance geometricClosureDecidableEq (A : Type*) [Field A] :
    DecidableEq (AlgebraicClosure A) := Classical.decEq _
variable {K : Type*} [Field K] [Fintype K]

def pointFrobenius (W : WeierstrassCurve.Affine K) (L : Type*)
    [Field L] [Algebra K L] [DecidableEq L] :
    (extendedCurve (L:=L) W).Point →+ (extendedCurve (L:=L) W).Point :=
  Point.map (W':=W) (FiniteField.frobeniusAlgHom K L)

theorem point_frobenius_injective (W : WeierstrassCurve.Affine K) (L : Type*)
    [Field L] [Algebra K L] [DecidableEq L] : Function.Injective (pointFrobenius W L) :=
  Point.map_injective _

theorem point_frobenius_kernel (W : WeierstrassCurve.Affine K) (L : Type*)
    [Field L] [Algebra K L] [DecidableEq L] (P : (extendedCurve (L:=L) W).Point) :
    pointFrobenius W L P = 0 ↔ P = 0 :=
  (point_frobenius_injective W L).eq_iff' (map_zero _)

theorem geometric_frobenius_bijective (W : WeierstrassCurve.Affine K) :
    Function.Bijective (pointFrobenius W (AlgebraicClosure K)) := by
  let e := FiniteField.frobeniusAlgEquivOfAlgebraic K (AlgebraicClosure K)
  have he : e.toAlgHom = FiniteField.frobeniusAlgHom K (AlgebraicClosure K) := by
    ext x
    rfl
  refine ⟨point_frobenius_injective W _,?_⟩
  intro P
  refine ⟨Point.map (W':=W) e.symm.toAlgHom P,?_⟩
  change Point.map (W':=W) (FiniteField.frobeniusAlgHom K (AlgebraicClosure K))
    (Point.map (W':=W) e.symm.toAlgHom P) = P
  rw [← he]
  cases P with
  | zero => rfl
  | some x y h =>
    change Point.some _ _ _ = Point.some x y h
    exact Eq.mpr (Point.some.injEq _ _ _ _ _ _)
      ⟨e.apply_symm_apply x,e.apply_symm_apply y⟩

theorem generic_frobenius_x (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    genericX W (pointFrobenius W W.FunctionField (genericPoint W hΔ)) =
      fieldFrobenius W (xFunction W) := by
  exact generic_x_map W (fieldFrobenius W) (genericPoint W hΔ)

theorem generic_frobenius_y (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    genericY W (pointFrobenius W W.FunctionField (genericPoint W hΔ)) =
      fieldFrobenius W (yFunction W) := by
  exact generic_y_map W (fieldFrobenius W) (genericPoint W hΔ)

#print axioms point_frobenius_kernel
#print axioms geometric_frobenius_bijective
#print axioms generic_frobenius_x
end
end IsogenyFactorizationGuardAudit
