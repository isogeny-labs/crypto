import TriplePoint
import FrobeniusPointDescent
import Mathlib.FieldTheory.Finite.GaloisField

namespace VeluFrobeniusAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit FrobeniusPointDescentAudit

instance fivePrime : Fact (Nat.Prime 5) := ⟨by decide +kernel⟩

def baseCurve : WeierstrassCurve.Affine (ZMod 5) := short 0 1
theorem baseCurve_delta_ne : baseCurve.Δ ≠ 0 := by
  norm_num [baseCurve, short, WeierstrassCurve.Δ, WeierstrassCurve.b₂, WeierstrassCurve.b₄,
    WeierstrassCurve.b₆, WeierstrassCurve.b₈]
  exact (CharP.cast_eq_zero_iff (ZMod 5) 5 432).not.mpr (by decide)

def pointEquivOfEq {K : Type*} [Field K] [DecidableEq K]
    {A B : WeierstrassCurve.Affine K} (h : A = B) : A.Point ≃+ B.Point := by
  subst B
  exact AddEquiv.refl _

theorem pointEquivOfEq_some {K : Type*} [Field K] [DecidableEq K]
    {A B : WeierstrassCurve.Affine K} (h : A = B) (x y : K) (hN : A.Nonsingular x y) :
    pointEquivOfEq h (Point.some x y hN) = Point.some x y (h ▸ hN) := by
  subst B
  rfl

abbrev rationalInclusion (K : Type*) [Field K] [DecidableEq K] [Algebra (ZMod 5) K] :
    baseCurve.Point →+ (baseCurve.baseChange K).Point :=
  Point.map (W' := baseCurve) (Algebra.ofId (ZMod 5) K)

variable {K : Type*} [Field K] [DecidableEq K] [Algebra (ZMod 5) K] [CharP K 5]
abbrev curve := baseCurve.baseChange K

omit [DecidableEq K] [CharP K 5] in
theorem curve_eq : curve (K := K) = short (0 : K) 1 := by
  ext <;> simp [curve, baseCurve, short, WeierstrassCurve.baseChange, WeierstrassCurve.map]

omit [DecidableEq K] [CharP K 5] in
theorem curve_delta : (curve (K := K)).Δ = -432 := by
  rw [curve_eq]
  norm_num [short, WeierstrassCurve.Δ, WeierstrassCurve.b₂, WeierstrassCurve.b₄,
    WeierstrassCurve.b₆, WeierstrassCurve.b₈]

omit [DecidableEq K] in
theorem curve_delta_ne : (curve (K := K)).Δ ≠ 0 := by
  have hn : (432 : K) ≠ 0 := (CharP.cast_eq_zero_iff K 5 432).not.mpr (by decide)
  rw [curve_delta]
  exact neg_ne_zero.mpr hn

def rationalPoint : (curve (K := K)).Point := Point.some 0 1 (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_delta_ne).mp
  rw [curve_eq]
  exact (curve_equation 0 1 0 1).mpr (by norm_num [curveValue]))

def antiPoint (a : K) (ha : a ^ 2 = 2) : (curve (K := K)).Point := Point.some 1 a (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_delta_ne).mp
  rw [curve_eq]
  exact (curve_equation 0 1 1 a).mpr (by norm_num [curveValue, ha]))

theorem point_three (x y : K) (hN : (curve (K := K)).Nonsingular x y)
    (hx : (psi (0 : K) 1).IsRoot x) : (3 : ℕ) • Point.some x y hN = 0 := by
  let _ : NeZero (2 : K) := ⟨(CharP.cast_eq_zero_iff K 5 2).not.mpr (by decide)⟩
  have hn : (short (0 : K) 1).Nonsingular x y := by simpa only [curve_eq] using hN
  have hd : (short (0 : K) 1).Δ ≠ 0 := by simpa only [curve_eq] using (curve_delta_ne (K := K))
  let e := pointEquivOfEq (curve_eq (K := K))
  apply e.injective
  rw [map_nsmul, map_zero]
  change (3 : ℕ) • pointEquivOfEq (curve_eq (K := K)) (Point.some x y hN) = 0
  rw [pointEquivOfEq_some]
  exact (three_point_iff 0 1 x y hd hn).mpr hx

theorem rationalPoint_three : (3 : ℕ) • rationalPoint (K := K) = 0 := by
  unfold rationalPoint
  apply point_three
  norm_num [Polynomial.IsRoot, psi_eval]

theorem antiPoint_three (a : K) (ha : a ^ 2 = 2) : (3 : ℕ) • antiPoint a ha = 0 := by
  unfold antiPoint
  apply point_three
  have hn : (15 : K) = 0 := (CharP.cast_eq_zero_iff K 5 15).mpr (by decide)
  convert hn using 1; norm_num [Polynomial.IsRoot, psi_eval]

omit [DecidableEq K] in
theorem rationalPoint_nonzero : rationalPoint (K := K) ≠ 0 := Point.some_ne_zero _
omit [DecidableEq K] in
theorem antiPoint_nonzero (a : K) (ha : a ^ 2 = 2) : antiPoint a ha ≠ 0 := Point.some_ne_zero _

omit [DecidableEq K] in
theorem root_frobenius (a : K) (ha : a ^ 2 = 2) :
    FiniteField.frobeniusAlgHom (ZMod 5) K a = -a := by
  change a ^ Fintype.card (ZMod 5) = -a
  rw [ZMod.card]
  have h5 : (5 : K) = 0 := (CharP.cast_eq_zero_iff K 5 5).mpr (by decide)
  calc
    _ = a * (a ^ 2) ^ 2 := by ring
    _ = 4 * a := by rw [ha]; ring
    _ = -a := by linear_combination h5 * a

theorem rationalPoint_frobenius :
    pointFrobenius baseCurve (rationalPoint (K := K)) = rationalPoint := by
  simp only [pointFrobenius, rationalPoint, Point.map_some, map_zero, map_one]

theorem antiPoint_frobenius (a : K) (ha : a ^ 2 = 2) :
    pointFrobenius baseCurve (antiPoint a ha) = -antiPoint a ha := by
  simp only [pointFrobenius, antiPoint, Point.map_some, Point.neg_some, map_one,
    root_frobenius a ha]
  rw [Point.some.injEq]
  constructor
  · rfl
  · simp [curve_eq, short, WeierstrassCurve.Affine.negY]

#print axioms rationalPoint_three
#print axioms antiPoint_three
#print axioms antiPoint_frobenius
end
end VeluFrobeniusAudit
