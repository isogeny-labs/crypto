import WeilTarget

namespace PairingMillerTangentAudit
noncomputable section
open PairingMillerCurveAudit PairingMillerExpressionsAudit PairingMillerTorsionAudit
open Polynomial

def tangentSlope (x y : F25) : F25 := 3*x^2/(2*y)
def tangentY (x y : F25) : F25[X] := C (tangentSlope x y)*(X-C x)+C y
def intersectionPolynomial (x y : F25) : F25[X] := X^3+1-(tangentY x y)^2

/-- Concrete polynomial intersection certificate. This is not a definition
of a geometric principal divisor or a valuation at infinity. -/
def InflectionCertificate : curve.Point→Prop
  | .zero => True
  | .some x y _ => y≠0 ∧ intersectionPolynomial x y=(X-C x)^3

def FiniteInflectionData : curve.Point→Prop
  | .zero => True
  | .some x y _ => y≠0 ∧ (tangentSlope x y)^2=3*x ∧
      2*tangentSlope x y*y=3*x^2 ∧ 1-(y-tangentSlope x y*x)^2= -x^3

instance finiteInflectionDataDecidable (R : curve.Point) : Decidable (FiniteInflectionData R) := by
  cases R <;> unfold FiniteInflectionData <;> infer_instance

theorem indexed_inflection_data : ∀ u : Index,FiniteInflectionData (indexedPoint u) := by
  decide +kernel

theorem data_implies_certificate (R : curve.Point) (hR : FiniteInflectionData R) :
    InflectionCertificate R := by
  cases R with
  | zero => trivial
  | some x y h =>
    obtain ⟨hy,hs,ht,hc⟩:=hR
    refine ⟨hy,?_⟩
    have hs' : (C (tangentSlope x y):F25[X])^2=3*C x := by
      simpa only [map_pow,map_mul,C_ofNat] using congrArg (fun z : F25=>(C z:F25[X])) hs
    have ht' : (2:F25[X])*C (tangentSlope x y)*C y=3*(C x)^2 := by
      simpa only [map_pow,map_mul,C_ofNat] using congrArg (fun z : F25=>(C z:F25[X])) ht
    have hc' : (1:F25[X])-(C y-C (tangentSlope x y)*C x)^2= -(C x)^3 := by
      simpa using congrArg (fun z : F25=>(C z:F25[X])) hc
    unfold intersectionPolynomial tangentY
    linear_combination (-X^2+2*C x*X)*hs' - X*ht' + hc'

theorem indexed_inflection_certificate (u : Index) : InflectionCertificate (indexedPoint u) :=
  data_implies_certificate _ (indexed_inflection_data u)

theorem every_torsion_inflection_certificate (U : threeTorsion) : InflectionCertificate U.val := by
  obtain ⟨u,rfl⟩:=torsionHom_bijective.surjective U
  exact indexed_inflection_certificate u

theorem line_matches_polynomial {xp yp xr yr : F25}
    (hp : curve.Nonsingular xp yp) (hr : curve.Nonsingular xr yr) :
    millerLine (.some xp yp hp) (.some xr yr hr)=yr-(tangentY xp yp).eval xr := by
  simp only [millerLine,tangentY,tangentSlope,eval_add,eval_mul,eval_C,eval_sub,eval_X]
  ring

/-- Algebraic local-chart identity only. Turning the unit factors into
orders of vanishing remains a separate geometric formalization. -/
theorem infinity_chart_factor (u v : F25) (h : v=u^3+v^3) : v*(1-v^2)=u^3 := by
  linear_combination h

theorem tangent_chart_nonzero_at_infinity (slope intercept : F25) :
    1-slope*0-intercept*0≠0 := by simp

#print axioms every_torsion_inflection_certificate
#print axioms line_matches_polynomial
end
end PairingMillerTangentAudit
