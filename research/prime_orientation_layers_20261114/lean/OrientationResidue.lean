module
public import OrientationHierarchy
import all OrientationAnchor
import all OrientationValuation
import all OrientationContent
import all OrientationProduct
public section
noncomputable section
namespace OrientationResidue
open Polynomial Finset OrientationAnchor

/-- Concrete rational congruence to one modulo two, no finite-field cast assumed. -/
def OneModTwo (q : ℚ) : Prop := VBound 1 (q-1)

theorem vbound_div {k : ℤ} {q r : ℚ} (h : VBound k q) :
    VBound (k-padicValRat 2 r) (q/r) := by
  have hr : VBound (-padicValRat 2 r) r⁻¹ := Or.inr (by rw [padicValRat.inv])
  simpa only [div_eq_mul_inv, sub_eq_add_neg] using h.mul hr

theorem OneModTwo.integral {q : ℚ} (h : OneModTwo q) : VBound 0 q := by
  have hh : VBound 0 (q-1) := h.mono (by decide)
  simpa only [sub_add_cancel] using hh.add (Or.inr (by simp) : VBound 0 (1:ℚ))

theorem OneModTwo.one : OneModTwo 1 := by simp [OneModTwo, VBound]

theorem OneModTwo.mul {q r : ℚ} (hq : OneModTwo q) (hr : OneModTwo r) :
    OneModTwo (q*r) := by
  have hprod : VBound 1 ((q-1)*r) := by simpa only [add_zero] using VBound.mul hq hr.integral
  have h := VBound.add hprod hr
  unfold OneModTwo
  convert h using 1
  ring

theorem OneModTwo.pow {q : ℚ} (h : OneModTwo q) (n : ℕ) : OneModTwo (q^n) := by
  induction n with
  | zero => simpa only [pow_zero] using OneModTwo.one
  | succ n ih => simpa only [pow_succ] using ih.mul h

theorem OneModTwo.prod {ι : Type*} (S : Finset ι) (q : ι → ℚ)
    (h : ∀ j ∈ S, OneModTwo (q j)) : OneModTwo (∏ j ∈ S, q j) := by
  classical
  induction S using Finset.induction_on with
  | empty => simpa using OneModTwo.one
  | @insert j S hj ih =>
    rw [prod_insert hj]
    exact (h j (mem_insert_self _ _)).mul (ih (by intro k hk; exact h k (mem_insert_of_mem hk)))

/-- A strictly even rational perturbation leaves the denominator a unit. -/
theorem one_sub_ne_zero {e : ℚ} (h : VBound 1 e) : 1-e ≠ 0 := by
  intro hz
  have he : e=1 := by linarith
  rcases h with h0 | hv
  · rw [he] at h0
    exact one_ne_zero h0
  · rw [he, padicValRat.one] at hv
    omega

theorem one_sub_valuation {e : ℚ} (h : VBound 1 e) : padicValRat 2 (1-e) = 0 := by
  by_cases he : e=0
  · simp [he]
  have hv : 1 ≤ padicValRat 2 e := h.resolve_left he
  have hh := padicValRat.add_eq_min (p:=2) (q:=1) (r:= -e)
    (by simpa only [← sub_eq_add_neg] using one_sub_ne_zero h) one_ne_zero
    (neg_ne_zero.mpr he) (by rw [padicValRat.one, padicValRat.neg]; omega)
  simp only [padicValRat.one, padicValRat.neg] at hh
  rw [← sub_eq_add_neg, min_eq_left (by omega)] at hh
  exact hh

/-- The inverse has the same concrete one-mod-two residue. -/
theorem inv_one_sub {e : ℚ} (h : VBound 1 e) : OneModTwo ((1-e)⁻¹) := by
  have hi : VBound 0 ((1-e)⁻¹) := Or.inr (by rw [padicValRat.inv, one_sub_valuation h]; simp)
  have hx : (1-e)⁻¹-1 = e*(1-e)⁻¹ := by field_simp [one_sub_ne_zero h]; ring
  unfold OneModTwo
  rw [hx]
  simpa only [add_zero] using h.mul hi

/-- An actual denominator ratio, provided the displacement is strictly smaller two-adically. -/
theorem ratio_one {q d : ℚ} (hq : q ≠ 0) (hd : VBound 1 (d/q)) :
    OneModTwo (q/(q-d)) := by
  have he : 1-d/q = (q-d)/q := by field_simp [hq]
  have h := inv_one_sub hd
  rw [he, inv_div] at h
  exact h

theorem two_power_valuation (j : ℕ) : padicValRat 2 ((2:ℚ)^j) = (j:ℤ) := by
  rw [padicValRat.pow, show padicValRat 2 (2:ℚ)=1 from padicValRat.self (p:=2) (by decide), mul_one]

/-- Matching retained residues give the actual signed displacement's divisibility. -/
theorem displacement_bound (u x y : ℕ) (h : x % 2^u = y % 2^u) :
    VBound u (delta x y) := by
  have hm : x ≡ y [MOD 2^u] := h
  have hz := Int.modEq_iff_dvd.mp (Int.natCast_modEq_iff.mpr hm)
  have hdiv : (2:ℤ)^u ∣ (y:ℤ)-x := by exact_mod_cast hz
  simpa only [delta_as_int] using int_bound_of_dvd ((y:ℤ)-x) u hdiv

/-- Every retained bit agrees; this is the actual mask, not an abstract selector. -/
theorem matching_mask (u x y : ℕ) (h : x % 2^u = y % 2^u) : mask u x y = range u := by
  ext j
  simp only [mask, mem_filter]
  constructor
  · exact And.left
  · intro hj
    refine ⟨hj, ?_⟩
    have hb := congrArg (fun n : ℕ => n.testBit j) h
    have hju : j < u := mem_range.mp hj
    simpa [hju] using hb

/-- Nonmatching retained residues force an actual differing position below u. -/
theorem order_lt_of_residue_ne (u x y : ℕ) (h : x % 2^u ≠ y % 2^u) : order x y < u := by
  by_contra hh
  have hdiv : (2:ℤ)^u ∣ (y:ℤ)-x :=
    (padicValInt_dvd_iff (p:=2) _ _).mpr (Or.inr (by change u ≤ order x y; omega))
  have hm : x ≡ y [MOD 2^u] := by
    apply Int.natCast_modEq_iff.mp
    apply Int.modEq_iff_dvd.mpr
    exact_mod_cast hdiv
  exact h hm

/-- Actual anchor ratios are one modulo two on matching retained residue classes. -/
theorem anchor_ratio_one (u x y j : ℕ) (hj : j < u) (h : x % 2^u = y % 2^u) :
    OneModTwo (((2:ℚ)^j)/((2:ℚ)^j-delta x y)) := by
  apply ratio_one (by positivity)
  have hb := vbound_div (displacement_bound u x y h) (r:=(2:ℚ)^j)
  rw [two_power_valuation] at hb
  exact hb.mono (by omega)

end OrientationResidue
