import RecodedCanonical

noncomputable section
open MvPolynomial
namespace DeterminantalTop
variable {K : Type*} [Field K]
local notation "P" => MvPolynomial (Fin 13) K
set_option maxHeartbeats 300000

def quadratic (r a b : K) : Fin 10 → P :=
  ![X 2^2, X 0*X 1-C r*X 2^2, -(X 2*X 0), -(X 2*X 1),
    X 0*X 9-C a*X 3*X 10, X 1*X 11-C b*X 5*X 12,
    X 3^2,X 4^2,X 5^2,X 6^2]
def linear (a b : K) : Fin 4 → P :=
  ![C a*X 0+X 9, X 4+X 10, C b*X 1+X 11, X 6+X 12]
def generators (r a b : K) (c : Fin 10 → K) (v : Fin 10 → Fin 13 → K) : Set P :=
  Set.range (fun i => quadratic r a b i+RecodedCanonical.affine (c i) (v i)) ∪
    Set.range (linear a b)

theorem quadratic_homogeneous (r a b : K) (i : Fin 10) :
    (quadratic r a b i).IsHomogeneous 2 := by
  fin_cases i
  · exact isHomogeneous_X_pow _ _
  · exact ((isHomogeneous_X K 0).mul (isHomogeneous_X K 1)).sub
      ((isHomogeneous_X_pow _ _).C_mul r)
  · exact ((isHomogeneous_X K 2).mul (isHomogeneous_X K 0)).neg
  · exact ((isHomogeneous_X K 2).mul (isHomogeneous_X K 1)).neg
  · change (X 0*X 9-C a*X 3*X 10 : P).IsHomogeneous 2
    simpa only [mul_assoc] using ((isHomogeneous_X K 0).mul (isHomogeneous_X K 9)).sub
      ((((isHomogeneous_X K 3).mul (isHomogeneous_X K 10)).C_mul a))
  · change (X 1*X 11-C b*X 5*X 12 : P).IsHomogeneous 2
    simpa only [mul_assoc] using ((isHomogeneous_X K 1).mul (isHomogeneous_X K 11)).sub
      ((((isHomogeneous_X K 5).mul (isHomogeneous_X K 12)).C_mul b))
  all_goals exact isHomogeneous_X_pow _ _

theorem quadratic_ne_zero (r a b : K) (i : Fin 10) : quadratic r a b i ≠ 0 := by
  let probes : Fin 10 → Fin 13 → K :=
    ![ ![0,0,1,0,0,0,0,0,0,0,0,0,0],
       ![1,1,0,0,0,0,0,0,0,0,0,0,0],
       ![1,0,1,0,0,0,0,0,0,0,0,0,0],
       ![0,1,1,0,0,0,0,0,0,0,0,0,0],
       ![1,0,0,0,0,0,0,0,0,1,0,0,0],
       ![0,1,0,0,0,0,0,0,0,0,0,1,0],
       ![0,0,0,1,0,0,0,0,0,0,0,0,0],
       ![0,0,0,0,1,0,0,0,0,0,0,0,0],
       ![0,0,0,0,0,1,0,0,0,0,0,0,0],
       ![0,0,0,0,0,0,1,0,0,0,0,0,0] ]
  intro h
  have hh := congrArg (eval (probes i)) h
  fin_cases i <;> simp [quadratic,probes] at hh

theorem linear_homogeneous (a b : K) (i : Fin 4) :
    (linear a b i).IsHomogeneous 1 := by
  fin_cases i
  · exact ((isHomogeneous_X K 0).C_mul a).add (isHomogeneous_X K 9)
  · exact (isHomogeneous_X K 4).add (isHomogeneous_X K 10)
  · exact ((isHomogeneous_X K 1).C_mul b).add (isHomogeneous_X K 11)
  · exact (isHomogeneous_X K 6).add (isHomogeneous_X K 12)

theorem linear_ne_zero (a b : K) (i : Fin 4) : linear a b i ≠ 0 := by
  intro h
  have hh := congrArg (eval (![0,0,0,0,0,0,0,0,0,1,1,1,1] : Fin 13 → K)) h
  fin_cases i <;> simp [linear] at hh

theorem linear_top (a b : K) (i : Fin 4) : DegreeBridge.top (linear a b i) = linear a b i := by
  unfold DegreeBridge.top
  rw [(linear_homogeneous a b i).totalDegree (linear_ne_zero a b i)]
  simpa using homogeneousComponent_of_mem (linear_homogeneous a b i)

def free : P →+* K := eval ![0,0,0,0,0,0,0,1,0,0,0,0,0]
@[simp] theorem free_X (i : Fin 13) :
    free (X i) = (![0,0,0,0,0,0,0,1,0,0,0,0,0] i : K) := eval_X _
@[simp] theorem free_C (c : K) : free (C c) = c := eval_C _

theorem top_ideal_in_kernel (r a b : K) (c : Fin 10 → K)
    (v : Fin 10 → Fin 13 → K) :
    DegreeBridge.topIdeal (generators r a b c v) ≤ RingHom.ker free := by
  apply Ideal.span_le.mpr
  rintro p ⟨q,hq,rfl⟩
  rcases hq with ⟨i,rfl⟩|⟨i,rfl⟩
  · rw [RecodedCanonical.split_top _ (quadratic_homogeneous r a b i)
      (quadratic_ne_zero r a b i)]
    change free (quadratic r a b i) = 0
    fin_cases i <;> simp [quadratic]
  · rw [linear_top]
    change free (linear a b i) = 0
    fin_cases i <;> simp [linear]

theorem ordinate_power_not_mem (r a b : K) (c : Fin 10 → K)
    (v : Fin 10 → Fin 13 → K) (d : ℕ) :
    (X 7 : P)^d ∉ DegreeBridge.topIdeal (generators r a b c v) := by
  intro h
  have hh := top_ideal_in_kernel r a b c v h
  change free ((X 7 : P)^d) = 0 at hh
  have hf : free (X 7 : P) = 1 := by rw [free_X]; rfl
  rw [_root_.map_pow,hf,one_pow] at hh
  exact one_ne_zero hh

theorem no_finite_vanishing (r a b : K) (c : Fin 10 → K)
    (v : Fin 10 → Fin 13 → K) (d : ℕ) :
    ¬SecantIdeal.Vanishes (DegreeBridge.topIdeal (generators r a b c v)) d := by
  intro h
  exact ordinate_power_not_mem r a b c v d (h _ (isHomogeneous_X_pow 7 d))

end DeterminantalTop
