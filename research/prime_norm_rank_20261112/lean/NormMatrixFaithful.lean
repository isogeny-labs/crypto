module
public import NormMomentMatrix
public import NormBranchRadical
import all NormMomentMatrix
import all NormLift
public section
namespace NormMatrixFaithful
open Polynomial
variable {K : Type*} [Field K]

noncomputable def coordinatePair (a b : ℕ) (w : Fin ((a+1)+(b+1)) → K) :
    Polynomial.degreeLT K (a+1) × Polynomial.degreeLT K (b+1) :=
  (Polynomial.degreeLT.basisProd K (a+1) (b+1)).equivFun.symm w

theorem upperProjection (a b : ℕ) (w : Fin ((a+1)+(b+1)) → K) :
    ((coordinatePair a b w).1 : Polynomial K)=
      NormLift.assemble (NormMomentMatrix.upperFamily a b) w := by
  simp only [coordinatePair,Module.Basis.equivFun_symm_apply,Prod.fst_sum,Prod.smul_fst,
    Submodule.coe_sum,Submodule.coe_smul,NormLift.assemble,NormMomentMatrix.upperFamily]

theorem lowerProjection (a b : ℕ) (w : Fin ((a+1)+(b+1)) → K) :
    ((coordinatePair a b w).2 : Polynomial K)=
      NormLift.assemble (NormMomentMatrix.lowerFamily a b) w := by
  simp only [coordinatePair,Module.Basis.equivFun_symm_apply,Prod.snd_sum,Prod.smul_snd,
    Submodule.coe_sum,Submodule.coe_smul,NormLift.assemble,NormMomentMatrix.lowerFamily]

theorem coordinatesInjective (a b : ℕ) (w z : Fin ((a+1)+(b+1)) → K)
    (hA : NormLift.assemble (NormMomentMatrix.upperFamily a b) w=
      NormLift.assemble (NormMomentMatrix.upperFamily a b) z)
    (hB : NormLift.assemble (NormMomentMatrix.lowerFamily a b) w=
      NormLift.assemble (NormMomentMatrix.lowerFamily a b) z) : w=z := by
  have hpair : coordinatePair a b w=coordinatePair a b z := by
    apply Prod.ext
    · apply Subtype.ext
      simpa only [upperProjection] using hA
    · apply Subtype.ext
      simpa only [lowerProjection] using hB
  exact (Polynomial.degreeLT.basisProd K (a+1) (b+1)).equivFun.symm.injective hpair

theorem assembledUpperDegree (a b : ℕ) (w : Fin ((a+1)+(b+1)) → K) :
    (NormLift.assemble (NormMomentMatrix.upperFamily a b) w).natDegree≤a := by
  rw [← upperProjection]
  have hd := mem_degreeLT.mp (coordinatePair a b w).1.property
  by_cases hz : ((coordinatePair a b w).1 : Polynomial K)=0
  · simp [hz]
  · exact Nat.le_of_lt_succ ((natDegree_lt_iff_degree_lt hz).mpr hd)

theorem assembledLowerDegree (a b : ℕ) (w : Fin ((a+1)+(b+1)) → K) :
    (NormLift.assemble (NormMomentMatrix.lowerFamily a b) w).natDegree≤b := by
  rw [← lowerProjection]
  have hd := mem_degreeLT.mp (coordinatePair a b w).2.property
  by_cases hz : ((coordinatePair a b w).2 : Polynomial K)=0
  · simp [hz]
  · exact Nat.le_of_lt_succ ((natDegree_lt_iff_degree_lt hz).mpr hd)

/-- Actual polynomial polarization with arbitrary coefficient vectors. -/
theorem assembledPairing {I : Type*} [Fintype I]
    (f : Polynomial K) (A B : I → Polynomial K) (w z : I → K) :
    NormLift.assemble A w*NormLift.assemble A z-
        f*(NormLift.assemble B w*NormLift.assemble B z)=
      ∑ i, ∑ j, (w i*z j) • (A i*A j-f*(B i*B j)) := by
  simp only [NormLift.assemble,Polynomial.smul_eq_C_mul,map_mul,mul_sub,
    Finset.sum_sub_distrib,Finset.sum_mul,Finset.mul_sum]
  congr 1
  · rw [Finset.sum_comm]
    apply Finset.sum_congr rfl
    intro i _
    apply Finset.sum_congr rfl
    intro j _
    ring
  · rw [Finset.sum_comm]
    apply Finset.sum_congr rfl
    intro i _
    apply Finset.sum_congr rfl
    intro j _
    ring

/-- Arbitrary-pair matrix identity; no diagonal-to-bilinear inference. -/
theorem bilinearMoment {I : Type*} [Fintype I]
    (mu : Polynomial K →ₗ[K] K) (f : Polynomial K) (A B : I → Polynomial K)
    (w z : I → K) :
    (∑ i, ∑ j, NormMomentMatrix.momentMatrix mu f A B i j*(w i*z j))=
      mu (NormLift.assemble A w*NormLift.assemble A z-
        f*(NormLift.assemble B w*NormLift.assemble B z)) := by
  rw [assembledPairing]
  simp only [map_sum,map_smul,smul_eq_mul,NormMomentMatrix.momentMatrix,
    NormMomentMatrix.entryPolynomial]
  apply Finset.sum_congr rfl
  intro i _
  apply Finset.sum_congr rfl
  intro j _
  ring

/-- Assembly respects actual coefficient scaling. -/
theorem assembleSmul {I : Type*} [Fintype I] (A : I → Polynomial K)
    (w : I → K) (t : K) : NormLift.assemble A (t • w)=t • NormLift.assemble A w := by
  simp only [NormLift.assemble,Pi.smul_apply,smul_smul,Finset.smul_sum,smul_eq_mul]

/-- The actual coordinate branch radical is precisely the witness coefficient line. -/
theorem coordinateBranchRadical (F H f U V : Polynomial K) (q y alpha beta : K)
    (hf : F.Monic) (hh : H.Monic) (hhp : 0<H.natDegree)
    (hgap : H.natDegree+1<F.natDegree) (hbez : U*F+V*(f*H)=1)
    (hFq : F.eval q≠0) (halpha : alpha≠0)
    (hW : (C alpha*F).eval q-y*(C beta*H).eval q=0)
    (w0 w : Fin ((F.natDegree+1)+(H.natDegree+1)) → K)
    (hA0 : NormLift.assemble (NormMomentMatrix.upperFamily F.natDegree H.natDegree) w0=C alpha*F)
    (hB0 : NormLift.assemble (NormMomentMatrix.lowerFamily F.natDegree H.natDegree) w0=C beta*H)
    (hbranch : (NormLift.assemble (NormMomentMatrix.upperFamily F.natDegree H.natDegree) w).eval q-
      y*(NormLift.assemble (NormMomentMatrix.lowerFamily F.natDegree H.natDegree) w).eval q=0) :
    (∀ z : Fin ((F.natDegree+1)+(H.natDegree+1)) → K,
      (NormLift.assemble (NormMomentMatrix.upperFamily F.natDegree H.natDegree) z).eval q-
        y*(NormLift.assemble (NormMomentMatrix.lowerFamily F.natDegree H.natDegree) z).eval q=0 →
      (∑ i, ∑ j, NormMomentMatrix.momentMatrix (NormPencilFunctional.moment F V) f
        (NormMomentMatrix.upperFamily F.natDegree H.natDegree)
        (NormMomentMatrix.lowerFamily F.natDegree H.natDegree) i j*(w i*z j))=0) ↔
    ∃ t : K, w=t • w0 := by
  let A := NormMomentMatrix.upperFamily (K:=K) F.natDegree H.natDegree
  let B := NormMomentMatrix.lowerFamily (K:=K) F.natDegree H.natDegree
  change (∀ z, (NormLift.assemble A z).eval q-y*(NormLift.assemble B z).eval q=0 →
    (∑ i, ∑ j, NormMomentMatrix.momentMatrix (NormPencilFunctional.moment F V) f A B i j*(w i*z j))=0) ↔ _
  have htests :
      (∀ z, (NormLift.assemble A z).eval q-y*(NormLift.assemble B z).eval q=0 →
        (∑ i, ∑ j, NormMomentMatrix.momentMatrix (NormPencilFunctional.moment F V) f A B i j*(w i*z j))=0) ↔
      (∀ Q T : Polynomial K, Q.natDegree≤F.natDegree → T.natDegree≤H.natDegree →
        Q.eval q-y*T.eval q=0 → NormPencilFunctional.moment F V
          (NormLift.assemble A w*Q-f*(NormLift.assemble B w*T))=0) := by
    constructor
    · intro h Q T hQ hT hb
      obtain ⟨z,hzA,hzB⟩ := NormMomentMatrix.familiesComplete F.natDegree H.natDegree Q T hQ hT
      have hz := h z (by simpa only [A,B,hzA,hzB] using hb)
      rw [bilinearMoment] at hz
      simpa only [A,B,hzA,hzB] using hz
    · intro h z hz
      rw [bilinearMoment]
      exact h _ _ (assembledUpperDegree _ _ z) (assembledLowerDegree _ _ z) hz
  rw [htests,NormBranchRadical.restrictedRadicalWitnessLine F H f
    (NormLift.assemble A w) (NormLift.assemble B w) U V q y alpha beta hf hh hhp hgap hbez
    hFq halpha hW hbranch (assembledUpperDegree _ _ w) (assembledLowerDegree _ _ w)]
  have hscaleA (t : K) : NormLift.assemble A (t • w0)=C (t*alpha)*F := by
    rw [assembleSmul,hA0]
    simp only [smul_eq_C_mul,map_mul]
    ring
  have hscaleB (t : K) : NormLift.assemble B (t • w0)=C (t*beta)*H := by
    rw [assembleSmul,hB0]
    simp only [smul_eq_C_mul,map_mul]
    ring
  constructor
  · rintro ⟨t,htA,htB⟩
    exact ⟨t,coordinatesInjective _ _ w (t • w0)
      (htA.trans (hscaleA t).symm) (htB.trans (hscaleB t).symm)⟩
  · rintro ⟨t,rfl⟩
    exact ⟨t,hscaleA t,hscaleB t⟩

/-- Actual complete coordinates supply a nonzero witness vector without a coordinate oracle. -/
theorem witnessCoordinates (F H : Polynomial K) (q alpha beta : K)
    (hFq : F.eval q≠0) (halpha : alpha≠0) :
    ∃ w0 : Fin ((F.natDegree+1)+(H.natDegree+1)) → K, w0≠0 ∧
      NormLift.assemble (NormMomentMatrix.upperFamily F.natDegree H.natDegree) w0=C alpha*F ∧
      NormLift.assemble (NormMomentMatrix.lowerFamily F.natDegree H.natDegree) w0=C beta*H := by
  obtain ⟨w0,hA,hB⟩ := NormMomentMatrix.familiesComplete F.natDegree H.natDegree
    (C alpha*F) (C beta*H) (natDegree_C_mul_le alpha F) (natDegree_C_mul_le beta H)
  refine ⟨w0,?_,hA,hB⟩
  intro hw
  apply NormBranchRadical.witnessNonzero F q alpha hFq halpha
  rw [← hA,hw]
  simp only [NormLift.assemble,Pi.zero_apply,zero_smul,Finset.sum_const_zero]
end NormMatrixFaithful
#print axioms NormMatrixFaithful.upperProjection
#print axioms NormMatrixFaithful.lowerProjection
#print axioms NormMatrixFaithful.coordinatesInjective
#print axioms NormMatrixFaithful.assembledUpperDegree
#print axioms NormMatrixFaithful.assembledLowerDegree
#print axioms NormMatrixFaithful.assembledPairing
#print axioms NormMatrixFaithful.bilinearMoment
#print axioms NormMatrixFaithful.assembleSmul
#print axioms NormMatrixFaithful.coordinateBranchRadical
#print axioms NormMatrixFaithful.witnessCoordinates
