import QuarticRoots

namespace ThreeTorsionRankAudit
noncomputable section
open Polynomial SemaevS3
variable {K : Type*} [Field K]

def shortCurve (a b : K) : WeierstrassCurve.Affine K := ⟨0,0,0,a,b⟩

theorem repeated_s3_quartic (a b x : K) :
    S3 a b x x x= -(quartic a b).eval x := by
  rw [quartic_eval]; unfold S3; ring

theorem affine_three_torsion_iff [DecidableEq K] (a b : K)
    (h2 : (2:K)≠0) {x y : K} (h : (shortCurve a b).Nonsingular x y) :
    (3:ℕ) • WeierstrassCurve.Affine.Point.some x y h=0 ↔
      (quartic a b).eval x=0 := by
  let P : (shortCurve a b).Point := .some x y h
  have hs := TheoremAudit.S3_repeated_mathlib_iff (shortCurve a b)
    rfl rfl rfl h2 h x
  change S3 a b x x x=0 ↔ pointX (P+P)=some x at hs
  rw [repeated_s3_quartic,neg_eq_zero] at hs
  constructor
  · intro ht
    have ht' : P+P+P=0 := by
      simpa only [show (3:ℕ)=2+1 by decide,add_nsmul,one_nsmul,two_nsmul] using ht
    have hh : P+P= -P := (eq_neg_iff_add_eq_zero).mpr ht'
    apply hs.mpr
    rw [hh,TheoremAudit.pointX_neg]
    rfl
  · intro hx
    have hxx := hs.mp hx
    have hc : P+P=P ∨ P+P= -P := by
      cases he : P+P with
      | zero => rw [he] at hxx; cases hxx
      | some u v huv =>
        rw [he] at hxx
        have hu : u=x := Option.some.inj hxx
        subst u
        exact WeierstrassCurve.Affine.Point.X_eq_iff.mp rfl
    rcases hc with hc | hc
    · have hz : P=0 := add_right_cancel (show P+P=0+P by simpa using hc)
      have hp : P≠0 := by simp [P]
      exact (hp hz).elim
    · change (3:ℕ) • P=0
      rw [show (3:ℕ)=2+1 by decide,add_nsmul,two_nsmul,one_nsmul,hc,neg_add_cancel]

def squarePolynomial (v : K) : K[X] := X^2-C v

theorem square_polynomial_ne_zero (v : K) : squarePolynomial v≠0 := by
  intro h
  have hh : (squarePolynomial v).natDegree=2 := by simp [squarePolynomial]
  rw [h,natDegree_zero] at hh
  omega

theorem square_root_card [IsAlgClosed K] (v : K) (h2 : (2:K)≠0) (hv : v≠0) :
    Fintype.card ((squarePolynomial v).rootSet K)=2 := by
  change Fintype.card ((X^2-C v).rootSet K)=2
  rw [card_rootSet_eq_natDegree (separable_X_pow_sub_C v h2 hv) (IsAlgClosed.splits _)]
  simp [squarePolynomial]

abbrev xRoots (a b : K) := (quartic a b).rootSet K
abbrev yRoots (a b : K) (x : xRoots a b) := (squarePolynomial (cubicValue a b x)).rootSet K

theorem x_root_eval (a b : K) (x : xRoots a b) : (quartic a b).eval (x:K)=0 := by
  simpa using Polynomial.aeval_eq_zero_of_mem_rootSet x.property

theorem y_root_equation (a b : K) (x : xRoots a b) (y : yRoots a b x) :
    (y:K)^2=cubicValue a b x := by
  have hh := Polynomial.aeval_eq_zero_of_mem_rootSet y.property
  simpa [squarePolynomial,sub_eq_zero] using hh

theorem root_pair_nonsingular (a b : K) (hd : 4*a^3+27*b^2≠0)
    (h2 : (2:K)≠0) (x : xRoots a b) (y : yRoots a b x) :
    (shortCurve a b).Nonsingular x y := by
  have he := y_root_equation a b x y
  have hy : (y:K)≠0 := by
    intro hz
    have hg := quartic_cubic_no_common_root a b hd x (x_root_eval a b x)
    apply hg
    rw [hz,zero_pow (by decide : (2:ℕ)≠0)] at he
    exact he.symm
  apply ((shortCurve a b).nonsingular_iff' (x:K) (y:K)).mpr
  constructor
  · simpa [WeierstrassCurve.Affine.equation_iff,shortCurve,cubicValue] using he
  · right
    simpa [shortCurve] using mul_ne_zero h2 hy

#print axioms affine_three_torsion_iff
#print axioms root_pair_nonsingular
end
end ThreeTorsionRankAudit
