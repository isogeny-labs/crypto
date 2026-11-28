import GenericFrobenius
import Mathlib.Data.ZMod.Basic

namespace BKLinearAudit
open Matrix
noncomputable section
variable {R : Type*} [CommRing R]

def jordan (a : R) : M2 R := !![1,a;0,1]

theorem jordan_zero : jordan (0:R)=1 := by
  ext i j
  fin_cases i <;> fin_cases j <;> simp [jordan]

theorem jordan_mul (a b : R) : jordan a*jordan b=jordan (a+b) := by
  ext i j
  fin_cases i <;> fin_cases j <;>
    simp [jordan,Matrix.mul_apply,Fin.sum_univ_two,add_comm]

theorem jordan_eq_one_iff (a : R) : jordan a=1 ↔ a=0 := by
  constructor
  · intro h
    have hh := congrArg (fun A : M2 R => A 0 1) h
    simpa [jordan] using hh
  · intro h
    rw [h,jordan_zero]

theorem jordan_det (a : R) : (jordan a).det=1 := by simp [jordan,det_fin_two]

theorem jordan_fixed_vector (a : R) : jordan a*ᵥ ![1,0]= ![1,0] := by
  ext i
  fin_cases i <;> simp [jordan,Matrix.mulVec,dotProduct,Fin.sum_univ_two]

theorem jordan_one_pow (k : ℕ) : jordan (1:R)^k=jordan (k:R) := by
  induction k with
  | zero => simp [jordan_zero]
  | succ k ih =>
      rw [pow_succ,ih,jordan_mul]
      simp only [Nat.cast_add,Nat.cast_one]

theorem jordan_one_pow_eq_one_iff (k : ℕ) : jordan (1:R)^k=1 ↔ (k:R)=0 := by
  rw [jordan_one_pow,jordan_eq_one_iff]

theorem jordan_modulus_period (n k : ℕ) : jordan (1:ZMod n)^k=1 ↔ n∣k := by
  rw [jordan_one_pow_eq_one_iff,ZMod.natCast_eq_zero_iff]

theorem first_vector_ne_zero [Nontrivial R] : (![1,0] : Fin 2 → R)≠0 := by
  intro h
  have he := congrArg (fun v : Fin 2 → R => v 0) h
  simp at he

/-- For every prime modulus the cyclotomic condition q^k=1 with q=1
always holds, while the actual Jordan matrix is fixed only at multiples of r. -/
theorem jordan_prime_boundary (r : ℕ) (hr : r.Prime) :
    (∀ k : ℕ,(1:ZMod r)^k=1 ∧ (jordan (1:ZMod r)^k=1 ↔ r∣k)) ∧
    (jordan (1:ZMod r)).det=1 ∧
    ((![1,0] : Fin 2 → ZMod r)≠0 ∧ jordan (1:ZMod r)*ᵥ ![1,0]= ![1,0]) ∧
    ¬ (∀ k : ℕ,jordan (1:ZMod r)^k=1 ↔ (1:ZMod r)^k=1) := by
  have : Fact r.Prime := ⟨hr⟩
  refine ⟨fun k => ⟨one_pow k,jordan_modulus_period r k⟩,jordan_det 1,
    ⟨first_vector_ne_zero,jordan_fixed_vector 1⟩,?_⟩
  intro h
  have hk := (jordan_modulus_period r 1).mp ((h 1).mpr (one_pow 1))
  exact hr.not_dvd_one hk

#print axioms jordan_prime_boundary
end
end BKLinearAudit
