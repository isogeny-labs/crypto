import ActualTorsion
import FrobeniusFiniteExtension
import Mathlib.Algebra.CharP.Algebra

namespace BKUnipotentAudit
noncomputable section
open Polynomial SemaevS3 FrobeniusPointDescentAudit
private instance primeSeven : Fact (Nat.Prime 7) := ⟨by decide⟩
abbrev F := ZMod 7
abbrev curve := TorsionBoundaryAudit.bkCurve
abbrev cubic := TorsionBoundaryAudit.cubic

def seventh : F[X] := C 6 * X^2 + C 6 * X + C 3

theorem cubic_divides_seventh : cubic ∣ X^7-seventh := by
  refine ⟨X^4+2*X^3+X^2+X+2, ?_⟩
  simp only [TorsionBoundaryAudit.cubic,seventh,map_ofNat]
  have h7 : (7:F[X])=0 := CharP.cast_eq_zero _ 7
  linear_combination -(1+2*X+3*X^2+2*X^3+2*X^4+2*X^5+X^6)*h7

theorem cubic_divides_square : cubic ∣ seventh^2-(X+1) := by
  refine ⟨X+4, ?_⟩
  simp only [TorsionBoundaryAudit.cubic,seventh,map_ofNat]
  have h7 : (7:F[X])=0 := CharP.cast_eq_zero _ 7
  linear_combination (3*X+7*X^2+9*X^3+5*X^4)*h7

variable {L : Type*} [Field L] [Algebra F L]
local instance extensionChar : CharP L 7 := charP_of_injective_algebraMap' F 7

theorem seven_zero : (7:L)=0 := CharP.cast_eq_zero L 7

theorem two_ne_zero : (2:L)≠0 := by
  change ¬(2:ℕ)=(0:L)
  rw [CharP.cast_eq_zero_iff L 7]
  decide

theorem three_ne_zero : (3:L)≠0 := by
  change ¬(3:ℕ)=(0:L)
  rw [CharP.cast_eq_zero_iff L 7]
  decide

theorem cubic_aeval (x:L) : aeval x cubic=x^3+5*x^2+3*x+2 := by
  simp [TorsionBoundaryAudit.cubic,map_ofNat]

theorem seventh_aeval (x:L) : aeval x seventh=6*x^2+6*x+3 := by
  simp [seventh,map_ofNat]

theorem aeval_zero_of_cubic_dvd {p : F[X]} (hp : cubic ∣ p) {x:L}
    (hx : x^3+5*x^2+3*x+2=0) : aeval x p=0 := by
  obtain ⟨q,rfl⟩ := hp
  rw [map_mul,cubic_aeval,hx,zero_mul]

theorem cubic_seventh {x:L} (hx : x^3+5*x^2+3*x+2=0) :
    x^7=6*x^2+6*x+3 := by
  have h := aeval_zero_of_cubic_dvd cubic_divides_seventh hx
  simpa only [map_sub,map_pow,aeval_X,seventh_aeval,sub_eq_zero] using h

theorem cubic_seventh_square {x:L} (hx : x^3+5*x^2+3*x+2=0) :
    (6*x^2+6*x+3)^2=x+1 := by
  have h := aeval_zero_of_cubic_dvd cubic_divides_square hx
  simpa only [map_sub,map_pow,map_add,map_one,aeval_X,seventh_aeval,sub_eq_zero] using h

abbrev scalarFrob := FiniteField.frobeniusAlgHom F L

theorem scalar_frob_cubic {x:L} (hx : x^3+5*x^2+3*x+2=0) :
    scalarFrob x=6*x^2+6*x+3 := by
  change x^Fintype.card F=_
  rw [ZMod.card]
  exact cubic_seventh hx

theorem scalar_frob_square_cubic {x:L} (hx : x^3+5*x^2+3*x+2=0) :
    scalarFrob (scalarFrob x)=x^2+6 := by
  rw [scalar_frob_cubic hx]
  simp only [map_add,map_mul,map_pow,map_ofNat]
  rw [scalar_frob_cubic hx,cubic_seventh_square hx]
  linear_combination (5*x^2+6*x+3)*(seven_zero (L:=L))

theorem scalar_frob_cube_cubic {x:L} (hx : x^3+5*x^2+3*x+2=0) :
    scalarFrob (scalarFrob (scalarFrob x))=x := by
  rw [scalar_frob_square_cubic hx]
  simp only [map_add,map_pow,map_ofNat]
  rw [scalar_frob_cubic hx,cubic_seventh_square hx]
  linear_combination (seven_zero (L:=L))

abbrev geomCurve := curve.baseChange L

theorem actual_equation {x y:L} (h : (geomCurve (L:=L)).Nonsingular x y) :
    y^2=x^3+3*x+2 := by
  have he := h.1
  simpa [geomCurve,curve,TorsionBoundaryAudit.bkCurve,
    WeierstrassCurve.baseChange,WeierstrassCurve.map,
    WeierstrassCurve.Affine.equation_iff,map_ofNat] using he

variable [DecidableEq L]

theorem diagonal_of_three {x y:L} (h : (geomCurve (L:=L)).Nonsingular x y)
    (ht : (3:ℕ) • WeierstrassCurve.Affine.Point.some x y h=0) :
    S3 (3:L) 2 x x x=0 := by
  let P : (geomCurve (L:=L)).Point := .some x y h
  have hd : P+P= -P := by
    apply eq_neg_iff_add_eq_zero.mpr
    simpa only [show (3:ℕ)=2+1 by decide,add_nsmul,two_nsmul,one_nsmul] using ht
  have hx : pointX (P+P)=some x := by
    rw [hd,TheoremAudit.pointX_neg]
    rfl
  have hs := (TheoremAudit.S3_repeated_mathlib_iff (geomCurve (L:=L))
    (by simp [geomCurve,curve,TorsionBoundaryAudit.bkCurve,
      WeierstrassCurve.baseChange,WeierstrassCurve.map])
    (by simp [geomCurve,curve,TorsionBoundaryAudit.bkCurve,
      WeierstrassCurve.baseChange,WeierstrassCurve.map])
    (by simp [geomCurve,curve,TorsionBoundaryAudit.bkCurve,
      WeierstrassCurve.baseChange,WeierstrassCurve.map]) two_ne_zero h x).mpr hx
  simpa [geomCurve,curve,TorsionBoundaryAudit.bkCurve,
    WeierstrassCurve.baseChange,WeierstrassCurve.map,map_ofNat] using hs

theorem three_coordinate_classification {x y:L}
    (h : (geomCurve (L:=L)).Nonsingular x y)
    (ht : (3:ℕ) • WeierstrassCurve.Affine.Point.some x y h=0) :
    (x=5 ∧ (y=3 ∨ y= -3)) ∨
      (x^3+5*x^2+3*x+2=0 ∧ (y=3*x ∨ y= -(3*x))) := by
  have hs := diagonal_of_three h ht
  have hf : (3:L)*(x-5)*(x^3+5*x^2+3*x+2)=0 := by
    unfold S3 at hs
    linear_combination -hs-(12*x^2+9*x+3)*(seven_zero (L:=L))
  have he := actual_equation h
  rcases (mul_eq_zero.mp hf) with hz | hc
  · have hx : x=5 := sub_eq_zero.mp ((mul_eq_zero.mp hz).resolve_left three_ne_zero)
    left
    refine ⟨hx,?_⟩
    have hy : y^2=(3:L)^2 := by
      rw [hx] at he
      linear_combination he+19*(seven_zero (L:=L))
    exact (sq_eq_sq_iff_eq_or_eq_neg).mp hy
  · right
    refine ⟨hc,?_⟩
    have hy : y^2=(3*x)^2 := by
      linear_combination he+hc-2*(x^2)*(seven_zero (L:=L))
    exact (sq_eq_sq_iff_eq_or_eq_neg).mp hy

#print axioms cubic_divides_seventh
#print axioms scalar_frob_cube_cubic
#print axioms three_coordinate_classification

end
end BKUnipotentAudit
