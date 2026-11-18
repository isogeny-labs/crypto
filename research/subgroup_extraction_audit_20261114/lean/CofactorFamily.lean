import PrimeRecovery
import Mathlib.Data.Nat.Prime.Infinite

namespace SubgroupExtractionAudit

def familyP (r : ℕ) : ZMod (2*r) := 2

theorem family_order (r : ℕ) (hr : 0 < r) : addOrderOf (familyP r) = r := by
  unfold familyP
  change addOrderOf ((2 : ℕ) : ZMod (2*r)) = r
  rw [ZMod.addOrderOf_coe 2 (by omega)]
  simp

theorem family_ambient_card (r : ℕ) (hr : 0 < r) : Nat.card (ZMod (2*r)) = 2*r := by
  let : NeZero (2*r) := ⟨by omega⟩
  simp

theorem family_point_nonzero (r : ℕ) (hr : 1 < r) : familyP r ≠ 0 := by
  intro hz
  have ho := family_order r (by omega)
  rw [hz, addOrderOf_zero] at ho
  omega

theorem family_nonzero_beta (r : ℕ) (hr : 0 < r) : (r : ZMod (2*r)) ≠ 0 := by
  intro hz
  have hd := (ZMod.natCast_eq_zero_iff r (2*r)).mp hz
  exact Nat.not_dvd_of_pos_of_lt hr (by omega) hd

theorem family_nonunit_beta (r : ℕ) (hr : 1 < r) : ¬ IsUnit (r : ZMod (2*r)) := by
  rw [ZMod.isUnit_iff_coprime]
  intro h
  have hdiv : r ∣ 2*r := dvd_mul_left r 2
  have hg := h.gcd_eq_one
  rw [Nat.gcd_eq_left hdiv] at hg
  omega

theorem family_zero_subgroup_beta (r : ℕ) : (r : ZMod r) = 0 := by simp

/-- This nonzero ambient coefficient gives the same relation for every candidate log. -/
theorem family_uninformative (r : ℕ) (hr : 0 < r) (x : ℕ) :
    (r : ℕ) • (x • familyP r) = 0 := by
  have hk : r • familyP r = 0 := by
    have h := addOrderOf_nsmul_eq_zero (familyP r)
    rwa [family_order r hr] at h
  rw [smul_comm r x (familyP r), hk, nsmul_zero]

theorem unbounded_prime_family (b : ℕ) : ∃ r : ℕ, Nat.Prime r ∧ b < r ∧
    addOrderOf (familyP r) = r ∧ (r : ZMod (2*r)) ≠ 0 ∧
    ¬ IsUnit (r : ZMod (2*r)) ∧ (∀ x : ℕ, r • (x • familyP r) = 0) := by
  obtain ⟨r, hbr, hr⟩ := Nat.exists_infinite_primes (b+1)
  exact ⟨r, hr, by omega, family_order r hr.pos, family_nonzero_beta r hr.pos,
    family_nonunit_beta r hr.one_lt, family_uninformative r hr.pos⟩

end SubgroupExtractionAudit
