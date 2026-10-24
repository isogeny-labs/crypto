import Std.Tactic
namespace ResidualMass

def countLabel (r : Nat) : List Nat → Nat
  | [] => 0
  | k :: ks => (if k = r then 1 else 0) + countLabel r ks

def bucketMass : Nat → List Nat → Nat
  | 0, _ => 0
  | n+1, ks => bucketMass n ks + countLabel n ks

theorem empty_mass (n : Nat) : bucketMass n [] = 0 := by
  induction n with
  | zero => rfl
  | succ n ih => simp [bucketMass, countLabel, ih]

theorem cons_mass (n k : Nat) (ks : List Nat) :
    bucketMass n (k :: ks) = bucketMass n [k] + bucketMass n ks := by
  induction n with
  | zero => rfl
  | succ n ih =>
    simp only [bucketMass, countLabel]
    rw [ih]
    omega

theorem singleton_mass (n k : Nat) :
    bucketMass n [k] = if k < n then 1 else 0 := by
  induction n with
  | zero => simp [bucketMass]
  | succ n ih =>
    by_cases hkn : k = n
    · subst k
      simp [bucketMass, countLabel, ih]
    · by_cases hk : k < n
      · have hks : k < n+1 := by omega
        simp [bucketMass, countLabel, ih, hk, hks, hkn]
      · have hks : ¬ k < n+1 := by omega
        simp [bucketMass, countLabel, ih, hk, hks, hkn]

theorem total_of_bounded (n : Nat) (ks : List Nat)
    (h : ∀ k ∈ ks, k < n) : bucketMass n ks = ks.length := by
  induction ks with
  | nil => simp [empty_mass]
  | cons k ks ih =>
    have hk : k < n := h k (by simp)
    have ht : ∀ a ∈ ks, a < n := by
      intro a ha
      exact h a (by simp [ha])
    rw [cons_mass, singleton_mass]
    simp [hk, ih ht]

theorem mapped_candidates_total {A : Type} (candidates : List A)
    (n : Nat) (residual : A → Nat)
    (h : ∀ a ∈ candidates, residual a < n) :
    bucketMass n (candidates.map residual) = candidates.length := by
  rw [total_of_bounded]
  · simp
  · intro k hk
    obtain ⟨a, ha, rfl⟩ := List.mem_map.mp hk
    exact h a ha

theorem two_images_same_mass {A : Type} (candidates : List A) (n : Nat)
    (first second : A → Nat)
    (hf : ∀ a ∈ candidates, first a < n)
    (hs : ∀ a ∈ candidates, second a < n) :
    bucketMass n (candidates.map first) =
      bucketMass n (candidates.map second) := by
  rw [mapped_candidates_total candidates n first hf,
    mapped_candidates_total candidates n second hs]

#print axioms empty_mass
#print axioms cons_mass
#print axioms singleton_mass
#print axioms total_of_bounded
#print axioms mapped_candidates_total
#print axioms two_images_same_mass
end ResidualMass
