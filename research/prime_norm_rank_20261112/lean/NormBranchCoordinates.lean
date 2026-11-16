module
public import NormMatrixFaithful
public import Mathlib.LinearAlgebra.FiniteDimensional.Basic
import all Mathlib.Data.Matrix.Mul
import all Mathlib.Data.Fin.VecNotation
import all Mathlib.Data.Fin.Tuple.Basic
import all NormMatrixFaithful
import all NormMomentMatrix
import all NormLift
public section
namespace NormBranchCoordinates
open Polynomial Matrix
variable {K : Type*} [Field K] {n : ℕ}

noncomputable def branchForm (ell w : Fin (n+1) → K) : K :=
  ∑ i, ell i*w i

/-- Public insertion; no norm witness or moment coefficients enter this map. -/
noncomputable def insert (ell : Fin (n+1) → K) (z : Fin n → K) : Fin (n+1) → K :=
  Fin.cons (-(∑ j, ell j.succ*z j)) z

def delete (w : Fin (n+1) → K) : Fin n → K := fun j => w j.succ

theorem insertedBranch (ell : Fin (n+1) → K) (h0 : ell 0=1) (z : Fin n → K) :
    branchForm ell (insert ell z)=0 := by
  simp [branchForm,insert,Fin.sum_univ_succ,h0]

theorem deleteInsert (ell : Fin (n+1) → K) (z : Fin n → K) :
    delete (insert ell z)=z := by
  funext j
  simp [delete,insert]

theorem insertDelete (ell w : Fin (n+1) → K) (h0 : ell 0=1)
    (hw : branchForm ell w=0) : insert ell (delete w)=w := by
  have h : w 0+∑ j : Fin n, ell j.succ*w j.succ=0 := by
    simpa [branchForm,Fin.sum_univ_succ,h0] using hw
  funext i
  refine Fin.cases ?_ (fun j => ?_) i
  · simpa [insert,delete] using (eq_neg_of_add_eq_zero_left h).symm
  · simp [insert,delete]

theorem insertInjective (ell : Fin (n+1) → K) : Function.Injective (insert ell) := by
  intro z v h
  have ht := congrArg delete h
  simpa only [deleteInsert] using ht

noncomputable def actualCoefficients (a b : ℕ) (q y : K) :
    Fin ((a+1)+(b+1)) → K :=
  NormLift.branch (NormMomentMatrix.upperFamily a b) (NormMomentMatrix.lowerFamily a b) q y

/-- The public product basis begins with the polynomial pair (1,0). -/
theorem actualFirstCoefficient (a b : ℕ) (q y : K) :
    actualCoefficients a b q y 0=1 := by
  change (NormMomentMatrix.upperFamily a b ((0 : Fin (a+1)).castAdd (b+1))).eval q-
    y*(NormMomentMatrix.lowerFamily a b ((0 : Fin (a+1)).castAdd (b+1))).eval q=1
  simp [NormMomentMatrix.upperFamily,NormMomentMatrix.lowerFamily]

/-- Actual branch evaluation equals the public coordinate linear form. -/
theorem actualBranchForm (a b : ℕ) (q y : K)
    (w : Fin ((a+1)+(b+1)) → K) :
    branchForm (actualCoefficients a b q y) w=
      (NormLift.assemble (NormMomentMatrix.upperFamily a b) w).eval q-
      y*(NormLift.assemble (NormMomentMatrix.lowerFamily a b) w).eval q := by
  rw [NormLift.branchEval]
  unfold branchForm actualCoefficients
  exact Finset.sum_congr rfl (fun i _ => mul_comm _ _)

/-- Sparse public columns e_(j+1)-ell_(j+1)e_0. -/
noncomputable def insertionMatrix (ell : Fin (n+1) → K) : Matrix (Fin (n+1)) (Fin n) K :=
  Matrix.of (Fin.cons (fun j => -ell j.succ) (1 : Matrix (Fin n) (Fin n) K))

theorem insertionZero (ell : Fin (n+1) → K) (j : Fin n) :
    insertionMatrix ell 0 j = -ell j.succ := by
  exact congrFun (Fin.cons_zero (α:=fun _ : Fin (n+1) => Fin n → K)
    (fun j : Fin n => -ell j.succ) (1 : Matrix (Fin n) (Fin n) K)) j

theorem insertionSucc (ell : Fin (n+1) → K) (i j : Fin n) :
    insertionMatrix ell i.succ j = if i=j then 1 else 0 := by
  change Fin.cons (α:=fun _ : Fin (n+1) => Fin n → K)
    (fun j : Fin n => -ell j.succ) (1 : Matrix (Fin n) (Fin n) K) i.succ j = _
  exact (congrFun (Fin.cons_succ (α:=fun _ : Fin (n+1) => Fin n → K)
    (fun j : Fin n => -ell j.succ) (1 : Matrix (Fin n) (Fin n) K) i) j).trans
      Matrix.one_apply

theorem matrixInserts (ell : Fin (n+1) → K) (z : Fin n → K) :
    (insertionMatrix ell).mulVec z=insert ell z := by
  funext i
  refine Fin.cases ?_ (fun j => ?_) i
  · change (∑ j, insertionMatrix ell 0 j*z j) = _
    simp [insertionZero,insert,neg_mul,Finset.sum_neg_distrib]
  · change (∑ k, insertionMatrix ell j.succ k*z k) = _
    simp [insertionSucc,insert,ite_mul]

noncomputable def reducedMatrix (ell : Fin (n+1) → K)
    (M : Matrix (Fin (n+1)) (Fin (n+1)) K) : Matrix (Fin n) (Fin n) K :=
  (insertionMatrix ell).transpose * M * insertionMatrix ell

/-- Congruence has an explicit quadratic-size entry formula. -/
theorem reducedEntry (ell : Fin (n+1) → K)
    (M : Matrix (Fin (n+1)) (Fin (n+1)) K) (i j : Fin n) :
    reducedMatrix ell M i j=M i.succ j.succ-ell i.succ*M 0 j.succ-
      ell j.succ*M i.succ 0+ell i.succ*ell j.succ*M 0 0 := by
  unfold reducedMatrix
  rw [Matrix.mul_apply]
  simp only [Matrix.mul_apply,Matrix.transpose_apply,Fin.sum_univ_succ,
    insertionZero,insertionSucc]
  simp [ite_mul,mul_ite]
  ring

theorem insertScale (ell : Fin (n+1) → K) (t : K) (z : Fin n → K) :
    insert ell (t • z)=t • insert ell z := by
  rw [← matrixInserts,Matrix.mulVec_smul,matrixInserts]

theorem deleteScale (w : Fin (n+1) → K) (t : K) :
    delete (t • w)=t • delete w := by
  funext i
  simp only [delete,Pi.smul_apply]

/-- The actual reduced matrix represents the original bilinear form on the branch. -/
theorem reducedBilinear (ell : Fin (n+1) → K)
    (M : Matrix (Fin (n+1)) (Fin (n+1)) K) (x z : Fin n → K) :
    dotProduct x ((reducedMatrix ell M).mulVec z)=
      dotProduct (insert ell x) (M.mulVec (insert ell z)) := by
  unfold reducedMatrix
  rw [← Matrix.mulVec_mulVec,← Matrix.mulVec_mulVec,Matrix.dotProduct_transpose_mulVec]
  simpa only [matrixInserts] using
    (dotProduct_comm (M.mulVec ((insertionMatrix ell).mulVec z))
      ((insertionMatrix ell).mulVec x))

/-- Restriction transfers a supplied exact radical line through the public coordinates. -/
theorem reducedRadicalTransfer (ell : Fin (n+1) → K) (h0 : ell 0=1)
    (M : Matrix (Fin (n+1)) (Fin (n+1)) K) (w0 : Fin (n+1) → K)
    (hw0 : branchForm ell w0=0)
    (hline : ∀ w, branchForm ell w=0 →
      ((∀ z, branchForm ell z=0 → dotProduct w (M.mulVec z)=0) ↔
        ∃ t : K, w=t • w0)) (x : Fin n → K) :
    (∀ z, dotProduct x ((reducedMatrix ell M).mulVec z)=0) ↔
      ∃ t : K, x=t • delete w0 := by
  have hf := hline (insert ell x) (insertedBranch ell h0 x)
  constructor
  · intro hx
    have hfull : ∀ z, branchForm ell z=0 → dotProduct (insert ell x) (M.mulVec z)=0 := by
      intro z hz
      calc
        dotProduct (insert ell x) (M.mulVec z)=
            dotProduct (insert ell x) (M.mulVec (insert ell (delete z))) := by
          rw [insertDelete ell z h0 hz]
        _=dotProduct x ((reducedMatrix ell M).mulVec (delete z)) :=
          (reducedBilinear ell M x (delete z)).symm
        _=0 := hx (delete z)
    obtain ⟨t,ht⟩ := hf.mp hfull
    refine ⟨t,?_⟩
    simpa only [deleteInsert,deleteScale] using congrArg delete ht
  · rintro ⟨t,ht⟩
    have hi : insert ell x=t • w0 := by
      rw [ht,insertScale,insertDelete ell w0 h0 hw0]
    have hfull := hf.mpr ⟨t,hi⟩
    intro z
    rw [reducedBilinear]
    exact hfull _ (insertedBranch ell h0 z)

theorem deletedNonzero (ell w : Fin (n+1) → K) (h0 : ell 0=1)
    (hw : branchForm ell w=0) (hn : w≠0) : delete w≠0 := by
  intro hz
  apply hn
  rw [← insertDelete ell w h0 hw,hz]
  rw [← matrixInserts,Matrix.mulVec_zero]

/-- The row/column convention is tied to the actual double sum used earlier. -/
theorem dotMatrixSum (M : Matrix (Fin (n+1)) (Fin (n+1)) K)
    (w z : Fin (n+1) → K) :
    dotProduct w (M.mulVec z)=∑ i, ∑ j, M i j*(w i*z j) := by
  simp only [Matrix.mulVec,dotProduct,Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro i _
  apply Finset.sum_congr rfl
  intro j _
  ring

noncomputable def actualReducedMatrix (mu : Polynomial K →ₗ[K] K) (f : Polynomial K)
    (a b : ℕ) (q y : K) : Matrix (Fin ((a+1)+b)) (Fin ((a+1)+b)) K :=
  reducedMatrix (actualCoefficients a b q y)
    (NormMomentMatrix.momentMatrix mu f
      (NormMomentMatrix.upperFamily a b) (NormMomentMatrix.lowerFamily a b))

/-- Exact radical of the actual public-coordinate reduction of the moment matrix. -/
theorem actualReducedRadical (F H f U V : Polynomial K) (q y alpha beta : K)
    (hf : F.Monic) (hh : H.Monic) (hhp : 0<H.natDegree)
    (hgap : H.natDegree+1<F.natDegree) (hbez : U*F+V*(f*H)=1)
    (hFq : F.eval q≠0) (halpha : alpha≠0)
    (hW : (C alpha*F).eval q-y*(C beta*H).eval q=0)
    (w0 : Fin ((F.natDegree+1)+(H.natDegree+1)) → K)
    (hA0 : NormLift.assemble (NormMomentMatrix.upperFamily F.natDegree H.natDegree) w0=C alpha*F)
    (hB0 : NormLift.assemble (NormMomentMatrix.lowerFamily F.natDegree H.natDegree) w0=C beta*H)
    (x : Fin ((F.natDegree+1)+H.natDegree) → K) :
    (∀ z, dotProduct x ((actualReducedMatrix (NormPencilFunctional.moment F V) f
      F.natDegree H.natDegree q y).mulVec z)=0) ↔
      ∃ t : K, x=t • delete w0 := by
  have hw0 : branchForm (actualCoefficients F.natDegree H.natDegree q y) w0=0 := by
    rw [actualBranchForm,hA0,hB0]
    exact hW
  unfold actualReducedMatrix
  apply reducedRadicalTransfer (actualCoefficients F.natDegree H.natDegree q y)
    (actualFirstCoefficient F.natDegree H.natDegree q y) _ w0 hw0 _ x
  intro w hw
  have hbranch :
      (NormLift.assemble (NormMomentMatrix.upperFamily F.natDegree H.natDegree) w).eval q-
        y*(NormLift.assemble (NormMomentMatrix.lowerFamily F.natDegree H.natDegree) w).eval q=0 :=
    (actualBranchForm _ _ q y w).symm.trans hw
  have hline := NormMatrixFaithful.coordinateBranchRadical F H f U V q y alpha beta
    hf hh hhp hgap hbez hFq halpha hW w0 w hA0 hB0 hbranch
  constructor
  · intro h
    apply hline.mp
    intro z hz
    have htest := h z ((actualBranchForm _ _ q y z).trans hz)
    exact (dotMatrixSum _ w z).symm.trans htest
  · intro h z hz
    rw [dotMatrixSum]
    exact (hline.mpr h) z ((actualBranchForm _ _ q y z).symm.trans hz)

/-- Left annihilation is the ordinary kernel of the transpose; no symmetry premise. -/
theorem leftRadicalIff (M : Matrix (Fin n) (Fin n) K) (x : Fin n → K) :
    (∀ z, dotProduct x (M.mulVec z)=0) ↔ M.transpose.mulVec x=0 := by
  simp_rw [Matrix.dotProduct_mulVec,← Matrix.mulVec_transpose]
  exact dotProduct_eq_zero_iff

/-- A proved nonzero radical line supplies numerical corank one. -/
theorem rankOfRadicalLine (M : Matrix (Fin n) (Fin n) K) (u : Fin n → K)
    (hu : u≠0)
    (hline : ∀ x, (∀ z, dotProduct x (M.mulVec z)=0) ↔ ∃ t : K, x=t • u) :
    M.rank+1=n := by
  have hker : LinearMap.ker M.transpose.mulVecLin=K ∙ u := by
    ext x
    simp only [LinearMap.mem_ker,Matrix.mulVecLin_apply]
    rw [← leftRadicalIff,hline x,Submodule.mem_span_singleton]
    exact exists_congr (fun t => eq_comm)
  have hk : Module.finrank K (LinearMap.ker M.transpose.mulVecLin)=1 := by
    rw [hker,finrank_span_singleton hu]
  have hr := M.transpose.mulVecLin.finrank_range_add_finrank_ker
  rw [hk,Module.finrank_pi,Fintype.card_fin] at hr
  change M.transpose.rank+1=n at hr
  rwa [Matrix.rank_transpose] at hr

/-- Numerical corank for the actual matrix, with coordinates constructed by completeness. -/
theorem actualReducedCorank (F H f U V : Polynomial K) (q y alpha beta : K)
    (hf : F.Monic) (hh : H.Monic) (hhp : 0<H.natDegree)
    (hgap : H.natDegree+1<F.natDegree) (hbez : U*F+V*(f*H)=1)
    (hFq : F.eval q≠0) (halpha : alpha≠0)
    (hW : (C alpha*F).eval q-y*(C beta*H).eval q=0) :
    (actualReducedMatrix (NormPencilFunctional.moment F V) f
      F.natDegree H.natDegree q y).rank+1=(F.natDegree+1)+H.natDegree := by
  obtain ⟨w0,hn,hA,hB⟩ := NormMatrixFaithful.witnessCoordinates F H q alpha beta hFq halpha
  have hw0 : branchForm (actualCoefficients F.natDegree H.natDegree q y) w0=0 := by
    rw [actualBranchForm,hA,hB]
    exact hW
  apply rankOfRadicalLine _ (delete w0)
    (deletedNonzero _ w0 (actualFirstCoefficient F.natDegree H.natDegree q y) hw0 hn)
  intro x
  exact actualReducedRadical F H f U V q y alpha beta hf hh hhp hgap hbez hFq halpha
    hW w0 hA hB x

theorem actualReducedSingular (F H f U V : Polynomial K) (q y alpha beta : K)
    (hf : F.Monic) (hh : H.Monic) (hhp : 0<H.natDegree)
    (hgap : H.natDegree+1<F.natDegree) (hbez : U*F+V*(f*H)=1)
    (hFq : F.eval q≠0) (halpha : alpha≠0)
    (hW : (C alpha*F).eval q-y*(C beta*H).eval q=0) :
    (actualReducedMatrix (NormPencilFunctional.moment F V) f
      F.natDegree H.natDegree q y).det=0 := by
  by_contra hd
  have hr := actualReducedCorank F H f U V q y alpha beta hf hh hhp hgap hbez hFq halpha hW
  rw [Matrix.rank_of_det_ne_zero hd,Fintype.card_fin] at hr
  omega

theorem reducedSymmetric (ell : Fin (n+1) → K)
    (M : Matrix (Fin (n+1)) (Fin (n+1)) K) (hs : M.IsSymm) :
    (reducedMatrix ell M).IsSymm := by
  unfold reducedMatrix
  rw [Matrix.IsSymm,Matrix.transpose_mul,Matrix.transpose_mul,
    Matrix.transpose_transpose,hs.eq,Matrix.mul_assoc]

/-- A nonzero radical coordinate gives an explicit nonzero diagonal cofactor. -/
theorem cofactorPivot (M : Matrix (Fin n) (Fin n) K) (hs : M.IsSymm)
    (u : Fin n → K)
    (hline : ∀ x, (∀ z, dotProduct x (M.mulVec z)=0) ↔ ∃ t : K, x=t • u)
    (k : Fin n) (huk : u k≠0) : M.adjugate k k≠0 := by
  let A := M.updateRow k (Pi.single k 1)
  have hleft : ∀ z, dotProduct u (M.mulVec z)=0 :=
    (hline u).mpr ⟨1,by simp⟩
  have hzero : ∀ v, A.mulVec v=0 → v=0 := by
    intro v hv
    have hvk : v k=0 := by
      have h := congrFun hv k
      simpa only [A,Matrix.mulVec,Matrix.updateRow_self,single_dotProduct,
        one_mul,Pi.zero_apply] using h
    have hother : ∀ i, i≠k → M.mulVec v i=0 := by
      intro i hi
      have h := congrFun hv i
      simpa only [A,Matrix.mulVec,Matrix.updateRow_ne hi,Pi.zero_apply] using h
    have hsum : dotProduct u (M.mulVec v)=u k*(M.mulVec v k) := by
      unfold dotProduct
      exact Finset.sum_eq_single k
        (fun i _ hi => by rw [hother i hi,mul_zero]) (by simp)
    have hmk : M.mulVec v k=0 :=
      (mul_eq_zero.mp (hsum.symm.trans (hleft v))).resolve_left huk
    have hmv : M.mulVec v=0 := by
      funext i
      by_cases hi : i=k
      · simpa only [hi,Pi.zero_apply] using hmk
      · exact hother i hi
    have hvrad : ∀ z, dotProduct v (M.mulVec z)=0 :=
      (leftRadicalIff M v).mpr (by rw [hs.eq]; exact hmv)
    obtain ⟨t,ht⟩ := (hline v).mp hvrad
    have htk : v k=t*u k := by
      simpa only [Pi.smul_apply,smul_eq_mul] using congrFun ht k
    have ht0 : t=0 := (mul_eq_zero.mp (htk.symm.trans hvk)).resolve_right huk
    simpa only [ht0,zero_smul] using ht
  have hinj : Function.Injective A.mulVec := by
    intro v w hvw
    apply sub_eq_zero.mp
    apply hzero
    rw [Matrix.mulVec_sub,hvw,sub_self]
  have hunit : IsUnit A := Matrix.mulVec_injective_iff_isUnit.mp hinj
  have hdet : A.det≠0 := ((Matrix.isUnit_iff_isUnit_det A).mp hunit).ne_zero
  rw [Matrix.adjugate_apply]
  exact hdet

/-- The actual reduced matrix has a nonzero diagonal adjugate entry. -/
theorem actualReducedCofactor (F H f U V : Polynomial K) (q y alpha beta : K)
    (hf : F.Monic) (hh : H.Monic) (hhp : 0<H.natDegree)
    (hgap : H.natDegree+1<F.natDegree) (hbez : U*F+V*(f*H)=1)
    (hFq : F.eval q≠0) (halpha : alpha≠0)
    (hW : (C alpha*F).eval q-y*(C beta*H).eval q=0) :
    ∃ k, (actualReducedMatrix (NormPencilFunctional.moment F V) f
      F.natDegree H.natDegree q y).adjugate k k≠0 := by
  obtain ⟨w0,hn,hA,hB⟩ := NormMatrixFaithful.witnessCoordinates F H q alpha beta hFq halpha
  have hw0 : branchForm (actualCoefficients F.natDegree H.natDegree q y) w0=0 := by
    rw [actualBranchForm,hA,hB]
    exact hW
  have ht : delete w0≠0 :=
    deletedNonzero _ w0 (actualFirstCoefficient F.natDegree H.natDegree q y) hw0 hn
  have hex : ∃ k, delete w0 k≠0 := by
    by_contra h
    push_neg at h
    exact ht (funext h)
  obtain ⟨k,hk⟩ := hex
  have hs : (actualReducedMatrix (NormPencilFunctional.moment F V) f
      F.natDegree H.natDegree q y).IsSymm := by
    unfold actualReducedMatrix
    exact reducedSymmetric _ _ (NormMomentMatrix.momentSymmetric _ _ _ _)
  refine ⟨k,cofactorPivot _ hs (delete w0) ?_ k hk⟩
  intro x
  exact actualReducedRadical F H f U V q y alpha beta hf hh hhp hgap hbez hFq halpha
    hW w0 hA hB x
/-- Recovery on the actual reduced coefficient family; structural guards are derived. -/
theorem actualCoefficientRecovery (F H f G U V : Polynomial K) (q y alpha beta : K)
    (hf : F.Monic) (hh : H.Monic) (hhp : 0<H.natDegree)
    (hgap : H.natDegree+1<F.natDegree) (hbez : U*F+V*(f*H)=1)
    (hFq : F.eval q≠0) (halpha : alpha≠0)
    (hW : (C alpha*F).eval q-y*(C beta*H).eval q=0)
    (hpair : ∀ j : Fin G.natDegree,
      NormDualPencil.pairing
        (actualReducedMatrix (NormMomentMatrix.residualFunctional G j.val) f
          F.natDegree H.natDegree q y)
        (actualReducedMatrix (NormPencilFunctional.moment F V) f
          F.natDegree H.natDegree q y).adjugate=0) :
    ∃ x : Fin ((F.natDegree+1)+H.natDegree) → K, (∃ i, x i≠0) ∧
      ∀ j : Fin G.natDegree,
        NormDualPencil.quadratic
          (actualReducedMatrix (NormMomentMatrix.residualFunctional G j.val) f
            F.natDegree H.natDegree q y) x=0 := by
  have hs : (actualReducedMatrix (NormPencilFunctional.moment F V) f
      F.natDegree H.natDegree q y).IsSymm := by
    unfold actualReducedMatrix
    exact reducedSymmetric _ _ (NormMomentMatrix.momentSymmetric _ _ _ _)
  have hd := actualReducedSingular F H f U V q y alpha beta
    hf hh hhp hgap hbez hFq halpha hW
  have hr := actualReducedCorank F H f U V q y alpha beta
    hf hh hhp hgap hbez hFq halpha hW
  obtain ⟨k,hk⟩ := actualReducedCofactor F H f U V q y alpha beta
    hf hh hhp hgap hbez hFq halpha hW
  exact NormDualPencil.recoverQuadratic _ _ hs hd (by simpa only [Fintype.card_fin] using hr)
    ⟨k,k,hk⟩ hpair
end NormBranchCoordinates
#print axioms NormBranchCoordinates.insertedBranch
#print axioms NormBranchCoordinates.deleteInsert
#print axioms NormBranchCoordinates.insertDelete
#print axioms NormBranchCoordinates.insertInjective
#print axioms NormBranchCoordinates.actualFirstCoefficient
#print axioms NormBranchCoordinates.actualBranchForm
#print axioms NormBranchCoordinates.matrixInserts
#print axioms NormBranchCoordinates.reducedEntry
#print axioms NormBranchCoordinates.insertionZero
#print axioms NormBranchCoordinates.insertionSucc
#print axioms NormBranchCoordinates.insertScale
#print axioms NormBranchCoordinates.deleteScale
#print axioms NormBranchCoordinates.reducedBilinear
#print axioms NormBranchCoordinates.reducedRadicalTransfer
#print axioms NormBranchCoordinates.deletedNonzero
#print axioms NormBranchCoordinates.dotMatrixSum
#print axioms NormBranchCoordinates.actualReducedRadical
#print axioms NormBranchCoordinates.leftRadicalIff
#print axioms NormBranchCoordinates.rankOfRadicalLine
#print axioms NormBranchCoordinates.actualReducedCorank
#print axioms NormBranchCoordinates.actualReducedSingular
#print axioms NormBranchCoordinates.reducedSymmetric
#print axioms NormBranchCoordinates.cofactorPivot
#print axioms NormBranchCoordinates.actualReducedCofactor
#print axioms NormBranchCoordinates.actualCoefficientRecovery
