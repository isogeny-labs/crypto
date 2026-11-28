import Mathlib

/-! Layer 3b/3c: correctness of the two reduction algorithms.

The hardness ledger separates two kinds of claims:

* **algebraic correctness** — the algorithm really returns the discrete log;
* **cost and success premises** — the work, memory and success probability, which are
  modelling assumptions and are recorded in `docs/ecdlp/hardness-ledger.md`.

This file formalises the first kind for baby-step giant-step (BSGS) and
Pohlig--Hellman (PH).  Both algorithms only use the group axioms and the
elementary order theory of a cyclic group, so no curve-specific input is
needed here.

Notation follows Mathlib's additive convention: `n • a` is scalar
multiplication, `addOrderOf a` is the exact (additive) order, and
`a ≡ b [MOD n]` means `a % n = b % n`.
-/

namespace ECDLPAlgorithms

/-! ## Baby-step giant-step -/

section BSGS

variable {G : Type*} [AddCommGroup G]

/-- Baby step `j`: the `j`-th multiple of the generator. -/
def baby (a : G) (j : ℕ) : G := j • a

/-- Giant step `i`: the target shifted back by `i * m` generator multiples. -/
def giant (a : G) (q : G) (m i : ℕ) : G := q - (i * m) • a

/-- A baby/giant match is exactly the statement that `i * m + j` is an
exponent of the target.  This is the algebraic content of the BSGS search. -/
theorem baby_eq_giant_iff (a q : G) (m i j : ℕ) :
    baby a j = giant a q m i ↔ (i * m + j) • a = q := by
  unfold baby giant
  rw [eq_sub_iff_add_eq, add_nsmul, add_comm (j • a) ((i * m) • a)]

/-- BSGS decomposition: with a table of `m` baby steps and a modulus bound
`n ≤ m * m`, every exponent below `n` splits as `i * m + j` with `i, j < m`. -/
theorem bsgs_decomposition {n m x : ℕ} (hm : 0 < m) (hx : x < n) (hn : n ≤ m * m) :
    ∃ i j : ℕ, j < m ∧ i < m ∧ x = i * m + j := by
  refine ⟨x / m, x % m, Nat.mod_lt _ hm, ?_, ?_⟩
  · exact Nat.div_lt_of_lt_mul (Nat.lt_of_lt_of_le hx hn)
  · rw [Nat.mul_comm]
    exact (Nat.div_add_mod x m).symm

/-- **BSGS existence.**  If the target is `x • a` with `x < n ≤ m²`, some pair
of indices in `[0, m)` matches.  So the double loop over the baby table and
the giant steps always terminates with at least one candidate. -/
theorem bsgs_exists_match {a : G} {q : G} {n m x : ℕ}
    (hm : 0 < m) (hx : x < n) (hn : n ≤ m * m) (hq : x • a = q) :
    ∃ i j : ℕ, j < m ∧ i < m ∧ baby a j = giant a q m i := by
  obtain ⟨i, j, hj, hi, rfl⟩ := bsgs_decomposition hm hx hn
  exact ⟨i, j, hj, hi, (baby_eq_giant_iff a q m i j).mpr hq⟩

/-- **BSGS correctness.**  Any baby/giant match yields an exponent congruent to
the true logarithm modulo the order.  In particular the candidate is correct
when reduced modulo `n`; no assumption about which match the search finds
first is needed. -/
theorem bsgs_candidate_correct {a : G} {q : G} {n x i j : ℕ}
    (hord : addOrderOf a = n) (hx : x < n) (hq : x • a = q)
    (hmatch : baby a j = giant a q m i) :
    (i * m + j) % n = x := by
  have hc : (i * m + j) • a = x • a := by
    rw [← baby_eq_giant_iff]
    rw [hmatch, hq]
  have hmod : i * m + j ≡ x [MOD n] := by
    have h := (nsmul_eq_nsmul_iff_modEq (x := a)).mp hc
    simpa [hord] using h
  simpa [Nat.ModEq, Nat.mod_eq_of_lt hx] using hmod

/-- Multiples below the order are distinct: the baby table has no collisions
when its length does not exceed the order. -/
theorem baby_injective {a : G} {n m : ℕ} (hord : addOrderOf a = n) (hm : m ≤ n) :
    ∀ ⦃i j : ℕ⦄, i < m → j < m → i • a = j • a → i = j := by
  intro i j hi hj heq
  have hmod : i ≡ j [MOD n] := by
    have h := (nsmul_eq_nsmul_iff_modEq (x := a)).mp heq
    simpa [hord] using h
  have hi' : i % n = i := Nat.mod_eq_of_lt (lt_of_lt_of_le hi hm)
  have hj' : j % n = j := Nat.mod_eq_of_lt (lt_of_lt_of_le hj hm)
  simpa [Nat.ModEq, hi', hj'] using hmod

/-- **BSGS memory.**  The baby table built from `m` steps has exactly `m`
distinct entries, so its storage cost is `m` group elements (no hidden
collisions to deduplicate). -/
theorem baby_table_card [DecidableEq G] {a : G} {n m : ℕ}
    (hord : addOrderOf a = n) (hm : m ≤ n) :
    ((Finset.range m).image (fun j => j • a)).card = m := by
  have hinj : Set.InjOn (fun j => j • a) ((Finset.range m : Finset ℕ) : Set ℕ) := by
    intro i hi j hj hij
    exact baby_injective hord hm (Finset.mem_range.mp hi) (Finset.mem_range.mp hj) hij
  rw [Finset.card_image_of_injOn hinj]
  simp

end BSGS

/-! ## Pohlig--Hellman -/

section PohligHellman

variable {G : Type*} [AddCommGroup G]

/-- Chinese-remainder step at the level of natural numbers: two congruences
modulo coprime moduli combine into one modulo the product.  This is the only
number-theoretic input to the PH assembly. -/
theorem modEq_mul_of_coprime {a b m n : ℕ} (h : m.Coprime n)
    (hm : a ≡ b [MOD m]) (hn : a ≡ b [MOD n]) : a ≡ b [MOD m * n] := by
  rcases le_total a b with hab | hba
  · rw [Nat.modEq_iff_dvd' hab] at hm hn ⊢
    exact h.mul_dvd_of_dvd_of_dvd hm hn
  · have hm' : m ∣ a - b := (Nat.modEq_iff_dvd' hba).mp hm.symm
    have hn' : n ∣ a - b := (Nat.modEq_iff_dvd' hba).mp hn.symm
    exact ((Nat.modEq_iff_dvd' hba).mpr (h.mul_dvd_of_dvd_of_dvd hm' hn')).symm

/-- **PH subgroup step.**  If `a` has order `n` and `d ∣ n`, then `d • a` has
order `n / d`.  The DLP for `a` therefore reduces to a DLP of size `n / d`
in the subgroup generated by `d • a`. -/
theorem ph_subgroup_order {a : G} {n d : ℕ} (hord : addOrderOf a = n)
    (hd : d ∣ n) (hd0 : d ≠ 0) : addOrderOf (d • a) = n / d := by
  have hd' : d ∣ addOrderOf a := by simpa [hord] using hd
  rw [addOrderOf_nsmul_of_dvd (x := a) hd0 hd', hord]

/-- **PH projection.**  The residue of the logarithm modulo `m₁` is the
discrete log of `m₂ • q` to the base `m₂ • a`.  This is what makes the
subgroup instances of the PH recursion correct. -/
theorem ph_projection {a : G} {n m₁ m₂ x : ℕ} (hord : addOrderOf a = n)
    (hn : n = m₁ * m₂) :
    (x % m₁) • (m₂ • a) = m₂ • (x • a) := by
  rw [← mul_nsmul a m₂ (x % m₁), ← mul_nsmul a x m₂]
  have hmodeq : m₂ * (x % m₁) ≡ x * m₂ [MOD n] := by
    rw [Nat.ModEq, Nat.mul_comm m₂ (x % m₁)]
    have hx : x = m₁ * (x / m₁) + x % m₁ :=
      (Nat.div_add_mod x m₁).symm
    have h1 : m₁ * (x / m₁) * m₂ = n * (x / m₁) := by
      rw [hn]
      ring
    calc (x % m₁) * m₂ % n
        = (n * (x / m₁) + (x % m₁) * m₂) % n := by
          rw [Nat.mul_add_mod_self_left]
      _ = ((m₁ * (x / m₁)) * m₂ + (x % m₁) * m₂) % n := by rw [h1]
      _ = ((m₁ * (x / m₁) + x % m₁) * m₂) % n := by rw [Nat.add_mul]
      _ = (x * m₂) % n := by rw [← hx]
  have hns : (m₂ * (x % m₁)) • a = (x * m₂) • a := by
    have h := (nsmul_eq_nsmul_iff_modEq (x := a)).mpr (by simpa [hord] using hmodeq)
    simpa [hord] using h
  exact hns

/-- **PH reconstruction.**  If the logarithm is known modulo each of two
coprime factors of the order, then it is known modulo the whole order.  This
is the CRT assembly step of Pohlig--Hellman, stated without any cost premise. -/
theorem ph_reconstruct {n m₁ m₂ x y : ℕ}
    (hcop : m₁.Coprime m₂) (hn : n = m₁ * m₂) (hx : x < n)
    (h₁ : y % m₁ = x % m₁) (h₂ : y % m₂ = x % m₂) :
    y % n = x := by
  have hmod : y ≡ x [MOD m₁ * m₂] :=
    modEq_mul_of_coprime hcop h₁ h₂
  rw [← hn, Nat.ModEq, Nat.mod_eq_of_lt hx] at hmod
  exact hmod

/-- The PH recursion specialised to one prime-power factor: solving in the
subgroup of order `p ^ e` recovers `x` modulo `p ^ e`. -/
theorem ph_prime_power_step {a : G} {n p e : ℕ} (hord : addOrderOf a = n)
    (hdvd : p ^ e ∣ n) (hn0 : n ≠ 0) (hne : p ^ e ≠ 0) :
    addOrderOf ((n / p ^ e) • a) = p ^ e := by
  have hd : (n / p ^ e) ∣ n := Nat.div_dvd_of_dvd hdvd
  have hle : p ^ e ≤ n := Nat.le_of_dvd (Nat.pos_of_ne_zero hn0) hdvd
  have hd0 : n / p ^ e ≠ 0 :=
    Nat.pos_iff_ne_zero.mp (Nat.div_pos hle (Nat.pos_of_ne_zero hne))
  have h := ph_subgroup_order (a := a) hord hd hd0
  rw [h]
  exact Nat.div_div_self hdvd hn0

end PohligHellman

end ECDLPAlgorithms
