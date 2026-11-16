import CoverDecodeExample
import Mathlib.Tactic.ComputeDegree

/-!
# A fixed integral model realizing the two specializations

Use A = Z[z]/(z²+z+1) and t₀ = 2 + 65537*z. The source specialization
lands in F_(65537²), with its model and marked points defined over F_65537.
The target specialization lands in the actual four-element field.
These are ring maps from a common integral model, not a field map between
different characteristics. No Jacobian functor is postulated here.
-/

noncomputable section
namespace CoverIntegralLift

open CoverDecodeExample (Small w)

abbrev SourceResidue := GaloisField 65537 2

instance : DecidableEq SourceResidue := Classical.decEq _

theorem exists_source_root : ∃ z : SourceResidue, IsPrimitiveRoot z 3 := by
  classical
  letI : Fintype SourceResidue := Fintype.ofFinite _
  have hcard : Fintype.card SourceResidue = 65537 ^ 2 := by
    rw [← Nat.card_eq_fintype_card]
    exact GaloisField.card 65537 2 (by decide)
  obtain ⟨u, hu⟩ := exists_prime_orderOf_dvd_card (G := SourceResidueˣ) 3
    (by rw [Fintype.card_units, hcard]; norm_num)
  refine ⟨(u : SourceResidue), ?_⟩
  rw [IsPrimitiveRoot.coe_units_iff]
  simpa only [hu] using IsPrimitiveRoot.orderOf u

def zSource : SourceResidue := Classical.choose exists_source_root

theorem source_root_relation : zSource ^ 2 + zSource + 1 = 0 := by
  have h := (Classical.choose_spec exists_source_root).geom_sum_eq_zero (by decide : 1 < 3)
  simpa only [zSource, Finset.sum_range_succ, Finset.sum_range_zero, pow_zero,
    pow_one, zero_add, add_comm, add_left_comm, add_assoc] using h

theorem target_root_relation : w ^ 2 + w + 1 = 0 :=
  Subtype.ext CrossCharacteristicLarger.omega_relation

def zPolynomial : Polynomial ℤ := Polynomial.X ^ 2 + Polynomial.X + 1

abbrev LiftRing := AdjoinRoot zPolynomial

theorem zPolynomial_degree : zPolynomial.degree = 2 := by
  unfold zPolynomial
  compute_degree <;> norm_num [WithBot.unbotD] <;> decide

instance : CharZero LiftRing :=
  charZero_of_injective_ringHom
    (AdjoinRoot.of.injective_of_degree_ne_zero (f := zPolynomial)
      (by rw [zPolynomial_degree]; decide))

theorem integral_root_relation : AdjoinRoot.root zPolynomial ^ 2 +
    AdjoinRoot.root zPolynomial + 1 = 0 := by
  simpa [zPolynomial] using AdjoinRoot.eval₂_root zPolynomial

def toSource : LiftRing →+* SourceResidue :=
  AdjoinRoot.lift (Int.castRingHom SourceResidue) zSource (by
    simpa [zPolynomial] using source_root_relation)

def toTarget : LiftRing →+* Small :=
  AdjoinRoot.lift (Int.castRingHom Small) w (by
    simpa [zPolynomial] using target_root_relation)

def parameter : LiftRing := 2 + 65537 * AdjoinRoot.root zPolynomial

theorem parameter_source : toSource parameter = 2 := by
  simp only [parameter, map_add, map_mul, map_ofNat]
  have hp : (65537 : SourceResidue) = 0 := CharP.cast_eq_zero SourceResidue 65537
  rw [hp, zero_mul, add_zero]

theorem parameter_target : toTarget parameter = w := by
  have hp : (65537 : Small) = 1 := by simpa using CharP.cast_eq_mod Small 2 65537
  have htwo : (2 : Small) = 0 := CharTwo.two_eq_zero
  simp only [parameter, map_add, map_mul, map_ofNat, htwo, hp, one_mul, zero_add]
  simp [toTarget]

def integralModel : WeierstrassCurve LiftRing :=
  ⟨1 - (parameter ^ 2 - parameter), -(parameter ^ 3 - parameter ^ 2),
    -(parameter ^ 3 - parameter ^ 2), 0, 0⟩

theorem source_model : integralModel.map toSource =
    CrossCharacteristicLarger.sourceCurve.map (algebraMap (ZMod 65537) SourceResidue) := by
  ext1 <;> simp [integralModel, WeierstrassCurve.map, parameter_source,
    CrossCharacteristicLarger.sourceCurve] <;> norm_num <;> simp only [map_ofNat]

theorem target_model : integralModel.map toTarget = CoverDecodeExample.curve := by
  ext1 <;> simp only [integralModel, WeierstrassCurve.map, parameter_target,
    CoverDecodeExample.curve, map_sub, map_pow, map_neg, map_one, map_zero,
    CoverDecodeExample.target_parameters.1, CoverDecodeExample.target_parameters.2,
    sub_self, CharTwo.neg_eq]

theorem source_discriminant : toSource integralModel.Δ ≠ 0 := by
  rw [← WeierstrassCurve.map_Δ, source_model]
  exact isUnit_iff_ne_zero.mp
    (CrossCharacteristicLarger.sourceCurve.map
      (algebraMap (ZMod 65537) SourceResidue)).isUnit_Δ

theorem target_discriminant : toTarget integralModel.Δ ≠ 0 := by
  rw [← WeierstrassCurve.map_Δ, target_model]
  exact isUnit_iff_ne_zero.mp CoverDecodeExample.curve.isUnit_Δ

theorem concrete_good_specializations :
    integralModel.map toSource =
      CrossCharacteristicLarger.sourceCurve.map (algebraMap (ZMod 65537) SourceResidue) ∧
    integralModel.map toTarget = CoverDecodeExample.curve ∧
    toSource integralModel.Δ ≠ 0 ∧ toTarget integralModel.Δ ≠ 0 :=
  ⟨source_model, target_model, source_discriminant, target_discriminant⟩

end CoverIntegralLift
