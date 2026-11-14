module
public import RankOneExtraction
public import Mathlib.Algebra.Polynomial.Coeff
public section
namespace NormLift
open Polynomial
variable {K I : Type*} [Field K] [Fintype I]

noncomputable def assemble (A : I → Polynomial K) (w : I → K) : Polynomial K :=
  ∑ i, w i • A i

noncomputable def norm (f : Polynomial K) (A B : I → Polynomial K) (w : I → K) :
    Polynomial K := (assemble A w)^2 - f*(assemble B w)^2

/-- The actual linear lift of the polynomial curve norm, not an assumed map. -/
noncomputable def lift (f : Polynomial K) (A B : I → Polynomial K) :
    (I → I → K) →ₗ[K] Polynomial K where
  toFun M := ∑ i, ∑ j, M i j • (A i*A j - f*(B i*B j))
  map_add' M N := by simp [add_smul, Finset.sum_add_distrib]
  map_smul' a M := by simp [Finset.smul_sum, smul_smul]

/-- Substituting a vector outer product recovers the exact polynomial norm. -/
theorem outerNorm (f : Polynomial K) (A B : I → Polynomial K) (w : I → K) :
    lift f A B (fun i j => w i*w j) = norm f A B w := by
  simp only [lift, norm, assemble, LinearMap.coe_mk, AddHom.coe_mk,
    Polynomial.smul_eq_C_mul, map_mul, pow_two, mul_sub,
    Finset.sum_sub_distrib, Finset.sum_mul, Finset.mul_sum]
  congr 1
  · apply Finset.sum_congr rfl
    intro i hi
    apply Finset.sum_congr rfl
    intro j hj
    ring
  · apply Finset.sum_congr rfl
    intro i hi
    apply Finset.sum_congr rfl
    intro j hj
    ring

noncomputable def branch (A B : I → Polynomial K) (q y : K) (i : I) : K :=
  (A i).eval q - y*(B i).eval q

/-- The finite linear branch equation is precisely A(q)-y B(q)=0. -/
theorem branchEval (A B : I → Polynomial K) (q y : K) (w : I → K) :
    (assemble A w).eval q - y*(assemble B w).eval q =
      ∑ i, w i * branch A B q y i := by
  simp only [assemble, branch, Polynomial.smul_eq_C_mul,
    eval_finsetSum, eval_mul, eval_C, Finset.mul_sum,
    mul_sub, Finset.sum_sub_distrib]
  congr 1
  apply Finset.sum_congr rfl
  intro i hi
  ring
/-- A genuine polynomial witness includes the actual target-branch evaluation. -/
def NormWitness (f G : Polynomial K) (A B : I → Polynomial K) (q y : K) : Prop :=
  ∃ (w : I → K) (c : K), c ≠ 0 ∧
    (assemble A w).eval q - y*(assemble B w).eval q = 0 ∧
    norm f A B w = c • G

/-- Explicit equations, not a numerical-rank oracle or a relaxed linear system. -/
def MatrixWitness (f G : Polynomial K) (A B : I → Polynomial K) (q y : K) : Prop :=
  ∃ (M : I → I → K) (c : K), c ≠ 0 ∧ (∃ i j, M i j ≠ 0) ∧
    (∀ i j, M i j = M j i) ∧ RankOneExtraction.MinorsVanish M ∧
    (∀ i, ∑ j, M i j * branch A B q y j = 0) ∧ lift f A B M = c • G

/-- A nonzero prescribed norm excludes the zero coefficient vector. -/
theorem witnessVectorNonzero (f G : Polynomial K) (A B : I → Polynomial K)
    (w : I → K) (c : K) (hc : c ≠ 0) (hG : G ≠ 0)
    (hn : norm f A B w = c • G) : ∃ i, w i ≠ 0 := by
  by_contra h
  have hz : ∀ i, w i = 0 := by
    intro i
    by_contra hi
    exact h ⟨i,hi⟩
  have hzero : norm f A B w = 0 := by simp [norm,assemble,hz]
  exact (smul_ne_zero hc hG) (hn.symm.trans hzero)

/-- Lifting a norm witness supplies every matrix constraint, including minors. -/
theorem normToMatrix (f G : Polynomial K) (A B : I → Polynomial K)
    (q y : K) (hG : G ≠ 0) :
    NormWitness f G A B q y → MatrixWitness f G A B q y := by
  rintro ⟨w,c,hc,hbranch,hn⟩
  obtain ⟨k,hk⟩ := witnessVectorNonzero f G A B w c hc hG hn
  refine ⟨(fun i j => w i*w j), c, hc, ⟨k,k,mul_ne_zero hk hk⟩,
    (fun i j => mul_comm _ _), ?_, ?_, ?_⟩
  · exact RankOneExtraction.outerMinors w
  · intro i
    calc
      ∑ j, (w i*w j)*branch A B q y j =
          w i * ∑ j, w j*branch A B q y j := by
        rw [Finset.mul_sum]
        apply Finset.sum_congr rfl
        intro j hj
        ring
      _ = 0 := by rw [← branchEval, hbranch, mul_zero]
  · rw [outerNorm]
    exact hn

/-- A supplied constrained matrix gives a genuine prescribed-norm witness. -/
theorem matrixToNorm (f G : Polynomial K) (A B : I → Polynomial K)
    (q y : K) : MatrixWitness f G A B q y → NormWitness f G A B q y := by
  rintro ⟨M,c,hc,hn,hsym,hmin,hbranch,hL⟩
  obtain ⟨k,hk⟩ := RankOneExtraction.diagonalPivot M hsym hmin hn
  let w : I → K := fun i => M i k
  let s : K := (M k k)⁻¹
  have hs : s ≠ 0 := inv_ne_zero hk
  have hf : M = s • (fun i j => w i*w j) := by
    funext i j
    exact RankOneExtraction.pivotFactor M hsym hmin k hk i j
  refine ⟨w, c/s, div_ne_zero hc hs, ?_, ?_⟩
  · rw [branchEval]
    exact RankOneExtraction.extractBranch M (branch A B q y) hsym hbranch k
  · rw [← outerNorm]
    exact RankOneExtraction.extractLinearImage (lift f A B) M w s c G hs hf hL

/-- Exact two-way polynomial witness equivalence for a nonzero prescribed norm. -/
theorem witnessEquivalence (f G : Polynomial K) (A B : I → Polynomial K)
    (q y : K) (hG : G ≠ 0) :
    NormWitness f G A B q y ↔ MatrixWitness f G A B q y :=
  ⟨normToMatrix f G A B q y hG, matrixToNorm f G A B q y⟩
end NormLift
#print axioms NormLift.outerNorm
#print axioms NormLift.branchEval
#print axioms NormLift.witnessVectorNonzero
#print axioms NormLift.normToMatrix
#print axioms NormLift.matrixToNorm
#print axioms NormLift.witnessEquivalence
#print axioms RankOneExtraction.outerMinors
