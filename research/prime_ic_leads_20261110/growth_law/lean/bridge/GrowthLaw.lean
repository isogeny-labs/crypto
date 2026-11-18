namespace GrowthLaw

/-
Growth-law note (research/prime_ic_leads_20261110/growth_law/README.md), formal parts.
Core Lean 4 only (no Mathlib). Pinned toolchain: leanprover/lean4:v4.35.0 (tools/lean/lean-toolchain).

What is formalised:
* `pop`              : binary digit sum (the popcount degree of the Kummer monomial z^e).
* `pop_mod_le`       : pop (e % 2^t) ≤ pop e. Reduction modulo z^{2^t} − 1 never raises the degree of a
                       monomial (the degree hypothesis of Theorem 1, on monomials).
* `top_window`       : for e < 2^t and j ≤ t, if the product y_j · z^e wraps past z^{2^t}
                       (2^t ≤ e + 2^j) then t − j ≤ pop e (Theorem 2).
* `closure_image`    : the abstract core of Theorem 1: a map that is additive, compatible with scalars,
                       sends each variable to a variable or to the identity action, does not raise degree
                       and maps generators to generators, maps the degree-c closure into the degree-c
                       closure; hence `closure_one` (if 1 is in the source closure at degree c, the image
                       of 1 is in the target closure at degree c).

Added 2026-11-15:
* pop lemmas       : `pop_concat` (pop (2^k q + r) = pop q + pop r for r < 2^k), `pop_two_pow`,
                     `pop_le_of_lt_two_pow` (per coordinate, a normal-form exponent < 2^t has pop ≤ t),
                     `pop_add_of_and_eq_zero` (additivity on disjoint bits), `pop_add_le`
                     (subadditivity), `pop_mul_mod_le` (pop is submultiplicative on one coordinate's
                     monomials: pop ((a+b) % 2^t) ≤ pop a + pop b), `pop_mulvar_le` (y_j · z^e raises degree by ≤ 1), and the Theorem 4
                     range facts `pop_le_four_eq_two_iff`, `pop_le_two_of_le_four`, `pop_le_one_of_le_two`.
* monotonicity     : `Level`, `Refutes`, `IsLastFall`, `LevelMap` (the Theorem 1 hypotheses on
                     φ : R_{t+1} → R_t); `refutes_descend` (refutation at level t+1 and degree c gives
                     refutation at level t and degree c), `lower_bound_lifts`, `lastFall_le`
                     (d_F(t) ≤ d_F(t+1)), `lastFall_unique`, `closure_mono_deg`, and the tower forms
                     `tower_refutes_descend`, `tower_lastFall_mono` (t ≤ t' ⇒ d_F(t) ≤ d_F(t')),
                     and `lastFall_exists_of_refutes`, `lastFall_descend_exists` (a last fall degree at
                     level t+1 gives one at level t, by least element), `lastFall_le_of_top` (d_F(t) ≤
                     d_F(t+1) needing IsLastFall only at level t+1).
                     Still abstract: the concrete R_t and the reduction map are not instantiated.

Missing bridge (not formalised): the instantiation of `closure_image` with the concrete tower algebras
F_p[z1,z2]/(z_i^{2^t} − 1), i.e. that the reduction map R_{t+1} → R_t satisfies the hypotheses for
every element (not only monomials) and that `Closure` coincides with the subspace definition of
PREREGISTRATION §2. That needs polynomial and linear-algebra libraries (Mathlib).
-/

/-- Binary digit sum. -/
def pop : Nat → Nat
  | 0 => 0
  | n + 1 => (n + 1) % 2 + pop ((n + 1) / 2)
decreasing_by omega

theorem pop_eq (n : Nat) : pop n = n % 2 + pop (n / 2) := by
  cases n with
  | zero => simp [pop]
  | succ n => rw [pop]

theorem pop_bit (b x : Nat) (hb : b < 2) : pop (b + 2 * x) = b + pop x := by
  rw [pop_eq]
  have h1 : (b + 2 * x) % 2 = b := by omega
  have h2 : (b + 2 * x) / 2 = x := by omega
  rw [h1, h2]

/-- Reducing the exponent modulo 2^t never raises the popcount. -/
theorem pop_mod_le (t : Nat) : ∀ e : Nat, pop (e % 2 ^ t) ≤ pop e := by
  induction t with
  | zero =>
    intro e
    have h0 : pop 0 = 0 := by simp [pop]
    rw [Nat.pow_zero, Nat.mod_one, h0]
    exact Nat.zero_le _
  | succ t ih =>
    intro e
    have hsplit : e % 2 ^ (t + 1) = e % 2 + 2 * (e / 2 % 2 ^ t) := by
      rw [Nat.pow_succ, Nat.mul_comm (2 ^ t) 2]
      exact Nat.mod_mul
    rw [hsplit, pop_bit _ _ (Nat.mod_lt _ (by decide))]
    have := ih (e / 2)
    rw [pop_eq e]
    omega

theorem pop_div_le (j : Nat) : ∀ e : Nat, pop (e / 2 ^ j) ≤ pop e := by
  induction j with
  | zero => intro e; simp
  | succ j ih =>
    intro e
    have h : e / 2 ^ (j + 1) = (e / 2) / 2 ^ j := by
      rw [Nat.pow_succ, Nat.mul_comm (2 ^ j) 2, Nat.div_div_eq_div_mul]
    rw [h]
    have := ih (e / 2)
    rw [pop_eq e]
    omega

theorem pop_pow_sub_one (k : Nat) : pop (2 ^ k - 1) = k := by
  induction k with
  | zero => simp [pop]
  | succ k ih =>
    have hp : 0 < 2 ^ k := Nat.two_pow_pos k
    have h : 2 ^ (k + 1) - 1 = 1 + 2 * (2 ^ k - 1) := by
      rw [Nat.pow_succ]; omega
    rw [h, pop_bit 1 _ (by decide), ih]
    omega

/-- Theorem 2 (top window): a wrap of y_j · z^e past z^{2^t} forces pop e ≥ t − j. -/
theorem top_window (t j e : Nat) (hj : j ≤ t) (he : e < 2 ^ t) (hwrap : 2 ^ t ≤ e + 2 ^ j) :
    t - j ≤ pop e := by
  have hP : 0 < 2 ^ j := Nat.two_pow_pos j
  have hpow : 2 ^ t = 2 ^ j * 2 ^ (t - j) := by
    rw [← Nat.pow_add]; congr 1; omega
  have hdm := Nat.div_add_mod e (2 ^ j)
  have hr : e % 2 ^ j < 2 ^ j := Nat.mod_lt _ hP
  -- q := e / 2^j satisfies 2^(t-j) - 1 ≤ q < 2^(t-j)
  have hq_lt : e / 2 ^ j < 2 ^ (t - j) := by
    rw [Nat.div_lt_iff_lt_mul hP, Nat.mul_comm, ← hpow]; exact he
  have hq_ge : 2 ^ (t - j) - 1 ≤ e / 2 ^ j := by
    rw [Nat.le_div_iff_mul_le hP]
    have hM : 0 < 2 ^ (t - j) := Nat.two_pow_pos _
    have : (2 ^ (t - j) - 1) * 2 ^ j + 2 ^ j = 2 ^ t := by
      rw [hpow, Nat.mul_comm (2 ^ j) (2 ^ (t - j)), Nat.sub_one_mul]
      have : 2 ^ j ≤ 2 ^ (t - j) * 2 ^ j := Nat.le_mul_of_pos_left _ hM
      omega
    -- from 2^t ≤ e + 2^j and e = 2^j * q + r with r < 2^j
    have hmulq : 2 ^ j * (e / 2 ^ j) + e % 2 ^ j + 2 ^ j ≥ (2 ^ (t - j) - 1) * 2 ^ j + 2 ^ j := by
      omega
    -- cancel: (M-1)*P ≤ P*q + r, r < P  ⇒  (M-1)*P ≤ q*P
    have h3 : (2 ^ (t - j) - 1) * 2 ^ j < (e / 2 ^ j + 1) * 2 ^ j := by
      rw [Nat.add_mul, Nat.one_mul, Nat.mul_comm (e / 2 ^ j)]; omega
    have h4 : 2 ^ (t - j) - 1 < e / 2 ^ j + 1 := Nat.lt_of_mul_lt_mul_right h3
    have h5 : 2 ^ (t - j) - 1 ≤ e / 2 ^ j := by omega
    have h6 := Nat.mul_le_mul_right (2 ^ j) h5
    have h7 : e / 2 ^ j * 2 ^ j ≤ e := Nat.div_mul_le_self e (2 ^ j)
    omega
  have hq : e / 2 ^ j = 2 ^ (t - j) - 1 := by omega
  have := pop_div_le j e
  rw [hq, pop_pow_sub_one] at this
  exact this

/-! ### The abstract core of Theorem 1 -/

/-- The operations a closure needs: zero, addition, scalars, multiplication by a variable, degree. -/
structure FilteredData (K R X : Type) where
  zero : R
  add : R → R → R
  smul : K → R → R
  mulv : X → R → R
  deg : R → Nat

/-- The degree-c closure (PREREGISTRATION §2): generators of degree ≤ c, closed under the vector
operations and under multiplication by a variable applied to elements of degree < c. -/
inductive Closure {K R X : Type} (D : FilteredData K R X) (G : R → Prop) (c : Nat) : R → Prop
  | gen {r : R} : G r → D.deg r ≤ c → Closure D G c r
  | zero : Closure D G c D.zero
  | add {a b : R} : Closure D G c a → Closure D G c b → Closure D G c (D.add a b)
  | smul {a : R} (k : K) : Closure D G c a → Closure D G c (D.smul k a)
  | mulv {a : R} (x : X) : Closure D G c a → D.deg a < c → Closure D G c (D.mulv x a)

theorem closure_image {K R S X Y : Type} (D : FilteredData K R X) (E : FilteredData K S Y)
    (G : R → Prop) (H : S → Prop) (c : Nat) (φ : R → S)
    (hzero : φ D.zero = E.zero)
    (hadd : ∀ a b, φ (D.add a b) = E.add (φ a) (φ b))
    (hsmul : ∀ k a, φ (D.smul k a) = E.smul k (φ a))
    (hmul : ∀ x, (∃ y, ∀ a, φ (D.mulv x a) = E.mulv y (φ a)) ∨ (∀ a, φ (D.mulv x a) = φ a))
    (hdeg : ∀ a, E.deg (φ a) ≤ D.deg a)
    (hgen : ∀ r, G r → H (φ r)) :
    ∀ r, Closure D G c r → Closure E H c (φ r) := by
  intro r h
  induction h with
  | gen hr hd => exact Closure.gen (hgen _ hr) (Nat.le_trans (hdeg _) hd)
  | zero => rw [hzero]; exact Closure.zero
  | add _ _ ih1 ih2 => rw [hadd]; exact Closure.add ih1 ih2
  | smul k _ ih => rw [hsmul]; exact Closure.smul k ih
  | @mulv a x _ hlt ih =>
    cases hmul x with
    | inl hy =>
      obtain ⟨y, hy⟩ := hy
      rw [hy]; exact Closure.mulv y ih (Nat.lt_of_le_of_lt (hdeg a) hlt)
    | inr hid => rw [hid]; exact ih

/-- Theorem 1, abstract form: refutation at degree c in the source gives refutation at degree c
in the target, so the least such c (the last fall degree) can only be smaller in the target. -/
theorem closure_one {K R S X Y : Type} (D : FilteredData K R X) (E : FilteredData K S Y)
    (G : R → Prop) (H : S → Prop) (c : Nat) (φ : R → S) (one : R) (one' : S)
    (hone : φ one = one')
    (hzero : φ D.zero = E.zero)
    (hadd : ∀ a b, φ (D.add a b) = E.add (φ a) (φ b))
    (hsmul : ∀ k a, φ (D.smul k a) = E.smul k (φ a))
    (hmul : ∀ x, (∃ y, ∀ a, φ (D.mulv x a) = E.mulv y (φ a)) ∨ (∀ a, φ (D.mulv x a) = φ a))
    (hdeg : ∀ a, E.deg (φ a) ≤ D.deg a)
    (hgen : ∀ r, G r → H (φ r))
    (h1 : Closure D G c one) : Closure E H c one' := by
  rw [← hone]
  exact closure_image D E G H c φ hzero hadd hsmul hmul hdeg hgen one h1


/-! ### Combinatorial lemmas about `pop` used by Theorems 1, 3 and 4 (added 2026-11-15) -/

theorem pop_zero : pop 0 = 0 := by simp [pop]

theorem pop_one : pop 1 = 1 := by
  have h := pop_bit 1 0 (by decide)
  rw [Nat.mul_zero, Nat.add_zero, pop_zero] at h
  exact h

/-- Concatenation: placing `q` above `k` low bits `r` adds the popcounts. -/
theorem pop_concat (k : Nat) : ∀ q r : Nat, r < 2 ^ k → pop (2 ^ k * q + r) = pop q + pop r := by
  induction k with
  | zero =>
    intro q r hr
    have hr0 : r = 0 := by rw [Nat.pow_zero] at hr; omega
    subst hr0
    rw [Nat.pow_zero, Nat.one_mul, Nat.add_zero, pop_zero, Nat.add_zero]
  | succ k ih =>
    intro q r hr
    have hr2 : r / 2 < 2 ^ k := by rw [Nat.pow_succ] at hr; omega
    have hsplit : 2 ^ (k + 1) * q + r = r % 2 + 2 * (2 ^ k * q + r / 2) := by
      rw [Nat.pow_succ]
      have := Nat.mul_assoc (2 ^ k) 2 q
      have h2 : 2 ^ k * 2 * q = 2 * (2 ^ k * q) := by
        rw [Nat.mul_comm (2 ^ k) 2, Nat.mul_assoc]
      omega
    rw [hsplit, pop_bit _ _ (Nat.mod_lt _ (by decide)), ih q (r / 2) hr2, pop_eq r]
    omega

theorem pop_two_pow (k : Nat) : pop (2 ^ k) = 1 := by
  have h := pop_concat k 1 0 (Nat.two_pow_pos k)
  rw [Nat.mul_one, Nat.add_zero, pop_one, pop_zero] at h
  exact h

/-- Per coordinate: a normal-form exponent `e < 2^t` has `pop e ≤ t` (the degree of the monomial in one
coordinate; a monomial of `R_t ⊗ R_t` has total degree ≤ 2t). -/
theorem pop_le_of_lt_two_pow (t : Nat) : ∀ e : Nat, e < 2 ^ t → pop e ≤ t := by
  induction t with
  | zero =>
    intro e he
    have : e = 0 := by rw [Nat.pow_zero] at he; omega
    rw [this, pop_zero]; exact Nat.le_refl 0
  | succ t ih =>
    intro e he
    have h2 : e / 2 < 2 ^ t := by rw [Nat.pow_succ] at he; omega
    have := ih (e / 2) h2
    have hm : e % 2 < 2 := Nat.mod_lt _ (by decide)
    rw [pop_eq e]; omega

/-- Additivity on disjoint bit sets (no carries). -/
theorem pop_add_of_and_eq_zero (n : Nat) :
    ∀ a b : Nat, a + b ≤ n → a &&& b = 0 → pop (a + b) = pop a + pop b := by
  induction n with
  | zero =>
    intro a b hab _
    have ha : a = 0 := by omega
    have hb : b = 0 := by omega
    subst ha; subst hb; rw [pop_zero]
  | succ n ih =>
    intro a b hab hand
    have hnot : ¬ (a % 2 = 1 ∧ b % 2 = 1) := by
      intro h
      have := (Nat.and_mod_two_eq_one (a := a) (b := b)).mpr h
      rw [hand] at this; exact absurd this (by decide)
    have hand2 : a / 2 &&& b / 2 = 0 := by
      rw [← Nat.and_div_two, hand]
    have hle : a / 2 + b / 2 ≤ n := by omega
    have hsplit : a + b = (a % 2 + b % 2) + 2 * (a / 2 + b / 2) := by omega
    have hlt : a % 2 + b % 2 < 2 := by omega
    rw [hsplit, pop_bit _ _ hlt, ih _ _ hle hand2, pop_eq a, pop_eq b]
    omega

/-- Subadditivity: carries only lower the popcount. -/
theorem pop_add_le_aux (n : Nat) : ∀ a b : Nat, a + b ≤ n → pop (a + b) ≤ pop a + pop b := by
  induction n with
  | zero =>
    intro a b hab
    have ha : a = 0 := by omega
    have hb : b = 0 := by omega
    subst ha; subst hb; rw [Nat.add_zero, pop_zero]; exact Nat.le_refl 0
  | succ n ih =>
    intro a b hab
    have hpa := pop_eq a
    have hpb := pop_eq b
    by_cases h : a % 2 = 1 ∧ b % 2 = 1
    · have hsplit : a + b = 0 + 2 * (a / 2 + (b / 2 + 1)) := by omega
      have h1 : a / 2 + (b / 2 + 1) ≤ n := by omega
      have h2 : b / 2 + 1 ≤ n := by omega
      have i1 := ih (a / 2) (b / 2 + 1) h1
      have i2 := ih (b / 2) 1 h2
      rw [pop_one] at i2
      rw [hsplit, pop_bit 0 _ (by decide)]
      omega
    · have hsplit : a + b = (a % 2 + b % 2) + 2 * (a / 2 + b / 2) := by omega
      have hlt : a % 2 + b % 2 < 2 := by omega
      have hle : a / 2 + b / 2 ≤ n := by omega
      have i1 := ih (a / 2) (b / 2) hle
      rw [hsplit, pop_bit _ _ hlt]
      omega

theorem pop_add_le (a b : Nat) : pop (a + b) ≤ pop a + pop b :=
  pop_add_le_aux (a + b) a b (Nat.le_refl _)

/-- `pop` is submultiplicative on one coordinate's monomials: `pop ((a + b) % 2^t) ≤ pop a + pop b`,
i.e. `deg(z^a · z^b) ≤ deg z^a + deg z^b` after reduction mod `z^{2^t} − 1`. This is a Nat fact; it does
not by itself say the filtration on `R_t` is multiplicative. -/
theorem pop_mul_mod_le (t a b : Nat) : pop ((a + b) % 2 ^ t) ≤ pop a + pop b :=
  Nat.le_trans (pop_mod_le t (a + b)) (pop_add_le a b)

/-- Multiplying a monomial by a tower variable `y_j = z^{2^j}` raises the degree by at most one. -/
theorem pop_mulvar_le (t j e : Nat) : pop ((e + 2 ^ j) % 2 ^ t) ≤ pop e + 1 := by
  have := pop_mul_mod_le t e (2 ^ j)
  rw [pop_two_pow] at this
  exact this

theorem pop_two : pop 2 = 1 := by
  have := pop_two_pow 1; rw [Nat.pow_one] at this; exact this

theorem pop_three : pop 3 = 2 := by
  have := pop_pow_sub_one 2; exact this

theorem pop_four : pop 4 = 1 := by
  have := pop_two_pow 2; exact this

/-- Theorem 4, combinatorial part (Nat facts only). Hypotheses for the prose conclusion: the exponents
are normal forms (`W < 2^t`, i.e. `t ≥ 2` for `W = 3` and `t ≥ 3` for `W = 4`), and the top
coefficients are generic nonzero. Statement: on `[0, 4]` the only exponent of popcount 2 is `3 = 11₂`
and no exponent has popcount above 2. `D_W` and the "top form" are NOT defined in Lean; the
conclusion about `D_W` (`D_1 = D_2 = 2`, `D_3 = D_4 = 4`, top form the single monomial `z_1^3 z_2^3`
for `W = 3, 4`) is prose. -/
theorem pop_le_four_eq_two_iff : ∀ k : Nat, k ≤ 4 → (pop k = 2 ↔ k = 3) := by
  intro k hk
  have : k = 0 ∨ k = 1 ∨ k = 2 ∨ k = 3 ∨ k = 4 := by omega
  rcases this with h | h | h | h | h <;> subst h <;>
    simp [pop_zero, pop_one, pop_two, pop_three, pop_four]

/-- Nat fact, same hypotheses as `pop_le_four_eq_two_iff` (normal-form exponents, `t ≥ 2` for `W = 3`,
`t ≥ 3` for `W = 4`, generic nonzero top coefficients); `D_W` is not defined in Lean. -/
theorem pop_le_two_of_le_four : ∀ k : Nat, k ≤ 4 → pop k ≤ 2 := by
  intro k hk
  have : k = 0 ∨ k = 1 ∨ k = 2 ∨ k = 3 ∨ k = 4 := by omega
  rcases this with h | h | h | h | h <;> subst h <;>
    simp [pop_zero, pop_one, pop_two, pop_three, pop_four]

/-- Nat fact for `W ≤ 2` (normal-form exponents, `t ≥ 2`, generic nonzero top coefficients);
`D_W` is not defined in Lean. -/
theorem pop_le_one_of_le_two : ∀ k : Nat, k ≤ 2 → pop k ≤ 1 := by
  intro k hk
  have : k = 0 ∨ k = 1 ∨ k = 2 := by omega
  rcases this with h | h | h <;> subst h <;> simp [pop_zero, pop_one, pop_two]

/-! ### Theorem 1, abstract monotonicity in the tower height (added 2026-11-15) -/

/-- Raising the degree bound keeps every element of the closure. -/
theorem closure_mono_deg {K R X : Type} (D : FilteredData K R X) (G : R → Prop) {c c' : Nat}
    (hcc : c ≤ c') : ∀ r, Closure D G c r → Closure D G c' r := by
  intro r h
  induction h with
  | gen hr hd => exact Closure.gen hr (Nat.le_trans hd hcc)
  | zero => exact Closure.zero
  | add _ _ ih1 ih2 => exact Closure.add ih1 ih2
  | smul k _ ih => exact Closure.smul k ih
  | mulv x _ hlt ih => exact Closure.mulv x ih (Nat.lt_of_lt_of_le hlt hcc)

/-- One tower level: the filtered algebra `R_t` with its operations, the generator set and `1`.
For `m = 2` the ring is `R_t ⊗ R_t` (two coordinates), so monomial degree is at most `2t`. -/
structure Level (K : Type) where
  R : Type
  X : Type
  D : FilteredData K R X
  G : R → Prop
  one : R

/-- Refutation at degree `c`: `1` lies in the degree-`c` closure. -/
def Refutes {K : Type} (L : Level K) (c : Nat) : Prop := Closure L.D L.G c L.one

/-- `c` is the last fall degree `d_F`: refutation at degree `c` and at no smaller degree. -/
def IsLastFall {K : Type} (L : Level K) (c : Nat) : Prop :=
  Refutes L c ∧ ∀ c', c' < c → ¬ Refutes L c'

/-- The hypotheses of Theorem 1 on the reduction map `φ : R_{t+1} → R_t` (source `A` = level
`t + 1`, target `B` = level `t`): sends `1` to `1`, linear, sends each variable to a variable or acts
as the identity (`y_t ↦ 1`), does not raise degree upper bounds, and maps generators to generators. -/
structure LevelMap {K : Type} (A B : Level K) where
  φ : A.R → B.R
  hone : φ A.one = B.one
  hzero : φ A.D.zero = B.D.zero
  hadd : ∀ a b, φ (A.D.add a b) = B.D.add (φ a) (φ b)
  hsmul : ∀ k a, φ (A.D.smul k a) = B.D.smul k (φ a)
  hmul : ∀ x, (∃ y, ∀ a, φ (A.D.mulv x a) = B.D.mulv y (φ a)) ∨ (∀ a, φ (A.D.mulv x a) = φ a)
  hdeg : ∀ a, B.D.deg (φ a) ≤ A.D.deg a
  hgen : ∀ r, A.G r → B.G (φ r)

/-- Theorem 1 (abstract): refutation at level `t + 1` and degree `c` gives refutation at level `t`
and degree `c`. -/
theorem refutes_descend {K : Type} {A B : Level K} (m : LevelMap A B) (c : Nat) :
    Refutes A c → Refutes B c :=
  closure_one A.D B.D A.G B.G c m.φ A.one B.one m.hone m.hzero m.hadd m.hsmul m.hmul m.hdeg m.hgen

/-- Contrapositive: a lower bound `d_F ≥ c` at level `t` lifts to level `t + 1`. -/
theorem lower_bound_lifts {K : Type} {A B : Level K} (m : LevelMap A B) (c : Nat)
    (hB : ∀ c', c' < c → ¬ Refutes B c') : ∀ c', c' < c → ¬ Refutes A c' :=
  fun c' hc hA => hB c' hc (refutes_descend m c' hA)

/-- Theorem 1 (abstract): `d_F(t) ≤ d_F(t + 1)`. -/
theorem lastFall_le {K : Type} {A B : Level K} (m : LevelMap A B) {c c' : Nat}
    (hB : IsLastFall B c) (hA : IsLastFall A c') : c ≤ c' := by
  cases Nat.lt_or_ge c' c with
  | inl hlt => exact absurd (refutes_descend m c' hA.1) (hB.2 c' hlt)
  | inr hge => exact hge

/-- The last fall degree, when it exists, is unique. -/
theorem lastFall_unique {K : Type} {L : Level K} {c c' : Nat}
    (h : IsLastFall L c) (h' : IsLastFall L c') : c = c' := by
  cases Nat.lt_or_ge c c' with
  | inl hlt => exact absurd h.1 (h'.2 c hlt)
  | inr hge =>
    cases Nat.lt_or_ge c' c with
    | inl hlt' => exact absurd h'.1 (h.2 c' hlt')
    | inr hge' => exact Nat.le_antisymm hge' hge

/-- A whole tower: levels `L t` and reduction maps `L (t+1) → L t`; refutation descends from any
higher level to any lower one. -/
theorem tower_refutes_descend {K : Type} (L : Nat → Level K) (m : ∀ t, LevelMap (L (t + 1)) (L t))
    (c t : Nat) : ∀ d, Refutes (L (t + d)) c → Refutes (L t) c := by
  intro d
  induction d with
  | zero => intro h; exact h
  | succ d ih => intro h; exact ih (refutes_descend (m (t + d)) c h)

/-- Theorem 1 along a tower: `t ≤ t'` implies `d_F(t) ≤ d_F(t')`. -/
theorem tower_lastFall_mono {K : Type} (L : Nat → Level K) (m : ∀ t, LevelMap (L (t + 1)) (L t))
    {t t' c c' : Nat} (htt : t ≤ t') (h : IsLastFall (L t) c) (h' : IsLastFall (L t') c') :
    c ≤ c' := by
  cases Nat.lt_or_ge c' c with
  | inl hlt =>
    have ht' : t' = t + (t' - t) := by omega
    rw [ht'] at h'
    exact absurd (tower_refutes_descend L m c' t (t' - t) h'.1) (h.2 c' hlt)
  | inr hge => exact hge

#print axioms pop_mod_le
#print axioms top_window
#print axioms closure_image
#print axioms closure_one
#print axioms pop_concat
#print axioms pop_two_pow
#print axioms pop_le_of_lt_two_pow
#print axioms pop_add_of_and_eq_zero
#print axioms pop_add_le
#print axioms pop_mul_mod_le
#print axioms pop_mulvar_le
#print axioms pop_le_four_eq_two_iff
#print axioms pop_le_two_of_le_four
#print axioms pop_le_one_of_le_two
#print axioms closure_mono_deg
#print axioms refutes_descend
#print axioms lower_bound_lifts
#print axioms lastFall_le
#print axioms lastFall_unique
#print axioms tower_refutes_descend
#print axioms tower_lastFall_mono

/-- Least-element argument: if `L` refutes at degree `c`, it has a last fall degree `c₀ ≤ c`. -/
theorem lastFall_exists_of_refutes {K : Type} (L : Level K) :
    ∀ c, Refutes L c → ∃ c₀, c₀ ≤ c ∧ IsLastFall L c₀ := by
  intro c
  induction c using Nat.strongRecOn with
  | _ c ih =>
    intro h
    by_cases hmin : ∀ c', c' < c → ¬ Refutes L c'
    · exact ⟨c, Nat.le_refl c, h, hmin⟩
    · have : ∃ c', c' < c ∧ Refutes L c' := by
        apply Classical.byContradiction
        intro hne
        apply hmin
        intro c' hc' hr
        exact hne ⟨c', hc', hr⟩
      obtain ⟨c', hc', hr⟩ := this
      obtain ⟨c₀, hle, hl⟩ := ih c' hc' hr
      exact ⟨c₀, Nat.le_trans hle (Nat.le_of_lt hc'), hl⟩

/-- Existence descends: a last fall degree `c` at level `t + 1` gives a last fall degree `c₀ ≤ c` at
level `t`. -/
theorem lastFall_descend_exists {K : Type} {A B : Level K} (m : LevelMap A B) {c : Nat}
    (hA : IsLastFall A c) : ∃ c₀, c₀ ≤ c ∧ IsLastFall B c₀ :=
  lastFall_exists_of_refutes B c (refutes_descend m c hA.1)

/-- Variant of `lastFall_le` needing `IsLastFall` only at level `t + 1`: the level-`t` last fall degree
exists and is at most `c'`. -/
theorem lastFall_le_of_top {K : Type} {A B : Level K} (m : LevelMap A B) {c' : Nat}
    (hA : IsLastFall A c') : ∃ c, IsLastFall B c ∧ c ≤ c' :=
  match lastFall_descend_exists m hA with
  | ⟨c₀, hle, hl⟩ => ⟨c₀, hl, hle⟩

end GrowthLaw
