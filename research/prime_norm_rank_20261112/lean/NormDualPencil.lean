module
public import NormRank
public import RankOneExtraction
public import Mathlib.LinearAlgebra.Matrix.Adjugate
public import Mathlib.LinearAlgebra.Matrix.Rank
public section
namespace NormDualPencil
open Matrix
variable {K I J : Type*} [Field K] [Fintype I] [DecidableEq I]

noncomputable def quadratic (Q : Matrix I I K) (w : I → K) : K :=
  ∑ i, ∑ j, Q i j * (w i*w j)
noncomputable def pairing (Q M : Matrix I I K) : K :=
  ∑ i, ∑ j, Q i j * M i j

theorem scaledPairing (Q : Matrix I I K) (s : K) (w : I → K) :
    pairing Q (fun i j => s*(w i*w j)) = s*quadratic Q w := by
  simp only [pairing,quadratic,Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro i hi
  apply Finset.sum_congr rfl
  intro j hj
  ring

/-- Numerical rank at most one supplies every minor condition used by extraction. -/
theorem minorsOfRank (M : Matrix I I K) (hr : M.rank ≤ 1) :
    RankOneExtraction.MinorsVanish M := by
  apply RankOneExtraction.minorsOfEntries M
  intro i j k l
  let T : Matrix (Fin 2) (Fin 2) K := M.submatrix ![i,k] ![j,l]
  have hzero : T.det=0 := by
    by_contra hz
    have ht := Matrix.rank_of_det_ne_zero hz
    have hle := (Matrix.rank_submatrix_le M
      (![i,k] : Fin 2 → I) (![j,l] : Fin 2 → I)).trans hr
    change T.rank ≤ 1 at hle
    rw [ht] at hle
    norm_num at hle
  apply sub_eq_zero.mp
  rw [Matrix.det_fin_two] at hzero
  simpa [T] using hzero

/-- A corank-one singular matrix has adjugate rank at most one. -/
theorem adjugateRank (S : Matrix I I K) (hd : S.det=0)
    (hr : S.rank+1=Fintype.card I) : S.adjugate.rank ≤ 1 := by
  have hm : S*S.adjugate=0 := by rw [Matrix.mul_adjugate,hd,zero_smul]
  have hb := Matrix.rank_add_rank_le_card_of_mul_eq_zero hm
  omega

/-- All cofactor pairings vanish only at a genuine quadratic witness,
provided the singular matrix has corank one and a nonzero adjugate. -/
theorem recoverQuadratic (Q : J → Matrix I I K) (S : Matrix I I K)
    (hsym : S.IsSymm) (hd : S.det=0) (hr : S.rank+1=Fintype.card I)
    (hn : ∃ i j, S.adjugate i j ≠ 0)
    (hpair : ∀ t, pairing (Q t) S.adjugate=0) :
    ∃ w : I → K, (∃ i, w i ≠ 0) ∧ ∀ t, quadratic (Q t) w=0 := by
  have hminor := minorsOfRank S.adjugate (adjugateRank S hd hr)
  have hsym' : ∀ i j, S.adjugate i j=S.adjugate j i :=
    fun i j => (hsym.adjugate.apply i j).symm
  obtain ⟨s,w,hs,hw,hfactor⟩ :=
    RankOneExtraction.factorExists S.adjugate hsym' hminor hn
  refine ⟨w,hw,?_⟩
  intro t
  have heq : pairing (Q t) S.adjugate=s*quadratic (Q t) w := by
    have hf : S.adjugate=(fun i j => s*(w i*w j)) := by
      funext i j
      exact hfactor i j
    rw [hf]
    exact scaledPairing (Q t) s w
  have hz : s*quadratic (Q t) w=0 := heq.symm.trans (hpair t)
  exact (mul_eq_zero.mp hz).resolve_left hs

noncomputable def first (U : K) : Matrix (Fin 2) (Fin 2) K := !![1,0;0,-U]
noncomputable def second (V y : K) : Matrix (Fin 2) (Fin 2) K := !![0,y;y,-V]
noncomputable def pencil (U V y a b : K) : Matrix (Fin 2) (Fin 2) K :=
  a • first U + b • second V y

/-- Determinant zero is weaker than the cofactor stationarity conditions. -/
theorem pencilDet (U V y a b : K) :
    (pencil U V y a b).det = -U*a^2-V*a*b-y^2*b^2 := by
  simp [pencil,first,second,Matrix.det_fin_two]
  ring

theorem pencilCofactors (U V y a b : K) :
    pairing (first U) (pencil U V y a b).adjugate = -2*U*a-V*b ∧
    pairing (second V y) (pencil U V y a b).adjugate = -V*a-2*y^2*b := by
  simp [pairing,pencil,first,second,Matrix.adjugate_fin_two,Fin.sum_univ_two]
  constructor <;> ring

local instance : Fact (Nat.Prime 17) := ⟨by decide⟩

/-- Two actual curve targets can give the same singular matrix but different cofactor tests. -/
theorem sameMatrixDifferentTargets :
    pencil (4:ZMod 17) 10 6 3 1=pencil 11 6 6 3 1 := by
  ext i j
  fin_cases i <;> fin_cases j <;> decide

theorem positiveCofactors :
    pairing (first (4:ZMod 17)) (pencil 4 10 6 3 1).adjugate=0 ∧
    pairing (second (10:ZMod 17) 6) (pencil 4 10 6 3 1).adjugate=0 := by
  obtain ⟨h1,h2⟩ := pencilCofactors (4:ZMod 17) 10 6 3 1
  rw [h1,h2]
  constructor <;> decide

theorem positiveQuadratic :
    quadratic (first (4:ZMod 17)) ![15,1]=0 ∧
    quadratic (second (10:ZMod 17) 6) ![15,1]=0 := by decide

theorem positiveActualNorm :
    (NormRank.trial (10:ZMod 17) 6 15)^2-NormRank.cubic 2 2+
      NormRank.basePair 5 6*(Polynomial.X-Polynomial.C 10)=0 := by
  exact (NormRank.normEquationIff (2:ZMod 17) 2 10 6 5 6 15
    (by decide) (by decide)).2 (by decide)

/-- The explicit adjugate column normalizes to the genuine norm slope. -/
theorem positiveRecoveredSlope :
    (pencil (4:ZMod 17) 10 6 3 1).adjugate 1 1 ≠ 0 ∧
    (pencil (4:ZMod 17) 10 6 3 1).adjugate 0 1 /
      (pencil (4:ZMod 17) 10 6 3 1).adjugate 1 1 = 15 := by
  have hp : (pencil (4:ZMod 17) 10 6 3 1).adjugate 1 1 ≠ 0 := by decide
  exact ⟨hp, (div_eq_iff hp).2 (by decide)⟩

theorem positiveRecoveredNorm :
    (NormRank.trial (10:ZMod 17) 6
      ((pencil (4:ZMod 17) 10 6 3 1).adjugate 0 1 /
        (pencil (4:ZMod 17) 10 6 3 1).adjugate 1 1))^2-
      NormRank.cubic 2 2+NormRank.basePair 5 6*
        (Polynomial.X-Polynomial.C 10)=0 := by
  rw [positiveRecoveredSlope.2]
  exact positiveActualNorm

theorem falseRoot :
    (pencil (11:ZMod 17) 6 6 3 1).det=0 ∧
    (pencil (11:ZMod 17) 6 6 3 1).adjugate 1 1≠0 ∧
    pairing (first (11:ZMod 17)) (pencil 11 6 6 3 1).adjugate≠0 := by
  rw [pencilDet]
  constructor
  · decide
  constructor
  · decide
  · have h := (pencilCofactors (11:ZMod 17) 6 6 3 1).1
    rw [h]
    decide

/-- Generic higher-corank control: cofactor equations can all vanish at a nonzero matrix. -/
noncomputable def spurious : Matrix (Fin 3) (Fin 3) (ZMod 17) :=
  !![1,0,0;0,0,0;0,0,0]

theorem spuriousAdjugate : spurious.adjugate=0 := by
  rw [Matrix.adjugate_fin_three]
  ext i j
  fin_cases i <;> fin_cases j <;> decide

theorem spuriousPairing (Q : Matrix (Fin 3) (Fin 3) (ZMod 17)) :
    pairing Q spurious.adjugate=0 := by
  rw [spuriousAdjugate]
  simp [pairing]
noncomputable def coordinateForm (i : Fin 3) : Matrix (Fin 3) (Fin 3) (ZMod 17) :=
  fun j k => if j=i ∧ k=i then 1 else 0

theorem coordinateQuadratic (i : Fin 3) (w : Fin 3 → ZMod 17) :
    quadratic (coordinateForm i) w=(w i)^2 := by
  fin_cases i <;> simp [quadratic,coordinateForm,Fin.sum_univ_three,pow_two]

theorem spuriousIsPencilPoint : spurious=coordinateForm 0 := by
  ext i j
  fin_cases i <;> fin_cases j <;> decide

theorem spuriousSingularNonzero : spurious.det=0 ∧ spurious 0 0≠0 := by decide

theorem noCoordinateWitness : ¬∃ w : Fin 3 → ZMod 17,
    (∃ i, w i ≠ 0) ∧ ∀ i, quadratic (coordinateForm i) w=0 := by
  rintro ⟨w,⟨i,hi⟩,h⟩
  have hz : w i*w i=0 := by simpa only [coordinateQuadratic,pow_two] using h i
  exact hi ((mul_eq_zero.mp hz).elim id id)
/-- The kernel recovered from the false determinant root fails the actual norm. -/
theorem falseRootNormResidual :
    (NormRank.trial (0:ZMod 17) 6 15)^2-NormRank.cubic 2 2+
      NormRank.basePair 5 6*(Polynomial.X-Polynomial.C 0)≠0 := by
  intro hz
  have h := NormRank.defectIdentity (2:ZMod 17) 2 0 6 5 6 15
  rw [hz] at h
  have hc := congrArg (fun P : Polynomial (ZMod 17) => P.eval 1) h
  norm_num at hc
  exact (by decide : (0:ZMod 17) ≠ 422) hc

/-- Complete higher-corank negative control in the coordinate-quadrics pencil. -/
theorem spuriousControl :
    spurious.det=0 ∧ spurious 0 0≠0 ∧ spurious.adjugate=0 ∧
    (∀ i, pairing (coordinateForm i) spurious.adjugate=0) ∧
    ¬∃ w : Fin 3 → ZMod 17, (∃ i, w i ≠ 0) ∧
      ∀ i, quadratic (coordinateForm i) w=0 :=
  ⟨spuriousSingularNonzero.1,spuriousSingularNonzero.2,spuriousAdjugate,
    (fun i => spuriousPairing (coordinateForm i)),noCoordinateWitness⟩
end NormDualPencil
#print axioms NormDualPencil.minorsOfRank
#print axioms NormDualPencil.adjugateRank
#print axioms NormDualPencil.recoverQuadratic
#print axioms NormDualPencil.pencilDet
#print axioms NormDualPencil.pencilCofactors
#print axioms NormDualPencil.sameMatrixDifferentTargets
#print axioms NormDualPencil.positiveCofactors
#print axioms NormDualPencil.positiveQuadratic
#print axioms NormDualPencil.positiveActualNorm
#print axioms NormDualPencil.falseRoot
#print axioms NormDualPencil.spuriousAdjugate
#print axioms NormDualPencil.spuriousPairing
#print axioms NormDualPencil.positiveRecoveredSlope
#print axioms NormDualPencil.positiveRecoveredNorm
#print axioms NormDualPencil.coordinateQuadratic
#print axioms NormDualPencil.spuriousIsPencilPoint
#print axioms NormDualPencil.spuriousSingularNonzero
#print axioms NormDualPencil.noCoordinateWitness
#print axioms NormDualPencil.falseRootNormResidual
#print axioms NormDualPencil.spuriousControl
