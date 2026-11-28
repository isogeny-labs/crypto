import Mathlib
import Algorithms

/-! Layer 3d: correctness of Pollard's rho.

The probabilistic half of rho is the claim that the walk collides after about
`√(πn/2)` steps.  That is a probability statement about a random walk and
stays a cost premise.  The algebraic half is not probabilistic at all:

* a deterministic endofunction of a finite set always collides;
* Floyd's tortoise/hare iteration reaches a collision;
* a collision of two `a·P + b·Q` states yields a linear relation for the
  logarithm, and when the coefficient difference is invertible modulo the
  order, that relation determines the logarithm.

This file proves those three statements. -/

namespace ECDLPRho

/-! ## Finite orbits and Floyd's algorithm -/

/-- Iterates of a fixed point: if `f^[d] y = y` and `d ∣ m`, then `f^[m] y = y`. -/
theorem iterate_eq_of_dvd {α : Type*} (f : α → α) {d : ℕ} {y : α}
    (hy : f^[d] y = y) {m : ℕ} (h : d ∣ m) : f^[m] y = y := by
  obtain ⟨q, rfl⟩ := h
  induction q with
  | zero => simp
  | succ q ih =>
      rw [Nat.mul_succ, Function.iterate_add_apply, hy, ih]

/-- Iterates of two lengths commute on a point. -/
theorem iterate_comm_apply {α : Type*} (f : α → α) (m n : ℕ) (x : α) :
    f^[m] (f^[n] x) = f^[n] (f^[m] x) := by
  rw [← Function.iterate_add_apply, ← Function.iterate_add_apply, Nat.add_comm]

/-- Any endofunction of a finite type has a collision among its iterates:
two distinct times reach the same point. -/
theorem exists_collision_of_finite {α : Type*} [Fintype α] (f : α → α) (x : α) :
    ∃ i j : ℕ, i < j ∧ f^[i] x = f^[j] x := by
  by_contra h
  push Not at h
  have hinj : Function.Injective (fun i : Fin (Fintype.card α + 1) => f^[(i : ℕ)] x) := by
    intro i j hij
    by_cases hlt : (i : ℕ) < (j : ℕ)
    · exact absurd hij (h _ _ hlt)
    · have hle : (j : ℕ) ≤ (i : ℕ) := Nat.le_of_not_lt hlt
      rcases Nat.lt_or_eq_of_le hle with hlt' | heq
      · exact absurd hij.symm (h _ _ hlt')
      · exact Fin.ext heq.symm
  have hle := Fintype.card_le_of_injective _ hinj
  simp only [Fintype.card_fin] at hle
  omega

/-- A collision gives the eventual periodic structure `μ, λ` of the orbit. -/
theorem exists_tail_cycle {α : Type*} [Fintype α] (f : α → α) (x : α) :
    ∃ μ lam : ℕ, 0 < lam ∧ f^[μ + lam] x = f^[μ] x := by
  obtain ⟨i, j, hij, hcoll⟩ := exists_collision_of_finite f x
  exact ⟨i, j - i, Nat.sub_pos_of_lt hij, by
    simpa [Nat.add_sub_of_le (Nat.le_of_lt hij)] using hcoll.symm⟩

/-- **Floyd's tortoise/hare meeting.**  Some positive time `k` has
`f^[k] x = f^[2k] x`, which is exactly the collision Floyd's algorithm reports. -/
theorem floyd_meets {α : Type*} [Fintype α] (f : α → α) (x : α) :
    ∃ k : ℕ, 0 < k ∧ f^[k] x = f^[2 * k] x := by
  obtain ⟨i, j, hij, hcoll⟩ := exists_collision_of_finite f x
  set d := j - i with hd
  have hdpos : 0 < d := Nat.sub_pos_of_lt hij
  have hj : i + d = j := Nat.add_sub_of_le (Nat.le_of_lt hij)
  -- the point after `i` steps is fixed by `f^[d]`
  have hy : f^[d] (f^[i] x) = f^[i] x := by
    have h1 : f^[i + d] x = f^[i] (f^[d] x) := Function.iterate_add_apply f i d x
    have h2 : f^[i] (f^[d] x) = f^[d] (f^[i] x) := iterate_comm_apply f i d x
    rw [← hj] at hcoll
    rw [h1, h2] at hcoll
    exact hcoll.symm
  set k := d * (i / d + 1) with hkdef
  have hkpos : 0 < k := Nat.mul_pos hdpos (Nat.succ_pos _)
  have hkdvd : d ∣ k := ⟨i / d + 1, rfl⟩
  have hkge : i ≤ k := by
    have h1 : i = d * (i / d) + i % d := (Nat.div_add_mod i d).symm
    have h2 : i % d < d := Nat.mod_lt _ hdpos
    calc i = d * (i / d) + i % d := h1
      _ ≤ d * (i / d) + d := Nat.add_le_add_left (Nat.le_of_lt h2) _
      _ = d * (i / d + 1) := by rw [Nat.mul_succ]
      _ = k := rfl
  have h2k : i ≤ 2 * k := by omega
  refine ⟨k, hkpos, ?_⟩
  have hk_eq : f^[k] x = f^[k - i] (f^[i] x) := by
    conv_lhs => rw [← Nat.add_sub_of_le hkge]
    rw [Function.iterate_add_apply, iterate_comm_apply]
  have h2k_eq : f^[2 * k] x = f^[2 * k - i] (f^[i] x) := by
    conv_lhs => rw [← Nat.add_sub_of_le h2k]
    rw [Function.iterate_add_apply, iterate_comm_apply]
  have hky : f^[k] (f^[i] x) = f^[i] x := iterate_eq_of_dvd f hy hkdvd
  have hsum : 2 * k - i = (k - i) + k := by omega
  calc f^[k] x = f^[k - i] (f^[i] x) := hk_eq
    _ = f^[(k - i) + k] (f^[i] x) := by
        rw [Function.iterate_add_apply, hky]
    _ = f^[2 * k] x := by
        rw [h2k_eq, hsum]

/-! ## From a collision to the discrete logarithm -/

section Recover

variable {G : Type*} [AddCommGroup G]

/-- **Rho relation.**  Two colliding walk states `a·P + b·Q` and
`a'·P + b'·Q`, with `Q = x·P`, satisfy the linear congruence
`(a - a') + (b - b')·x ≡ 0 (mod n)` in `ZMod n`. -/
theorem rho_relation_mod {P : G} {n : ℕ} (hord : addOrderOf P = n)
    {Q : G} {x a b a' b' : ℕ} (hQ : Q = x • P)
    (hcoll : a • P + b • Q = a' • P + b' • Q) :
    ((a : ZMod n) - a') + ((b : ZMod n) - b') * (x : ZMod n) = 0 := by
  have hcoef : (a + b * x) • P = (a' + b' * x) • P := by
    rw [hQ] at hcoll
    rw [← mul_nsmul P x b, ← mul_nsmul P x b'] at hcoll
    rw [Nat.mul_comm x b, Nat.mul_comm x b'] at hcoll
    rw [← add_nsmul, ← add_nsmul] at hcoll
    exact hcoll
  have hmod : a + b * x ≡ a' + b' * x [MOD n] := by
    have h := (nsmul_eq_nsmul_iff_modEq (x := P)).mp hcoef
    simpa [hord] using h
  have hcast : ((a + b * x : ℕ) : ZMod n) = ((a' + b' * x : ℕ) : ZMod n) := by
    rw [ZMod.natCast_eq_natCast_iff']
    exact hmod
  push_cast at hcast
  linear_combination hcast

/-- Solving the rho relation in a field of prime order. -/
theorem rho_solve {n : ℕ} [Fact n.Prime] {a b a' b' x : ZMod n}
    (h : (a - a') + (b - b') * x = 0) (hb : b - b' ≠ 0) :
    x = -(a - a') / (b - b') := by
  have h1 : (b - b') * x = -(a - a') := by linear_combination h
  rw [eq_div_iff hb, mul_comm]
  exact h1

/-- **Rho recovers the discrete log.**  If the order `n` is prime and the two
colliding walks have distinct `Q`-coefficients, then the collision determines
`x` uniquely modulo `n`.  This is the algebraic correctness of rho; the
expected number of steps until such a collision stays a probability premise. -/
theorem rho_dlp_recover {P : G} {n : ℕ} [Fact n.Prime] {Q : G} {x a b a' b' : ℕ}
    (hord : addOrderOf P = n) (hQ : Q = x • P)
    (hcoll : a • P + b • Q = a' • P + b' • Q)
    (hb : (b : ZMod n) - b' ≠ 0) :
    (x : ZMod n) = -((a : ZMod n) - a') / ((b : ZMod n) - b') := by
  exact rho_solve (rho_relation_mod hord hQ hcoll) hb

end Recover

end ECDLPRho
