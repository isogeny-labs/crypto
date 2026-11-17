module
public import OrientationCompact
public import OrientationNode
public import OrientationHigher
import all OrientationCompact
import all OrientationNode
import all OrientationHigher
import all OrientationMoment
import all OrientationContent
public section
noncomputable section
namespace OrientationChartDegree
open Polynomial Finset OrientationAnchor OrientationMoment OrientationNodePolynomial
  OrientationNodeMoment OrientationGradedNode OrientationRefinement OrientationLeading
  OrientationDiagonal OrientationJet OrientationResidue OrientationBivariate

theorem graded_zero (s : ℤ) : Graded s (0:ℚ[X]) := by
  intro n
  simp only [coeff_zero]
  exact VBound.zero _

theorem graded_sub {s : ℤ} {P Q : ℚ[X]} (hP : Graded s P) (hQ : Graded s Q) :
    Graded s (P-Q) := by
  intro n
  rw [coeff_sub]
  simpa only [sub_eq_add_neg] using (hP n).add (hQ n).neg

def Mixed (s : ℤ) (P : (ℚ[X])[X]) : Prop := ∀ b : ℕ,Graded (s+b) (P.coeff b)

theorem Mixed.C {s : ℤ} {P : ℚ[X]} (hP : Graded s P) : Mixed s (C P) := by
  intro b
  by_cases hb : b=0
  · subst b; simpa using hP
  · simp only [coeff_C,hb,ite_false]
    exact graded_zero _

theorem Mixed.one : Mixed 0 (1:(ℚ[X])[X]) := by
  simpa using Mixed.C Graded.one

theorem Mixed.add {s : ℤ} {P Q : (ℚ[X])[X]} (hP : Mixed s P) (hQ : Mixed s Q) :
    Mixed s (P+Q) := by
  intro b
  rw [coeff_add]
  exact (hP b).add (hQ b)

theorem Mixed.sub {s : ℤ} {P Q : (ℚ[X])[X]} (hP : Mixed s P) (hQ : Mixed s Q) :
    Mixed s (P-Q) := by
  intro b
  rw [coeff_sub]
  exact graded_sub (hP b) (hQ b)

theorem Mixed.mul {s t : ℤ} {P Q : (ℚ[X])[X]} (hP : Mixed s P) (hQ : Mixed t Q) :
    Mixed (s+t) (P*Q) := by
  intro b
  rw [coeff_mul]
  apply Graded.sum
  intro ij hij
  have hs := mem_antidiagonal.mp hij
  have he : (s+(ij.1:ℤ))+(t+ij.2)=(s+t)+b := by omega
  simpa only [he] using (hP ij.1).mul (hQ ij.2)

theorem Mixed.pow {P : (ℚ[X])[X]} (hP : Mixed 0 P) (n : ℕ) : Mixed 0 (P^n) := by
  induction n with
  | zero => simpa using Mixed.one
  | succ n ih => simpa only [pow_succ,zero_add] using ih.mul hP

theorem Mixed.prod {ι : Type*} (S : Finset ι) (P : ι → (ℚ[X])[X])
    (hP : ∀ j ∈ S,Mixed 0 (P j)) : Mixed 0 (∏ j ∈ S,P j) := by
  classical
  induction S using Finset.induction_on with
  | empty => simpa using Mixed.one
  | @insert j S hj ih =>
    rw [prod_insert hj]
    simpa only [zero_add] using (hP j (mem_insert_self _ _)).mul
      (ih (by intro z hz; exact hP z (mem_insert_of_mem hz)))

theorem Mixed.sum {ι : Type*} (S : Finset ι) (P : ι → (ℚ[X])[X]) (s : ℤ)
    (hP : ∀ j ∈ S,Mixed s (P j)) : Mixed s (∑ j ∈ S,P j) := by
  intro b
  rw [finsetSum_coeff]
  exact Graded.sum S (fun j => (P j).coeff b) (s+b) (fun j hj => hP j hj b)

theorem mixed_affine (A B : ℚ[X]) (hA : Graded 0 A) (hB : Graded 1 B) :
    Mixed 0 (C A+C B*X) := by
  intro b
  by_cases h0 : b=0
  · subst b; simpa using hA
  by_cases h1 : b=1
  · subst b; simpa using hB
  · simp only [coeff_add,coeff_C,coeff_C_mul,coeff_X,h0,Ne.symm h1,ite_false,mul_zero,add_zero]
    exact graded_zero _

theorem anchor_rows_mixed (u x y : ℕ) (m : ℕ → ℕ) : Mixed 0 (anchorRows u x y m) := by
  apply Mixed.prod
  intro j hj
  apply Mixed.pow
  exact mixed_affine 1 _ Graded.one (Graded.C (anchor_slope_even u x y j hj))

theorem chart_linear_graded (a u i x y : ℕ) (t : ℤ)
    (hiu : i<u) (hu : u≤a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) : Graded 0 (1+C (chartSlope a u x y t)*X) := by
  have hSlope := chart_slope_even a u i x y t hiu hu ht0 hi hc
  have hlin : Graded 0 (C (chartSlope a u x y t)*X) := by
    simpa using (Graded.C hSlope).mul Graded.X
  exact Graded.one.add hlin

theorem denom_rows_mixed (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hiu : i<u) (hu : u≤a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) : Mixed 0 (denomRows a u x y t m) := by
  have hA := chart_linear_graded a u i x y t hiu hu ht0 hi hc
  have hB := Graded.C (node_slope_even u i x y t hiu ht0 hi hc)
  simpa only [zero_add,denomRows] using (mixed_affine _ _ hA hB).mul (anchor_rows_mixed u x y m)

theorem numerator_graded (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hy : y<2^a) (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0)
    (hi : padicValInt 2 t=i) (hiu : i<u) (hu : u≤a)
    (hc : x.testBit i=y.testBit i) : Graded (-1) (rowNumerator a u i x y t m) := by
  have hlin := chart_linear_graded a u i x y t hiu hu ht0 hi hc
  have hp := actual_normalized_graded a u i (rowResidue u x) y t m
    (row_source_bound a u x hu) hy ht ht0 hi hiu hu (row_crossing u i x y hiu hc)
  simpa only [rowNumerator,zero_add] using hlin.mul hp

theorem geometric_rows_mixed (D : (ℚ[X])[X]) (k : ℕ) (hD : Mixed 0 D) :
    Mixed 0 (geometricRows D k) :=
  Mixed.sum _ _ 0 (fun n _ => (Mixed.one.sub hD).pow n)

theorem actual_approx_rows_mixed (a u i x y k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hy : y<2^a) (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0)
    (hi : padicValInt 2 t=i) (hiu : i<u) (hu : u≤a)
    (hc : x.testBit i=y.testBit i) : Mixed (-1) (approxRows a u i x y k t m) := by
  have hN := Mixed.C (numerator_graded a u i x y t m hy ht ht0 hi hiu hu hc)
  have hD := denom_rows_mixed a u i x y t m hiu hu ht0 hi hc
  simpa only [approxRows,add_zero] using hN.mul (geometric_rows_mixed _ k hD)

theorem actual_poly_right_graded (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hiu : i<u) (hu : u≤a) (rb : MomentLabels u k) (y : Ports a) :
    Graded (-1) (polyRight a u i k t m rb y) := by
  unfold polyRight
  split_ifs with hc
  · have hA := actual_approx_rows_mixed a u i rb.1.val y.val k t m y.isLt ht ht0 hi hiu hu hc rb.2.val
    have hv : VBound (-(rb.2.val:ℤ)) (((2:ℚ)^rb.2.val)⁻¹) := Or.inr (by
      rw [padicValRat.inv,OrientationResidue.two_power_valuation])
    have he : -(rb.2.val:ℤ)+(-1+rb.2.val)=(-1:ℤ) := by omega
    simpa only [he] using (Graded.C hv).mul hA
  · exact graded_zero _

def trimZ (k : ℕ) (P : ℚ[X]) : ℚ[X] := PowerSeries.trunc (k+1) P.toPowerSeries

theorem trimZ_error (P : ℚ[X]) (hP : Graded (-1) P) (k : ℕ) :
    Content (k:ℤ) (P-trimZ k P) := by
  intro n
  rw [coeff_sub,trimZ,PowerSeries.coeff_trunc]
  split_ifs with hn
  · simp only [Polynomial.coeff_coe,sub_self]
    exact VBound.zero _
  · simp only [sub_zero]
    exact (hP n).mono (by omega)

theorem trimZ_integral (P : ℚ[X]) (hP : Content 0 P) (k : ℕ) : Content 0 (trimZ k P) := by
  intro n
  rw [trimZ,PowerSeries.coeff_trunc]
  split_ifs
  · simpa only [Polynomial.coeff_coe] using hP n
  · exact VBound.zero _

theorem trimZ_degree (P : ℚ[X]) (k : ℕ) : (trimZ k P).natDegree≤k :=
  Nat.le_of_lt_succ (PowerSeries.natDegree_trunc_lt P.toPowerSeries k)

def trimmedRight (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ) :
    Matrix (MomentLabels u k) (Ports a) ℚ[X] :=
  fun rb y => trimZ k (polyRight a u i k t m rb y)

theorem trimmed_right_integral (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hiu : i<u) (hu : u≤a) (rb : MomentLabels u k) (y : Ports a) :
    Content 0 (trimmedRight a u i k t m rb y) :=
  trimZ_integral _ (poly_right_integral a u i k t m ht ht0 hi hiu hu rb y) k

theorem trimmed_right_degree (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (rb : MomentLabels u k) (y : Ports a) : (trimmedRight a u i k t m rb y).natDegree≤k :=
  trimZ_degree _ k

theorem trimmed_right_error (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hiu : i<u) (hu : u≤a) (rb : MomentLabels u k) (y : Ports a) :
    Content (k:ℤ) (polyRight a u i k t m rb y-trimmedRight a u i k t m rb y) :=
  trimZ_error _ (actual_poly_right_graded a u i k t m ht ht0 hi hiu hu rb y) k

theorem poly_left_error_product (a u k : ℕ)
    (R S : Matrix (MomentLabels u k) (Ports a) ℚ[X])
    (h : ∀ rb y,Content (k:ℤ) (R rb y-S rb y)) (x y : Ports a) :
    Content (k:ℤ) ((polyLeft a u k*R) x y-(polyLeft a u k*S) x y) := by
  rw [Matrix.mul_apply,Matrix.mul_apply,← Finset.sum_sub_distrib]
  apply content_sum
  intro rb _
  have hL := (poly_left_weight a u k x rb).mono (show (0:ℤ)≤rb.2.val by omega)
  simpa only [zero_add,mul_sub] using hL.mul (h rb y)

theorem actual_degree_limited_precision (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hiu : i<u) (hu : u≤a) (x y : Ports a) :
    Content (k:ℤ) (nodePolynomialMatrix a u i t m x y-
      (polyLeft a u k*trimmedRight a u i k t m) x y) := by
  have hOld := actual_compact_poly_precision a u i k t m ht ht0 hi hiu hu x y
  have hTrim := poly_left_error_product a u k _ _
    (trimmed_right_error a u i k t m ht ht0 hi hiu hu) x y
  convert hOld.add hTrim using 1; ring

theorem weighted_trim_product (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ) (x y : Ports a) :
    (weightedLeft a u i k m*trimmedRight a u i k t m) x y=
      C (diagonalJet a u i m)*(polyLeft a u k*trimmedRight a u i k t m) x y := by
  simp only [Matrix.mul_apply,OrientationBivariate.weightedLeft]
  rw [mul_sum]
  apply sum_congr rfl
  intro rb _
  ring

theorem actual_degree_limited_weighted_precision (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hiu : i<u) (hu : u≤a) (x y : Ports a) :
    Content (jetWeight a u i m+k) (nodeCrossingPolynomial a u i t m x y-
      (weightedLeft a u i k m*trimmedRight a u i k t m) x y) := by
  have hv : VBound (jetWeight a u i m) (diagonalJet a u i m) := Or.inr (by
    rw [diagonalJet_valuation a u i m hiu hu]; rfl)
  have he := (Content.C hv).mul (actual_degree_limited_precision a u i k t m ht ht0 hi hiu hu x y)
  rw [node_crossing_scale a u i t m (by omega),weighted_trim_product]
  simpa only [mul_sub] using he

theorem actual_degree_limited_profile (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hiu : i<u) (hu : u≤a) :
    (∀ (x : Ports a) (rb : MomentLabels u k),
      Content (jetWeight a u i m+rb.2.val) (weightedLeft a u i k m x rb)) ∧
    (∀ (rb : MomentLabels u k) (y : Ports a),Content 0 (trimmedRight a u i k t m rb y)) ∧
    (∀ (rb : MomentLabels u k) (y : Ports a),(trimmedRight a u i k t m rb y).natDegree≤k) ∧
    (∀ x y : Ports a,Content (jetWeight a u i m+k) (nodeCrossingPolynomial a u i t m x y-
      (weightedLeft a u i k m*trimmedRight a u i k t m) x y)) ∧
    Fintype.card (MomentLabels u k)=2^u*k := by
  exact ⟨weighted_left_weight a u i k m hiu hu,
    trimmed_right_integral a u i k t m ht ht0 hi hiu hu,
    trimmed_right_degree a u i k t m,
    actual_degree_limited_weighted_precision a u i k t m ht ht0 hi hiu hu,
    moment_label_count u k⟩

end OrientationChartDegree

#print axioms OrientationChartDegree.actual_approx_rows_mixed
#print axioms OrientationChartDegree.actual_poly_right_graded
#print axioms OrientationChartDegree.trimmed_right_degree
#print axioms OrientationChartDegree.actual_degree_limited_precision
#print axioms OrientationChartDegree.actual_degree_limited_profile
