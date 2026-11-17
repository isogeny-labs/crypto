import Mathlib

/-!
# A degree-2 solving-degree lower bound, aimed at Petit–Quisquater Assumption 2

Corpus target: `HEUR-GB-PQ-ASSUMPTION-2` (WEAKNESS_TRIAGE W1).

Petit–Quisquater Assumption 2 says the degree of regularity, read as the maximal
degree reached by the solver, of a Weil-descended summation system equals its
first fall degree up to `o(1)`. At `m = 2` the first fall degree is `2` for every
`n` (Kosters–Yeo, Cor. 4.11), so the assumption read literally predicts solving
degree `2` for all large `n`.

Solving degree `≤ 2` (Caminata–Gorla: Gaussian elimination on the degree-≤2
Macaulay matrix yields a Gröbner basis for a degree-compatible order) means: for
generators of degree exactly `2`, including the field equations `x_j^2 + x_j`,
the degree-≤2 Macaulay rows are the generators themselves, so a Gröbner basis
lies in their `K`-span.

* `linear_count_of_gb_in_span`: if a Gröbner basis of `⟨F⟩` lies in `span_K F`,
  `1 ∉ ⟨F⟩`, and the ideal contains `k` polynomials whose leading monomials are
  `k` distinct variables, then `k ≤ dim (span_K F ∩ {totalDegree ≤ 1})`.
* `low_add_top_le_card`: `dim (span_K F ∩ {totalDegree ≤ 1})` plus the rank of
  the degree-2 homogeneous parts of `F` is at most `|F|` (rank–nullity).
* `solving_degree_ge_three`: together, if `k + rank(top₂ F) > |F|` then no
  Gröbner basis lies in `span_K F`: the solving degree is at least `3`.
* `not_tendsto_of_frequently_gap`: a gap of at least `1` infinitely often
  rules out `D_reg = D_ff + o(1)`.

What is **not** formalised here, and is a per-instance finite check: for the
descended `S_3` system (`e` equations, `N` bit variables, `N` field equations),
`k ≥ N − affdim(Z)` where `Z` is the solution set (the affine functions vanishing
on `Z` lie in the ideal, by the finite-field Nullstellensatz with field
equations), and `rank(top₂ F) = N + ρ` with `ρ` the rank of the square-free
quadratic parts. The numeric condition then reads `N − affdim(Z) + ρ > e`.
-/

open MvPolynomial
open scoped BigOperators

namespace SolvingDegreeBound

variable {σ : Type*} {K : Type*} [Field K]

/-- Gröbner basis (standard definition): every nonzero element of the ideal has
its leading monomial divisible by the leading monomial of a nonzero basis
element. Pointwise `≤` on exponent vectors is divisibility of monomials. -/
def IsGB (m : MonomialOrder σ) (G : Set (MvPolynomial σ K))
    (I : Ideal (MvPolynomial σ K)) : Prop :=
  G ⊆ I ∧ ∀ f ∈ I, f ≠ 0 → ∃ g ∈ G, g ≠ 0 ∧ m.degree g ≤ m.degree f

/-- Degree-compatible monomial order (e.g. grevlex, deglex). -/
def DegCompat (m : MonomialOrder σ) : Prop :=
  ∀ a b : σ →₀ ℕ, a.degree < b.degree → m.toSyn a < m.toSyn b

/-- The degree-≤1 part of a subspace. -/
noncomputable def low (S : Submodule K (MvPolynomial σ K)) : Submodule K (MvPolynomial σ K) :=
  S ⊓ restrictTotalDegree σ K 1

/-- A divisor of the exponent of a single variable is `0` or that variable. -/
theorem le_single_one {d : σ →₀ ℕ} {i : σ} (h : d ≤ Finsupp.single i 1) :
    d = 0 ∨ d = Finsupp.single i 1 := by
  classical
  have hx : ∀ x, x ≠ i → d x = 0 := by
    intro x hx
    have := h x
    simp [Ne.symm hx] at this
    exact this
  have hi : d i ≤ 1 := by simpa using h i
  rcases Nat.le_one_iff_eq_zero_or_eq_one.mp hi with h0 | h1
  · left; ext x; by_cases hxi : x = i
    · subst hxi; simp [h0]
    · simp [hx x hxi]
  · right; ext x; by_cases hxi : x = i
    · subst hxi; simp [h1]
    · simp [hx x hxi, Ne.symm hxi]

/-- Under a degree-compatible order, leading monomial `x_i` forces total degree `≤ 1`. -/
theorem totalDegree_le_one_of_degree_single {m : MonomialOrder σ} (hm : DegCompat m)
    {g : MvPolynomial σ K} {i : σ} (hg : m.degree g = Finsupp.single i 1) :
    g.totalDegree ≤ 1 := by
  rw [totalDegree, Finset.sup_le_iff]
  intro s hs
  have hle := m.le_degree hs
  rw [hg] at hle
  have hsd : s.degree ≤ 1 := by
    by_contra hlt
    have hdeg : (Finsupp.single i 1 : σ →₀ ℕ).degree < s.degree := by
      rw [Finsupp.degree_single]; omega
    exact absurd hle (not_le.mpr (hm _ _ hdeg))
  simpa [Finsupp.sum, Finsupp.degree_apply] using hsd

/-- Nonzero polynomials with pairwise distinct leading monomials are linearly
independent. -/
theorem linearIndependent_of_degree_injective {m : MonomialOrder σ} {k : ℕ}
    (g : Fin k → MvPolynomial σ K) (hne : ∀ j, g j ≠ 0)
    (hinj : Function.Injective fun j => m.degree (g j)) :
    LinearIndependent K g := by
  classical
  rw [Fintype.linearIndependent_iff]
  intro a ha
  by_contra hnot
  push Not at hnot
  set T := Finset.univ.filter fun j => a j ≠ 0
  have hT : T.Nonempty := by
    obtain ⟨j, hj⟩ := hnot
    exact ⟨j, by simp [T, hj]⟩
  obtain ⟨j0, hj0T, hmax⟩ := T.exists_max_image (fun j => m.toSyn (m.degree (g j))) hT
  have hj0 : a j0 ≠ 0 := by simpa [T] using hj0T
  -- coefficient of the sum at the top leading monomial
  have hcoeff := congrArg (fun p : MvPolynomial σ K => p.coeff (m.degree (g j0))) ha
  simp only [coeff_sum, coeff_smul, smul_eq_mul] at hcoeff
  rw [show (0 : MvPolynomial σ K).coeff (m.degree (g j0)) = 0 by simp] at hcoeff
  rw [Finset.sum_eq_single j0] at hcoeff
  · have hlc : (g j0).coeff (m.degree (g j0)) ≠ 0 := by
      rw [← mem_support_iff]; exact m.degree_mem_support (hne j0)
    exact (mul_ne_zero hj0 hlc) hcoeff
  · intro j _ hjne
    by_cases haj : a j = 0
    · simp [haj]
    · have hjT : j ∈ T := by simp [T, haj]
      have hle := hmax j hjT
      have hneq : m.degree (g j) ≠ m.degree (g j0) := fun h => hjne (hinj h)
      have hlt : m.toSyn (m.degree (g j)) < m.toSyn (m.degree (g j0)) :=
        lt_of_le_of_ne hle (fun h => hneq (m.toSyn.injective h))
      rw [m.coeff_eq_zero_of_lt hlt, mul_zero]
  · intro h; exact absurd (Finset.mem_univ j0) h

theorem span_le_ideal (F : Finset (MvPolynomial σ K)) :
    (Submodule.span K (F : Set (MvPolynomial σ K)) : Set (MvPolynomial σ K)) ⊆
      (Ideal.span (F : Set (MvPolynomial σ K)) : Set (MvPolynomial σ K)) := by
  intro p hp
  have : Submodule.span K (F : Set (MvPolynomial σ K)) ≤
      (Ideal.span (F : Set (MvPolynomial σ K))).restrictScalars K :=
    Submodule.span_le.mpr (fun f hf => Ideal.subset_span hf)
  exact this hp

/-- **Main counting step.** A Gröbner basis inside `span_K F` (solving degree ≤ 2
for degree-2 generators) can supply at most `dim low(span F)` linear leading
monomials. -/
theorem linear_count_of_gb_in_span (m : MonomialOrder σ) (hm : DegCompat m)
    (F : Finset (MvPolynomial σ K)) (G : Set (MvPolynomial σ K))
    (hGspan : G ⊆ Submodule.span K (F : Set (MvPolynomial σ K)))
    (hGB : IsGB m G (Ideal.span (F : Set (MvPolynomial σ K))))
    (h1 : (1 : MvPolynomial σ K) ∉ Ideal.span (F : Set (MvPolynomial σ K)))
    {k : ℕ} (v : Fin k → σ) (hv : Function.Injective v)
    (L : Fin k → MvPolynomial σ K)
    (hL : ∀ j, L j ∈ Ideal.span (F : Set (MvPolynomial σ K)))
    (hLdeg : ∀ j, m.degree (L j) = Finsupp.single (v j) 1) :
    k ≤ Module.finrank K (low (Submodule.span K (F : Set (MvPolynomial σ K)))) := by
  classical
  set S := Submodule.span K (F : Set (MvPolynomial σ K))
  have hLne : ∀ j, L j ≠ 0 := by
    intro j h
    have := hLdeg j
    rw [h, MonomialOrder.degree_zero] at this
    exact (Finsupp.single_ne_zero.mpr one_ne_zero) this.symm
  -- choose a basis element dividing each linear leading monomial
  have hchoose : ∀ j, ∃ g ∈ G, g ≠ 0 ∧ m.degree g = Finsupp.single (v j) 1 := by
    intro j
    obtain ⟨g, hgG, hg0, hgle⟩ := hGB.2 (L j) (hL j) (hLne j)
    rw [hLdeg j] at hgle
    rcases le_single_one hgle with h0 | hs
    · exfalso
      have hgI : g ∈ Ideal.span (F : Set (MvPolynomial σ K)) := hGB.1 hgG
      have hC := m.eq_C_of_degree_eq_zero h0
      have hc : m.leadingCoeff g ≠ 0 := m.leadingCoeff_ne_zero_iff.mpr hg0
      generalize m.leadingCoeff g = c at hC hc
      apply h1
      have : (1 : MvPolynomial σ K) = C c⁻¹ * g := by
        rw [hC, ← C_mul, inv_mul_cancel₀ hc, C_1]
      rw [this]
      exact Ideal.mul_mem_left _ _ hgI
    · exact ⟨g, hgG, hg0, hs⟩
  choose g hgG hg0 hgdeg using hchoose
  have hgS : ∀ j, g j ∈ low S := by
    intro j
    refine ⟨hGspan (hgG j), ?_⟩
    show g j ∈ restrictTotalDegree σ K 1
    exact (mem_restrictTotalDegree _ _ _).mpr (totalDegree_le_one_of_degree_single hm (hgdeg j))
  have hinj : Function.Injective fun j => m.degree (g j) := by
    intro a b hab
    simp only [hgdeg] at hab
    exact hv (Finsupp.single_left_injective one_ne_zero hab)
  have hli : LinearIndependent K g := linearIndependent_of_degree_injective g hg0 hinj
  let g' : Fin k → low S := fun j => ⟨g j, hgS j⟩
  have hli' : LinearIndependent K g' := by
    apply LinearIndependent.of_comp (low S).subtype
    exact hli
  have : FiniteDimensional K S := FiniteDimensional.span_finset K F
  have : FiniteDimensional K (low S) :=
    Submodule.finiteDimensional_of_le (inf_le_left : low S ≤ S)
  simpa using hli'.fintype_card_le_finrank

/-- Rank–nullity: the degree-≤1 part of `span F` and the span of the degree-2
homogeneous parts of `F` together have dimension at most `|F|`. -/
theorem low_add_top_le_card (F : Finset (MvPolynomial σ K)) :
    Module.finrank K (low (Submodule.span K (F : Set (MvPolynomial σ K)))) +
      Module.finrank K (Submodule.span K
        ((homogeneousComponent 2 : MvPolynomial σ K →ₗ[K] MvPolynomial σ K) '' F))
      ≤ F.card := by
  classical
  set S := Submodule.span K (F : Set (MvPolynomial σ K))
  have : FiniteDimensional K S := FiniteDimensional.span_finset K F
  set φ : S →ₗ[K] MvPolynomial σ K := (homogeneousComponent 2).domRestrict S
  have hrange : LinearMap.range φ =
      Submodule.span K ((homogeneousComponent 2 : MvPolynomial σ K →ₗ[K] _) '' F) := by
    rw [LinearMap.range_domRestrict, Submodule.map_span]
  have hrn := LinearMap.finrank_range_add_finrank_ker φ
  -- low S injects into ker φ
  let ι : low S →ₗ[K] LinearMap.ker φ :=
    { toFun := fun p => ⟨⟨p.1, p.2.1⟩, by
        simp only [LinearMap.mem_ker, φ, LinearMap.domRestrict_apply]
        have hp : (p.1 : MvPolynomial σ K) ∈ restrictTotalDegree σ K 1 := p.2.2
        exact homogeneousComponent_eq_zero 2 p.1
          (lt_of_le_of_lt ((mem_restrictTotalDegree _ _ _).mp hp) (by norm_num))⟩
      map_add' := by intro a b; rfl
      map_smul' := by intro c a; rfl }
  have hι : Function.Injective ι := by
    intro a b h
    have := congrArg (fun x : LinearMap.ker φ => ((x : S) : MvPolynomial σ K)) h
    exact Subtype.ext this
  have hlow : Module.finrank K (low S) ≤ Module.finrank K (LinearMap.ker φ) :=
    LinearMap.finrank_le_finrank_of_injective hι
  have hS : Module.finrank K S ≤ F.card := finrank_span_finset_le_card F
  rw [← hrange]
  omega

/-- **Solving degree ≥ 3.** If the ideal has `k` linear elements with distinct
leading variables and `k + rank(top₂ F) > |F|`, then no Gröbner basis for a
degree-compatible order lies in `span_K F`. -/
theorem solving_degree_ge_three (m : MonomialOrder σ) (hm : DegCompat m)
    (F : Finset (MvPolynomial σ K))
    (h1 : (1 : MvPolynomial σ K) ∉ Ideal.span (F : Set (MvPolynomial σ K)))
    {k : ℕ} (v : Fin k → σ) (hv : Function.Injective v)
    (L : Fin k → MvPolynomial σ K)
    (hL : ∀ j, L j ∈ Ideal.span (F : Set (MvPolynomial σ K)))
    (hLdeg : ∀ j, m.degree (L j) = Finsupp.single (v j) 1)
    (hcount : F.card < k + Module.finrank K (Submodule.span K
        ((homogeneousComponent 2 : MvPolynomial σ K →ₗ[K] MvPolynomial σ K) '' F))) :
    ¬ ∃ G : Set (MvPolynomial σ K),
        G ⊆ Submodule.span K (F : Set (MvPolynomial σ K)) ∧
        IsGB m G (Ideal.span (F : Set (MvPolynomial σ K))) := by
  rintro ⟨G, hGspan, hGB⟩
  have h1' := linear_count_of_gb_in_span m hm F G hGspan hGB h1 v hv L hL hLdeg
  have h2 := low_add_top_le_card F
  omega

/-- An integer gap of at least one, infinitely often, is not `o(1)`. -/
theorem not_tendsto_of_frequently_gap (dreg dff : ℕ → ℕ)
    (h : ∃ᶠ n in Filter.atTop, dff n + 1 ≤ dreg n) :
    ¬ Filter.Tendsto (fun n => (dreg n : ℝ) - dff n) Filter.atTop (nhds 0) := by
  intro ht
  have hev : ∀ᶠ n in Filter.atTop, |(dreg n : ℝ) - dff n| < 1 := by
    have := (Metric.tendsto_nhds.mp ht) 1 one_pos
    simpa [Real.dist_eq] using this
  obtain ⟨n, hn1, hn2⟩ := (h.and_eventually hev).exists
  have : (1 : ℝ) ≤ (dreg n : ℝ) - dff n := by
    have : ((dff n + 1 : ℕ) : ℝ) ≤ dreg n := by exact_mod_cast hn1
    push_cast at this; linarith
  rw [abs_lt] at hn2
  linarith [hn2.2]

/-! ## Non-vacuity: a concrete system over `F_2` with solving degree at least 3

`F = {x₀² + x₀, x₁² + x₁, x₀x₁ + x₀ + x₁}` has the single `F_2`-solution `(0,0)`,
so `x₀, x₁ ∈ ⟨F⟩` (`k = 2`), while the quadratic parts `x₀², x₁², x₀x₁` have rank
`3`: `|F| = 3 < 2 + 3`. No Gröbner basis for degLex lies in `span F`. -/

theorem degLex_degCompat : DegCompat (MonomialOrder.degLex : MonomialOrder (Fin 2)) := by
  intro a b h
  rw [MonomialOrder.degLex_lt_iff, Finsupp.DegLex.lt_iff]
  left
  simpa using h

namespace Toy

local notation "R" => MvPolynomial (Fin 2) (ZMod 2)

noncomputable def e0 : R := X 0 ^ 2 + X 0
noncomputable def e1 : R := X 1 ^ 2 + X 1
noncomputable def f : R := X 0 * X 1 + X 0 + X 1

open Classical in
noncomputable def F : Finset R := {e0, e1, f}

theorem two_eq_zero : (2 : R) = 0 := by
  rw [show (2 : R) = C 2 from (map_ofNat C 2).symm]
  rw [show (2 : ZMod 2) = 0 from rfl, C_0]

theorem mem_F {p : R} (hp : p = e0 ∨ p = e1 ∨ p = f) : p ∈ (F : Set R) := by
  classical
  rcases hp with h | h | h <;> simp [F, h]

theorem X0_mem : (X 0 : R) ∈ Ideal.span (F : Set R) := by
  have h : (X 0 : R) = X 0 * f + X 1 * e0 + e0 := by
    simp only [f, e0]
    linear_combination (-(X 0 ^ 2 * X 1 + X 0 ^ 2 + X 0 * X 1 : R)) * two_eq_zero
  rw [h]
  refine Ideal.add_mem _ (Ideal.add_mem _ ?_ ?_) ?_
  · exact Ideal.mul_mem_left _ _ (Ideal.subset_span (mem_F (Or.inr (Or.inr rfl))))
  · exact Ideal.mul_mem_left _ _ (Ideal.subset_span (mem_F (Or.inl rfl)))
  · exact Ideal.subset_span (mem_F (Or.inl rfl))

theorem X1_mem : (X 1 : R) ∈ Ideal.span (F : Set R) := by
  have h : (X 1 : R) = X 1 * f + X 0 * e1 + e1 := by
    simp only [f, e1]
    linear_combination (-(X 1 ^ 2 * X 0 + X 1 ^ 2 + X 0 * X 1 : R)) * two_eq_zero
  rw [h]
  refine Ideal.add_mem _ (Ideal.add_mem _ ?_ ?_) ?_
  · exact Ideal.mul_mem_left _ _ (Ideal.subset_span (mem_F (Or.inr (Or.inr rfl))))
  · exact Ideal.mul_mem_left _ _ (Ideal.subset_span (mem_F (Or.inr (Or.inl rfl))))
  · exact Ideal.subset_span (mem_F (Or.inr (Or.inl rfl)))

theorem one_not_mem : (1 : R) ∉ Ideal.span (F : Set R) := by
  classical
  intro h
  have hle : Ideal.span (F : Set R) ≤ RingHom.ker (eval (0 : Fin 2 → ZMod 2)) := by
    rw [Ideal.span_le]
    intro p hp
    simp only [F, Finset.coe_insert, Finset.coe_singleton, Set.mem_insert_iff,
      Set.mem_singleton_iff] at hp
    rcases hp with rfl | rfl | rfl <;> simp [e0, e1, f]
  have := hle h
  simp at this

theorem top_e0 : homogeneousComponent 2 e0 = (X 0 ^ 2 : R) := by
  simp only [e0, map_add]
  rw [homogeneousComponent_eq_self (isHomogeneous_X_pow _ _),
    homogeneousComponent_of_mem (isHomogeneous_X _ _)]
  simp

theorem top_e1 : homogeneousComponent 2 e1 = (X 1 ^ 2 : R) := by
  simp only [e1, map_add]
  rw [homogeneousComponent_eq_self (isHomogeneous_X_pow _ _),
    homogeneousComponent_of_mem (isHomogeneous_X _ _)]
  simp

theorem top_f : homogeneousComponent 2 f = (X 0 * X 1 : R) := by
  simp only [f, map_add]
  rw [homogeneousComponent_eq_self ((isHomogeneous_X _ _).mul (isHomogeneous_X _ _)),
    homogeneousComponent_of_mem (isHomogeneous_X _ _),
    homogeneousComponent_of_mem (isHomogeneous_X _ _)]
  simp

theorem rank_top : 3 ≤ Module.finrank (ZMod 2) (Submodule.span (ZMod 2)
    ((homogeneousComponent 2 : R →ₗ[ZMod 2] R) '' F)) := by
  classical
  set W := Submodule.span (ZMod 2) ((homogeneousComponent 2 : R →ₗ[ZMod 2] R) '' F)
  let g : Fin 3 → R := ![X 0 ^ 2, X 1 ^ 2, X 0 * X 1]
  have hgW : ∀ j, g j ∈ W := by
    intro j
    apply Submodule.subset_span
    fin_cases j
    · exact ⟨e0, mem_F (Or.inl rfl), top_e0⟩
    · exact ⟨e1, mem_F (Or.inr (Or.inl rfl)), top_e1⟩
    · exact ⟨f, mem_F (Or.inr (Or.inr rfl)), top_f⟩
  have hdeg : ∀ j, (MonomialOrder.degLex : MonomialOrder (Fin 2)).degree (g j) =
      ![Finsupp.single 0 2, Finsupp.single 1 2, Finsupp.single 0 1 + Finsupp.single 1 1] j := by
    intro j
    fin_cases j
    · simp [g, MonomialOrder.degree_pow, MonomialOrder.degree_X]
    · simp [g, MonomialOrder.degree_pow, MonomialOrder.degree_X]
    · show (MonomialOrder.degLex : MonomialOrder (Fin 2)).degree ((X 0 : R) * X 1) = _
      rw [MonomialOrder.degree_mul (X_ne_zero 0) (X_ne_zero 1), MonomialOrder.degree_X,
        MonomialOrder.degree_X]
      rfl
  have hne : ∀ j, g j ≠ 0 := by
    intro j; fin_cases j <;> simp [g, X_ne_zero]
  have hinj : Function.Injective fun j => (MonomialOrder.degLex : MonomialOrder (Fin 2)).degree (g j) := by
    intro a b hab
    simp only [hdeg] at hab
    fin_cases a <;> fin_cases b <;> simp_all [Finsupp.single_eq_single_iff] <;>
      (have := congrArg (fun d : Fin 2 →₀ ℕ => d 0) hab; simp at this)
  have hli := linearIndependent_of_degree_injective (m := MonomialOrder.degLex) g hne hinj
  let g' : Fin 3 → W := fun j => ⟨g j, hgW j⟩
  have hli' : LinearIndependent (ZMod 2) g' :=
    LinearIndependent.of_comp W.subtype hli
  have : FiniteDimensional (ZMod 2) W := FiniteDimensional.span_of_finite _
    ((F.finite_toSet).image _)
  simpa using hli'.fintype_card_le_finrank

/-- The toy system has solving degree at least `3` for degLex. -/
theorem toy_solving_degree_ge_three :
    ¬ ∃ G : Set R, G ⊆ Submodule.span (ZMod 2) (F : Set R) ∧
        IsGB MonomialOrder.degLex G (Ideal.span (F : Set R)) := by
  apply solving_degree_ge_three MonomialOrder.degLex degLex_degCompat F one_not_mem
    (k := 2) id Function.injective_id ![X 0, X 1]
  · intro j; fin_cases j
    · exact X0_mem
    · exact X1_mem
  · intro j; fin_cases j <;> simp [MonomialOrder.degree_X]
  · have hcard : F.card ≤ 3 := by
      classical
      simpa [F] using Finset.card_le_three
    have := rank_top
    omega

end Toy

end SolvingDegreeBound
