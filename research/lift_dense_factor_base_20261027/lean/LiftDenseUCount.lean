import Mathlib

/-
  Lift-dense factor bases, Proposition 3 (the `u`-count), formalised.

  `uSet L c` is `{u ∈ M* : Tr_{M/L}(u) = 0 ∧ Tr_{M/L}(c u⁻¹) = 0}`.
  Proved here: (c) `(q-1) ∣ |uSet|` for every finite `M/L`; (a) the `n = 2` case in
  characteristic 2 (iff and exact count); (b) a nonzero binary quadratic form over a finite field
  has at most `2 (q-1)` nonzero zeros, and the `n = 3` classification
  `|uSet| ∈ {0, q-1, 2(q-1), q²-1}`. See `PROOF_MAP.md`.
-/

namespace LiftDenseUCount

open Algebra MulAction

/-- A free action of a finite group on a finite type: `|G|` divides the size of the type. -/
theorem free_dvd {G X : Type*} [Group G] [Finite G] [MulAction G X] [Finite X]
    (hfree : ∀ (g : G) (x : X), g • x = x → g = 1) :
    Nat.card G ∣ Nat.card X := by
  classical
  have horb : ∀ x : X, Nat.card (orbit G x) = Nat.card G := by
    intro x
    refine (Nat.card_congr (Equiv.ofBijective (fun g : G => (⟨g • x, g, rfl⟩ : orbit G x))
      ⟨fun g h hgh => ?_, fun ⟨_, g, hg⟩ => ⟨g, Subtype.ext hg⟩⟩)).symm
    have h1 : g • x = h • x := congrArg Subtype.val hgh
    have h2 : (h⁻¹ * g) • x = x := by rw [mul_smul, h1, inv_smul_smul]
    have := hfree _ x h2
    exact (inv_mul_eq_one.mp this).symm
  have := Fintype.ofFinite (orbitRel.Quotient G X)
  rw [Nat.card_congr (selfEquivSigmaOrbits G X), Nat.card_sigma]
  simp only [horb]
  rw [Finset.sum_const, smul_eq_mul]
  exact dvd_mul_left _ _

section UCount

variable (L : Type*) {M : Type*} [Field L] [Field M] [Algebra L M]

/-- The `u`-set of Proposition 3: nonzero `u ∈ M` with `Tr_{M/L}(u) = 0` and
`Tr_{M/L}(c u⁻¹) = 0`. -/
def uSet (c : M) : Set M :=
  {u | u ≠ 0 ∧ Algebra.trace L M u = 0 ∧ Algebra.trace L M (c * u⁻¹) = 0}

variable {L}

theorem smul_mem_uSet (c : M) (g : Lˣ) {u : M} (hu : u ∈ uSet L c) : g • u ∈ uSet L c := by
  obtain ⟨h0, h1, h2⟩ := hu
  refine ⟨?_, ?_, ?_⟩
  · rw [Units.smul_def]; exact smul_ne_zero g.ne_zero h0
  · rw [Units.smul_def, map_smul, h1, smul_zero]
  · have : c * ((g : L) • u)⁻¹ = (g : L)⁻¹ • (c * u⁻¹) := by
      simp only [Algebra.smul_def, map_inv₀, mul_inv]; ring
    rw [Units.smul_def, this, map_smul, h2, smul_zero]

theorem uCount_dvd [Finite L] [Module.Finite L M] (c : M) :
    (Nat.card L - 1) ∣ Nat.card (uSet L c) := by
  have : Finite M := Module.finite_of_finite L
  let S : SubMulAction Lˣ M := ⟨uSet L c, fun g u hu => smul_mem_uSet c g hu⟩
  have h := free_dvd (G := Lˣ) (X := S) (by
    rintro g ⟨x, hx⟩ hg
    have hg' : g • x = x := congrArg Subtype.val hg
    have h2 : ((g : L) - 1) • x = 0 := by
      rw [sub_smul, one_smul, ← Units.smul_def, hg', sub_self]
    rcases smul_eq_zero.mp h2 with h | h
    · exact Units.ext (sub_eq_zero.mp h)
    · exact absurd h hx.1)
  rw [Nat.card_units] at h
  exact h

end UCount


section Two

variable {L : Type*} {M : Type*} [Field L] [Field M] [Algebra L M]
  [Finite L] [CharP L 2] [Module.Finite L M]

/-- For `[M : L] = 2` in characteristic 2, `ker Tr_{M/L} = L`. -/
theorem trace_eq_zero_iff_of_finrank_two (hn : Module.finrank L M = 2) (u : M) :
    Algebra.trace L M u = 0 ↔ ∃ l : L, algebraMap L M l = u := by
  have hrange : LinearMap.range (Algebra.linearMap L M) ≤ LinearMap.ker (Algebra.trace L M) := by
    rintro _ ⟨l, rfl⟩
    show Algebra.trace L M (algebraMap L M l) = 0
    rw [Algebra.trace_algebraMap, hn, two_nsmul, CharTwo.add_self_eq_zero]
  have heq : LinearMap.range (Algebra.linearMap L M) = LinearMap.ker (Algebra.trace L M) := by
    apply Submodule.eq_of_le_of_finrank_eq hrange
    have h1 : Module.finrank L (LinearMap.range (Algebra.trace L M)) = 1 := by
      have : LinearMap.range (Algebra.trace L M) = ⊤ :=
        LinearMap.range_eq_top.mpr (Algebra.trace_surjective L M)
      rw [this, finrank_top, Module.finrank_self]
    have h2 := LinearMap.finrank_range_add_finrank_ker (Algebra.trace L M)
    have h3 : Module.finrank L (LinearMap.range (Algebra.linearMap L M)) = 1 := by
      rw [LinearMap.finrank_range_of_inj (algebraMap L M).injective, Module.finrank_self]
    omega
  constructor
  · intro h
    have h' : u ∈ LinearMap.ker (Algebra.trace L M) := h
    rw [← heq] at h'
    obtain ⟨l, hl⟩ := h'
    exact ⟨l, hl⟩
  · rintro ⟨l, rfl⟩
    have : algebraMap L M l ∈ LinearMap.ker (Algebra.trace L M) := hrange ⟨l, rfl⟩
    exact this

/-- Proposition 3(a), existence form: for `n = 2` the `u`-set is nonempty iff `c ∈ L`. -/
theorem uSet_nonempty_iff_of_finrank_two (hn : Module.finrank L M = 2) (c : M) :
    (uSet L c).Nonempty ↔ ∃ l : L, algebraMap L M l = c := by
  constructor
  · rintro ⟨u, hu0, hu1, hu2⟩
    obtain ⟨l, rfl⟩ := (trace_eq_zero_iff_of_finrank_two hn _).mp hu1
    obtain ⟨m, hm⟩ := (trace_eq_zero_iff_of_finrank_two hn _).mp hu2
    have hl : l ≠ 0 := fun h => hu0 (by rw [h, map_zero])
    refine ⟨m * l, ?_⟩
    rw [map_mul, hm]
    have : algebraMap L M l ≠ 0 := hu0
    field_simp
  · rintro ⟨l, rfl⟩
    refine ⟨1, one_ne_zero, ?_, ?_⟩
    · exact (trace_eq_zero_iff_of_finrank_two hn _).mpr ⟨1, map_one _⟩
    · rw [inv_one, mul_one]
      exact (trace_eq_zero_iff_of_finrank_two hn _).mpr ⟨l, rfl⟩

open scoped Classical in
/-- Proposition 3(a), exact count: for `[M : L] = 2` in characteristic 2 the `u`-count is
`q - 1` if `c ∈ L` and `0` otherwise. -/
theorem uCount_of_finrank_two (hn : Module.finrank L M = 2) (c : M) :
    Nat.card (uSet L c) =
      if ∃ l : L, algebraMap L M l = c then Nat.card L - 1 else 0 := by
  split_ifs with h
  · obtain ⟨l0, rfl⟩ := h
    have hset : uSet L (algebraMap L M l0) = algebraMap L M '' {l : L | l ≠ 0} := by
      ext u
      constructor
      · rintro ⟨hu0, hu1, -⟩
        obtain ⟨l, rfl⟩ := (trace_eq_zero_iff_of_finrank_two hn _).mp hu1
        exact ⟨l, fun h => hu0 (by rw [h, map_zero]), rfl⟩
      · rintro ⟨l, hl, rfl⟩
        refine ⟨fun h => hl ((map_eq_zero_iff _ (algebraMap L M).injective).mp h),
          (trace_eq_zero_iff_of_finrank_two hn _).mpr ⟨l, rfl⟩, ?_⟩
        refine (trace_eq_zero_iff_of_finrank_two hn _).mpr ⟨l0 * l⁻¹, ?_⟩
        simp
    rw [hset, Nat.card_image_of_injective (algebraMap L M).injective]
    have : Nat.card {l : L | l ≠ 0} = Nat.card Lˣ :=
      Nat.card_congr (unitsEquivNeZero (G₀ := L)).symm
    rw [this, Nat.card_units]
  · have : uSet L c = ∅ := by
      by_contra hne
      exact h ((uSet_nonempty_iff_of_finrank_two hn c).mp (Set.nonempty_iff_ne_empty.mpr hne))
    rw [this]; simp

end Two

section Quadratic

/-- Nonzero elements of a finite type with zero: `|α| - 1` of them. -/
theorem ncard_compl_zero' {α : Type*} [Zero α] [Finite α] :
    ({0}ᶜ : Set α).ncard = Nat.card α - 1 := by
  rw [Set.compl_eq_univ_sdiff, Set.ncard_sdiff_singleton_of_mem (Set.mem_univ _), Set.ncard_univ]

variable {F : Type*} [Field F] [Finite F]

/-- Nonzero zeros in `F × F` of the binary quadratic form `a x² + b x y + c y²`. -/
def qZero (a b c : F) : Set (F × F) :=
  {v | v ≠ 0 ∧ a * v.1 ^ 2 + b * v.1 * v.2 + c * v.2 ^ 2 = 0}

theorem ncard_compl_zero : ({0}ᶜ : Set F).ncard = Nat.card F - 1 := ncard_compl_zero'

/-- A nonzero binary quadratic form over a finite field has at most `2 (q - 1)` nonzero zeros
in `F²`, i.e. at most two projective zeros. -/
theorem qZero_ncard_le (a b c : F) (h : ¬(a = 0 ∧ b = 0 ∧ c = 0)) :
    (qZero a b c).ncard ≤ 2 * (Nat.card F - 1) := by
  classical
  set R : Set F := {t | a * t ^ 2 + b * t + c = 0} with hR
  have hsub1 : {v : F × F | v ∈ qZero a b c ∧ v.2 ≠ 0} ⊆
      (fun p : F × F => (p.1 * p.2, p.2)) '' (R ×ˢ ({0}ᶜ : Set F)) := by
    rintro ⟨x, y⟩ ⟨⟨-, hq⟩, hy⟩
    simp only at hq hy
    refine ⟨(x / y, y), ⟨?_, hy⟩, by simp [div_mul_cancel₀ _ hy]⟩
    show a * (x / y) ^ 2 + b * (x / y) + c = 0
    have : a * (x / y) ^ 2 + b * (x / y) + c = (a * x ^ 2 + b * x * y + c * y ^ 2) / y ^ 2 := by
      field_simp
    rw [this, hq, zero_div]
  have hsub2 : {v : F × F | v ∈ qZero a b c ∧ ¬ v.2 ≠ 0} ⊆
      (fun x : F => (x, (0 : F))) '' ({0}ᶜ : Set F) := by
    rintro ⟨x, y⟩ ⟨⟨hv, -⟩, hy⟩
    simp only [not_not] at hy
    subst hy
    refine ⟨x, fun hx => hv ?_, rfl⟩
    simp only [Set.mem_singleton_iff] at hx
    simp [hx]
  have hZ : qZero a b c ⊆ {v : F × F | v ∈ qZero a b c ∧ v.2 ≠ 0} ∪
      {v : F × F | v ∈ qZero a b c ∧ ¬ v.2 ≠ 0} := by
    intro v hv
    by_cases h2 : v.2 ≠ 0
    · exact Or.inl ⟨hv, h2⟩
    · exact Or.inr ⟨hv, h2⟩
  have hcomp := ncard_compl_zero (F := F)
  have hA : {v : F × F | v ∈ qZero a b c ∧ v.2 ≠ 0}.ncard ≤ R.ncard * (Nat.card F - 1) := by
    calc _ ≤ ((fun p : F × F => (p.1 * p.2, p.2)) '' (R ×ˢ ({0}ᶜ : Set F))).ncard :=
          Set.ncard_le_ncard hsub1 (Set.toFinite _)
      _ ≤ (R ×ˢ ({0}ᶜ : Set F)).ncard := Set.ncard_image_le (Set.toFinite _)
      _ = _ := by rw [Set.ncard_prod, hcomp]
  have hB : {v : F × F | v ∈ qZero a b c ∧ ¬ v.2 ≠ 0}.ncard ≤ Nat.card F - 1 := by
    calc _ ≤ ((fun x : F => (x, (0 : F))) '' ({0}ᶜ : Set F)).ncard :=
          Set.ncard_le_ncard hsub2 (Set.toFinite _)
      _ ≤ _ := by rw [← hcomp]; exact Set.ncard_image_le (Set.toFinite _)
  by_cases ha : a = 0
  · -- degenerate: at most one root, and the `y = 0` line may contribute
    have hbc : ¬(b = 0 ∧ c = 0) := fun ⟨hb, hc⟩ => h ⟨ha, hb, hc⟩
    have hR1 : R.ncard ≤ 1 := by
      rw [Set.ncard_le_one_iff_subsingleton]
      intro t1 ht1 t2 ht2
      simp only [hR, Set.mem_ofPred_eq, ha, zero_mul, zero_add] at ht1 ht2
      by_cases hb : b = 0
      · exact absurd ⟨hb, by simpa [hb] using ht1⟩ hbc
      · have : b * (t1 - t2) = 0 := by linear_combination ht1 - ht2
        rcases mul_eq_zero.mp this with h0 | h0
        · exact absurd h0 hb
        · exact sub_eq_zero.mp h0
    calc (qZero a b c).ncard
        ≤ ({v : F × F | v ∈ qZero a b c ∧ v.2 ≠ 0} ∪
            {v : F × F | v ∈ qZero a b c ∧ ¬ v.2 ≠ 0}).ncard :=
          Set.ncard_le_ncard hZ (Set.toFinite _)
      _ ≤ _ + _ := Set.ncard_union_le _ _
      _ ≤ 1 * (Nat.card F - 1) + (Nat.card F - 1) :=
          add_le_add (hA.trans (Nat.mul_le_mul_right _ hR1)) hB
      _ = _ := by ring
  · -- `y = 0` contributes nothing; at most two roots
    have hB0 : {v : F × F | v ∈ qZero a b c ∧ ¬ v.2 ≠ 0} = ∅ := by
      ext ⟨x, y⟩
      simp only [Set.mem_ofPred_eq, Set.mem_empty_iff_false, iff_false, not_and, not_not]
      rintro ⟨hv, hq⟩ hy
      subst hy
      simp only [mul_zero, add_zero, zero_pow two_ne_zero] at hq
      have hx : x ≠ 0 := fun hx => hv (by simp [hx])
      exact mul_ne_zero ha (pow_ne_zero 2 hx) hq
    have hR2 : R.ncard ≤ 2 := by
      obtain ⟨p, hp⟩ : ∃ p : Polynomial F, p = Polynomial.C a * Polynomial.X ^ 2 +
          Polynomial.C b * Polynomial.X + Polynomial.C c := ⟨_, rfl⟩
      have hp0 : p ≠ 0 := by
        intro h0
        have := congrArg (fun q => Polynomial.coeff q 2) h0
        simp [hp] at this
        exact ha this
      have hdeg : p.natDegree ≤ 2 := by
        rw [hp]; exact Polynomial.natDegree_quadratic_le
      have hRsub : R ⊆ ↑p.roots.toFinset := by
        intro t ht
        rw [Finset.mem_coe, Multiset.mem_toFinset, Polynomial.mem_roots hp0,
          Polynomial.IsRoot.def]
        have ht' : a * t ^ 2 + b * t + c = 0 := ht
        simpa [hp] using ht'
      calc R.ncard ≤ (↑p.roots.toFinset : Set F).ncard := Set.ncard_le_ncard hRsub (Set.toFinite _)
        _ = p.roots.toFinset.card := Set.ncard_coe_finset _
        _ ≤ Multiset.card p.roots := Multiset.toFinset_card_le _
        _ ≤ p.natDegree := Polynomial.card_roots' p
        _ ≤ 2 := hdeg
    calc (qZero a b c).ncard
        ≤ ({v : F × F | v ∈ qZero a b c ∧ v.2 ≠ 0} ∪
            {v : F × F | v ∈ qZero a b c ∧ ¬ v.2 ≠ 0}).ncard :=
          Set.ncard_le_ncard hZ (Set.toFinite _)
      _ ≤ _ + _ := Set.ncard_union_le _ _
      _ ≤ 2 * (Nat.card F - 1) + 0 := by
          refine add_le_add (hA.trans (Nat.mul_le_mul_right _ hR2)) ?_
          rw [hB0]; simp
      _ = _ := by ring

end Quadratic


section Three

variable {L M : Type*} [Field L] [Field M] [Algebra L M] [Finite L] [Module.Finite L M]

/-- The `q`-power Frobenius of `M` as an `L`-algebra automorphism. -/
theorem exists_frob : ∃ σ : M ≃ₐ[L] M, ∀ x, σ x = x ^ Nat.card L := by
  have := Fintype.ofFinite L
  refine ⟨FiniteField.frobeniusAlgEquivOfAlgebraic L M, fun x => ?_⟩
  rw [FiniteField.coe_frobeniusAlgEquivOfAlgebraic, Nat.card_eq_fintype_card]

/-- For `[M : L] = 3`: `N(x) = x · x^q · x^(q²)`. -/
theorem algebraMap_norm_three (hn : Module.finrank L M = 3) (σ : M ≃ₐ[L] M)
    (hσ : ∀ x, σ x = x ^ Nat.card L) (x : M) :
    algebraMap L M (Algebra.norm L x) = x * σ x * σ (σ x) := by
  have : Finite M := Module.finite_of_finite L
  rw [FiniteField.algebraMap_norm_eq_prod_pow, hn]
  simp only [Finset.prod_range_succ, Finset.range_zero, Finset.prod_empty, hσ]
  ring

/-- The quadratic form `u ↦ Tr_{M/L}(c · σu · σ²u)` (with `σ` the `q`-Frobenius). -/
noncomputable def qf (σ : M ≃ₐ[L] M) (c : M) (u : M) : L :=
  Algebra.trace L M (c * σ u * σ (σ u))

omit [Finite L] [Module.Finite L M] in
theorem qf_comb (σ : M ≃ₐ[L] M) (c e1 e2 : M) (x y : L) :
    qf σ c (x • e1 + y • e2) =
      x * x * qf σ c e1
        + x * y * Algebra.trace L M (c * (σ e1 * σ (σ e2) + σ e2 * σ (σ e1)))
        + y * y * qf σ c e2 := by
  have e : c * σ (x • e1 + y • e2) * σ (σ (x • e1 + y • e2)) =
      (x * x) • (c * σ e1 * σ (σ e1))
        + (x * y) • (c * (σ e1 * σ (σ e2) + σ e2 * σ (σ e1)))
        + (y * y) • (c * σ e2 * σ (σ e2)) := by
    simp only [Algebra.smul_def, map_add, map_mul, AlgEquiv.commutes]
    ring
  unfold qf
  rw [e, map_add, map_add, map_smul, map_smul, map_smul]
  simp only [smul_eq_mul]

/-- For `u ≠ 0`: `N(u) · Tr(c u⁻¹) = Tr(c σu σ²u)`, with `N(u) ≠ 0`. -/
theorem norm_mul_trace_inv (hn : Module.finrank L M = 3) (σ : M ≃ₐ[L] M)
    (hσ : ∀ x, σ x = x ^ Nat.card L) (c : M) {u : M} (hu : u ≠ 0) :
    Algebra.norm L u ≠ 0 ∧
      Algebra.norm L u * Algebra.trace L M (c * u⁻¹) = qf σ c u := by
  have hN := algebraMap_norm_three hn σ hσ u
  have hσu : σ u ≠ 0 := by simpa using hu
  have hσσu : σ (σ u) ≠ 0 := by simpa using hu
  refine ⟨?_, ?_⟩
  · intro h0
    rw [h0, map_zero] at hN
    exact mul_ne_zero (mul_ne_zero hu hσu) hσσu hN.symm
  · have h1 : algebraMap L M (Algebra.norm L u) * (c * u⁻¹) = c * σ u * σ (σ u) := by
      rw [hN]; field_simp
    unfold qf
    rw [← h1, ← Algebra.smul_def, map_smul, smul_eq_mul]

/-- Proposition 3(b): for `[M : L] = 3` the `u`-count lies in
`{0, q - 1, 2 (q - 1), q² - 1}`. -/
theorem uCount_of_finrank_three (hn : Module.finrank L M = 3) (c : M) :
    Nat.card (uSet L c) = 0 ∨ Nat.card (uSet L c) = Nat.card L - 1 ∨
      Nat.card (uSet L c) = 2 * (Nat.card L - 1) ∨
      Nat.card (uSet L c) = Nat.card L ^ 2 - 1 := by
  have : Finite M := Module.finite_of_finite L
  obtain ⟨σ, hσ⟩ := exists_frob (L := L) (M := M)
  have hV : Module.finrank L (LinearMap.ker (Algebra.trace L M)) = 2 := by
    have h1 : Module.finrank L (LinearMap.range (Algebra.trace L M)) = 1 := by
      have : LinearMap.range (Algebra.trace L M) = ⊤ :=
        LinearMap.range_eq_top.mpr (Algebra.trace_surjective L M)
      rw [this, finrank_top, Module.finrank_self]
    have h2 := LinearMap.finrank_range_add_finrank_ker (Algebra.trace L M)
    omega
  let b := Module.finBasisOfFinrankEq L (LinearMap.ker (Algebra.trace L M)) hV
  let e1 : M := (b 0 : M)
  let e2 : M := (b 1 : M)
  let A : L := qf σ c e1
  let B : L := Algebra.trace L M (c * (σ e1 * σ (σ e2) + σ e2 * σ (σ e1)))
  let C : L := qf σ c e2
  let f : L × L → M := fun v => ((b.equivFun.symm ![v.1, v.2] :
    LinearMap.ker (Algebra.trace L M)) : M)
  have hf : ∀ v, f v = v.1 • e1 + v.2 • e2 := by
    intro v
    simp [f, e1, e2, Module.Basis.equivFun_symm_apply, Fin.sum_univ_two]
  have hf_mem : ∀ v, Algebra.trace L M (f v) = 0 := fun v =>
    (b.equivFun.symm ![v.1, v.2]).2
  have hf_surj : ∀ u : M, Algebra.trace L M u = 0 → ∃ v, f v = u := by
    intro u hu
    refine ⟨(b.equivFun ⟨u, hu⟩ 0, b.equivFun ⟨u, hu⟩ 1), ?_⟩
    have : (![b.equivFun ⟨u, hu⟩ 0, b.equivFun ⟨u, hu⟩ 1] : Fin 2 → L) =
        b.equivFun ⟨u, hu⟩ := by
      funext i; fin_cases i <;> rfl
    simp only [f, this, LinearEquiv.symm_apply_apply]
  have hf_inj : Function.Injective f := by
    intro v v' h
    have h1 := b.equivFun.symm.injective (Subtype.val_injective h)
    have h2 := congrFun h1 0
    have h3 := congrFun h1 1
    exact Prod.ext (by simpa using h2) (by simpa using h3)
  have hf0 : f 0 = 0 := by
    rw [hf]; simp
  have hq : ∀ v, qf σ c (f v) = v.1 * v.1 * A + v.1 * v.2 * B + v.2 * v.2 * C := by
    intro v; rw [hf, qf_comb]
  have hset : uSet L c = f '' qZero A B C := by
    ext u
    constructor
    · rintro ⟨hu0, hT, hcu⟩
      obtain ⟨v, rfl⟩ := hf_surj u hT
      have hv0 : v ≠ 0 := fun h => hu0 (by rw [h, hf0])
      obtain ⟨-, hnt⟩ := norm_mul_trace_inv hn σ hσ c hu0
      rw [hcu, mul_zero, hq] at hnt
      exact ⟨v, ⟨hv0, by linear_combination hnt.symm⟩, rfl⟩
    · rintro ⟨v, ⟨hv0, hz⟩, rfl⟩
      have hu0 : f v ≠ 0 := fun h => hv0 (hf_inj (h.trans hf0.symm))
      refine ⟨hu0, hf_mem v, ?_⟩
      obtain ⟨hN0, hnt⟩ := norm_mul_trace_inv hn σ hσ c hu0
      rw [hq] at hnt
      have : A * v.1 ^ 2 + B * v.1 * v.2 + C * v.2 ^ 2 = 0 := hz
      have h2 : Algebra.norm L (f v) * Algebra.trace L M (c * (f v)⁻¹) = 0 := by
        rw [hnt]; linear_combination this
      exact (mul_eq_zero.mp h2).resolve_left hN0
  have hN : Nat.card (uSet L c) = (qZero A B C).ncard := by
    rw [Nat.card_coe_set_eq, hset, Set.ncard_image_of_injective _ hf_inj]
  rw [hN]
  by_cases hABC : A = 0 ∧ B = 0 ∧ C = 0
  · right; right; right
    obtain ⟨h1, h2, h3⟩ := hABC
    have : qZero A B C = ({0}ᶜ : Set (L × L)) := by
      ext v; simp [qZero, h1, h2, h3]
    rw [this, ncard_compl_zero', Nat.card_prod, sq]
  · have hle := qZero_ncard_le A B C hABC
    have hdvd := uCount_dvd (L := L) c
    rw [hN] at hdvd
    obtain ⟨k, hk⟩ := hdvd
    rw [hk] at hle ⊢
    have hq1 : 0 < Nat.card L - 1 := by
      have := Finite.one_lt_card (α := L); omega
    have hk2 : k ≤ 2 := by
      by_contra hk3
      have : (Nat.card L - 1) * 3 ≤ (Nat.card L - 1) * k :=
        Nat.mul_le_mul_left _ (by omega)
      omega
    interval_cases k
    · left; simp
    · right; left; simp
    · right; right; left; ring
    
end Three

end LiftDenseUCount
