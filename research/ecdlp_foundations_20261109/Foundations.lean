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

/-! ## Layer 0b: injectivity, modular reduction, and homomorphism reductions

These are the lemmas every reduction attack uses: a discrete log is unique on
`Fin n`, scalar multiples reduce modulo the order, and a group homomorphism
transports a DLP instance; hardness is preserved exactly when the map is
injective on the cyclic subgroup. -/

namespace ECDLP

variable {G H : Type} [AddGroup G] [AddGroup H]

/-- Reducing the exponent modulo the order does not change the multiple. -/
theorem nsmul_mod {a : G} {n k : Nat} (h : IsOrder a n) :
    nsmul (k % n) a = nsmul k a := by
  have hsplit : nsmul k a = add (nsmul (n * (k / n)) a) (nsmul (k % n) a) := by
    have := nsmul_add (n * (k / n)) (k % n) a
    rw [← (Nat.div_add_mod k n).symm] at this
    exact this
  have hz : nsmul (n * (k / n)) a = zero := by
    induction (k / n) with
    | zero => simp
    | succ q ih =>
        rw [Nat.mul_succ, nsmul_add, ih, h.annihilates]
        simp
  rw [hsplit, hz, zero_add]

/-- Below the order, distinct exponents give distinct group elements. -/
theorem nsmul_injective_below {a : G} {n : Nat} (h : IsOrder a n) {k l : Nat}
    (hk : k < n) (hl : l < n) (heq : nsmul k a = nsmul l a) : k = l := by
  rcases Nat.lt_trichotomy k l with hlt | heq' | hgt
  · have hdvd : n ∣ l - k := order_dvd_diff_of_nsmul_eq h (Nat.le_of_lt hlt) heq
    have hpos : 0 < l - k := Nat.sub_pos_of_lt hlt
    have hle : n ≤ l - k := Nat.le_of_dvd hpos hdvd
    have hlt' : l - k ≤ l := Nat.sub_le _ _
    omega
  · exact heq'
  · have hdvd : n ∣ k - l := order_dvd_diff_of_nsmul_eq h (Nat.le_of_lt hgt) heq.symm
    have hpos : 0 < k - l := Nat.sub_pos_of_lt hgt
    have hle : n ≤ k - l := Nat.le_of_dvd hpos hdvd
    have hlt' : k - l ≤ k := Nat.sub_le _ _
    omega

/-- A group homomorphism for the additive notation used here. -/
structure Hom where
  map : G → H
  map_zero : map zero = zero
  map_add : ∀ a b : G, map (add a b) = add (map a) (map b)

namespace Hom

variable (φ : Hom (G := G) (H := H))

theorem map_nsmul (k : Nat) (a : G) : φ.map (nsmul k a) = nsmul k (φ.map a) := by
  induction k with
  | zero => simp [φ.map_zero]
  | succ k ih => simp [nsmul_succ, φ.map_add, ih]

theorem map_neg (a : G) : φ.map (neg a) = neg (φ.map a) := by
  apply neg_unique
  rw [← φ.map_add, add_neg, φ.map_zero]

/-- Kernel triviality on the cyclic subgroup: an injective map keeps the DLP
unique. -/
theorem dlp_unique_of_injective {a q : G} {n x y : Nat} (h : IsOrder a n)
    (hx : nsmul x a = q) (hxlt : x < n) (hylt : y < n)
    (hmap : nsmul y (φ.map a) = φ.map q)
    (hinj : ∀ z : G, φ.map z = zero → z = zero) : y = x := by
  have hy : φ.map (nsmul y a) = φ.map q := by
    rw [φ.map_nsmul, hmap]
  have hzero : φ.map (add (nsmul y a) (neg (nsmul x a))) = zero := by
    rw [φ.map_add, φ.map_neg, hy, hx]
    exact add_neg _
  have hgroup : add (nsmul y a) (neg (nsmul x a)) = zero := hinj _ hzero
  have h1 : neg (nsmul x a) = neg (nsmul y a) := neg_unique hgroup
  have heq : nsmul x a = nsmul y a := by
    have := congrArg neg h1
    simpa [neg_neg] using this
  exact nsmul_injective_below h hylt hxlt heq.symm

/-- Any solution reduces modulo the order; the DLP is well defined in `Z/n`. -/
theorem dlp_mod_of_injective {a q : G} {n x y : Nat} (h : IsOrder a n)
    (hx : nsmul x a = q) (hxlt : x < n)
    (hmap : nsmul y (φ.map a) = φ.map q)
    (hinj : ∀ z : G, φ.map z = zero → z = zero) : y % n = x := by
  have hy : nsmul (y % n) (φ.map a) = φ.map q := by
    rw [← φ.map_nsmul, nsmul_mod h, φ.map_nsmul, hmap]
  have := dlp_unique_of_injective (φ := φ) h hx hxlt (Nat.mod_lt _ h.pos) hy hinj
  simpa using this

end Hom

end ECDLP

/-! ## Layer 3a: the arithmetic cores of BSGS and Pohlig--Hellman

These are the group/arithmetic lemmas the reduction algorithms rest on. -/

namespace ECDLP

variable {G : Type} [AddGroup G]

/-- Pohlig--Hellman subgroup step: if `d ∣ n` and `a` has order `n`, then
`d·a` has order `n/d`. Solving the DLP in the smaller subgroup recovers the
exponent modulo `n/d`. -/
theorem nsmul_isOrder_div {a : G} {n d : Nat} (h : IsOrder a n) (hd : d ∣ n)
    (hdpos : 0 < d) : IsOrder (nsmul d a) (n / d) := by
  have hnpos : 0 < n := h.pos
  refine ⟨Nat.div_pos (Nat.le_of_dvd hnpos hd) hdpos, ?_, ?_⟩
  · show nsmul (n / d) (nsmul d a) = zero
    rw [← nsmul_mul, Nat.div_mul_cancel hd]
    exact h.annihilates
  · intro m hm hlt hzero
    have hzero' : nsmul (m * d) a = zero := by
      rw [nsmul_mul]
      exact hzero
    have hmd : m * d = d * m := Nat.mul_comm m d
    rw [hmd] at hzero'
    have hpos : 0 < d * m := Nat.mul_pos hdpos hm
    have hlt' : d * m < n := by
      rw [← Nat.mul_div_cancel' hd]
      exact Nat.mul_lt_mul_of_pos_left hlt hdpos
    exact h.minimal hpos hlt' hzero'

/-- BSGS decomposition: with a table of `m` baby steps and a modulus bound
`n ≤ m²`, every exponent below `n` splits as `i·m + j` with `i,j < m`. -/
theorem bsgs_decomposition {n m x : Nat} (hm : 0 < m) (hx : x < n) (hn : n ≤ m * m) :
    ∃ i j : Nat, j < m ∧ i < m ∧ x = i * m + j := by
  refine ⟨x / m, x % m, Nat.mod_lt _ hm, ?_, ?_⟩
  · exact Nat.div_lt_of_lt_mul (Nat.lt_of_lt_of_le hx hn)
  · rw [Nat.mul_comm]
    exact (Nat.div_add_mod x m).symm

end ECDLP

/-! ## Layer 4 logic: the MOV reduction

The MOV attack does not break the group law; it transports the DLP through a
bilinear map to a target group. These lemmas formalise that reduction logic.
Constructing the Weil pairing and proving its non-degeneracy is separate. -/

namespace ECDLP

variable {G H : Type} [AddGroup G] [AddGroup H]

/-- A biadditive map, the algebraic interface of a pairing. -/
structure Bilin where
  e : G → G → H
  add_left : ∀ a b c : G, e (add a b) c = add (e a c) (e b c)
  add_right : ∀ a b c : G, e a (add b c) = add (e a b) (e a c)

namespace Bilin

variable (B : Bilin (G := G) (H := H))

theorem e_zero_left (b : G) : B.e zero b = zero := by
  have h : B.e zero b = add (B.e zero b) (B.e zero b) := by
    calc B.e zero b = B.e (add zero zero) b := by rw [zero_add]
      _ = add (B.e zero b) (B.e zero b) := B.add_left zero zero b
  have h2 : add (B.e zero b) zero = add (B.e zero b) (B.e zero b) := by
    rw [add_zero]
    exact h
  exact (add_left_cancel h2).symm

theorem e_zero_right (a : G) : B.e a zero = zero := by
  have h : B.e a zero = add (B.e a zero) (B.e a zero) := by
    calc B.e a zero = B.e a (add zero zero) := by rw [zero_add]
      _ = add (B.e a zero) (B.e a zero) := B.add_right a zero zero
  have h2 : add (B.e a zero) zero = add (B.e a zero) (B.e a zero) := by
    rw [add_zero]
    exact h
  exact (add_left_cancel h2).symm

theorem e_nsmul_left (k : Nat) (a b : G) :
    B.e (nsmul k a) b = nsmul k (B.e a b) := by
  induction k with
  | zero => simpa using B.e_zero_left b
  | succ k ih => simp [nsmul_succ, B.add_left, ih]

theorem e_nsmul_right (k : Nat) (a b : G) :
    B.e a (nsmul k b) = nsmul k (B.e a b) := by
  induction k with
  | zero => simpa using B.e_zero_right a
  | succ k ih => simp [nsmul_succ, B.add_right, ih]

/-- MOV reduction: if `e(a,a)` has the same order as `a`, then a solution of
the DLP in the target group is the unique source exponent below the order. -/
theorem mov_reduction {a : G} {n : Nat} (_h : IsOrder a n)
    (htarget : IsOrder (B.e a a) n) {x y : Nat} (hx : x < n) (hy : y < n)
    (hmap : nsmul y (B.e a a) = B.e a (nsmul x a)) : y = x := by
  have hmap' : nsmul y (B.e a a) = nsmul x (B.e a a) := by
    rw [hmap, B.e_nsmul_right]
  exact nsmul_injective_below htarget hy hx hmap'

end Bilin

end ECDLP
