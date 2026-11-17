import FrobeniusImports
import FieldNormCertificate

namespace IsogenyFactorizationGuardAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit FunctionDivisorFinitenessAudit EllipticTameSymbolAudit MillerFinitePointAudit
open EllipticTranslationAudit MillerBasechangeAudit
variable {K : Type*} [Field K] [Fintype K]

def fieldFrobenius (W : WeierstrassCurve.Affine K) : W.FunctionField →ₐ[K] W.FunctionField :=
  FiniteField.frobeniusAlgHom K W.FunctionField

def coordinateFrobenius (W : WeierstrassCurve.Affine K) : W.CoordinateRing →ₐ[K] W.CoordinateRing :=
  FiniteField.frobeniusAlgHom K W.CoordinateRing

theorem field_frobenius_apply (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    fieldFrobenius W f = f ^ Fintype.card K := rfl

theorem field_frobenius_constants (W : WeierstrassCurve.Affine K) (c : K) :
    fieldFrobenius W (constantHom W c) = constantHom W c :=
  (fieldFrobenius W).commutes c

theorem field_frobenius_injective (W : WeierstrassCurve.Affine K) :
    Function.Injective (fieldFrobenius W) := (fieldFrobenius W).injective

theorem coordinate_field_frobenius (W : WeierstrassCurve.Affine K) (a : W.CoordinateRing) :
    algebraMap W.CoordinateRing W.FunctionField (coordinateFrobenius W a) =
      fieldFrobenius W (algebraMap W.CoordinateRing W.FunctionField a) := map_pow _ _ _

omit [Fintype K] in
/-- The actual order-one parameter cannot be a higher power in the actual function field. -/
theorem parameter_not_power (W : WeierstrassCurve.Affine K) (q : Nat) (hq : 2 ≤ q) :
    ¬ ∃ f : W.FunctionField, f^q = (infinityParameterUnit W : W.FunctionField) := by
  rintro ⟨f,hf⟩
  have hqn : q ≠ 0 := by omega
  have hfn : f ≠ 0 := by
    intro hz
    rw [hz,zero_pow hqn] at hf
    exact (infinityParameterUnit W).ne_zero hf.symm
  let u : W.FunctionFieldˣ := Units.mk0 f hfn
  have hu : ((u^(q : Int) : W.FunctionFieldˣ) : W.FunctionField) =
      (infinityParameterUnit W : W.FunctionField) := by
    simpa only [zpow_natCast,Units.val_pow_eq_pow_val,Units.val_mk0,u] using hf
  have ho := order_zpow W (0 : W.Point) u (q : Int)
  rw [hu,infinity_parameter_order] at ho
  have hqi : (2 : Int) ≤ q := by exact_mod_cast hq
  by_cases hk : pointOrder W (u : W.FunctionField) 0 ≤ 0
  · have hm := mul_nonpos_of_nonneg_of_nonpos (show (0 : Int) ≤ q by omega) hk
    omega
  · have hk1 : (1 : Int) ≤ pointOrder W (u : W.FunctionField) 0 := by omega
    have hm := mul_le_mul_of_nonneg_left hk1 (show (0 : Int) ≤ q by omega)
    simp only [mul_one] at hm
    omega

theorem field_frobenius_not_surjective (W : WeierstrassCurve.Affine K) :
    ¬ Function.Surjective (fieldFrobenius W) := by
  intro h
  obtain ⟨f,hf⟩ := h (infinityParameterUnit W)
  exact parameter_not_power W (Fintype.card K) Fintype.one_lt_card ⟨f,hf⟩

theorem no_field_right_inverse (W : WeierstrassCurve.Affine K) :
    ¬ ∃ ell : W.FunctionField →+* W.FunctionField,
      (fieldFrobenius W).toRingHom.comp ell = RingHom.id W.FunctionField := by
  rintro ⟨ell,hfactor⟩
  apply field_frobenius_not_surjective W
  intro f
  exact ⟨ell f,RingHom.congr_fun hfactor f⟩

def frobeniusImage (W : WeierstrassCurve.Affine K) : IntermediateField K W.FunctionField :=
  (fieldFrobenius W).fieldRange

theorem frobenius_image_purely_inseparable (W : WeierstrassCurve.Affine K) :
    IsPurelyInseparable (frobeniusImage W) W.FunctionField := by
  obtain ⟨p,hp,n,hprime,hcard⟩ := FiniteField.card' K
  have : CharP K p := hp
  have : CharP (frobeniusImage W) p := charP_of_injective_algebraMap' K p
  have : ExpChar (frobeniusImage W) p := ExpChar.prime hprime
  apply (isPurelyInseparable_iff_pow_mem (frobeniusImage W) p).mpr
  intro x
  refine ⟨n,?_⟩
  change ∃ y : frobeniusImage W, (y : W.FunctionField) = x ^ p ^ (n : Nat)
  refine ⟨⟨fieldFrobenius W x,⟨x,rfl⟩⟩,?_⟩
  exact congrArg (fun m : Nat => x^m) hcard

theorem frobenius_image_not_separable (W : WeierstrassCurve.Affine K) :
    ¬ Algebra.IsSeparable (frobeniusImage W) W.FunctionField := by
  intro hsep
  have := hsep
  have := frobenius_image_purely_inseparable W
  have hs := IsPurelyInseparable.surjective_algebraMap_of_isSeparable
    (frobeniusImage W) W.FunctionField
  apply field_frobenius_not_surjective W
  intro x
  obtain ⟨y,hy⟩ := hs x
  obtain ⟨z,hz⟩ := y.property
  exact ⟨z,hz.trans hy⟩

#print axioms parameter_not_power
#print axioms no_field_right_inverse
#print axioms frobenius_image_not_separable
end
end IsogenyFactorizationGuardAudit
