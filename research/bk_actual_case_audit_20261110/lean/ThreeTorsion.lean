import FrobeniusFiniteExtension
import Mathlib.Algebra.CharP.Algebra

/-! A qualified actual BK case: E/F5:y²=x³+1 and r=3.
The coordinate classification and Frobenius-square identity use actual
Mathlib torsion points, without a torsion basis/determinant assumption. -/
namespace BKActualThreeAudit
noncomputable section
open SemaevS3 TheoremAudit FrobeniusPointDescentAudit
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev F := ZMod 5
abbrev curve := SkepticalAudit.rationalCurve
variable {L : Type*} [Field L] [Algebra F L]
local instance extensionChar : CharP L 5 := charP_of_injective_algebraMap' F 5
abbrev geomCurve := curve.baseChange L

theorem five_zero : (5:L)=0 := CharP.cast_eq_zero L 5

theorem two_ne_zero : (2:L)≠0 := by
  change ¬(2:ℕ)=(0:L)
  rw [CharP.cast_eq_zero_iff L 5]
  decide

theorem three_ne_zero : (3:L)≠0 := by
  change ¬(3:ℕ)=(0:L)
  rw [CharP.cast_eq_zero_iff L 5]
  decide

theorem coefficients :
    (geomCurve (L:=L)).a₁=0 ∧ (geomCurve (L:=L)).a₂=0 ∧
    (geomCurve (L:=L)).a₃=0 ∧ (geomCurve (L:=L)).a₄=0 ∧
    (geomCurve (L:=L)).a₆=1 := by
  simp [geomCurve,curve,SkepticalAudit.rationalCurve,
    WeierstrassCurve.baseChange,WeierstrassCurve.map]

theorem geom_nonsingular : (geomCurve (L:=L)).Δ≠0 := by
  change (curve.map (algebraMap F L)).Δ≠0
  rw [WeierstrassCurve.map_Δ]
  exact (map_ne_zero (algebraMap F L)).mpr SkepticalAudit.rationalCurve_nonsingular

theorem actual_equation {x y : L} (h : (geomCurve (L:=L)).Nonsingular x y) :
    y^2=x^3+1 := by
  have he := h.1
  rw [WeierstrassCurve.Affine.equation_iff] at he
  obtain ⟨h1,h2,h3,h4,h6⟩ := coefficients (L:=L)
  simpa only [h1,h2,h3,h4,h6,zero_mul,zero_add,mul_zero,add_zero] using he

omit [Algebra F L] in
theorem short_diagonal (x : L) : S3 (0:L) 1 x x x= -3*x^4-12*x := by
  unfold S3
  ring

variable [DecidableEq L]

theorem diagonal_of_three {x y : L} (h : (geomCurve (L:=L)).Nonsingular x y)
    (ht : (3:ℕ) • WeierstrassCurve.Affine.Point.some x y h=0) :
    S3 (0:L) 1 x x x=0 := by
  let P : (geomCurve (L:=L)).Point := .some x y h
  have hd : P+P= -P := by
    apply eq_neg_iff_add_eq_zero.mpr
    simpa only [show (3:ℕ)=2+1 by decide,add_nsmul,two_nsmul,one_nsmul] using ht
  have hx : pointX (P+P)=some x := by
    rw [hd,TheoremAudit.pointX_neg]
    rfl
  obtain ⟨h1,h2,h3,h4,h6⟩ := coefficients (L:=L)
  have hh := (S3_repeated_mathlib_iff (geomCurve (L:=L)) h1 h2 h3 two_ne_zero h x).mpr hx
  simpa only [h4,h6] using hh

theorem three_coordinate_classification {x y : L}
    (h : (geomCurve (L:=L)).Nonsingular x y)
    (ht : (3:ℕ) • WeierstrassCurve.Affine.Point.some x y h=0) :
    x=0 ∨ x^3=1 := by
  have hs := diagonal_of_three h ht
  rw [short_diagonal] at hs
  have hf : (3:L)*x*(x^3-1)=0 := by
    linear_combination -hs-3*x*(five_zero (L:=L))
  rcases mul_eq_zero.mp hf with hh | hh
  · exact Or.inl ((mul_eq_zero.mp hh).resolve_left three_ne_zero)
  · exact Or.inr (sub_eq_zero.mp hh)

omit [Algebra F L] [DecidableEq L] in
theorem x_pow_twenty_five (x : L) (h : x=0 ∨ x^3=1) : x^25=x := by
  rcases h with rfl | h
  · norm_num
  · calc
      x^25=(x^3)^8*x := by ring
      _=x := by rw [h]; norm_num

omit [DecidableEq L] in
theorem two_pow_twelve : (2:L)^12=1 := by
  norm_num
  linear_combination 819*(five_zero (L:=L))

omit [DecidableEq L] in
theorem y_pow_twenty_five (y : L) (h : y^2=1 ∨ y^2=2) : y^25=y := by
  calc
    y^25=(y^2)^12*y := by ring
    _=y := by
      rcases h with h | h
      · rw [h]; norm_num
      · rw [h,two_pow_twelve]; norm_num

theorem frobenius_square_fixed (P : (geomCurve (L:=L)).Point)
    (ht : (3:ℕ) • P=0) : pointFrobenius curve (pointFrobenius curve P)=P := by
  cases P with
  | zero => rfl
  | some x y h =>
    have hx := three_coordinate_classification h ht
    have hy : y^2=1 ∨ y^2=2 := by
      have he := actual_equation h
      rcases hx with hx | hx
      · left
        simpa only [hx,zero_pow (by decide : (3:ℕ)≠0),zero_add] using he
      · right
        simpa only [hx,one_add_one_eq_two] using he
    have hxp := x_pow_twenty_five x hx
    have hyp := y_pow_twenty_five y hy
    change WeierstrassCurve.Affine.Point.some
      (FiniteField.frobeniusAlgHom F L (FiniteField.frobeniusAlgHom F L x))
      (FiniteField.frobeniusAlgHom F L (FiniteField.frobeniusAlgHom F L y)) _=
        WeierstrassCurve.Affine.Point.some x y h
    rw [WeierstrassCurve.Affine.Point.some.injEq]
    change (x^Fintype.card F)^Fintype.card F=x ∧
      (y^Fintype.card F)^Fintype.card F=y
    simpa only [ZMod.card,←pow_mul] using And.intro hxp hyp

#print axioms three_coordinate_classification
#print axioms frobenius_square_fixed
end
end BKActualThreeAudit
