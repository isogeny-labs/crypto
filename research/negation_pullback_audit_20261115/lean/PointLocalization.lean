module
public import ChartCertificate
public import LineEvaluation

@[expose] public section
namespace MillerFinitePointAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit
variable {K : Type*} [Field K]

theorem point_eval_constant (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (c : K) :
    pointEval W x y h (algebraMap K W.CoordinateRing c) = c := by
  change AdjoinRoot.evalEval (show W.polynomial.evalEval x y = 0 from h)
    (AdjoinRoot.mk W.polynomial (C (C c))) = c
  rw [AdjoinRoot.evalEval_mk]
  simp only [evalEval_C, eval_C]

theorem point_eval_surjective (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : Function.Surjective (pointEval W x y h) :=
  fun c => ⟨algebraMap K W.CoordinateRing c, point_eval_constant W h c⟩

def evaluationIdeal (W : WeierstrassCurve.Affine K) (x y : K) (h : W.Equation x y) :
    Ideal W.CoordinateRing := RingHom.ker (pointEval W x y h)

instance evaluationMaximal (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : (evaluationIdeal W x y h).IsMaximal :=
  RingHom.ker_isMaximal_of_surjective _ (point_eval_surjective W h)

theorem point_ideal_maximal (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : (XYIdeal W x (C y)).IsMaximal := by
  have he : (W.polynomial.eval (C y)).eval x = 0 := by
    simpa [WeierstrassCurve.Affine.Equation, WeierstrassCurve.Affine.polynomial,
      Polynomial.evalEval] using h
  exact Ideal.Quotient.maximal_of_isField _
    ((quotientXYIdealEquiv he).toMulEquiv.isField (Field.toIsField K))

theorem evaluation_ideal_coordinates (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : evaluationIdeal W x y h = XYIdeal W x (C y) := by
  symm
  apply (point_ideal_maximal W h).eq_of_le (evaluationMaximal W h).ne_top
  apply Ideal.span_le.mpr
  intro a ha
  simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at ha
  rcases ha with rfl | rfl
  · change pointEval W x y h (XClass W x) = 0
    rw [point_eval_XClass, sub_self]
  · change pointEval W x y h (YClass W (C y)) = 0
    rw [point_eval_YClass, eval_C, sub_self]

instance coordinateNoetherian (W : WeierstrassCurve.Affine K) :
    IsNoetherianRing W.CoordinateRing :=
  IsNoetherianRing.of_finite K[X] W.CoordinateRing

abbrev PointLocalRing (W : WeierstrassCurve.Affine K) (x y : K) (h : W.Equation x y) :=
  Localization.AtPrime (evaluationIdeal W x y h)

instance pointLocalDomain (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : IsDomain (PointLocalRing W x y h) :=
  IsLocalization.isDomain_of_local_atPrime (inferInstanceAs (evaluationIdeal W x y h).IsPrime)

def localU (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y) :
    PointLocalRing W x y h :=
  algebraMap W.CoordinateRing (PointLocalRing W x y h) (XClass W x)

def localV (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y) :
    PointLocalRing W x y h :=
  algebraMap W.CoordinateRing (PointLocalRing W x y h) (YClass W (C y))

theorem local_maximal_pair (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) :
    IsLocalRing.maximalIdeal (PointLocalRing W x y h) = Ideal.span {localU W h, localV W h} := by
  rw [← Localization.AtPrime.map_eq_maximalIdeal]
  have he := congrArg (Ideal.map (algebraMap W.CoordinateRing (PointLocalRing W x y h)))
    (evaluation_ideal_coordinates W h)
  rw [he, XYIdeal, Ideal.map_span, Set.image_pair]
  rfl

theorem local_u_nonzero (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y) :
    localU W h ≠ 0 := by
  have hi := IsLocalization.injective (S := PointLocalRing W x y h)
    (evaluationIdeal W x y h).primeCompl_le_nonZeroDivisors
  intro hz
  apply XClass_ne_zero x
  apply hi
  simpa only [localU, map_zero] using hz

theorem local_v_nonzero (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y) :
    localV W h ≠ 0 := by
  have hi := IsLocalization.injective (S := PointLocalRing W x y h)
    (evaluationIdeal W x y h).primeCompl_le_nonZeroDivisors
  intro hz
  apply YClass_ne_zero (C y)
  apply hi
  simpa only [localV, map_zero] using hz

#print axioms evaluation_ideal_coordinates
#print axioms local_u_nonzero
end
end MillerFinitePointAudit
