import MumfordStollCycleGroup

/-! General one-step genus-two Cantor certificates with a normalization unit.
The Bezout coefficients are fixed algorithm choices. Output uniqueness is
proved from actual polynomial equations, before any finite group transfer.
No geometric Picard identification is assumed. -/
namespace MumfordStollGeneralAudit
noncomputable section
open Polynomial MumfordStollAudit MumfordStollCycleGroupAudit
set_option backward.isDefEq.respectTransparency false

/-- Full polynomial run, with fixed Bezout coefficients. The reduction branch
allows the nontrivial normalization unit and exact final remainder. -/
def GeneralCantorRun (a b c : K[X]) (P Q R : ReducedPair) : Prop :=
  ∃ d U V q A B C F : K[X],
    d.Monic ∧ U.Monic ∧ d*A=P.u ∧ d*B=Q.u ∧ d*C=P.v+Q.v ∧
    a*P.u+b*Q.u+c*(P.v+Q.v)=d ∧ d^2*U=P.u*Q.u ∧
    a*P.u*Q.v+b*Q.u*P.v+c*(P.v*Q.v+curvePolynomial)=d*(V+U*q) ∧
    V.degree<U.degree ∧ U*F=curvePolynomial-V^2 ∧
    ((U.natDegree≤2 ∧ R.u=U ∧ R.v=V) ∨
      (2<U.natDegree ∧ ∃ (lam : K) (qR : K[X]), lam≠0 ∧
        U*R.u=Polynomial.C lam*(curvePolynomial-V^2) ∧ -V=R.v+R.u*qR))

theorem bounded_remainder_unique (U V W q r : K[X]) (hU : U.Monic)
    (hV : V.degree<U.degree) (hW : W.degree<U.degree)
    (he : V+U*q=W+U*r) : V=W := by
  have hd : U∣V-W := ⟨r-q,by linear_combination he⟩
  have hm := Polynomial.modByMonic_eq_of_dvd_sub hU hd
  rw [(Polynomial.modByMonic_eq_self_iff hU).mpr hV,
    (Polynomial.modByMonic_eq_self_iff hU).mpr hW] at hm
  exact hm

theorem output_remainder (R : ReducedPair) (V qR : K[X])
    (he : -V=R.v+R.u*qR) : R.v=(-V)%ₘR.u :=
  ((Polynomial.div_modByMonic_unique qR R.v R.monic_u ⟨he.symm,R.degree_v⟩).2).symm

theorem normalized_reduction_unique (U f : K[X]) (R S : ReducedPair)
    (hU : U.Monic) (lam mu : K) (hlam : lam≠0)
    (hR : U*R.u=Polynomial.C lam*f) (hS : U*S.u=Polynomial.C mu*f) : R.u=S.u := by
  have hc : Polynomial.C mu*R.u=Polynomial.C lam*S.u := by
    apply mul_left_cancel₀ hU.ne_zero
    calc
      U*(Polynomial.C mu*R.u)=Polynomial.C mu*(U*R.u) := by ring
      _=Polynomial.C lam*(U*S.u) := by rw [hR,hS]; ring
      _=U*(Polynomial.C lam*S.u) := by ring
  have he : mu=lam := by
    have hh := congrArg Polynomial.leadingCoeff hc
    have hlR : R.u.leadingCoeff=1 := R.monic_u
    have hlS : S.u.leadingCoeff=1 := S.monic_u
    simpa only [Polynomial.leadingCoeff_mul,Polynomial.leadingCoeff_C,hlR,hlS,mul_one] using hh
  rw [he] at hc
  exact mul_left_cancel₀ (Polynomial.C_ne_zero.mpr hlam) hc

theorem run_output_unique (a b c : K[X]) (P Q R S : ReducedPair)
    (hR : GeneralCantorRun a b c P Q R) (hS : GeneralCantorRun a b c P Q S) : R=S := by
  obtain ⟨d,U,V,q,A,B,C,F,hd,hU,hA,hB,hC,hbez,hcomp,hnum,hV,hF,hout⟩ := hR
  obtain ⟨e,W,Z,r,A',B',C',F',he,hW,hA',hB',hC',hbez',hcomp',hnum',hZ,hF',hout'⟩ := hS
  have hde : d=e := (normalized_full_gcd P Q d a b c A B C hd hA hB hC hbez).symm.trans
    (normalized_full_gcd P Q e a b c A' B' C' he hA' hB' hC' hbez')
  cases hde
  have hd0 : d≠0 := by
    intro hz
    have hh : P.u*Q.u=0 := by rw [←hcomp,hz,zero_pow (by decide : (2:ℕ)≠0),zero_mul]
    exact mul_ne_zero P.monic_u.ne_zero Q.monic_u.ne_zero hh
  have hUW : U=W := mul_left_cancel₀ (pow_ne_zero 2 hd0) (hcomp.trans hcomp'.symm)
  cases hUW
  have hVZ : V=Z := bounded_remainder_unique U V Z q r hU hV hZ
    (mul_left_cancel₀ hd0 (hnum.symm.trans hnum'))
  cases hVZ
  apply ReducedPair.ext
  · rcases hout with ⟨hdeg,hu,hv⟩ | ⟨hdeg,lam,qR,hlam,hu,hv⟩ <;>
      rcases hout' with ⟨hdeg',hu',hv'⟩ | ⟨hdeg',mu,qS,hmu,hu',hv'⟩
    · exact hu.trans hu'.symm
    · omega
    · omega
    · exact normalized_reduction_unique U (curvePolynomial-V^2) R S hU lam mu hlam hu hu'
  · rcases hout with ⟨hdeg,hu,hv⟩ | ⟨hdeg,lam,qR,hlam,hu,hv⟩ <;>
      rcases hout' with ⟨hdeg',hu',hv'⟩ | ⟨hdeg',mu,qS,hmu,hu',hv'⟩
    · exact hv.trans hv'.symm
    · omega
    · omega
    · have huu := normalized_reduction_unique U (curvePolynomial-V^2) R S hU lam mu hlam hu hu'
      rw [output_remainder R V qR hv,output_remainder S V qS hv',huu]

#print axioms run_output_unique
end
end MumfordStollGeneralAudit
