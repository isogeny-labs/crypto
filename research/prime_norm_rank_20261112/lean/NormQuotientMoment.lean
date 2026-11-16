module
public import NormDualPencil
import all NormDualPencil
public import Mathlib.Algebra.Polynomial.Degree.SmallDegree
public import Mathlib.Algebra.Polynomial.Div
public section
namespace NormQuotientMoment
open Polynomial Matrix

/-- A complementary monomial detects every nonzero class below a monic modulus. -/
theorem topCoefficientDetects {K : Type*} [Field K] (A P : Polynomial K)
    (hm : A.Monic) (hp : P ≠ 0) (hd : P.natDegree < A.natDegree) :
    ∃ Q : Polynomial K, Q.natDegree < A.natDegree ∧
      ((P*Q) %ₘ A).coeff (A.natDegree-1) ≠ 0 := by
  let t := A.natDegree-1-P.natDegree
  have ha : 0 < A.natDegree := lt_of_le_of_lt (Nat.zero_le _) hd
  have hx : (X^t : Polynomial K) ≠ 0 := pow_ne_zero _ X_ne_zero
  have hn : P*X^t ≠ 0 := mul_ne_zero hp hx
  have hdeg : (P*X^t).natDegree=A.natDegree-1 := by
    rw [natDegree_mul_X_pow t hp]
    dsimp [t]
    omega
  have hl : (P*X^t).degree < A.degree := by
    rw [degree_eq_natDegree hn,degree_eq_natDegree hm.ne_zero]
    exact_mod_cast (show (P*X^t).natDegree < A.natDegree by rw [hdeg]; omega)
  refine ⟨X^t, ?_, ?_⟩
  · simp only [natDegree_X_pow]
    dsimp [t]
    omega
  · rw [(modByMonic_eq_self_iff hm).mpr hl, ← hdeg, coeff_natDegree,
      leadingCoeff_mul,leadingCoeff_X_pow,mul_one]
    exact leadingCoeff_ne_zero.mpr hp

/-- Multiplication by a unit modulo A preserves coefficient-pairing detection. -/
theorem twistedCoefficientDetects {K : Type*} [Field K] (A U P : Polynomial K)
    (hm : A.Monic) (hu : IsCoprime A U) (hp : P≠0)
    (hd : P.natDegree<A.natDegree) :
    ∃ Q : Polynomial K, Q.natDegree<A.natDegree ∧
      ((P*U*Q) %ₘ A).coeff (A.natDegree-1)≠0 := by
  have ha : A≠1 := by
    intro h
    rw [h,natDegree_one] at hd
    omega
  let R := (P*U) %ₘ A
  have hr : R≠0 := by
    intro hz
    have hdvd : A ∣ P*U := (modByMonic_eq_zero_iff_dvd hm).mp hz
    have hap : A ∣ P := hu.dvd_of_dvd_mul_right hdvd
    have hle := natDegree_le_of_dvd hap hp
    omega
  have hrd : R.natDegree<A.natDegree := natDegree_modByMonic_lt (P*U) hm ha
  obtain ⟨Q,hq,hdetect⟩ := topCoefficientDetects A R hm hr hrd
  refine ⟨Q,hq,?_⟩
  have hdifference : A ∣ P*U-R := by
    refine ⟨(P*U) /ₘ A, ?_⟩
    dsimp [R]
    rw [modByMonic_eq_sub_mul_div]
    ring
  have hproduct : A ∣ P*U*Q-R*Q := by
    obtain ⟨T,ht⟩ := hdifference
    refine ⟨T*Q, ?_⟩
    rw [← sub_mul,ht,mul_assoc]
  have heq := modByMonic_eq_of_dvd_sub hm hproduct
  rw [heq]
  exact hdetect

local instance : Fact (Nat.Prime 17) := ⟨by decide⟩
noncomputable def modulus : Polynomial (ZMod 17) := (X-C 16)*(X-C 9)
noncomputable def curve : Polynomial (ZMod 17) := X^3+C 2*X+C 2
noncomputable def unitInverse : Polynomial (ZMod 17) := C 7*X+C 6
noncomputable def normA : Polynomial (ZMod 17) := C 5*X^2+C 11*X+C 6
noncomputable def target : Polynomial (ZMod 17) := X*(X-C 5)*(X-C 6)*(X-C 3)
noncomputable def moment (P : Polynomial (ZMod 17)) : ZMod 17 :=
  ((P*unitInverse) %ₘ modulus).coeff 1

theorem modulusMonic : modulus.Monic :=
  (monic_X_sub_C (16:ZMod 17)).mul (monic_X_sub_C 9)
theorem modulusDegree : modulus.degree=2 := by norm_num [modulus,degree_mul]
theorem modulusNatDegree : modulus.natDegree=2 :=
  natDegree_eq_of_degree_eq_some modulusDegree
theorem modulusNeOne : modulus≠1 := by
  intro h
  have hd := congrArg Polynomial.degree h
  rw [modulusDegree] at hd
  norm_num at hd

theorem inverseIdentity : curve*unitInverse-1 =
    modulus*(C 7*X^2+C 11*X+C 12) := by
  simp only [curve,unitInverse,modulus,map_ofNat]
  ring_nf
  reduce_mod_char
  all_goals ring_nf
  all_goals reduce_mod_char

theorem actualNorm : normA^2-curve=C 8*target := by
  simp only [normA,curve,target,map_ofNat]
  ring_nf
  reduce_mod_char
  all_goals ring_nf
  all_goals reduce_mod_char

/-- The test's quotient-ring definition agrees with this evaluation functional for every P. -/
theorem momentEvaluation (P : Polynomial (ZMod 17)) :
    moment P=12*(P.eval 16+P.eval 9) := by
  let R := (P*unitInverse) %ₘ modulus
  have hr : R.natDegree≤1 := by
    have h := natDegree_modByMonic_lt (P*unitInverse) modulusMonic modulusNeOne
    rw [modulusNatDegree] at h
    exact Nat.le_of_lt_succ h
  have hform := eq_X_add_C_of_natDegree_le_one hr
  have ha16 : modulus.eval 16=0 := by
    simp only [modulus,eval_mul,eval_sub,eval_X,eval_C]
    decide
  have ha9 : modulus.eval 9=0 := by
    simp only [modulus,eval_mul,eval_sub,eval_X,eval_C]
    decide
  have hu16 : unitInverse.eval 16=16 := by
    simp only [unitInverse,eval_add,eval_mul,eval_C,eval_X]
    decide
  have hu9 : unitInverse.eval 9=1 := by
    simp only [unitInverse,eval_add,eval_mul,eval_C,eval_X]
    decide
  have h16 : R.eval 16=P.eval 16*16 := by
    simp only [R,modByMonic_eq_sub_mul_div,eval_sub,eval_mul,ha16,hu16,
      zero_mul,sub_zero]
  have h9 : R.eval 9=P.eval 9 := by
    simp only [R,modByMonic_eq_sub_mul_div,eval_sub,eval_mul,ha9,hu9,
      zero_mul,sub_zero,mul_one]
  rw [hform] at h16 h9
  simp only [eval_add,eval_mul,eval_C,eval_X] at h16 h9
  have hdiff : (7:ZMod 17)*R.coeff 1=16*P.eval 16-P.eval 9 := by
    linear_combination h16-h9
  have hs := congrArg (fun z : ZMod 17 => 5*z) hdiff
  ring_nf at hs
  reduce_mod_char at hs
  ring_nf at hs
  reduce_mod_char at hs
  change R.coeff 1=12*(P.eval 16+P.eval 9)
  linear_combination hs

theorem momentValues :
    moment 1=7 ∧ moment X=11 ∧ moment (X^2)=15 ∧
    moment (X^3)=15 ∧ moment (X^4)=0 := by
  simp only [momentEvaluation,eval_one,eval_X,eval_pow]
  decide

theorem momentCurve : moment curve=0 := by
  simp only [momentEvaluation,curve,eval_add,eval_pow,eval_mul,eval_C,eval_X]
  decide

noncomputable def fullDual : Matrix (Fin 4) (Fin 4) (ZMod 17) :=
  !![moment 1,moment X,moment (X^2),0;
     moment X,moment (X^2),moment (X^3),0;
     moment (X^2),moment (X^3),moment (X^4),0;
     0,0,0,-moment curve]
noncomputable def branchBasis : Matrix (Fin 4) (Fin 3) (ZMod 17) :=
  !![0,0,6;1,0,0;0,1,0;0,0,1]
noncomputable def forms : Fin 3 → Matrix (Fin 3) (Fin 3) (ZMod 17) :=
  ![!![0,1,0;1,-3,0;0,0,-1],
    !![1,0,0;0,-12,6;0,6,0],
    !![0,0,6;0,-12,0;6,0,-2]]
noncomputable def candidate : Matrix (Fin 3) (Fin 3) (ZMod 17) :=
  !![15,15,15;15,0,5;15,5,14]

theorem momentRestriction : branchBasis.transpose*fullDual*branchBasis=candidate := by
  simp only [fullDual,momentValues.1,momentValues.2.1,momentValues.2.2.1,
    momentValues.2.2.2.1,momentValues.2.2.2.2,momentCurve]
  ext i j
  fin_cases i <;> fin_cases j <;> decide

theorem inActualPencil : (15:ZMod 17) • forms 0+15 • forms 1+11 • forms 2=candidate := by
  ext i j
  fin_cases i <;> fin_cases j <;> decide

theorem candidateSymmetric : candidate.IsSymm := by
  ext i j
  fin_cases i <;> fin_cases j <;> decide

theorem candidateDet : candidate.det=0 := by decide

theorem candidateRank : candidate.rank=2 := by
  let A : Matrix (Fin 3) (Fin 2) (ZMod 17) := !![15,15;15,0;15,5]
  let B : Matrix (Fin 2) (Fin 3) (ZMod 17) := !![1,0,6;0,1,12]
  have hf : candidate=A*B := by
    ext i j
    fin_cases i <;> fin_cases j <;> decide
  have hu : candidate.rank≤2 := by
    rw [hf]
    exact (Matrix.rank_mul_le_left A B).trans (Matrix.rank_le_width A)
  let T : Matrix (Fin 2) (Fin 2) (ZMod 17) :=
    candidate.submatrix ![0,1] ![0,1]
  have ht : T.det≠0 := by decide
  have hr : T.rank=2 := by simpa using Matrix.rank_of_det_ne_zero ht
  have hl := Matrix.rank_submatrix_le candidate
    (![0,1] : Fin 2 → Fin 3) (![0,1] : Fin 2 → Fin 3)
  change T.rank≤candidate.rank at hl
  rw [hr] at hl
  omega

theorem candidateStationary :
    ∀ i, NormDualPencil.pairing (forms i) candidate.adjugate=0 := by
  intro i
  fin_cases i <;> rw [NormDualPencil.pairing.eq_def] <;> decide

theorem candidatePivot : candidate.adjugate 2 2≠0 := by decide

theorem recoveredCoordinates :
    candidate.adjugate 0 2/candidate.adjugate 2 2=11 ∧
    candidate.adjugate 1 2/candidate.adjugate 2 2=5 := by
  constructor <;> apply (div_eq_iff candidatePivot).2 <;> decide

theorem genericRecoveryApplies : ∃ w : Fin 3 → ZMod 17,
    (∃ i, w i ≠ 0) ∧ ∀ i, NormDualPencil.quadratic (forms i) w=0 := by
  apply NormDualPencil.recoverQuadratic forms candidate candidateSymmetric candidateDet
  · simp only [candidateRank,Fintype.card_fin]
  · exact ⟨2,2,candidatePivot⟩
  · exact candidateStationary
/-- Exact ranks of both the full moment form and its branch restriction are checked. -/
theorem fullDualRank : fullDual.rank=2 := by
  let A : Matrix (Fin 4) (Fin 2) (ZMod 17) := !![7,11;11,15;15,15;0,0]
  let B : Matrix (Fin 2) (Fin 4) (ZMod 17) := !![1,0,9,0;0,1,8,0]
  have hf : fullDual=A*B := by
    simp only [fullDual,momentValues.1,momentValues.2.1,momentValues.2.2.1,
      momentValues.2.2.2.1,momentValues.2.2.2.2,momentCurve]
    ext i j
    fin_cases i <;> fin_cases j <;> decide
  have hu : fullDual.rank≤2 := by
    rw [hf]
    exact (Matrix.rank_mul_le_left A B).trans (Matrix.rank_le_width A)
  have hl : candidate.rank≤fullDual.rank := by
    rw [← momentRestriction]
    exact (Matrix.rank_mul_le_left (branchBasis.transpose*fullDual) branchBasis).trans
      (Matrix.rank_mul_le_right branchBasis.transpose fullDual)
  rw [candidateRank] at hl
  omega

/-- The supplied matrices really encode the three norm coefficient equations. -/
theorem formsEvaluate (s t u : ZMod 17) :
    NormDualPencil.quadratic (forms 0) ![s,t,u]=2*t*s-u^2-3*t^2 ∧
    NormDualPencil.quadratic (forms 1) ![s,t,u]=s^2+12*t*u-12*t^2 ∧
    NormDualPencil.quadratic (forms 2) ![s,t,u]=12*s*u-2*u^2-12*t^2 := by
  simp [NormDualPencil.quadratic.eq_def,forms,Fin.sum_univ_three]
  constructor <;> (try constructor) <;> ring_nf <;> reduce_mod_char <;>
    ring_nf <;> reduce_mod_char

theorem normQuadricsExpansion (s t u : ZMod 17) :
    (C t*X^2+C s*X+C (6*u))^2-curve*(C u)^2-C (t^2)*target=
      C (2*t*s-u^2-3*t^2)*X^3+C (s^2+12*t*u-12*t^2)*X^2+
        C (12*s*u-2*u^2-12*t^2)*X := by
  simp only [curve,target,map_add,map_sub,map_mul,map_pow,map_ofNat]
  ring_nf
  reduce_mod_char
  all_goals ring_nf
  all_goals reduce_mod_char

/-- The actual normalized adjugate column gives the intended norm, not just an abstract quadric. -/
theorem recoveredActualNorm :
    (C (candidate.adjugate 1 2/candidate.adjugate 2 2)*X^2+
      C (candidate.adjugate 0 2/candidate.adjugate 2 2)*X+C 6)^2-curve=
        C 8*target := by
  rw [recoveredCoordinates.1,recoveredCoordinates.2]
  exact actualNorm
/-- The complete predicted adjugate is checked, not just its normalized column. -/
theorem candidateAdjugate : candidate.adjugate=!![9,1,7;1,2,14;7,14,13] := by
  rw [Matrix.adjugate_fin_three]
  ext i j
  fin_cases i <;> fin_cases j <;> decide
end NormQuotientMoment
#print axioms NormQuotientMoment.topCoefficientDetects
#print axioms NormQuotientMoment.inverseIdentity
#print axioms NormQuotientMoment.actualNorm
#print axioms NormQuotientMoment.momentEvaluation
#print axioms NormQuotientMoment.momentRestriction
#print axioms NormQuotientMoment.inActualPencil
#print axioms NormQuotientMoment.candidateRank
#print axioms NormQuotientMoment.candidateStationary
#print axioms NormQuotientMoment.recoveredCoordinates
#print axioms NormQuotientMoment.genericRecoveryApplies
#print axioms NormQuotientMoment.fullDualRank
#print axioms NormQuotientMoment.formsEvaluate
#print axioms NormQuotientMoment.normQuadricsExpansion
#print axioms NormQuotientMoment.recoveredActualNorm
#print axioms NormQuotientMoment.twistedCoefficientDetects
#print axioms NormQuotientMoment.candidateAdjugate
