/-
Upper-bound note (research/prime_ic_leads_20261110/growth_law/upper_bound/README.md), formal parts.
Core Lean 4 only (no Mathlib). Pinned toolchain: leanprover/lean4:v4.35.0 (`lean-toolchain`).
Self-contained: `FilteredData`, `Closure`, `pop` and the `pop` helper lemmas are copied verbatim from
`../../lean/GrowthLaw.lean` so that this file compiles on its own.

What is formalised (abstract closure layer and `pop` arithmetic only):
* `closure_mulv_list`  : Lemma A. If `a` is in the degree-`c'` closure and `deg a + n ≤ c'`, then the
                         product of `a` with any `n` variables is in it (each variable raises degree by
                         at most one).
* `closure_trans`      : Lemma T. If every generator of `G'` lies in the degree-`c'` closure of `G` and
                         has degree ≤ c ≤ c', then the degree-`c` closure of `G'` lies in the
                         degree-`c'` closure of `G`.
* `closure_glue`       : Lemma G (gluing). A linear map `M` (multiplication by a fixed element `e` of
                         degree ≤ δ) that commutes with every variable and raises degree by at most δ
                         maps the degree-`c` closure of `G'` into the degree-`(c+δ)` closure of `G`,
                         provided `M` maps each generator of `G'` into the latter.
* `closure_lift_defect`: the abstract core of Theorem E. A linear lift `ι` that does not raise degree
                         and turns each variable step into a variable step plus a defect that is in
                         the target closure maps the degree-`c` closure into the degree-`c'` closure.
* `wrap_pop`           : if `e < 2^t`, `j < t` and `2^t ≤ e + 2^j` (the product y_j·z^e wraps), then
                         `pop (e + 2^j - 2^t) + (t - j) = pop e` (the wrapped monomial loses exactly
                         `t - j` bits; the defect bound of Theorem E).
* `orbit_all_ones`     : for `r < 2^s` and `s ≤ t`, the exponent `r + 2^s·(2^(t-s) - 1)` (low part `r`,
                         high bits s..t-1 all set) has `pop ≥ t - s` and is `< 2^t` (the combinatorial
                         core of Lemma B, the fibre-gluing barrier).

Not formalised: the concrete group algebras F_p[z1,z2]/(z_i^{2^t} − 1), the idempotents, the identity
of `Closure` with the subspace definition, the one-variable D8 chain, base change. These need
polynomial and linear-algebra libraries (Mathlib).
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

theorem pop_zero : pop 0 = 0 := by simp [pop]

theorem pop_pow_sub_one (k : Nat) : pop (2 ^ k - 1) = k := by
  induction k with
  | zero => simp [pop]
  | succ k ih =>
    have hp : 0 < 2 ^ k := Nat.two_pow_pos k
    have h : 2 ^ (k + 1) - 1 = 1 + 2 * (2 ^ k - 1) := by
      rw [Nat.pow_succ]; omega
    rw [h, pop_bit 1 _ (by decide), ih]
    omega

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

/-- `2^s · (2^(t-s) - 1) = 2^t - 2^s` for `s ≤ t`. -/
theorem two_pow_block (s t : Nat) (hst : s ≤ t) : 2 ^ s * (2 ^ (t - s) - 1) = 2 ^ t - 2 ^ s := by
  have ht : 2 ^ t = 2 ^ s * 2 ^ (t - s) := by
    rw [← Nat.pow_add]; congr 1; omega
  rw [Nat.mul_sub_one, ht]

/-- Lemma B, combinatorial core: an orbit of `+2^s` on exponents `< 2^t` contains the exponent whose
high bits `s..t-1` are all set; that exponent has popcount at least `t - s`. -/
theorem orbit_all_ones (s t r : Nat) (hst : s ≤ t) (hr : r < 2 ^ s) :
    t - s ≤ pop (r + 2 ^ s * (2 ^ (t - s) - 1)) ∧ r + 2 ^ s * (2 ^ (t - s) - 1) < 2 ^ t := by
  have hc := pop_concat s (2 ^ (t - s) - 1) r hr
  rw [pop_pow_sub_one] at hc
  have hb := two_pow_block s t hst
  have hps : 2 ^ s ≤ 2 ^ t := Nat.pow_le_pow_right (by decide) hst
  constructor
  · rw [Nat.add_comm, hc]; exact Nat.le_add_right _ _
  · rw [hb]; omega

/-- Wrap defect (Theorem E): if `y_j · z^e` wraps at level `t`, the wrapped exponent
`e + 2^j - 2^t` has exactly `t - j` fewer bits than `e`. -/
theorem wrap_pop (t j e : Nat) (hj : j < t) (he : e < 2 ^ t) (hwrap : 2 ^ t ≤ e + 2 ^ j) :
    pop (e + 2 ^ j - 2 ^ t) + (t - j) = pop e := by
  have hb := two_pow_block j t (Nat.le_of_lt hj)
  have hpj : 2 ^ j ≤ 2 ^ t := Nat.pow_le_pow_right (by decide) (Nat.le_of_lt hj)
  have hr : e + 2 ^ j - 2 ^ t < 2 ^ j := by omega
  have hdecomp : e = 2 ^ j * (2 ^ (t - j) - 1) + (e + 2 ^ j - 2 ^ t) := by
    rw [hb]; omega
  have hc := pop_concat j (2 ^ (t - j) - 1) (e + 2 ^ j - 2 ^ t) hr
  rw [pop_pow_sub_one] at hc
  rw [← hdecomp] at hc
  omega

/-! ### The abstract closure (copied from `GrowthLaw.lean`) -/

/-- The operations a closure needs: zero, addition, scalars, multiplication by a variable, degree. -/
structure FilteredData (K R X : Type) where
  zero : R
  add : R → R → R
  smul : K → R → R
  mulv : X → R → R
  deg : R → Nat

/-- The degree-c closure (PREREGISTRATION §2). -/
inductive Closure {K R X : Type} (D : FilteredData K R X) (G : R → Prop) (c : Nat) : R → Prop
  | gen {r : R} : G r → D.deg r ≤ c → Closure D G c r
  | zero : Closure D G c D.zero
  | add {a b : R} : Closure D G c a → Closure D G c b → Closure D G c (D.add a b)
  | smul {a : R} (k : K) : Closure D G c a → Closure D G c (D.smul k a)
  | mulv {a : R} (x : X) : Closure D G c a → D.deg a < c → Closure D G c (D.mulv x a)

/-! ### Lemmas A, T, G and the lift with defects -/

/-- Multiply by a list of variables, last element first. -/
def mulvList {K R X : Type} (D : FilteredData K R X) : List X → R → R
  | [], a => a
  | x :: xs, a => D.mulv x (mulvList D xs a)

/-- Lemma A: if each variable raises degree by at most one, `a ∈ V_{c'}` and `deg a + n ≤ c'`,
then `a` times any `n` variables is in `V_{c'}`. -/
theorem closure_mulv_list {K R X : Type} (D : FilteredData K R X) (G : R → Prop) (c' : Nat)
    (hvar : ∀ x a, D.deg (D.mulv x a) ≤ D.deg a + 1) :
    ∀ (xs : List X) (a : R), Closure D G c' a → D.deg a + xs.length ≤ c' →
      Closure D G c' (mulvList D xs a) ∧ D.deg (mulvList D xs a) ≤ D.deg a + xs.length := by
  intro xs
  induction xs with
  | nil => intro a ha _; exact ⟨ha, by simp [mulvList]⟩
  | cons x xs ih =>
    intro a ha hlen
    have hlen' : D.deg a + xs.length ≤ c' := by simp at hlen; omega
    obtain ⟨hc, hd⟩ := ih a ha hlen'
    have hlt : D.deg (mulvList D xs a) < c' := by simp at hlen; omega
    refine ⟨Closure.mulv x hc hlt, ?_⟩
    have := hvar x (mulvList D xs a)
    simp [mulvList]
    omega

/-- Lemma T (transitivity). -/
theorem closure_trans {K R X : Type} (D : FilteredData K R X) (G G' : R → Prop) {c c' : Nat}
    (hcc : c ≤ c') (hgen : ∀ r, G' r → D.deg r ≤ c → Closure D G c' r) :
    ∀ r, Closure D G' c r → Closure D G c' r := by
  intro r h
  induction h with
  | gen hr hd => exact hgen _ hr hd
  | zero => exact Closure.zero
  | add _ _ ih1 ih2 => exact Closure.add ih1 ih2
  | smul k _ ih => exact Closure.smul k ih
  | mulv x _ hlt ih => exact Closure.mulv x ih (Nat.lt_of_lt_of_le hlt hcc)

/-- Lemma G (gluing). `M` is multiplication by a fixed element of degree ≤ δ: linear, commuting with
every variable, raising degree by at most δ. If `M` sends every generator of `G'` (of degree ≤ c)
into the degree-`(c+δ)` closure of `G`, it sends the whole degree-`c` closure of `G'` there. -/
theorem closure_glue {K R X : Type} (D : FilteredData K R X) (G G' : R → Prop) (c δ : Nat)
    (M : R → R)
    (hzero : M D.zero = D.zero)
    (hadd : ∀ a b, M (D.add a b) = D.add (M a) (M b))
    (hsmul : ∀ k a, M (D.smul k a) = D.smul k (M a))
    (hcomm : ∀ x a, M (D.mulv x a) = D.mulv x (M a))
    (hdeg : ∀ a, D.deg (M a) ≤ D.deg a + δ)
    (hgen : ∀ r, G' r → D.deg r ≤ c → Closure D G (c + δ) (M r)) :
    ∀ r, Closure D G' c r → Closure D G (c + δ) (M r) := by
  intro r h
  induction h with
  | gen hr hd => exact hgen _ hr hd
  | zero => rw [hzero]; exact Closure.zero
  | add _ _ ih1 ih2 => rw [hadd]; exact Closure.add ih1 ih2
  | smul k _ ih => rw [hsmul]; exact Closure.smul k ih
  | @mulv a x _ hlt ih =>
    rw [hcomm]
    have := hdeg a
    exact Closure.mulv x ih (by omega)

/-- Theorem E, abstract core: a linear lift `ι` that does not raise degree, maps generators into
the target closure, and turns each variable step (on an element of degree < c) into a variable step
plus a defect lying in the target closure, maps the degree-`c` closure into the degree-`c'` closure
(`c ≤ c'`). -/
theorem closure_lift_defect {K R S X Y : Type} (D : FilteredData K R X) (E : FilteredData K S Y)
    (G : R → Prop) (H : S → Prop) {c c' : Nat} (hcc : c ≤ c') (ι : R → S)
    (hzero : ι D.zero = E.zero)
    (hadd : ∀ a b, ι (D.add a b) = E.add (ι a) (ι b))
    (hsmul : ∀ k a, ι (D.smul k a) = E.smul k (ι a))
    (hdeg : ∀ a, E.deg (ι a) ≤ D.deg a)
    (hgen : ∀ r, G r → D.deg r ≤ c → Closure E H c' (ι r))
    (hstep : ∀ x a, D.deg a < c →
      ∃ y d, ι (D.mulv x a) = E.add (E.mulv y (ι a)) d ∧ Closure E H c' d) :
    ∀ r, Closure D G c r → Closure E H c' (ι r) := by
  intro r h
  induction h with
  | gen hr hd => exact hgen _ hr hd
  | zero => rw [hzero]; exact Closure.zero
  | add _ _ ih1 ih2 => rw [hadd]; exact Closure.add ih1 ih2
  | smul k _ ih => rw [hsmul]; exact Closure.smul k ih
  | @mulv a x _ hlt ih =>
    obtain ⟨y, d, hy, hd⟩ := hstep x a hlt
    rw [hy]
    have := hdeg a
    exact Closure.add (Closure.mulv y ih (by omega)) hd

#print axioms orbit_all_ones
#print axioms wrap_pop
#print axioms closure_mulv_list
#print axioms closure_trans
#print axioms closure_glue
#print axioms closure_lift_defect
