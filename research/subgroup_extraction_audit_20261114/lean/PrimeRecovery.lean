import Mathlib.Data.ZMod.Basic
import Mathlib.GroupTheory.OrderOfElement
import Mathlib.Tactic

namespace SubgroupExtractionAudit

theorem recovery_ring_identity {R : Type*} [CommSemiring R] (a b x : R) :
    x*b+a = a+b*x := by ring

/-- Reduction to the actual subgroup order, followed by division in its prime field. -/
theorem recover_mod_prime {G : Type*} [AddCommGroup G]
    (r : ℕ) [Fact (Nat.Prime r)] (P : G) (horder : addOrderOf P = r)
    (a b x : ℕ) (hrel : a • P + b • (x • P) = 0)
    (hb : (b : ZMod r) ≠ 0) :
    (x : ZMod r) = -(a : ZMod r) / (b : ZMod r) := by
  have hkill : (a+b*x) • P = 0 := by simpa only [add_nsmul, mul_smul] using hrel
  have hd : r ∣ a+b*x := by
    rw [← horder]
    exact addOrderOf_dvd_iff_nsmul_eq_zero.mpr hkill
  have hz : (a : ZMod r)+(b : ZMod r)*(x : ZMod r)=0 := by
    exact_mod_cast (ZMod.natCast_eq_zero_iff (a+b*x) r).mpr hd
  apply (eq_div_iff hb).mpr
  linear_combination hz

end SubgroupExtractionAudit
