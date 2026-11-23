import Lean.Elab.Tactic.Omega

/-!
Pure version comparison used by `ic update`.

`update.rs` parses `vMAJOR.MINOR.PATCH` into three `u64`s and compares them
lexicographically. This file proves that the lexicographic relation is a
strict total order. The Rust unit tests cover the string parser and the
`--check` / `--force` control flow.
-/

namespace VersionSelection

abbrev Version := Nat × Nat × Nat

/-- Strict lexicographic order on `(major, minor, patch)`. -/
def less (a b : Version) : Prop :=
  a.1 < b.1 ∨
    (a.1 = b.1 ∧
      (a.2.1 < b.2.1 ∨ (a.2.1 = b.2.1 ∧ a.2.2 < b.2.2)))

instance (a b : Version) : Decidable (less a b) := by
  unfold less
  infer_instance

/-- The boolean used by the updater. -/
def isNewer (current latest : Version) : Bool := decide (less current latest)

theorem less_irrefl (a : Version) : ¬ less a a := by
  unfold less
  omega

theorem less_asymm {a b : Version} (h : less a b) : ¬ less b a := by
  unfold less at h ⊢
  omega

theorem less_trans {a b c : Version} (hab : less a b) (hbc : less b c) : less a c := by
  unfold less at hab hbc ⊢
  omega

theorem less_total {a b : Version} (hne : a ≠ b) : less a b ∨ less b a := by
  obtain ⟨a1, a2, a3⟩ := a
  obtain ⟨b1, b2, b3⟩ := b
  simp [less] at hne ⊢
  omega

theorem newer_iff_less {current latest : Version} :
    isNewer current latest = true ↔ less current latest := by
  simp [isNewer]

theorem newer_irrefl (a : Version) : isNewer a a = false := by
  simp [isNewer, less_irrefl]

theorem newer_trans {a b c : Version} (hab : isNewer a b = true) (hbc : isNewer b c = true) :
    isNewer a c = true := by
  simp [newer_iff_less] at hab hbc ⊢
  exact less_trans hab hbc

end VersionSelection

#print axioms VersionSelection.less_irrefl
#print axioms VersionSelection.less_asymm
#print axioms VersionSelection.less_trans
#print axioms VersionSelection.less_total
#print axioms VersionSelection.newer_trans
