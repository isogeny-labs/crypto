import Mathlib.NumberTheory.LucasPrimality
import Mathlib.Tactic.NormNum.Prime
import OakleyOrders

/-!
# Pratt primality certificates for the Oakley Group 3 and 4 subgroup orders

`OakleyOrders.lean` names the RFC 2409 subgroup orders `oakley3Prime` and
`oakley4Prime` but does not prove them prime. This file does, by Pratt
(Lucas) certificates checked by the Lean kernel:

* `pratt` reduces `Nat.Prime p` to Mathlib's `lucas_primality` given a
  witness `a`, a list `L` of primes with product `p - 1`, `a^(p-1) ≡ 1` and
  `a^((p-1)/q) ≢ 1 (mod p)` for every `q ∈ L`.
* The modular powers are computed by `powMod`, a square-and-multiply loop with
  fixed fuel 256 whose correctness (`powModAux_cast`) is proved for every
  exponent below `2^256`; the concrete checks are closed by `decide +kernel`
  (kernel evaluation with GMP-backed `Nat` literals; no `native_decide`, no
  `Lean.ofReduceBool`).
* Primes of `p - 1` below `2^20` are proved by Mathlib's `norm_num` primality
  extension; larger ones by their own certificate in this file.

The certificate data is also in `../validation/oakley_pratt_certificates.json`
and is re-checked by the Rust binary `check_pratt_certificates` (`../src/bin/`), which shares
no code with this file.
-/

namespace WeakCurveLeaderboard
namespace Pratt

/-- Square-and-multiply: `powModAux fuel b e m acc ≡ acc * b ^ e (mod m)` when
`e < 2 ^ fuel` (`powModAux_cast`); the result is reduced mod `m`. -/
def powModAux : ℕ → ℕ → ℕ → ℕ → ℕ → ℕ
  | 0, _, _, m, acc => acc % m
  | fuel + 1, b, e, m, acc =>
      powModAux fuel (b * b % m) (e / 2) m (if e % 2 = 1 then acc * b % m else acc)

/-- `b ^ e % m` for `e < 2 ^ 256`. -/
def powMod (b e m : ℕ) : ℕ := powModAux 256 b e m 1

theorem powModAux_mod (m fuel : ℕ) :
    ∀ b e acc : ℕ, powModAux fuel b e m acc % m = powModAux fuel b e m acc := by
  induction fuel with
  | zero => intro b e acc; exact Nat.mod_mod acc m
  | succ fuel ih =>
      intro b e acc
      exact ih (b * b % m) (e / 2) (if e % 2 = 1 then acc * b % m else acc)

theorem powModAux_cast (m fuel : ℕ) :
    ∀ b e acc : ℕ, e < 2 ^ fuel →
      ((powModAux fuel b e m acc : ℕ) : ZMod m) = (acc : ZMod m) * (b : ZMod m) ^ e := by
  induction fuel with
  | zero =>
      intro b e acc he
      have he0 : e = 0 := by simpa using he
      subst he0
      show ((acc % m : ℕ) : ZMod m) = (acc : ZMod m) * (b : ZMod m) ^ 0
      rw [ZMod.natCast_mod, pow_zero, mul_one]
  | succ fuel ih =>
      intro b e acc he
      have hstep : powModAux (fuel + 1) b e m acc
          = powModAux fuel (b * b % m) (e / 2) m
              (if e % 2 = 1 then acc * b % m else acc) := rfl
      have hk : e / 2 < 2 ^ fuel := by
        rw [pow_succ] at he
        exact (Nat.div_lt_iff_lt_mul (by norm_num)).2 he
      rw [hstep, ih _ _ _ hk]
      rcases Nat.mod_two_eq_zero_or_one e with h | h
      · have he' : e = 2 * (e / 2) := by omega
        rw [if_neg (show ¬(e % 2 = 1) by omega)]
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
    ∀ L : List ℕ, (∀ q ∈ L, q.Prime) → ∀ r : ℕ, r.Prime → r ∣ L.prod → r ∈ L := by
  intro L
  induction L with
  | nil =>
      intro _ r hr hd
      rw [List.prod_nil, Nat.dvd_one] at hd
      exact absurd hd hr.ne_one
  | cons x L ih =>
      intro hL r hr hd
      rw [List.prod_cons] at hd
      rcases (Nat.Prime.dvd_mul hr).1 hd with h | h
      · have hx : x.Prime := hL x (List.mem_cons_self)
        rw [(Nat.prime_dvd_prime_iff_eq hr hx).1 h]
        exact List.mem_cons_self
      · exact List.mem_cons_of_mem x
          (ih (fun q hq => hL q (List.mem_cons_of_mem x hq)) r hr h)

theorem primes_nil : ∀ q ∈ ([] : List ℕ), q.Prime :=
  fun _ h => absurd h List.not_mem_nil

theorem primes_cons {a : ℕ} {L : List ℕ} (ha : a.Prime) (hL : ∀ q ∈ L, q.Prime) :
    ∀ q ∈ a :: L, q.Prime :=
  List.forall_mem_cons.2 ⟨ha, hL⟩

/-- **Pratt step.** A witness `a`, a list `L` of primes with product `p - 1`,
`a^(p-1) ≡ 1 (mod p)` and `a^((p-1)/q) ≢ 1 (mod p)` for each `q ∈ L` prove
`p` prime (`lucas_primality`). The bound `p < 2^256` keeps every exponent in
the range where `powMod` is proved correct. -/
theorem pratt (p a : ℕ) (L : List ℕ) (hp2 : 2 ≤ p) (hp : p < 2 ^ 256)
    (hprod : L.prod = p - 1) (hL : ∀ q ∈ L, q.Prime)
    (hfermat : powMod a (p - 1) p = 1)
    (hwit : (L.all fun q => powMod a ((p - 1) / q) p != 1) = true) : p.Prime := by
  have hp1 : 1 < p := by omega
  have hpm : p - 1 < 2 ^ 256 := lt_of_le_of_lt (Nat.sub_le p 1) hp
  have key : ∀ e, e < 2 ^ 256 → ((powMod a e p : ℕ) : ZMod p) = (a : ZMod p) ^ e := by
    intro e he
    rw [powMod, powModAux_cast p 256 a e 1 he, Nat.cast_one, one_mul]
  apply lucas_primality p (a : ZMod p)
  · rw [← key (p - 1) hpm, hfermat, Nat.cast_one]
  · intro q hq hqd h
    have hqL : q ∈ L := mem_of_prime_dvd_prod L hL q hq (by rw [hprod]; exact hqd)
    have hw := List.all_eq_true.1 hwit q hqL
    have hlt : (p - 1) / q < 2 ^ 256 := lt_of_le_of_lt (Nat.div_le_self _ _) hpm
    rw [← key _ hlt, ← (Nat.cast_one (R := ZMod p)), ZMod.natCast_eq_natCast_iff', powMod, powModAux_mod,
      Nat.mod_eq_of_lt hp1] at h
    rw [powMod, h] at hw
    exact absurd hw (by decide)

/-- Pratt certificate: witness `3`, `p - 1 = 2 * 2 * 2 * 2 * 2 * 2 * 7 * 2617`. -/
theorem prime_1172417 : Nat.Prime 1172417 :=
  pratt 1172417 3 [2, 2, 2, 2, 2, 2, 7, 2617]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) primes_nil))))))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `3`, `p - 1 = 2 * 3 * 3 * 5 * 5 * 11923`. -/
theorem prime_5365351 : Nat.Prime 5365351 :=
  pratt 5365351 3 [2, 3, 3, 5, 5, 11923]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) primes_nil))))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `5`, `p - 1 = 2 * 43 * 71 * 1087`. -/
theorem prime_6637223 : Nat.Prime 6637223 :=
  pratt 6637223 5 [2, 43, 71, 1087]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) primes_nil))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `2`, `p - 1 = 2 * 7 * 6637223`. -/
theorem prime_92921123 : Nat.Prime 92921123 :=
  pratt 92921123 2 [2, 7, 6637223]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons prime_6637223 primes_nil)))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `2`, `p - 1 = 2 * 11 * 5365351`. -/
theorem prime_118037723 : Nat.Prime 118037723 :=
  pratt 118037723 2 [2, 11, 5365351]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons prime_5365351 primes_nil)))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `5`, `p - 1 = 2 * 2 * 2 * 3 * 118037723`. -/
theorem prime_2832905353 : Nat.Prime 2832905353 :=
  pratt 2832905353 5 [2, 2, 2, 3, 118037723]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons prime_118037723 primes_nil)))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `2`, `p - 1 = 2 * 2 * 23 * 92921123`. -/
theorem prime_8548743317 : Nat.Prime 8548743317 :=
  pratt 8548743317 2 [2, 2, 23, 92921123]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons prime_92921123 primes_nil))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `2`, `p - 1 = 2 * 2 * 5 * 8548743317`. -/
theorem prime_170974866341 : Nat.Prime 170974866341 :=
  pratt 170974866341 2 [2, 2, 5, 8548743317]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons prime_8548743317 primes_nil))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `2`, `p - 1 = 2 * 2 * 167 * 601 * 719 * 3313`. -/
theorem prime_956315644997 : Nat.Prime 956315644997 :=
  pratt 956315644997 2 [2, 2, 167, 601, 719, 3313]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) primes_nil))))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `3`, `p - 1 = 2 * 2 * 2 * 17 * 61 * 73 * 97 * 202933`. -/
theorem prime_11921091281609 : Nat.Prime 11921091281609 :=
  pratt 11921091281609 3 [2, 2, 2, 17, 61, 73, 97, 202933]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) primes_nil))))))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `2`, `p - 1 = 2 * 35533 * 170974866341`. -/
theorem prime_12150499851389507 : Nat.Prime 12150499851389507 :=
  pratt 12150499851389507 2 [2, 35533, 170974866341]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons prime_170974866341 primes_nil)))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `5`, `p - 1 = 2 * 7 * 53 * 181 * 181 * 2832905353`. -/
theorem prime_68864138704067687 : Nat.Prime 68864138704067687 :=
  pratt 68864138704067687 5 [2, 7, 53, 181, 181, 2832905353]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons prime_2832905353 primes_nil))))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `2`, `p - 1 = 2 * 3 * 3 * 13 * 2297 * 956315644997`. -/
theorem prime_514017746554597507 : Nat.Prime 514017746554597507 :=
  pratt 514017746554597507 2 [2, 3, 3, 13, 2297, 956315644997]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons prime_956315644997 primes_nil))))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `3`, `p - 1 = 2 * 3 * 3 * 68864138704067687`. -/
theorem prime_1239554496673218367 : Nat.Prime 1239554496673218367 :=
  pratt 1239554496673218367 3 [2, 3, 3, 68864138704067687]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons prime_68864138704067687 primes_nil))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `2`, `p - 1 = 2 * 2 * 1172417 * 11921091281609`. -/
theorem prime_55905960308440715813 : Nat.Prime 55905960308440715813 :=
  pratt 55905960308440715813 2 [2, 2, 1172417, 11921091281609]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons prime_1172417 (primes_cons prime_11921091281609 primes_nil))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `6`, `p - 1 = 2 * 2 * 3 * 5 * 68947 * 12150499851389507`. -/
theorem prime_50264430795225140347741 : Nat.Prime 50264430795225140347741 :=
  pratt 50264430795225140347741 6 [2, 2, 3, 5, 68947, 12150499851389507]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons prime_12150499851389507 primes_nil))))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `2`, `p - 1 = 2 * 3 * 10181 * 1239554496673218367 * 50264430795225140347741`. -/
theorem prime_3805993847215893016155463826195386266397436443 : Nat.Prime 3805993847215893016155463826195386266397436443 :=
  pratt 3805993847215893016155463826195386266397436443 2 [2, 3, 10181, 1239554496673218367, 50264430795225140347741]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons prime_1239554496673218367 (primes_cons prime_50264430795225140347741 primes_nil)))))
    (by decide +kernel) (by decide +kernel)

/-- Pratt certificate: witness `7`, `p - 1 = 2 * 3 * 11 * 109 * 8123 * 12277 * 594667 * 514017746554597507 * 55905960308440715813`. -/
theorem prime_12259964326927110866866776214413170562013096250261263279 : Nat.Prime 12259964326927110866866776214413170562013096250261263279 :=
  pratt 12259964326927110866866776214413170562013096250261263279 7 [2, 3, 11, 109, 8123, 12277, 594667, 514017746554597507, 55905960308440715813]
    (by norm_num) (by norm_num) (by decide +kernel)
    (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons (by norm_num) (primes_cons prime_514017746554597507 (primes_cons prime_55905960308440715813 primes_nil)))))))))
    (by decide +kernel) (by decide +kernel)

end Pratt

/-- The Oakley Group 3 subgroup order (RFC 2409, `12 · r` curve order) is prime. -/
theorem oakley3Prime_prime : Nat.Prime oakley3Prime := Pratt.prime_3805993847215893016155463826195386266397436443

/-- The Oakley Group 4 subgroup order (RFC 2409, `4 · r` curve order) is prime. -/
theorem oakley4Prime_prime : Nat.Prime oakley4Prime :=
  Pratt.prime_12259964326927110866866776214413170562013096250261263279

end WeakCurveLeaderboard
