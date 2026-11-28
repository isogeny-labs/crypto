import SplitInduction

namespace SemaevGeneralSplitAudit
open Polynomial SemaevSymmetryAudit
noncomputable section
variable {K : Type*} [Field K]

def arityCast (N : ℕ) (h : 3≤N) : Fin (N-3+3) → Fin N :=
  Fin.cast (Nat.sub_add_cancel h)

def arityPolynomial (W : WeierstrassCurve.Affine K) (N : ℕ) (h : 3≤N) :
    MvPolynomial (Fin N) K :=
  MvPolynomial.rename (arityCast N h) (canonicalMv W (N-3))

theorem canonicalMv_cast (W : WeierstrassCurve.Affine K) {n m : ℕ} (h : n=m) :
    MvPolynomial.rename (Fin.cast (congrArg (fun a => a+3) h)) (canonicalMv W n)=
      canonicalMv W m := by
  cases h
  simp

theorem universal_listValue (W : WeierstrassCurve.Affine K) (N : ℕ) (h : 3≤N)
    (hf : 3≤(List.ofFn (MvPolynomial.X : Fin N → MvPolynomial (Fin N) K)).length) :
    listValue W MvPolynomial.C (List.ofFn MvPolynomial.X) hf=arityPolynomial W N h := by
  obtain ⟨n,hn⟩ := Nat.exists_eq_add_of_le h
  have he : N=n+3 := by omega
  clear hn
  subst N
  rw [listValue_ofFn W MvPolynomial.C n MvPolynomial.X hf,sourceValue_as_mv,
    MvPolynomial.eval₂_eta]
  exact (canonicalMv_cast W (Nat.add_sub_cancel n 3)).symm

def leftVariables (K : Type*) [Field K] (m n : ℕ) :
    Fin (m+2) → MvPolynomial (Fin ((m+2)+(n+2))) K :=
  fun i => MvPolynomial.X (Fin.castAdd (n+2) i)

def rightVariables (K : Type*) [Field K] (m n : ℕ) :
    Fin (n+2) → MvPolynomial (Fin ((m+2)+(n+2))) K :=
  fun i => MvPolynomial.X (Fin.natAdd (m+2) i)

theorem variable_blocks (m n : ℕ) :
    List.ofFn (leftVariables K m n)++List.ofFn (rightVariables K m n)=
      List.ofFn (MvPolynomial.X : Fin ((m+2)+(n+2)) → MvPolynomial _ K) := by
  rw [← List.ofFn_fin_append]
  congr 1
  funext i
  refine Fin.addCases (fun j => ?_) (fun j => ?_) i <;>
    simp [leftVariables,rightVariables]

def leftFactor (W : WeierstrassCurve.Affine K) (m n : ℕ) :
    Polynomial (MvPolynomial (Fin ((m+2)+(n+2))) K) :=
  sourceKernel W m MvPolynomial.C (leftVariables K m n)

def rightFactor (W : WeierstrassCurve.Affine K) (m n : ℕ) :
    Polynomial (MvPolynomial (Fin ((m+2)+(n+2))) K) :=
  sourceKernel W n MvPolynomial.C (rightVariables K m n)

theorem blockArityProof (m n : ℕ) : 3≤(m+2)+(n+2) := by omega

/-- Literal polynomial identity in independent variables, not a finite-field
pointwise inference. The factors are the actual canonical source kernels. -/
theorem universal_prescribed_split (W : WeierstrassCurve.Affine K) (m n : ℕ) :
    Polynomial.resultant (leftFactor W m n) (rightFactor W m n)
      (2^(m+1)) (2^(n+1))=
      arityPolynomial W ((m+2)+(n+2)) (blockArityProof m n) := by
  have hx : 2≤(List.ofFn (leftVariables K m n)).length := by simp
  have hy : 2≤(List.ofFn (rightVariables K m n)).length := by simp
  have h := all_list_splits W MvPolynomial.C (List.ofFn (rightVariables K m n))
    (List.ofFn (leftVariables K m n)) hx hy
  rw [listKernel_ofFn,listKernel_ofFn] at h
  have hm : m+2-1=m+1 := by omega
  have hn : n+2-1=n+1 := by omega
  simp only [kernelSize,List.length_ofFn,hm,hn] at h
  have hj : 3≤(List.ofFn (leftVariables K m n)++List.ofFn (rightVariables K m n)).length := by
    simp only [List.length_append,List.length_ofFn]; omega
  have hu : 3≤(List.ofFn (MvPolynomial.X :
      Fin ((m+2)+(n+2)) → MvPolynomial _ K)).length := by simp; omega
  calc
    Polynomial.resultant (leftFactor W m n) (rightFactor W m n)
        (2^(m+1)) (2^(n+1))=
      listValue W MvPolynomial.C
        (List.ofFn (leftVariables K m n)++List.ofFn (rightVariables K m n)) hj := h
    _ = listValue W MvPolynomial.C (List.ofFn MvPolynomial.X) hu :=
      listValue_congr W MvPolynomial.C (variable_blocks m n) hj hu
    _ = arityPolynomial W ((m+2)+(n+2)) (blockArityProof m n) :=
      universal_listValue W _ (blockArityProof m n) hu

#print axioms universal_prescribed_split
end
end SemaevGeneralSplitAudit
