import Mathlib

/-! Layer 4a: the MOV reduction and why non-degeneracy is necessary.

Foundations.lean already proves the reduction logic for an abstract biadditive
map.  This file puts the same statement on Mathlib's additive groups and adds
the two facts the ledger previously called assumptions:

* the target element `e(P,P)` always has order dividing the order of `P`, so
  the MOV target is never larger than the source;
* if that order is a proper divisor, the target DLP has at least two
  solutions, so a degenerate pairing cannot recover the logarithm.

The construction of the Weil pairing and the proof that `e(P,P)` has full
order remain the open part of Layer 4b. -/

namespace ECDLPMov

variable {G H : Type*} [AddCommGroup G] [AddCommGroup H]

/-- A biadditive map, the algebraic interface of a pairing. -/
structure Bilin where
  e : G → G → H
  add_left : ∀ a b c : G, e (a + b) c = e a c + e b c
  add_right : ∀ a b c : G, e a (b + c) = e a b + e a c

namespace Bilin

variable (B : Bilin (G := G) (H := H))

theorem e_zero_left (b : G) : B.e 0 b = 0 := by
  have h : B.e 0 b = B.e 0 b + B.e 0 b := by
    calc B.e 0 b = B.e (0 + 0) b := by rw [zero_add]
      _ = B.e 0 b + B.e 0 b := B.add_left 0 0 b
  have h2 : B.e 0 b + 0 = B.e 0 b + B.e 0 b := by rw [add_zero]; exact h
  exact (add_left_cancel h2).symm

theorem e_zero_right (a : G) : B.e a 0 = 0 := by
  have h : B.e a 0 = B.e a 0 + B.e a 0 := by
    calc B.e a 0 = B.e a (0 + 0) := by rw [zero_add]
      _ = B.e a 0 + B.e a 0 := B.add_right a 0 0
  have h2 : B.e a 0 + 0 = B.e a 0 + B.e a 0 := by rw [add_zero]; exact h
  exact (add_left_cancel h2).symm

theorem e_nsmul_left (k : ℕ) (a b : G) :
    B.e (k • a) b = k • B.e a b := by
  induction k with
  | zero => simpa using B.e_zero_left b
  | succ k ih => rw [succ_nsmul, B.add_left, ih, succ_nsmul]

theorem e_nsmul_right (k : ℕ) (a b : G) :
    B.e a (k • b) = k • B.e a b := by
  induction k with
  | zero => simpa using B.e_zero_right a
  | succ k ih => rw [succ_nsmul, B.add_right, ih, succ_nsmul]

/-- The MOV target order divides the source order. -/
theorem target_order_dvd (a : G) : addOrderOf (B.e a a) ∣ addOrderOf a := by
  rw [addOrderOf_dvd_iff_nsmul_eq_zero]
  rw [← B.e_nsmul_right (addOrderOf a) a a, addOrderOf_nsmul_eq_zero, B.e_zero_right]

/-- Kernel of the target map: `x • e(P,P) = 0` iff the target order divides
`x`. -/
theorem target_annihilator (a : G) (x : ℕ) :
    x • B.e a a = 0 ↔ addOrderOf (B.e a a) ∣ x :=
  addOrderOf_dvd_iff_nsmul_eq_zero.symm

/-- **Non-degeneracy is necessary.**  If `e(P,P)` has order `d < n` with
`d ∣ n`, then `0` and `d` are distinct exponents below `n` with the same
target value, so the target DLP has at least two solutions. -/
theorem mov_degenerate_ambiguity {a : G} {n d : ℕ}
    (hE : addOrderOf (B.e a a) = d) (_hdvd : d ∣ n)
    (hdpos : 0 < d) (hdlt : d < n) :
    ∃ y y' : ℕ, y ≠ y' ∧ y < n ∧ y' < n ∧
      y • B.e a a = y' • B.e a a := by
  refine ⟨0, d, by omega, by omega, hdlt, ?_⟩
  rw [zero_nsmul, ← hE]
  exact (addOrderOf_nsmul_eq_zero (B.e a a)).symm

/-- The target scalar map is injective below the order when the target has
full order. -/
theorem mov_target_injective {a : G} {n : ℕ}
    (hE : addOrderOf (B.e a a) = n) :
    Set.InjOn (fun x : ℕ => x • B.e a a) (Set.Iio n) := by
  intro x hx y hy hxy
  have hmod : x ≡ y [MOD n] := by
    have h := (nsmul_eq_nsmul_iff_modEq (x := B.e a a)).mp hxy
    simpa [hE] using h
  have hx' : x % n = x := Nat.mod_eq_of_lt hx
  have hy' : y % n = y := Nat.mod_eq_of_lt hy
  simpa [Nat.ModEq, hx', hy'] using hmod

/-- **MOV reduction.**  With `e(P,P)` of full order, a solution of the target
DLP below `n` is the unique source exponent. -/
theorem mov_reduction {a : G} {n : ℕ} (hE : addOrderOf (B.e a a) = n)
    {x y : ℕ} (hx : x < n) (hy : y < n)
    (hmap : y • B.e a a = B.e a (x • a)) : y = x := by
  have hmap' : y • B.e a a = x • B.e a a := by
    rw [hmap, B.e_nsmul_right]
  exact mov_target_injective B hE hy hx hmap'

end Bilin

end ECDLPMov
