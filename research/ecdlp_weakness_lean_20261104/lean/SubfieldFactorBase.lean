import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.FieldTheory.IntermediateField.Adjoin.Basic
import Mathlib.FieldTheory.Minpoly.Field
import Mathlib.SetTheory.Cardinal.NatCard

/-!
# Subfield curves: the `x ∈ F_q` factor base never leaves `E(F_q)` in odd degree

Two corpus heuristics count decompositions over the factor base
`F = {P ∈ E(F_{q^n}) : x(P) ∈ F_q}`:

* `HEUR-SP-JV-ASSUMPTION1` (Joux–Vitse): under `#F ∈ Ω(q)`, the number of points of
  `E(F_{q^n})` that are sums of `n - 1` factor-base points is `Ω(q^{n-1}/(n-1)!)`;
* `HEUR-BK-GG-DECOMP-PROPORTION` (Galbraith–Gaudry): with `#F ~ q`, a random point is a
  sum of `n` factor-base points with probability about `1/n!`.

Neither statement excludes curves defined over the subfield `F_q`. For such a curve and
odd `n` this file proves that every factor-base point is the image of a point of `E(F_q)`
(`baseChange_range_of_x_mem`), so every finite sum of factor-base points lies in the
image of `E(F_q)` (`closure_factorBase_le_range`). The number of decomposable points is
therefore at most `#E(F_q)` (`card_decomposable_le`), whatever the number of summands.

Since `#E(F_q) ≥ q + 1 - 2√q` (Hasse), these curves satisfy the hypotheses `#F ∈ Ω(q)` and
`#F ~ q`: the factor base contains `E(F_q) \ {O}`. Yet the decomposable points number at
most `#E(F_q) ≤ 2q + 1`, against the claimed `Ω(q^{n-1}/(n-1)!)` for `n ≥ 3`, and a
random point of `E(F_{q^n})` (about `q^n` of them) decomposes with probability at most
about `2q / q^n`, against the claimed `1/n!`. The Hasse bound is cited, not formalised; the
upper bound `2q + 1` is proved here (`card_point_le`).

The counterexample family is the whole class of subfield curves with `n` odd; the
heuristics remain open for curves whose coefficients generate `F_{q^n}`.
-/

open Polynomial WeierstrassCurve WeierstrassCurve.Affine IntermediateField

namespace SubfieldFactorBase

variable {K L : Type*} [Field K] [Field L] [Algebra K L]

/-- In an extension of odd degree, a root of a monic quadratic over the base field lies in
the base field. -/
theorem mem_range_of_quadratic_of_odd [FiniteDimensional K L]
    (hodd : Odd (Module.finrank K L)) {y : L} (b c : K)
    (hy : y ^ 2 + algebraMap K L b * y + algebraMap K L c = 0) :
    y ∈ Set.range (algebraMap K L) := by
  have hint : IsIntegral K y := .of_finite K y
  -- the minimal polynomial divides `X^2 + (b X + c)`, so its degree is at most 2
  have hpmon : (X ^ 2 + (C b * X + C c) : K[X]).Monic :=
    monic_X_pow_add (by simpa using degree_linear_lt (a := b) (b := c))
  have hpdeg : (X ^ 2 + (C b * X + C c) : K[X]).natDegree ≤ 2 := by compute_degree
  have hroot : aeval y (X ^ 2 + (C b * X + C c) : K[X]) = 0 := by
    simp only [map_add, map_mul, map_pow, aeval_X, aeval_C]
    linear_combination hy
  have hdvd : minpoly K y ∣ (X ^ 2 + (C b * X + C c) : K[X]) := minpoly.dvd K y hroot
  have hle : (minpoly K y).natDegree ≤ 2 :=
    (natDegree_le_of_dvd hdvd hpmon.ne_zero).trans hpdeg
  -- its degree is `[K(y):K]`, which divides the odd `[L:K]`
  have hfr : Module.finrank K K⟮y⟯ = (minpoly K y).natDegree :=
    IntermediateField.adjoin.finrank hint
  have hdvdL : (minpoly K y).natDegree ∣ Module.finrank K L := by
    rw [← hfr]; exact ⟨_, (Module.finrank_mul_finrank K K⟮y⟯ L).symm⟩
  have hpos : 0 < (minpoly K y).natDegree := minpoly.natDegree_pos hint
  have hne2 : (minpoly K y).natDegree ≠ 2 := by
    intro h2; rw [h2] at hdvdL
    exact (Nat.not_even_iff_odd.mpr hodd) (even_iff_two_dvd.mpr hdvdL)
  have h1 : (minpoly K y).natDegree = 1 := by omega
  exact minpoly.mem_range_of_degree_eq_one K y
    (by rw [degree_eq_natDegree (minpoly.ne_zero hint), h1]; rfl)

variable [DecidableEq K] [DecidableEq L] (W : WeierstrassCurve K)

/-- The factor base of points of `W(L)` whose x-coordinate lies in `K` (with the identity). -/
def factorBase : Set (W⁄L).Point :=
  {P | P = 0 ∨ ∃ (x y : L) (h : (W⁄L).Nonsingular x y),
    P = .some x y h ∧ x ∈ Set.range (algebraMap K L)}

/-- In odd degree, every factor-base point comes from a point of `W(K)`. -/
theorem baseChange_range_of_x_mem [FiniteDimensional K L] (hodd : Odd (Module.finrank K L))
    {P : (W⁄L).Point} (hP : P ∈ factorBase (L := L) W) :
    P ∈ (Point.baseChange (W' := W) K L).range := by
  rcases hP with rfl | ⟨x, y, h, rfl, x₀, rfl⟩
  · exact AddSubgroup.zero_mem _
  · -- `y` is a root of `Y^2 + (a₁ x₀ + a₃) Y - (x₀^3 + a₂ x₀^2 + a₄ x₀ + a₆)` over `K`
    have heq : (W⁄L).Equation (algebraMap K L x₀) y := h.1
    rw [Affine.equation_iff] at heq
    have hq : y ^ 2 + algebraMap K L (W.a₁ * x₀ + W.a₃) * y +
        algebraMap K L (-(x₀ ^ 3 + W.a₂ * x₀ ^ 2 + W.a₄ * x₀ + W.a₆)) = 0 := by
      simp only [WeierstrassCurve.baseChange, WeierstrassCurve.map_a₁, WeierstrassCurve.map_a₂,
        WeierstrassCurve.map_a₃, WeierstrassCurve.map_a₄, WeierstrassCurve.map_a₆] at heq
      simp only [map_add, map_mul, map_neg, map_pow]
      linear_combination heq
    obtain ⟨y₀, rfl⟩ := mem_range_of_quadratic_of_odd hodd _ _ hq
    have h₀ : (W⁄K).Nonsingular x₀ y₀ :=
      (W.toAffine.baseChange_nonsingular (f := Algebra.ofId K L) (Algebra.ofId K L).injective x₀ y₀).mp h
    exact ⟨.some x₀ y₀ h₀, rfl⟩

/-- In odd degree, every finite sum (and difference) of factor-base points lies in the image
of `W(K)`. -/
theorem closure_factorBase_le_range [FiniteDimensional K L] (hodd : Odd (Module.finrank K L)) :
    AddSubgroup.closure (factorBase (L := L) W) ≤ (Point.baseChange (W' := W) K L).range :=
  (AddSubgroup.closure_le _).mpr fun _ hP => baseChange_range_of_x_mem W hodd hP

/-- **Counting.** The points of `W(L)` that are sums of factor-base points (any number of
summands, signs allowed) number at most `#W(K)`. -/
theorem card_decomposable_le [FiniteDimensional K L] [Finite (W⁄K).Point]
    (hodd : Odd (Module.finrank K L)) :
    Nat.card (AddSubgroup.closure (factorBase (L := L) W)) ≤ Nat.card (W⁄K).Point := by
  have hinj : Function.Injective (Point.baseChange (W' := W) K L) := Point.map_injective _
  let e := AddMonoidHom.ofInjective hinj
  have : Finite (Point.baseChange (W' := W) K L).range := Finite.of_equiv _ e.toEquiv
  calc Nat.card (AddSubgroup.closure (factorBase (L := L) W))
      ≤ Nat.card (Point.baseChange (W' := W) K L).range :=
        AddSubgroup.card_le_of_le (closure_factorBase_le_range W hodd)
    _ = Nat.card (W⁄K).Point := (Nat.card_congr e.toEquiv).symm

/-- **A trivial point count.** A Weierstrass curve over a finite field `K` has at most
`2 #K + 1` points: each `x` carries at most two `y` (roots of a monic quadratic), plus `O`. -/
theorem card_point_le [Fintype K] (V : Affine K) : Nat.card V.Point ≤ 2 * Fintype.card K + 1 := by
  classical
  let S : Finset (K × K) := Finset.univ.filter fun xy => V.Equation xy.1 xy.2
  have hfib : ∀ a ∈ S.image Prod.fst, (S.filter fun xy => xy.1 = a).card ≤ 2 := by
    intro a _
    obtain ⟨b, hb⟩ : ∃ b : K, b = V.a₁ * a + V.a₃ := ⟨_, rfl⟩
    obtain ⟨c, hc⟩ : ∃ c : K, c = -(a ^ 3 + V.a₂ * a ^ 2 + V.a₄ * a + V.a₆) := ⟨_, rfl⟩
    obtain ⟨p, hp⟩ : ∃ p : K[X], p = X ^ 2 + (C b * X + C c) := ⟨_, rfl⟩
    have hpmon : p.Monic := hp ▸ monic_X_pow_add (by simpa using degree_linear_lt (a := b) (b := c))
    have hpdeg : p.natDegree ≤ 2 := by rw [hp]; compute_degree
    have hinj : Set.InjOn Prod.snd ((S.filter fun xy => xy.1 = a : Finset (K × K)) : Set (K × K)) := by
      intro u hu v hv h
      simp only [Finset.coe_filter, Set.mem_ofPred_eq] at hu hv
      exact Prod.ext (hu.2.trans hv.2.symm) h
    rw [← Finset.card_image_of_injOn hinj]
    refine (card_le_degree_of_subset_roots ?_).trans hpdeg
    intro y hy
    obtain ⟨⟨x', y'⟩, hxy, rfl⟩ := Finset.mem_image.mp hy
    simp only [S, Finset.mem_filter, Finset.mem_univ, true_and] at hxy
    obtain ⟨he, rfl⟩ := hxy
    rw [mem_roots hpmon.ne_zero, IsRoot.def, hp]
    rw [Affine.equation_iff] at he
    simp only [eval_add, eval_mul, eval_pow, eval_X, eval_C, hb, hc]
    linear_combination he
  have hS : S.card ≤ 2 * Fintype.card K :=
    (Finset.card_le_mul_card_image S 2 hfib).trans
      (Nat.mul_le_mul_left 2 (Finset.card_le_univ _))
  have hsub : {xy : K × K | V.Nonsingular xy.1 xy.2} ⊆ (S : Set (K × K)) := by
    intro xy hxy
    simp only [S, Finset.coe_filter, Finset.mem_univ, true_and, Set.mem_ofPred_eq]
    exact hxy.1
  calc Nat.card V.Point
      = Nat.card (WithZero {xy : K × K // V.Nonsingular xy.1 xy.2}) :=
        Nat.card_congr V.nonsingularPointEquiv
    _ = Nat.card {xy : K × K // V.Nonsingular xy.1 xy.2} + 1 := Finite.card_option
    _ ≤ S.card + 1 := by
        gcongr
        calc Nat.card {xy : K × K // V.Nonsingular xy.1 xy.2}
            ≤ Nat.card (S : Set (K × K)) := Nat.card_mono (Set.toFinite _) hsub
          _ = S.card := Nat.card_coe_set_eq _ |>.trans (by simp)
    _ ≤ 2 * Fintype.card K + 1 := by omega

/-- **The counterexample bound.** For a curve over a finite field `K` and an odd-degree
extension `L`, the points of `W(L)` that are sums of `x ∈ K` factor-base points number at
most `2 #K + 1`. With `#K = q` and `[L:K] = n ≥ 3` odd, this contradicts
`Ω(q^{n-1}/(n-1)!)` (`HEUR-SP-JV-ASSUMPTION1`) and the `1/n!` proportion
(`HEUR-BK-GG-DECOMP-PROPORTION`) for all large `q`, although `#F ≥ #E(F_q) - 1 ≥ q - 2√q`
satisfies their factor-base-size hypotheses (Hasse, cited). -/
theorem card_decomposable_le_two_mul_card_add_one [Fintype K] [FiniteDimensional K L]
    [Finite (W⁄K).Point] (hodd : Odd (Module.finrank K L)) :
    Nat.card (AddSubgroup.closure (factorBase (L := L) W)) ≤ 2 * Fintype.card K + 1 :=
  (card_decomposable_le W hodd).trans (card_point_le (W⁄K).toAffine)

end SubfieldFactorBase
