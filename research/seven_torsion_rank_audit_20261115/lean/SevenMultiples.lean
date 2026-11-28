import SevenKummer

namespace SevenTorsionRankAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine FiveTorsionRankAudit ThreeTorsionRankAudit.General
variable {K : Type*} [Field K] [DecidableEq K]

omit [DecidableEq K] in
theorem inverse_ne_of_two_eval (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (hd : W.Ψ₂Sq.eval x≠0) : y≠W.negY x y := by
  apply (W.den_duplication_eq_zero_iff hN.1).not.mp
  simpa only [two_eval] using hd

/-- Actual tripling, derived from the actual symmetric addition/subtraction theorem. -/
theorem triple_xRep (W : WeierstrassCurve.Affine K) {x y : K} (hN : W.Nonsingular x y)
    (hd : W.Ψ₂Sq.eval x≠0) (ht : W.Ψ₃.eval x≠0) :
    ((3:ℕ) • Point.some x y hN).xRep= ![tripleX W x,1] := by
  let P : W.Point := Point.some x y hN
  have hy := inverse_ne_of_two_eval W hN hd
  let u := W.addX x x (W.slope x x y y)
  let v := W.addY x x y (W.slope x x y y)
  have hQ : W.Nonsingular u v := W.nonsingular_add hN hN (fun hxy => hy hxy.2)
  let Q : W.Point := Point.some u v hQ
  have he : P+P=Q := Point.add_self_of_Y_ne hy
  have hf := actual_double_relation W hN hQ hy he
  have hx : x≠u := by
    intro hxu
    have hh := hf
    rw [←hxu,doubleNumerator_identity] at hh
    exact ht (by linear_combination -hh)
  let w := W.addX x u (W.slope x u y v)
  let z := W.addY x u y (W.slope x u y v)
  have hR : W.Nonsingular w z := W.nonsingular_add hN hQ (fun hxy => hx hxy.1)
  have hs : P+Q=Point.some w z hR := Point.add_of_X_ne hx
  have hminus : P-Q= -P := by rw [←he]; abel
  have hsum := actual_sum_relation W hN hQ hR hs hminus
  have hX : w=tripleX W x := triple_scalar W x u w hd ht hf hsum
  change ((3:ℕ) • P).xRep=_
  rw [show (3:ℕ)=1+2 by decide,add_nsmul,one_nsmul,two_nsmul,he,hs]
  change ![w,1]= ![tripleX W x,1]
  rw [hX]

/-- Actual fourfold multiplication, with both doubling domains derived. -/
theorem quadruple_xRep (W : WeierstrassCurve.Affine K) {x y : K} (hN : W.Nonsingular x y)
    (hd : W.Ψ₂Sq.eval x≠0) (hh : W.preΨ₄.eval x≠0) :
    ((4:ℕ) • Point.some x y hN).xRep= ![quadrupleX W x,1] := by
  let P : W.Point := Point.some x y hN
  have hy := inverse_ne_of_two_eval W hN hd
  let u := W.addX x x (W.slope x x y y)
  let v := W.addY x x y (W.slope x x y y)
  have hQ : W.Nonsingular u v := W.nonsingular_add hN hN (fun hxy => hy hxy.2)
  let Q : W.Point := Point.some u v hQ
  have he : P+P=Q := Point.add_self_of_Y_ne hy
  have hf := actual_double_relation W hN hQ hy he
  have hden := double_denominator_identity W x u hd hf
  have hdu : W.Ψ₂Sq.eval u≠0 := by
    intro hz
    have hp : (W.preΨ₄.eval x)^2=0 := by rw [←hden,hz,mul_zero]
    exact pow_ne_zero 2 hh hp
  have hv := inverse_ne_of_two_eval W hQ hdu
  let w := W.addX u u (W.slope u u v v)
  let z := W.addY u u v (W.slope u u v v)
  have hR : W.Nonsingular w z := W.nonsingular_add hQ hQ (fun hxy => hv hxy.2)
  have hs : Q+Q=Point.some w z hR := Point.add_self_of_Y_ne hv
  have hg := actual_double_relation W hQ hR hv hs
  have hX : w=quadrupleX W x := quadruple_scalar W x u w hd hh hf hg
  change ((4:ℕ) • P).xRep=_
  rw [show (4:ℕ)=2+2 by decide,add_nsmul,two_nsmul,he,hs]
  change ![w,1]= ![quadrupleX W x,1]
  rw [hX]

#print axioms triple_xRep
#print axioms quadruple_xRep
end
end SevenTorsionRankAudit
