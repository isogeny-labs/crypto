import Mathlib.Algebra.BigOperators.GroupWithZero.Finset
import Mathlib.Algebra.Field.Defs
namespace RootCertificate46
open scoped BigOperators
variable {K : Type*} [Field K]
/-- A reconstructed product of distinct linear factors certifies a complete
root list. This does not formalize the native GCD or trace-splitting code. -/
theorem linear_product_roots (s : Finset K) (x : K) :
    (∏ r ∈ s, (x-r))=0 ↔ x∈s := by
  classical
  simp [Finset.prod_eq_zero_iff, sub_eq_zero]

theorem complete_root_certificate (f : K → K) (s : Finset K) (c x : K)
    (hc : c≠0) (hf : ∀ y, f y=c*(∏ r ∈ s, (y-r))) :
    f x=0 ↔ x∈s := by
  rw [hf]
  simp only [mul_eq_zero, hc, false_or]
  exact linear_product_roots s x
end RootCertificate46
