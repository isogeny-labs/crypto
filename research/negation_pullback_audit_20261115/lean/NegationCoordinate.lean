module
public import FunctionDivisorCertificate

@[expose] public section
namespace NegationPullbackAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit MillerFinitePointAudit FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K]

def linearH (W : WeierstrassCurve.Affine K) : K[X] := C W.a₁ * X + C W.a₃

def negativeRoot (W : WeierstrassCurve.Affine K) : W.CoordinateRing :=
  -AdjoinRoot.root W.polynomial - AdjoinRoot.of W.polynomial (linearH W)

theorem negative_root_equation (W : WeierstrassCurve.Affine K) :
    W.polynomial.eval₂ (AdjoinRoot.of W.polynomial) (negativeRoot W) = 0 := by
  have h := AdjoinRoot.eval₂_root W.polynomial
  have he (a : W.CoordinateRing) :
      W.polynomial.eval₂ (AdjoinRoot.of W.polynomial) a =
        a ^ 2 + AdjoinRoot.of W.polynomial (linearH W) * a -
          AdjoinRoot.of W.polynomial
            (X ^ 3 + C W.a₂ * X ^ 2 + C W.a₄ * X + C W.a₆) := by
    change eval₂ _ a ((X : K[X][X]) ^ 2 + C (linearH W) * X -
      C (X ^ 3 + C W.a₂ * X ^ 2 + C W.a₄ * X + C W.a₆)) = _
    simp only [eval₂_sub, eval₂_add, eval₂_mul, eval₂_pow, eval₂_X, eval₂_C]
  rw [he] at h ⊢
  simp only [negativeRoot]
  linear_combination h

/-- Pullback by actual Weierstrass point negation, on the actual coordinate ring. -/
def coordinateNeg (W : WeierstrassCurve.Affine K) : W.CoordinateRing →+* W.CoordinateRing :=
  AdjoinRoot.lift (AdjoinRoot.of W.polynomial) (negativeRoot W) (negative_root_equation W)

theorem coordinate_neg_coefficient (W : WeierstrassCurve.Affine K) (p : K[X]) :
    coordinateNeg W (mk W (C p)) = mk W (C p) :=
  AdjoinRoot.lift_of (negative_root_equation W)

theorem coordinate_neg_root (W : WeierstrassCurve.Affine K) :
    coordinateNeg W (mk W X) = -mk W X - mk W (C (linearH W)) :=
  AdjoinRoot.lift_root (negative_root_equation W)

theorem coordinate_neg_square (W : WeierstrassCurve.Affine K) :
    (coordinateNeg W).comp (coordinateNeg W) = RingHom.id W.CoordinateRing := by
  apply AdjoinRoot.ringHom_ext
  · apply RingHom.ext
    intro p
    change coordinateNeg W (coordinateNeg W (mk W (C p))) = mk W (C p)
    rw [coordinate_neg_coefficient, coordinate_neg_coefficient]
  · change coordinateNeg W (coordinateNeg W (mk W X)) = mk W X
    rw [coordinate_neg_root, map_sub, map_neg, coordinate_neg_root,
      coordinate_neg_coefficient]
    ring

theorem coordinate_neg_involutive (W : WeierstrassCurve.Affine K) :
    Function.Involutive (coordinateNeg W) :=
  fun a => RingHom.congr_fun (coordinate_neg_square W) a

def coordinateNegEquiv (W : WeierstrassCurve.Affine K) : W.CoordinateRing ≃+* W.CoordinateRing :=
  RingEquiv.ofBijective (coordinateNeg W) (coordinate_neg_involutive W).bijective

theorem coordinate_neg_basis (W : WeierstrassCurve.Affine K) (p q : K[X]) :
    coordinateNeg W (p • (1 : W.CoordinateRing) + q • mk W X) =
      (p - q * linearH W) • 1 + (-q) • mk W X := by
  simp only [smul, map_add, map_mul, mul_one, coordinate_neg_coefficient,
    coordinate_neg_root, map_sub, map_neg]
  ring

theorem coordinate_neg_norm (W : WeierstrassCurve.Affine K) (a : W.CoordinateRing) :
    Algebra.norm K[X] (coordinateNeg W a) = Algebra.norm K[X] a := by
  obtain ⟨p, q, rfl⟩ := exists_smul_basis_eq a
  rw [coordinate_neg_basis, norm_smul_basis, norm_smul_basis]
  simp only [linearH]
  ring

theorem point_eval_negation (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (a : W.CoordinateRing) :
    pointEval W x y h (coordinateNeg W a) =
      pointEval W x (W.negY x y) ((W.equation_neg x y).mpr h) a := by
  obtain ⟨p, q, rfl⟩ := exists_smul_basis_eq a
  have hy (z : K) (hz : W.Equation x z) : pointEval W x z hz (mk W X) = z := by
    simpa only [YClass, map_zero, sub_zero, eval_zero] using point_eval_YClass W hz (0 : K[X])
  rw [coordinate_neg_basis]
  simp only [smul, map_add, map_mul, mul_one, point_eval_polynomial, hy,
    eval_sub, eval_mul, eval_neg, linearH, eval_add, eval_C, eval_X,
    WeierstrassCurve.Affine.negY]
  ring

#print axioms coordinate_neg_square
#print axioms coordinate_neg_norm
#print axioms point_eval_negation
end
end NegationPullbackAudit
