module
public import NormBranchCoordinates
import all NormBranchCoordinates
import all NormMomentMatrix
import all NormLift
import all NormDualPencil
import all RankOneExtraction
import all Mathlib.Data.Matrix.Mul
public section
namespace NormCertificateRecovery
open Polynomial Matrix
variable {K : Type*} [Field K] {n : ℕ}

theorem quadraticDot (Q : Matrix (Fin n) (Fin n) K) (w : Fin n → K) :
    NormDualPencil.quadratic Q w=dotProduct w (Q.mulVec w) := by
  simp only [NormDualPencil.quadratic,Matrix.mulVec,dotProduct,Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro i _
  apply Finset.sum_congr rfl
  intro j _
  ring

theorem quadraticScale (Q : Matrix (Fin n) (Fin n) K) (t : K) (w : Fin n → K) :
    NormDualPencil.quadratic Q (t • w)=t^2*NormDualPencil.quadratic Q w := by
  simp only [NormDualPencil.quadratic,Pi.smul_apply,smul_eq_mul,Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro i _
  apply Finset.sum_congr rfl
  intro j _
  ring

theorem adjugateColumnKernel (S : Matrix (Fin n) (Fin n) K) (hd : S.det=0) (k : Fin n) :
    S.mulVec (fun i => S.adjugate i k)=0 := by
  have hm : S*S.adjugate=0 := by rw [Matrix.mul_adjugate,hd,zero_smul]
  funext i
  change (∑ j, S i j*S.adjugate j k)=0
  simpa only [Matrix.mul_apply,Matrix.zero_apply] using congrArg (fun M => M i k) hm

theorem adjugateColumnQuadratic (S Q : Matrix (Fin n) (Fin n) K)
    (hs : S.IsSymm) (hd : S.det=0) (hr : S.rank+1=n)
    (k : Fin n) (hk : S.adjugate k k≠0)
    (hp : NormDualPencil.pairing Q S.adjugate=0) :
    NormDualPencil.quadratic Q (fun i => S.adjugate i k)=0 := by
  have hm := NormDualPencil.minorsOfRank S.adjugate
    (NormDualPencil.adjugateRank S hd (by simpa only [Fintype.card_fin] using hr))
  have hsym : ∀ i j, S.adjugate i j=S.adjugate j i :=
    fun i j => (hs.adjugate.apply i j).symm
  have hf : S.adjugate=(fun i j => (S.adjugate k k)⁻¹*(S.adjugate i k*S.adjugate j k)) := by
    funext i j
    exact RankOneExtraction.pivotFactor S.adjugate hsym hm k hk i j
  have heq := NormDualPencil.scaledPairing Q (S.adjugate k k)⁻¹
    (fun i => S.adjugate i k)
  rw [← hf,hp] at heq
  exact (mul_eq_zero.mp heq.symm).resolve_left (inv_ne_zero hk)

/-- Cofactor equations force the supplied exact radical direction to satisfy Q. -/
theorem directionQuadratic (S Q : Matrix (Fin n) (Fin n) K)
    (hs : S.IsSymm) (hd : S.det=0) (hr : S.rank+1=n) (u : Fin n → K)
    (hline : ∀ x, (∀ z, dotProduct x (S.mulVec z)=0) ↔ ∃ t : K, x=t • u)
    (k : Fin n) (hk : S.adjugate k k≠0)
    (hp : NormDualPencil.pairing Q S.adjugate=0) :
    NormDualPencil.quadratic Q u=0 := by
  have hkernel := adjugateColumnKernel S hd k
  have hrad : ∀ z, dotProduct (fun i => S.adjugate i k) (S.mulVec z)=0 :=
    (NormBranchCoordinates.leftRadicalIff S _).mpr (by rw [hs.eq]; exact hkernel)
  obtain ⟨t,ht⟩ := (hline _).mp hrad
  have ht0 : t≠0 := by
    intro hz
    have hc := congrFun ht k
    rw [hz,zero_smul] at hc
    exact hk hc
  have hq := adjugateColumnQuadratic S Q hs hd hr k hk hp
  rw [ht,quadraticScale] at hq
  exact (mul_eq_zero.mp hq).resolve_left (pow_ne_zero 2 ht0)
theorem branchBetaNonzero (F H : Polynomial K) (q y alpha beta : K)
    (hFq : F.eval q≠0) (ha : alpha≠0)
    (hb : (C alpha*F).eval q-y*(C beta*H).eval q=0) : beta≠0 := by
  intro hz
  simp only [hz,map_zero,zero_mul,eval_zero,mul_zero,sub_zero,eval_mul,eval_C] at hb
  exact (mul_ne_zero ha hFq) hb

/-- Nonzero norm follows from the actual inverse and proper-degree branch chart. -/
theorem chartNormNonzero (F H f U V : Polynomial K) (q y alpha beta : K)
    (hf : F.Monic) (hh : H.Monic) (hgap : H.natDegree+1<F.natDegree)
    (hbez : U*F+V*(f*H)=1) (hFq : F.eval q≠0) (ha : alpha≠0)
    (hb : (C alpha*F).eval q-y*(C beta*H).eval q=0) :
    (C alpha*F)^2-f*(C beta*H)^2≠0 := by
  have hbeta := branchBetaNonzero F H q y alpha beta hFq ha hb
  intro hz
  have heq := sub_eq_zero.mp hz
  have hB : f*(C beta*H)^2=C (beta^2)*(f*H^2) := by
    simp only [map_pow]
    ring
  have hinv : C ((beta^2)⁻¹)*C (beta^2)=(1 : Polynomial K) := by
    rw [← map_mul,inv_mul_cancel₀ (pow_ne_zero 2 hbeta),map_one]
  have hfH : f*H^2=C ((beta^2)⁻¹)*(C alpha*F)^2 := by
    calc
      f*H^2=(C ((beta^2)⁻¹)*C (beta^2))*(f*H^2) := by rw [hinv,one_mul]
      _=C ((beta^2)⁻¹)*(f*(C beta*H)^2) := by rw [hB]; ring
      _=C ((beta^2)⁻¹)*(C alpha*F)^2 := by rw [heq]
  have hdiv : F ∣ H := by
    refine ⟨U*H+V*C ((beta^2)⁻¹)*(C alpha)^2*F,?_⟩
    calc
      H=(U*F+V*(f*H))*H := by rw [hbez,one_mul]
      _=F*(U*H)+V*(f*H^2) := by ring
      _=F*(U*H+V*C ((beta^2)⁻¹)*(C alpha)^2*F) := by rw [hfH]; ring
  have hle := hf.natDegree_le_of_dvd hh.ne_zero hdiv
  omega

theorem chartNormDegree (F H f : Polynomial K) (alpha beta : K) (d : ℕ)
    (hA : 2*F.natDegree≤d) (hB : f.natDegree+2*H.natDegree≤d) :
    ((C alpha*F)^2-f*(C beta*H)^2).natDegree≤d := by
  have ha := natDegree_pow_le_of_le 2 (natDegree_C_mul_le alpha F)
  have hb := natDegree_pow_le_of_le 2 (natDegree_C_mul_le beta H)
  have hm : (f*(C beta*H)^2).natDegree≤f.natDegree+((C beta*H)^2).natDegree :=
    natDegree_mul_le
  exact (natDegree_sub_le _ _).trans (max_le (by omega) (by omega))

/-- Finite residual equations and a degree bound give the full polynomial identity. -/
theorem reconstructPolynomial (N G : Polynomial K) (hg : G.Monic)
    (hdeg : N.natDegree≤G.natDegree)
    (hres : ∀ j : Fin G.natDegree,
      N.coeff j.val-N.coeff G.natDegree*G.coeff j.val=0) :
    N=C (N.coeff G.natDegree)*G := by
  ext j
  rw [coeff_C_mul]
  by_cases hj : j<G.natDegree
  · exact sub_eq_zero.mp (hres ⟨j,hj⟩)
  · by_cases heq : j=G.natDegree
    · subst j
      rw [hg.coeff_natDegree,mul_one]
    · have hgt : G.natDegree<j := by omega
      rw [coeff_eq_zero_of_natDegree_lt (hdeg.trans_lt hgt),
        coeff_eq_zero_of_natDegree_lt hgt,mul_zero]

theorem reconstructedScalarNonzero (N G : Polynomial K) (hn : N≠0)
    (heq : N=C (N.coeff G.natDegree)*G) : N.coeff G.natDegree≠0 := by
  intro hz
  apply hn
  rw [heq,hz,map_zero,zero_mul]
theorem reducedQuadratic (ell : Fin (n+1) → K)
    (Q : Matrix (Fin (n+1)) (Fin (n+1)) K) (x : Fin n → K) :
    NormDualPencil.quadratic (NormBranchCoordinates.reducedMatrix ell Q) x=
      NormDualPencil.quadratic Q (NormBranchCoordinates.insert ell x) := by
  rw [quadraticDot,NormBranchCoordinates.reducedBilinear,← quadraticDot]

/-- Actual cofactor equations annihilate each actual norm residual coefficient. -/
theorem actualNormResidual (F H f G U V : Polynomial K) (q y alpha beta : K)
    (hf : F.Monic) (hh : H.Monic) (hhp : 0<H.natDegree)
    (hgap : H.natDegree+1<F.natDegree) (hbez : U*F+V*(f*H)=1)
    (hFq : F.eval q≠0) (ha : alpha≠0)
    (hb : (C alpha*F).eval q-y*(C beta*H).eval q=0)
    (w0 : Fin ((F.natDegree+1)+(H.natDegree+1)) → K)
    (hA : NormLift.assemble (NormMomentMatrix.upperFamily F.natDegree H.natDegree) w0=C alpha*F)
    (hB : NormLift.assemble (NormMomentMatrix.lowerFamily F.natDegree H.natDegree) w0=C beta*H)
    (j : Fin G.natDegree)
    (hp : NormDualPencil.pairing
      (NormBranchCoordinates.actualReducedMatrix (NormMomentMatrix.residualFunctional G j.val) f
        F.natDegree H.natDegree q y)
      (NormBranchCoordinates.actualReducedMatrix (NormPencilFunctional.moment F V) f
        F.natDegree H.natDegree q y).adjugate=0) :
    ((C alpha*F)^2-f*(C beta*H)^2).coeff j.val-
      ((C alpha*F)^2-f*(C beta*H)^2).coeff G.natDegree*G.coeff j.val=0 := by
  let S := NormBranchCoordinates.actualReducedMatrix (NormPencilFunctional.moment F V) f
    F.natDegree H.natDegree q y
  let Q := NormBranchCoordinates.actualReducedMatrix (NormMomentMatrix.residualFunctional G j.val) f
    F.natDegree H.natDegree q y
  have hs : S.IsSymm := by
    dsimp only [S,NormBranchCoordinates.actualReducedMatrix]
    exact NormBranchCoordinates.reducedSymmetric _ _ (NormMomentMatrix.momentSymmetric _ _ _ _)
  have hd : S.det=0 := NormBranchCoordinates.actualReducedSingular F H f U V q y alpha beta
    hf hh hhp hgap hbez hFq ha hb
  have hr : S.rank+1=(F.natDegree+1)+H.natDegree :=
    NormBranchCoordinates.actualReducedCorank F H f U V q y alpha beta hf hh hhp hgap hbez hFq ha hb
  have hline : ∀ x, (∀ z, dotProduct x (S.mulVec z)=0) ↔
      ∃ t : K, x=t • NormBranchCoordinates.delete w0 := by
    intro x
    exact NormBranchCoordinates.actualReducedRadical F H f U V q y alpha beta
      hf hh hhp hgap hbez hFq ha hb w0 hA hB x
  obtain ⟨k,hk⟩ := NormBranchCoordinates.actualReducedCofactor F H f U V q y alpha beta
    hf hh hhp hgap hbez hFq ha hb
  have hq := directionQuadratic S Q hs hd hr (NormBranchCoordinates.delete w0) hline k hk hp
  have hbranch : NormBranchCoordinates.branchForm
      (NormBranchCoordinates.actualCoefficients F.natDegree H.natDegree q y) w0=0 := by
    rw [NormBranchCoordinates.actualBranchForm,hA,hB]
    exact hb
  have hbridge : NormDualPencil.quadratic Q (NormBranchCoordinates.delete w0)=
      ((C alpha*F)^2-f*(C beta*H)^2).coeff j.val-
        ((C alpha*F)^2-f*(C beta*H)^2).coeff G.natDegree*G.coeff j.val := by
    dsimp only [Q,NormBranchCoordinates.actualReducedMatrix]
    rw [reducedQuadratic,NormBranchCoordinates.insertDelete _ w0
      (NormBranchCoordinates.actualFirstCoefficient F.natDegree H.natDegree q y) hbranch]
    change NormDualPencil.quadratic
      (NormMomentMatrix.coefficientMatrix G f
        (NormMomentMatrix.upperFamily F.natDegree H.natDegree)
        (NormMomentMatrix.lowerFamily F.natDegree H.natDegree) j.val) w0= _
    rw [NormMomentMatrix.quadraticCoefficient]
    simp only [NormLift.norm,hA,hB]
  exact hbridge.symm.trans hq

/-- Cofactor equations imply a full nonzero norm, without assuming that norm. -/
theorem actualNonzeroNorm (F H f G U V : Polynomial K) (q y alpha beta : K)
    (hf : F.Monic) (hh : H.Monic) (hg : G.Monic) (hhp : 0<H.natDegree)
    (hgap : H.natDegree+1<F.natDegree) (hbez : U*F+V*(f*H)=1)
    (hFq : F.eval q≠0) (ha : alpha≠0)
    (hb : (C alpha*F).eval q-y*(C beta*H).eval q=0)
    (hupper : 2*F.natDegree≤G.natDegree)
    (hlower : f.natDegree+2*H.natDegree≤G.natDegree)
    (hpair : ∀ j : Fin G.natDegree, NormDualPencil.pairing
      (NormBranchCoordinates.actualReducedMatrix (NormMomentMatrix.residualFunctional G j.val) f
        F.natDegree H.natDegree q y)
      (NormBranchCoordinates.actualReducedMatrix (NormPencilFunctional.moment F V) f
        F.natDegree H.natDegree q y).adjugate=0) :
    ∃ c : K, c≠0 ∧ (C alpha*F)^2-f*(C beta*H)^2=C c*G := by
  obtain ⟨w0,_,hA,hB⟩ := NormMatrixFaithful.witnessCoordinates F H q alpha beta hFq ha
  have hres : ∀ j : Fin G.natDegree,
      ((C alpha*F)^2-f*(C beta*H)^2).coeff j.val-
        ((C alpha*F)^2-f*(C beta*H)^2).coeff G.natDegree*G.coeff j.val=0 := by
    intro j
    exact actualNormResidual F H f G U V q y alpha beta hf hh hhp hgap hbez hFq ha hb
      w0 hA hB j (hpair j)
  have hdeg := chartNormDegree F H f alpha beta G.natDegree hupper hlower
  have heq := reconstructPolynomial _ G hg hdeg hres
  exact ⟨_,reconstructedScalarNonzero _ G
    (chartNormNonzero F H f U V q y alpha beta hf hh hgap hbez hFq ha hb) heq,heq⟩
theorem evenNormCoefficient (F H f : Polynomial K) (alpha beta : K) (hf : F.Monic)
    (hchart : F.natDegree=H.natDegree+2) (hcurve : f.natDegree≤3) :
    ((C alpha*F)^2-f*(C beta*H)^2).coeff (2*F.natDegree)=alpha^2 := by
  have hb := natDegree_pow_le_of_le 2 (natDegree_C_mul_le beta H)
  have hm : (f*(C beta*H)^2).natDegree≤f.natDegree+((C beta*H)^2).natDegree :=
    natDegree_mul_le
  have hlow : (f*(C beta*H)^2).natDegree<2*F.natDegree := by omega
  have hup : (C alpha*F)^2=C (alpha^2)*(F^2) := by
    simp only [map_pow]
    ring
  rw [coeff_sub,hup,coeff_C_mul,coeff_eq_zero_of_natDegree_lt hlow,sub_zero,
    ← hf.natDegree_pow 2,(hf.pow 2).coeff_natDegree,mul_one]

/-- In the even pole chart the actual nonzero normalization is alpha squared. -/
theorem actualNormalizedNorm (F H f G U V : Polynomial K) (q y alpha beta : K)
    (hf : F.Monic) (hh : H.Monic) (hg : G.Monic) (hhp : 0<H.natDegree)
    (hchart : F.natDegree=H.natDegree+2) (hcurve : f.natDegree≤3)
    (hG : G.natDegree=2*F.natDegree) (hbez : U*F+V*(f*H)=1)
    (hFq : F.eval q≠0) (ha : alpha≠0)
    (hb : (C alpha*F).eval q-y*(C beta*H).eval q=0)
    (hpair : ∀ j : Fin G.natDegree, NormDualPencil.pairing
      (NormBranchCoordinates.actualReducedMatrix (NormMomentMatrix.residualFunctional G j.val) f
        F.natDegree H.natDegree q y)
      (NormBranchCoordinates.actualReducedMatrix (NormPencilFunctional.moment F V) f
        F.natDegree H.natDegree q y).adjugate=0) :
    (C alpha*F)^2-f*(C beta*H)^2=C (alpha^2)*G ∧ alpha^2≠0 := by
  obtain ⟨c,_,heq⟩ := actualNonzeroNorm F H f G U V q y alpha beta hf hh hg hhp
    (by omega) hbez hFq ha hb (by omega) (by omega) hpair
  have hcoeff : ((C alpha*F)^2-f*(C beta*H)^2).coeff G.natDegree=alpha^2 := by
    rw [hG]
    exact evenNormCoefficient F H f alpha beta hf hchart hcurve
  have hc := congrArg (fun P : Polynomial K => P.coeff G.natDegree) heq
  rw [hcoeff,coeff_C_mul,hg.coeff_natDegree,mul_one] at hc
  exact ⟨by simpa only [hc] using heq,pow_ne_zero 2 ha⟩

/-- One fixed nonzero norm has at most one monic target, regardless of rank. -/
theorem monicScalarTargetUnique (N G G' : Polynomial K) (hg : G.Monic) (hg' : G'.Monic)
    (c c' : K) (hc : c≠0) (hN : N=C c*G) (hN' : N=C c'*G') : G=G' := by
  have heq := hN.symm.trans hN'
  have hs : c=c' := by
    have h := congrArg (fun P : Polynomial K => P.leadingCoeff) heq
    simpa only [leadingCoeff_mul,leadingCoeff_C,hg.leadingCoeff,hg'.leadingCoeff,mul_one] using h
  rw [← hs] at heq
  exact mul_left_cancel₀ (C_ne_zero.mpr hc) heq
end NormCertificateRecovery
#print axioms NormCertificateRecovery.quadraticDot
#print axioms NormCertificateRecovery.quadraticScale
#print axioms NormCertificateRecovery.adjugateColumnKernel
#print axioms NormCertificateRecovery.adjugateColumnQuadratic
#print axioms NormCertificateRecovery.directionQuadratic
#print axioms NormCertificateRecovery.branchBetaNonzero
#print axioms NormCertificateRecovery.chartNormNonzero
#print axioms NormCertificateRecovery.chartNormDegree
#print axioms NormCertificateRecovery.reconstructPolynomial
#print axioms NormCertificateRecovery.reconstructedScalarNonzero
#print axioms NormCertificateRecovery.reducedQuadratic
#print axioms NormCertificateRecovery.actualNormResidual
#print axioms NormCertificateRecovery.actualNonzeroNorm
#print axioms NormCertificateRecovery.evenNormCoefficient
#print axioms NormCertificateRecovery.actualNormalizedNorm
#print axioms NormCertificateRecovery.monicScalarTargetUnique
