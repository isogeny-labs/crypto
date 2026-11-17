import SubspaceCriterion

/-!
# F4 stays inside any degree-`D` closed subspace

`SubspaceCriterion.no_gb_in_rows_of_low_lt` says no Gröbner basis lies in a
subspace whose degree-≤1 part is too small. The `pq2_rank` census applies it to
the *iterated degree-`D` closure* and reads the result as a lower bound on F4's
maximal step degree. That reading needs one more fact: everything an F4 run
with step degree at most `D` produces lies in the closure. This file proves it
for an abstract model of F4 that covers what the algorithm does.

* `DClosed D S`: every `f ∈ S` with `totalDegree f + 1 ≤ D` has `X i * f ∈ S`.
* `StepRows D A`: the rows `monomial s 1 * g` with `g ∈ A` and total degree at
  most `D`. F4's symbolic preprocessing only ever builds such rows (Faugère
  1999, §2), and the "simplify" heuristic only substitutes rows of this form
  for polynomials computed earlier, so it uses the same set with `A` the union
  of all earlier bases.
* `F4Run D F G`: `G 0` lies in the span of the generators `F`, and each `G (t+1)`
  lies in the span of the earlier bases and their step rows. Linear algebra,
  deletion by criteria and the final interreduction all keep this property.

Main results:

* `run_subset`: if `F ⊆ S` and `S` is `D`-closed, every `G t ⊆ S`.
* `no_gb_of_run_of_low_lt`: with `finrank (low S) < k`, no basis of the run is
  a Gröbner basis of `I` (degree-compatible order, `1 ∉ I`, `k` linear leading
  variables in `I`). So F4's maximal step degree exceeds `D`.
* `dclosed_of_basis`: closure of `span B` follows from a check on `B` alone,
  given that the elements of `span B` of degree `< D` are spanned by the
  elements of `B` of degree `< D` (true for an echelon basis in a
  degree-graded column order). This is the finite certificate a census can
  emit and a checker can re-run.

What the model does not cover: an implementation that multiplies by a
polynomial that is not a monomial, or that reaches degree above `D` in some
step. F4 as published does neither; the second is what "maximal step degree
`D`" excludes. That a given implementation (msolve, Magma, FGb) fits the model
is a reading of its source, not a theorem here.
-/

open MvPolynomial
open scoped BigOperators

namespace SolvingDegreeBound

variable {σ : Type*} {K : Type*} [Field K]

/-- `S` is closed under multiplication by a variable whenever the product stays
within total degree `D`. -/
def DClosed (D : ℕ) (S : Submodule K (MvPolynomial σ K)) : Prop :=
  ∀ f ∈ S, f.totalDegree + 1 ≤ D → ∀ i : σ, X i * f ∈ S

/-- Rows F4 may build from the polynomials `A` at step degree at most `D`. -/
def StepRows (D : ℕ) (A : Set (MvPolynomial σ K)) : Set (MvPolynomial σ K) :=
  {p | ∃ s : σ →₀ ℕ, ∃ g ∈ A, p = monomial s (1 : K) * g ∧ p.totalDegree ≤ D}

/-- Everything the run has produced up to step `t`. -/
def Past (G : ℕ → Set (MvPolynomial σ K)) (t : ℕ) : Set (MvPolynomial σ K) :=
  {p | ∃ u ≤ t, p ∈ G u}

/-- An F4 run with maximal step degree `D` from the generators `F`. -/
def F4Run (D : ℕ) (F : Set (MvPolynomial σ K)) (G : ℕ → Set (MvPolynomial σ K)) : Prop :=
  G 0 ⊆ Submodule.span K F ∧
    ∀ t, G (t + 1) ⊆ Submodule.span K (Past G t ∪ StepRows D (Past G t))

theorem totalDegree_le_mul {a : MvPolynomial σ K} (ha : a ≠ 0) (b : MvPolynomial σ K) :
    b.totalDegree ≤ (a * b).totalDegree := by
  by_cases hb : b = 0
  · simp [hb]
  · rw [totalDegree_mul_of_isDomain ha hb]; omega

theorem X_mul_mem {D : ℕ} {S : Submodule K (MvPolynomial σ K)} (hS : DClosed D S)
    {f : MvPolynomial σ K} (hf : f ∈ S) (i : σ) (hd : (X i * f).totalDegree ≤ D) :
    X i * f ∈ S := by
  by_cases h0 : f = 0
  · simp [h0]
  · apply hS f hf _ i
    have := totalDegree_mul_of_isDomain (X_ne_zero i) h0
    rw [totalDegree_X] at this
    omega

theorem X_pow_mul_mem {D : ℕ} {S : Submodule K (MvPolynomial σ K)} (hS : DClosed D S)
    (i : σ) : ∀ (b : ℕ) (q : MvPolynomial σ K), q ∈ S →
      (X i ^ b * q).totalDegree ≤ D → X i ^ b * q ∈ S := by
  intro b
  induction b with
  | zero => intro q hq _; simpa using hq
  | succ b ih =>
    intro q hq hd
    have heq : X i ^ (b + 1) * q = X i * (X i ^ b * q) := by ring
    rw [heq] at hd ⊢
    apply X_mul_mem hS _ i hd
    exact ih q hq (le_trans (totalDegree_le_mul (X_ne_zero i) _) hd)

/-- A step row built from an element of a `D`-closed subspace stays in it. -/
theorem monomial_mul_mem {D : ℕ} {S : Submodule K (MvPolynomial σ K)} (hS : DClosed D S)
    (s : σ →₀ ℕ) : ∀ f ∈ S, (monomial s (1 : K) * f).totalDegree ≤ D →
      monomial s (1 : K) * f ∈ S := by
  induction s using Finsupp.induction with
  | zero => intro f hf _; simpa using hf
  | single_add a b t _ _ ih =>
    intro f hf hd
    have heq : monomial (Finsupp.single a b + t) (1 : K) * f =
        X a ^ b * (monomial t (1 : K) * f) := by
      rw [monomial_single_add, mul_assoc]
    rw [heq] at hd ⊢
    apply X_pow_mul_mem hS a b _ _ hd
    exact ih f hf (le_trans (totalDegree_le_mul (pow_ne_zero _ (X_ne_zero a)) _) hd)

theorem stepRows_subset {D : ℕ} {S : Submodule K (MvPolynomial σ K)} (hS : DClosed D S)
    {A : Set (MvPolynomial σ K)} (hA : A ⊆ S) : StepRows D A ⊆ S := by
  rintro p ⟨s, g, hg, rfl, hd⟩
  exact monomial_mul_mem hS s g (hA hg) hd

/-- **Containment.** Every basis of an F4 run with step degree at most `D` lies in
any `D`-closed subspace that contains the generators. -/
theorem run_subset {D : ℕ} {S : Submodule K (MvPolynomial σ K)} (hS : DClosed D S)
    {F : Set (MvPolynomial σ K)} (hF : F ⊆ S) {G : ℕ → Set (MvPolynomial σ K)}
    (hrun : F4Run D F G) : ∀ t, G t ⊆ S := by
  have key : ∀ t, ∀ u ≤ t, G u ⊆ S := by
    intro t
    induction t with
    | zero =>
      intro u hu
      have : u = 0 := by omega
      subst this
      exact hrun.1.trans (Submodule.span_le.mpr hF)
    | succ t ih =>
      intro u hu
      rcases Nat.lt_or_ge u (t + 1) with h | h
      · exact ih u (by omega)
      · have : u = t + 1 := by omega
        subst this
        have hpast : Past G t ⊆ S := by
          rintro p ⟨w, hw, hp⟩
          exact ih w hw hp
        refine (hrun.2 t).trans (Submodule.span_le.mpr ?_)
        exact Set.union_subset hpast (stepRows_subset hS hpast)
  exact fun t => key t t le_rfl

/-- **The census bound for F4.** If the degree-≤1 part of a `D`-closed,
finite-dimensional subspace containing the generators has dimension `< k`, no
basis an F4 run with step degree at most `D` ever holds is a Gröbner basis. -/
theorem no_gb_of_run_of_low_lt (m : MonomialOrder σ) (hm : DegCompat m)
    (I : Ideal (MvPolynomial σ K)) {D : ℕ} (S : Submodule K (MvPolynomial σ K))
    [FiniteDimensional K S] (hS : DClosed D S)
    {F : Set (MvPolynomial σ K)} (hF : F ⊆ S) {G : ℕ → Set (MvPolynomial σ K)}
    (hrun : F4Run D F G) (h1 : (1 : MvPolynomial σ K) ∉ I)
    {k : ℕ} (v : Fin k → σ) (hv : Function.Injective v)
    (L : Fin k → MvPolynomial σ K) (hL : ∀ j, L j ∈ I)
    (hLdeg : ∀ j, m.degree (L j) = Finsupp.single (v j) 1)
    (hlow : Module.finrank K (low S) < k) :
    ∀ t, ¬ IsGB m (G t) I := by
  intro t hGB
  have := linear_count_of_gb_in_subspace m hm I S (G t) (run_subset hS hF hrun t) hGB h1
    v hv L hL hLdeg
  omega

/-- **Finite certificate for closure.** For `S = span B`: if the elements of `S`
of degree `< D` are spanned by the elements of `B` of degree `< D`, and each such
basis element times each variable lies in `S`, then `S` is `D`-closed. -/
theorem dclosed_of_basis {D : ℕ} (B : Set (MvPolynomial σ K))
    (hlow : ∀ f ∈ Submodule.span K B, f.totalDegree + 1 ≤ D →
      f ∈ Submodule.span K {b | b ∈ B ∧ b.totalDegree + 1 ≤ D})
    (hmul : ∀ b ∈ B, b.totalDegree + 1 ≤ D → ∀ i : σ, X i * b ∈ Submodule.span K B) :
    DClosed D (Submodule.span K B) := by
  intro f hf hd i
  have hfl := hlow f hf hd
  -- multiplication by `X i` is linear; check it on the spanning set
  have hmap : ∀ g ∈ Submodule.span K {b | b ∈ B ∧ b.totalDegree + 1 ≤ D},
      X i * g ∈ Submodule.span K B := by
    intro g hg
    induction hg using Submodule.span_induction with
    | mem x hx => exact hmul x hx.1 hx.2 i
    | zero => simp
    | add x y _ _ hx hy => rw [mul_add]; exact Submodule.add_mem _ hx hy
    | smul c x _ hx =>
      rw [mul_smul_comm]; exact Submodule.smul_mem _ c hx
  exact hmap f hfl

end SolvingDegreeBound
