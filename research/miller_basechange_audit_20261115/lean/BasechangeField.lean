module
public import GeometricCertificate

@[expose] public section
namespace MillerBasechangeAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit FunctionDivisorFinitenessAudit
variable {K L : Type*} [Field K] [Field L] [Algebra K L]

abbrev extendedCurve (W : WeierstrassCurve.Affine K) : WeierstrassCurve.Affine L :=
  W.map (algebraMap K L)

/-- Extend the official actual injective coordinate-ring map to the actual full function field. -/
def fieldBaseChange (W : WeierstrassCurve.Affine K) :
    W.FunctionField →+* (extendedCurve (L := L) W).FunctionField :=
  IsFractionRing.map (j := CoordinateRing.map W (algebraMap K L))
    (CoordinateRing.map_injective (W' := W) (algebraMap K L).injective)

theorem field_base_injective (W : WeierstrassCurve.Affine K) :
    Function.Injective (fieldBaseChange (L := L) W) := (fieldBaseChange (L := L) W).injective

theorem field_base_map (W : WeierstrassCurve.Affine K) (a : W.CoordinateRing) :
    fieldBaseChange (L := L) W (algebraMap W.CoordinateRing W.FunctionField a) =
      algebraMap (extendedCurve (L := L) W).CoordinateRing (extendedCurve (L := L) W).FunctionField
        (CoordinateRing.map W (algebraMap K L) a) := by
  simp only [fieldBaseChange, IsFractionRing.map, IsLocalization.map_eq]

theorem field_base_polynomial (W : WeierstrassCurve.Affine K) (p : K[X]) :
    fieldBaseChange (L := L) W (polynomialFunction W p) =
      polynomialFunction (extendedCurve (L := L) W) (p.map (algebraMap K L)) := by
  simp only [polynomialFunction, RingHom.comp_apply, field_base_map,
    CoordinateRing.map_mk, Polynomial.map_C]
  rfl

theorem field_base_constant (W : WeierstrassCurve.Affine K) (c : K) :
    fieldBaseChange (L := L) W (constantHom W c) =
      constantHom (extendedCurve (L := L) W) (algebraMap K L c) := by
  simpa only [polynomial_function_constant, Polynomial.map_C] using
    field_base_polynomial (L := L) W (C c)

theorem field_base_x (W : WeierstrassCurve.Affine K) :
    fieldBaseChange (L := L) W (xFunction W) = xFunction (extendedCurve (L := L) W) := by
  simpa only [Polynomial.map_X, polynomialFunction, xFunction, XClass, map_zero, sub_zero,
    RingHom.comp_apply] using field_base_polynomial (L := L) W X

theorem field_base_y (W : WeierstrassCurve.Affine K) :
    fieldBaseChange (L := L) W (yFunction W) = yFunction (extendedCurve (L := L) W) := by
  simpa only [yFunction, YClass, map_zero, sub_zero, CoordinateRing.map_mk, Polynomial.map_X] using
    field_base_map (L := L) W (mk W X)

#print axioms field_base_injective
#print axioms field_base_polynomial
end
end MillerBasechangeAudit
