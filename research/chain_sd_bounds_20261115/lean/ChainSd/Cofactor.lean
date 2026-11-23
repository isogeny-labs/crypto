import Mathlib
import ChainSd.Closure
import ChainSd.Popcount
import ChainSd.GroupAlgebra
import ChainSd.LowerThree

/-!
# The cofactor reduction (Theorem A)

For a unit `g` of the chain algebra `Q_k = F_p[(ℤ/2^k)²]` supported on `{0,1,2}²` with nonzero
coefficient at `(2,2)`, write `ginv` for its inverse.  The *cofactor* of `u` is `ginv * u`.

`topSet k` is the set of exponents with some coordinate `≥ 2^k - 2` (the top two residues).

**Theorem A** (`one_not_mem_closure_of_cofactorHyp`).  Suppose the *cofactor hypothesis*
`CofactorHyp g c`: every `v` whose support avoids `topSet k` and with `g * v` of weight `≤ c - 1`
has, at every exponent of its support, binary weight `≤ k - 3` in each coordinate.  Then `1` is not
in the degree-`c` closure of `g`.

The proof: the subspace of weight-`≤ c` elements whose cofactor avoids `topSet k` is `c`-closed
(a shift moves an exponent of popcount `≤ k - 3` by `2^j`, which cannot land on `2^k - 2` or
`2^k - 1`), and it does not contain `1` (if `ginv` avoided `topSet k`, the coefficient of
`g * ginv` at the lex-largest exponent of `ginv` shifted by `(2,2)` would be nonzero, but
`g * ginv = 1`).  The hypothesis is the finite form of the missing lemma of the report: it follows
from a bound on the binary weight of cofactors of low-weight multiples of `S` in `F_p[x₁, x₂]`.
-/

namespace ChainSd

open AddMonoidAlgebra

/-! ### Popcount arithmetic -/

theorem popcount_add_le : ∀ x y : ℕ, popcount (x + y) ≤ popcount x + popcount y := by
  intro x y
  induction h : x + y using Nat.strong_induction_on generalizing x y with
  | _ n ih =>
    subst h
    rcases Nat.eq_zero_or_pos (x + y) with h0 | hpos
    · rw [h0, popcount_zero]; exact Nat.zero_le _
    · rw [popcount_eq (x + y), popcount_eq x, popcount_eq y]
      have hx := Nat.mod_two_eq_zero_or_one x
      have hy := Nat.mod_two_eq_zero_or_one y
      rcases hx with hx | hx <;> rcases hy with hy | hy
      · have e1 : (x + y) / 2 = x / 2 + y / 2 := by omega
        have e2 : (x + y) % 2 = 0 := by omega
        rw [e1, e2, hx, hy]
        have := ih (x / 2 + y / 2) (by omega) (x / 2) (y / 2) rfl
        omega
      · have e1 : (x + y) / 2 = x / 2 + y / 2 := by omega
        have e2 : (x + y) % 2 = 1 := by omega
        rw [e1, e2, hx, hy]
        have := ih (x / 2 + y / 2) (by omega) (x / 2) (y / 2) rfl
        omega
      · have e1 : (x + y) / 2 = x / 2 + y / 2 := by omega
        have e2 : (x + y) % 2 = 1 := by omega
        rw [e1, e2, hx, hy]
        have := ih (x / 2 + y / 2) (by omega) (x / 2) (y / 2) rfl
        omega
      · -- both odd: (x+y)/2 = x/2 + y/2 + 1
        have e1 : (x + y) / 2 = (x / 2 + y / 2) + 1 := by omega
        have e2 : (x + y) % 2 = 0 := by omega
        rw [e1, e2, hx, hy]
        have h1 := ih ((x / 2 + y / 2) + 1) (by omega) (x / 2 + y / 2) 1 rfl
        have h2 := ih (x / 2 + y / 2) (by omega) (x / 2) (y / 2) rfl
        have h3 : popcount 1 = 1 := by
          rw [popcount_eq 1, popcount_zero]
        omega

theorem popcount_two_pow (j : ℕ) : popcount (2 ^ j) = 1 := by
  have := popcount_add_pow_of_testBit_false j 0 (by simp)
  simpa [popcount_zero] using this

theorem popcount_two_pow_sub_one : ∀ n : ℕ, popcount (2 ^ n - 1) = n
  | 0 => by simp [popcount_zero]
  | n + 1 => by
    have e : 2 ^ (n + 1) - 1 = (2 ^ n - 1) + 2 ^ n := by
      have : 1 ≤ 2 ^ n := Nat.one_le_two_pow
      rw [pow_succ]; omega
    have hlt : 2 ^ n - 1 < 2 ^ n := by have : 1 ≤ 2 ^ n := Nat.one_le_two_pow; omega
    rw [e, popcount_add_pow_of_testBit_false n _ (testBit_false_of_lt hlt),
      popcount_two_pow_sub_one n]

/-- **Key arithmetic lemma.** Adding `2^j` (mod `2^k`) to a residue of popcount `≤ k - 3` never
lands on the top two residues `2^k - 2`, `2^k - 1`. -/
theorem shift_avoids_top {k a j : ℕ} (hk : 2 ≤ k) (ha : a < 2 ^ k) (hpop : popcount a + 3 ≤ k)
    (hj : j < k) : (a + 2 ^ j) % 2 ^ k < 2 ^ k - 2 := by
  have hjk : 2 ^ j ≤ 2 ^ (k - 1) := Nat.pow_le_pow_right (by norm_num) (by omega)
  have hk1 : 2 ≤ 2 ^ (k - 1) := by
    calc 2 = 2 ^ 1 := by norm_num
      _ ≤ 2 ^ (k - 1) := Nat.pow_le_pow_right (by norm_num) (by omega)
  have hkk : 2 ^ k = 2 * 2 ^ (k - 1) := by
    rw [← pow_succ']; congr 1; omega
  by_cases hw : a + 2 ^ j < 2 ^ k
  · rw [Nat.mod_eq_of_lt hw]
    by_contra hcon
    push Not at hcon
    -- r := 2^k - 1 - a lies in {2^j, 2^j + 1}
    set r := 2 ^ k - 1 - a with hr
    have hsum : a + r = 2 ^ k - 1 := by omega
    have hr1 : 2 ^ j ≤ r := by omega
    have hr2 : r ≤ 2 ^ j + 1 := by omega
    have hpr : popcount r ≤ 2 := by
      rcases Nat.eq_or_lt_of_le hr2 with h | h
      · rw [h]
        have := popcount_add_le (2 ^ j) 1
        rw [popcount_two_pow] at this
        have h1 : popcount 1 = 1 := by rw [popcount_eq 1, popcount_zero]
        omega
      · have : r = 2 ^ j := by omega
        rw [this, popcount_two_pow]; omega
    have htot := popcount_add_le a r
    rw [hsum, popcount_two_pow_sub_one] at htot
    omega
  · push Not at hw
    have hlt : a + 2 ^ j < 2 * 2 ^ k := by omega
    have : (a + 2 ^ j) % 2 ^ k = a + 2 ^ j - 2 ^ k := by
      rw [Nat.mod_eq_sub_mod hw, Nat.mod_eq_of_lt (by omega)]
    rw [this]
    omega

/-! ### The generic chain system and the cofactor hypothesis -/

variable (p k : ℕ) [Fact p.Prime]

/-- The chain system with an arbitrary generator `g`. -/
noncomputable def chainSystemG (g : ChainAlg p k) :
    ShiftSystem (ZMod p) (ChainAlg p k) (Fin k × Bool) where
  F := weightSub p k
  mono := weightSub_mono p k
  σ := shiftMap p k
  g := g

theorem chainSystem_eq_G (A B t : ZMod p) :
    chainSystem p k A B t = chainSystemG p k (gen p k A B t) := rfl

/-- Exponents with some coordinate among the top two residues. -/
def topSet : Set (Expo k) := {x | 2 ^ k - 2 ≤ x.1.val ∨ 2 ^ k - 2 ≤ x.2.val}

/-- Elements whose support avoids `topSet`. -/
noncomputable def avoidSub : Submodule (ZMod p) (ChainAlg p k) :=
  (Finsupp.supported (ZMod p) (ZMod p) (topSet k)ᶜ).comap
    (coeffLinearEquiv (R := ZMod p) (S := ZMod p) (M := Expo k)).toLinearMap

theorem mem_avoidSub {v : ChainAlg p k} :
    v ∈ avoidSub p k ↔ ∀ x ∈ v.coeff.support, x ∉ topSet k := by
  simp only [avoidSub, Submodule.mem_comap, LinearEquiv.coe_coe, Finsupp.mem_supported,
    Set.subset_def, Finset.mem_coe, Set.mem_compl_iff]
  rfl

/-- The cofactor hypothesis `H(c)`. -/
def CofactorHyp (g : ChainAlg p k) (c : ℕ) : Prop :=
  ∀ v : ChainAlg p k, v ∈ avoidSub p k → g * v ∈ weightSub p k (c - 1) →
    ∀ x ∈ v.coeff.support, popcount x.1.val + 3 ≤ k ∧ popcount x.2.val + 3 ≤ k

/-- The shift hypothesis `H'(c)`: every cofactor `v` off the top residues with `g v` of weight
`≤ c − 1` stays off the top residues after each generator shift. It is the only property of the
cofactors that the closure step of Theorem A uses; `CofactorHyp` implies it
(`shiftHyp_of_cofactorHyp`). -/
def ShiftHyp (g : ChainAlg p k) (c : ℕ) : Prop :=
  ∀ v : ChainAlg p k, v ∈ avoidSub p k → g * v ∈ weightSub p k (c - 1) →
    ∀ js : Fin k × Bool, single (shiftExpo k js.1 js.2) (1 : ZMod p) * v ∈ avoidSub p k

/-! ### Support of a shifted element -/

variable {p k}

theorem shiftExpo_fst (j : Fin k) : (shiftExpo k j false).1 = ((2 ^ (j : ℕ) : ℕ) : ZMod (2 ^ k)) ∧
    (shiftExpo k j false).2 = 0 := by simp [shiftExpo]

theorem shiftExpo_snd (j : Fin k) : (shiftExpo k j true).1 = 0 ∧
    (shiftExpo k j true).2 = ((2 ^ (j : ℕ) : ℕ) : ZMod (2 ^ k)) := by simp [shiftExpo]

theorem val_add_two_pow {k : ℕ} (a : ZMod (2 ^ k)) (j : ℕ) :
    (a + ((2 ^ j : ℕ) : ZMod (2 ^ k))).val = (a.val + 2 ^ j) % 2 ^ k := by
  rw [ZMod.val_add, ZMod.val_natCast, Nat.add_mod_mod]

theorem mem_support_single_mul {e : Expo k} {v : ChainAlg p k} {y : Expo k}
    (hy : y ∈ (single e (1 : ZMod p) * v).coeff.support) : ∃ x ∈ v.coeff.support, y = e + x := by
  rw [Finsupp.mem_support_iff, coeff_single_mul_apply, one_mul] at hy
  refine ⟨-e + y, Finsupp.mem_support_iff.mpr hy, by abel⟩

/-- Shifting preserves avoidance of `topSet` for cofactors of small popcount. -/
theorem shift_mem_avoidSub (hk : 2 ≤ k) (js : Fin k × Bool) {v : ChainAlg p k}
    (hv : v ∈ avoidSub p k)
    (hpop : ∀ x ∈ v.coeff.support, popcount x.1.val + 3 ≤ k ∧ popcount x.2.val + 3 ≤ k) :
    single (shiftExpo k js.1 js.2) (1 : ZMod p) * v ∈ avoidSub p k := by
  rw [mem_avoidSub] at hv ⊢
  intro y hy
  obtain ⟨x, hx, rfl⟩ := mem_support_single_mul hy
  have hxt := hv x hx
  simp only [topSet, Set.mem_setOf_eq, not_or, not_le] at hxt ⊢
  obtain ⟨hp1, hp2⟩ := hpop x hx
  obtain ⟨j, s⟩ := js
  cases s
  · obtain ⟨h1, h2⟩ := shiftExpo_fst (k := k) j
    refine ⟨?_, ?_⟩
    · rw [Prod.fst_add, h1, add_comm, val_add_two_pow]
      exact shift_avoids_top hk (ZMod.val_lt _) hp1 j.isLt
    · rw [Prod.snd_add, h2, zero_add]; exact hxt.2
  · obtain ⟨h1, h2⟩ := shiftExpo_snd (k := k) j
    refine ⟨?_, ?_⟩
    · rw [Prod.fst_add, h1, zero_add]; exact hxt.1
    · rw [Prod.snd_add, h2, add_comm, val_add_two_pow]
      exact shift_avoids_top hk (ZMod.val_lt _) hp2 j.isLt

/-- The popcount hypothesis implies the shift hypothesis. -/
theorem shiftHyp_of_cofactorHyp (hk : 2 ≤ k) {g : ChainAlg p k} {c : ℕ}
    (H : CofactorHyp p k g c) : ShiftHyp p k g c :=
  fun v hv hw js => shift_mem_avoidSub hk js hv (H v hv hw)

/-! ### Weight of a shift -/

theorem bitWeight_add_two_pow_le (a : ZMod (2 ^ k)) (j : ℕ) :
    bitWeight k (a + ((2 ^ j : ℕ) : ZMod (2 ^ k))) ≤ bitWeight k a + 1 := by
  unfold bitWeight
  rw [val_add_two_pow]
  calc popcount ((a.val + 2 ^ j) % 2 ^ k) ≤ popcount (a.val + 2 ^ j) := popcount_mod_pow_le _ _
    _ ≤ popcount a.val + popcount (2 ^ j) := popcount_add_le _ _
    _ = popcount a.val + 1 := by rw [popcount_two_pow]

theorem shift_mem_weightSub (js : Fin k × Bool) {u : ChainAlg p k} {c : ℕ} (hc : 0 < c)
    (hu : u ∈ weightSub p k (c - 1)) : shiftMap p k js u ∈ weightSub p k c := by
  rw [mem_weightSub] at hu ⊢
  intro y hy
  simp only [shiftMap, LinearMap.mulLeft_apply] at hy
  obtain ⟨x, hx, rfl⟩ := mem_support_single_mul hy
  have := hu x hx
  obtain ⟨j, s⟩ := js
  unfold wt at this ⊢
  cases s
  · obtain ⟨h1, h2⟩ := shiftExpo_fst (k := k) j
    rw [Prod.fst_add, Prod.snd_add, h1, h2, zero_add, add_comm (((2 ^ (j : ℕ) : ℕ) : ZMod (2 ^ k)))]
    have := bitWeight_add_two_pow_le x.1 j
    omega
  · obtain ⟨h1, h2⟩ := shiftExpo_snd (k := k) j
    rw [Prod.fst_add, Prod.snd_add, h1, h2, zero_add, add_comm (((2 ^ (j : ℕ) : ℕ) : ZMod (2 ^ k)))]
    have := bitWeight_add_two_pow_le x.2 j
    omega

/-! ### The inverse touches the top -/

theorem val_two (hk : 2 ≤ k) : (((2 : ℕ) : ZMod (2 ^ k))).val = 2 := by
  rw [ZMod.val_natCast, Nat.mod_eq_of_lt]
  calc 2 < 4 := by norm_num
    _ = 2 ^ 2 := by norm_num
    _ ≤ 2 ^ k := Nat.pow_le_pow_right (by norm_num) hk

/-- If `g * ginv = 1` with `g` supported on `{0,1,2}²` and `g_{22} ≠ 0`, then `ginv` meets
`topSet`. -/
theorem inv_not_mem_avoidSub (hk : 2 ≤ k) (g ginv : ChainAlg p k) (hinv : g * ginv = 1)
    (hsupp : ∀ x ∈ g.coeff.support, x.1.val ≤ 2 ∧ x.2.val ≤ 2)
    (h22 : g.coeff (((2 : ℕ) : ZMod (2 ^ k)), ((2 : ℕ) : ZMod (2 ^ k))) ≠ 0) :
    ginv ∉ avoidSub p k := by
  classical
  intro hav
  rw [mem_avoidSub] at hav
  have hne : ginv.coeff.support.Nonempty := by
    rw [Finsupp.support_nonempty_iff]
    intro h0
    have : ginv = 0 := by ext x; simp [h0]
    rw [this, mul_zero] at hinv
    exact zero_ne_one hinv
  -- lex-max exponent of ginv
  obtain ⟨x1, hx1, hmax1⟩ := Finset.exists_max_image ginv.coeff.support (fun x => x.1.val) hne
  set s2 := ginv.coeff.support.filter (fun x => x.1.val = x1.1.val) with hs2
  have hne2 : s2.Nonempty := ⟨x1, by simp [hs2, hx1]⟩
  obtain ⟨xs, hxs, hmax2⟩ := Finset.exists_max_image s2 (fun x => x.2.val) hne2
  simp only [hs2, Finset.mem_filter] at hxs
  obtain ⟨hxs_mem, hxs1⟩ := hxs
  have hxs_top := hav xs hxs_mem
  simp only [topSet, Set.mem_setOf_eq, not_or, not_le] at hxs_top
  set two : ZMod (2 ^ k) := ((2 : ℕ) : ZMod (2 ^ k)) with htwo
  set t : Expo k := (xs.1 + two, xs.2 + two) with ht
  -- coefficient of g * ginv at t
  have hcoef : (g * ginv).coeff t = g.coeff (two, two) * ginv.coeff xs := by
    rw [coeff_mul_apply_left]
    rw [Finsupp.sum_eq_single (two, two)]
    · congr 2
      ext <;> simp [ht]
    · intro h hh hne22
      obtain ⟨hh1, hh2⟩ := hsupp h (Finsupp.mem_support_iff.mpr hh)
      -- the shifted index lies outside the support of ginv
      have hz : ginv.coeff (-h + t) = 0 := by
        by_contra hz
        have hzmem : -h + t ∈ ginv.coeff.support := Finsupp.mem_support_iff.mpr hz
        -- value of the first coordinate
        have hv1 : (-h + t).1.val = xs.1.val + 2 - h.1.val := by
          have e : (-h + t).1 = (((xs.1.val + 2 - h.1.val : ℕ)) : ZMod (2 ^ k)) := by
            rw [Nat.cast_sub (by omega), Nat.cast_add, ZMod.natCast_zmod_val, ZMod.natCast_zmod_val]
            simp [ht, htwo]; abel
          rw [e, ZMod.val_natCast, Nat.mod_eq_of_lt (by omega)]
        have hv2 : (-h + t).2.val = xs.2.val + 2 - h.2.val := by
          have e : (-h + t).2 = (((xs.2.val + 2 - h.2.val : ℕ)) : ZMod (2 ^ k)) := by
            rw [Nat.cast_sub (by omega), Nat.cast_add, ZMod.natCast_zmod_val, ZMod.natCast_zmod_val]
            simp [ht, htwo]; abel
          rw [e, ZMod.val_natCast, Nat.mod_eq_of_lt (by omega)]
        have hm1 := hmax1 _ hzmem
        by_cases h1 : h.1.val = 2
        · -- then the second coordinate must be < 2
          have h2 : h.2.val ≠ 2 := by
            intro h2
            apply hne22
            have e1 : h.1 = two := by
              rw [htwo, ← ZMod.natCast_zmod_val h.1, h1]
            have e2 : h.2 = two := by
              rw [htwo, ← ZMod.natCast_zmod_val h.2, h2]
            exact Prod.ext e1 e2
          have hfst : (-h + t).1.val = x1.1.val := by rw [hv1, h1, ← hxs1]; omega
          have hmem2 : -h + t ∈ s2 := by rw [hs2, Finset.mem_filter]; exact ⟨hzmem, hfst⟩
          have hm2 := hmax2 _ hmem2
          omega
        · omega
      rw [hz, mul_zero]
    · intro hnot
      simp [hnot]
  rw [hinv] at hcoef
  have hgx : ginv.coeff xs ≠ 0 := Finsupp.mem_support_iff.mp hxs_mem
  have hrhs : g.coeff (two, two) * ginv.coeff xs ≠ 0 := mul_ne_zero h22 hgx
  rw [← hcoef] at hrhs
  apply hrhs
  -- (1 : Q_k).coeff t = 0 since t ≠ 0
  rw [one_def, coeff_single, Finsupp.single_apply, if_neg]
  intro h0
  have h1 : (0 : Expo k).1 = t.1 := congrArg Prod.fst h0
  have hval : t.1.val = xs.1.val + 2 := by
    simp only [ht]
    rw [ZMod.val_add_of_lt] <;> rw [htwo, val_two hk]; omega
  rw [← h1] at hval
  simp at hval

/-! ### Theorem A -/

variable (p k)

/-- The invariant subspace: weight `≤ c` elements whose cofactor avoids `topSet`. -/
noncomputable def cofactorSub (ginv : ChainAlg p k) (c : ℕ) : Submodule (ZMod p) (ChainAlg p k) :=
  weightSub p k c ⊓ (avoidSub p k).comap (LinearMap.mulLeft (ZMod p) ginv)

variable {p k}

/-- **Theorem A′.** Under the shift hypothesis, `1` is not in the degree-`c` closure of `g`. -/
theorem one_not_mem_closure_of_shiftHyp (hk : 2 ≤ k) {c : ℕ} (g ginv : ChainAlg p k)
    (hinv : g * ginv = 1) (hsupp : ∀ x ∈ g.coeff.support, x.1.val ≤ 2 ∧ x.2.val ≤ 2)
    (h22 : g.coeff (((2 : ℕ) : ZMod (2 ^ k)), ((2 : ℕ) : ZMod (2 ^ k))) ≠ 0)
    (hg : g ∈ weightSub p k c) (H : ShiftHyp p k g c) :
    (1 : ChainAlg p k) ∉ (chainSystemG p k g).closure c := by
  have hinv' : ginv * g = 1 := by rw [mul_comm]; exact hinv
  have hcl : (chainSystemG p k g).IsClosed c (cofactorSub p k ginv c) := by
    refine ⟨?_, ?_⟩
    · refine ⟨hg, ?_⟩
      show ginv * g ∈ avoidSub p k
      rw [hinv', mem_avoidSub]
      intro x hx
      rw [one_def, coeff_single] at hx
      have hx0 : x = 0 := by
        by_contra hne; exact (Finsupp.mem_support_iff.mp hx) (Finsupp.single_eq_of_ne hne)
      subst hx0
      simp only [topSet, Set.mem_setOf_eq, Prod.fst_zero, Prod.snd_zero, ZMod.val_zero, not_or,
        not_le]
      have : 4 ≤ 2 ^ k := by
        calc 4 = 2 ^ 2 := by norm_num
          _ ≤ 2 ^ k := Nat.pow_le_pow_right (by norm_num) hk
      omega
    · intro js u hu hc hw
      obtain ⟨hu1, hu2⟩ := hu
      change ginv * u ∈ avoidSub p k at hu2
      refine ⟨shift_mem_weightSub js hc hw, ?_⟩
      show ginv * shiftMap p k js u ∈ avoidSub p k
      have e : ginv * shiftMap p k js u = single (shiftExpo k js.1 js.2) (1 : ZMod p) * (ginv * u) := by
        simp only [shiftMap, LinearMap.mulLeft_apply]; ring
      rw [e]
      have hgv : g * (ginv * u) = u := by rw [← mul_assoc, hinv, one_mul]
      exact H (ginv * u) hu2 (by rw [hgv]; exact hw) js
  intro h1
  have hmem := (chainSystemG p k g).closure_le hcl h1
  have : ginv * 1 ∈ avoidSub p k := hmem.2
  rw [mul_one] at this
  exact inv_not_mem_avoidSub hk g ginv hinv hsupp h22 this

/-- **Theorem A.** Under the cofactor (popcount) hypothesis, `1` is not in the degree-`c`
closure of `g`. -/
theorem one_not_mem_closure_of_cofactorHyp (hk : 2 ≤ k) {c : ℕ} (g ginv : ChainAlg p k)
    (hinv : g * ginv = 1) (hsupp : ∀ x ∈ g.coeff.support, x.1.val ≤ 2 ∧ x.2.val ≤ 2)
    (h22 : g.coeff (((2 : ℕ) : ZMod (2 ^ k)), ((2 : ℕ) : ZMod (2 ^ k))) ≠ 0)
    (hg : g ∈ weightSub p k c) (H : CofactorHyp p k g c) :
    (1 : ChainAlg p k) ∉ (chainSystemG p k g).closure c :=
  one_not_mem_closure_of_shiftHyp hk g ginv hinv hsupp h22 hg (shiftHyp_of_cofactorHyp hk H)

/-- Theorem A for the Semaev generator of the chain system. -/
theorem one_not_mem_chainSystem_of_cofactorHyp (hk : 2 ≤ k) {c : ℕ} (hc : 2 ≤ c)
    (A B t : ZMod p) (ginv : ChainAlg p k) (hinv : gen p k A B t * ginv = 1)
    (hsupp : ∀ x ∈ (gen p k A B t).coeff.support, x.1.val ≤ 2 ∧ x.2.val ≤ 2)
    (hg : gen p k A B t ∈ weightSub p k c) (H : CofactorHyp p k (gen p k A B t) c) :
    (1 : ChainAlg p k) ∉ (chainSystem p k A B t).closure c := by
  rw [chainSystem_eq_G]
  refine one_not_mem_closure_of_cofactorHyp hk _ ginv hinv hsupp ?_ hg H
  rw [gen_coeff_22 hk]; exact one_ne_zero

/-- Theorem A′ for the Semaev generator of the chain system. -/
theorem one_not_mem_chainSystem_of_shiftHyp (hk : 2 ≤ k) {c : ℕ}
    (A B t : ZMod p) (ginv : ChainAlg p k) (hinv : gen p k A B t * ginv = 1)
    (hsupp : ∀ x ∈ (gen p k A B t).coeff.support, x.1.val ≤ 2 ∧ x.2.val ≤ 2)
    (hg : gen p k A B t ∈ weightSub p k c) (H : ShiftHyp p k (gen p k A B t) c) :
    (1 : ChainAlg p k) ∉ (chainSystem p k A B t).closure c := by
  rw [chainSystem_eq_G]
  refine one_not_mem_closure_of_shiftHyp hk _ ginv hinv hsupp ?_ hg H
  rw [gen_coeff_22 hk]; exact one_ne_zero

end ChainSd
