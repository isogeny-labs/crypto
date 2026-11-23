module
public import OrientationHigher
public import OrientationNode
public import OrientationMoment
public import OrientationContent
import all OrientationHigher
import all OrientationNode
import all OrientationMoment
import all OrientationContent
public section
noncomputable section
namespace OrientationBivariate
open Polynomial Finset OrientationAnchor OrientationMoment OrientationNodePolynomial OrientationNodeMoment OrientationGradedNode OrientationRefinement OrientationLeading OrientationDiagonal OrientationJet

theorem content_zero (k : ℤ) : Content k (0:ℚ[X]) := by
  intro n
  simp only [coeff_zero]
  exact VBound.zero _

def Weighted (P : (ℚ[X])[X]) : Prop := ∀ b : ℕ,Content (b:ℤ) (P.coeff b)

theorem Weighted.one : Weighted (1:(ℚ[X])[X]) := by
  intro b
  by_cases hb : b=0
  · subst b; simpa using Content.one
  · simp only [coeff_one,hb,ite_false]
    exact content_zero _

theorem Weighted.C {P : ℚ[X]} (hP : Content 0 P) : Weighted (C P) := by
  intro b
  by_cases hb : b=0
  · subst b; simpa using hP
  · simp only [coeff_C,hb,ite_false]
    exact content_zero _

theorem Weighted.add {P Q : (ℚ[X])[X]} (hP : Weighted P) (hQ : Weighted Q) :
    Weighted (P+Q) := by
  intro b
  rw [coeff_add]
  exact (hP b).add (hQ b)

theorem Weighted.sub {P Q : (ℚ[X])[X]} (hP : Weighted P) (hQ : Weighted Q) :
    Weighted (P-Q) := by
  intro b
  rw [coeff_sub]
  exact content_sub (hP b) (hQ b)

theorem Weighted.mul {P Q : (ℚ[X])[X]} (hP : Weighted P) (hQ : Weighted Q) :
    Weighted (P*Q) := by
  intro b
  rw [coeff_mul]
  apply content_sum
  intro ij hij
  have hs := mem_antidiagonal.mp hij
  simpa only [← Nat.cast_add,hs] using (hP ij.1).mul (hQ ij.2)

theorem Weighted.pow {P : (ℚ[X])[X]} (hP : Weighted P) (n : ℕ) : Weighted (P^n) := by
  induction n with
  | zero => simpa using Weighted.one
  | succ n ih => simpa only [pow_succ] using ih.mul hP

theorem Weighted.prod {ι : Type*} (S : Finset ι) (P : ι → (ℚ[X])[X])
    (hP : ∀ j ∈ S,Weighted (P j)) : Weighted (∏ j ∈ S,P j) := by
  classical
  induction S using Finset.induction_on with
  | empty => simpa using Weighted.one
  | @insert j S hj ih =>
    rw [prod_insert hj]
    exact (hP j (mem_insert_self _ _)).mul
      (ih (by intro z hz; exact hP z (mem_insert_of_mem hz)))

theorem Weighted.sum {ι : Type*} (S : Finset ι) (P : ι → (ℚ[X])[X])
    (hP : ∀ j ∈ S,Weighted (P j)) : Weighted (∑ j ∈ S,P j) := by
  intro b
  rw [finsetSum_coeff]
  exact content_sum S (fun j => (P j).coeff b) b (fun j hj => hP j hj b)

theorem weighted_affine (A B : ℚ[X]) (hA : Content 0 A) (hB : Content 1 B) :
    Weighted (C A+C B*X) := by
  intro b
  by_cases h0 : b=0
  · subst b; simpa using hA
  by_cases h1 : b=1
  · subst b; simpa using hB
  · simp only [coeff_add,coeff_C,coeff_C_mul,coeff_X,h0,Ne.symm h1,ite_false,mul_zero,add_zero]
    exact content_zero _

def geometricRows (D : (ℚ[X])[X]) (k : ℕ) : (ℚ[X])[X] := ∑ n ∈ range k,(1-D)^n

theorem weighted_geometric (D : (ℚ[X])[X]) (k : ℕ) (hD : Weighted D) :
    Weighted (geometricRows D k) :=
  Weighted.sum _ _ (fun n _ => (Weighted.one.sub hD).pow n)

theorem eval_tail_content (P : (ℚ[X])[X]) (hP : Weighted P) (k : ℕ)
    (hz : ∀ n<k,P.coeff n=0) (z : ℤ) : Content (k:ℤ) (P.eval (C (z:ℚ))) := by
  rw [eval_eq_sum,Polynomial.sum_def]
  apply content_sum
  intro n _
  by_cases hn : n<k
  · rw [hz n hn,zero_mul]
    exact content_zero _
  · have hc := (hP n).mono (show (k:ℤ)≤n by omega)
    have hp : Content 0 ((C (z:ℚ):ℚ[X])^n) := by
      simpa only [map_pow] using Content.C (integral_power (OrientationRank.integer_integral z) n)
    simpa only [add_zero] using hc.mul hp

def lowRows (P : (ℚ[X])[X]) (k : ℕ) : (ℚ[X])[X] := PowerSeries.trunc k P.toPowerSeries

theorem weighted_low (P : (ℚ[X])[X]) (k : ℕ) (hP : Weighted P) : Weighted (lowRows P k) := by
  intro b
  rw [lowRows,PowerSeries.coeff_trunc]
  split_ifs
  · simpa only [Polynomial.coeff_coe] using hP b
  · exact content_zero _

theorem low_rows_error (P : (ℚ[X])[X]) (hP : Weighted P) (k : ℕ) (z : ℤ) :
    Content (k:ℤ) (P.eval (C (z:ℚ))-(lowRows P k).eval (C (z:ℚ))) := by
  have htail := hP.sub (weighted_low P k hP)
  have hz : ∀ n<k,(P-lowRows P k).coeff n=0 := by
    intro n hn
    simp only [coeff_sub,lowRows,PowerSeries.coeff_trunc,hn,ite_true,Polynomial.coeff_coe,sub_self]
  simpa only [eval_sub] using eval_tail_content (P-lowRows P k) htail k hz z

def anchorRows (u x y : ℕ) (m : ℕ → ℕ) : (ℚ[X])[X] :=
  ∏ j ∈ mask u x y,
    (1+C (C (slope u (rowResidue u x) y ((2:ℚ)^j)))*X)^m j

def denomRows (a u x y : ℕ) (t : ℤ) (m : ℕ → ℕ) : (ℚ[X])[X] :=
  (C (1+C (chartSlope a u x y t)*X)+
    C (C (slope u (rowResidue u x) y (t:ℚ)))*X)*anchorRows u x y m

def approxRows (a u i x y k : ℕ) (t : ℤ) (m : ℕ → ℕ) : (ℚ[X])[X] :=
  C (rowNumerator a u i x y t m)*geometricRows (denomRows a u x y t m) k

theorem anchor_rows_weighted (u x y : ℕ) (m : ℕ → ℕ) : Weighted (anchorRows u x y m) := by
  apply Weighted.prod
  intro j hj
  apply Weighted.pow
  exact weighted_affine 1 _ Content.one (Content.C (anchor_slope_even u x y j hj))

theorem denom_rows_weighted (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hiu : i<u) (hu : u≤a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) : Weighted (denomRows a u x y t m) := by
  have hs := (chart_slope_even a u i x y t hiu hu ht0 hi hc).mono (by norm_num : (0:ℤ)≤1)
  have hA : Content 0 (1+C (chartSlope a u x y t)*X) :=
    Content.one.add (by simpa only [add_zero] using (Content.C hs).mul Content.X)
  exact (weighted_affine _ _ hA (Content.C (node_slope_even u i x y t hiu ht0 hi hc))).mul
    (anchor_rows_weighted u x y m)

theorem approx_rows_weighted (a u i x y k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hy : y<2^a) (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0)
    (hi : padicValInt 2 t=i) (hiu : i<u) (hu : u≤a)
    (hc : x.testBit i=y.testBit i) : Weighted (approxRows a u i x y k t m) :=
  (Weighted.C (actual_row_numerator_integral a u i x y t m hy ht ht0 hi hiu hu hc)).mul
    (weighted_geometric _ k (denom_rows_weighted a u i x y t m hiu hu ht0 hi hc))

theorem anchor_ratio_inverse (u x y : ℕ) (m : ℕ → ℕ) :
    (correctionRatio u x (rowResidue u x) y m)⁻¹ =
      ∏ j ∈ mask u x y,
        (1+slope u (rowResidue u x) y ((2:ℚ)^j)*(rowQuotient u x:ℚ))^m j := by
  have hmask := masks_stable u u x (rowResidue u x) y le_rfl (row_mod u x)
  have hp : correctionRatio u x (rowResidue u x) y m =
      ∏ j ∈ mask u x y,
        ((1+slope u (rowResidue u x) y ((2:ℚ)^j)*(rowQuotient u x:ℚ))⁻¹)^m j := by
    unfold correctionRatio
    apply prod_congr rfl
    intro j hj
    rw [denominator_inverse u x y ((2:ℚ)^j)
      (denominator_ne_zero u (rowResidue u x) y j (hmask ▸ hj))]
  rw [hp]
  simp only [Finset.prod_inv_distrib,inv_pow,inv_inv]

theorem anchor_rows_eval (u x y : ℕ) (m : ℕ → ℕ) :
    (anchorRows u x y m).eval (C (rowQuotient u x:ℚ)) =
      C ((correctionRatio u x (rowResidue u x) y m)⁻¹) := by
  rw [anchor_ratio_inverse]
  simp only [anchorRows,eval_prod,eval_pow,eval_add,eval_one,eval_mul,eval_C,eval_X,
    map_prod,map_pow,map_add,map_one,map_mul]

theorem row_denominator_split (u x y : ℕ) (t : ℤ)
    (hd : (t:ℚ)-delta (rowResidue u x) y ≠ 0) :
    ((t:ℚ)-delta x y)/((t:ℚ)-delta (rowResidue u x) y)=
      1+slope u (rowResidue u x) y (t:ℚ)*(rowQuotient u x:ℚ) := by
  have hs := row_split u x
  unfold OrientationMoment.slope delta at *
  field_simp [hd]
  nlinarith [hs]

theorem denom_rows_eval (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hiu : i<u) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) :
    (denomRows a u x y t m).eval (C (rowQuotient u x:ℚ))=rowDenominator a u x y t m := by
  have hdr := sub_ne_zero.mpr (Ne.symm (actual_displacement_ne_node (rowResidue u x) y i t ht0 hi
    (row_crossing u i x y hiu hc)))
  rw [denomRows,eval_mul,anchor_rows_eval]
  simp only [eval_add,eval_C,eval_mul,eval_X]
  rw [rowDenominator,row_denominator_split u x y t hdr]
  simp only [map_add,map_one,map_mul]
  ring

theorem geometric_rows_eval (D : (ℚ[X])[X]) (k : ℕ) (h : ℚ[X]) :
    (geometricRows D k).eval h=geometricInverse (D.eval h) k := by
  simp only [geometricRows,geometricInverse,eval_finsetSum,eval_pow,eval_sub,eval_one]

theorem actual_approx_rows_eval (a u i x y k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hiu : i<u) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) :
    (approxRows a u i x y k t m).eval (C (rowQuotient u x:ℚ))=
      actualChartApproximation a u i x y k t m := by
  rw [approxRows,eval_mul,eval_C,geometric_rows_eval,
    denom_rows_eval a u i x y t m hiu ht0 hi hc]
  rfl

theorem actual_low_rows_precision (a u i x y k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hy : y<2^a) (ht : (t:ℚ) ∈ grid a)
    (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u) (hu : u≤a)
    (hc : x.testBit i=y.testBit i) :
    Content (k:ℤ) (normalizedNodeEntry a u i x y t m-
      (lowRows (approxRows a u i x y k t m) k).eval (C (rowQuotient u x:ℚ))) := by
  have hP := approx_rows_weighted a u i x y k t m hy ht ht0 hi hiu hu hc
  have he := low_rows_error (approxRows a u i x y k t m) hP k (rowQuotient u x:ℤ)
  have he' : Content (k:ℤ) (actualChartApproximation a u i x y k t m-
      (lowRows (approxRows a u i x y k t m) k).eval (C (rowQuotient u x:ℚ))) := by
    simpa only [Int.cast_natCast,actual_approx_rows_eval a u i x y k t m hiu ht0 hi hc] using he
  have hh := (actual_whole_chart_precision a u i x y k t m hx hy ht ht0 hi hiu hu hc).add he'
  convert hh using 1; ring

theorem anchor_rows_class (u x y : ℕ) (m : ℕ → ℕ) :
    anchorRows u x y m=anchorRows u (rowResidue u x) y m := by
  have hm := masks_stable u u x (rowResidue u x) y le_rfl (row_mod u x)
  unfold anchorRows
  rw [← hm]
  simp only [rowResidue,Nat.mod_mod]

theorem denom_rows_class (a u x y : ℕ) (t : ℤ) (m : ℕ → ℕ) :
    denomRows a u x y t m=denomRows a u (rowResidue u x) y t m := by
  unfold denomRows
  rw [anchor_rows_class u x y m]
  simp only [chartSlope,rowResidue,Nat.mod_mod]

theorem numerator_rows_class (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ) :
    rowNumerator a u i x y t m=rowNumerator a u i (rowResidue u x) y t m := by
  simp only [rowNumerator,chartSlope,rowResidue,Nat.mod_mod]

theorem approx_rows_class (a u i x y k : ℕ) (t : ℤ) (m : ℕ → ℕ) :
    approxRows a u i x y k t m=approxRows a u i (rowResidue u x) y k t m := by
  unfold approxRows
  rw [denom_rows_class a u x y t m,numerator_rows_class a u i x y t m]

theorem low_rows_eval (P : (ℚ[X])[X]) (k : ℕ) (h : ℚ[X]) :
    (lowRows P k).eval h=∑ b : Fin k,P.coeff b.val*h^b.val := by
  have ht := PowerSeries.eval₂_trunc_eq_sum_range h (RingHom.id ℚ[X]) k P.toPowerSeries
  have hs := Fin.sum_univ_eq_sum_range (fun b : ℕ => P.coeff b*h^b) k
  simpa only [lowRows,Polynomial.eval,RingHom.id_apply,Polynomial.coeff_coe,← hs] using ht

def polyLeft (a u k : ℕ) : Matrix (Ports a) (MomentLabels u k) ℚ[X] :=
  fun x rb => C (momentLeft a u k x rb)

def polyRight (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ) :
    Matrix (MomentLabels u k) (Ports a) ℚ[X] := fun rb y =>
  if rb.1.val.testBit i=y.val.testBit i then
    C (((2:ℚ)^rb.2.val)⁻¹)*(approxRows a u i rb.1.val y.val k t m).coeff rb.2.val else 0

theorem poly_left_weight (a u k : ℕ) (x : Ports a) (rb : MomentLabels u k) :
    Content (rb.2.val:ℤ) (polyLeft a u k x rb) := Content.C (moment_left_weight a u k x rb)

theorem poly_right_integral (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hiu : i<u) (hu : u≤a) (rb : MomentLabels u k) (y : Ports a) :
    Content 0 (polyRight a u i k t m rb y) := by
  unfold polyRight
  split_ifs with hc
  · have hb := approx_rows_weighted a u i rb.1.val y.val k t m y.isLt ht ht0 hi hiu hu hc rb.2.val
    have hv : VBound (-(rb.2.val:ℤ)) (((2:ℚ)^rb.2.val)⁻¹) := Or.inr (by
      rw [padicValRat.inv,OrientationResidue.two_power_valuation])
    simpa only [neg_add_cancel] using (Content.C hv).mul hb
  · exact content_zero _

theorem coefficient_moment_cancel (b : ℕ) (h : ℚ) (R : ℚ[X]) :
    C ((2:ℚ)^b*h^b)*(C (((2:ℚ)^b)⁻¹)*R)=R*(C h)^b := by
  rw [map_mul]
  calc
    _ = (C ((2:ℚ)^b)*C (((2:ℚ)^b)⁻¹))*R*C (h^b) := by ring
    _ = _ := by rw [← map_mul,mul_inv_cancel₀ (by positivity : (2:ℚ)^b ≠ 0),map_one,one_mul,map_pow]

theorem poly_product (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ) (x y : Ports a) :
    (polyLeft a u k*polyRight a u i k t m) x y =
      if (rowResidue u x.val).testBit i=y.val.testBit i then
        (lowRows (approxRows a u i (rowResidue u x.val) y.val k t m) k).eval
          (C (rowQuotient u x.val:ℚ)) else 0 := by
  let r0 : Labels u := ⟨rowResidue u x.val,row_bound u x.val⟩
  have hz (r : Labels u) (hr : r ≠ r0) (b : Fin k) : polyLeft a u k x (r,b)=0 := by
    unfold polyLeft momentLeft
    split_ifs with he
    · exact False.elim (hr (Fin.ext he.symm))
    · simp only [map_zero]
  have hs : (∑ r : Labels u,∑ b : Fin k,
      polyLeft a u k x (r,b)*polyRight a u i k t m (r,b) y) =
      ∑ b : Fin k,polyLeft a u k x (r0,b)*polyRight a u i k t m (r0,b) y :=
    Finset.sum_eq_single r0 (by intro r _ hr; simp only [hz r hr,zero_mul,sum_const_zero])
      (by intro h; exact False.elim (h (mem_univ _)))
  rw [Matrix.mul_apply,Fintype.sum_prod_type,hs]
  by_cases hc : (rowResidue u x.val).testBit i=y.val.testBit i
  · simp only [hc,ite_true,polyLeft,polyRight,momentLeft,r0,ite_true]
    rw [low_rows_eval]
    apply sum_congr rfl
    intro b _
    exact coefficient_moment_cancel b.val (rowQuotient u x.val:ℚ) _
  · simp only [hc,ite_false,polyRight,r0,mul_zero,sum_const_zero]

theorem actual_compact_poly_precision (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hiu : i<u) (hu : u≤a) (x y : Ports a) :
    Content (k:ℤ) (nodePolynomialMatrix a u i t m x y-
      (polyLeft a u k*polyRight a u i k t m) x y) := by
  rw [poly_product]
  have hb := matching_bits u x.val (rowResidue u x.val) i hiu (row_mod u x.val)
  by_cases hc : x.val.testBit i=y.val.testBit i
  · have hr : (rowResidue u x.val).testBit i=y.val.testBit i := hb.symm.trans hc
    simp only [nodePolynomialMatrix,hc,hr,ite_true]
    rw [← approx_rows_class a u i x.val y.val k t m]
    exact actual_low_rows_precision a u i x.val y.val k t m x.isLt y.isLt ht ht0 hi hiu hu hc
  · have hr : (rowResidue u x.val).testBit i ≠ y.val.testBit i := by rwa [← hb]
    simp only [nodePolynomialMatrix,hc,hr,ite_false,sub_self]
    exact content_zero _

def weightedLeft (a u i k : ℕ) (m : ℕ → ℕ) : Matrix (Ports a) (MomentLabels u k) ℚ[X] :=
  fun x rb => C (diagonalJet a u i m)*polyLeft a u k x rb

theorem weighted_left_weight (a u i k : ℕ) (m : ℕ → ℕ)
    (hiu : i<u) (hu : u≤a) (x : Ports a) (rb : MomentLabels u k) :
    Content (jetWeight a u i m+rb.2.val) (weightedLeft a u i k m x rb) := by
  have hv : VBound (jetWeight a u i m) (diagonalJet a u i m) := Or.inr (by
    rw [diagonalJet_valuation a u i m hiu hu]; rfl)
  exact (Content.C hv).mul (poly_left_weight a u k x rb)

theorem weighted_left_integral (a u i k : ℕ) (m : ℕ → ℕ)
    (hiu : i<u) (hu : u≤a) (hw : 0≤jetWeight a u i m) (x : Ports a) (rb : MomentLabels u k) :
    Content 0 (weightedLeft a u i k m x rb) :=
  (weighted_left_weight a u i k m hiu hu x rb).mono (by omega)

theorem weighted_poly_product (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ) (x y : Ports a) :
    (weightedLeft a u i k m*polyRight a u i k t m) x y=
      C (diagonalJet a u i m)*(polyLeft a u k*polyRight a u i k t m) x y := by
  simp only [Matrix.mul_apply,weightedLeft]
  rw [mul_sum]
  apply sum_congr rfl
  intro rb _
  ring

theorem actual_compact_weighted_precision (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hiu : i<u) (hu : u≤a) (x y : Ports a) :
    Content (jetWeight a u i m+k) (nodeCrossingPolynomial a u i t m x y-
      (weightedLeft a u i k m*polyRight a u i k t m) x y) := by
  have hv : VBound (jetWeight a u i m) (diagonalJet a u i m) := Or.inr (by
    rw [diagonalJet_valuation a u i m hiu hu]; rfl)
  have he := (Content.C hv).mul (actual_compact_poly_precision a u i k t m ht ht0 hi hiu hu x y)
  rw [node_crossing_scale a u i t m (by omega),weighted_poly_product]
  simpa only [mul_sub] using he

theorem actual_compact_weighted_profile (a u i k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hiu : i<u) (hu : u≤a) :
    (∀ (x : Ports a) (rb : MomentLabels u k),
      Content (jetWeight a u i m+rb.2.val) (weightedLeft a u i k m x rb)) ∧
    (∀ (rb : MomentLabels u k) (y : Ports a),Content 0 (polyRight a u i k t m rb y)) ∧
    (∀ x y : Ports a,Content (jetWeight a u i m+k) (nodeCrossingPolynomial a u i t m x y-
      (weightedLeft a u i k m*polyRight a u i k t m) x y)) ∧
    Fintype.card (MomentLabels u k)=2^u*k := by
  exact ⟨weighted_left_weight a u i k m hiu hu,
    poly_right_integral a u i k t m ht ht0 hi hiu hu,
    actual_compact_weighted_precision a u i k t m ht ht0 hi hiu hu,
    moment_label_count u k⟩

end OrientationBivariate

#print axioms OrientationBivariate.denom_rows_eval
#print axioms OrientationBivariate.actual_low_rows_precision
#print axioms OrientationBivariate.poly_right_integral
#print axioms OrientationBivariate.actual_compact_poly_precision
#print axioms OrientationBivariate.actual_compact_weighted_profile
