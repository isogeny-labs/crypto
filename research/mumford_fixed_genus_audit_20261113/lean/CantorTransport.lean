import ScalarExtension

namespace MumfordFixedGenusAudit
open Polynomial
noncomputable section
variable {F : Type*} [Field F]
local instance : DecidableEq F := Classical.decEq _

theorem base_gcd_monic (P Q : BasePair) : (gcd P.u Q.u).Monic := by
  obtain ⟨i,rfl⟩ := MumfordCantorAudit.point_surjective P
  obtain ⟨j,rfl⟩ := MumfordCantorAudit.point_surjective Q
  change (gcd (MumfordSmoothnessAudit.uPolynomial (MumfordSmoothnessAudit.representative i))
    (MumfordSmoothnessAudit.uPolynomial (MumfordSmoothnessAudit.representative j))).Monic
  rw [MumfordCantorAudit.representative_u_factorization,
    MumfordCantorAudit.representative_u_factorization,MumfordCantorAudit.factorProduct_gcd]
  exact MumfordCantorAudit.factorProduct_monic _

/-- Transport the normalized gcd itself, not merely an associated gcd. -/
theorem normalized_gcd_map (ι : Base →+* F) (P Q : BasePair) :
    gcd (P.u.map ι) (Q.u.map ι)=(gcd P.u Q.u).map ι := by
  obtain ⟨a,b,c,hbez,_⟩ := MumfordCantorAudit.zero_v_composition_witness P Q
  simp only [MumfordCantorAudit.every_v_zero,zero_add,mul_zero,add_zero] at hbez
  have h₁ : gcd (P.u.map ι) (Q.u.map ι)∣(gcd P.u Q.u).map ι := by
    rw [← hbez,Polynomial.map_add,Polynomial.map_mul,Polynomial.map_mul]
    exact dvd_add (dvd_mul_of_dvd_right (gcd_dvd_left _ _) _)
      (dvd_mul_of_dvd_right (gcd_dvd_right _ _) _)
  have h₂ : (gcd P.u Q.u).map ι∣gcd (P.u.map ι) (Q.u.map ι) :=
    dvd_gcd (Polynomial.map_dvd ι (gcd_dvd_left _ _))
      (Polynomial.map_dvd ι (gcd_dvd_right _ _))
  exact (gcd_eq_normalize h₁ h₂).trans ((base_gcd_monic P Q).map ι).normalize_eq_self

/-- Actual normalized gcd composition and its exact quotient/reduction. -/
def ZeroStep (P Q R : Pair F) : Prop := by
  classical
  exact P.v=0 ∧ Q.v=0 ∧ R.v=0 ∧ ∃ w : F[X],w.Monic ∧ w∣curve F ∧
    (gcd P.u Q.u)^2*w=P.u*Q.u ∧
    ((w.natDegree≤2 ∧ R.u=w) ∨ (2<w.natDegree ∧ w*R.u=curve F))

theorem zeroStep_extend (ι : Base →+* F) (P Q R : BasePair)
    (h : MumfordCantorAudit.ZeroCantorStep P Q R) :
    ZeroStep (extend ι P) (extend ι Q) (extend ι R) := by
  classical
  obtain ⟨hP,hQ,hR,w,hw,hwd,hcomp,hout⟩ := h
  refine ⟨extend_v_zero ι P,extend_v_zero ι Q,extend_v_zero ι R,
    w.map ι,hw.map ι,?_,?_,?_⟩
  · rw [← curve_map ι]
    exact Polynomial.map_dvd ι hwd
  · change (gcd (P.u.map ι) (Q.u.map ι))^2*w.map ι=P.u.map ι*Q.u.map ι
    rw [normalized_gcd_map,← Polynomial.map_pow,← Polynomial.map_mul,hcomp,Polynomial.map_mul]
  · rcases hout with ⟨hd,he⟩ | ⟨hd,he⟩
    · left
      exact ⟨by simpa only [natDegree_map_eq_of_injective ι.injective] using hd,
        congrArg (fun p : Base[X] => p.map ι) he⟩
    · right
      refine ⟨by simpa only [natDegree_map_eq_of_injective ι.injective] using hd,?_⟩
      change w.map ι*R.u.map ι=curve F
      rw [← Polynomial.map_mul,he,curve_map]

theorem zeroStep_unique (P Q R S : Pair F) (hR : ZeroStep P Q R)
    (hS : ZeroStep P Q S) : R=S := by
  classical
  obtain ⟨_,_,hvR,w,hw,_,heR,hr⟩ := hR
  obtain ⟨_,_,hvS,z,hz,_,heS,hs⟩ := hS
  have hg : (gcd P.u Q.u)^2≠0 := pow_ne_zero _ (gcd_ne_zero_of_left P.monic_u.ne_zero)
  have hwz : w=z := mul_left_cancel₀ hg (heR.trans heS.symm)
  subst z
  apply Pair.ext
  · rcases hr with ⟨hd,hu⟩ | ⟨hd,hu⟩ <;> rcases hs with ⟨hd',hu'⟩ | ⟨hd',hu'⟩
    · exact hu.trans hu'.symm
    · omega
    · omega
    · exact mul_left_cancel₀ hw.ne_zero (hu.trans hu'.symm)
  · exact hvR.trans hvS.symm

theorem extended_addition_is_Cantor (ι : Base →+* F) (P Q : BasePair) :
    ZeroStep (extend ι P) (extend ι Q) (extend ι (P+Q)) :=
  zeroStep_extend ι P Q (P+Q) (MumfordCantorGroupAudit.addition_is_Cantor P Q)

theorem extended_full_gcd (ι : Base →+* F) (P Q : BasePair) :
    gcd (gcd (extend ι P).u (extend ι Q).u) ((extend ι P).v+(extend ι Q).v)=
      gcd (extend ι P).u (extend ι Q).u := by
  classical
  rw [extend_v_zero,extend_v_zero,zero_add,gcd_zero_right]
  exact normalize_gcd _ _

theorem extended_composition_numerator (ι : Base →+* F) (P Q : BasePair) :
    ∃ a b c : F[X],
      a*(extend ι P).u+b*(extend ι Q).u+c*((extend ι P).v+(extend ι Q).v)=
        gcd (extend ι P).u (extend ι Q).u ∧
      a*(extend ι P).u*(extend ι Q).v+b*(extend ι Q).u*(extend ι P).v+
        c*((extend ι P).v*(extend ι Q).v+curve F)=0 := by
  classical
  obtain ⟨a,b,c,hbez,hnum⟩ := MumfordCantorAudit.zero_v_composition_witness P Q
  refine ⟨a.map ι,b.map ι,c.map ι,?_,?_⟩
  · have h := congrArg (fun p : Base[X] => p.map ι) hbez
    simpa only [Polynomial.map_add,Polynomial.map_mul,extend,normalized_gcd_map] using h
  · have h := congrArg (fun p : Base[X] => p.map ι) hnum
    simpa only [Polynomial.map_add,Polynomial.map_mul,Polynomial.map_zero,curve_map,extend] using h

#print axioms extended_addition_is_Cantor
#print axioms extended_composition_numerator
end
end MumfordFixedGenusAudit
