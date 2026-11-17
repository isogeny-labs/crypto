import Mathlib
import LiftDense

/-!
# Subgroup traps and the `2E` trace condition for lift-dense factor bases

Lean companion to `research/subgroup_trap_20261107/` (statements T1 and T2 of
`PROTOCOL.md`, including Amendment 1, and the P3 mechanism of `README.md`).

* T1a (`LiftDenseTrap.mem_range_of_quadratic`): over a field extension `K/F` of
  odd degree, a root in `K` of a monic quadratic over `F` lies in `F`.
* T1 general (`LiftDenseTrap.finrank_adjoin_y_le_two`): if a point of a Weierstrass
  curve defined over `F` has `x ∈ F`, then `[F(y) : F] ≤ 2`.
* T1b (`LiftDenseTrap.sum_mem_range`, `LiftDenseTrap.trap_sum_mem_range`,
  `LiftDenseTrap.trap_no_decomposition`): the image of `W(F) → W(K)` is an
  additive subgroup; for `[K:F]` odd every point of `W(K)` with `x ∈ F` lies in
  it, so every integer combination of such points does, and a target outside it
  has no decomposition.
* T2 easy direction (`LiftDenseTrap.trace_x_double`, `LiftDenseTrap.trace_x_of_two_nsmul`):
  on `y² + xy = x³ + a x² + b` over a finite field of characteristic 2, every
  nonzero point of `2E` has `Tr(x) = Tr(a)`; the `x`-coordinate of `2Q` is the
  `x`-coordinate Mathlib's group law produces.
* P3 mechanism, conditional form (`LiftDenseTrap.p3_no_decomposition`).
-/

namespace LiftDenseTrap

open Polynomial Module

/-! ## T1a: the parity lemma -/

section Parity

variable {F K : Type*} [Field F] [Field K] [Algebra F K]

/-- A root of a monic quadratic over `F` has degree at most 2 over `F`. -/
theorem natDegree_minpoly_le_two (b c : F) (y : K)
    (hy : y ^ 2 + algebraMap F K b * y + algebraMap F K c = 0) :
    IsIntegral F y ∧ (minpoly F y).natDegree ≤ 2 := by
  set p : F[X] := X ^ 2 + C b * X + C c with hp_def
  have hmonic : p.Monic := by rw [hp_def]; monicity!
  have hp : aeval y p = 0 := by
    simp only [hp_def, map_add, map_mul, map_pow, aeval_X, aeval_C]
    exact hy
  have hint : IsIntegral F y := ⟨p, hmonic, by rwa [← aeval_def]⟩
  refine ⟨hint, ?_⟩
  have hdvd : minpoly F y ∣ p := minpoly.dvd F y hp
  have hle : p.natDegree ≤ 2 := by rw [hp_def]; compute_degree
  exact (natDegree_le_of_dvd hdvd hmonic.ne_zero).trans hle

/-- **T1a (parity lemma).** Let `K/F` be a finite extension of odd degree. If
`y ∈ K` is a root of the monic quadratic `X² + bX + c` with `b, c ∈ F`, then
`y ∈ F`. -/
theorem mem_range_of_quadratic [FiniteDimensional F K] (hodd : Odd (finrank F K))
    (b c : F) (y : K) (hy : y ^ 2 + algebraMap F K b * y + algebraMap F K c = 0) :
    y ∈ (algebraMap F K).range := by
  obtain ⟨hint, hle⟩ := natDegree_minpoly_le_two b c y hy
  have hpos := minpoly.natDegree_pos hint
  have hdvd := minpoly.degree_dvd hint
  rw [← minpoly.natDegree_eq_one_iff]
  by_contra hne
  have h2 : (minpoly F y).natDegree = 2 := by omega
  rw [h2] at hdvd
  exact (Nat.not_even_iff_odd.mpr hodd) (even_iff_two_dvd.mpr hdvd)

/-- T1a for the binary-curve form: if `[K:F]` is odd and `y² + x y = c` with
`x, c ∈ F`, then `y ∈ F`. -/
theorem mem_range_of_artin_schreier [FiniteDimensional F K] (hodd : Odd (finrank F K))
    (x c : F) (y : K) (hy : y ^ 2 + algebraMap F K x * y = algebraMap F K c) :
    y ∈ (algebraMap F K).range :=
  mem_range_of_quadratic hodd x (-c) y (by rw [map_neg, hy]; ring)

end Parity

/-! ## T1b: the image of `W(F)` in `W(K)` is a subgroup, and the trap -/

section Trap

open WeierstrassCurve WeierstrassCurve.Affine

variable {R F K : Type*} [Field R] [Field F] [Field K] [Algebra R F] [Algebra R K]
  [Algebra F K] [IsScalarTower R F K]

/-- On a Weierstrass curve with coefficients in `R ⊆ F ⊆ K`, the equation of a
point `(algebraMap x₀, y)` is a monic quadratic in `y` with coefficients in `F`. -/
theorem quadratic_of_equation (W : Affine R) (x₀ : F) (y : K)
    (h : (W⁄K).Equation (algebraMap F K x₀) y) :
    y ^ 2 + algebraMap F K (algebraMap R F W.a₁ * x₀ + algebraMap R F W.a₃) * y
      + algebraMap F K (-(x₀ ^ 3 + algebraMap R F W.a₂ * x₀ ^ 2
          + algebraMap R F W.a₄ * x₀ + algebraMap R F W.a₆)) = 0 := by
  rw [equation_iff] at h
  simp only [WeierstrassCurve.baseChange, WeierstrassCurve.map_a₁, WeierstrassCurve.map_a₂,
    WeierstrassCurve.map_a₃, WeierstrassCurve.map_a₄, WeierstrassCurve.map_a₆,
    IsScalarTower.algebraMap_apply R F K] at h
  simp only [map_add, map_mul, map_neg, map_pow]
  linear_combination h

/-- **T1, general degree.** If a point of a Weierstrass curve defined over `R ⊆ F`
has abscissa in `F`, then its ordinate `y` has degree at most 2 over `F`, that
is, `y` lies in a quadratic (or trivial) extension of `F` inside `K`. -/
theorem finrank_adjoin_y_le_two (W : Affine R) (x₀ : F) (y : K)
    (h : (W⁄K).Equation (algebraMap F K x₀) y) :
    finrank F (IntermediateField.adjoin F {y}) ≤ 2 := by
  obtain ⟨hint, hle⟩ := natDegree_minpoly_le_two _ _ y (quadratic_of_equation W x₀ y h)
  rw [IntermediateField.adjoin.finrank hint]
  exact hle

variable [DecidableEq F] [DecidableEq K]

omit [Algebra F K] [IsScalarTower R F K] in
/-- **T1b, closure.** For any field homomorphism `f : F →ₐ[S] K`, every integer
combination of points of `W(K)` that lie in the image of `W(F)` lies in that
image. (`(Point.map f).range` is an `AddSubgroup`.) -/
theorem sum_mem_range {S : Type*} [CommRing S] [Algebra R S] [Algebra S F]
    [IsScalarTower R S F] [Algebra S K] [IsScalarTower R S K] (W : Affine R)
    (f : F →ₐ[S] K) {ι : Type*} (s : Finset ι) (n : ι → ℤ) (P : ι → (W⁄K).Point)
    (hP : ∀ i ∈ s, P i ∈ (Point.map (W' := W) f).range) :
    ∑ i ∈ s, n i • P i ∈ (Point.map (W' := W) f).range :=
  AddSubgroup.sum_mem _ fun i hi => AddSubgroup.zsmul_mem _ (hP i hi) _

omit [Algebra F K] [IsScalarTower R F K] in
/-- **T1b, no decomposition.** A target outside the image of `W(F)` is not an
integer combination of points in that image. -/
theorem no_decomposition {S : Type*} [CommRing S] [Algebra R S] [Algebra S F]
    [IsScalarTower R S F] [Algebra S K] [IsScalarTower R S K] (W : Affine R)
    (f : F →ₐ[S] K) {T : (W⁄K).Point} (hT : T ∉ (Point.map (W' := W) f).range)
    {ι : Type*} (s : Finset ι) (n : ι → ℤ) (P : ι → (W⁄K).Point)
    (hP : ∀ i ∈ s, P i ∈ (Point.map (W' := W) f).range) :
    ∑ i ∈ s, n i • P i ≠ T := fun h => hT (h ▸ sum_mem_range W f s n P hP)

/-- **T1 (odd degree).** Let `W` be a Weierstrass curve with coefficients in
`R`, and `R ⊆ F ⊆ K` with `[K:F]` finite and odd. Every point of `W(K)` whose
abscissa lies in `F` is the image of a point of `W(F)` under base change. -/
theorem mem_range_of_x_mem [FiniteDimensional F K] (hodd : Odd (finrank F K))
    (W : Affine R) {x y : K} (h : (W⁄K).Nonsingular x y)
    (hx : x ∈ (algebraMap F K).range) :
    Point.some x y h ∈ (Point.baseChange (W' := W) F K).range := by
  obtain ⟨x₀, rfl⟩ := hx
  obtain ⟨y₀, rfl⟩ := mem_range_of_quadratic hodd _ _ y (quadratic_of_equation W x₀ y h.1)
  have h₀ : (W⁄F).Nonsingular x₀ y₀ :=
    (W.baseChange_nonsingular (Algebra.ofId F K).injective x₀ y₀).mp h
  exact ⟨Point.some x₀ y₀ h₀, rfl⟩

/-- **T1 trap (odd degree), closure form.** If `[K:F]` is odd, the curve is
defined over `R ⊆ F`, and every factor-base point `P i` is an affine point with
abscissa in `F` (or the point at infinity), then every integer combination of
them lies in the image of `W(F)`. -/
theorem trap_sum_mem_range [FiniteDimensional F K] (hodd : Odd (finrank F K))
    (W : Affine R) {ι : Type*} (s : Finset ι) (n : ι → ℤ) (P : ι → (W⁄K).Point)
    (hP : ∀ i ∈ s, P i = 0 ∨ ∃ x y h, P i = Point.some x y h ∧ x ∈ (algebraMap F K).range) :
    ∑ i ∈ s, n i • P i ∈ (Point.baseChange (W' := W) F K).range := by
  refine sum_mem_range W (Algebra.ofId F K) s n P fun i hi => ?_
  rcases hP i hi with h0 | ⟨x, y, h, hPi, hx⟩
  · rw [h0]; exact AddSubgroup.zero_mem _
  · rw [hPi]; exact mem_range_of_x_mem hodd W h hx

/-- **T1 trap (odd degree), no decomposition.** Under the hypotheses of
`trap_sum_mem_range`, a target outside the image of `W(F)` has no
decomposition over the factor base, with any integer coefficients. -/
theorem trap_no_decomposition [FiniteDimensional F K] (hodd : Odd (finrank F K))
    (W : Affine R) {T : (W⁄K).Point} (hT : T ∉ (Point.baseChange (W' := W) F K).range)
    {ι : Type*} (s : Finset ι) (n : ι → ℤ) (P : ι → (W⁄K).Point)
    (hP : ∀ i ∈ s, P i = 0 ∨ ∃ x y h, P i = Point.some x y h ∧ x ∈ (algebraMap F K).range) :
    ∑ i ∈ s, n i • P i ≠ T := fun h => hT (h ▸ trap_sum_mem_range hodd W s n P hP)

/-! ### T1, even degree: the quadratic subfield `K* = K₂ ∩ M` -/

omit [DecidableEq F] [DecidableEq K] in
/-- In a finite extension of finite fields `K/F`, an element of degree at most 2
over `F` lies in every intermediate field `L` with `[L:F] = 2`. (So the
quadratic subfield is the unique home of all such elements.) -/
theorem mem_of_finrank_adjoin_le_two [Finite K] [FiniteDimensional F K]
    (L : IntermediateField F K) (hL : finrank F L = 2) (y : K)
    (hy : finrank F (IntermediateField.adjoin F {y}) ≤ 2) : y ∈ L := by
  classical
  have : Finite F := Finite.of_injective _ (algebraMap F K).injective
  set q := Nat.card F with hq
  have hq1 : 1 < q := Finite.one_lt_card
  have hfix : ∀ (E : IntermediateField F K) (z : E), z ^ (q ^ finrank F E) = z := by
    intro E z
    have := Fintype.ofFinite E
    rw [hq, ← Module.natCard_eq_pow_finrank, ← Fintype.card_eq_nat_card]
    exact FiniteField.pow_card z
  have hyq : y ^ (q ^ 2) = y := by
    have hyE : y ∈ IntermediateField.adjoin F {y} := IntermediateField.mem_adjoin_simple_self F y
    have h1 := congrArg Subtype.val (hfix _ ⟨y, hyE⟩)
    simp only [IntermediateField.coe_pow] at h1
    have hpos : 0 < finrank F (IntermediateField.adjoin F {y}) := Module.finrank_pos
    rcases (by omega : finrank F (IntermediateField.adjoin F {y}) = 1 ∨
        finrank F (IntermediateField.adjoin F {y}) = 2) with hd | hd <;> rw [hd] at h1
    · rw [pow_one] at h1; rw [sq, pow_mul, h1, h1]
    · exact h1
  have hL' : ∀ z : L, (z : K) ^ (q ^ 2) = z := by
    intro z
    have h1 := congrArg Subtype.val (hfix L z)
    rw [hL] at h1
    simpa only [IntermediateField.coe_pow] using h1
  by_contra hyL
  have hq2 : 1 < q ^ 2 := Nat.one_lt_pow two_ne_zero hq1
  set p : K[X] := X ^ (q ^ 2) - X with hp_def
  have hdeg : p.natDegree = q ^ 2 := FiniteField.X_pow_card_sub_X_natDegree_eq K hq2
  have hp0 : p ≠ 0 := FiniteField.X_pow_card_sub_X_ne_zero K hq2
  have hroot : ∀ z : K, z ^ (q ^ 2) = z → z ∈ p.roots.toFinset := by
    intro z hz
    rw [Multiset.mem_toFinset, mem_roots hp0, IsRoot.def, hp_def, eval_sub, eval_pow, eval_X,
      hz, sub_self]
  have := Fintype.ofFinite L
  set T : Finset K := insert y ((Finset.univ : Finset L).map (Function.Embedding.subtype _))
  have hT : T ⊆ p.roots.toFinset := by
    intro z hz
    rcases Finset.mem_insert.mp hz with rfl | hz
    · exact hroot _ hyq
    · obtain ⟨w, -, rfl⟩ := Finset.mem_map.mp hz
      exact hroot _ (hL' w)
  have hcardT : T.card = q ^ 2 + 1 := by
    rw [Finset.card_insert_of_notMem, Finset.card_map, Finset.card_univ,
      Fintype.card_eq_nat_card, Module.natCard_eq_pow_finrank (K := F), hL]
    simp only [Finset.mem_map, Finset.mem_univ, Function.Embedding.coe_subtype, true_and,
      not_exists]
    intro w hw
    exact hyL (hw ▸ w.2)
  have := (Finset.card_le_card hT).trans ((Multiset.toFinset_card_le _).trans (card_roots' p))
  omega

omit [DecidableEq F] in
/-- **T1 (even degree).** Let `W` be a Weierstrass curve over a finite field
`F`, `K/F` a finite extension, and `L` an intermediate field with `[L:F] = 2`
(the quadratic extension `K₂` of `F`, which lies in `K` exactly when `[K:F]` is
even). Every point of `W(K)` with abscissa in `F` is the image of a point of
`W(L)`. -/
theorem mem_range_of_x_mem_quadratic [Finite K] [FiniteDimensional F K]
    (W : Affine F) (L : IntermediateField F K) [DecidableEq L] (hL : finrank F L = 2) {x y : K}
    (h : (W⁄K).Nonsingular x y) (hx : x ∈ (algebraMap F K).range) :
    Point.some x y h ∈ (Point.map (W' := W) L.val).range := by
  obtain ⟨x₀, hx₀⟩ := hx
  have hyL : y ∈ L := mem_of_finrank_adjoin_le_two L hL y
    (finrank_adjoin_y_le_two W x₀ y (hx₀ ▸ h.1))
  have hxL : x ∈ L := hx₀ ▸ L.algebraMap_mem x₀
  obtain ⟨x', rfl⟩ : ∃ x' : L, L.val x' = x := ⟨⟨x, hxL⟩, rfl⟩
  obtain ⟨y', rfl⟩ : ∃ y' : L, L.val y' = y := ⟨⟨y, hyL⟩, rfl⟩
  have h₀ : (W⁄L).Nonsingular x' y' := (W.baseChange_nonsingular L.val.injective x' y').mp h
  exact ⟨Point.some x' y' h₀, rfl⟩

omit [DecidableEq F] in
/-- **T1 trap (even degree), closure and no decomposition.** Under the
hypotheses of `mem_range_of_x_mem_quadratic`, every integer combination of
factor-base points (each `O` or with abscissa in `F`) lies in the image of
`W(L)`, so a target outside that image has no decomposition. -/
theorem trap_no_decomposition_quadratic [Finite K] [FiniteDimensional F K]
    (W : Affine F) (L : IntermediateField F K) [DecidableEq L] (hL : finrank F L = 2)
    {T : (W⁄K).Point} (hT : T ∉ (Point.map (W' := W) L.val).range)
    {ι : Type*} (s : Finset ι) (n : ι → ℤ) (P : ι → (W⁄K).Point)
    (hP : ∀ i ∈ s, P i = 0 ∨ ∃ x y h, P i = Point.some x y h ∧ x ∈ (algebraMap F K).range) :
    ∑ i ∈ s, n i • P i ≠ T := by
  refine no_decomposition W L.val hT s n P fun i hi => ?_
  rcases hP i hi with h0 | ⟨x, y, h, hPi, hx⟩
  · rw [h0]; exact AddSubgroup.zero_mem _
  · rw [hPi]; exact mem_range_of_x_mem_quadratic W L hL h hx

end Trap

/-! ## T2: the trace of the abscissa on `2E` -/

section Halving

open WeierstrassCurve WeierstrassCurve.Affine

variable {M : Type*} [Field M] [Finite M] [Algebra (ZMod 2) M]

local notation "Tr" => Algebra.trace (ZMod 2) M

/-- **T2, explicit formula.** If `(x, y)` lies on `y² + xy = x³ + a x² + b` and
`x ≠ 0`, then `Tr(x² + b/x²) = Tr(a)`. -/
theorem trace_x_double (a b x y : M) (hx : x ≠ 0)
    (h : y ^ 2 + x * y = x ^ 3 + a * x ^ 2 + b) :
    Tr (x ^ 2 + b / x ^ 2) = Tr a := by
  have hl := (LiftDense.lifts_iff a b x hx).mp ⟨y, h⟩
  rw [map_add, map_add] at hl
  rw [map_add, LiftDense.trace_sq]
  have h2 : (2 : ZMod 2) = 0 := rfl
  linear_combination hl - Tr a * h2

/-- The ordinary binary curve `y² + xy = x³ + a x² + b` as a Mathlib
Weierstrass curve: `a₁ = 1`, `a₂ = a`, `a₃ = a₄ = 0`, `a₆ = b`. -/
def binCurve (a b : M) : Affine M := ⟨1, a, 0, 0, b⟩

omit [Finite M] [Algebra (ZMod 2) M] in
theorem binCurve_equation (a b x y : M) :
    (binCurve a b).Equation x y ↔ y ^ 2 + x * y = x ^ 3 + a * x ^ 2 + b := by
  rw [equation_iff]
  simp only [binCurve]
  constructor <;> intro h <;> linear_combination h

omit [Finite M] in
/-- The doubling formula of Mathlib's group law on `binCurve a b`: for an affine
point `Q = (x, y)` with `x ≠ 0`, `Q + Q` is an affine point with abscissa
`x² + b/x²`. -/
theorem double_x [DecidableEq M] (a b x y : M) (h : (binCurve a b).Nonsingular x y)
    (hx : x ≠ 0) :
    ∃ y' h', Point.some x y h + Point.some x y h = Point.some (x ^ 2 + b / x ^ 2) y' h' := by
  have := LiftDense.charTwo (M := M)
  have h2 : (2 : M) = 0 := CharTwo.two_eq_zero
  have hneg : (binCurve a b).negY x y = y + x := by
    simp only [negY, binCurve]; linear_combination (-y - x) * h2
  have hden : y - (binCurve a b).negY x y = x := by
    rw [hneg]; linear_combination (-x) * h2
  have hy : y ≠ (binCurve a b).negY x y := by
    intro e; apply hx; rw [← hden, ← e, sub_self]
  have heq := (binCurve_equation a b x y).mp h.1
  have hslope : (binCurve a b).slope x x y y = (x ^ 2 + y) / x := by
    rw [slope_of_Y_ne rfl hy, hden]
    simp only [binCurve]
    congr 1
    linear_combination (x ^ 2 + a * x - y) * h2
  have hX : (binCurve a b).addX x x ((binCurve a b).slope x x y y) = x ^ 2 + b / x ^ 2 := by
    rw [hslope, addX]
    simp only [binCurve]
    field_simp
    linear_combination heq + (x ^ 2 * y) * h2
  rw [Point.add_self_of_Y_ne hy]
  generalize_proofs hns
  revert hns
  rw [hX]
  intro hns
  exact ⟨_, hns, rfl⟩

/-- **T2, easy direction, on Mathlib's group law.** On the curve
`y² + xy = x³ + a x² + b` over a finite field of characteristic 2, every affine
point of the form `2 • Q` has `Tr(x) = Tr(a)`. Equivalently (see
`not_two_nsmul_of_trace_ne`), a point with `Tr(x) ≠ Tr(a)` is not in `2E(M)`. -/
theorem trace_x_of_two_nsmul [DecidableEq M] (a b : M) (Q : (binCurve a b).Point)
    {x y : M} {h : (binCurve a b).Nonsingular x y} (hQ : 2 • Q = Point.some x y h) :
    Tr x = Tr a := by
  have := LiftDense.charTwo (M := M)
  rw [two_nsmul] at hQ
  rcases Q with _ | ⟨x₀, y₀, h₀⟩
  · change (0 : (binCurve a b).Point) + 0 = _ at hQ
    rw [add_zero] at hQ
    exact (Point.some_ne_zero _ hQ.symm).elim
  · by_cases hx₀ : x₀ = 0
    · exfalso
      have hy : y₀ = (binCurve a b).negY x₀ y₀ := by
        simp only [negY, binCurve, hx₀, mul_zero, sub_zero, CharTwo.neg_eq]
      rw [Point.add_self_of_Y_eq hy] at hQ
      exact Point.some_ne_zero _ hQ.symm
    · obtain ⟨y', h', hd⟩ := double_x a b x₀ y₀ h₀ hx₀
      rw [hd, Point.some.injEq] at hQ
      rw [← hQ.1]
      exact trace_x_double a b x₀ y₀ hx₀ ((binCurve_equation a b x₀ y₀).mp h₀.1)

/-- Contrapositive of `trace_x_of_two_nsmul`: a point with `Tr(x) ≠ Tr(a)` is
not twice any point. -/
theorem not_two_nsmul_of_trace_ne [DecidableEq M] (a b : M) {x y : M}
    (h : (binCurve a b).Nonsingular x y) (ht : Tr x ≠ Tr a) (Q : (binCurve a b).Point) :
    2 • Q ≠ Point.some x y h := fun hQ => ht (trace_x_of_two_nsmul a b Q hQ)

/-- `2E` as an additive subgroup: the range of `Q ↦ 2 • Q`. -/
def twoE {G : Type*} [AddCommGroup G] : AddSubgroup G := (nsmulAddMonoidHom 2 : G →+ G).range

/-- **P3 mechanism, conditional form.** If every factor-base point lies in
`2E(M)`, then no integer combination of factor-base points equals an affine
target `R` with `Tr(x_R) ≠ Tr(a)`. The hypothesis "factor base ⊆ 2E" is the
hard direction of T2 (open here); it is a hypothesis, not a conclusion. -/
theorem p3_no_decomposition [DecidableEq M] (a b : M) {x y : M}
    (h : (binCurve a b).Nonsingular x y) (ht : Tr x ≠ Tr a)
    {ι : Type*} (s : Finset ι) (n : ι → ℤ) (P : ι → (binCurve a b).Point)
    (hP : ∀ i ∈ s, P i ∈ twoE) :
    ∑ i ∈ s, n i • P i ≠ Point.some x y h := by
  intro hsum
  have hmem : ∑ i ∈ s, n i • P i ∈ twoE :=
    AddSubgroup.sum_mem _ fun i hi => AddSubgroup.zsmul_mem _ (hP i hi) _
  obtain ⟨Q, hQ⟩ := hmem
  rw [hsum] at hQ
  exact not_two_nsmul_of_trace_ne a b h ht Q hQ

/-! ### T2, converse: halving -/

omit [Finite M] in
/-- On `binCurve a b`, every solution with `x ≠ 0` is a nonsingular point
(the `Y`-partial derivative is `x` in characteristic 2). -/
theorem binCurve_nonsingular (a b x y : M) (hx : x ≠ 0)
    (h : y ^ 2 + x * y = x ^ 3 + a * x ^ 2 + b) : (binCurve a b).Nonsingular x y := by
  have := LiftDense.charTwo (M := M)
  have h2 : (2 : M) = 0 := CharTwo.two_eq_zero
  rw [nonsingular_iff, binCurve_equation]
  refine ⟨h, Or.inr fun e => hx ?_⟩
  simp only [binCurve] at e
  linear_combination e - y * h2

/-- Every element of a finite field of characteristic 2 is a square. -/
theorem exists_sq_eq (t : M) : ∃ s : M, s ^ 2 = t := by
  obtain ⟨s, hs⟩ := (FiniteField.frobeniusAlgEquivOfAlgebraic (ZMod 2) M).surjective t
  rw [FiniteField.coe_frobeniusAlgEquivOfAlgebraic, ZMod.card] at hs
  exact ⟨s, hs⟩

/-- **T2, converse (halving).** On `y² + xy = x³ + a x² + b` with `b ≠ 0` over a
finite field of characteristic 2, every affine point with `Tr(x) = Tr(a)` is
`2 • Q` for some point `Q` of the curve. -/
theorem exists_half [DecidableEq M] (a b : M) (hb : b ≠ 0) {x y : M}
    (h : (binCurve a b).Nonsingular x y) (ht : Tr x = Tr a) :
    ∃ Q : (binCurve a b).Point, 2 • Q = Point.some x y h := by
  have := LiftDense.charTwo (M := M)
  have h2 : (2 : M) = 0 := CharTwo.two_eq_zero
  have h2z : (2 : ZMod 2) = 0 := rfl
  have heqP := (binCurve_equation a b x y).mp h.1
  -- a nonzero root `t` of `t² + x t = b`; then `s = √t` is the abscissa of a half
  obtain ⟨t, ht0, htq⟩ : ∃ t : M, t ≠ 0 ∧ t ^ 2 + x * t = b := by
    by_cases hx : x = 0
    · obtain ⟨t, htb⟩ := exists_sq_eq b
      refine ⟨t, ?_, by rw [hx, zero_mul, add_zero, htb]⟩
      rintro rfl
      exact hb (by rw [← htb]; ring)
    · have hl := (LiftDense.lifts_iff a b x hx).mp ⟨y, heqP⟩
      rw [map_add, map_add] at hl
      have hbx : Tr (b / x ^ 2) = 0 := by linear_combination hl - ht - Tr a * h2z
      obtain ⟨z, hz⟩ := (LiftDense.artin_schreier _).mpr hbx
      have hx2 : x ^ 2 ≠ 0 := pow_ne_zero 2 hx
      refine ⟨x * z, mul_ne_zero hx fun hz0 => ?_, ?_⟩
      · have : b / x ^ 2 = 0 := by rw [← hz, hz0]; ring
        rcases div_eq_zero_iff.mp this with h0 | h0
        · exact hb h0
        · exact hx2 h0
      · rw [show (x * z) ^ 2 + x * (x * z) = x ^ 2 * (z ^ 2 + z) by ring, hz]
        field_simp
  obtain ⟨s, hs⟩ := exists_sq_eq t
  have hs0 : s ≠ 0 := by rintro rfl; apply ht0; rw [← hs]; ring
  have hbt : b / s ^ 2 = t + x := by
    rw [hs, div_eq_iff ht0]; linear_combination -htq
  have hts : Tr s = Tr t := by rw [← hs, LiftDense.trace_sq]
  have hlift : Tr (s + a + b / s ^ 2) = 0 := by
    rw [hbt, map_add, map_add, map_add]
    linear_combination hts + ht + (Tr t + Tr a) * h2z
  obtain ⟨y₀, hy₀⟩ := (LiftDense.lifts_iff a b s hs0).mpr hlift
  have hQ := binCurve_nonsingular a b s y₀ hs0 hy₀
  obtain ⟨y', h', hd⟩ := double_x a b s y₀ hQ hs0
  have hxe : s ^ 2 + b / s ^ 2 = x := by rw [hbt, hs]; linear_combination t * h2
  subst hxe
  rcases (Point.X_eq_iff (h₁ := h') (h₂ := h)).mp rfl with e | e
  · exact ⟨Point.some s y₀ hQ, by rw [two_nsmul, hd, e]⟩
  · exact ⟨-Point.some s y₀ hQ, by rw [smul_neg, two_nsmul, hd, e, neg_neg]⟩

/-- **T2 (both directions).** For `b ≠ 0`, an affine point of
`y² + xy = x³ + a x² + b` over a finite field of characteristic 2 lies in
`2E(M)` if and only if `Tr(x) = Tr(a)`. -/
theorem some_mem_twoE_iff [DecidableEq M] (a b : M) (hb : b ≠ 0) {x y : M}
    (h : (binCurve a b).Nonsingular x y) :
    Point.some x y h ∈ (twoE : AddSubgroup (binCurve a b).Point) ↔ Tr x = Tr a := by
  simp only [twoE, AddMonoidHom.mem_range, nsmulAddMonoidHom_apply]
  exact ⟨fun ⟨Q, hQ⟩ => trace_x_of_two_nsmul a b Q hQ, exists_half a b hb h⟩

/-- **T2, subgroup form.** For `b ≠ 0`, `2E(M) = {O} ∪ {P : Tr(x_P) = Tr(a)}`;
in particular the right-hand side is an additive subgroup. -/
theorem mem_twoE_iff [DecidableEq M] (a b : M) (hb : b ≠ 0) (P : (binCurve a b).Point) :
    P ∈ (twoE : AddSubgroup (binCurve a b).Point) ↔
      P = 0 ∨ ∃ x y h, P = Point.some x y h ∧ Tr x = Tr a := by
  constructor
  · intro hP
    rcases P with _ | ⟨x, y, h⟩
    · exact Or.inl rfl
    · exact Or.inr ⟨x, y, h, rfl, (some_mem_twoE_iff a b hb h).mp hP⟩
  · rintro (rfl | ⟨x, y, h, rfl, ht⟩)
    · exact AddSubgroup.zero_mem _
    · exact (some_mem_twoE_iff a b hb h).mpr ht

/-- **P3 mechanism, unconditional form.** On `y² + xy = x³ + a x² + b` with
`b ≠ 0` over a finite field of characteristic 2: if every factor-base point is
`O` or has `Tr(x) = Tr(a)`, then no integer combination of factor-base points
equals an affine target `R` with `Tr(x_R) ≠ Tr(a)`. -/
theorem p3_no_decomposition' [DecidableEq M] (a b : M) (hb : b ≠ 0) {x y : M}
    (h : (binCurve a b).Nonsingular x y) (ht : Tr x ≠ Tr a)
    {ι : Type*} (s : Finset ι) (n : ι → ℤ) (P : ι → (binCurve a b).Point)
    (hP : ∀ i ∈ s, P i = 0 ∨ ∃ x y h, P i = Point.some x y h ∧ Tr x = Tr a) :
    ∑ i ∈ s, n i • P i ≠ Point.some x y h :=
  p3_no_decomposition a b h ht s n P fun i hi => (mem_twoE_iff a b hb (P i)).mpr (hP i hi)

end Halving

end LiftDenseTrap
