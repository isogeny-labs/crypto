import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic
import CryptoLean.Curve.Conductor

/-!
# Floor endomorphism obstruction and Hasse-interval thinness

Using `CryptoLean.quadraticNorm`, an element of the ideal `ℓ · O` has norm
divisible by `ℓ²`. A quadratic congruence over a field has at most two roots,
giving Hasse-interval thinness for conductor-prime traces.

**Not formalized here:** `End(E) ≃ O_f` identification.
-/

set_option autoImplicit false

namespace FloorObstructionAndDensity

open CryptoLean

/-- Floor endomorphism in the ideal `ℓ · O`. -/
structure FloorEndomorphism (ℓ : ℕ) (a b : ℤ) : Prop where
  hℓ : Nat.Prime ℓ
  ha : (ℓ : ℤ) ∣ a
  hb : (ℓ : ℤ) ∣ b

/-- The quadratic norm of a floor endomorphism is divisible by `ℓ²`. -/
theorem floor_endomorphism_sq_dvd_norm {ℓ : ℕ} {traceOmega normOmega a b : ℤ}
    (h : FloorEndomorphism ℓ a b) :
    (ℓ : ℤ) ^ 2 ∣ quadraticNorm traceOmega normOmega a b := by
  obtain ⟨a', ha⟩ := h.ha
  obtain ⟨b', hb⟩ := h.hb
  rw [ha, hb, quadraticNorm]
  exact ⟨a' ^ 2 + traceOmega * a' * b' + normOmega * b' ^ 2, by ring⟩

/-- No floor endomorphism can have norm coprime to `ℓ` (equivalently: not
divisible by `ℓ`), since the norm is divisible by `ℓ²`. -/
theorem no_coprime_cofactor_floor_endomorphism {ℓ : ℕ} {traceOmega normOmega a b : ℤ}
    (h : FloorEndomorphism ℓ a b)
    (hcop : ¬ (ℓ : ℤ) ∣ quadraticNorm traceOmega normOmega a b) : False := by
  have hdiv := floor_endomorphism_sq_dvd_norm (traceOmega := traceOmega) (normOmega := normOmega) h
  exact hcop (dvd_trans (dvd_pow_self (ℓ : ℤ) (by decide : (2 : ℕ) ≠ 0)) hdiv)

/-- No Kani-style pure power-of-two floor endomorphism for an odd prime `ℓ`. -/
theorem no_kani_two_power_floor_endomorphism {ℓ k : ℕ} {traceOmega normOmega a b : ℤ}
    (h : FloorEndomorphism ℓ a b) (hℓodd : Odd ℓ)
    (hnorm : quadraticNorm traceOmega normOmega a b = (2 : ℤ) ^ k) : False := by
  have hdiv := floor_endomorphism_sq_dvd_norm (traceOmega := traceOmega) (normOmega := normOmega) h
  rw [hnorm] at hdiv
  have hp := h.hℓ
  have hℓ2 : (ℓ : ℤ) ∣ (2 : ℤ) ^ k :=
    dvd_trans (dvd_pow_self (ℓ : ℤ) (by decide : (2 : ℕ) ≠ 0)) hdiv
  have hnat : ℓ ∣ 2 ^ k := Int.natCast_dvd_natCast.mp (by simpa using hℓ2)
  -- An odd prime cannot divide a power of 2.
  have hcop : Nat.Coprime ℓ 2 := by
    rw [Nat.coprime_two_right, Nat.odd_iff]
    exact Nat.odd_iff.mp hℓodd
  have : ¬ ℓ ∣ 2 ^ k := hp.coprime_iff_not_dvd.1 (hcop.pow_right k)
  exact this hnat

/-- In a field, `X² − a` has at most two roots. -/
theorem sq_congruence_dichotomy {ℓ : ℕ} [Fact ℓ.Prime] (a : ZMod ℓ) (x y z : ZMod ℓ)
    (hx : x ^ 2 = a) (hy : y ^ 2 = a) (hz : z ^ 2 = a)
    (hxy : x ≠ y) (hxz : x ≠ z) : y = z := by
  have h1 : (x - y) * (x + y) = 0 := by linear_combination hx - hy
  have h2 : (x - z) * (x + z) = 0 := by linear_combination hx - hz
  have hyx : y = -x := by
    have : x + y = 0 := (mul_eq_zero.1 h1).resolve_left fun h => hxy (sub_eq_zero.1 h)
    linear_combination this
  have hzx : z = -x := by
    have : x + z = 0 := (mul_eq_zero.1 h2).resolve_left fun h => hxz (sub_eq_zero.1 h)
    linear_combination this
  simp [hyx, hzx]

/-- At most two solutions of `x² ≡ a (mod ℓ)`. -/
theorem card_sq_congruence_window_le_two {ℓ : ℕ} [Fact ℓ.Prime] (a : ZMod ℓ) :
    (Finset.univ.filter fun x : ZMod ℓ => x ^ 2 = a).card ≤ 2 := by
  classical
  let S := Finset.univ.filter fun x : ZMod ℓ => x ^ 2 = a
  change S.card ≤ 2
  by_cases hempty : S = ∅
  · simp [hempty]
  · obtain ⟨r, hr⟩ := Finset.nonempty_iff_ne_empty.2 hempty
    have hrA : r ^ 2 = a := (Finset.mem_filter.1 hr).2
    have hsub : S ⊆ ({r, -r} : Finset (ZMod ℓ)) := by
      intro x hx
      have hxA : x ^ 2 = a := (Finset.mem_filter.1 hx).2
      have : (r - x) * (r + x) = 0 := by linear_combination hrA - hxA
      rcases mul_eq_zero.1 this with h | h
      · exact Finset.mem_insert.2 (Or.inl (sub_eq_zero.1 h).symm)
      · exact Finset.mem_insert.2 (Or.inr (Finset.mem_singleton.2 (by
          have : x = -r := by linear_combination h
          exact this)))
    exact (Finset.card_le_card hsub).trans Finset.card_le_two

/-- Interval form: any subset still has at most two hits. -/
theorem card_sq_congruence_interval_bound {ℓ : ℕ} [Fact ℓ.Prime]
    (a : ZMod ℓ) (T : Finset (ZMod ℓ)) :
    (T.filter fun x => x ^ 2 = a).card ≤ 2 := by
  classical
  have hsub : T.filter (fun x => x ^ 2 = a) ⊆ Finset.univ.filter fun x => x ^ 2 = a := by
    intro x hx
    exact Finset.mem_filter.2 ⟨Finset.mem_univ _, (Finset.mem_filter.1 hx).2⟩
  exact (Finset.card_le_card hsub).trans (card_sq_congruence_window_le_two a)

/-- Hasse-interval thinness for conductor-prime trace congruences. -/
theorem hasse_trace_conductor_prime_thinness {ℓ : ℕ} [Fact ℓ.Prime]
    (a : ZMod ℓ) (T : Finset (ZMod ℓ)) :
    (T.filter fun x => x ^ 2 = a).card ≤ 2 :=
  card_sq_congruence_interval_bound a T

end FloorObstructionAndDensity
