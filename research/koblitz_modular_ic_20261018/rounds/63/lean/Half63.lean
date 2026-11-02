import Mathlib.Tactic.FieldSimp
import Mathlib.Tactic.LinearCombination
import Lean.Elab.Tactic.Omega

namespace Half63

/-- Characteristic-two squaring is injective in a field. -/
theorem square_injective {K : Type*} [Field K] (hchar : (2 : K) = 0)
    {a b : K} (h : a^2 = b^2) : a = b := by
  have hz : (a-b)^2 = 0 := by
    linear_combination h + (-a*b+b^2)*hchar
  exact sub_eq_zero.mp (eq_zero_of_pow_eq_zero hz)

/-- A source endomorphism fixing a square also fixes its unique root. -/
theorem root_fixed {K : Type*} [Field K] (hchar : (2 : K) = 0)
    (phi : K →+* K) (f g : K) (hg : g^2 = f) (hf : phi f = f) : phi g = g := by
  apply square_injective hchar
  calc
    (phi g)^2 = phi (g^2) := (map_pow phi g 2).symm
    _ = phi f := congrArg phi hg
    _ = f := hf
    _ = g^2 := hg.symm

/-- The pole cap halves once the actual doubled valuation bound is known. -/
theorem pole_cap_half (valuation cap : ℤ) (h : -cap ≤ 2*valuation) :
    -(cap / 2) ≤ valuation := by omega

/-- A checked left inverse transfers an ambient involution to its restriction. -/
theorem restriction_involution {U V : Type*} (i : U → V) (r : V → U)
    (s : V → V) (t : U → U) (hr : Function.LeftInverse r i)
    (hinter : ∀ x, i (t x) = s (i x)) (hs : ∀ y, s (s y) = y) :
    ∀ x, t (t x) = x := by
  intro x
  apply hr.injective
  calc
    i (t (t x)) = s (i (t x)) := hinter (t x)
    _ = s (s (i x)) := congrArg s (hinter x)
    _ = i x := hs (i x)

/-- An explicit annihilating functional excludes a preimage in the stated space. -/
theorem separator_blocks_preimage {U K : Type*} [Zero K] (n : U → U)
    (ell : U → K) (c : U) (hzero : ∀ x, ell (n x) = 0) (hc : ell c ≠ 0) :
    ∀ x, n x ≠ c := by
  intro x hx
  apply hc
  calc
    ell c = ell (n x) := congrArg ell hx.symm
    _ = 0 := hzero x

end Half63
