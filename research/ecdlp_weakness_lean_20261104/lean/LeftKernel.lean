import Mathlib

/-!
# Left-kernel index calculus needs no rank condition

Corpus target: `HEUR-RELATION-RANK` (ECDLP claim corpus, WEAKNESS_TRIAGE W2).

Setting. A group of prime order `q` with generator `P` and target `Q = x P`.
Each relation `i` is a random point `R_i = a_i P + b_i Q` written over the factor
base `F_1, ..., F_B` as `R_i = sum_j M i j * F_j`. Taking logarithms base `P`,
with `f j = log F_j`, gives `a i + b i * x = sum_j M i j * f j`.

1. `left_kernel_identity`: any `lam` with `lam ⬝ M = 0` gives
   `sum lam a + x * sum lam b = 0`.
2. `left_kernel_recovers`: if moreover `sum lam b ≠ 0`, then
   `x = -(sum lam a) / (sum lam b)`.
3. `exists_left_kernel`: with more relations than factor-base columns a
   nonzero `lam` exists for every matrix `M`, whatever its rank.
4. `card_bad_pairs`: the failure event `sum lam b = 0` has density exactly `1/q`
   whenever `lam` is chosen from the points `R_i` alone. Formally, for any
   nonzero-valued `Λ` applied to the logs `a + b * x`, exactly `q^(2n-1)` of the
   `q^(2n)` pairs `(a, b)` fail.

Item 4 is the probabilistic content of the rigorous left-kernel method: `lam`
can depend on everything the solver sees (the points, hence their logs), but
not on the hidden split of each log into `a i` and `b i`.
-/

open scoped BigOperators

namespace LeftKernel

section Algebra

variable {K : Type*} [Field K] {n B : ℕ}

/-- Relation identity: a left-kernel vector of the relation matrix makes the
weighted sum of the logs of the relation points vanish. -/
theorem left_kernel_identity
    (M : Fin n → Fin B → K) (f : Fin B → K) (x : K) (a b lam : Fin n → K)
    (hrel : ∀ i, a i + b i * x = ∑ j, M i j * f j)
    (hker : ∀ j, ∑ i, lam i * M i j = 0) :
    (∑ i, lam i * a i) + x * ∑ i, lam i * b i = 0 := by
  have h1 : (∑ i, lam i * a i) + x * ∑ i, lam i * b i
      = ∑ i, lam i * (a i + b i * x) := by
    rw [Finset.mul_sum, ← Finset.sum_add_distrib]
    apply Finset.sum_congr rfl
    intro i _
    ring
  have h2 : ∑ i, lam i * (a i + b i * x) = ∑ j, (∑ i, lam i * M i j) * f j := by
    simp_rw [hrel, Finset.mul_sum, Finset.sum_mul]
    rw [Finset.sum_comm]
    apply Finset.sum_congr rfl
    intro j _
    apply Finset.sum_congr rfl
    intro i _
    ring
  rw [h1, h2]
  simp [hker]

/-- Recovery: when the `b`-weight of the kernel vector is nonzero, the
discrete logarithm is determined. -/
theorem left_kernel_recovers
    (M : Fin n → Fin B → K) (f : Fin B → K) (x : K) (a b lam : Fin n → K)
    (hrel : ∀ i, a i + b i * x = ∑ j, M i j * f j)
    (hker : ∀ j, ∑ i, lam i * M i j = 0)
    (hb : (∑ i, lam i * b i) ≠ 0) :
    x = -(∑ i, lam i * a i) / (∑ i, lam i * b i) := by
  have h := left_kernel_identity M f x a b lam hrel hker
  rw [eq_div_iff hb]
  linear_combination h

/-- Existence with no rank hypothesis: more relations than columns always give
a nonzero left-kernel vector. -/
theorem exists_left_kernel (M : Fin n → Fin B → K) (h : B < n) :
    ∃ lam : Fin n → K, lam ≠ 0 ∧ ∀ j, ∑ i, lam i * M i j = 0 := by
  let L : (Fin n → K) →ₗ[K] (Fin B → K) :=
    { toFun := fun lam j => ∑ i, lam i * M i j
      map_add' := by
        intro u v
        funext j
        simp [add_mul, Finset.sum_add_distrib]
      map_smul' := by
        intro c u
        funext j
        simp [Finset.mul_sum, mul_assoc] }
  have hlt : Module.finrank K (Fin B → K) < Module.finrank K (Fin n → K) := by
    simpa using h
  obtain ⟨lam, hmem, hne⟩ :=
    (Submodule.ne_bot_iff _).mp (LinearMap.ker_ne_bot_of_finrank_lt (f := L) hlt)
  refine ⟨lam, hne, fun j => ?_⟩
  have := congrFun (LinearMap.mem_ker.mp hmem) j
  simpa [L] using this

end Algebra

section Counting

variable {K : Type*} [Field K] [Fintype K] [DecidableEq K] {n : ℕ}

/-- The dot-product functional with a fixed coefficient vector. -/
def dot (m : Fin n → K) : (Fin n → K) →ₗ[K] K :=
  { toFun := fun b => ∑ i, m i * b i
    map_add' := by
      intro u v
      simp [mul_add, Finset.sum_add_distrib]
    map_smul' := by
      intro c u
      simp [Finset.mul_sum, mul_left_comm] }

omit [Fintype K] [DecidableEq K] in
theorem dot_surjective {m : Fin n → K} (hm : m ≠ 0) :
    Function.Surjective (dot m) := by
  obtain ⟨k, hk⟩ := Function.ne_iff.mp hm
  have hk' : m k ≠ 0 := by simpa using hk
  intro y
  refine ⟨Pi.single k (y / m k), ?_⟩
  simp [dot, Pi.single_apply]
  field_simp

omit [DecidableEq K] in
/-- A nonzero dot-product kernel has exactly `q^(n-1)` points (stated without
subtraction: its size times `q` is `q^n`). -/
theorem card_dot_ker_mul {m : Fin n → K} (hm : m ≠ 0) :
    Nat.card (LinearMap.ker (dot m)) * Fintype.card K = Fintype.card K ^ n := by
  have hrank := LinearMap.finrank_range_add_finrank_ker (dot m)
  have hrange : Module.finrank K (LinearMap.range (dot m)) = 1 := by
    rw [LinearMap.range_eq_top.mpr (dot_surjective hm)]
    simp
  rw [hrange, Module.finrank_fin_fun] at hrank
  rw [Module.natCard_eq_pow_finrank (K := K) (V := LinearMap.ker (dot m)),
    Nat.card_eq_fintype_card, ← pow_succ,
    show Module.finrank K (LinearMap.ker (dot m)) + 1 = n by omega]

/-- The bad pairs: `Λ` sees only the logs `ℓ = a + b * x` (the relation points),
and the method fails when the chosen vector has zero `b`-weight. -/
def badPairs (Λ : (Fin n → K) → (Fin n → K)) (x : K) :
    Finset ((Fin n → K) × (Fin n → K)) :=
  Finset.univ.filter fun p => dot (Λ (fun i => p.1 i + p.2 i * x)) p.2 = 0

/-- Exact density `1/q` of failure: `|bad| * q = q^(2n)`. -/
theorem card_bad_pairs (Λ : (Fin n → K) → (Fin n → K)) (hΛ : ∀ ℓ, Λ ℓ ≠ 0) (x : K) :
    (badPairs Λ x).card * Fintype.card K = Fintype.card K ^ (2 * n) := by
  classical
  -- Reparametrise (a, b) ↦ (ℓ, b) with ℓ = a + b * x.
  let e : ((Fin n → K) × (Fin n → K)) ≃ ((Fin n → K) × (Fin n → K)) :=
    { toFun := fun p => (fun i => p.1 i + p.2 i * x, p.2)
      invFun := fun p => (fun i => p.1 i - p.2 i * x, p.2)
      left_inv := by intro p; ext i <;> simp
      right_inv := by intro p; ext i <;> simp }
  have hcard : (badPairs Λ x).card =
      (Finset.univ.filter fun p : (Fin n → K) × (Fin n → K) => dot (Λ p.1) p.2 = 0).card := by
    apply Finset.card_bij' (fun p _ => e p) (fun p _ => e.symm p)
    · intro p hp
      simpa [badPairs, e] using hp
    · intro p hp
      simpa [badPairs, e] using hp
    · intro p _; simp
    · intro p _; simp
  rw [hcard]
  -- Split the filter over ℓ.
  have hsplit :
      (Finset.univ.filter fun p : (Fin n → K) × (Fin n → K) => dot (Λ p.1) p.2 = 0).card
        = ∑ ℓ : Fin n → K, (Finset.univ.filter fun b : Fin n → K => dot (Λ ℓ) b = 0).card := by
    rw [← Finset.card_sigma]
    apply Finset.card_bij' (fun p _ => ⟨p.1, p.2⟩) (fun s _ => (s.1, s.2))
    all_goals (intro _ h; simp_all)
  rw [hsplit, Finset.sum_mul]
  have hfiber : ∀ ℓ : Fin n → K,
      (Finset.univ.filter fun b : Fin n → K => dot (Λ ℓ) b = 0).card * Fintype.card K
        = Fintype.card K ^ n := by
    intro ℓ
    rw [← card_dot_ker_mul (hΛ ℓ)]
    congr 1
    rw [← Nat.card_eq_finsetCard]
    exact Nat.card_congr (Equiv.subtypeEquivRight (fun b => by simp [LinearMap.mem_ker]))
  simp_rw [hfiber]
  simp [Finset.card_univ, two_mul, pow_add]

end Counting

end LeftKernel
