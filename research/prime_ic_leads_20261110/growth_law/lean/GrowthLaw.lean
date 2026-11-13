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

#print axioms pop_mod_le
#print axioms top_window
#print axioms closure_image
#print axioms closure_one
