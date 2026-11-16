module
public import OrientationNode
public import OrientationMoment
public import OrientationAnchor
public import OrientationValuation
public import OrientationContent
public import OrientationProduct
public import OrientationResidue
public import OrientationDiagonal
public import OrientationJet
public import OrientationLeading
public import OrientationRefinement
public import OrientationRank
import all OrientationNode
import all OrientationMoment
import all OrientationAnchor
import all OrientationValuation
import all OrientationContent
import all OrientationProduct
import all OrientationResidue
import all OrientationDiagonal
import all OrientationJet
import all OrientationLeading
import all OrientationRefinement
import all OrientationRank
public section
noncomputable section
namespace OrientationNodeMoment
open Polynomial Finset OrientationAnchor OrientationMoment OrientationGradedNode
  OrientationResidue OrientationDiagonal OrientationJet OrientationLeading OrientationRefinement

theorem node_slope_even (u i x y : ℕ) (t : ℤ)
    (hiu : i < u) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) :
    VBound 1 (slope u (rowResidue u x) y (t:ℚ)) := by
  have hp := actual_node_denominator_le (rowResidue u x) y i t ht0 hi
    (row_crossing u i x y hiu hc)
  have hb := vbound_div (power_integral u) (r:=((t:ℚ)-delta (rowResidue u x) y))
  exact hb.mono (by omega)

def nodeReciprocal (u x y : ℕ) (t : ℤ) (m : ℕ → ℕ) : ℚ :=
  (1+slope u (rowResidue u x) y (t:ℚ)*(rowQuotient u x:ℚ))⁻¹ *
  ∏ j ∈ mask u x y,
    ((1+slope u (rowResidue u x) y ((2:ℚ)^j)*(rowQuotient u x:ℚ))⁻¹)^m j

theorem node_comparison_reciprocal (u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hiu : i < u) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) :
    nodeComparisonRatio u x (rowResidue u x) y t m = nodeReciprocal u x y t m := by
  have hmask := masks_stable u u x (rowResidue u x) y le_rfl (row_mod u x)
  have hb := denominator_inverse u x y (t:ℚ)
    (sub_ne_zero.mpr (Ne.symm (actual_displacement_ne_node (rowResidue u x) y i t ht0 hi
      (row_crossing u i x y hiu hc))))
  have hp : correctionRatio u x (rowResidue u x) y m =
      ∏ j ∈ mask u x y,
        ((1+slope u (rowResidue u x) y ((2:ℚ)^j)*(rowQuotient u x:ℚ))⁻¹)^m j := by
    unfold correctionRatio
    apply prod_congr rfl
    intro j hj
    rw [denominator_inverse u x y ((2:ℚ)^j)
      (denominator_ne_zero u (rowResidue u x) y j (hmask ▸ hj))]
  simp only [nodeComparisonRatio,hb,hp,nodeReciprocal]

def nodeDenominator (u x y : ℕ) (t : ℤ) (m : ℕ → ℕ) : ℚ[X] :=
  (1+C (slope u (rowResidue u x) y (t:ℚ))*X)*
    ∏ j ∈ mask u x y, (1+C (slope u (rowResidue u x) y ((2:ℚ)^j))*X)^m j

theorem node_denominator_weighted (u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hiu : i < u) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) : WeightedPolynomial (nodeDenominator u x y t m) := by
  apply (weighted_linear (node_slope_even u i x y t hiu ht0 hi hc)).mul
  apply WeightedPolynomial.prod
  intro j hj
  exact (weighted_linear (anchor_slope_even u x y j hj)).pow (m j)

theorem node_denominator_constant (u x y : ℕ) (t : ℤ) (m : ℕ → ℕ) :
    (nodeDenominator u x y t m).coeff 0=1 := by
  rw [coeff_zero_eq_eval_zero]
  simp only [nodeDenominator,eval_mul,eval_prod,eval_pow,eval_add,eval_one,eval_C,eval_X,
    mul_zero,add_zero,one_pow,prod_const_one,mul_one]

theorem node_denominator_reciprocal (u x y : ℕ) (t : ℤ) (m : ℕ → ℕ) :
    nodeReciprocal u x y t m =
      ((nodeDenominator u x y t m).eval (rowQuotient u x:ℚ))⁻¹ := by
  simp only [nodeDenominator,nodeReciprocal,eval_mul,eval_prod,eval_pow,eval_add,
    eval_one,eval_C,eval_X,mul_inv_rev,← Finset.prod_inv_distrib,← inv_pow]
  ring

theorem node_denominator_row_class (u x y : ℕ) (t : ℤ) (m : ℕ → ℕ) :
    nodeDenominator u x y t m = nodeDenominator u (rowResidue u x) y t m := by
  have hm := masks_stable u u x (rowResidue u x) y le_rfl (row_mod u x)
  unfold nodeDenominator
  rw [← hm]
  have hr : rowResidue u (rowResidue u x)=rowResidue u x := by simp [rowResidue]
  rw [hr]

theorem actual_node_reciprocal_family (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hy : y<2^a) (ht : (t:ℚ) ∈ grid a)
    (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u) (hua : u≤a)
    (hc : x.testBit i=y.testBit i) :
    (chart a t (completed (grid a) u x y m)).coeff 1/diagonalJet a u i m =
      ((chart a t (completed (grid a) u (rowResidue u x) y m)).coeff 1/diagonalJet a u i m)*
        nodeReciprocal u x y t m := by
  rw [actual_node_jet_comparison a u u i x (rowResidue u x) y t m hx
    (row_source_bound a u x hua) hy hiu le_rfl hua (row_mod u x) ht ht0 hi hc,
    node_comparison_reciprocal u i x y t m hiu ht0 hi hc]

theorem actual_node_moment_error (a u i x y k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hy : y<2^a) (ht : (t:ℚ) ∈ grid a)
    (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u) (hua : u≤a)
    (hc : x.testBit i=y.testBit i) :
    VBound (k:ℤ) ((chart a t (completed (grid a) u x y m)).coeff 1/diagonalJet a u i m-
      ((chart a t (completed (grid a) u (rowResidue u x) y m)).coeff 1/diagonalJet a u i m)*
        (inverseTrunc (nodeDenominator u (rowResidue u x) y t m) k).eval
          (rowQuotient u x:ℚ)) := by
  have hD := node_denominator_weighted u i x y t m hiu ht0 hi hc
  have h0 := node_denominator_constant u x y t m
  have ht' := inverse_trunc_error (nodeDenominator u x y t m) hD h0 k (rowQuotient u x:ℤ)
  have hp := actual_node_first_integral a u i (rowResidue u x) y t m
    (row_source_bound a u x hua) hy ht ht0 hi hiu hua (row_crossing u i x y hiu hc)
  have hh : VBound (k:ℤ)
      (((nodeDenominator u x y t m).eval (rowQuotient u x:ℚ))⁻¹-
        (inverseTrunc (nodeDenominator u x y t m) k).eval (rowQuotient u x:ℚ)) := by
    simpa only [Int.cast_natCast] using ht'
  have he := hp.mul hh
  rw [actual_node_reciprocal_family a u i x y t m hx hy ht ht0 hi hiu hua hc,
    node_denominator_reciprocal,← node_denominator_row_class u x y t m]
  simpa only [zero_add,mul_sub] using he

def nodeMomentRight (a u i k : ℕ) (hu : u≤a) (t : ℤ) (m : ℕ → ℕ) :
    Matrix (MomentLabels u k) (Ports a) ℚ := fun rb y =>
  nodeCrossingFirst a u i t m (OrientationRank.classPort a u hu rb.1) y *
    PowerSeries.coeff rb.2.val (inverseSeries (nodeDenominator u rb.1.val y.val t m)) /
      (2:ℚ)^rb.2.val

theorem node_moment_right_integral (a u i k : ℕ) (hu : u≤a) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u)
    (rb : MomentLabels u k) (y : Ports a) : VBound 0 (nodeMomentRight a u i k hu t m rb y) := by
  unfold nodeMomentRight nodeCrossingFirst
  split_ifs with hc
  · have hR := actual_node_first_integral a u i rb.1.val y.val t m
      (OrientationRank.classPort a u hu rb.1).isLt y.isLt ht ht0 hi hiu hu hc
    have hD := node_denominator_weighted u i rb.1.val y.val t m hiu ht0 hi hc
    have hb := weighted_inverse_coefficients _ hD rb.2.val
    have hq := vbound_div hb (r:=(2:ℚ)^rb.2.val)
    rw [two_power_valuation,sub_self] at hq
    simpa only [OrientationRank.classPort,zero_add,mul_div_assoc] using hR.mul hq
  · simp only [zero_mul,zero_div]
    exact VBound.zero _

theorem node_moment_product (a u i k : ℕ) (hu : u≤a) (t : ℤ) (m : ℕ → ℕ) (x y : Ports a) :
    (momentLeft a u k*nodeMomentRight a u i k hu t m) x y =
      nodeCrossingFirst a u i t m
        (OrientationRank.classPort a u hu ⟨rowResidue u x.val,row_bound u x.val⟩) y *
      (inverseTrunc (nodeDenominator u (rowResidue u x.val) y.val t m) k).eval
        (rowQuotient u x.val:ℚ) := by
  let r0 : Labels u := ⟨rowResidue u x.val,row_bound u x.val⟩
  have hz (r : Labels u) (hr : r ≠ r0) (b : Fin k) : momentLeft a u k x (r,b)=0 := by
    unfold momentLeft
    split_ifs with he
    · exact False.elim (hr (Fin.ext he.symm))
    · rfl
  have hs : (∑ r : Labels u,∑ b : Fin k,
      momentLeft a u k x (r,b)*nodeMomentRight a u i k hu t m (r,b) y) =
      ∑ b : Fin k,momentLeft a u k x (r0,b)*nodeMomentRight a u i k hu t m (r0,b) y :=
    Finset.sum_eq_single r0 (by intro r _ hr; simp only [hz r hr,zero_mul,sum_const_zero])
      (by intro h; exact False.elim (h (mem_univ _)))
  rw [Matrix.mul_apply,Fintype.sum_prod_type,hs,trunc_eval_moments,mul_sum]
  apply sum_congr rfl
  intro b _
  simp only [momentLeft,nodeMomentRight,r0,ite_true]
  field_simp [show (2:ℚ)^b.val ≠ 0 by positivity]

theorem actual_node_moment_factorization (a u i k : ℕ) (hu : u≤a) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u)
    (x y : Ports a) : VBound (k:ℤ) (nodeCrossingFirst a u i t m x y-
      (momentLeft a u k*nodeMomentRight a u i k hu t m) x y) := by
  rw [node_moment_product]
  have hb := matching_bits u x.val (rowResidue u x.val) i hiu (row_mod u x.val)
  by_cases hc : x.val.testBit i=y.val.testBit i
  · have hr : (rowResidue u x.val).testBit i=y.val.testBit i := hb.symm.trans hc
    simp only [nodeCrossingFirst,hc,hr,OrientationRank.classPort,ite_true]
    exact actual_node_moment_error a u i x.val y.val k t m x.isLt y.isLt ht ht0 hi hiu hu hc
  · have hr : (rowResidue u x.val).testBit i ≠ y.val.testBit i := by rwa [← hb]
    simp only [nodeCrossingFirst,hc,hr,OrientationRank.classPort,ite_false,zero_mul,sub_self]
    exact VBound.zero _

def nodeActualFirst (a u i : ℕ) (t : ℤ) (m : ℕ → ℕ) : Matrix (Ports a) (Ports a) ℚ :=
  fun x y => if x.val.testBit i=y.val.testBit i then
    (chart a t (completed (grid a) u x.val y.val m)).coeff 1 else 0

theorem node_first_scale (a u i : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hia : i≤a) (x y : Ports a) :
    nodeActualFirst a u i t m x y=diagonalJet a u i m*nodeCrossingFirst a u i t m x y := by
  unfold nodeActualFirst nodeCrossingFirst
  split_ifs
  · field_simp [diagonalJet_ne_zero a u i m hia]
  · ring

theorem node_moment_weighted_product (a u i k : ℕ) (hu : u≤a) (t : ℤ) (m : ℕ → ℕ)
    (x y : Ports a) :
    (momentWeightedLeft a u i k m*nodeMomentRight a u i k hu t m) x y =
      diagonalJet a u i m*(momentLeft a u k*nodeMomentRight a u i k hu t m) x y := by
  simp only [Matrix.mul_apply,momentWeightedLeft]
  rw [mul_sum]
  apply sum_congr rfl
  intro rb _
  ring

theorem actual_node_weighted_moment_factorization (a u i k : ℕ) (hu : u≤a) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u)
    (x y : Ports a) : VBound (jetWeight a u i m+k) (nodeActualFirst a u i t m x y-
      (momentWeightedLeft a u i k m*nodeMomentRight a u i k hu t m) x y) := by
  have hr := actual_node_moment_factorization a u i k hu t m ht ht0 hi hiu x y
  have hc : VBound (jetWeight a u i m) (diagonalJet a u i m) := Or.inr (by
    rw [diagonalJet_valuation a u i m hiu hu]
    rfl)
  have hh := hc.mul hr
  rw [node_first_scale a u i t m (by omega),node_moment_weighted_product]
  simpa only [mul_sub] using hh

theorem actual_node_moment_profile (a u i k : ℕ) (hu : u≤a) (t : ℤ) (m : ℕ → ℕ)
    (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u) :
    (∀ (x : Ports a) (rb : MomentLabels u k),
      VBound (jetWeight a u i m+rb.2.val) (momentWeightedLeft a u i k m x rb)) ∧
    (∀ (rb : MomentLabels u k) (y : Ports a),VBound 0 (nodeMomentRight a u i k hu t m rb y)) ∧
    (∀ x y : Ports a,VBound (jetWeight a u i m+k) (nodeActualFirst a u i t m x y-
      (momentWeightedLeft a u i k m*nodeMomentRight a u i k hu t m) x y)) ∧
    Fintype.card (MomentLabels u k)=2^u*k := by
  exact ⟨moment_weighted_left_bound a u i k m hiu hu,
    node_moment_right_integral a u i k hu t m ht ht0 hi hiu,
    actual_node_weighted_moment_factorization a u i k hu t m ht ht0 hi hiu,
    moment_label_count u k⟩

theorem positive_anchor_moment_right_zero (a u i k : ℕ) (hu : u≤a) (m : ℕ → ℕ)
    (hiu : i<u) (hm : 0 < m i) (rb : MomentLabels u k) (y : Ports a) :
    nodeMomentRight a u i k hu ((2:ℤ)^i) m rb y=0 := by
  unfold nodeMomentRight nodeCrossingFirst
  split_ifs with hc
  · simp only [OrientationRank.classPort] at hc ⊢
    rw [positive_anchor_first_zero a u i rb.1.val y.val m hiu hu hc hm]
    simp
  · simp

end OrientationNodeMoment

namespace OrientationNodePolynomial
open Polynomial Finset OrientationAnchor OrientationMoment OrientationGradedNode
  OrientationResidue OrientationDiagonal OrientationJet OrientationLeading OrientationRefinement

theorem content_neg {k : ℤ} {P : ℚ[X]} (h : Content k P) : Content k (-P) := by
  intro n
  simpa only [coeff_neg] using (h n).neg

theorem content_sub {k : ℤ} {P Q : ℚ[X]} (hP : Content k P) (hQ : Content k Q) :
    Content k (P-Q) := by
  simpa only [sub_eq_add_neg] using hP.add (content_neg hQ)

theorem content_sum {ι : Type*} (S : Finset ι) (P : ι → ℚ[X]) (k : ℤ)
    (h : ∀ j ∈ S,Content k (P j)) : Content k (∑ j ∈ S,P j) := by
  intro n
  rw [finsetSum_coeff]
  exact VBound.sum _ _ _ (fun j hj => h j hj n)

/-- A denominator congruent to one preserves polynomial content; no local jet is inverted. -/
theorem content_cancel_near_one (D R : ℚ[X]) (k : ℕ)
    (hD : Content 1 (D-1)) (hR : Content 0 R) (hDR : Content (k:ℤ) (D*R)) :
    Content (k:ℤ) R := by
  have step (n : ℕ) : n≤k → Content (n:ℤ) R := by
    induction n with
    | zero => intro _; exact hR
    | succ n ih =>
      intro hn
      have hprev := ih (by omega)
      have hprod : Content ((n:ℤ)+1) ((D-1)*R) := by
        simpa only [add_comm] using hD.mul hprev
      have hd : Content ((n:ℤ)+1) (D*R) := hDR.mono (by omega)
      have hh := content_sub hd hprod
      have he : D*R-(D-1)*R=R := by ring
      simpa only [he,Nat.cast_add,Nat.cast_one] using hh
  exact step k le_rfl

def geometricInverse (D : ℚ[X]) (k : ℕ) : ℚ[X] := ∑ n ∈ range k,(1-D)^n

theorem geometric_inverse_identity (D : ℚ[X]) (k : ℕ) :
    D*geometricInverse D k=1-(1-D)^k := by
  induction k with
  | zero => simp [geometricInverse]
  | succ k ih =>
    simp only [geometricInverse,sum_range_succ] at *
    rw [mul_add,ih,pow_succ]
    ring

theorem geometric_inverse_integral (D : ℚ[X]) (k : ℕ) (hD : Content 1 (D-1)) :
    Content 0 (geometricInverse D k) := by
  apply content_sum
  intro n _
  have hE : Content 1 (1-D) := by simpa only [neg_sub] using content_neg hD
  exact (hE.pow n).mono (by omega)

/-- Actual polynomial denominator identity implies a constructed whole-polynomial approximation. -/
theorem geometric_polynomial_error (D P N : ℚ[X]) (k : ℕ)
    (hD : Content 1 (D-1)) (hP : Content 0 P) (hN : Content 0 N)
    (hactual : D*P=N) : Content (k:ℤ) (P-N*geometricInverse D k) := by
  have hS := geometric_inverse_integral D k hD
  have hR : Content 0 (P-N*geometricInverse D k) :=
    content_sub hP (by simpa only [zero_add] using hN.mul hS)
  have hE : Content 1 (1-D) := by simpa only [neg_sub] using content_neg hD
  have htail : Content (k:ℤ) (N*(1-D)^k) := by
    simpa only [mul_one,zero_add] using hN.mul (hE.pow k)
  apply content_cancel_near_one D _ k hD hR
  have he : D*(P-N*geometricInverse D k)=N*(1-D)^k := by
    calc
      _ = D*P-N*(D*geometricInverse D k) := by ring
      _ = N-N*(1-(1-D)^k) := by rw [hactual,geometric_inverse_identity]
      _ = _ := by ring
  rwa [he]

theorem actual_correction_row_polynomial (u x y : ℕ) (m : ℕ → ℕ) :
    correction u x y m = correction u (rowResidue u x) y m *
      C (correctionRatio u x (rowResidue u x) y m) := by
  apply Polynomial.funext
  intro T
  simpa only [eval_mul,eval_C] using
    actual_node_correction_comparison u u x (rowResidue u x) y T m le_rfl (row_mod u x)

theorem actual_completed_row_polynomial (a u x y : ℕ) (m : ℕ → ℕ)
    (hx : x<2^a) (hy : y<2^a) (hu : u≤a) :
    (X-C (delta x y))*completed (grid a) u x y m =
      C (correctionRatio u x (rowResidue u x) y m)*
        (X-C (delta (rowResidue u x) y))*completed (grid a) u (rowResidue u x) y m := by
  have hd := nodal_puncture (grid a) (delta x y) (delta_in_grid a x y hx hy)
  have hr := nodal_puncture (grid a) (delta (rowResidue u x) y)
    (delta_in_grid a (rowResidue u x) y (row_source_bound a u x hu) hy)
  simp only [completed,base]
  calc
    _ = C ((normalizer (grid a))⁻¹)*(puncture (grid a) (delta x y)*(X-C (delta x y)))*
      correction u x y m := by ring
    _ = C ((normalizer (grid a))⁻¹)*nodal (grid a)*
      (correction u (rowResidue u x) y m*C (correctionRatio u x (rowResidue u x) y m)) := by
        rw [hd,actual_correction_row_polynomial]
    _ = _ := by rw [← hr]; ring

theorem actual_node_integral (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hy : y<2^a) (ht : (t:ℚ) ∈ grid a)
    (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u) (hu : u≤a)
    (hc : x.testBit i=y.testBit i) : Content 0 (normalizedNodeEntry a u i x y t m) := by
  intro n
  cases n with
  | zero => rw [actual_node_constant a u i x y t m ht ht0 hi hc]; exact VBound.zero _
  | succ n => exact (actual_node_coefficient_weight a u i x y t m hx hy ht ht0 hi hiu hu hc n).mono (by omega)

theorem correction_ratio_one (u x y : ℕ) (m : ℕ → ℕ) :
    OneModTwo (correctionRatio u x (rowResidue u x) y m) := by
  have h := correction_ratio_near u u x (rowResidue u x) y m le_rfl (row_mod u x)
  simpa only [sub_self,zero_add,NearOne,OneModTwo] using h

theorem reciprocal_one {q : ℚ} (hq : OneModTwo q) : OneModTwo q⁻¹ := by
  have hne := one_mod_two_ne_zero hq
  have hv := one_mod_two_valuation hq
  have h := vbound_div hq (r:=q)
  rw [hv,sub_zero] at h
  have he : -((q-1)/q)=q⁻¹-1 := by field_simp [hne]; ring
  have hh := h.neg
  rwa [he] at hh

def chartSlope (a u x y : ℕ) (t : ℤ) : ℚ :=
  (2:ℚ)^(a+1)/((t:ℚ)-delta (rowResidue u x) y)

def rowDenominator (a u x y : ℕ) (t : ℤ) (m : ℕ → ℕ) : ℚ[X] :=
  C ((correctionRatio u x (rowResidue u x) y m)⁻¹)*
    (C (((t:ℚ)-delta x y)/((t:ℚ)-delta (rowResidue u x) y))+C (chartSlope a u x y t)*X)

def rowNumerator (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ) : ℚ[X] :=
  (1+C (chartSlope a u x y t)*X)*normalizedNodeEntry a u i (rowResidue u x) y t m

theorem chart_slope_even (a u i x y : ℕ) (t : ℤ)
    (hiu : i<u) (hu : u≤a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) : VBound 1 (chartSlope a u x y t) := by
  have hd := actual_node_denominator_le (rowResidue u x) y i t ht0 hi
    (row_crossing u i x y hiu hc)
  have hb := vbound_div (power_integral (a+1)) (r:=((t:ℚ)-delta (rowResidue u x) y))
  exact hb.mono (by omega)

theorem actual_row_denominator_near (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hiu : i<u) (hu : u≤a) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) : Content 1 (rowDenominator a u x y t m-1) := by
  have hq := reciprocal_one (correction_ratio_one u x y m)
  have hr : NearOne 1 (((t:ℚ)-delta x y)/((t:ℚ)-delta (rowResidue u x) y)) := by
    simpa only [sub_self,zero_add] using
      node_base_ratio_near u u i (rowResidue u x) x y t hiu (row_mod u x).symm ht0 hi
        (row_crossing u i x y hiu hc)
  have hv := NearOne.mul (by norm_num : (0:ℤ)≤1) hq hr
  have hs := (NearOne.integral (by norm_num : (0:ℤ)≤1) hq).mul
    (chart_slope_even a u i x y t hiu hu ht0 hi hc)
  have hconst := Content.C hv
  have hlinear : Content 1 (C ((correctionRatio u x (rowResidue u x) y m)⁻¹*
      chartSlope a u x y t)*X) := by
    simpa only [zero_add,add_zero] using (Content.C hs).mul Content.X
  have hsum := hconst.add hlinear
  convert hsum using 1
  simp only [rowDenominator,map_sub,map_mul,map_one]
  ring

theorem actual_row_denominator_identity (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hy : y<2^a) (hu : u≤a)
    (hiu : i<u) (ht0 : t ≠ 0) (hi : padicValInt 2 t=i)
    (hc : x.testBit i=y.testBit i) :
    rowDenominator a u x y t m*normalizedNodeEntry a u i x y t m=
      rowNumerator a u i x y t m := by
  have hq := one_mod_two_ne_zero (correction_ratio_one u x y m)
  have hdr := sub_ne_zero.mpr (Ne.symm (actual_displacement_ne_node (rowResidue u x) y i t ht0 hi
    (row_crossing u i x y hiu hc)))
  have hc0 := diagonalJet_ne_zero a u i m (by omega)
  apply Polynomial.funext
  intro z
  have hrow := congrArg (fun P : ℚ[X] => P.eval ((t:ℚ)+(2:ℚ)^(a+1)*z))
    (actual_completed_row_polynomial a u x y m hx hy hu)
  simp only [eval_mul,eval_sub,eval_X,eval_C] at hrow
  simp only [rowDenominator,rowNumerator,normalizedNodeEntry,chart,chartSlope,
    eval_mul,eval_add,eval_C,eval_comp,eval_one,eval_X]
  field_simp [hq,hdr,hc0]
  nlinarith [hrow]

theorem actual_row_numerator_integral (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hy : y<2^a) (ht : (t:ℚ) ∈ grid a) (ht0 : t ≠ 0)
    (hi : padicValInt 2 t=i) (hiu : i<u) (hu : u≤a)
    (hc : x.testBit i=y.testBit i) : Content 0 (rowNumerator a u i x y t m) := by
  have hs := (chart_slope_even a u i x y t hiu hu ht0 hi hc).mono (by norm_num : (0:ℤ)≤1)
  have hlinear : Content 0 (1+C (chartSlope a u x y t)*X) :=
    Content.one.add (by simpa only [add_zero] using (Content.C hs).mul Content.X)
  have hp := actual_node_integral a u i (rowResidue u x) y t m
    (row_source_bound a u x hu) hy ht ht0 hi hiu hu (row_crossing u i x y hiu hc)
  simpa only [rowNumerator,add_zero] using hlinear.mul hp

def actualChartApproximation (a u i x y k : ℕ) (t : ℤ) (m : ℕ → ℕ) : ℚ[X] :=
  rowNumerator a u i x y t m*geometricInverse (rowDenominator a u x y t m) k

theorem actual_whole_chart_precision (a u i x y k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hy : y<2^a) (ht : (t:ℚ) ∈ grid a)
    (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u) (hu : u≤a)
    (hc : x.testBit i=y.testBit i) :
    Content (k:ℤ) (normalizedNodeEntry a u i x y t m-actualChartApproximation a u i x y k t m) := by
  exact geometric_polynomial_error _ _ _ k
    (actual_row_denominator_near a u i x y t m hiu hu ht0 hi hc)
    (actual_node_integral a u i x y t m hx hy ht ht0 hi hiu hu hc)
    (actual_row_numerator_integral a u i x y t m hy ht ht0 hi hiu hu hc)
    (actual_row_denominator_identity a u i x y t m hx hy hu hiu ht0 hi hc)

theorem normalized_node_scale (a u i x y : ℕ) (t : ℤ) (m : ℕ → ℕ) (hia : i≤a) :
    C (diagonalJet a u i m)*normalizedNodeEntry a u i x y t m=
      chart a t (completed (grid a) u x y m) := by
  simp only [normalizedNodeEntry,← mul_assoc,← map_mul,
    mul_inv_cancel₀ (diagonalJet_ne_zero a u i m hia),map_one,one_mul]

theorem actual_weighted_whole_chart_precision (a u i x y k : ℕ) (t : ℤ) (m : ℕ → ℕ)
    (hx : x<2^a) (hy : y<2^a) (ht : (t:ℚ) ∈ grid a)
    (ht0 : t ≠ 0) (hi : padicValInt 2 t=i) (hiu : i<u) (hu : u≤a)
    (hc : x.testBit i=y.testBit i) :
    Content (jetWeight a u i m+k) (chart a t (completed (grid a) u x y m)-
      C (diagonalJet a u i m)*actualChartApproximation a u i x y k t m) := by
  have hv : VBound (jetWeight a u i m) (diagonalJet a u i m) := Or.inr (by
    rw [diagonalJet_valuation a u i m hiu hu]; rfl)
  have he := (Content.C hv).mul
    (actual_whole_chart_precision a u i x y k t m hx hy ht ht0 hi hiu hu hc)
  rwa [mul_sub,normalized_node_scale a u i x y t m (by omega)] at he

theorem geometric_inverse_degree (D : ℚ[X]) (k : ℕ) (hD : D.natDegree≤1) :
    (geometricInverse D k).natDegree≤k-1 := by
  have hE : (1-D).natDegree≤1 :=
    (natDegree_sub_le _ _).trans (max_le (by simp) hD)
  induction k with
  | zero => simp [geometricInverse]
  | succ k ih =>
    have hp : ((1-D)^k).natDegree≤k := by
      exact natDegree_pow_le.trans (by simpa using Nat.mul_le_mul_left k hE)
    rw [geometricInverse,sum_range_succ]
    exact (natDegree_add_le _ _).trans (max_le (le_trans ih (by omega)) hp)

theorem actual_row_denominator_degree (a u x y : ℕ) (t : ℤ) (m : ℕ → ℕ) :
    (rowDenominator a u x y t m).natDegree≤1 := by
  unfold rowDenominator
  have hlin : (C (((t:ℚ)-delta x y)/((t:ℚ)-delta (rowResidue u x) y))+
      C (chartSlope a u x y t)*X).natDegree≤1 :=
    (natDegree_add_le _ _).trans (max_le (by simp) (by simpa using
      (natDegree_mul_le (p:=C (chartSlope a u x y t)) (q:=X))))
  exact natDegree_mul_le.trans (by simpa using hlin)

theorem actual_approximation_degree (a u i x y k : ℕ) (t : ℤ) (m : ℕ → ℕ) :
    (actualChartApproximation a u i x y k t m).natDegree≤
      (normalizedNodeEntry a u i (rowResidue u x) y t m).natDegree+k := by
  cases k with
  | zero => simp [actualChartApproximation,geometricInverse]
  | succ k =>
    have hlin : (1+C (chartSlope a u x y t)*X:ℚ[X]).natDegree≤1 :=
      (natDegree_add_le _ _).trans (max_le (by simp) (by simpa using
        (natDegree_mul_le (p:=C (chartSlope a u x y t)) (q:=X))))
    have hN : (rowNumerator a u i x y t m).natDegree≤
        1+(normalizedNodeEntry a u i (rowResidue u x) y t m).natDegree :=
      natDegree_mul_le.trans (Nat.add_le_add_right hlin _)
    have hS := geometric_inverse_degree (rowDenominator a u x y t m) (k+1)
      (actual_row_denominator_degree a u x y t m)
    exact natDegree_mul_le.trans (by omega)

end OrientationNodePolynomial

#print axioms OrientationNodeMoment.actual_node_moment_profile
#print axioms OrientationNodeMoment.positive_anchor_moment_right_zero
#print axioms OrientationNodePolynomial.actual_row_denominator_identity
#print axioms OrientationNodePolynomial.actual_whole_chart_precision
#print axioms OrientationNodePolynomial.actual_weighted_whole_chart_precision
#print axioms OrientationNodePolynomial.actual_approximation_degree
