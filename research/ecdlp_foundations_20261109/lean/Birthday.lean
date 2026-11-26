import Mathlib

/-! Layer 3d: the birthday union bound.

Both Pollard rho and Shoup's generic-group bound use the same finite counting
fact: among `t` independent uniform draws from a set of size `N`, the number of
tuples containing a collision is at most `t(t-1) N^(t-1)`.  Dividing by `N^t`
gives the birthday bound `t(t-1)/N`, the union-bound core of the `√N`
heuristics.

The theorem here is purely combinatorial; the probabilistic interpretation is
the map `card / N^t`. -/

namespace ECDLPBirthday

variable {t N : ℕ}

/-- Restrict a tuple with `f i = f j` to the coordinates other than `i`. -/
def restrictFun {i j : Fin t} (f : {f : Fin t → Fin N // f i = f j}) :
    ({k : Fin t // k ≠ i} → Fin N) :=
  fun k => f.1 k.1

/-- Extend a function on the coordinates other than `i` by copying `j` to `i`. -/
def extendFun {i j : Fin t} (hij : i ≠ j) (g : {k : Fin t // k ≠ i} → Fin N) :
    {f : Fin t → Fin N // f i = f j} :=
  ⟨fun k => if h : k = i then g ⟨j, hij.symm⟩ else g ⟨k, h⟩, by
    have hi : (if h : i = i then g ⟨j, hij.symm⟩ else g ⟨i, h⟩) = g ⟨j, hij.symm⟩ := by
      simp
    have hj : (if h : j = i then g ⟨j, hij.symm⟩ else g ⟨j, h⟩) = g ⟨j, hij.symm⟩ := by
      rw [dif_neg hij.symm]
    simpa [hi, hj]⟩

/-- The equality fiber `{f // f i = f j}` is equivalent to the functions on the
other `t - 1` coordinates. -/
noncomputable def fiberEquiv {i j : Fin t} (hij : i ≠ j) :
    {f : Fin t → Fin N // f i = f j} ≃ ({k : Fin t // k ≠ i} → Fin N) where
  toFun := restrictFun
  invFun := extendFun hij
  left_inv := by
    intro f
    ext k
    by_cases hk : k = i
    · subst hk
      simp [extendFun, restrictFun, f.2]
    · simp [extendFun, restrictFun, hk]
  right_inv := by
    intro g
    ext k
    simp [extendFun, restrictFun, k.2]

/-- A collision fiber has exactly `N^(t-1)` tuples. -/
theorem card_fiber {i j : Fin t} (hij : i ≠ j) :
    Fintype.card {f : Fin t → Fin N // f i = f j} = N ^ (t - 1) := by
  rw [Fintype.card_congr (fiberEquiv hij)]
  rw [Fintype.card_fun, Fintype.card_fin]
  have hcard : Fintype.card {k : Fin t // k ≠ i} = t - 1 := by
    have := Fintype.card_subtype_compl (α := Fin t) (p := fun k => k = i)
    simpa [Fintype.card_fin] using this
  rw [hcard]

/-- **Birthday union bound.**  The number of `t`-tuples from `Fin N` containing
a repeated value is at most `t(t-1) N^(t-1)`. -/
theorem collision_card_le (t N : ℕ) :
    (Finset.univ.filter (fun f : Fin t → Fin N =>
      ∃ i j : Fin t, i ≠ j ∧ f i = f j)).card
      ≤ t * (t - 1) * N ^ (t - 1) := by
  classical
  let P : Finset (Fin t × Fin t) := Finset.univ.offDiag
  have hsub : (Finset.univ.filter (fun f : Fin t → Fin N =>
        ∃ i j : Fin t, i ≠ j ∧ f i = f j)) ⊆
      P.biUnion (fun p : Fin t × Fin t =>
        Finset.univ.filter (fun f : Fin t → Fin N => f p.1 = f p.2)) := by
    intro f hf
    simp only [Finset.mem_filter, Finset.mem_univ, true_and] at hf
    obtain ⟨i, j, hij, hfij⟩ := hf
    rw [Finset.mem_biUnion]
    refine ⟨(i, j), ?_, ?_⟩
    · simp [P, Finset.mem_offDiag, hij]
    · simp [hfij]
  calc (Finset.univ.filter (fun f : Fin t → Fin N =>
          ∃ i j : Fin t, i ≠ j ∧ f i = f j)).card
      ≤ (P.biUnion (fun p : Fin t × Fin t =>
          Finset.univ.filter (fun f : Fin t → Fin N => f p.1 = f p.2))).card :=
        Finset.card_le_card hsub
    _ ≤ P.card * N ^ (t - 1) := by
        apply Finset.card_biUnion_le_card_mul
        intro p hp
        have hp' : p.1 ≠ p.2 := by simpa [P, Finset.mem_offDiag] using hp
        rw [← Fintype.card_subtype (p := fun f : Fin t → Fin N => f p.1 = f p.2)]
        exact le_of_eq (card_fiber hp')
    _ = t * (t - 1) * N ^ (t - 1) := by
        have hP : P.card = t * t - t := by
          simp [P, Finset.offDiag_card, Fintype.card_fin]
        rw [hP]
        have : t * t - t = t * (t - 1) := by
          rw [Nat.mul_sub_one]
        rw [this]

end ECDLPBirthday

namespace ECDLPBirthday

/-- **Birthday bound, probability form.**  A uniform `t`-tuple from a set of
size `N > 0` contains a collision with probability at most `t(t-1)/N`. -/
theorem collision_prob_le (t N : ℕ) (hN : 0 < N) :
    ((Finset.univ.filter (fun f : Fin t → Fin N =>
      ∃ i j : Fin t, i ≠ j ∧ f i = f j)).card : ℚ) / (N : ℚ) ^ t
      ≤ (t * (t - 1) : ℚ) / N := by
  have h := collision_card_le t N
  rcases Nat.eq_zero_or_pos t with ht | ht
  · subst ht
    simp
  · have ht1 : 1 ≤ t := ht
    have hNq : (0 : ℚ) < N := by exact_mod_cast hN
    have hcard : ((Finset.univ.filter (fun f : Fin t → Fin N =>
        ∃ i j : Fin t, i ≠ j ∧ f i = f j)).card : ℚ)
        ≤ (t : ℚ) * ((t : ℚ) - 1) * (N : ℚ) ^ (t - 1) := by
      have h' : ((Finset.univ.filter (fun f : Fin t → Fin N =>
          ∃ i j : Fin t, i ≠ j ∧ f i = f j)).card : ℚ)
          ≤ ((t * (t - 1) * N ^ (t - 1) : ℕ) : ℚ) := by
        exact_mod_cast h
      push_cast at h'
      rw [Nat.cast_sub ht1] at h'
      exact h'
    have hpow : (0 : ℚ) < (N : ℚ) ^ (t - 1) := pow_pos hNq _
    have ht' : (N : ℚ) ^ t = (N : ℚ) * (N : ℚ) ^ (t - 1) := by
      rw [← pow_succ']
      congr 1
      omega
    rw [ht', div_le_div_iff₀ (by positivity) hNq]
    have hmul := mul_le_mul_of_nonneg_right hcard (le_of_lt hNq)
    calc ((Finset.univ.filter (fun f : Fin t → Fin N =>
          ∃ i j : Fin t, i ≠ j ∧ f i = f j)).card : ℚ) * N
        ≤ ((t : ℚ) * ((t : ℚ) - 1) * (N : ℚ) ^ (t - 1)) * N := hmul
      _ = (t : ℚ) * ((t : ℚ) - 1) * (N * (N : ℚ) ^ (t - 1)) := by ring

end ECDLPBirthday
