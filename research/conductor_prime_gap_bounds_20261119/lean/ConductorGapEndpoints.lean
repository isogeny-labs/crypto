import Mathlib.NumberTheory.LucasPrimality
import Mathlib.Tactic.NormNum.Prime
import Mathlib.Tactic
import ConductorOrthogonality
import HasseTransferCeiling
import DeepGap

/-!
# Kernel-certified endpoint primes and volcano-gap bounds

Lucas (Pratt) certificates for the study's gap primes, checked by a fuelled
`powMod` loop and `decide` (no `native_decide`, no WellFounded.fix blowups).
Endpoint theorems package the resulting primes into conductor-gap bounds for
the ECC2K-130-scale, deep-volcano, and P-256 crater instances.

**Not formalized here:** `End(E) ≃ O_f`; Robert Remark 4.5.
-/

set_option autoImplicit false

namespace ConductorGapEndpoints

/-! ## Fuelled modular exponentiation -/

/-- Square-and-multiply with fixed fuel: correct for `e < 2^fuel`. -/
def powModAux : ℕ → ℕ → ℕ → ℕ → ℕ → ℕ
  | 0, _, _, m, acc => acc % m
  | fuel + 1, b, e, m, acc =>
      powModAux fuel (b * b % m) (e / 2) m (if e % 2 = 1 then acc * b % m else acc)

def powMod (b e m : ℕ) : ℕ := powModAux 64 b e m 1

theorem powModAux_mod (m fuel : ℕ) :
    ∀ b e acc : ℕ, powModAux fuel b e m acc % m = powModAux fuel b e m acc := by
  induction fuel with
  | zero => intro b e acc; exact Nat.mod_mod acc m
  | succ fuel ih =>
      intro b e acc
      exact ih (b * b % m) (e / 2) (if e % 2 = 1 then acc * b % m else acc)

theorem powModAux_cast (m fuel : ℕ) :
    ∀ b e acc : ℕ, e < 2 ^ fuel →
      ((powModAux fuel b e m acc : ℕ) : ZMod m) =
        (acc : ZMod m) * (b : ZMod m) ^ e := by
  induction fuel with
  | zero =>
      intro b e acc he
      have he0 : e = 0 := by simpa using he
      subst he0
      show ((acc % m : ℕ) : ZMod m) = (acc : ZMod m) * (b : ZMod m) ^ 0
      rw [ZMod.natCast_mod, pow_zero, mul_one]
  | succ fuel ih =>
      intro b e acc he
      have hk : e / 2 < 2 ^ fuel := by
        rw [pow_succ] at he
        exact (Nat.div_lt_iff_lt_mul (by decide)).2 he
      rw [show powModAux (fuel + 1) b e m acc =
            powModAux fuel (b * b % m) (e / 2) m
              (if e % 2 = 1 then acc * b % m else acc) from rfl,
        ih _ _ _ hk]
      rcases Nat.mod_two_eq_zero_or_one e with h | h
      · have he' : e = 2 * (e / 2) := by omega
        rw [if_neg (by omega)]
        simp only [ZMod.natCast_mod]
        conv_rhs => rw [he']
        push_cast
        ring
      · have he' : e = 2 * (e / 2) + 1 := by omega
        rw [if_pos h]
        simp only [ZMod.natCast_mod]
        conv_rhs => rw [he']
        push_cast
        ring

theorem mem_of_prime_dvd_prod :
    ∀ L : List ℕ, (∀ q ∈ L, q.Prime) → ∀ r : ℕ, r.Prime → r ∣ L.prod → r ∈ L
  | [], _, r, hr, hd => by
      rw [List.prod_nil, Nat.dvd_one] at hd
      exact absurd hd hr.ne_one
  | x :: L, hL, r, hr, hd => by
      rw [List.prod_cons] at hd
      rcases (Nat.Prime.dvd_mul hr).1 hd with h | h
      · have hx : x.Prime := hL x (List.mem_cons_self)
        rw [(Nat.prime_dvd_prime_iff_eq hr hx).1 h]
        exact List.mem_cons_self
      · exact List.mem_cons_of_mem _
          (mem_of_prime_dvd_prod L (fun q hq => hL q (List.mem_cons_of_mem _ hq)) r hr h)

theorem primes_nil : ∀ q ∈ ([] : List ℕ), q.Prime := fun _ h => absurd h List.not_mem_nil

theorem primes_cons {a : ℕ} {L : List ℕ} (ha : a.Prime) (hL : ∀ q ∈ L, q.Prime) :
    ∀ q ∈ a :: L, q.Prime := List.forall_mem_cons.2 ⟨ha, hL⟩

/-- One Lucas check: `powMod a ((p-1)/q) p ≠ 1`. -/
def checkLucasStep (a p q : ℕ) : Bool :=
  decide (powMod a ((p - 1) / q) p ≠ 1)

/-- Pratt/Lucas step with fuel 64 (sufficient for all primes in this file). -/
theorem pratt (p a : ℕ) (L : List ℕ) (hp2 : 2 ≤ p) (hp : p < 2 ^ 64)
    (hprod : L.prod = p - 1) (hL : ∀ q ∈ L, q.Prime)
    (hfermat : powMod a (p - 1) p = 1)
    (hwit : (L.all fun q => powMod a ((p - 1) / q) p != 1) = true) :
    p.Prime := by
  have hp1 : 1 < p := by omega
  have hpm : p - 1 < 2 ^ 64 := lt_of_le_of_lt (Nat.sub_le p 1) hp
  have key : ∀ e, e < 2 ^ 64 → ((powMod a e p : ℕ) : ZMod p) = (a : ZMod p) ^ e := by
    intro e he
    rw [powMod, powModAux_cast p 64 a e 1 he, Nat.cast_one, one_mul]
  apply lucas_primality p (a : ZMod p)
  · rw [← key (p - 1) hpm, hfermat, Nat.cast_one]
  · intro q hq hqd h
    have hqL : q ∈ L := mem_of_prime_dvd_prod L hL q hq (by rw [hprod]; exact hqd)
    have hw := List.all_eq_true.1 hwit q hqL
    have hlt : (p - 1) / q < 2 ^ 64 := lt_of_le_of_lt (Nat.div_le_self _ _) hpm
    rw [← key _ hlt, ← (Nat.cast_one (R := ZMod p)), ZMod.natCast_eq_natCast_iff', powMod,
      powModAux_mod, Nat.mod_eq_of_lt hp1] at h
    rw [powMod, h] at hw
    exact absurd hw (by decide)

/-! ## Small primes used in certificates -/

theorem prime_2 : Nat.Prime 2 := Nat.prime_two
theorem prime_3 : Nat.Prime 3 := by norm_num
theorem prime_5 : Nat.Prime 5 := by norm_num
theorem prime_17 : Nat.Prime 17 := by norm_num
theorem prime_19 : Nat.Prime 19 := by norm_num
theorem prime_31 : Nat.Prime 31 := by norm_num
theorem prime_67 : Nat.Prime 67 := by norm_num
theorem prime_131 : Nat.Prime 131 := by norm_num
theorem prime_151 : Nat.Prime 151 := by norm_num
theorem prime_859 : Nat.Prime 859 := by norm_num

theorem prime_4001 : Nat.Prime 4001 :=
  pratt 4001 3 [2, 2, 2, 2, 2, 5, 5, 5]
    (by norm_num) (by norm_num) (by decide)
    (primes_cons prime_2 (primes_cons prime_2 (primes_cons prime_2 (primes_cons prime_2
      (primes_cons prime_2 (primes_cons prime_5 (primes_cons prime_5 (primes_cons prime_5
        primes_nil))))))))
    (by decide) (by decide)

theorem prime_289510489 : Nat.Prime 289510489 :=
  pratt 289510489 13 [2, 2, 2, 3, 3, 31, 151, 859]
    (by norm_num) (by norm_num) (by decide)
    (primes_cons prime_2 (primes_cons prime_2 (primes_cons prime_2 (primes_cons prime_3
      (primes_cons prime_3 (primes_cons prime_31 (primes_cons prime_151 (primes_cons prime_859
        primes_nil))))))))
    (by decide) (by decide)

/-- Big ECC2K-130-scale gap prime. -/
theorem prime_ecc2k130_big : Nat.Prime 146505763881528721 :=
  pratt 146505763881528721 13 [2, 2, 2, 2, 3, 5, 17, 31, 4001, 289510489]
    (by norm_num) (by norm_num) (by decide)
    (primes_cons prime_2 (primes_cons prime_2 (primes_cons prime_2 (primes_cons prime_2
      (primes_cons prime_3 (primes_cons prime_5 (primes_cons prime_17 (primes_cons prime_31
        (primes_cons prime_4001 (primes_cons prime_289510489 primes_nil))))))))))
    (by decide) (by decide)

theorem prime_263 : Nat.Prime 263 :=
  pratt 263 5 [2, 131]
    (by norm_num) (by norm_num) (by decide)
    (primes_cons prime_2 (primes_cons prime_131 primes_nil))
    (by decide) (by decide)

theorem prime_631409 : Nat.Prime 631409 :=
  pratt 631409 3 [2, 2, 2, 2, 19, 31, 67]
    (by norm_num) (by norm_num) (by decide)
    (primes_cons prime_2 (primes_cons prime_2 (primes_cons prime_2 (primes_cons prime_2
      (primes_cons prime_19 (primes_cons prime_31 (primes_cons prime_67 primes_nil)))))))
    (by decide) (by decide)

/-! ## Generic volcano-gap certificate -/

/-- A kernel certificate that a prime `ℓ` divides a conductor gap between
`f₁` and `f₂` with a single degree-`ℓ` step. -/
structure VolcanoGapCert where
  f₁ : ℕ
  f₂ : ℕ
  ℓ : ℕ
  hf₁ : f₁ ≠ 0
  hf₂ : f₂ ≠ 0
  hℓ : Nat.Prime ℓ
  hstep : ConductorOrthogonality.ValidConductorStep f₁ f₂ ℓ
  hgap : padicValNat ℓ f₁ ≠ padicValNat ℓ f₂

/-- From a gap certificate, the large-step theorem applies to the one-step walk. -/
theorem checkVolcanoGapCert_sound (c : VolcanoGapCert) :
    ∃ d ∈ ([c.ℓ] : List ℕ), c.ℓ ∣ d ∧ c.ℓ ≤ d := by
  let w : ConductorOrthogonality.ConductorWalk c.f₁ [c.ℓ] :=
    ConductorOrthogonality.ConductorWalk.cons c.hstep c.hℓ.ne_zero c.hf₁ c.hf₂
      (ConductorOrthogonality.ConductorWalk.singleton c.f₂ c.hf₂)
  have hend : w.endpoint = c.f₂ := rfl
  have hgap' : padicValNat c.ℓ c.f₁ ≠ padicValNat c.ℓ w.endpoint := by
    rw [hend]; exact c.hgap
  have : Fact c.ℓ.Prime := ⟨c.hℓ⟩
  exact ConductorOrthogonality.conductor_gap_forces_large_step w hgap'

/-- Alias kept for study references. -/
theorem checkVolcanoGapCert_large_step (c : VolcanoGapCert) :
    ∃ d ∈ ([c.ℓ] : List ℕ), c.ℓ ∣ d ∧ c.ℓ ≤ d :=
  checkVolcanoGapCert_sound c

/-! ## Study endpoints -/

/-- ECC2K-130-scale big gap: conductor jump by the prime `146505763881528721`,
forcing a large step of degree `ℓ`. -/
theorem ecc2k130_big_gap_bounds :
    let ℓ := 146505763881528721
    Nat.Prime ℓ ∧
      ConductorOrthogonality.ValidConductorStep 1 ℓ ℓ ∧
        padicValNat ℓ 1 ≠ padicValNat ℓ ℓ ∧
          ∃ d ∈ ([ℓ] : List ℕ), ℓ ∣ d ∧ ℓ ≤ d := by
  refine ⟨prime_ecc2k130_big, ⟨by simp, by simp⟩, ?_, ?_⟩
  · have : Fact (Nat.Prime 146505763881528721) := ⟨prime_ecc2k130_big⟩
    have h0 : padicValNat 146505763881528721 1 = 0 :=
      padicValNat.eq_zero_of_not_dvd (by decide)
    have h1 : padicValNat 146505763881528721 146505763881528721 = 1 :=
      padicValNat_self
    omega
  · let ℓ := 146505763881528721
    have hprime := prime_ecc2k130_big
    have : Fact (Nat.Prime ℓ) := ⟨hprime⟩
    have h0 : padicValNat ℓ 1 = 0 := padicValNat.eq_zero_of_not_dvd (by decide)
    have h1 : padicValNat ℓ ℓ = 1 := padicValNat_self
    let c : VolcanoGapCert :=
      { f₁ := 1, f₂ := ℓ, ℓ := ℓ
        hf₁ := by decide, hf₂ := hprime.ne_zero, hℓ := hprime
        hstep := ⟨by simp, by simp⟩
        hgap := by omega }
    exact checkVolcanoGapCert_sound c

/-- The big gap prime is not a base-field rational torsion order for a
curve of conductor 1 in the sense that `v_ℓ(1) = 0 ≠ 1 = v_ℓ(ℓ)`. -/
theorem ecc2k130_big_not_basefield_rational :
    padicValNat 146505763881528721 1 = 0 ∧
      padicValNat 146505763881528721 146505763881528721 = 1 := by
  have : Fact (Nat.Prime 146505763881528721) := ⟨prime_ecc2k130_big⟩
  exact ⟨padicValNat.eq_zero_of_not_dvd (by decide), padicValNat_self⟩

/-- ECC2K-130-scale small gap prime `263`. -/
theorem ecc2k130_small_gap_bounds :
    Nat.Prime 263 ∧
      ConductorOrthogonality.ValidConductorStep 1 263 263 ∧
        padicValNat 263 1 ≠ padicValNat 263 263 := by
  refine ⟨prime_263, ⟨by simp, by simp⟩, ?_⟩
  have : Fact (Nat.Prime 263) := ⟨prime_263⟩
  have h0 : padicValNat 263 1 = 0 := padicValNat.eq_zero_of_not_dvd (by decide)
  have h1 : padicValNat 263 263 = 1 := padicValNat_self
  omega

/-- Deep-volcano gap prime `631409` from `DeepGap`. -/
theorem deep_volcano_gap_bounds :
    Nat.Prime 631409 ∧
      (DeepGap.N : ℤ) = 631409 ∧
        ConductorOrthogonality.ValidConductorStep 1 631409 631409 :=
  ⟨prime_631409, rfl, ⟨by simp, by simp⟩⟩

/-- A conductor-1 crater walk ending at conductor 1 has no valuation gap:
`v_ℓ` is unchanged, so there is no vertical prime gap to exploit. -/
theorem p256_crater_no_vertical_gaps {ℓ : ℕ} [Fact ℓ.Prime] {ds : List ℕ}
    (w : ConductorOrthogonality.ConductorWalk 1 ds) (hend : w.endpoint = 1) :
    padicValNat ℓ 1 = padicValNat ℓ w.endpoint := by
  rw [hend]

end ConductorGapEndpoints
