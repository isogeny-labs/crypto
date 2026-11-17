import Lean.Elab.Tactic.Omega

/-!
Archive selection used by `ic update`.

The Rust selector first filters to `crypto-cli-*.tar.gz` assets (excluding
`SHA256SUMS`), then prefers a universal archive and otherwise matches the
running architecture. This file proves the selection policy over the two
booleans that drive the choice. The archive-name filter and the exact
`macos-*` matching are covered by Rust unit tests.
-/

namespace AssetSelection

structure Asset where
  name : String
  universal : Bool
  matchesArch : Bool
  deriving Repr

/-- First element satisfying a boolean predicate. -/
def find (p : Asset → Bool) : List Asset → Option Asset
  | [] => none
  | asset :: rest => if p asset then some asset else find p rest

theorem find_mem {p : Asset → Bool} {assets : List Asset} {asset : Asset}
    (h : find p assets = some asset) : asset ∈ assets := by
  induction assets with
  | nil => simp [find] at h
  | cons head tail ih =>
      by_cases hp : p head = true
      · simp [find, hp] at h
        subst asset
        simp
      · simp [find, hp] at h
        exact List.mem_cons_of_mem head (ih h)

theorem find_spec {p : Asset → Bool} {assets : List Asset} {asset : Asset}
    (h : find p assets = some asset) : p asset = true := by
  induction assets with
  | nil => simp [find] at h
  | cons head tail ih =>
      by_cases hp : p head = true
      · simp [find, hp] at h
        subst asset
        exact hp
      · simp [find, hp] at h
        exact ih h

theorem find_isSome {p : Asset → Bool} {assets : List Asset} :
    (find p assets).isSome = true ↔ ∃ asset ∈ assets, p asset = true := by
  induction assets with
  | nil => simp [find]
  | cons head tail ih =>
      by_cases hp : p head = true
      · simp [find, hp]
      · simp [find, hp, ih]

def choose (assets : List Asset) : Option Asset :=
  match find (fun asset => asset.universal) assets with
  | some asset => some asset
  | none => find (fun asset => asset.matchesArch) assets

theorem choose_universal {assets : List Asset}
    (h : ∃ asset ∈ assets, asset.universal = true) :
    ∃ asset, choose assets = some asset ∧ asset.universal = true := by
  have hsome : (find (fun asset => asset.universal) assets).isSome = true := by
    rw [find_isSome]
    exact h
  unfold choose
  cases hfind : find (fun asset => asset.universal) assets with
  | none => simp [hfind] at hsome
  | some asset =>
      refine ⟨asset, by simp, find_spec hfind⟩

theorem choose_arch {assets : List Asset}
    (hnoUniversal : ¬ ∃ asset ∈ assets, asset.universal = true)
    (h : ∃ asset ∈ assets, asset.matchesArch = true) :
    ∃ asset, choose assets = some asset ∧ asset.matchesArch = true := by
  have hnone : find (fun asset => asset.universal) assets = none := by
    cases hfind : find (fun asset => asset.universal) assets with
    | none => rfl
    | some asset =>
        exact absurd ⟨asset, find_mem hfind, find_spec hfind⟩ hnoUniversal
  have hsome : (find (fun asset => asset.matchesArch) assets).isSome = true := by
    rw [find_isSome]
    exact h
  unfold choose
  rw [hnone]
  cases hfind : find (fun asset => asset.matchesArch) assets with
  | none => simp [hfind] at hsome
  | some asset =>
      refine ⟨asset, by simp, find_spec hfind⟩

theorem choose_none {assets : List Asset}
    (h : ¬ ∃ asset ∈ assets, asset.universal = true ∨ asset.matchesArch = true) :
    choose assets = none := by
  have hnone : find (fun asset => asset.universal) assets = none := by
    cases hfind : find (fun asset => asset.universal) assets with
    | none => rfl
    | some asset =>
        exact absurd ⟨asset, find_mem hfind, Or.inl (find_spec hfind)⟩ h
  have hnoneArch : find (fun asset => asset.matchesArch) assets = none := by
    cases hfind : find (fun asset => asset.matchesArch) assets with
    | none => rfl
    | some asset =>
        exact absurd ⟨asset, find_mem hfind, Or.inr (find_spec hfind)⟩ h
  unfold choose
  rw [hnone, hnoneArch]

end AssetSelection

#print axioms AssetSelection.find_mem
#print axioms AssetSelection.find_spec
#print axioms AssetSelection.find_isSome
#print axioms AssetSelection.choose_universal
#print axioms AssetSelection.choose_arch
#print axioms AssetSelection.choose_none
