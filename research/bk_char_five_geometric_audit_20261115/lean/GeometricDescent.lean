module
public import BaseCurve
public import Mathlib.FieldTheory.IsAlgClosed.AlgebraicClosure

@[expose] public section
namespace BKCharFiveGeometricAudit
noncomputable section
open WeierstrassCurve.Affine
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing F := (inferInstance : Field F).toCommRing
local instance : DecidableEq (AlgebraicClosure F) := Classical.decEq _
variable (K : Type*) [Field K] [DecidableEq K] [Algebra F K]
local instance extensionChar : CharP K 5 := charP_of_injective_algebraMap' F 5

abbrev geometricCurve := (curve F).baseChange K
abbrev GeometricFiveTorsion := {P : (geometricCurve K).Point // (5 : ℕ) • P = 0}

def baseMap : (curve F).Point →+ (geometricCurve K).Point :=
  Point.map (W' := curve F) (Algebra.ofId F K)

omit [DecidableEq K] in
theorem geometric_curve_eq : geometricCurve K = curve K := by
  simp [geometricCurve, curve, WeierstrassCurve.baseChange, WeierstrassCurve.map]
  exact ⟨map_ofNat (algebraMap F K) 3, map_ofNat (algebraMap F K) 2⟩

def pointCurveCongr {W V : WeierstrassCurve.Affine K} (h : W = V) : W.Point ≃+ V.Point := by
  subst V
  exact AddEquiv.refl _

omit [Algebra F K] in
theorem point_curve_congr_some {W V : WeierstrassCurve.Affine K} (h : W = V)
    {x y : K} (hN : W.Nonsingular x y) :
    pointCurveCongr K h (Point.some x y hN) = Point.some x y (h ▸ hN) := by
  cases h
  rfl

/-- Actual point-map descent for every extension, including the algebraic closure. -/
theorem all_geometric_five_rational (P : (geometricCurve K).Point) (ht : (5 : ℕ) • P = 0) :
    ∃ Q : (curve F).Point, baseMap K Q = P := by
  cases P with
  | zero => exact ⟨0, rfl⟩
  | some x y hN =>
    have hN' : (curve K).Nonsingular x y := geometric_curve_eq K ▸ hN
    have ht' : (5 : ℕ) • Point.some x y hN' = 0 := by
      have h := congrArg (pointCurveCongr K (geometric_curve_eq K)) ht
      simpa only [map_nsmul, map_zero, point_curve_congr_some] using h
    obtain ⟨hx, hy⟩ := five_coordinates hN' ht'
    rcases hx with hx | hx <;> rcases hy with hy | hy
    · refine ⟨basePoint false false, ?_⟩
      change Point.some (algebraMap F K 1) (algebraMap F K 1) _ = Point.some x y hN
      exact Eq.mpr (Point.some.injEq _ _ _ _ _ _)
        ⟨by simpa using hx.symm, by simpa using hy.symm⟩
    · refine ⟨basePoint false true, ?_⟩
      change Point.some (algebraMap F K 1) (algebraMap F K (-1)) _ = Point.some x y hN
      exact Eq.mpr (Point.some.injEq _ _ _ _ _ _)
        ⟨by simpa using hx.symm, by simpa using hy.symm⟩
    · refine ⟨basePoint true false, ?_⟩
      change Point.some (algebraMap F K 2) (algebraMap F K 1) _ = Point.some x y hN
      exact Eq.mpr (Point.some.injEq _ _ _ _ _ _)
        ⟨(map_ofNat (algebraMap F K) 2).trans hx.symm, by simpa using hy.symm⟩
    · refine ⟨basePoint true true, ?_⟩
      change Point.some (algebraMap F K 2) (algebraMap F K (-1)) _ = Point.some x y hN
      exact Eq.mpr (Point.some.injEq _ _ _ _ _ _)
        ⟨(map_ofNat (algebraMap F K) 2).trans hx.symm, by simpa using hy.symm⟩

def baseIntoTorsion (Q : (curve F).Point) : GeometricFiveTorsion K :=
  ⟨baseMap K Q, by
    simpa only [map_nsmul, map_zero] using congrArg (baseMap K) (all_base_points_five Q)⟩

theorem base_into_torsion_bijective : Function.Bijective (baseIntoTorsion K) := by
  constructor
  · intro P Q h
    apply Point.map_injective (W' := curve F) (Algebra.ofId F K)
    exact congrArg Subtype.val h
  · intro ⟨P, hP⟩
    obtain ⟨Q, hQ⟩ := all_geometric_five_rational K P hP
    exact ⟨Q, Subtype.ext hQ⟩

def geometricTorsionEquiv : (curve F).Point ≃ GeometricFiveTorsion K :=
  Equiv.ofBijective (baseIntoTorsion K) (base_into_torsion_bijective K)

theorem geometric_five_card : Nat.card (GeometricFiveTorsion K) = 5 := by
  rw [Nat.card_congr (geometricTorsionEquiv K).symm, base_count]

def allGeometricFiveRational : Prop :=
  ∀ P : (geometricCurve (AlgebraicClosure F)).Point, (5 : ℕ) • P = 0 →
    ∃ Q : (curve F).Point, baseMap (AlgebraicClosure F) Q = P

theorem geometric_five_rational_closed : allGeometricFiveRational :=
  all_geometric_five_rational (AlgebraicClosure F)

theorem inclusion_paraphrase_false :
    ¬ (allGeometricFiveRational ↔ 5 ∣ (5 : ℕ) - 1) := by
  intro h
  have hd := h.mp (all_geometric_five_rational (AlgebraicClosure F))
  norm_num at hd

theorem no_modular_embedding (k : ℕ) (hk : 0 < k) : ¬ 5 ∣ 5 ^ k - 1 := by
  intro hd
  have he := (ZMod.natCast_eq_zero_iff (5 ^ k - 1) 5).mpr hd
  rw [Nat.cast_sub (Nat.one_le_pow _ _ (by decide : 0 < 5)), Nat.cast_one,
    Nat.cast_pow, show ((5 : ℕ) : ZMod 5) = 0 by decide,
    zero_pow (Nat.ne_of_gt hk), zero_sub] at he
  exact neg_ne_zero.mpr one_ne_zero he

variable [Fintype K]

omit [DecidableEq K] in
theorem five_not_dvd_card_minus_one : ¬ 5 ∣ Fintype.card K - 1 := by
  intro hd
  have he := (CharP.cast_eq_zero_iff K 5 _).mpr hd
  rw [Nat.cast_sub (Nat.one_le_iff_ne_zero.mpr Fintype.card_ne_zero), Nat.cast_one,
    FiniteField.cast_card_eq_zero, zero_sub] at he
  exact neg_ne_zero.mpr one_ne_zero he

/-- The original r-squared count criterion survives in every actual finite extension. -/
theorem original_count_criterion :
    Nat.card (GeometricFiveTorsion K) = 5 ^ 2 ↔ 5 ∣ Fintype.card K - 1 := by
  rw [geometric_five_card]
  exact iff_of_false (by decide) (five_not_dvd_card_minus_one K)

#print axioms all_geometric_five_rational
#print axioms geometricTorsionEquiv
#print axioms geometric_five_card
#print axioms inclusion_paraphrase_false
#print axioms original_count_criterion
end
end BKCharFiveGeometricAudit
