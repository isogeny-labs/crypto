import Mathlib

/-! Layer 4b', the two pairing interfaces.

`Mov.lean` proves the MOV reduction for a biadditive map with `e(P,P)` of full
order.  That hypothesis is *not* satisfied by the Weil pairing: the Weil
pairing is alternating, so its diagonal is trivial.  This file records the
resulting no-go theorem and the correct interface, a bilinear map composed with
a distortion map `φ` (an additive endomorphism), which is what the
supersingular MOV attack uses.

Contents:

* `AltBilin`: a bilinear map with `e a a = 0`.
* `alt_diag_order_ne`: an alternating map can never have `e(P,P)` of order
  `n > 1`, so `Mov.mov_reduction` does not apply to the Weil pairing.
* `e_antisymm`: alternating bilinear maps are antisymmetric.
* `DistortedBilin`: `e : G × G' → H` plus `φ : G →+ G'`.
* `distorted_mov_reduction`: if `e(P, φ(P))` has full order, a target solution
  below the order is the unique source exponent.
-/

namespace ECDLPPairing

variable {G G' H : Type*} [AddCommGroup G] [AddCommGroup G'] [AddCommGroup H]

/-- A bilinear map that is alternating: `e a a = 0` for every `a`. -/
structure AltBilin (G H : Type*) [AddCommGroup G] [AddCommGroup H] where
  e : G → G → H
  add_left : ∀ a b c : G, e (a + b) c = e a c + e b c
  add_right : ∀ a b c : G, e a (b + c) = e a b + e a c
  alt : ∀ a : G, e a a = 0

namespace AltBilin

variable (B : AltBilin (G := G) (H := H))

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

theorem e_neg_left (a b : G) : B.e (-a) b = -B.e a b := by
  have h : B.e (-a) b + B.e a b = 0 := by
    rw [← B.add_left, neg_add_cancel, B.e_zero_left]
  exact eq_neg_of_add_eq_zero_left h

theorem e_neg_right (a b : G) : B.e a (-b) = -B.e a b := by
  have h : B.e a (-b) + B.e a b = 0 := by
    rw [← B.add_right, neg_add_cancel, B.e_zero_right]
  exact eq_neg_of_add_eq_zero_left h

theorem e_nsmul_left (k : ℕ) (a b : G) : B.e (k • a) b = k • B.e a b := by
  induction k with
  | zero => simpa using B.e_zero_left b
  | succ k ih => rw [succ_nsmul, B.add_left, ih, succ_nsmul]

/-- Alternating bilinear maps are antisymmetric. -/
theorem e_antisymm (a b : G) : B.e a b = -B.e b a := by
  have h := B.alt (a + b)
  rw [B.add_left, B.add_right, B.add_right, B.alt a, B.alt b] at h
  have h' : B.e a b + B.e b a = 0 := by
    simpa [add_assoc, add_comm, add_left_comm] using h
  exact eq_neg_of_add_eq_zero_left h'

/-- **No-go for the Weil pairing.**  An alternating map has trivial diagonal, so
`e(P,P)` has order `1`; it can never have order `n > 1`.  The MOV reduction of
`Mov.lean`, which assumes `addOrderOf (e P P) = n`, therefore cannot be applied
to the Weil pairing itself. -/
theorem alt_diag_order_ne {a : G} {n : ℕ} (hn : 1 < n) :
    addOrderOf (B.e a a) ≠ n := by
  rw [B.alt a, addOrderOf_zero]
  omega

end AltBilin

/-- A bilinear map on two groups, together with an additive distortion map
`φ`.  This is the interface of the supersingular MOV attack, where `e` is a
Tate/Weil-type pairing and `φ` a distortion map. -/
structure DistortedBilin (G G' H : Type*) [AddCommGroup G] [AddCommGroup G']
    [AddCommGroup H] where
  e : G → G' → H
  add_left : ∀ (a b : G) (c : G'), e (a + b) c = e a c + e b c
  add_right : ∀ (a : G) (b c : G'), e a (b + c) = e a b + e a c
  φ : G →+ G'

namespace DistortedBilin

variable (B : DistortedBilin (G := G) (G' := G') (H := H))

theorem e_zero_left (b : G') : B.e 0 b = 0 := by
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

theorem e_nsmul_right (k : ℕ) (a : G) (b : G') : B.e a (k • b) = k • B.e a b := by
  induction k with
  | zero => simpa using B.e_zero_right a
  | succ k ih => rw [succ_nsmul, B.add_right, ih, succ_nsmul]

/-- Scalar multiplication in the distortion: `e(P, φ(y·P)) = y · e(P, φ(P))`. -/
theorem e_phi_nsmul (k : ℕ) (a : G) :
    B.e a (B.φ (k • a)) = k • B.e a (B.φ a) := by
  rw [B.φ.map_nsmul, B.e_nsmul_right]

/-- Injectivity below the order of a target element. -/
theorem nsmul_injective_below {E : H} {n : ℕ} (hE : addOrderOf E = n) :
    Set.InjOn (fun y : ℕ => y • E) (Set.Iio n) := by
  intro x hx y hy hxy
  have hmod : x ≡ y [MOD n] := by
    have h := (nsmul_eq_nsmul_iff_modEq (x := E)).mp hxy
    simpa [hE] using h
  have hx' : x % n = x := Nat.mod_eq_of_lt hx
  have hy' : y % n = y := Nat.mod_eq_of_lt hy
  simpa [Nat.ModEq, hx', hy'] using hmod

/-- **Distorted MOV reduction.**  If `e(P, φ(P))` has full order `n` and
`Q = x·P`, a solution below `n` of `y · e(P, φ(P)) = e(P, φ(Q))` is exactly
`x`.  This is the form used when the pairing is alternating and a distortion
map provides the non-degeneracy. -/
theorem distorted_mov_reduction {P : G} {n : ℕ}
    (hE : addOrderOf (B.e P (B.φ P)) = n) {x y : ℕ} (hx : x < n) (hy : y < n)
    (hmap : y • B.e P (B.φ P) = B.e P (B.φ (x • P))) : y = x := by
  have hmap' : y • B.e P (B.φ P) = x • B.e P (B.φ P) := by
    rw [hmap, B.e_phi_nsmul]
  exact nsmul_injective_below hE hy hx hmap'

end DistortedBilin

end ECDLPPairing
