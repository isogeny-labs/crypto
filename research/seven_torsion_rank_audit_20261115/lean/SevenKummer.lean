import SevenPolynomial
import SevenKummerImports

namespace SevenTorsionRankAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine FiveTorsionRankAudit ThreeTorsionRankAudit.General
variable {K : Type*} [Field K]

def sumNumerator (W : WeierstrassCurve K) (x u : K) : K :=
  2*(x+u)*(x*u)+W.b₂*(x*u)+W.b₄*(x+u)+W.b₆
def tripleX (W : WeierstrassCurve K) (x : K) : K :=
  x-W.Ψ₂Sq.eval x*W.preΨ₄.eval x/(W.Ψ₃.eval x)^2
def quadrupleX (W : WeierstrassCurve K) (x : K) : K :=
  x-W.Ψ₃.eval x*(psiFive W).eval x/(W.Ψ₂Sq.eval x*(W.preΨ₄.eval x)^2)

theorem double_displacement (W : WeierstrassCurve K) (x u : K)
    (hd : W.Ψ₂Sq.eval x≠0) (h : doubleNumerator W x=W.Ψ₂Sq.eval x*u) :
    u-x= -W.Ψ₃.eval x/W.Ψ₂Sq.eval x := by
  rw [doubleNumerator_identity] at h
  apply (eq_div_iff hd).mpr
  linear_combination -h

theorem sum_identity (W : WeierstrassCurve K) (x u : K) :
    sumNumerator W x u-2*x*(x-u)^2=(central W).eval x*(u-x)+W.Ψ₂Sq.eval x := by
  simp [sumNumerator,central,WeierstrassCurve.Ψ₂Sq]
  ring

theorem central_identity_eval (W : WeierstrassCurve K) (x : K) :
    W.Ψ₂Sq.eval x*(central W).derivative.eval x=((central W).eval x)^2+4*W.Ψ₃.eval x := by
  simpa using congrArg (Polynomial.evalRingHom x) (central_identity W)

theorem denominator_taylor (W : WeierstrassCurve K) (x u : K) :
    W.Ψ₂Sq.eval u=W.Ψ₂Sq.eval x+2*(central W).eval x*(u-x)+
      (central W).derivative.eval x*(u-x)^2+4*(u-x)^3 := by
  simp [WeierstrassCurve.Ψ₂Sq,central,Polynomial.derivative_pow]
  ring

theorem double_denominator_identity (W : WeierstrassCurve K) (x u : K)
    (hd : W.Ψ₂Sq.eval x≠0) (h : doubleNumerator W x=W.Ψ₂Sq.eval x*u) :
    (W.Ψ₂Sq.eval x)^3*W.Ψ₂Sq.eval u=(W.preΨ₄.eval x)^2 := by
  rw [denominator_taylor W x u,double_displacement W x u hd h,fourth_eval]
  field_simp
  linear_combination (W.Ψ₃.eval x)^2*central_identity_eval W x

theorem triple_scalar (W : WeierstrassCurve K) (x u w : K)
    (hd : W.Ψ₂Sq.eval x≠0) (ht : W.Ψ₃.eval x≠0)
    (h : doubleNumerator W x=W.Ψ₂Sq.eval x*u)
    (hs : (x-u)^2*(w+x)=sumNumerator W x u) : w=tripleX W x := by
  have hδ := double_displacement W x u hd h
  have hh : (u-x)^2*(w-x)=(central W).eval x*(u-x)+W.Ψ₂Sq.eval x := by
    linear_combination hs+sum_identity W x u
  rw [hδ] at hh
  unfold tripleX
  rw [fourth_eval]
  field_simp at hh ⊢
  linear_combination hh

theorem quadruple_scalar (W : WeierstrassCurve K) (x u w : K)
    (hd : W.Ψ₂Sq.eval x≠0) (hh : W.preΨ₄.eval x≠0)
    (h : doubleNumerator W x=W.Ψ₂Sq.eval x*u)
    (h' : doubleNumerator W u=W.Ψ₂Sq.eval u*w) : w=quadrupleX W x := by
  have he := twice_double_elimination W x u hd h
  rw [h'] at he
  have hden := double_denominator_identity W x u hd h
  have hw : W.Ψ₂Sq.eval x*(W.preΨ₄.eval x)^2*(w-x)= -W.Ψ₃.eval x*(psiFive W).eval x := by
    linear_combination he-W.Ψ₂Sq.eval x*(w-x)*hden
  unfold quadrupleX
  field_simp
  linear_combination hw

theorem multiple_gap (W : WeierstrassCurve K) (x : K)
    (hd : W.Ψ₂Sq.eval x≠0) (ht : W.Ψ₃.eval x≠0) (hh : W.preΨ₄.eval x≠0) :
    W.Ψ₂Sq.eval x*(W.preΨ₄.eval x)^2*(W.Ψ₃.eval x)^2*(quadrupleX W x-tripleX W x)=
      -(psiSeven W).eval x := by
  unfold quadrupleX tripleX
  rw [seven_eval]
  field_simp
  ring

variable [DecidableEq K]

/-- The actual official addition/subtraction map supplies the sum of abscissas. -/
theorem actual_sum_relation (W : WeierstrassCurve.Affine K) {x y u v w z : K}
    (hP : W.Nonsingular x y) (hQ : W.Nonsingular u v) (hR : W.Nonsingular w z)
    (hs : Point.some x y hP+Point.some u v hQ=Point.some w z hR)
    (hd : Point.some x y hP-Point.some u v hQ= -Point.some x y hP) :
    (x-u)^2*(w+x)=sumNumerator W x u := by
  obtain ⟨t,_ht,he⟩ := Point.sym2x_add_sub_eq_addSubMap_sym2x
    (Point.some x y hP) (Point.some u v hQ)
  rw [Point.addSubMap_sym2x_some_some,hs,hd,Point.sym2x_neg_right,Point.sym2x_some_some] at he
  have ht : t=(x-u)^2 := by simpa using congrFun he (2:Fin 3)
  have hh : t*(w+x)=sumNumerator W x u := by simpa [sumNumerator] using congrFun he (1:Fin 3)
  rwa [ht] at hh

#print axioms actual_sum_relation
#print axioms triple_scalar
#print axioms quadruple_scalar
end
end SevenTorsionRankAudit
