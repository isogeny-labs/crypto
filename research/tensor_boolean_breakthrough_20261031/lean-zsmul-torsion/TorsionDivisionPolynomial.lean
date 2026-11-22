import Mathlib.AlgebraicGeometry.EllipticCurve.DivisionPolynomial.ZSMul

open Polynomial

namespace WeierstrassCurve

open Jacobian

variable {F : Type*} [Field F] (W : WeierstrassCurve F)

/-- A torsion relation at a nonsingular affine point forces the corresponding
univariate squared division polynomial to vanish at its `x`-coordinate. -/
theorem divisionPolynomialSq_eval_eq_zero_of_zsmul_eq_zero
    {x y : F} (h : Affine.Nonsingular W x y) (n : ℤ)
    (hn : n • Point.fromAffine (Affine.Point.some _ _ h) = 0) :
    (W.ΨSq n).eval x = 0 := by
  have hpoint := zsmul_eq_smulEval W h n
  rw [hn, Point.zero_point] at hpoint
  have hequiv : (![1, 1, 0] : Fin 3 → F) ≈ smulEval W x y n :=
    Quotient.eq.mp hpoint
  have hψ : evalEval x y (W.ψ n) = 0 := by
    have hz : (smulEval W x y n) (2 : Fin 3) = 0 :=
      (Jacobian.Z_eq_zero_of_equiv hequiv).mp rfl
    simpa [smulEval] using hz
  let ev : W.toAffine.CoordinateRing →+* F := AdjoinRoot.evalEval h.1
  have hψΨ : evalEval x y (W.ψ n) = evalEval x y (W.Ψ n) := by
    calc
      _ = ev (Affine.CoordinateRing.mk W (W.ψ n)) :=
        (AdjoinRoot.evalEval_mk h.1 _).symm
      _ = ev (Affine.CoordinateRing.mk W (W.Ψ n)) :=
        congrArg ev (Affine.CoordinateRing.mk_ψ W n)
      _ = _ := AdjoinRoot.evalEval_mk h.1 _
  have hΨ : evalEval x y (W.Ψ n) = 0 := by
    exact hψΨ.symm.trans hψ
  have hsq' : evalEval x y (W.Ψ n) ^ 2 = (W.ΨSq n).eval x := by
    calc
      _ = (ev (Affine.CoordinateRing.mk W (W.Ψ n))) ^ 2 :=
        congrArg (fun t : F => t ^ 2) (AdjoinRoot.evalEval_mk h.1 _).symm
      _ = ev ((Affine.CoordinateRing.mk W (W.Ψ n)) ^ 2) := (map_pow ev _ 2).symm
      _ = ev (Affine.CoordinateRing.mk W (C <| W.ΨSq n)) :=
        congrArg ev (Affine.CoordinateRing.mk_Ψ_sq W n)
      _ = evalEval x y (C <| W.ΨSq n) := AdjoinRoot.evalEval_mk h.1 _
      _ = _ := evalEval_C x y _
  rw [hΨ] at hsq'
  simpa using hsq'.symm

end WeierstrassCurve
