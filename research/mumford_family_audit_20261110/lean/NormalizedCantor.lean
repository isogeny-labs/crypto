import ReducedPairs

/-! Genus-parametric actual polynomial certificates for composition followed
by zero or one normalized Cantor reduction. Not a complete arbitrary-genus
addition algorithm or a geometric Jacobian identification. -/
namespace MumfordFamilyCantorAudit
noncomputable section
open Polynomial MumfordFamilyPairsAudit
variable {g : ℕ} {f : K[X]}

def OneStepRun (a b c : K[X]) (P Q R : ReducedPair g f) : Prop :=
  ∃ d U V q A B C F : K[X],
    d.Monic ∧ U.Monic ∧ d*A=P.u ∧ d*B=Q.u ∧ d*C=P.v+Q.v ∧
    a*P.u+b*Q.u+c*(P.v+Q.v)=d ∧ d^2*U=P.u*Q.u ∧
    a*P.u*Q.v+b*Q.u*P.v+c*(P.v*Q.v+f)=d*(V+U*q) ∧
    V.degree < U.degree ∧ U*F=f-V^2 ∧
    ((U.natDegree≤g ∧ R.u=U ∧ R.v=V) ∨
      (g < U.natDegree ∧ ∃ (lam : K) (qR : K[X]), lam≠0 ∧
        U*R.u=Polynomial.C lam*(f-V^2) ∧ -V=R.v+R.u*qR))

theorem normalized_full_gcd (P Q : ReducedPair g f) (d a b c A B C : K[X])
    (hd : d.Monic) (hA : d*A=P.u) (hB : d*B=Q.u) (hC : d*C=P.v+Q.v)
    (hbez : a*P.u+b*Q.u+c*(P.v+Q.v)=d) :
    gcd (gcd P.u Q.u) (P.v+Q.v)=d := by
  let h := gcd (gcd P.u Q.u) (P.v+Q.v)
  have hhP : h∣P.u := dvd_trans (gcd_dvd_left _ _) (gcd_dvd_left _ _)
  have hhQ : h∣Q.u := dvd_trans (gcd_dvd_left _ _) (gcd_dvd_right _ _)
  have hhV : h∣P.v+Q.v := gcd_dvd_right _ _
  have hhd : h∣d := by
    rw [←hbez]
    exact dvd_add (dvd_add (dvd_mul_of_dvd_right hhP a)
      (dvd_mul_of_dvd_right hhQ b)) (dvd_mul_of_dvd_right hhV c)
  have hdh : d∣h := dvd_gcd (dvd_gcd ⟨A,hA.symm⟩ ⟨B,hB.symm⟩) ⟨C,hC.symm⟩
  exact (gcd_eq_normalize hhd hdh).trans hd.normalize_eq_self

theorem bounded_remainder_unique (U V W q r : K[X]) (hU : U.Monic)
    (hV : V.degree < U.degree) (hW : W.degree < U.degree)
    (he : V+U*q=W+U*r) : V=W := by
  have hd : U∣V-W := ⟨r-q,by linear_combination he⟩
  have hm := Polynomial.modByMonic_eq_of_dvd_sub hU hd
  rw [(Polynomial.modByMonic_eq_self_iff hU).mpr hV,
    (Polynomial.modByMonic_eq_self_iff hU).mpr hW] at hm
  exact hm

theorem output_remainder (R : ReducedPair g f) (V qR : K[X])
    (he : -V=R.v+R.u*qR) : R.v=(-V)%ₘR.u :=
  ((Polynomial.div_modByMonic_unique qR R.v R.monic_u ⟨he.symm,R.degree_v⟩).2).symm

theorem normalized_reduction_unique (U F : K[X]) (R S : ReducedPair g f)
    (hU : U.Monic) (lam mu : K) (hlam : lam≠0)
    (hR : U*R.u=Polynomial.C lam*F) (hS : U*S.u=Polynomial.C mu*F) : R.u=S.u := by
  have hc : Polynomial.C mu*R.u=Polynomial.C lam*S.u := by
    apply mul_left_cancel₀ hU.ne_zero
    calc
      U*(Polynomial.C mu*R.u)=Polynomial.C mu*(U*R.u) := by ring
      _=Polynomial.C lam*(U*S.u) := by rw [hR,hS];ring
      _=U*(Polynomial.C lam*S.u) := by ring
  have he : mu=lam := by
    have hh:=congrArg Polynomial.leadingCoeff hc
    have hR1 : R.u.leadingCoeff=1 := R.monic_u
    have hS1 : S.u.leadingCoeff=1 := S.monic_u
    simpa only [Polynomial.leadingCoeff_mul,Polynomial.leadingCoeff_C,hR1,hS1,mul_one] using hh
  rw [he] at hc
  exact mul_left_cancel₀ (Polynomial.C_ne_zero.mpr hlam) hc

theorem run_output_unique (a b c : K[X]) (P Q R S : ReducedPair g f)
    (hR : OneStepRun a b c P Q R) (hS : OneStepRun a b c P Q S) : R=S := by
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
  apply pair_ext
  · rcases hout with ⟨hdeg,hu,hv⟩ | ⟨hdeg,lam,qR,hlam,hu,hv⟩ <;>
      rcases hout' with ⟨hdeg',hu',hv'⟩ | ⟨hdeg',mu,qS,hmu,hu',hv'⟩
    · exact hu.trans hu'.symm
    · omega
    · omega
    · exact normalized_reduction_unique U (f-V^2) R S hU lam mu hlam hu hu'
  · rcases hout with ⟨hdeg,hu,hv⟩ | ⟨hdeg,lam,qR,hlam,hu,hv⟩ <;>
      rcases hout' with ⟨hdeg',hu',hv'⟩ | ⟨hdeg',mu,qS,hmu,hu',hv'⟩
    · exact hv.trans hv'.symm
    · omega
    · omega
    · have huu:=normalized_reduction_unique U (f-V^2) R S hU lam mu hlam hu hu'
      rw [output_remainder R V qR hv,output_remainder S V qS hv',huu]

theorem run_gcd_certificate (a b c : K[X]) (P Q R : ReducedPair g f)
    (h : OneStepRun a b c P Q R) :
    ∃ d U : K[X],d=gcd (gcd P.u Q.u) (P.v+Q.v) ∧
      d.Monic ∧ U.Monic ∧ d^2*U=P.u*Q.u := by
  obtain ⟨d,U,V,q,A,B,C,F,hd,hU,hA,hB,hC,hbez,hcomp,_⟩:=h
  exact ⟨d,U,(normalized_full_gcd P Q d a b c A B C hd hA hB hC hbez).symm,
    hd,hU,hcomp⟩

#print axioms run_output_unique
#print axioms run_gcd_certificate
end
end MumfordFamilyCantorAudit
