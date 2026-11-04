import SolvingDegreeBound

/-!
# The linear-count criterion for any finite-dimensional subspace

`SolvingDegreeBound.linear_count_of_gb_in_span` bounds the number of linear
leading monomials a Gröbner basis can supply when the basis lies in `span_K F`,
i.e. in the degree-2 Macaulay row space of degree-2 generators. The same
argument works for a Gröbner basis lying in **any** finite-dimensional subspace
`S`: the row space of the Macaulay matrix at any degree `d`, or anything else.

* `linear_count_of_gb_in_subspace`: `G ⊆ S`, `G` a Gröbner basis of `I`
  (degree-compatible order), `1 ∉ I`, and `I` contains `k` polynomials whose
  leading monomials are `k` distinct variables, then
  `k ≤ dim (S ∩ {totalDegree ≤ 1})`.
* `no_gb_in_rows_of_low_lt`: for a finite set of rows (e.g. all products
  monomial × generator of degree at most `d`), if the degree-≤1 part of their
  span has dimension `< k`, no Gröbner basis of `I` lies in that span. With the
  Caminata–Gorla definition this says the solving degree exceeds `d`: the
  quantity `d_lin` computed by the `pq2_rank` census is a lower bound on `sd`.
-/

open MvPolynomial
open scoped BigOperators

namespace SolvingDegreeBound

variable {σ : Type*} {K : Type*} [Field K]

theorem linear_count_of_gb_in_subspace (m : MonomialOrder σ) (hm : DegCompat m)
    (I : Ideal (MvPolynomial σ K)) (S : Submodule K (MvPolynomial σ K))
    [FiniteDimensional K S] (G : Set (MvPolynomial σ K))
    (hGS : G ⊆ S) (hGB : IsGB m G I) (h1 : (1 : MvPolynomial σ K) ∉ I)
    {k : ℕ} (v : Fin k → σ) (hv : Function.Injective v)
    (L : Fin k → MvPolynomial σ K) (hL : ∀ j, L j ∈ I)
    (hLdeg : ∀ j, m.degree (L j) = Finsupp.single (v j) 1) :
    k ≤ Module.finrank K (low S) := by
  classical
  have hLne : ∀ j, L j ≠ 0 := by
    intro j h
    have := hLdeg j
    rw [h, MonomialOrder.degree_zero] at this
    exact (Finsupp.single_ne_zero.mpr one_ne_zero) this.symm
  have hchoose : ∀ j, ∃ g ∈ G, g ≠ 0 ∧ m.degree g = Finsupp.single (v j) 1 := by
    intro j
    obtain ⟨g, hgG, hg0, hgle⟩ := hGB.2 (L j) (hL j) (hLne j)
    rw [hLdeg j] at hgle
    rcases le_single_one hgle with h0 | hs
    · exfalso
      have hgI : g ∈ I := hGB.1 hgG
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
    refine ⟨hGS (hgG j), ?_⟩
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
  have : FiniteDimensional K (low S) :=
    Submodule.finiteDimensional_of_le (inf_le_left : low S ≤ S)
  simpa using hli'.fintype_card_le_finrank

/-- Census form: `rows` is any finite set of polynomials (the Macaulay rows at
some degree). If the degree-≤1 part of their span has fewer than `k`
dimensions, no Gröbner basis of `I` lies in the span. -/
theorem no_gb_in_rows_of_low_lt (m : MonomialOrder σ) (hm : DegCompat m)
    (I : Ideal (MvPolynomial σ K)) (rows : Finset (MvPolynomial σ K))
    (h1 : (1 : MvPolynomial σ K) ∉ I)
    {k : ℕ} (v : Fin k → σ) (hv : Function.Injective v)
    (L : Fin k → MvPolynomial σ K) (hL : ∀ j, L j ∈ I)
    (hLdeg : ∀ j, m.degree (L j) = Finsupp.single (v j) 1)
    (hlow : Module.finrank K (low (Submodule.span K (rows : Set (MvPolynomial σ K)))) < k) :
    ¬ ∃ G : Set (MvPolynomial σ K),
        G ⊆ Submodule.span K (rows : Set (MvPolynomial σ K)) ∧ IsGB m G I := by
  rintro ⟨G, hGS, hGB⟩
  have : FiniteDimensional K (Submodule.span K (rows : Set (MvPolynomial σ K))) :=
    FiniteDimensional.span_finset K rows
  have := linear_count_of_gb_in_subspace m hm I _ G hGS hGB h1 v hv L hL hLdeg
  omega

end SolvingDegreeBound
