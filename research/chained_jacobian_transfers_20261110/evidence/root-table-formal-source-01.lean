import Init.Data.List.Basic
import Init.Data.List.Find

/-! Canonical first-match preservation. The field/no-zero-divisors bridge
for the actual Fp2 coordinate-axis predicate is written separately. -/
set_option autoImplicit false
namespace RootTable

theorem first_match_filter {A : Type} (xs : List A)
    (keep matches : A → Bool)
    (h : ∀ x, matches x = true → keep x = true) :
    (xs.filter keep).find? matches = xs.find? matches := by
  induction xs with
  | nil => rfl
  | cons x xs ih =>
    cases hm : matches x with
    | false =>
      cases hk : keep x <;> simp [List.filter, List.find?, hm, hk, ih]
    | true =>
      have hk := h x hm
      simp [List.filter, List.find?, hm, hk]

end RootTable
#print axioms RootTable.first_match_filter
