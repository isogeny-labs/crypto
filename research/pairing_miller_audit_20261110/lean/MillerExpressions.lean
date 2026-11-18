import MillerCurve

/-! Prescribed order-three Miller expressions on actual Mathlib curve points.
The tangent is the one-step n=3 Miller function for nonzero 3-torsion.
Its divisor/canonical-pairing identification is a separate obligation. -/
namespace PairingMillerExpressionsAudit
open PairingMillerCurveAudit

def millerLine : curve.Point→curve.Point→F25
  | .zero,_ => 1
  | .some _ _ _,.zero => 0
  | .some xp yp _,.some xr yr _ => yr-yp-(3*xp^2/(2*yp))*(xr-xp)

/-- Galbraith (26.1), with D_P=(P+S)-(S), D_Q=(Q)-(O).
The function for D_P is translated by -S. -/
def weilMiller (U V : curve.Point) : F25 :=
  (millerLine V (U+auxiliary)/millerLine V auxiliary)*
  (millerLine U (-auxiliary)/millerLine U (V-auxiliary))

/-- Galbraith (26.2), with D=(V+S)-(S), then final exponent (25-1)/3=8. -/
def reducedTateMiller (U V : curve.Point) : F25 :=
  (millerLine U (V+auxiliary)/millerLine U auxiliary)^8

theorem weil_generator_values :
    weilMiller P Q=omega ∧ weilMiller Q P=omega^2 ∧
    weilMiller P P=1 ∧ weilMiller Q Q=1 := by decide +kernel

theorem tate_generator_values :
    reducedTateMiller P Q=omega^2 ∧ reducedTateMiller Q P=omega ∧
    reducedTateMiller P P=1 ∧ reducedTateMiller Q Q=1 := by decide +kernel

theorem auxiliary_three : (3:ℕ) • auxiliary=auxiliary := by
  rw [show (3:ℕ)=2+1 by decide,add_nsmul,two_nsmul,auxiliary_two,one_nsmul,zero_add]

theorem shifted_points_separated (u v : Index) : indexedPoint u+auxiliary≠indexedPoint v := by
  intro h
  have hh:=congrArg (fun R : curve.Point=>(3:ℕ) • R) h
  rw [nsmul_add,indexedPoint_three,indexedPoint_three,auxiliary_three,zero_add] at hh
  exact auxiliary_ne_zero hh

theorem shifted_point_nonzero (v : Index) : indexedPoint v+auxiliary≠0 := by
  have hz : indexedPoint (0:Index)=0 := by simp [indexedPoint]
  rw [←hz]
  exact shifted_points_separated v 0

theorem auxiliary_neg : -auxiliary=auxiliary := by
  exact (eq_neg_iff_add_eq_zero.mpr auxiliary_two).symm

theorem generator_denominators_nonzero :
    millerLine P auxiliary≠0 ∧ millerLine Q auxiliary≠0 ∧
    millerLine P (Q-auxiliary)≠0 ∧ millerLine Q (P-auxiliary)≠0 := by
  decide +kernel

#print axioms weil_generator_values
#print axioms tate_generator_values
end PairingMillerExpressionsAudit
