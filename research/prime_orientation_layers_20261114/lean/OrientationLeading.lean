module
public import OrientationJet
import all OrientationAnchor
import all OrientationValuation
import all OrientationContent
import all OrientationProduct
import all OrientationHierarchy
import all OrientationResidue
import all OrientationDiagonal
import all OrientationBaseBridge
import all OrientationJet
public section
noncomputable section
namespace OrientationLeading
open Polynomial Finset OrientationAnchor OrientationResidue OrientationDiagonal OrientationJet

theorem poleQ_value (i : ℕ) : padicValRat 2 (poleQ i) = (i:ℤ) := by
  rw [poleQ,padicValRat.neg,two_power_valuation]

theorem matching_bits (u x y i : ℕ) (hi : i < u) (h : x % 2^u = y % 2^u) :
    x.testBit i = y.testBit i := by
  have hh := congrArg (fun n : ℕ => n.testBit i) h
  simpa [hi] using hh

/-- Actual crossing denominator valuation; the zero-displacement branch is separate. -/
theorem crossing_denominator (x y i : ℕ) (hxy : x ≠ y)
    (hc : x.testBit i = y.testBit i) :
    padicValRat 2 (poleQ i-delta x y) = min (i:ℤ) (order x y) := by
  have hd : delta x y ≠ 0 := by rw [delta_as_int]; exact_mod_cast signed_ne_zero x y hxy
  have hsi : i ≠ order x y := by
    intro he
    have hb := order_bit_diff x y hxy
    rw [← he] at hb
    exact hb hc
  have hh := padicValRat.add_eq_min (p:=2) (q:=poleQ i) (r:= -delta x y)
    (by simpa only [← sub_eq_add_neg] using sub_ne_zero.mpr (Ne.symm (crossing_displacement_ne_pole x y i hc)))
    (poleQ_ne_zero i) (neg_ne_zero.mpr hd)
    (by rw [poleQ_value,padicValRat.neg,delta_valuation]; exact_mod_cast hsi)
  simpa only [← sub_eq_add_neg,poleQ_value,padicValRat.neg,delta_valuation] using hh

theorem crossing_base_ratio_integral (x y i : ℕ) (hc : x.testBit i = y.testBit i) :
    VBound 0 (poleQ i/(poleQ i-delta x y)) := by
  by_cases hxy : x=y
  · subst y
    simp only [delta,sub_self,sub_zero,div_self (poleQ_ne_zero i)]
    exact Or.inr (by simp)
  · right
    rw [padicValRat.div (poleQ_ne_zero i)
      (sub_ne_zero.mpr (Ne.symm (crossing_displacement_ne_pole x y i hc))),
      poleQ_value,crossing_denominator x y i hxy hc]
    exact sub_nonneg.mpr (min_le_left _ _)

theorem matching_base_ratio_one (u x y i : ℕ) (hi : i < u)
    (h : x % 2^u = y % 2^u) : OneModTwo (poleQ i/(poleQ i-delta x y)) := by
  apply ratio_one (poleQ_ne_zero i)
  have hb := vbound_div (displacement_bound u x y h) (r:=poleQ i)
  rw [poleQ_value] at hb
  exact hb.mono (by omega)

/-- Actual normalized first-jet entry in matching retained classes. -/
theorem matching_firstJet_one (a u i x y : ℕ) (m : ℕ → ℕ)
    (hx : x < 2^a) (hy : y < 2^a) (hi : i < u) (hu : u ≤ a)
    (h : x % 2^u = y % 2^u) :
    OneModTwo (firstJet a u i x y m/diagonalJet a u i m) := by
  rw [normalized_firstJet a u i x y m hx hy (by omega) (matching_bits u x y i hi h)]
  exact (matching_base_ratio_one u x y i hi h).mul (matching_ratio_one u x y i m h)

/-- Actual normalized first-jet entry outside matching retained classes. -/
theorem nonmatching_firstJet_even (a u i x y : ℕ) (m : ℕ → ℕ)
    (hx : x < 2^a) (hy : y < 2^a) (hi : i < u) (hu : u ≤ a)
    (hc : x.testBit i = y.testBit i) (hn : x % 2^u ≠ y % 2^u)
    (hm : ∀ j < u, 0 < m j) :
    VBound 1 (firstJet a u i x y m/diagonalJet a u i m) := by
  rw [normalized_firstJet a u i x y m hx hy (by omega) hc]
  simpa only [zero_add] using VBound.mul (crossing_base_ratio_integral x y i hc)
    (nonmatching_ratio_even u x y i m hi hc hn hm)

abbrev Ports (a : ℕ) := Fin (2^a)
abbrev Labels (u : ℕ) := Fin (2^u)

/-- Explicit integral row factors; no factorization oracle or supplied rank. -/
def leftFactor (a u : ℕ) : Matrix (Ports a) (Labels u) ℚ :=
  fun x r => if x.val % 2^u = r.val then 1 else 0

/-- Explicit integral column factors. -/
def rightFactor (a u : ℕ) : Matrix (Labels u) (Ports a) ℚ :=
  fun r y => if y.val % 2^u = r.val then 1 else 0

theorem left_integral (a u : ℕ) (x : Ports a) (r : Labels u) :
    VBound 0 (leftFactor a u x r) := by
  unfold leftFactor
  split_ifs <;> simp [VBound]

theorem right_integral (a u : ℕ) (r : Labels u) (y : Ports a) :
    VBound 0 (rightFactor a u r y) := by
  unfold rightFactor
  split_ifs <;> simp [VBound]

/-- Exact product of the actual explicit factors. -/
theorem factor_entry (a u : ℕ) (x y : Ports a) :
    (leftFactor a u*rightFactor a u) x y = if x.val % 2^u = y.val % 2^u then 1 else 0 := by
  let r0 : Labels u := ⟨x.val % 2^u,Nat.mod_lt _ (by positivity)⟩
  have hz (r : Labels u) (hr : r ≠ r0) : leftFactor a u x r = 0 := by
    unfold leftFactor
    split_ifs with he
    · exact False.elim (hr (Fin.ext he.symm))
    · rfl
  have hs : (∑ r : Labels u, leftFactor a u x r*rightFactor a u r y) =
      leftFactor a u x r0*rightFactor a u r0 y :=
    Finset.sum_eq_single r0 (by intro r _ hr; rw [hz r hr,zero_mul])
      (by intro h; exact False.elim (h (mem_univ _)))
  rw [Matrix.mul_apply,hs]
  simp [leftFactor,rightFactor,r0,eq_comm]

/-- The actual normalized crossing matrix; noncrossing entries are explicitly zero. -/
def normalizedCrossing (a u i : ℕ) (m : ℕ → ℕ) : Matrix (Ports a) (Ports a) ℚ :=
  fun x y => if x.val.testBit i = y.val.testBit i then
    firstJet a u i x.val y.val m/diagonalJet a u i m else 0

/-- Constructive factorization modulo two of the actual first crossing-jet layer. -/
theorem actual_leading_factorization (a u i : ℕ) (m : ℕ → ℕ)
    (hi : i < u) (hu : u ≤ a) (hm : ∀ j < u, 0 < m j)
    (x y : Ports a) :
    VBound 1 (normalizedCrossing a u i m x y-(leftFactor a u*rightFactor a u) x y) := by
  rw [factor_entry]
  by_cases he : x.val % 2^u = y.val % 2^u
  · have hc := matching_bits u x.val y.val i hi he
    simp only [normalizedCrossing,hc,ite_true,he]
    exact matching_firstJet_one a u i x.val y.val m x.isLt y.isLt hi hu he
  · rw [ite_eq_right he]
    by_cases hc : x.val.testBit i = y.val.testBit i
    · simp only [normalizedCrossing,hc,ite_true,sub_zero]
      exact nonmatching_firstJet_even a u i x.val y.val m x.isLt y.isLt hi hu hc he hm
    · simp [normalizedCrossing,hc,VBound]

/-- The actual unnormalized crossing first-jet matrix. -/
def crossingFirst (a u i : ℕ) (m : ℕ → ℕ) : Matrix (Ports a) (Ports a) ℚ :=
  fun x y => if x.val.testBit i = y.val.testBit i then firstJet a u i x.val y.val m else 0

/-- Explicit row coefficients carrying their actual first-jet valuation weight. -/
def weightedLeft (a u i : ℕ) (m : ℕ → ℕ) : Matrix (Ports a) (Labels u) ℚ :=
  fun x r => diagonalJet a u i m*leftFactor a u x r

def jetWeight (a u i : ℕ) (m : ℕ → ℕ) : ℤ := (a:ℤ)+1-i+schedule u i m

theorem weighted_product (a u i : ℕ) (m : ℕ → ℕ) (x y : Ports a) :
    (weightedLeft a u i m*rightFactor a u) x y =
      diagonalJet a u i m*(leftFactor a u*rightFactor a u) x y := by
  simp only [Matrix.mul_apply,weightedLeft]
  rw [mul_sum]
  apply sum_congr rfl
  intro r _
  ring

theorem weighted_left_bound (a u i : ℕ) (m : ℕ → ℕ) (hi : i < u) (hu : u ≤ a)
    (x : Ports a) (r : Labels u) : VBound (jetWeight a u i m) (weightedLeft a u i m x r) := by
  unfold weightedLeft leftFactor
  split_ifs
  · simp only [mul_one]
    right
    rw [diagonalJet_valuation a u i m hi hu]
    rfl
  · simp [VBound]

theorem crossing_first_scale (a u i : ℕ) (m : ℕ → ℕ) (hia : i ≤ a) (x y : Ports a) :
    crossingFirst a u i m x y = diagonalJet a u i m*normalizedCrossing a u i m x y := by
  unfold crossingFirst normalizedCrossing
  split_ifs
  · field_simp [diagonalJet_ne_zero a u i m hia]
  · simp

/-- Absolute residual weight, derived from the actual diagonal scalar, not a supplied valuation. -/
theorem actual_weighted_factorization (a u i : ℕ) (m : ℕ → ℕ)
    (hi : i < u) (hu : u ≤ a) (hm : ∀ j < u, 0 < m j) (x y : Ports a) :
    VBound (jetWeight a u i m+1)
      (crossingFirst a u i m x y-(weightedLeft a u i m*rightFactor a u) x y) := by
  have h := actual_leading_factorization a u i m hi hu hm x y
  have hc : VBound (jetWeight a u i m) (diagonalJet a u i m) := Or.inr (by
    rw [diagonalJet_valuation a u i m hi hu]
    rfl)
  have hs : VBound (jetWeight a u i m+1)
      ((normalizedCrossing a u i m x y-(leftFactor a u*rightFactor a u) x y)*diagonalJet a u i m) := by
    simpa only [add_comm] using h.mul hc
  have he : ((normalizedCrossing a u i m x y-(leftFactor a u*rightFactor a u) x y)*diagonalJet a u i m) =
      crossingFirst a u i m x y-(weightedLeft a u i m*rightFactor a u) x y := by
    rw [crossing_first_scale a u i m (by omega),weighted_product]
    ring
  rwa [he] at hs

/-- Feasibility makes the explicit weighted row factors integral. -/
theorem weighted_left_integral (a u i : ℕ) (m : ℕ → ℕ)
    (hi : i < u) (hu : u ≤ a) (hg : 0 ≤ schedule u i m) (x : Ports a) (r : Labels u) :
    VBound 0 (weightedLeft a u i m x r) := by
  apply (weighted_left_bound a u i m hi hu x r).mono
  dsimp [jetWeight]
  omega

end OrientationLeading
