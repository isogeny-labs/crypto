module
public import OrientationMoment
import all OrientationMoment
import all LocalIntegral
import all LocalCharts
import all OrientationAnchor
import all OrientationValuation
import all OrientationContent
import all OrientationProduct
import all OrientationResidue
import all OrientationDiagonal
import all OrientationBaseBridge
import all OrientationJet
import all OrientationLeading
import all OrientationRefinement
public section
noncomputable section
namespace OrientationFullChart
open Polynomial Finset OrientationAnchor OrientationResidue OrientationDiagonal OrientationBaseBridge OrientationJet OrientationLeading OrientationRefinement OrientationMoment

theorem mapped_integral_content (P : Polynomial LocalIntegral.integralRing) :
    Content 0 (P.map LocalIntegral.integralRing.subtype) := by
  intro n
  rw [coeff_map]
  exact vbound_of_norm_le_one _ (P.coeff n).property

def restPolynomial (a : ℕ) (m s : Nodes a) : Polynomial LocalIntegral.integralRing :=
  ∏ j ∈ (univ.erase m).erase s,(1+C (LocalIntegral.chartRatio a m j)*X)

theorem chartRatio_value (a : ℕ) (m s : Nodes a) :
    ((LocalIntegral.chartRatio a m s:LocalIntegral.integralRing):ℚ) =
      (2:ℚ)^(a+1)/(node a m-node a s) := by
  change (((2^(a+1):ℕ):ℚ)/((m.val:ℚ)-s.val)) = _
  simp only [Nat.cast_pow,Nat.cast_ofNat,node]
  congr 1
  ring

theorem mapped_localBase_factor (a : ℕ) (m s : Nodes a) (hsm : s ≠ m) :
    (LocalCharts.localBase a m s).map LocalIntegral.integralRing.subtype =
      C ((LocalIntegral.rhoUnit a m:LocalIntegral.integralRing):ℚ)*
        (C ((LocalIntegral.chartRatio a m s:LocalIntegral.integralRing):ℚ)*X)*
          (restPolynomial a m s).map LocalIntegral.integralRing.subtype := by
  simp only [LocalCharts.localBase,hsm,ite_false,Polynomial.map_mul,Polynomial.map_C,
    Polynomial.map_X,restPolynomial]
  rfl

theorem indexed_crossing_base_content (a i : ℕ) (m s : Nodes a) (hsm : s ≠ m)
    (hv : padicValRat 2 (node a m-node a s) ≤ (i:ℤ)) :
    Content ((a:ℤ)+1-i) ((base (grid a) (node a s)).comp
      (C (node a m)+C ((2:ℚ)^(a+1))*X)) := by
  have hr : VBound ((a:ℤ)+1-i)
      ((LocalIntegral.chartRatio a m s:LocalIntegral.integralRing):ℚ) := by
    rw [chartRatio_value]
    have hb := vbound_div (power_integral (a+1)) (r:=node a m-node a s)
    exact hb.mono (by omega)
  have hu : VBound 0 ((LocalIntegral.rhoUnit a m:LocalIntegral.integralRing):ℚ) :=
    vbound_of_norm_le_one _ (LocalIntegral.rhoUnit a m:LocalIntegral.integralRing).property
  rw [← chart_polynomial,mapped_localBase_factor a m s hsm]
  have hx : Content ((a:ℤ)+1-i)
      (C ((LocalIntegral.chartRatio a m s:LocalIntegral.integralRing):ℚ)*X) := by
    simpa only [add_zero] using (Content.C hr).mul Content.X
  have hfirst := (Content.C hu).mul hx
  simpa only [zero_add,add_zero] using hfirst.mul (mapped_integral_content (restPolynomial a m s))

/-- Strong actual base content at a retained crossing, without a content premise. -/
theorem actual_crossing_base_content (a u i x y : ℕ)
    (hx : x < 2^a) (hy : y < 2^a) (hi : i < u) (hu : u ≤ a)
    (hc : x.testBit i = y.testBit i) :
    Content ((a:ℤ)+1-i) (chart a (pole i) (base (grid a) (delta x y))) := by
  have ht := pole_member a i (by omega)
  have hd := delta_in_grid a x y hx hy
  rw [grid_eq_image] at ht hd
  obtain ⟨m,_,hm⟩ := mem_image.mp ht
  obtain ⟨s,_,hs⟩ := mem_image.mp hd
  have hsm : s ≠ m := by
    intro he
    subst s
    exact crossing_displacement_ne_pole x y i hc (hs.symm.trans hm)
  have hv : padicValRat 2 (node a m-node a s) ≤ (i:ℤ) := by
    rw [hm,hs]
    exact crossing_denominator_le x y i hc
  have hh := indexed_crossing_base_content a i m s hsm hv
  rw [hm,hs] at hh
  simpa only [chart,pole_cast] using hh

/-- Whole actual crossing chart at its derived diagonal weight, all coefficients. -/
theorem actual_completed_crossing_jetWeight (a u i x y : ℕ) (m : ℕ → ℕ)
    (hx : x < 2^a) (hy : y < 2^a) (hi : i < u) (hu : u ≤ a)
    (hc : x.testBit i = y.testBit i) :
    Content (jetWeight a u i m) (chart a (pole i) (completed (grid a) u x y m)) := by
  have hb := actual_crossing_base_content a u i x y hx hy hi hu hc
  have hh := completed_crossing_content_from_base a u x y i (pole i) m
    ((a:ℤ)+1-i) (schedule u i m) (pole_ne_zero i) (pole_valuation i)
    (by omega) hi hc le_rfl hb
  exact hh

/-- Dividing the WHOLE actual crossing chart by the actual diagonal is integral. -/
theorem normalized_crossing_chart_integral (a u i x y : ℕ) (m : ℕ → ℕ)
    (hx : x < 2^a) (hy : y < 2^a) (hi : i < u) (hu : u ≤ a)
    (hc : x.testBit i = y.testBit i) :
    Content 0 (C ((diagonalJet a u i m)⁻¹)*
      chart a (pole i) (completed (grid a) u x y m)) := by
  have hv : VBound (-jetWeight a u i m) ((diagonalJet a u i m)⁻¹) := Or.inr (by
    rw [padicValRat.inv,diagonalJet_valuation a u i m hi hu]
    rfl)
  have hh := (Content.C hv).mul (actual_completed_crossing_jetWeight a u i x y m hx hy hi hu hc)
  simpa only [neg_add_cancel] using hh


/-- Every actual grid-chart slope is strictly even, including its zero diagonal. -/
theorem actual_grid_slope_even (a : ℕ) (m j : Nodes a) :
    VBound 1 ((LocalIntegral.chartRatio a m j:LocalIntegral.integralRing):ℚ) := by
  by_cases he : m=j
  · subst j
    simp [LocalIntegral.chartRatio,VBound]
  let d : ℤ := (m:ℤ)-j
  have hd : d ≠ 0 := by
    intro hz
    have hmj : m.val=j.val := by dsimp [d] at hz; omega
    exact he (Fin.ext hmj)
  have hbound : d.natAbs < 2^(a+1) := by
    have hm := m.isLt
    have hj := j.isLt
    dsimp [d]
    omega
  have hn := LocalIntegral.bounded_valuation a d.natAbs (Int.natAbs_ne_zero.mpr hd) hbound
  have hv : padicValRat 2 (d:ℚ) ≤ (a:ℤ) := by
    rw [padicValRat.of_int]
    change (padicValNat 2 d.natAbs:ℤ) ≤ a
    exact_mod_cast Nat.le_of_lt_succ hn
  have hh := vbound_div (power_integral (a+1)) (r:=(d:ℚ))
  have hh1 := hh.mono (show 1 ≤ (a+1:ℕ)-padicValRat 2 (d:ℚ) by omega)
  simpa only [LocalIntegral.chartRatio,d,Nat.cast_pow,Nat.cast_ofNat,Int.cast_sub,
    Int.cast_natCast] using hh1

theorem mapped_rest_product (a : ℕ) (m s : Nodes a) :
    (restPolynomial a m s).map LocalIntegral.integralRing.subtype =
      ∏ j ∈ (univ.erase m).erase s,
        (1+C ((LocalIntegral.chartRatio a m j:LocalIntegral.integralRing):ℚ)*X) := by
  simp only [restPolynomial,Polynomial.map_prod,Polynomial.map_add,Polynomial.map_one,
    Polynomial.map_mul,Polynomial.map_C,Polynomial.map_X]
  rfl

theorem actual_rest_weighted (a : ℕ) (m s : Nodes a) :
    WeightedPolynomial ((restPolynomial a m s).map LocalIntegral.integralRing.subtype) := by
  rw [mapped_rest_product]
  apply WeightedPolynomial.prod
  intro j _
  exact weighted_linear (actual_grid_slope_even a m j)

theorem actual_rest_constant (a : ℕ) (m s : Nodes a) :
    ((restPolynomial a m s).map LocalIntegral.integralRing.subtype).coeff 0 = 1 := by
  rw [coeff_zero_eq_eval_zero,mapped_rest_product]
  simp only [eval_prod,eval_add,eval_one,eval_mul,eval_C,eval_X,mul_zero,add_zero,prod_const_one]

def chartAnchorSlope (a i j : ℕ) : ℚ := -((2:ℚ)^(a+1)/((2:ℚ)^j-poleQ i))

theorem actual_anchor_chart_slope_even (a i j : ℕ) (hi : i < a) (hj : j < a) :
    VBound 1 (chartAnchorSlope a i j) := by
  have hv : padicValRat 2 ((2:ℚ)^j-poleQ i) ≤ (a:ℤ) := by
    rw [poleQ,sub_neg_eq_add,sum_two_power_valuation]
    split_ifs <;> omega
  have hh := vbound_div (power_integral (a+1)) (r:=(2:ℚ)^j-poleQ i)
  exact (hh.mono (by omega)).neg

def chartUnit (a u i x y : ℕ) (m : ℕ → ℕ) : ℚ[X] :=
  ∏ j ∈ mask u x y,(1+C (chartAnchorSlope a i j)*X)^m j

theorem actual_chart_unit_weighted (a u i x y : ℕ) (m : ℕ → ℕ)
    (hi : i < u) (hu : u ≤ a) : WeightedPolynomial (chartUnit a u i x y m) := by
  apply WeightedPolynomial.prod
  intro j hj
  have hju : j < u := mem_range.mp (mem_filter.mp hj).1
  exact (weighted_linear (actual_anchor_chart_slope_even a i j (by omega) (by omega))).pow (m j)

theorem actual_chart_unit_constant (a u i x y : ℕ) (m : ℕ → ℕ) :
    (chartUnit a u i x y m).coeff 0=1 := by
  rw [coeff_zero_eq_eval_zero]
  simp only [chartUnit,eval_prod,eval_pow,eval_add,eval_one,eval_mul,eval_C,eval_X,
    mul_zero,add_zero,one_pow,prod_const_one]

/-- Actual anchor chart as its actual value times a constructed weighted unit. -/
theorem actual_factor_chart_unit (a u i x y j : ℕ) (hj : j ∈ mask u x y) :
    chart a (pole i) (factor (delta x y) j) =
      C ((factor (delta x y) j).eval (poleQ i))*(1+C (chartAnchorSlope a i j)*X) := by
  apply Polynomial.funext
  intro z
  have hd := denominator_ne_zero u x y j hj
  have hn : (2:ℚ)^j-poleQ i ≠ 0 := by rw [poleQ,sub_neg_eq_add]; positivity
  simp only [chart,eval_comp,eval_add,eval_C,eval_mul,eval_X,factor_eval,pole_cast,eval_one]
  unfold chartAnchorSlope
  field_simp [hd,hn]
  ring

theorem actual_correction_chart_unit (a u i x y : ℕ) (m : ℕ → ℕ) :
    chart a (pole i) (correction u x y m) =
      C ((correction u x y m).eval (poleQ i))*chartUnit a u i x y m := by
  have hc : (∏ j ∈ mask u x y,(C ((factor (delta x y) j).eval (poleQ i)):ℚ[X])^m j) =
      C ((correction u x y m).eval (poleQ i)) := by
    simp only [correction,eval_prod,eval_pow,← map_pow,← map_prod]
  calc
    _ = ∏ j ∈ mask u x y,(chart a (pole i) (factor (delta x y) j))^m j := by
      simp only [chart,correction,Polynomial.prod_comp,pow_comp]
    _ = ∏ j ∈ mask u x y,
        (C ((factor (delta x y) j).eval (poleQ i))*(1+C (chartAnchorSlope a i j)*X))^m j := by
      apply prod_congr rfl
      intro j hj
      rw [actual_factor_chart_unit a u i x y j hj]
    _ = _ := by
      simp only [mul_pow,prod_mul_distrib]
      rw [hc]
      rfl


/-- The actual whole crossing chart has a constructed degree-weighted unit factor. -/
theorem actual_completed_chart_unit (a u i x y : ℕ) (M : ℕ → ℕ)
    (hx : x < 2^a) (hy : y < 2^a) (hi : i < u) (hu : u ≤ a)
    (hc : x.testBit i = y.testBit i) :
    ∃ U : ℚ[X],WeightedPolynomial U ∧ U.coeff 0=1 ∧
      chart a (pole i) (completed (grid a) u x y M) = C (firstJet a u i x y M)*X*U := by
  have ht := pole_member a i (by omega)
  have hd := delta_in_grid a x y hx hy
  rw [grid_eq_image] at ht hd
  obtain ⟨m,_,hm⟩ := mem_image.mp ht
  obtain ⟨s,_,hs⟩ := mem_image.mp hd
  have hsm : s ≠ m := by
    intro he
    subst s
    exact crossing_displacement_ne_pole x y i hc (hs.symm.trans hm)
  let R : ℚ[X] := (restPolynomial a m s).map LocalIntegral.integralRing.subtype
  let U : ℚ[X] := R*chartUnit a u i x y M
  let q : ℚ := ((LocalIntegral.rhoUnit a m:LocalIntegral.integralRing):ℚ)*
    ((LocalIntegral.chartRatio a m s:LocalIntegral.integralRing):ℚ)
  have hw : WeightedPolynomial U := (actual_rest_weighted a m s).mul
    (actual_chart_unit_weighted a u i x y M hi hu)
  have hr0 : R.eval 0=1 := by rw [← coeff_zero_eq_eval_zero]; exact actual_rest_constant a m s
  have hu0 : (chartUnit a u i x y M).eval 0=1 := by
    rw [← coeff_zero_eq_eval_zero]
    exact actual_chart_unit_constant a u i x y M
  have hU0 : U.coeff 0=1 := by
    change (R*chartUnit a u i x y M).coeff 0=1
    rw [coeff_zero_eq_eval_zero,eval_mul,hr0,hu0,one_mul]
  have hb : chart a (pole i) (base (grid a) (delta x y)) = C q*X*R := by
    have hh := mapped_localBase_factor a m s hsm
    rw [chart_polynomial,hm,hs] at hh
    have hh' : chart a (pole i) (base (grid a) (delta x y)) =
        C ((LocalIntegral.rhoUnit a m:LocalIntegral.integralRing):ℚ)*
          (C ((LocalIntegral.chartRatio a m s:LocalIntegral.integralRing):ℚ)*X)*R := by
      simpa only [chart,pole_cast] using hh
    rw [hh']
    dsimp [q]
    rw [map_mul]
    ring
  have hfull : chart a (pole i) (completed (grid a) u x y M) =
      C (q*(correction u x y M).eval (poleQ i))*X*U := by
    have hm : chart a (pole i) (completed (grid a) u x y M) =
        chart a (pole i) (base (grid a) (delta x y))*chart a (pole i) (correction u x y M) := by
      simp only [chart,completed,mul_comp]
    rw [hm,hb,actual_correction_chart_unit]
    dsimp [U,q]
    simp only [map_mul]
    ring
  have hJ : firstJet a u i x y M = q*(correction u x y M).eval (poleQ i) := by
    have hh := congrArg (fun P : ℚ[X] => P.coeff 1) hfull
    simpa only [firstJet,mul_assoc,Polynomial.coeff_C_mul,Polynomial.coeff_X_mul,hU0,mul_one] using hh
  refine ⟨U,hw,hU0,?_⟩
  rw [hJ]
  exact hfull


def normalizedEntry (a u i x y : ℕ) (M : ℕ → ℕ) : ℚ[X] :=
  C ((diagonalJet a u i M)⁻¹)*chart a (pole i) (completed (grid a) u x y M)

theorem actual_first_ratio_integral (a u i x y : ℕ) (M : ℕ → ℕ)
    (hx : x < 2^a) (hy : y < 2^a) (hi : i < u) (hu : u ≤ a)
    (hc : x.testBit i = y.testBit i) :
    VBound 0 (firstJet a u i x y M/diagonalJet a u i M) := by
  have hh := normalized_crossing_chart_integral a u i x y M hx hy hi hu hc 1
  rw [Polynomial.coeff_C_mul] at hh
  change VBound 0 ((diagonalJet a u i M)⁻¹*firstJet a u i x y M) at hh
  simpa only [div_eq_mul_inv,mul_comm] using hh

theorem actual_normalized_coefficient_weight (a u i x y : ℕ) (M : ℕ → ℕ)
    (hx : x < 2^a) (hy : y < 2^a) (hi : i < u) (hu : u ≤ a)
    (hc : x.testBit i = y.testBit i) (n : ℕ) :
    VBound (n:ℤ) ((normalizedEntry a u i x y M).coeff (n+1)) := by
  obtain ⟨U,hU,hU0,hP⟩ := actual_completed_chart_unit a u i x y M hx hy hi hu hc
  have hR := actual_first_ratio_integral a u i x y M hx hy hi hu hc
  have he : (normalizedEntry a u i x y M).coeff (n+1) =
      (firstJet a u i x y M/diagonalJet a u i M)*U.coeff n := by
    rw [normalizedEntry,hP]
    simp only [mul_assoc,Polynomial.coeff_C_mul,Polynomial.coeff_X_mul]
    ring
  rw [he]
  simpa only [zero_add] using hR.mul (hU n)

theorem actual_normalized_constant (a u i x y : ℕ) (M : ℕ → ℕ)
    (hx : x < 2^a) (hy : y < 2^a) (hi : i < u) (hu : u ≤ a)
    (hc : x.testBit i = y.testBit i) : (normalizedEntry a u i x y M).coeff 0=0 := by
  obtain ⟨U,hU,hU0,hP⟩ := actual_completed_chart_unit a u i x y M hx hy hi hu hc
  rw [normalizedEntry,hP,coeff_zero_eq_eval_zero]
  simp only [eval_mul,eval_C,eval_X,mul_zero,zero_mul]

theorem actual_normalized_first (a u i x y : ℕ) (M : ℕ → ℕ) :
    (normalizedEntry a u i x y M).coeff 1 = firstJet a u i x y M/diagonalJet a u i M := by
  simp only [normalizedEntry,Polynomial.coeff_C_mul,firstJet,div_eq_mul_inv]
  ring

/-- All higher coefficients vanish in the same lowest normalized two-adic layer. -/
theorem actual_whole_chart_first_layer (a u i x y : ℕ) (M : ℕ → ℕ)
    (hx : x < 2^a) (hy : y < 2^a) (hi : i < u) (hu : u ≤ a)
    (hc : x.testBit i = y.testBit i) :
    Content 1 (normalizedEntry a u i x y M-C (firstJet a u i x y M/diagonalJet a u i M)*X) := by
  intro n
  rcases n with _ | n
  · rw [coeff_sub,actual_normalized_constant a u i x y M hx hy hi hu hc]
    simp [VBound]
  · by_cases hn : n=0
    · subst n
      rw [coeff_sub,actual_normalized_first]
      simp [VBound]
    · have hz : (C (firstJet a u i x y M/diagonalJet a u i M)*X).coeff (n+1)=0 := by
        simp only [Polynomial.coeff_C_mul,Polynomial.coeff_X,
          show ¬(1:ℕ)=n+1 by omega,ite_false,mul_zero]
      rw [coeff_sub,hz,sub_zero]
      exact (actual_normalized_coefficient_weight a u i x y M hx hy hi hu hc n).mono (by omega)


def normalizedChartMatrix (a u i : ℕ) (M : ℕ → ℕ) : Matrix (Ports a) (Ports a) ℚ[X] :=
  fun x y => if x.val.testBit i=y.val.testBit i then normalizedEntry a u i x.val y.val M else 0

theorem actual_chart_to_first (a u i : ℕ) (M : ℕ → ℕ)
    (hi : i < u) (hu : u ≤ a) (x y : Ports a) :
    Content 1 (normalizedChartMatrix a u i M x y-C (normalizedCrossing a u i M x y)*X) := by
  unfold normalizedChartMatrix normalizedCrossing
  split_ifs with hc
  · exact actual_whole_chart_first_layer a u i x.val y.val M x.isLt y.isLt hi hu hc
  · intro n
    simp [VBound]

theorem actual_chart_leading_product (a u i : ℕ) (M : ℕ → ℕ)
    (hi : i < u) (hu : u ≤ a) (hM : ∀ j < u,0 < M j) (x y : Ports a) :
    Content 1 (normalizedChartMatrix a u i M x y-C ((leftFactor a u*rightFactor a u) x y)*X) := by
  have hp := actual_chart_to_first a u i M hi hu x y
  have hR := actual_leading_factorization a u i M hi hu hM x y
  have hc : Content 1 (C (normalizedCrossing a u i M x y-(leftFactor a u*rightFactor a u) x y)*X) := by
    simpa only [add_zero] using (Content.C hR).mul Content.X
  have hh := hp.add hc
  have he : normalizedChartMatrix a u i M x y-C (normalizedCrossing a u i M x y)*X+
      C (normalizedCrossing a u i M x y-(leftFactor a u*rightFactor a u) x y)*X =
      normalizedChartMatrix a u i M x y-C ((leftFactor a u*rightFactor a u) x y)*X := by
    rw [map_sub]
    ring
  rwa [he] at hh

def chartLeft (a u : ℕ) : Matrix (Ports a) (Labels u) ℚ[X] :=
  fun x r => C (leftFactor a u x r)

def chartRight (a u : ℕ) : Matrix (Labels u) (Ports a) ℚ[X] :=
  fun r y => C (rightFactor a u r y)*X

theorem chart_product (a u : ℕ) (x y : Ports a) :
    (chartLeft a u*chartRight a u) x y=C ((leftFactor a u*rightFactor a u) x y)*X := by
  simp only [Matrix.mul_apply,chartLeft,chartRight]
  calc
    _ = ∑ r : Labels u,C (leftFactor a u x r*rightFactor a u r y)*X := by
      apply sum_congr rfl
      intro r _
      rw [map_mul]
      ring
    _ = (∑ r : Labels u,C (leftFactor a u x r*rightFactor a u r y))*X := (sum_mul _ _ _).symm
    _ = _ := by rw [map_sum]

theorem chart_left_integral (a u : ℕ) (x : Ports a) (r : Labels u) :
    Content 0 (chartLeft a u x r) := Content.C (left_integral a u x r)

theorem chart_right_integral (a u : ℕ) (r : Labels u) (y : Ports a) :
    Content 0 (chartRight a u r y) := by
  unfold chartRight
  simpa only [add_zero] using (Content.C (right_integral a u r y)).mul Content.X

/-- Explicit polynomial matrix factors for the whole actual lowest chart layer. -/
theorem actual_chart_leading_factorization (a u i : ℕ) (M : ℕ → ℕ)
    (hi : i < u) (hu : u ≤ a) (hM : ∀ j < u,0 < M j) (x y : Ports a) :
    Content 1 (normalizedChartMatrix a u i M x y-(chartLeft a u*chartRight a u) x y) := by
  rw [chart_product]
  exact actual_chart_leading_product a u i M hi hu hM x y

def crossingChartMatrix (a u i : ℕ) (M : ℕ → ℕ) : Matrix (Ports a) (Ports a) ℚ[X] :=
  fun x y => if x.val.testBit i=y.val.testBit i then
    chart a (pole i) (completed (grid a) u x.val y.val M) else 0

def chartWeightedLeft (a u i : ℕ) (M : ℕ → ℕ) : Matrix (Ports a) (Labels u) ℚ[X] :=
  fun x r => C (diagonalJet a u i M)*chartLeft a u x r

theorem crossing_chart_scale (a u i : ℕ) (M : ℕ → ℕ) (hia : i ≤ a) (x y : Ports a) :
    crossingChartMatrix a u i M x y=C (diagonalJet a u i M)*normalizedChartMatrix a u i M x y := by
  unfold crossingChartMatrix normalizedChartMatrix
  split_ifs
  · rw [normalizedEntry,← mul_assoc,← map_mul,mul_inv_cancel₀ (diagonalJet_ne_zero a u i M hia),map_one,one_mul]
  · simp

theorem chart_weighted_product (a u i : ℕ) (M : ℕ → ℕ) (x y : Ports a) :
    (chartWeightedLeft a u i M*chartRight a u) x y=
      C (diagonalJet a u i M)*(chartLeft a u*chartRight a u) x y := by
  simp only [Matrix.mul_apply,chartWeightedLeft]
  rw [mul_sum]
  apply sum_congr rfl
  intro r _
  ring

theorem chart_weighted_left_bound (a u i : ℕ) (M : ℕ → ℕ)
    (hi : i < u) (hu : u ≤ a) (x : Ports a) (r : Labels u) :
    Content (jetWeight a u i M) (chartWeightedLeft a u i M x r) := by
  unfold chartWeightedLeft
  have hc : VBound (jetWeight a u i M) (diagonalJet a u i M) := Or.inr (by
    rw [diagonalJet_valuation a u i M hi hu]
    rfl)
  simpa only [add_zero] using (Content.C hc).mul (chart_left_integral a u x r)

theorem actual_weighted_chart_leading_factorization (a u i : ℕ) (M : ℕ → ℕ)
    (hi : i < u) (hu : u ≤ a) (hM : ∀ j < u,0 < M j) (x y : Ports a) :
    Content (jetWeight a u i M+1)
      (crossingChartMatrix a u i M x y-(chartWeightedLeft a u i M*chartRight a u) x y) := by
  have hc : VBound (jetWeight a u i M) (diagonalJet a u i M) := Or.inr (by
    rw [diagonalJet_valuation a u i M hi hu]
    rfl)
  have hh := (Content.C hc).mul (actual_chart_leading_factorization a u i M hi hu hM x y)
  rw [crossing_chart_scale a u i M (by omega),chart_weighted_product]
  convert hh using 1
  ring


theorem chart_weighted_left_integral (a u i : ℕ) (M : ℕ → ℕ)
    (hi : i < u) (hu : u ≤ a) (hw : 0 ≤ jetWeight a u i M)
    (x : Ports a) (r : Labels u) : Content 0 (chartWeightedLeft a u i M x r) :=
  (chart_weighted_left_bound a u i M hi hu x r).mono hw

end OrientationFullChart
