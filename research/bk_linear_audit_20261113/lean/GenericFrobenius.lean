import Mathlib.LinearAlgebra.Matrix.Charpoly.Coeff
import Mathlib.LinearAlgebra.Matrix.ToLinearEquiv
import Mathlib.Algebra.Polynomial.RingDivision
import Mathlib.Tactic

namespace BKLinearAudit
open Matrix Polynomial
noncomputable section
variable {K : Type*} [Field K]
abbrev M2 (K : Type*) := Matrix (Fin 2) (Fin 2) K

/-- Actual kernel/determinant identities force the source characteristic
polynomial; no trace or diagonalization premise is supplied. -/
theorem trace_from_fixed_vector (A : M2 K) (q : K)
    (hdet : A.det=q) (v : Fin 2 → K) (hv : v≠0) (hfix : A*ᵥv=v) :
    A.trace=1+q := by
  have hker : (A-1)*ᵥv=0 := by rw [sub_mulVec,hfix,one_mulVec,sub_self]
  have hz : (A-1).det=0 := exists_mulVec_eq_zero_iff.mp ⟨v,hv,hker⟩
  rw [det_fin_two] at hz hdet
  have h01 : (0:Fin 2)≠1 := by decide +kernel
  have h10 : (1:Fin 2)≠0 := by decide +kernel
  simp only [Matrix.sub_apply,one_apply,h01,h10,ite_false,
    ite_true,sub_zero] at hz
  rw [trace_fin_two]
  linear_combination hdet-hz

theorem charpoly_from_fixed_vector (A : M2 K) (q : K)
    (hdet : A.det=q) (v : Fin 2 → K) (hv : v≠0) (hfix : A*ᵥv=v) :
    A.charpoly=(X-C 1)*(X-C q) := by
  rw [charpoly_fin_two,trace_from_fixed_vector A q hdet v hv hfix,hdet]
  simp only [map_add,map_one]
  ring

theorem distinct_factors_divide_power (q : K) (hq : q≠1) (k : ℕ)
    (hk : q^k=1) : (X-C 1)*(X-C q)∣(X^k-1 : Polynomial K) := by
  have hc : IsCoprime (X-C (1:K)) (X-C q) :=
    isCoprime_X_sub_C_of_isUnit_sub (sub_ne_zero.mpr hq.symm).isUnit
  apply hc.mul_dvd
  · apply Polynomial.dvd_iff_isRoot.mpr
    simp [Polynomial.IsRoot]
  · apply Polynomial.dvd_iff_isRoot.mpr
    simp [Polynomial.IsRoot,hk]

/-- Exact arbitrary-field linear step in qualified BK. The geometric
torsion dimension/determinant and descent bridges are separate obligations. -/
theorem power_identity_iff (A : M2 K) (q : K) (hq : q≠1)
    (hdet : A.det=q) (v : Fin 2 → K) (hv : v≠0) (hfix : A*ᵥv=v) (k : ℕ) :
    A^k=1 ↔ q^k=1 := by
  constructor
  · intro h
    have hd := congrArg Matrix.det h
    simpa only [det_pow,hdet,det_one] using hd
  · intro hk
    have hc := charpoly_from_fixed_vector A q hdet v hv hfix
    have hdiv : A.charpoly∣(X^k-1 : Polynomial K) := by
      rw [hc]
      exact distinct_factors_divide_power q hq k hk
    have he : aeval A (X^k-1 : Polynomial K)=0 :=
      Polynomial.aeval_eq_zero_of_dvd_aeval_eq_zero hdiv A.aeval_self_charpoly
    simpa only [map_sub,map_pow,aeval_X,map_one,sub_eq_zero] using he

#print axioms power_identity_iff
end
end BKLinearAudit
