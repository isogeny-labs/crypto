/-!
# Lemmas 1 and 2 of `research/ic_leads_20261018/LEMMAS.md`, in core Lean 4

No Mathlib (the container had no disk for it; see `README.md`). The algebra is
stated over `FieldOps`, a structure carrying exactly the field axioms the
proofs use, so each theorem holds for every field. `F₂` is `Bool` with `xor`.

* `functional_const`   — the heart of Lemma 1: a function `λ : K → B` that is
  constant on every product `x₁ · x₂ · u` (`x₁, x₂ ∈ V`, `u ∈ K`) is constant
  on all of `K`, as soon as `V` holds one nonzero element. Linearity of `λ`
  is not needed.
* `lemma1_no_dependency` — Lemma 1 as stated: the `n` coordinate forms of the
  trilinear map `(x₁, x₂, u) ↦ x₁x₂u` admit no nontrivial `F₂`-linear
  dependency, given coordinates with a dual basis.
* `sq_add_char2`        — the step of Lemma 1's proof that makes the squared
  parts of `S₃` `F₂`-linear: `(a + b)² = a² + b²` in characteristic 2.
* `lemma2_relation_phase` / `lemma2_budget` — Lemma 2: if the relation phase
  makes at least `R · A` oracle calls of cost at least `c`, it costs more than
  `T` unless `R · A · c ≤ T`, i.e. `c ≤ ⌊T / (R · A)⌋`.
-/

namespace ICLemmas

/-- The field axioms the proofs use (every field satisfies them). -/
structure FieldOps (K : Type) where
  zero : K
  one : K
  add : K → K → K
  mul : K → K → K
  inv : K → K
  mul_assoc : ∀ a b c : K, mul (mul a b) c = mul a (mul b c)
  one_mul : ∀ a : K, mul one a = a
  mul_inv : ∀ a : K, a ≠ zero → mul a (inv a) = one
  mul_ne_zero : ∀ a b : K, a ≠ zero → b ≠ zero → mul a b ≠ zero

/-- **Lemma 1, core.** If `λ` takes the value `b₀` on every `x₁ x₂ u` with
`x₁, x₂ ∈ V` and `u ∈ K`, and `V` contains a nonzero `v`, then `λ` is the
constant `b₀`. (With `b₀ = 0` and `λ` linear: the only functional killing the
trilinear map is zero.) -/
theorem functional_const {K B : Type} (F : FieldOps K) (V : K → Prop)
    (v : K) (hv : V v) (hv0 : v ≠ F.zero) (lam : K → B) (b₀ : B)
    (h : ∀ x₁ x₂ u, V x₁ → V x₂ → lam (F.mul (F.mul x₁ x₂) u) = b₀) :
    ∀ w, lam w = b₀ := by
  intro w
  have hvv : F.mul v v ≠ F.zero := F.mul_ne_zero v v hv0 hv0
  have key : F.mul (F.mul v v) (F.mul (F.inv (F.mul v v)) w) = w := by
    rw [← F.mul_assoc, F.mul_inv _ hvv, F.one_mul]
  have := h v v (F.mul (F.inv (F.mul v v)) w) hv hv
  rwa [key] at this

/-- `F₂`-sum of `n` bits. -/
def xsum : (n : Nat) → (Fin n → Bool) → Bool
  | 0, _ => false
  | n + 1, f => xor (f 0) (xsum n (fun i => f i.succ))

theorem xsum_false : ∀ (n : Nat), xsum n (fun _ => false) = false
  | 0 => rfl
  | n + 1 => by simp [xsum, xsum_false n]

/-- Picking out one bit with a Kronecker delta. -/
theorem xsum_delta : ∀ (n : Nat) (c : Fin n → Bool) (j : Fin n),
    xsum n (fun i => c i && decide (i = j)) = c j
  | 0, _, j => j.elim0
  | n + 1, c, j => by
    simp only [xsum]
    cases j using Fin.cases with
    | zero =>
      have hrest : (fun i : Fin n => c i.succ && decide (i.succ = (0 : Fin (n+1))))
          = fun _ => false := by
        funext i; simp [Fin.succ_ne_zero]
      rw [hrest, xsum_false]; simp
    | succ j =>
      have h0 : decide ((0 : Fin (n+1)) = j.succ) = false := by
        simp [(Fin.succ_ne_zero j).symm]
      have hrest : (fun i : Fin n => c i.succ && decide (i.succ = j.succ))
          = fun i => c i.succ && decide (i = j) := by
        funext i; simp [Fin.succ_inj]
      rw [h0, hrest, xsum_delta n (fun i => c i.succ) j]; simp

/-- **Lemma 1, as stated.** `K` has `n` coordinate functions `coord i : K → F₂`
with a dual basis `e` (`coord i (e j) = [i = j]`). A linear dependency among the
`n` coordinate forms of `(x₁, x₂, u) ↦ x₁x₂u` on `V × V × K` is a vector
`c ∈ F₂ⁿ` with `Σ cᵢ · coordᵢ(x₁x₂u) = 0` for all arguments. If `V ≠ {0}`, the
only such `c` is zero: the forms are linearly independent. -/
theorem lemma1_no_dependency {K : Type} {n : Nat} (F : FieldOps K)
    (V : K → Prop) (v : K) (hv : V v) (hv0 : v ≠ F.zero)
    (coord : Fin n → K → Bool) (e : Fin n → K)
    (hdual : ∀ i j, coord i (e j) = decide (i = j))
    (c : Fin n → Bool)
    (hdep : ∀ x₁ x₂ u, V x₁ → V x₂ →
      xsum n (fun i => c i && coord i (F.mul (F.mul x₁ x₂) u)) = false) :
    ∀ j, c j = false := by
  intro j
  have hconst := functional_const F V v hv hv0
    (fun w => xsum n (fun i => c i && coord i w)) false hdep (e j)
  simp only [hdual] at hconst
  rwa [xsum_delta] at hconst

/-- A commutative ring of characteristic 2, as far as `sq_add_char2` needs. -/
structure Char2Ring (K : Type) where
  add : K → K → K
  mul : K → K → K
  add_assoc : ∀ a b c : K, add (add a b) c = add a (add b c)
  add_comm : ∀ a b : K, add a b = add b a
  mul_comm : ∀ a b : K, mul a b = mul b a
  left_distrib : ∀ a b c : K, mul a (add b c) = add (mul a b) (mul a c)
  right_distrib : ∀ a b c : K, mul (add a b) c = add (mul a c) (mul b c)
  /-- `x + x = 0`, written as: adding `x + x` changes nothing. -/
  char2 : ∀ x y : K, add (add x x) y = y

/-- **Lemma 1, the squaring step.** In characteristic 2, `(a + b)² = a² + b²`,
so the squared parts `(a + c)²d²` and `(ac)²` of `S₃` are `F₂`-linear images of
products of fewer blocks. -/
theorem sq_add_char2 {K : Type} (R : Char2Ring K) (a b : K) :
    R.mul (R.add a b) (R.add a b) = R.add (R.mul a a) (R.mul b b) := by
  rw [R.right_distrib, R.left_distrib, R.left_distrib, R.mul_comm b a]
  -- a a + (a b + (a b + b b)) = a a + b b
  rw [R.add_assoc, ← R.add_assoc (R.mul a b) (R.mul a b), R.char2]

/-- **Lemma 2.** If the relation phase makes at least `R · A` oracle calls and
each call costs at least `c` operations, then its cost exceeds the matched
rho cost `T` whenever `R · A · c > T`. -/
theorem lemma2_relation_phase (R A c calls total T : Nat)
    (hcalls : R * A ≤ calls) (hcost : calls * c ≤ total) (hgt : T < R * A * c) :
    T < total :=
  Nat.lt_of_lt_of_le hgt (Nat.le_trans (Nat.mul_le_mul_right c hcalls) hcost)

/-- **Lemma 2, budget form.** If the relation phase stays within `T`, then the
per-call cost obeys `c ≤ ⌊T / (R · A)⌋ = B`. -/
theorem lemma2_budget (R A c calls total T : Nat) (hRA : 0 < R * A)
    (hcalls : R * A ≤ calls) (hcost : calls * c ≤ total) (hwithin : total ≤ T) :
    c ≤ T / (R * A) := by
  have h : R * A * c ≤ T :=
    Nat.le_trans (Nat.le_trans (Nat.mul_le_mul_right c hcalls) hcost) hwithin
  exact (Nat.le_div_iff_mul_le hRA).2 (by rw [Nat.mul_comm]; exact h)

end ICLemmas
