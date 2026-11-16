import Mathlib.Tactic

/-! Linear-algebra core only: the bridge identifying elliptic r-torsion with
this triangular model of Frobenius is not assumed proved by this file. -/
namespace TheoremAudit
variable {K : Type*} [Field K]

def triangular (q b : K) (v : K×K) : K×K := (v.1+b*v.2,q*v.2)

theorem triangular_iterate (q b : K) (k : ℕ) (v : K×K) :
    (triangular q b)^[k] v =
      (v.1+b*(∑ i ∈ Finset.range k, q^i)*v.2,q^k*v.2) := by
  induction k with
  | zero => simp
  | succ k ih =>
    rw [Function.iterate_succ_apply',ih]
    simp only [triangular,Finset.sum_range_succ,pow_succ]
    ext <;> simp only <;> ring

theorem geometric_sum_identity (q : K) (k : ℕ) :
    (q-1)*(∑ i ∈ Finset.range k,q^i)=q^k-1 := by
  induction k with
  | zero => simp
  | succ k ih =>
    rw [Finset.sum_range_succ,pow_succ]
    linear_combination ih

/-- This is the exact reason the q != 1 condition removes the Jordan issue. -/
theorem triangular_iterate_identity_iff (q b : K) (hq : q≠1) (k : ℕ) :
    (∀ v, (triangular q b)^[k] v=v) ↔ q^k=1 := by
  constructor
  · intro h
    have hh := congrArg Prod.snd (h (0,1))
    simpa [triangular_iterate] using hh
  · intro hk v
    have hs : (∑ i ∈ Finset.range k,q^i)=0 := by
      have hh := geometric_sum_identity q k
      rw [hk,sub_self] at hh
      exact (mul_eq_zero.mp hh).resolve_left (sub_ne_zero.mpr hq)
    simp [triangular_iterate,hs,hk]

/-- In the excluded unipotent case, q^1=1 does not imply identity on torsion. -/
theorem unipotent_not_identity (b : K) (hb : b≠0) :
    ¬ ∀ v, triangular 1 b v=v := by
  intro h
  have hh := congrArg Prod.fst (h (0,1))
  exact hb (by simpa [triangular] using hh)

end TheoremAudit
