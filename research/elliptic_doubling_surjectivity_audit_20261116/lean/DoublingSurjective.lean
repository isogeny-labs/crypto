import DoublingFibers

namespace EllipticDoublingSurjectivityAudit
noncomputable section
open WeierstrassCurve.Affine WeilAuxiliaryRatioAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

omit [DecidableEq K] [IsAlgClosed K] in
theorem actual_equal_x_ordinates (W : WeierstrassCurve.Affine K) (x y v : K)
    (hy : W.Equation x y) (hv : W.Equation x v) : y=v ∨ y=W.negY x v := by
  have he : (y-v)*(y-W.negY x v)=0 := by
    rw [WeierstrassCurve.Affine.negY]
    have hy' := (W.equation_iff' x y).mp hy
    have hv' := (W.equation_iff' x v).mp hv
    linear_combination hy'-hv'
  exact (mul_eq_zero.mp he).imp sub_eq_zero.mp sub_eq_zero.mp

theorem actual_doubling_surjective (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Function.Surjective (fun P : W.Point=>P+P) := by
  intro Q
  cases Q with
  | zero => exact ⟨0,zero_add 0⟩
  | some a b hQ =>
    obtain ⟨u,v,h,hn,hx⟩:=actual_half_coordinates W hΔ a
    let m:=W.slope u u v v
    let z:=W.addY u u v m
    have hz : W.Nonsingular a z := by
      have hh:=W.nonsingular_add h h (fun hxy=>hn hxy.2)
      rwa [hx] at hh
    let P : W.Point:=.some u v h
    have hp : P+P=Point.some a z hz := by
      change Point.some u v h+Point.some u v h=_
      rw [Point.add_self_of_Y_ne hn]
      simp only [Point.some.injEq]
      exact ⟨hx,rfl⟩
    rcases actual_equal_x_ordinates W a z b hz.1 hQ.1 with he|he
    · refine ⟨P,hp.trans ?_⟩
      simp only [Point.some.injEq]
      exact ⟨True.intro,he⟩
    · have hp' : P+P= -(Point.some a b hQ) := by
        rw [hp,Point.neg_some]
        simp only [Point.some.injEq]
        exact ⟨True.intro,he⟩
      exact ⟨-P,by
        change -P+ -P=Point.some a b hQ
        rw [←neg_add,hp',neg_neg]⟩

#print axioms actual_equal_x_ordinates
#print axioms actual_doubling_surjective
end
end EllipticDoublingSurjectivityAudit
