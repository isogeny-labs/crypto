module
public import NormQuotientMoment
public import NormCRTReciprocity
public section
namespace NormPairingRadical
open Polynomial
variable {K : Type*} [Field K]

/-- The quotient pairing has exactly the expected polynomial ideal as annihilator. -/
theorem annihilatorIff (F U P : Polynomial K) (hf : F.Monic)
    (hpos : 0<F.natDegree) (hu : IsCoprime F U) :
    (∀ Q : Polynomial K, Q.natDegree<F.natDegree →
      ((P*U*Q) %ₘ F).coeff (F.natDegree-1)=0) ↔ F ∣ P := by
  constructor
  · intro hv
    have hfne : F≠1 := by
      intro h
      rw [h,natDegree_one] at hpos
      omega
    let R := P %ₘ F
    have hr : R=0 := by
      by_contra hn
      have hd : R.natDegree<F.natDegree := natDegree_modByMonic_lt P hf hfne
      obtain ⟨Q,hq,hnonzero⟩ :=
        NormQuotientMoment.twistedCoefficientDetects F U R hf hu hn hd
      have hdiff : F ∣ P-R := by
        refine ⟨P /ₘ F, ?_⟩
        dsimp [R]
        rw [modByMonic_eq_sub_mul_div]
        ring
      have hprod : F ∣ P*U*Q-R*U*Q := by
        obtain ⟨T,ht⟩ := hdiff
        refine ⟨T*U*Q, ?_⟩
        rw [← sub_mul,← sub_mul,ht]
        ring
      have heq := modByMonic_eq_of_dvd_sub hf hprod
      have hz := hv Q hq
      rw [heq] at hz
      exact hnonzero hz
    exact (modByMonic_eq_zero_iff_dvd hf).mp hr
  · intro hd Q _
    have hprod : F ∣ P*U*Q := dvd_mul_of_dvd_left (dvd_mul_of_dvd_left hd U) Q
    rw [(modByMonic_eq_zero_iff_dvd hf).mpr hprod,coeff_zero]

/-- At degree at most the modulus degree, divisibility is exactly scalar span. -/
theorem boundedDivisibility (F P : Polynomial K) (hf : F.Monic)
    (hp : P.natDegree≤F.natDegree) :
    F ∣ P ↔ ∃ c : K, P=C c*F := by
  constructor
  · rintro ⟨Q,hq⟩
    by_cases hz : Q=0
    · exact ⟨0,by simp [hq,hz]⟩
    · have hd := natDegree_mul hf.ne_zero hz
      have hqd : Q.natDegree=0 := by rw [hq,hd] at hp; omega
      refine ⟨Q.coeff 0, ?_⟩
      calc
        P=F*Q := hq
        _=F*C (Q.coeff 0) := congrArg (fun T : Polynomial K => F*T)
          (eq_C_of_natDegree_eq_zero hqd)
        _=C (Q.coeff 0)*F := mul_comm _ _
  · rintro ⟨c,hc⟩
    exact ⟨C c,by rw [hc,mul_comm]⟩

/-- The bounded unit-twisted quotient pairing has radical precisely span(F). -/
theorem boundedRadical (F U P : Polynomial K) (hf : F.Monic)
    (hpos : 0<F.natDegree) (hu : IsCoprime F U)
    (hp : P.natDegree≤F.natDegree) :
    (∀ Q : Polynomial K, Q.natDegree<F.natDegree →
      ((P*U*Q) %ₘ F).coeff (F.natDegree-1)=0) ↔
      ∃ c : K, P=C c*F := by
  rw [annihilatorIff F U P hf hpos hu,boundedDivisibility F P hf hp]

/-- CRT identifies the lower norm pairing's annihilator without assuming its rank. -/
theorem lowerAnnihilator (F H f P U V : Polynomial K)
    (hf : F.Monic) (hh : H.Monic) (hhp : 0<H.natDegree)
    (hdeg : H.natDegree<F.natDegree)
    (hbez : U*F+V*(f*H)=1) (hp : P.natDegree≤H.natDegree) :
    (∀ Q : Polynomial K, Q.natDegree<H.natDegree →
      ((f*(P*Q)*V) %ₘ F).coeff (F.natDegree-1)=0) ↔ H ∣ P := by
  have hu : IsCoprime H U := ⟨V*f,F,by linear_combination hbez⟩
  have hrec (Q : Polynomial K) (hq : Q.natDegree<H.natDegree) :=
    NormCRTReciprocity.weightedReciprocity F H f (P*Q) U V hf hh
      (by omega) hhp hbez (by have hd : (P*Q).natDegree≤P.natDegree+Q.natDegree := natDegree_mul_le; omega)
  rw [← annihilatorIff H U P hh hhp hu]
  constructor
  · intro hv Q hq
    have hr := hrec Q hq
    rw [hv Q hq,zero_add] at hr
    simpa only [mul_assoc,mul_comm,mul_left_comm] using hr
  · intro hv Q hq
    have hr := hrec Q hq
    have hz : (((P*Q)*U) %ₘ H).coeff (H.natDegree-1)=0 := by
      simpa only [mul_assoc,mul_comm,mul_left_comm] using hv Q hq
    rw [hz,add_zero] at hr
    exact hr

/-- In the full-degree lower chart, the polynomial radical is exactly span(H). -/
theorem lowerBoundedRadical (F H f P U V : Polynomial K)
    (hf : F.Monic) (hh : H.Monic) (hhp : 0<H.natDegree)
    (hdeg : H.natDegree<F.natDegree)
    (hbez : U*F+V*(f*H)=1) (hp : P.natDegree≤H.natDegree) :
    (∀ Q : Polynomial K, Q.natDegree<H.natDegree →
      ((f*(P*Q)*V) %ₘ F).coeff (F.natDegree-1)=0) ↔
      ∃ c : K, P=C c*H := by
  rw [lowerAnnihilator F H f P U V hf hh hhp hdeg hbez hp,
    boundedDivisibility H P hh hp]

/-- Including tests of the modulus degree leaves the same one-dimensional radical. -/
theorem fullBoundedRadical (F U P : Polynomial K) (hf : F.Monic)
    (hpos : 0<F.natDegree) (hu : IsCoprime F U)
    (hp : P.natDegree≤F.natDegree) :
    (∀ Q : Polynomial K, Q.natDegree≤F.natDegree →
      ((P*U*Q) %ₘ F).coeff (F.natDegree-1)=0) ↔
      ∃ c : K, P=C c*F := by
  constructor
  · intro hv
    exact (boundedRadical F U P hf hpos hu hp).mp (fun Q hq => hv Q hq.le)
  · intro hs Q _
    have hd := (boundedDivisibility F P hf hp).mpr hs
    have hprod : F ∣ P*U*Q := dvd_mul_of_dvd_left (dvd_mul_of_dvd_left hd U) Q
    rw [(modByMonic_eq_zero_iff_dvd hf).mpr hprod,coeff_zero]

/-- The lower block with its full test space has exactly span(H) as radical. -/
theorem lowerFullRadical (F H f P U V : Polynomial K)
    (hf : F.Monic) (hh : H.Monic) (hhp : 0<H.natDegree)
    (hgap : H.natDegree+1<F.natDegree)
    (hbez : U*F+V*(f*H)=1) (hp : P.natDegree≤H.natDegree) :
    (∀ Q : Polynomial K, Q.natDegree≤H.natDegree →
      ((f*(P*Q)*V) %ₘ F).coeff (F.natDegree-1)=0) ↔
      ∃ c : K, P=C c*H := by
  constructor
  · intro hv
    exact (lowerBoundedRadical F H f P U V hf hh hhp (by omega) hbez hp).mp
      (fun Q hq => hv Q hq.le)
  · intro hs Q hq
    have hd := (boundedDivisibility H P hh hp).mpr hs
    have hprod : H ∣ P*Q*U := dvd_mul_of_dvd_left (dvd_mul_of_dvd_left hd Q) U
    have hz := (modByMonic_eq_zero_iff_dvd hh).mpr hprod
    have hr := NormCRTReciprocity.weightedReciprocity F H f (P*Q) U V hf hh
      (by omega) hhp hbez
      (by have hd : (P*Q).natDegree≤P.natDegree+Q.natDegree := natDegree_mul_le; omega)
    rw [hz,coeff_zero,add_zero] at hr
    exact hr

/-- The complete polynomial block pairing has exactly the two witness directions.
This does not assert matrix-pencil membership or compute the supplied Bezout data. -/
theorem normBlockRadical (F H f A B U V : Polynomial K)
    (hf : F.Monic) (hh : H.Monic) (hhp : 0<H.natDegree)
    (hgap : H.natDegree+1<F.natDegree)
    (hbez : U*F+V*(f*H)=1)
    (ha : A.natDegree≤F.natDegree) (hb : B.natDegree≤H.natDegree) :
    (∀ Q T : Polynomial K, Q.natDegree≤F.natDegree →
      T.natDegree≤H.natDegree →
      (((A*Q-f*(B*T))*V) %ₘ F).coeff (F.natDegree-1)=0) ↔
      ∃ c d : K, A=C c*F ∧ B=C d*H := by
  have hv : IsCoprime F V := ⟨U,f*H,by linear_combination hbez⟩
  have heq (Q T : Polynomial K) :
      (((A*Q-f*(B*T))*V) %ₘ F).coeff (F.natDegree-1)=
      ((A*V*Q) %ₘ F).coeff (F.natDegree-1)-
        ((f*(B*T)*V) %ₘ F).coeff (F.natDegree-1) := by
    rw [sub_mul,sub_modByMonic,coeff_sub]
    simp only [mul_comm,mul_left_comm]
  constructor
  · intro hz
    have hau : ∀ Q : Polynomial K, Q.natDegree≤F.natDegree →
        ((A*V*Q) %ₘ F).coeff (F.natDegree-1)=0 := by
      intro Q hq
      have h := hz Q 0 hq (by simp)
      rw [heq] at h
      simpa only [mul_zero,zero_mul,zero_modByMonic,coeff_zero,sub_zero] using h
    have hbl : ∀ T : Polynomial K, T.natDegree≤H.natDegree →
        ((f*(B*T)*V) %ₘ F).coeff (F.natDegree-1)=0 := by
      intro T ht
      have h := hz 0 T (by simp) ht
      rw [heq] at h
      simpa only [mul_zero,zero_mul,zero_modByMonic,coeff_zero,zero_sub,neg_eq_zero] using h
    obtain ⟨c,hc⟩ := (fullBoundedRadical F V A hf (by omega) hv ha).mp hau
    obtain ⟨d,hd⟩ := (lowerFullRadical F H f B U V hf hh hhp hgap hbez hb).mp hbl
    exact ⟨c,d,hc,hd⟩
  · rintro ⟨c,d,hc,hd⟩ Q T hq ht
    have hau := (fullBoundedRadical F V A hf (by omega) hv ha).mpr ⟨c,hc⟩ Q hq
    have hbl := (lowerFullRadical F H f B U V hf hh hhp hgap hbez hb).mpr ⟨d,hd⟩ T ht
    rw [heq,hau,hbl,sub_self]

/-- A branch hyperplane introduces no new left-radical vectors when it is
nonzero on a supplied right-radical vector. Membership of w in the branch,
when desired, is a separate condition. -/
theorem branchTestsDetectFull {W : Type*} [AddCommGroup W] [Module K W]
    (pairing : W →ₗ[K] W →ₗ[K] K) (branch : W →ₗ[K] K) (k w : W)
    (hk : ∀ v : W, pairing v k=0) (hbranch : branch k≠0) :
    (∀ v : W, branch v=0 → pairing w v=0) ↔
      (∀ v : W, pairing w v=0) := by
  constructor
  · intro hv v
    have hadjust : branch (v-(branch v/branch k) • k)=0 := by
      simp only [map_sub,map_smul,smul_eq_mul]
      rw [div_mul_cancel₀ _ hbranch,sub_self]
    have h := hv _ hadjust
    simpa only [map_sub,map_smul,hk w,smul_zero,sub_zero] using h
  · intro hv v _
    exact hv v
end NormPairingRadical
#print axioms NormPairingRadical.annihilatorIff
#print axioms NormPairingRadical.boundedDivisibility
#print axioms NormPairingRadical.boundedRadical
#print axioms NormPairingRadical.lowerAnnihilator
#print axioms NormPairingRadical.lowerBoundedRadical
#print axioms NormPairingRadical.fullBoundedRadical
#print axioms NormPairingRadical.lowerFullRadical
#print axioms NormPairingRadical.normBlockRadical
#print axioms NormPairingRadical.branchTestsDetectFull
