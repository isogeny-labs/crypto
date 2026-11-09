import Init
set_option autoImplicit false
set_option linter.unusedSectionVars false

/-! Foundations of the discrete-log hardness stack, layer 0: the abstract
group and scalar-action lemmas every ECDLP claim rests on.

Nothing here assumes a curve. These lemmas are the interface that curve
attacks (Pohlig--Hellman, BSGS, rho, MOV, anomalous) and lower bounds
(Shoup's generic-group bound) all quantify over. Formalizing this layer first
keeps the later statements honest: "DLP is well defined mod the order" is a
theorem, not an assumption. -/

class AddGroup (G : Type) where
  zero : G
  add : G → G → G
  neg : G → G
  add_assoc : ∀ a b c : G, add (add a b) c = add a (add b c)
  add_comm : ∀ a b : G, add a b = add b a
  add_zero : ∀ a : G, add a zero = a
  add_neg : ∀ a : G, add a (neg a) = zero

namespace ECDLP

variable {G : Type} [AddGroup G]

def add (a b : G) : G := AddGroup.add a b
def neg (a : G) : G := AddGroup.neg a
def zero : G := AddGroup.zero
def sub (a b : G) : G := add a (neg b)

@[simp] theorem add_assoc (a b c : G) : add (add a b) c = add a (add b c) :=
  AddGroup.add_assoc a b c
@[simp] theorem add_comm (a b : G) : add a b = add b a := AddGroup.add_comm a b
@[simp] theorem add_zero (a : G) : add a zero = a := AddGroup.add_zero a
@[simp] theorem add_neg (a : G) : add a (neg a) = zero := AddGroup.add_neg a
@[simp] theorem zero_add (a : G) : add zero a = a := by
  rw [add_comm, add_zero]
@[simp] theorem neg_add_self (a : G) : add (neg a) a = zero := by
  rw [add_comm, add_neg]

theorem add_left_comm (a b c : G) : add a (add b c) = add b (add a c) := by
  rw [← add_assoc, add_comm a b, add_assoc]

theorem add_right_comm (a b c : G) : add (add a b) c = add (add a c) b := by
  rw [add_assoc, add_comm b c, ← add_assoc]

theorem add_left_cancel {a b c : G} (h : add a b = add a c) : b = c := by
  have h' := congrArg (fun t => add (neg a) t) h
  rw [← add_assoc, ← add_assoc, neg_add_self, zero_add, zero_add] at h'
  exact h'

theorem add_right_cancel {a b c : G} (h : add a c = add b c) : a = b := by
  exact add_left_cancel (a := c) (by
    rw [add_comm c a, add_comm c b]
    exact h)

theorem neg_unique {a b : G} (h : add a b = zero) : b = neg a := by
  apply add_left_cancel (a := a)
  rw [h, add_neg]

theorem neg_neg (a : G) : neg (neg a) = a := by
  have h := neg_unique (a := neg a) (b := a) (by rw [neg_add_self])
  exact h.symm

theorem sub_eq_add_neg (a b : G) : sub a b = add a (neg b) := rfl

theorem add_sub_cancel (a b : G) : add (sub a b) b = a := by
  rw [sub, add_assoc, neg_add_self, add_zero]

theorem sub_self (a : G) : sub a a = zero := by
  simp [sub, add_neg]

/-- Scalar multiplication by a natural number. -/
def nsmul : Nat → G → G
  | 0, _ => zero
  | n + 1, a => add (nsmul n a) a

@[simp] theorem nsmul_zero (a : G) : nsmul 0 a = zero := rfl
@[simp] theorem nsmul_succ (n : Nat) (a : G) : nsmul (n + 1) a = add (nsmul n a) a := rfl

theorem nsmul_add (m n : Nat) (a : G) : nsmul (m + n) a = add (nsmul m a) (nsmul n a) := by
  induction n with
  | zero => simp
  | succ n ih =>
      rw [Nat.add_succ, nsmul_succ, ih, nsmul_succ, add_assoc]

theorem nsmul_mul (m n : Nat) (a : G) : nsmul (m * n) a = nsmul m (nsmul n a) := by
  induction m with
  | zero => simp
  | succ m ih =>
      rw [Nat.succ_mul, nsmul_add, ih, nsmul_succ]

theorem nsmul_one (a : G) : nsmul 1 a = a := by
  simp

@[simp] theorem nsmul_zero_right (n : Nat) : nsmul n (zero : G) = zero := by
  induction n with
  | zero => simp
  | succ n ih => simp [ih]

/-- `a` has exact order `n`: positive, annihilated by `n`, minimal. -/
def IsOrder (a : G) (n : Nat) : Prop :=
  0 < n ∧ nsmul n a = zero ∧ ∀ m : Nat, 0 < m → m < n → nsmul m a ≠ zero

theorem IsOrder.pos {a : G} {n : Nat} (h : IsOrder a n) : 0 < n := h.1

theorem IsOrder.annihilates {a : G} {n : Nat} (h : IsOrder a n) : nsmul n a = zero := h.2.1

theorem IsOrder.minimal {a : G} {n m : Nat} (h : IsOrder a n) (hm : 0 < m) (hlt : m < n) :
    nsmul m a ≠ zero := h.2.2 m hm hlt

/-- Any annihilating exponent is a multiple of the exact order. This is the
lemma behind "the discrete log is well defined modulo the order". -/
theorem order_dvd_of_annihilates {a : G} {n k : Nat} (h : IsOrder a n) (hk : nsmul k a = zero) :
    n ∣ k := by
  have hr_lt : k % n < n := Nat.mod_lt _ h.pos
  have hdiv : k = n * (k / n) + k % n := (Nat.div_add_mod k n).symm
  have hsplit : nsmul k a = add (nsmul (n * (k / n)) a) (nsmul (k % n) a) := by
    have := nsmul_add (n * (k / n)) (k % n) a
    rw [← hdiv] at this
    exact this
  have hz : nsmul (n * (k / n)) a = zero := by
    induction (k / n) with
    | zero => simp
    | succ q ih =>
        rw [Nat.mul_succ, nsmul_add, ih, h.annihilates]
        simp
  have hrem : nsmul (k % n) a = zero := by
    rw [hsplit, hz, zero_add] at hk
    exact hk
  cases Nat.eq_zero_or_pos (k % n) with
  | inl hzero => exact Nat.dvd_of_mod_eq_zero hzero
  | inr hpos => exact absurd hrem (h.minimal hpos hr_lt)

/-- If two exponents give the same group element and `k ≤ l`, the order
divides their difference: discrete logs are unique modulo the order. -/
theorem order_dvd_sub_of_add_eq {a : G} {n k l : Nat} (h : IsOrder a n)
    (hkl : k ≤ l) (heq : add (nsmul k a) (nsmul (l - k) a) = nsmul k a) : n ∣ l - k := by
  apply order_dvd_of_annihilates h
  have h1 : nsmul l a = add (nsmul k a) (nsmul (l - k) a) := by
    have := nsmul_add k (l - k) a
    rw [Nat.add_sub_of_le hkl] at this
    exact this
  have h2 : add (nsmul k a) (nsmul (l - k) a) = add (nsmul k a) zero := by
    rw [add_zero]
    calc add (nsmul k a) (nsmul (l - k) a) = nsmul l a := h1.symm
      _ = nsmul k a := by rw [h1]; exact heq
  exact add_left_cancel h2

/-- The same statement for equal scalar multiples: equal outputs differ by a
multiple of the order. -/
theorem order_dvd_diff_of_nsmul_eq {a : G} {n k l : Nat} (h : IsOrder a n) (hkl : k ≤ l)
    (heq : nsmul k a = nsmul l a) : n ∣ l - k := by
  apply order_dvd_sub_of_add_eq h hkl
  rw [← nsmul_add, Nat.add_sub_of_le hkl, heq]

end ECDLP
