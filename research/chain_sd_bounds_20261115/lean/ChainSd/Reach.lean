import Mathlib
import ChainSd.GroupAlgebra

/-!
# Reachability: once `1` is derived, every lower-weight monomial is

If `1 ∈ U_c(k)` then every character `x₁^a x₂^b` of weight `≤ c'` lies in `U_{c'}(k)` for `c' ≥ c`:
add the bits of `a`, then of `b`, in increasing order; each intermediate monomial has weight one
less than the next, so each shift is admitted.  Consequently `U_{c'}(k)` is the whole weight-`≤ c'`
part, which is the hypothesis `hreach` of `lastFallDegree_eq_solvingDegree`; the concrete
statement `lastFallDegree_eq_solvingDegree_chain` is the pinned equality
"solving degree = last fall degree" for the chain system.
-/

namespace ChainSd

open AddMonoidAlgebra

variable (p k : ℕ) [Fact p.Prime]

theorem bitWeight_natCast {x : ℕ} (hx : x < 2 ^ k) : bitWeight k ((x : ℕ) : ZMod (2 ^ k)) = popcount x := by
  unfold bitWeight
  rw [ZMod.val_natCast, Nat.mod_eq_of_lt hx]

theorem wt_pair_natCast {x y : ℕ} (hx : x < 2 ^ k) (hy : y < 2 ^ k) :
    wt k (((x : ℕ) : ZMod (2 ^ k)), ((y : ℕ) : ZMod (2 ^ k))) = popcount x + popcount y := by
  unfold wt
  simp only [bitWeight_natCast k hx, bitWeight_natCast k hy]

theorem single_mem_weightSub {v : Expo k} {c : ℕ} (hv : wt k v ≤ c) :
    single v (1 : ZMod p) ∈ weightSub p k c := by
  rw [mem_weightSub]
  intro w hw
  have : w = v := by
    have h := Finsupp.support_single_subset (a := v) (b := (1 : ZMod p)) hw
    simpa using h
  subst this; exact hv

/-- Shifting a character by the variable `(j, s)`. -/
theorem shiftMap_single (js : Fin k × Bool) (v : Expo k) :
    shiftMap p k js (single v (1 : ZMod p)) = single (shiftExpo k js.1 js.2 + v) 1 := by
  simp [shiftMap, LinearMap.mulLeft_apply, single_mul_single]

variable {p k}
variable (A B t : ZMod p) (c : ℕ)

/-- Building the first coordinate bit by bit. -/
theorem single_lowbits_mem (h1 : (1 : ChainAlg p k) ∈ (chainSystem p k A B t).closure c)
    (x : ℕ) (hx : x < 2 ^ k) (hxw : popcount x ≤ c) :
    ∀ m ≤ k, single (((x % 2 ^ m : ℕ) : ZMod (2 ^ k)), (0 : ZMod (2 ^ k))) (1 : ZMod p)
      ∈ (chainSystem p k A B t).closure c := by
  intro m
  induction m with
  | zero =>
    intro _
    simp only [pow_zero, Nat.mod_one, Nat.cast_zero]
    change single (0 : Expo k) (1 : ZMod p) ∈ _
    rw [← one_def]; exact h1
  | succ m ih =>
    intro hm
    have hmk : m < k := by omega
    have ihm := ih (le_of_lt hmk)
    -- x % 2^(m+1) = x % 2^m + (bit m) * 2^m
    have hsplit : x % 2 ^ (m + 1) = x % 2 ^ m + (x / 2 ^ m % 2) * 2 ^ m := by
      rw [Nat.mod_pow_succ]; ring
    by_cases hb : x / 2 ^ m % 2 = 0
    · rw [hsplit, hb, zero_mul, add_zero]; exact ihm
    · have hb1 : x / 2 ^ m % 2 = 1 := by omega
      rw [hsplit, hb1, one_mul]
      -- the shift by 2^m is admitted: the current weight is one less than the new weight ≤ c
      have hlt : x % 2 ^ m < 2 ^ m := Nat.mod_lt _ (by positivity)
      have hclear : (x % 2 ^ m).testBit m = false := testBit_false_of_lt hlt
      have hpop : popcount (x % 2 ^ m + 2 ^ m) = popcount (x % 2 ^ m) + 1 :=
        popcount_add_pow_of_testBit_false m _ hclear
      have hx1 : x % 2 ^ (m + 1) = x % 2 ^ m + 2 ^ m := by rw [hsplit, hb1, one_mul]
      have hle : popcount (x % 2 ^ m + 2 ^ m) ≤ popcount x := by
        rw [← hx1]; exact popcount_mod_pow_le (m + 1) x
      have hwt : wt k (((x % 2 ^ m : ℕ) : ZMod (2 ^ k)), (0 : ZMod (2 ^ k))) ≤ c - 1 := by
        have h0 : ((0 : ℕ) : ZMod (2 ^ k)) = 0 := Nat.cast_zero
        rw [← h0, wt_pair_natCast k (lt_of_lt_of_le hlt (Nat.pow_le_pow_right (by norm_num) hmk.le)) (by positivity)]
        rw [popcount_zero, add_zero]
        omega
      have hc : 0 < c := by omega
      have hmem := (chainSystem p k A B t).shift_mem_closure (⟨m, hmk⟩, false) ihm hc
        (single_mem_weightSub p k hwt)
      change shiftMap p k _ _ ∈ _ at hmem
      rw [shiftMap_single] at hmem
      convert hmem using 2
      simp [shiftExpo, Prod.ext_iff]
      push_cast
      ring

/-- Building the second coordinate bit by bit on top of a fixed first coordinate. -/
theorem single_highbits_mem (h1 : (1 : ChainAlg p k) ∈ (chainSystem p k A B t).closure c)
    (a : ZMod (2 ^ k)) (y : ℕ) (hy : y < 2 ^ k) (hw : bitWeight k a + popcount y ≤ c)
    (ha : single (a, (0 : ZMod (2 ^ k))) (1 : ZMod p) ∈ (chainSystem p k A B t).closure c) :
    ∀ m ≤ k, single (a, ((y % 2 ^ m : ℕ) : ZMod (2 ^ k))) (1 : ZMod p)
      ∈ (chainSystem p k A B t).closure c := by
  intro m
  induction m with
  | zero =>
    intro _
    simp only [pow_zero, Nat.mod_one, Nat.cast_zero]
    exact ha
  | succ m ih =>
    intro hm
    have hmk : m < k := by omega
    have ihm := ih (le_of_lt hmk)
    have hsplit : y % 2 ^ (m + 1) = y % 2 ^ m + (y / 2 ^ m % 2) * 2 ^ m := by
      rw [Nat.mod_pow_succ]; ring
    by_cases hb : y / 2 ^ m % 2 = 0
    · rw [hsplit, hb, zero_mul, add_zero]; exact ihm
    · have hb1 : y / 2 ^ m % 2 = 1 := by omega
      rw [hsplit, hb1, one_mul]
      have hlt : y % 2 ^ m < 2 ^ m := Nat.mod_lt _ (by positivity)
      have hclear : (y % 2 ^ m).testBit m = false := testBit_false_of_lt hlt
      have hpop : popcount (y % 2 ^ m + 2 ^ m) = popcount (y % 2 ^ m) + 1 :=
        popcount_add_pow_of_testBit_false m _ hclear
      have hy1 : y % 2 ^ (m + 1) = y % 2 ^ m + 2 ^ m := by rw [hsplit, hb1, one_mul]
      have hle : popcount (y % 2 ^ m + 2 ^ m) ≤ popcount y := by
        rw [← hy1]; exact popcount_mod_pow_le (m + 1) y
      have hwt : wt k (a, ((y % 2 ^ m : ℕ) : ZMod (2 ^ k))) ≤ c - 1 := by
        unfold wt
        simp only
        rw [bitWeight_natCast k (lt_of_lt_of_le hlt (Nat.pow_le_pow_right (by norm_num) hmk.le))]
        omega
      have hc : 0 < c := by omega
      have hmem := (chainSystem p k A B t).shift_mem_closure (⟨m, hmk⟩, true) ihm hc
        (single_mem_weightSub p k hwt)
      change shiftMap p k _ _ ∈ _ at hmem
      rw [shiftMap_single] at hmem
      convert hmem using 2
      simp [shiftExpo, Prod.ext_iff]
      push_cast
      ring

/-- Every character of weight `≤ c` lies in the degree-`c` closure once `1` does. -/
theorem single_mem_closure_of_one_mem (h1 : (1 : ChainAlg p k) ∈ (chainSystem p k A B t).closure c)
    (v : Expo k) (hv : wt k v ≤ c) : single v (1 : ZMod p) ∈ (chainSystem p k A B t).closure c := by
  obtain ⟨a, b⟩ := v
  have hak : a.val < 2 ^ k := ZMod.val_lt a
  have hbk : b.val < 2 ^ k := ZMod.val_lt b
  have hwt : bitWeight k a + bitWeight k b ≤ c := hv
  -- first coordinate
  have ha := single_lowbits_mem A B t c h1 a.val hak (by unfold bitWeight at hwt; omega) k le_rfl
  rw [Nat.mod_eq_of_lt hak, ZMod.natCast_zmod_val] at ha
  -- second coordinate
  have hb := single_highbits_mem A B t c h1 a b.val hbk (by unfold bitWeight at hwt; exact hwt) ha k le_rfl
  rw [Nat.mod_eq_of_lt hbk, ZMod.natCast_zmod_val] at hb
  exact hb

/-- The reachability hypothesis of `lastFallDegree_eq_solvingDegree` for the chain system. -/
theorem weightSub_eq_span (c : ℕ) : weightSub p k c =
    Submodule.span (ZMod p) ((fun v : Expo k => single v (1 : ZMod p)) '' {v | wt k v ≤ c}) := by
  unfold weightSub
  rw [Finsupp.supported_eq_span_single, Submodule.comap_equiv_eq_map_symm, Submodule.map_span,
    ← Set.image_comp]
  rfl

theorem weightSub_le_closure_of_one_mem (h1 : (1 : ChainAlg p k) ∈ (chainSystem p k A B t).closure c)
    (c' : ℕ) (hc : c ≤ c') : weightSub p k c' ≤ (chainSystem p k A B t).closure c' := by
  have h1' : (1 : ChainAlg p k) ∈ (chainSystem p k A B t).closure c' :=
    (chainSystem p k A B t).closure_mono hc h1
  rw [weightSub_eq_span]
  apply Submodule.span_le.mpr
  rintro _ ⟨v, hv, rfl⟩
  exact single_mem_closure_of_one_mem A B t c' h1' v hv

theorem one_mem_weightSub_zero : (1 : ChainAlg p k) ∈ weightSub p k 0 := by
  rw [one_def]
  apply single_mem_weightSub
  simp [wt, bitWeight, popcount_zero]

/-- **Theorem 2 for the chain system**: the last fall degree equals the pinned solving degree. -/
theorem lastFallDegree_eq_solvingDegree_chain :
    (chainSystem p k A B t).lastFallDegree = (chainSystem p k A B t).solvingDegree 1 :=
  (chainSystem p k A B t).lastFallDegree_eq_solvingDegree 1 one_mem_weightSub_zero
    (fun c h c' hc => weightSub_le_closure_of_one_mem A B t c h c' hc)

end ChainSd
