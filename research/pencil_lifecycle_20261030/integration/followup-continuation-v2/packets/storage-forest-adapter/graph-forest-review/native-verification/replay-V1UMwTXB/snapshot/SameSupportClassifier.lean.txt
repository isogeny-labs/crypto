import SameSupport

set_option autoImplicit false
namespace SameSupportCandidate
open Polynomial
variable {K : Type*} [Field K] [DecidableEq K]

/-- Common monic-quadratic validity excludes the zero-slope error in the
existing abstract same-support classifier. This is not a Rust semantics theorem. -/
theorem valid_inputs_exclude_same_support_error
    (f : Polynomial K) (q r s1 t1 s2 t2 : K)
    (hodd : (2 : K) ≠ 0)
    (valid1 : (X ^ 2 + C q * X + C r) ∣ f - (C s1 * X + C t1) ^ 2)
    (valid2 : (X ^ 2 + C q * X + C r) ∣ f - (C s2 * X + C t2) ^ 2) :
    ProjectiveLawRuntime.classifySameU22 s1 t1 s2 t2 ≠
      ProjectiveLawRuntime.Branch22Prelude.sameUMixedZeroSlope := by
  by_cases hs : s1 = s2
  · obtain ⟨hx, hc⟩ := validity_coefficients f q r s1 t1 s2 t2 valid1 valid2 hs
    exact ProjectiveLawRuntime.classifySameU22_ne_zeroSlope s1 t1 s2 t2 hodd hx hc
  · unfold ProjectiveLawRuntime.classifySameU22
    have hz : s1 - s2 ≠ 0 := sub_ne_zero.mpr hs
    simp only [hs, false_and, hz, ite_false]
    split_ifs <;> intro h <;> cases h

#print axioms valid_inputs_exclude_same_support_error
end SameSupportCandidate
