import CyclicObstruction

namespace PairingKOneAudit
noncomputable section
open Polynomial

def tangentSlope (x y : F) : F := (3*x^2+3)/(2*y)
def tangentY (x y : F) : F[X] := C (tangentSlope x y)*(X-C x)+C y
def intersectionPolynomial (x y : F) : F[X] := X^3+3*X+2-(tangentY x y)^2

def InflectionCertificate : curve.Point → Prop
  | .zero => True
  | .some x y _ => y≠0 ∧ intersectionPolynomial x y=(X-C x)^3

def FiniteInflectionData : curve.Point → Prop
  | .zero => True
  | .some x y _ => y≠0 ∧ (tangentSlope x y)^2=3*x ∧
      2*tangentSlope x y*y=3*x^2+3 ∧ 2-(y-tangentSlope x y*x)^2= -x^3

instance finiteInflectionDataDecidable (R : curve.Point) : Decidable (FiniteInflectionData R) := by
  cases R <;> unfold FiniteInflectionData <;> infer_instance

theorem indexed_inflection_data : ∀ u : ZMod 3,FiniteInflectionData (torsionPoint u) := by
  decide +kernel

theorem data_implies_certificate (R : curve.Point) (hR : FiniteInflectionData R) :
    InflectionCertificate R := by
  cases R with
  | zero => trivial
  | some x y h =>
    obtain ⟨hy,hs,ht,hc⟩ := hR
    refine ⟨hy,?_⟩
    have hs' : (C (tangentSlope x y):F[X])^2=3*C x := by
      simpa only [map_pow,map_mul,C_ofNat] using congrArg (fun z : F => (C z:F[X])) hs
    have ht' : (2:F[X])*C (tangentSlope x y)*C y=3*(C x)^2+3 := by
      simpa only [map_pow,map_mul,map_add,C_ofNat] using congrArg (fun z : F => (C z:F[X])) ht
    have hc' : (2:F[X])-(C y-C (tangentSlope x y)*C x)^2= -(C x)^3 := by
      simpa only [map_sub,map_mul,map_pow,map_neg,C_ofNat] using
        congrArg (fun z : F => (C z:F[X])) hc
    unfold intersectionPolynomial tangentY
    linear_combination (-X^2+2*C x*X)*hs' - X*ht' + hc'

theorem actual_inflection_certificate (U : threeTorsion) : InflectionCertificate U.val := by
  obtain ⟨u,rfl⟩ := torsionHom_bijective.surjective U
  exact data_implies_certificate _ (indexed_inflection_data u)

theorem line_matches_polynomial {xp yp xr yr : F}
    (hp : curve.Nonsingular xp yp) (hr : curve.Nonsingular xr yr) :
    millerLine (.some xp yp hp) (.some xr yr hr)=yr-(tangentY xp yp).eval xr := by
  simp only [millerLine,tangentY,tangentSlope,eval_add,eval_mul,eval_C,eval_sub,eval_X]
  ring

/-- Algebraic infinity-chart identity; a DVR/valuation/divisor bridge is not asserted. -/
theorem infinity_chart_factor (u v : F) (h : v=u^3+3*u*v^2+2*v^3) :
    v*(1-3*u*v-2*v^2)=u^3 := by
  linear_combination h

theorem tangent_chart_nonzero_at_infinity (slope intercept : F) :
    1-slope*0-intercept*0≠0 := by simp

def WeilAuxSupport (U V S : curve.Point) : Prop :=
  millerLine V (U+S)≠0 ∧ millerLine V S≠0 ∧
    millerLine U (-S)≠0 ∧ millerLine U (V-S)≠0

instance weilAuxSupportDecidable (U V S : curve.Point) : Decidable (WeilAuxSupport U V S) := by
  unfold WeilAuxSupport
  infer_instance

def weilAtAux (U V S : curve.Point) : F :=
  (millerLine V (U+S)/millerLine V S)*(millerLine U (-S)/millerLine U (V-S))

theorem indexed_weil_auxiliary_table : ∀ (u v : ZMod 3) (s : ZMod 9),
    WeilAuxSupport (torsionPoint u) (torsionPoint v) (indexedPoint s) →
      weilAtAux (torsionPoint u) (torsionPoint v) (indexedPoint s)=1 := by
  decide +kernel

theorem weil_all_valid_auxiliary_actual (U V : threeTorsion) (S : curve.Point)
    (h : WeilAuxSupport U.val V.val S) : weilAtAux U.val V.val S=1 := by
  obtain ⟨u,rfl⟩ := torsionHom_bijective.surjective U
  obtain ⟨v,rfl⟩ := torsionHom_bijective.surjective V
  obtain ⟨s,rfl⟩ := indexHom_surjective S
  exact indexed_weil_auxiliary_table u v s h

theorem fixed_weil_matches (U V : threeTorsion) : weilValue U V=weilAtAux U.val V.val G := rfl

#print axioms actual_inflection_certificate
#print axioms line_matches_polynomial
#print axioms weil_all_valid_auxiliary_actual

end
end PairingKOneAudit
