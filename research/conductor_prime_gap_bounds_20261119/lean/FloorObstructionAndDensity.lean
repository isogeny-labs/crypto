import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic
import CryptoLean.Curve.Conductor

/-!
# Floor endomorphism obstruction and Hasse-interval thinness

Using `CryptoLean.quadraticNorm`, an element of the ideal `ℓ · O` has norm
divisible by `ℓ²`. A quadratic congruence over a field has at most two roots;
across a Hasse window `[-B, B]` each residue class contributes
`O(B/ℓ)` hits, giving density `O(1/ℓ)`.

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

/-- No floor endomorphism can have norm coprime to `ℓ`. -/
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

/-- At most `N/ℓ + 1` naturals in `{0, …, N}` lie in a fixed residue class mod `ℓ`. -/
theorem card_residue_class_in_range {ℓ : ℕ} (hℓ : 0 < ℓ) (r N : ℕ) :
    ((Finset.range (N + 1)).filter fun n => n % ℓ = r % ℓ).card ≤ N / ℓ + 1 := by
  classical
  let S := (Finset.range (N + 1)).filter fun n => n % ℓ = r % ℓ
  let K := S.image fun n => n / ℓ
  have hinj : Set.InjOn (fun n : ℕ => n / ℓ) (S : Set ℕ) := by
    intro x hx y hy hxy
    have hxR : x % ℓ = r % ℓ := (Finset.mem_filter.1 hx).2
    have hyR : y % ℓ = r % ℓ := (Finset.mem_filter.1 hy).2
    have hmod : x % ℓ = y % ℓ := by simp [hxR, hyR]
    have hdiv : x / ℓ = y / ℓ := hxy
    calc
      x = ℓ * (x / ℓ) + x % ℓ := (Nat.div_add_mod x ℓ).symm
      _ = ℓ * (y / ℓ) + y % ℓ := by rw [hdiv, hmod]
      _ = y := Nat.div_add_mod y ℓ
  have hKcard : K.card = S.card := Finset.card_image_of_injOn hinj
  have hk_le : ∀ k ∈ K, k ≤ N / ℓ := by
    intro k hk
    obtain ⟨n, hn, rfl⟩ := Finset.mem_image.1 hk
    exact Nat.div_le_div_right (Nat.lt_succ_iff.mp (Finset.mem_range.1 (Finset.mem_filter.1 hn).1))
  have hKsub : K ⊆ Finset.range (N / ℓ + 1) := fun k hk =>
    Finset.mem_range.2 (Nat.lt_succ_of_le (hk_le k hk))
  calc
    S.card = K.card := hKcard.symm
    _ ≤ (Finset.range (N / ℓ + 1)).card := Finset.card_le_card hKsub
    _ = N / ℓ + 1 := Finset.card_range _

/-- Shift `[-B, B]` onto `{0, …, 2B}`; each residue class has ≤ `2B/ℓ + 1` hits. -/
theorem card_residue_class_in_Icc {ℓ : ℕ} [NeZero ℓ] (hℓ : 0 < ℓ) (r : ZMod ℓ) (B : ℕ) :
    ((Finset.Icc (-(B : ℤ)) (B : ℤ)).filter fun t : ℤ => (t : ZMod ℓ) = r).card ≤
      2 * B / ℓ + 1 := by
  classical
  let I : Finset ℤ := Finset.Icc (-(B : ℤ)) (B : ℤ)
  let S : Finset ℤ := I.filter fun t : ℤ => (t : ZMod ℓ) = r
  let φ : ℤ → ℕ := fun t => Int.toNat (t + (B : ℤ))
  have hφ_spec : ∀ t ∈ I, (φ t : ℤ) = t + B ∧ φ t ≤ 2 * B := by
    intro t ht
    have ⟨hle, hge⟩ := Finset.mem_Icc.1 ht
    have hnn : 0 ≤ t + (B : ℤ) := by omega
    have hle' : t + (B : ℤ) ≤ 2 * (B : ℤ) := by omega
    refine ⟨Int.toNat_of_nonneg hnn, ?_⟩
    exact_mod_cast (Int.toNat_le.mpr hle')
  have hinj : Set.InjOn φ (S : Set ℤ) := by
    intro x hx y hy hxy
    have ⟨hxeq, _⟩ := hφ_spec x (Finset.mem_filter.1 hx).1
    have ⟨hyeq, _⟩ := hφ_spec y (Finset.mem_filter.1 hy).1
    omega
  let T := S.image φ
  have hTcard : T.card = S.card := Finset.card_image_of_injOn hinj
  let rB : ℕ := ZMod.val ((r : ZMod ℓ) + (B : ZMod ℓ))
  have hTsub : T ⊆ (Finset.range (2 * B + 1)).filter fun n => n % ℓ = rB % ℓ := by
    intro n hn
    obtain ⟨t, ht, rfl⟩ := Finset.mem_image.1 hn
    have htI := (Finset.mem_filter.1 ht).1
    have htR : (t : ZMod ℓ) = r := (Finset.mem_filter.1 ht).2
    have ⟨htEq, hle⟩ := hφ_spec t htI
    refine Finset.mem_filter.2 ⟨Finset.mem_range.2 (Nat.lt_succ_of_le hle), ?_⟩
    have hφZ : (φ t : ZMod ℓ) = (t : ZMod ℓ) + (B : ZMod ℓ) := by
      have := congrArg (fun z : ℤ => (z : ZMod ℓ)) htEq
      simpa using this
    have heq : (φ t : ZMod ℓ) = (r : ZMod ℓ) + (B : ZMod ℓ) := by rw [hφZ, htR]
    have hval : (φ t : ZMod ℓ).val = φ t % ℓ := ZMod.val_natCast ℓ (φ t)
    rw [← hval, heq]
    exact (Nat.mod_eq_of_lt (ZMod.val_lt ((r : ZMod ℓ) + (B : ZMod ℓ)))).symm
  have hle : T.card ≤ ((Finset.range (2 * B + 1)).filter fun n => n % ℓ = rB % ℓ).card :=
    Finset.card_le_card hTsub
  have hbound := card_residue_class_in_range hℓ rB (2 * B)
  calc
    S.card = T.card := hTcard.symm
    _ ≤ _ := hle
    _ ≤ 2 * B / ℓ + 1 := hbound

/-- Hasse-interval thinness: among traces in `[-B, B]`, the congruence
`t² ≡ a (mod ℓ)` hits at most `2 · (2B/ℓ + 1)` integers. -/
theorem hasse_trace_conductor_prime_thinness {ℓ : ℕ} [hp : Fact ℓ.Prime]
    (a : ZMod ℓ) (B : ℕ) :
    ((Finset.Icc (-(B : ℤ)) (B : ℤ)).filter fun t : ℤ => (t : ZMod ℓ) ^ 2 = a).card ≤
      2 * (2 * B / ℓ + 1) := by
  classical
  let I : Finset ℤ := Finset.Icc (-(B : ℤ)) (B : ℤ)
  let S : Finset ℤ := I.filter fun t : ℤ => (t : ZMod ℓ) ^ 2 = a
  let R : Finset (ZMod ℓ) := Finset.univ.filter fun x : ZMod ℓ => x ^ 2 = a
  have hR : R.card ≤ 2 := card_sq_congruence_window_le_two a
  let fiber : ZMod ℓ → Finset ℤ := fun r => I.filter fun t : ℤ => (t : ZMod ℓ) = r
  have hcover : S ⊆ R.biUnion fiber := by
    intro t ht
    have htI : t ∈ I := (Finset.mem_filter.1 ht).1
    have htA : (t : ZMod ℓ) ^ 2 = a := (Finset.mem_filter.1 ht).2
    refine Finset.mem_biUnion.2 ⟨(t : ZMod ℓ), Finset.mem_filter.2 ⟨Finset.mem_univ _, htA⟩,
      Finset.mem_filter.2 ⟨htI, rfl⟩⟩
  have hcard := Finset.card_le_card hcover
  have hbunion := Finset.card_biUnion_le (s := R) (t := fiber)
  have hℓpos : 0 < ℓ := Nat.Prime.pos hp.out
  have : NeZero ℓ := ⟨ne_of_gt hℓpos⟩
  have hfib : ∀ r ∈ R, (fiber r).card ≤ 2 * B / ℓ + 1 := fun r _ =>
    card_residue_class_in_Icc hℓpos r B
  have hsum : (R.sum fun r => (fiber r).card) ≤ R.card * (2 * B / ℓ + 1) :=
    Finset.sum_le_card_nsmul _ _ _ hfib
  calc
    S.card ≤ (R.biUnion fiber).card := hcard
    _ ≤ R.sum fun r => (fiber r).card := hbunion
    _ ≤ R.card * (2 * B / ℓ + 1) := hsum
    _ ≤ 2 * (2 * B / ℓ + 1) := Nat.mul_le_mul_right _ hR

end FloorObstructionAndDensity
