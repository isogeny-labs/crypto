module
public import OrientationCompletion
import all OrientationAnchor
import all OrientationValuation
import all OrientationContent
import all OrientationProduct
import all OrientationBaseBridge
import all OrientationMoment
import all OrientationResidue
import all OrientationDiagonal
import all OrientationJet
import all OrientationLeading
import all OrientationRefinement
import all OrientationFullChart
import all LocalIntegral
public section
noncomputable section
namespace OrientationGradedNode
open Polynomial Finset OrientationAnchor OrientationMoment OrientationBaseBridge OrientationFullChart OrientationResidue OrientationDiagonal OrientationJet OrientationLeading OrientationRefinement

/-- Weighted coefficient content, retaining every coefficient degree. -/
def Graded (k : ℤ) (P : ℚ[X]) : Prop := ∀ n : ℕ, VBound (k+(n:ℤ)) (P.coeff n)

theorem Graded.mono {k l : ℤ} {P : ℚ[X]} (h : Graded k P) (hl : l ≤ k) :
    Graded l P := fun n => (h n).mono (by omega)

theorem Graded.C {k : ℤ} {q : ℚ} (h : VBound k q) : Graded k (C q) := by
  intro n
  by_cases hn : n=0
  · subst n; simpa using h
  · simp [coeff_C, hn, VBound]

theorem Graded.one : Graded 0 (1:ℚ[X]) := by
  simpa using (Graded.C (by simp [VBound] : VBound 0 (1:ℚ)))

theorem Graded.X : Graded (-1) (X:ℚ[X]) := by
  intro n
  by_cases hn : n=1
  · subst n; simp [VBound]
  · simp [coeff_X, Ne.symm hn, VBound]

theorem Graded.add {k : ℤ} {P Q : ℚ[X]} (hP : Graded k P) (hQ : Graded k Q) :
    Graded k (P+Q) := by
  intro n
  rw [coeff_add]
  exact (hP n).add (hQ n)

theorem Graded.mul {k l : ℤ} {P Q : ℚ[X]} (hP : Graded k P) (hQ : Graded l Q) :
    Graded (k+l) (P*Q) := by
  intro n
  rw [coeff_mul]
  apply VBound.sum
  intro ij hij
  have hs := mem_antidiagonal.mp hij
  have he : (k+(ij.1:ℤ))+(l+(ij.2:ℤ))=k+l+(n:ℤ) := by omega
  simpa only [he] using (hP ij.1).mul (hQ ij.2)

theorem Graded.pow {k : ℤ} {P : ℚ[X]} (h : Graded k P) (n : ℕ) :
    Graded ((n:ℤ)*k) (P^n) := by
  induction n with
  | zero => simpa using Graded.one
  | succ n ih =>
    simpa only [pow_succ, Nat.cast_add, Nat.cast_one, add_mul, one_mul] using ih.mul h

theorem Graded.prod {ι : Type*} (S : Finset ι) (P : ι → ℚ[X]) (k : ι → ℤ)
    (h : ∀ j ∈ S, Graded (k j) (P j)) : Graded (∑ j ∈ S, k j) (∏ j ∈ S, P j) := by
  classical
  induction S using Finset.induction_on with
  | empty => simpa using Graded.one
  | @insert i S hi ih =>
    rw [sum_insert hi, prod_insert hi]
    exact (h i (mem_insert_self _ _)).mul
      (ih (by intro j hj; exact h j (mem_insert_of_mem hj)))

theorem Graded.sum {ι : Type*} (S : Finset ι) (P : ι → ℚ[X]) (k : ℤ)
    (h : ∀ j ∈ S, Graded k (P j)) : Graded k (∑ j ∈ S, P j) := by
  intro n
  rw [finsetSum_coeff]
  exact VBound.sum S (fun j => (P j).coeff n) (k+(n:ℤ)) (fun j hj => h j hj n)

/-- Direct affine-factor bound, permitting a zero chart value. -/
theorem actual_factor_graded (a i j : ℕ) (t : ℤ) (d : ℚ)
    (ht : t ≠ 0) (hi : padicValInt 2 t=i) (hia : i<a) :
    Graded ((if j=i then (i:ℤ)+1 else min (i:ℤ) j)-padicValRat 2 ((2:ℚ)^j-d))
      (chart a t (factor d j)) := by
  let k : ℤ := (if j=i then (i:ℤ)+1 else min (i:ℤ) j)-padicValRat 2 ((2:ℚ)^j-d)
  have he : chart a t (factor d j) =
      C (((2:ℚ)^j-(t:ℚ))*((2:ℚ)^j-d)⁻¹)+
      C (-((2:ℚ)^(a+1))*((2:ℚ)^j-d)⁻¹)*X := by
    apply Polynomial.funext
    intro z
    simp only [chart, eval_comp, eval_add, eval_mul, eval_C, eval_X, factor_eval, div_eq_mul_inv]
    ring
  have h0 := chart_factor_content a i j t d ht hi (by omega) 0
  have hinv : VBound (-padicValRat 2 ((2:ℚ)^j-d)) (((2:ℚ)^j-d)⁻¹) :=
    Or.inr (by rw [padicValRat.inv])
  have hb := (power_integral (a+1)).neg.mul hinv
  have hslope : VBound (k+1) (-((2:ℚ)^(a+1))*((2:ℚ)^j-d)⁻¹) :=
    hb.mono (by dsimp [k]; split_ifs <;> omega)
  intro n
  by_cases hn0 : n=0
  · subst n; simpa [k] using h0
  by_cases hn1 : n=1
  · subst n
    have h1 : (chart a t (factor d j)).coeff 1 = -((2:ℚ)^(a+1))*((2:ℚ)^j-d)⁻¹ := by
      rw [he]
      simp only [coeff_add, coeff_C_mul, coeff_C, coeff_X]
      norm_num
    change VBound (k+1) ((chart a t (factor d j)).coeff 1)
    rw [h1]
    exact hslope
  · have hz : (chart a t (factor d j)).coeff n = 0 := by
      rw [he]
      simp only [coeff_add, coeff_C_mul, coeff_C, coeff_X]
      simp [hn0, Ne.symm hn1]
    rw [hz]
    exact VBound.zero _

theorem actual_correction_graded (a u x y i : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : t ≠ 0) (hi : padicValInt 2 t=i) (hia : i<a) :
    Graded (gain u x y i (chartOrder u x y) m) (chart a t (correction u x y m)) := by
  have h : ∀ j ∈ mask u x y,
      Graded ((m j:ℤ)*beta i (chartOrder u x y) j) ((chart a t (factor (delta x y) j))^m j) := by
    intro j hj
    have hb := actual_factor_graded a i j t (delta x y) ht hi hia
    rw [selected_denominator_valuation u x y j hj] at hb
    simpa only [beta, min_comm (j:ℤ)] using hb.pow (m j)
  have hp := Graded.prod (mask u x y) (fun j => (chart a t (factor (delta x y) j))^m j)
    (fun j => (m j:ℤ)*beta i (chartOrder u x y) j) h
  simpa only [gain, chart, correction, Polynomial.prod_comp, pow_comp] using hp

theorem actual_crossing_graded (a u x y i : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : t ≠ 0) (hi : padicValInt 2 t=i) (hia : i<a)
    (hiu : i<u) (hc : x.testBit i=y.testBit i) :
    Graded (schedule u i m) (chart a t (correction u x y m)) := by
  apply (actual_correction_graded a u x y i t m ht hi hia).mono
  exact crossing_gain u x y i (chartOrder u x y) m (mem_filter.mpr ⟨mem_range.mpr hiu,hc⟩)

theorem indexed_base_graded (a i : ℕ) (m s : Nodes a) (hsm : s ≠ m)
    (hv : padicValRat 2 (node a m-node a s) ≤ (i:ℤ)) :
    Graded ((a:ℤ)-i) ((base (grid a) (node a s)).comp
      (C (node a m)+C ((2:ℚ)^(a+1))*X)) := by
  have hr : VBound ((a:ℤ)+1-i)
      ((LocalIntegral.chartRatio a m s:LocalIntegral.integralRing):ℚ) := by
    rw [chartRatio_value]
    have hb := vbound_div (power_integral (a+1)) (r:=node a m-node a s)
    exact hb.mono (by omega)
  have hu : VBound 0 ((LocalIntegral.rhoUnit a m:LocalIntegral.integralRing):ℚ) :=
    vbound_of_norm_le_one _ (LocalIntegral.rhoUnit a m:LocalIntegral.integralRing).property
  have hrest : Graded 0 ((restPolynomial a m s).map LocalIntegral.integralRing.subtype) := by
    intro n
    simpa only [zero_add] using actual_rest_weighted a m s n
  rw [← chart_polynomial, mapped_localBase_factor a m s hsm]
  have hh := ((Graded.C hu).mul ((Graded.C hr).mul Graded.X)).mul hrest
  have he : (0+(((a:ℤ)+1-i)+(-1)))+0=(a:ℤ)-i := by omega
  simpa only [he] using hh

/-- Equal crossing bits exclude the actual source translation at any chart node. -/
theorem actual_displacement_ne_node (x y i : ℕ) (t : ℤ)
    (ht : t ≠ 0) (hi : padicValInt 2 t=i) (hc : x.testBit i=y.testBit i) :
    delta x y ≠ (t:ℚ) := by
  intro he
  have hxy : x ≠ y := by
    intro h
    subst y
    have htQ : (t:ℚ)=0 := by simpa [delta] using he.symm
    exact ht (by exact_mod_cast htQ)
  have hv : (order x y:ℤ)=(i:ℤ) := by
    calc
      (order x y:ℤ) = padicValRat 2 (delta x y) := (delta_valuation x y).symm
      _ = padicValRat 2 (t:ℚ) := by rw [he]
      _ = (i:ℤ) := by rw [padicValRat.of_int, hi]
  have ho : order x y=i := by exact_mod_cast hv
  exact order_bit_diff x y hxy (by simpa only [ho] using hc)

theorem actual_node_denominator_le (x y i : ℕ) (t : ℤ)
    (ht : t ≠ 0) (hi : padicValInt 2 t=i) (hc : x.testBit i=y.testBit i) :
    padicValRat 2 ((t:ℚ)-delta x y) ≤ (i:ℤ) := by
  have hv : padicValRat 2 (t:ℚ)=(i:ℤ) := by rw [padicValRat.of_int, hi]
  by_cases he : x=y
  · subst y
    simpa only [delta, sub_self, sub_zero, hv] using (le_rfl : (i:ℤ) ≤ i)
  · have ho : order x y ≠ i := by
      intro h
      exact order_bit_diff x y he (by simpa only [h] using hc)
    have htQ : (t:ℚ) ≠ 0 := by exact_mod_cast ht
    have hd : delta x y ≠ 0 := by
      rw [delta_as_int]
      exact_mod_cast signed_ne_zero x y he
    have hdiff := sub_ne_zero.mpr (Ne.symm (actual_displacement_ne_node x y i t ht hi hc))
    have hvne : padicValRat 2 (t:ℚ) ≠ padicValRat 2 (-delta x y) := by
      rw [hv, padicValRat.neg, delta_valuation]
      exact_mod_cast Ne.symm ho
    rw [sub_eq_add_neg, padicValRat.add_eq_min
      (by simpa only [← sub_eq_add_neg] using hdiff) htQ (neg_ne_zero.mpr hd) hvne, hv]
    exact min_le_left _ _

/-- The indexed base bound is discharged by actual source nodes and crossing bits. -/
theorem actual_base_graded (a i x y : ℕ) (t : ℤ)
    (hx : x<2^a) (hy : y<2^a) (ht : (t:ℚ) ∈ grid a)
    (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hc : x.testBit i=y.testBit i) :
    Graded ((a:ℤ)-i) (chart a t (base (grid a) (delta x y))) := by
  have hd := delta_in_grid a x y hx hy
  rw [grid_eq_image] at ht hd
  obtain ⟨m,_,hm⟩ := mem_image.mp ht
  obtain ⟨s,_,hs⟩ := mem_image.mp hd
  have hsm : s ≠ m := by
    intro he
    subst s
    exact actual_displacement_ne_node x y i t ht0 hi hc (hs.symm.trans hm)
  have hv : padicValRat 2 (node a m-node a s) ≤ (i:ℤ) := by
    rw [hm,hs]
    exact actual_node_denominator_le x y i t ht0 hi hc
  have hb := indexed_base_graded a i m s hsm hv
  rw [hm,hs] at hb
  simpa only [chart] using hb

theorem actual_completed_graded (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hy : y<2^a) (ht : (t:ℚ) ∈ grid a)
    (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u) (hua : u≤a)
    (hc : x.testBit i=y.testBit i) :
    Graded (jetWeight a u i m-1) (chart a t (completed (grid a) u x y m)) := by
  have hb := actual_base_graded a i x y t hx hy ht ht0 hi hc
  have hd := actual_crossing_graded a u x y i t m ht0 hi (by omega) hiu hc
  have he : (a:ℤ)-i+schedule u i m=jetWeight a u i m-1 := by
    unfold jetWeight
    ring
  simpa only [he, chart, completed, mul_comp] using hb.mul hd

def normalizedNodeEntry (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ) : ℚ[X] :=
  C ((diagonalJet a u i m)⁻¹)*chart a t (completed (grid a) u x y m)

/-- Normalization uses the existing actual nonzero negative-pole diagonal,
not the potentially zero first derivative at this arbitrary node. -/
theorem actual_normalized_graded (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hy : y<2^a) (ht : (t:ℚ) ∈ grid a)
    (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u) (hua : u≤a)
    (hc : x.testBit i=y.testBit i) :
    Graded (-1) (normalizedNodeEntry a u i x y t m) := by
  have hv : VBound (-jetWeight a u i m) ((diagonalJet a u i m)⁻¹) :=
    Or.inr (by rw [padicValRat.inv, diagonalJet_valuation a u i m hiu hua]; rfl)
  have hh := (Graded.C hv).mul (actual_completed_graded a u i x y t m hx hy ht ht0 hi hiu hua hc)
  have he : -jetWeight a u i m+(jetWeight a u i m-1)=(-1:ℤ) := by omega
  simpa only [he, normalizedNodeEntry] using hh

theorem actual_node_coefficient_weight (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hy : y<2^a) (ht : (t:ℚ) ∈ grid a)
    (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u) (hua : u≤a)
    (hc : x.testBit i=y.testBit i) (n : ℕ) :
    VBound (n:ℤ) ((normalizedNodeEntry a u i x y t m).coeff (n+1)) := by
  have h := actual_normalized_graded a u i x y t m hx hy ht ht0 hi hiu hua hc (n+1)
  simpa using h

theorem actual_node_constant (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) :
    (normalizedNodeEntry a u i x y t m).coeff 0=0 := by
  have hz : (completed (grid a) u x y m).eval (t:ℚ)=0 := by
    rw [completed, eval_mul, base_at_other_node (grid a) (delta x y) (t:ℚ) ht
      (actual_displacement_ne_node x y i t ht0 hi hc), zero_mul]
  simp only [normalizedNodeEntry, coeff_zero_eq_eval_zero, chart,
    eval_comp, eval_add, eval_C, eval_mul, eval_X, mul_zero, add_zero, hz]

theorem actual_node_first (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ) :
    (normalizedNodeEntry a u i x y t m).coeff 1=
      (chart a t (completed (grid a) u x y m)).coeff 1/diagonalJet a u i m := by
  rw [normalizedNodeEntry, coeff_C_mul, div_eq_mul_inv]
  ring

theorem actual_node_leading_entry (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hy : y<2^a) (ht : (t:ℚ) ∈ grid a)
    (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u) (hua : u≤a)
    (hc : x.testBit i=y.testBit i) :
    Content 1 (normalizedNodeEntry a u i x y t m-
      C ((chart a t (completed (grid a) u x y m)).coeff 1/diagonalJet a u i m)*X) := by
  let q : ℚ := (chart a t (completed (grid a) u x y m)).coeff 1/diagonalJet a u i m
  intro n
  cases n with
  | zero =>
    have hlinear : (C q*X).coeff 0=0 := by rw [coeff_C_mul]; simp
    rw [coeff_sub, actual_node_constant a u i x y t m ht ht0 hi hc, hlinear, sub_self]
    exact VBound.zero _
  | succ n =>
    by_cases hn : n=0
    · subst n
      have hlinear : (C q*X).coeff 1=q := by rw [coeff_C_mul]; simp
      rw [coeff_sub, actual_node_first, hlinear, sub_self]
      exact VBound.zero _
    · have hlinear : (C q*X).coeff (n+1)=0 := by
        rw [coeff_C_mul]
        have hX : (X:ℚ[X]).coeff (n+1)=0 := by
          simp only [coeff_X, show ¬(1:ℕ)=n+1 by omega, ite_false]
        rw [hX, mul_zero]
      rw [coeff_sub, hlinear, sub_zero]
      exact (actual_node_coefficient_weight a u i x y t m hx hy ht ht0 hi hiu hua hc n).mono (by omega)

theorem positive_anchor_member (a i : ℕ) (hi : i<a) : (2:ℚ)^i ∈ grid a := by
  have hp0 : 0 < (2:ℤ)^i := by positivity
  have hpa0 : 0 < (2:ℤ)^a := by positivity
  have hnext : (2:ℤ)^(i+1) ≤ (2:ℤ)^a := pow_le_pow_right₀ (by norm_num) (by omega)
  have hsmall : (2:ℤ)^i < (2:ℤ)^a := by rw [pow_succ] at hnext; omega
  apply mem_image.mpr
  refine ⟨(2:ℤ)^i, mem_Ico.mpr ⟨by omega,hsmall⟩, ?_⟩
  push_cast
  rfl

theorem positive_anchor_correction_zero (u i x y : ℕ) (m : ℕ → ℕ)
    (hi : i<u) (hc : x.testBit i=y.testBit i) (hm : 0 < m i) :
    (correction u x y m).eval ((2:ℚ)^i)=0 := by
  rw [correction, eval_prod]
  apply prod_eq_zero (mem_filter.mpr ⟨mem_range.mpr hi,hc⟩)
  rw [eval_pow, factor_eval]
  simp [Nat.ne_of_gt hm]

/-- A positive retained anchor may have zero first crossing jet; no local
first-jet unit is assumed by the arbitrary-node bounds. -/
theorem positive_anchor_first_zero (a u i x y : ℕ) (m : ℕ → ℕ)
    (hi : i<u) (hua : u≤a) (hc : x.testBit i=y.testBit i) (hm : 0 < m i) :
    (chart a ((2:ℤ)^i) (completed (grid a) u x y m)).coeff 1=0 := by
  have hdt : delta x y ≠ (2:ℚ)^i := by intro h; exact anchor_flips x y i h hc
  have hb := base_at_other_node (grid a) (delta x y) ((2:ℚ)^i)
    (positive_anchor_member a i (by omega)) hdt
  have hd := positive_anchor_correction_zero u i x y m hi hc hm
  rw [chart_first, completed, derivative_mul]
  simp only [Int.cast_pow, Int.cast_ofNat, eval_add, eval_mul, hd, hb, mul_zero, zero_mul, add_zero]

/-- Exact correction comparison at an arbitrary value, including a zero numerator. -/
theorem actual_node_correction_comparison (u v x z y : ℕ) (T : ℚ) (m : ℕ → ℕ)
    (hu : u≤v) (h : x % 2^v=z % 2^v) :
    (correction u x y m).eval T =
      (correction u z y m).eval T*correctionRatio u x z y m := by
  have hmask := masks_stable u v x z y hu h
  have hf (j : ℕ) (hj : j ∈ mask u x y) :
      (factor (delta x y) j).eval T =
        (factor (delta z y) j).eval T*
          (((2:ℚ)^j-delta z y)/((2:ℚ)^j-delta x y)) := by
    rw [factor_eval, factor_eval]
    have hd := denominator_ne_zero u x y j hj
    have hz := denominator_ne_zero u z y j (hmask ▸ hj)
    field_simp [hd,hz]
  simp only [correction, eval_prod, eval_pow]
  rw [← hmask]
  calc
    _ = ∏ j ∈ mask u x y,
      ((factor (delta z y) j).eval T*
        (((2:ℚ)^j-delta z y)/((2:ℚ)^j-delta x y)))^m j := by
        apply prod_congr rfl
        intro j hj
        rw [hf j hj]
    _ = _ := by simp only [mul_pow, prod_mul_distrib, correctionRatio]

def nodeComparisonRatio (u x z y : ℕ) (t : ℤ) (m : ℕ → ℕ) : ℚ :=
  (((t:ℚ)-delta z y)/((t:ℚ)-delta x y))*correctionRatio u x z y m

theorem node_base_ratio_near (u v i x z y : ℕ) (t : ℤ)
    (hiu : i<u) (h : x % 2^v=z % 2^v)
    (ht : t ≠ 0) (hi : padicValInt 2 t=i) (hc : x.testBit i=y.testBit i) :
    NearOne ((v:ℤ)-u+1) (((t:ℚ)-delta z y)/((t:ℚ)-delta x y)) := by
  have hh := denominator_ratio v x z y (t:ℚ) h
    (sub_ne_zero.mpr (Ne.symm (actual_displacement_ne_node x y i t ht hi hc)))
  have hv := actual_node_denominator_le x y i t ht hi hc
  exact hh.mono (by omega)

theorem node_comparison_near (u v i x z y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hiu : i<u) (huv : u≤v) (h : x % 2^v=z % 2^v)
    (ht : t ≠ 0) (hi : padicValInt 2 t=i) (hc : x.testBit i=y.testBit i) :
    NearOne ((v:ℤ)-u+1) (nodeComparisonRatio u x z y t m) :=
  (node_base_ratio_near u v i x z y t hiu h ht hi hc).mul (by omega)
    (correction_ratio_near u v x z y m huv h)

/-- Literal all-node first-jet comparison, without cancelling a chart value. -/
theorem actual_node_jet_comparison (a u v i x z y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hz : z<2^a) (hy : y<2^a)
    (hiu : i<u) (huv : u≤v) (hva : v≤a) (h : x % 2^v=z % 2^v)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) :
    (chart a t (completed (grid a) u x y m)).coeff 1/diagonalJet a u i m =
      ((chart a t (completed (grid a) u z y m)).coeff 1/diagonalJet a u i m)*
        nodeComparisonRatio u x z y t m := by
  have hb := matching_bits v x z i (by omega) h
  have hzc : z.testBit i=y.testBit i := hb.symm.trans hc
  have hdx := actual_displacement_ne_node x y i t ht0 hi hc
  have hdz := actual_displacement_ne_node z y i t ht0 hi hzc
  rw [completed_first (grid a) a u x y t m (delta_in_grid a x y hx hy) ht hdx,
    completed_first (grid a) a u z y t m (delta_in_grid a z y hz hy) ht hdz,
    actual_node_correction_comparison u v x z y (t:ℚ) m huv h]
  unfold nodeComparisonRatio
  field_simp [sub_ne_zero.mpr (Ne.symm hdx), sub_ne_zero.mpr (Ne.symm hdz),
    diagonalJet_ne_zero a u i m (by omega)]

theorem actual_node_first_integral (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hy : y<2^a) (ht : (t:ℚ) ∈ grid a)
    (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u) (hua : u≤a)
    (hc : x.testBit i=y.testBit i) :
    VBound 0 ((chart a t (completed (grid a) u x y m)).coeff 1/diagonalJet a u i m) := by
  have hh := actual_node_coefficient_weight a u i x y t m hx hy ht ht0 hi hiu hua hc 0
  rw [actual_node_first] at hh
  exact hh

theorem actual_node_precision_comparison (a u v i x z y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hz : z<2^a) (hy : y<2^a)
    (hiu : i<u) (huv : u≤v) (hva : v≤a) (h : x % 2^v=z % 2^v)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) :
    VBound ((v:ℤ)-u+1)
      ((chart a t (completed (grid a) u x y m)).coeff 1/diagonalJet a u i m-
        (chart a t (completed (grid a) u z y m)).coeff 1/diagonalJet a u i m) := by
  have hb := matching_bits v x z i (by omega) h
  have hzc : z.testBit i=y.testBit i := hb.symm.trans hc
  have hp := actual_node_first_integral a u i z y t m hz hy ht ht0 hi hiu (by omega) hzc
  have hr := node_comparison_near u v i x z y t m hiu huv h ht0 hi hc
  have hh : VBound ((v:ℤ)-u+1)
      ((nodeComparisonRatio u x z y t m-1)*
        ((chart a t (completed (grid a) u z y m)).coeff 1/diagonalJet a u i m)) := by
    simpa only [add_zero] using VBound.mul hr hp
  rw [actual_node_jet_comparison a u v i x z y t m hx hz hy hiu huv hva h ht ht0 hi hc]
  convert hh using 1
  ring

def nodeCrossingFirst (a u i : ℕ) (t : ℤ) (m : ℕ → ℕ) : Matrix (Ports a) (Ports a) ℚ :=
  fun x y => if x.val.testBit i=y.val.testBit i then
    (chart a t (completed (grid a) u x.val y.val m)).coeff 1/diagonalJet a u i m else 0

def nodeRight (a u v i : ℕ) (hva : v≤a) (t : ℤ) (m : ℕ → ℕ) :
    Matrix (Labels v) (Ports a) ℚ :=
  fun r y => nodeCrossingFirst a u i t m (representative a v hva r) y

theorem node_right_integral (a u v i : ℕ) (hva : v≤a) (t : ℤ) (m : ℕ → ℕ)
    (hiu : i<u) (huv : u≤v) (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0)
    (hi : padicValInt 2 t=i) (r : Labels v) (y : Ports a) :
    VBound 0 (nodeRight a u v i hva t m r y) := by
  unfold nodeRight nodeCrossingFirst
  split_ifs with hc
  · exact actual_node_first_integral a u i _ y.val t m (representative a v hva r).isLt
      y.isLt ht ht0 hi hiu (by omega) hc
  · exact VBound.zero _

theorem node_product (a u v i : ℕ) (hva : v≤a) (t : ℤ) (m : ℕ → ℕ) (x y : Ports a) :
    (leftFactor a v*nodeRight a u v i hva t m) x y =
      nodeCrossingFirst a u i t m
        (representative a v hva ⟨x.val % 2^v,Nat.mod_lt _ (by positivity)⟩) y := by
  let r0 : Labels v := ⟨x.val % 2^v,Nat.mod_lt _ (by positivity)⟩
  have hz (r : Labels v) (hr : r ≠ r0) : leftFactor a v x r=0 := by
    unfold leftFactor
    split_ifs with he
    · exact False.elim (hr (Fin.ext he.symm))
    · rfl
  have hs : (∑ r : Labels v,leftFactor a v x r*nodeRight a u v i hva t m r y) =
      leftFactor a v x r0*nodeRight a u v i hva t m r0 y :=
    Finset.sum_eq_single r0 (by intro r _ hr; rw [hz r hr,zero_mul])
      (by intro h; exact False.elim (h (mem_univ _)))
  rw [Matrix.mul_apply,hs]
  simp [leftFactor,nodeRight,r0]

theorem actual_node_precision_factorization (a u v i : ℕ) (hva : v≤a)
    (t : ℤ) (m : ℕ → ℕ) (hiu : i<u) (huv : u≤v)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (x y : Ports a) :
    VBound ((v:ℤ)-u+1)
      (nodeCrossingFirst a u i t m x y-(leftFactor a v*nodeRight a u v i hva t m) x y) := by
  rw [node_product]
  let z : Ports a := representative a v hva ⟨x.val % 2^v,Nat.mod_lt _ (by positivity)⟩
  have hz : z.val=x.val % 2^v := rfl
  have hmod : x.val % 2^v=z.val % 2^v := by rw [hz,Nat.mod_mod]
  have hb := matching_bits v x.val z.val i (by omega) hmod
  change VBound _ (nodeCrossingFirst a u i t m x y-nodeCrossingFirst a u i t m z y)
  by_cases hc : x.val.testBit i=y.val.testBit i
  · have hzc : z.val.testBit i=y.val.testBit i := hb.symm.trans hc
    simp only [nodeCrossingFirst,hc,hzc,ite_true]
    exact actual_node_precision_comparison a u v i x.val z.val y.val t m x.isLt z.isLt y.isLt
      hiu huv hva hmod ht ht0 hi hc
  · have hzc : z.val.testBit i ≠ y.val.testBit i := by rwa [← hb]
    simp [nodeCrossingFirst,hc,hzc,VBound]

theorem node_full_row_factorization (a u i : ℕ) (t : ℤ) (m : ℕ → ℕ) (x y : Ports a) :
    (leftFactor a a*nodeRight a u a i le_rfl t m) x y=nodeCrossingFirst a u i t m x y := by
  rw [node_product]
  have hz : representative a a le_rfl ⟨x.val % 2^a,Nat.mod_lt _ (by positivity)⟩=x := by
    apply Fin.ext
    exact Nat.mod_eq_of_lt x.isLt
  rw [hz]

def nodePolynomialMatrix (a u i : ℕ) (t : ℤ) (m : ℕ → ℕ) : Matrix (Ports a) (Ports a) ℚ[X] :=
  fun x y => if x.val.testBit i=y.val.testBit i then normalizedNodeEntry a u i x.val y.val t m else 0

def nodePolyRight (a u i : ℕ) (hua : u≤a) (t : ℤ) (m : ℕ → ℕ) :
    Matrix (Labels u) (Ports a) ℚ[X] := fun r y => C (nodeRight a u u i hua t m r y)*X

theorem node_poly_product (a u i : ℕ) (hua : u≤a) (t : ℤ) (m : ℕ → ℕ) (x y : Ports a) :
    (chartLeft a u*nodePolyRight a u i hua t m) x y =
      C ((leftFactor a u*nodeRight a u u i hua t m) x y)*X := by
  simp only [Matrix.mul_apply,chartLeft,nodePolyRight]
  calc
    _ = ∑ r : Labels u,C (leftFactor a u x r*nodeRight a u u i hua t m r y)*X := by
      apply sum_congr rfl
      intro r _
      rw [map_mul]
      ring
    _ = (∑ r : Labels u,C (leftFactor a u x r*nodeRight a u u i hua t m r y))*X := (sum_mul _ _ _).symm
    _ = _ := by rw [map_sum]

theorem node_poly_right_integral (a u i : ℕ) (hua : u≤a) (t : ℤ) (m : ℕ → ℕ)
    (hiu : i<u) (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (r : Labels u) (y : Ports a) : Content 0 (nodePolyRight a u i hua t m r y) := by
  unfold nodePolyRight
  simpa only [add_zero] using
    (Content.C (node_right_integral a u u i hua t m hiu le_rfl ht ht0 hi r y)).mul Content.X

theorem node_chart_to_first (a u i : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hiu : i<u) (hua : u≤a) (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0)
    (hi : padicValInt 2 t=i) (x y : Ports a) :
    Content 1 (nodePolynomialMatrix a u i t m x y-C (nodeCrossingFirst a u i t m x y)*X) := by
  unfold nodePolynomialMatrix nodeCrossingFirst
  split_ifs with hc
  · exact actual_node_leading_entry a u i x.val y.val t m x.isLt y.isLt ht ht0 hi hiu hua hc
  · intro n
    simp [VBound]

/-- Constructed all-retained-node polynomial factors, not a supplied rank witness. -/
theorem actual_node_chart_factorization (a u i : ℕ) (hua : u≤a) (t : ℤ) (m : ℕ → ℕ)
    (hiu : i<u) (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0)
    (hi : padicValInt 2 t=i) (x y : Ports a) :
    Content 1 (nodePolynomialMatrix a u i t m x y-(chartLeft a u*nodePolyRight a u i hua t m) x y) := by
  rw [node_poly_product]
  have hp := node_chart_to_first a u i t m hiu hua ht ht0 hi x y
  have hr : VBound 1 (nodeCrossingFirst a u i t m x y-
      (leftFactor a u*nodeRight a u u i hua t m) x y) := by
    simpa using actual_node_precision_factorization a u u i hua t m hiu le_rfl ht ht0 hi x y
  have hc : Content 1 (C (nodeCrossingFirst a u i t m x y-
      (leftFactor a u*nodeRight a u u i hua t m) x y)*X) := by
    simpa only [add_zero] using (Content.C hr).mul Content.X
  have hh := hp.add hc
  have he : nodePolynomialMatrix a u i t m x y-C (nodeCrossingFirst a u i t m x y)*X+
      C (nodeCrossingFirst a u i t m x y-(leftFactor a u*nodeRight a u u i hua t m) x y)*X =
      nodePolynomialMatrix a u i t m x y-C ((leftFactor a u*nodeRight a u u i hua t m) x y)*X := by
    rw [map_sub]
    ring
  rwa [he] at hh

def nodeCrossingPolynomial (a u i : ℕ) (t : ℤ) (m : ℕ → ℕ) : Matrix (Ports a) (Ports a) ℚ[X] :=
  fun x y => if x.val.testBit i=y.val.testBit i then chart a t (completed (grid a) u x.val y.val m) else 0

theorem node_crossing_scale (a u i : ℕ) (t : ℤ) (m : ℕ → ℕ) (hia : i≤a) (x y : Ports a) :
    nodeCrossingPolynomial a u i t m x y=C (diagonalJet a u i m)*nodePolynomialMatrix a u i t m x y := by
  unfold nodeCrossingPolynomial nodePolynomialMatrix
  split_ifs
  · rw [normalizedNodeEntry, ← mul_assoc, ← map_mul,
      mul_inv_cancel₀ (diagonalJet_ne_zero a u i m hia), map_one, one_mul]
  · simp

theorem node_weighted_product (a u i : ℕ) (hua : u≤a) (t : ℤ) (m : ℕ → ℕ) (x y : Ports a) :
    (chartWeightedLeft a u i m*nodePolyRight a u i hua t m) x y=
      C (diagonalJet a u i m)*(chartLeft a u*nodePolyRight a u i hua t m) x y := by
  simp only [Matrix.mul_apply,chartWeightedLeft]
  rw [mul_sum]
  apply sum_congr rfl
  intro r _
  ring

theorem actual_node_weighted_chart_factorization (a u i : ℕ) (hua : u≤a) (t : ℤ) (m : ℕ → ℕ)
    (hiu : i<u) (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0)
    (hi : padicValInt 2 t=i) (x y : Ports a) :
    Content (jetWeight a u i m+1)
      (nodeCrossingPolynomial a u i t m x y-(chartWeightedLeft a u i m*nodePolyRight a u i hua t m) x y) := by
  have hc : VBound (jetWeight a u i m) (diagonalJet a u i m) := Or.inr (by
    rw [diagonalJet_valuation a u i m hiu hua]
    rfl)
  have hh := (Content.C hc).mul (actual_node_chart_factorization a u i hua t m hiu ht ht0 hi x y)
  rw [node_crossing_scale a u i t m (by omega),node_weighted_product]
  convert hh using 1
  ring

end OrientationGradedNode
#print axioms OrientationGradedNode.actual_crossing_graded
#print axioms OrientationGradedNode.indexed_base_graded

#print axioms OrientationGradedNode.actual_completed_graded
#print axioms OrientationGradedNode.actual_node_coefficient_weight

#print axioms OrientationGradedNode.actual_node_leading_entry
#print axioms OrientationGradedNode.positive_anchor_first_zero

#print axioms OrientationGradedNode.actual_node_jet_comparison
#print axioms OrientationGradedNode.actual_node_precision_comparison

#print axioms OrientationGradedNode.actual_node_precision_factorization
#print axioms OrientationGradedNode.actual_node_chart_factorization
#print axioms OrientationGradedNode.actual_node_weighted_chart_factorization
