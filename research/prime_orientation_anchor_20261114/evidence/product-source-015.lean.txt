module
public import OrientationContent
import all OrientationAnchor
import all OrientationValuation
import all OrientationContent
public section
noncomputable section
namespace OrientationAnchor
open Polynomial Finset

/-- At diagonal entries u is enough to represent every selected denominator order. -/
def chartOrder (u x y : ℕ) : ℕ := if x=y then u else order x y

/-- This unified formula includes diagonal entries without assigning a valuation to zero. -/
theorem selected_denominator_valuation (u x y j : ℕ) (hj : j ∈ mask u x y) :
    padicValRat 2 ((2:ℚ)^j-delta x y) = min (j:ℤ) (chartOrder u x y) := by
  by_cases hxy : x=y
  · subst y
    rw [diagonal_denominator_valuation]
    have hjl : j < u := mem_range.mp (mem_filter.mp hj).1
    simp only [chartOrder, ite_true]
    omega
  · simpa [chartOrder, hxy] using denominator_valuation u x y j hxy hj

/-- Actual chart factor content, before substituting a denominator valuation. -/
theorem chart_factor_content (a i j : ℕ) (t : ℤ) (d : ℚ)
    (ht : t ≠ 0) (hi : padicValInt 2 t = i) (hia : i ≤ a) :
    Content ((if j=i then (i:ℤ)+1 else min (i:ℤ) j)-padicValRat 2 ((2:ℚ)^j-d))
      (chart a t (factor d j)) := by
  have hd : VBound (-padicValRat 2 ((2:ℚ)^j-d)) (((2:ℚ)^j-d)⁻¹) :=
    Or.inr (by rw [padicValRat.inv])
  have h := (chart_numerator_content a i j t ht hi hia).mul (Content.C hd)
  simpa only [chart, factor, mul_comp, C_comp, sub_eq_add_neg] using h

/-- Actual coefficientwise content of the full masked correction product. -/
theorem chart_correction_content (a u x y i : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : t ≠ 0) (hi : padicValInt 2 t = i) (hia : i ≤ a) :
    Content (gain u x y i (chartOrder u x y) m) (chart a t (correction u x y m)) := by
  have h : ∀ j ∈ mask u x y,
      Content ((m j:ℤ)*beta i (chartOrder u x y) j) ((chart a t (factor (delta x y) j))^m j) := by
    intro j hj
    have hb := chart_factor_content a i j t (delta x y) ht hi hia
    rw [selected_denominator_valuation u x y j hj] at hb
    have hp := hb.pow (m j)
    simpa only [beta, min_comm (j:ℤ)] using hp
  have hp := Content.prod (mask u x y) (fun j => (chart a t (factor (delta x y) j))^m j)
    (fun j => (m j:ℤ)*beta i (chartOrder u x y) j) h
  simpa only [gain, chart, correction, Polynomial.prod_comp, pow_comp] using hp

/-- A crossing at a retained level gains the full diagonal schedule in every coefficient. -/
theorem actual_crossing_content (a u x y i : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : t ≠ 0) (hi : padicValInt 2 t = i) (hia : i ≤ a)
    (hiu : i < u) (hc : x.testBit i = y.testBit i) :
    Content (schedule u i m) (chart a t (correction u x y m)) := by
  apply (chart_correction_content a u x y i t m ht hi hia).mono
  exact crossing_gain u x y i (chartOrder u x y) m (mem_filter.mpr ⟨mem_range.mpr hiu, hc⟩)

/-- An inactive level has no loss from any selected anchor. -/
theorem beta_nonnegative_of_j_le (i s j : ℕ) (hj : j ≤ i) : 0 ≤ beta i s j := by
  dsimp [beta]
  split_ifs <;> omega

/-- Noncrossings are integral, derived from actual differing bits. -/
theorem actual_noncrossing_content (a u x y i : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : t ≠ 0) (hi : padicValInt 2 t = i) (hia : i ≤ a)
    (hc : x.testBit i ≠ y.testBit i) :
    Content 0 (chart a t (correction u x y m)) := by
  apply (chart_correction_content a u x y i t m ht hi hia).mono
  apply sum_nonneg
  intro j hj
  have hxy : x ≠ y := by intro he; exact hc (he ▸ rfl)
  have hs : chartOrder u x y ≤ i := by
    simpa [chartOrder, hxy] using order_le_of_bit_diff x y i hc
  exact mul_nonneg (by positivity) (beta_nonnegative i _ j hs)

/-- Levels above the retained range, including the -L endpoint, are integral. -/
theorem actual_inactive_content (a u x y i : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : t ≠ 0) (hi : padicValInt 2 t = i) (hia : i ≤ a) (hui : u ≤ i) :
    Content 0 (chart a t (correction u x y m)) := by
  apply (chart_correction_content a u x y i t m ht hi hia).mono
  apply sum_nonneg
  intro j hj
  have hjl : j < u := mem_range.mp (mem_filter.mp hj).1
  exact mul_nonneg (by positivity) (beta_nonnegative_of_j_le i _ j (by omega))

/-- A feasible schedule gives an integral product at every nonzero source-node order. -/
theorem actual_nonzero_content (a u x y i : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : t ≠ 0) (hi : padicValInt 2 t = i) (hia : i ≤ a)
    (hm : ∀ j < u, 0 ≤ schedule u j m) :
    Content 0 (chart a t (correction u x y m)) := by
  by_cases hiu : i < u
  · by_cases hc : x.testBit i = y.testBit i
    · exact (actual_crossing_content a u x y i t m ht hi hia hiu hc).mono (hm i hiu)
    · exact actual_noncrossing_content a u x y i t m ht hi hia hc
  · exact actual_inactive_content a u x y i t m ht hi hia (by omega)

/-- The original old-guard counterexample is a retained crossing with the new guard. -/
theorem old_counterexample_covered (u : ℕ) (hu : 1 < u) : 1 ∈ mask u 0 1 := by
  apply mem_filter.mpr
  exact ⟨mem_range.mpr hu, by decide⟩


/-- The zero chart has integral correction factors, independently of schedule feasibility. -/
theorem zero_chart_factor_content (a j : ℕ) (d : ℚ) (hj : j ≤ a)
    (hd : padicValRat 2 ((2:ℚ)^j-d) ≤ j) :
    Content 0 (chart a 0 (factor d j)) := by
  have hpow : VBound (j:ℤ) ((2:ℚ)^j) :=
    Or.inr (by rw [padicValRat.pow, show padicValRat 2 (2:ℚ)=1 from padicValRat.self (p:=2) (by decide), mul_one])
  have halpha : VBound (j:ℤ) (-((2:ℚ)^(a+1))) := by
    apply VBound.neg
    right
    rw [padicValRat.pow, show padicValRat 2 (2:ℚ)=1 from padicValRat.self (p:=2) (by decide), mul_one]
    omega
  have hn : Content (j:ℤ) (C ((2:ℚ)^j) + C (-((2:ℚ)^(a+1)))*X) :=
    (Content.C hpow).add (by simpa only [add_zero] using (Content.C halpha).mul Content.X)
  have hi : Content (-padicValRat 2 ((2:ℚ)^j-d)) (C (((2:ℚ)^j-d)⁻¹)) :=
    Content.C (Or.inr (by rw [padicValRat.inv]))
  have hp := (hn.mul hi).mono (by omega : (0:ℤ) ≤ j + -padicValRat 2 ((2:ℚ)^j-d))
  convert hp using 1
  simp only [chart, factor, mul_comp, sub_comp, C_comp, X_comp, Int.cast_zero, C_0, zero_add, map_neg]
  ring

/-- All actual correction coefficients at the origin are integral. -/
theorem actual_zero_content (a u x y : ℕ) (m : ℕ → ℕ) (hu : u ≤ a) :
    Content 0 (chart a 0 (correction u x y m)) := by
  have h : ∀ j ∈ mask u x y, Content 0 ((chart a 0 (factor (delta x y) j))^m j) := by
    intro j hj
    have hjl : j < u := mem_range.mp (mem_filter.mp hj).1
    have hd : padicValRat 2 ((2:ℚ)^j-delta x y) ≤ j := by
      rw [selected_denominator_valuation u x y j hj]
      exact min_le_left _ _
    simpa only [mul_zero] using (zero_chart_factor_content a j (delta x y) (by omega) hd).pow (m j)
  have hp := Content.prod (mask u x y) (fun j => (chart a 0 (factor (delta x y) j))^m j) (fun _ => 0) h
  simpa only [sum_const_zero, chart, correction, Polynomial.prod_comp, pow_comp] using hp

/-- Every nonzero index in the literal source interval has valuation at most a. -/
theorem grid_order_le (a : ℕ) (t : ℤ)
    (ht : t ∈ Ico (-(2:ℤ)^a) ((2:ℤ)^a)) (ht0 : t ≠ 0) :
    padicValInt 2 t ≤ a := by
  have htlo := (mem_Ico.mp ht).1
  have hthi := (mem_Ico.mp ht).2
  have habs : t.natAbs ≤ 2^a := by
    have h : (t.natAbs:ℤ) ≤ (2:ℤ)^a := by
      by_cases hnonneg : 0 ≤ t
      · rw [Int.natCast_natAbs, abs_of_nonneg hnonneg]
        omega
      · rw [Int.natCast_natAbs, abs_of_nonpos (by omega)]
        omega
    exact_mod_cast h
  have hd : 2^(padicValInt 2 t) ≤ t.natAbs := by
    simpa using Int.natAbs_le_of_dvd_ne_zero (padicValInt_dvd (p:=2) t) ht0
  exact (pow_le_pow_iff_right₀ (by decide : (1:ℕ)<2)).mp (hd.trans habs)

/-- The correction product is integral at every literal source chart, including zero and -L. -/
theorem all_grid_correction_content (a u x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : t ∈ Ico (-(2:ℤ)^a) ((2:ℤ)^a)) (hu : u ≤ a)
    (hm : ∀ j < u, 0 ≤ schedule u j m) :
    Content 0 (chart a t (correction u x y m)) := by
  by_cases ht0 : t=0
  · subst t
    exact actual_zero_content a u x y m hu
  · exact actual_nonzero_content a u x y (padicValInt 2 t) t m ht0 rfl (grid_order_le a t ht ht0) hm

/-- The original normalized source chart keeps its content after the constructed correction.
The base premise is explicit: its literal Cauchy integrality is a separate obligation. -/
theorem completed_content_from_base (a u x y : ℕ) (t : ℤ) (m : ℕ → ℕ) (k : ℤ)
    (ht : t ∈ Ico (-(2:ℤ)^a) ((2:ℤ)^a)) (hu : u ≤ a)
    (hm : ∀ j < u, 0 ≤ schedule u j m)
    (hb : Content k (chart a t (base (grid a) (delta x y)))) :
    Content k (chart a t (completed (grid a) u x y m)) := by
  simpa only [chart, completed, mul_comp, add_zero] using
    hb.mul (all_grid_correction_content a u x y t m ht hu hm)

/-- Actual relative gain g at a retained crossing, with no desired gain assumed for the correction. -/
theorem completed_crossing_content_from_base (a u x y i : ℕ) (t : ℤ) (m : ℕ → ℕ) (k g : ℤ)
    (ht : t ≠ 0) (hi : padicValInt 2 t = i) (hia : i ≤ a)
    (hiu : i < u) (hc : x.testBit i = y.testBit i) (hg : g ≤ schedule u i m)
    (hb : Content k (chart a t (base (grid a) (delta x y)))) :
    Content (k+g) (chart a t (completed (grid a) u x y m)) := by
  have hcorr := (actual_crossing_content a u x y i t m ht hi hia hiu hc).mono hg
  simpa only [chart, completed, mul_comp] using hb.mul hcorr

end OrientationAnchor
