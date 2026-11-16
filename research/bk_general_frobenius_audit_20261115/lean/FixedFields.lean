import GeneralFrobenius

namespace BKGeneralFrobeniusAudit
noncomputable section
open FrobeniusPointDescentAudit FrobeniusFiniteExtensionAudit
variable {F K L : Type*} [Field F] [Fintype F] [Field K] [Fintype K] [Field L]
  [Algebra F K] [Algebra F L] [Algebra K L] [IsScalarTower F K L]
  [DecidableEq K] [DecidableEq L]

def torsionInclusion (W : WeierstrassCurve.Affine F) (r : ℕ) :
    Torsion (W.baseChange K) r →ₗ[ZMod r] Torsion (W.baseChange L) r :=
  linearTorsionMap W r ((Algebra.ofId K L).restrictScalars F)

omit [Fintype F] [Fintype K] in
theorem torsionInclusion_val (W : WeierstrassCurve.Affine F) (r : ℕ)
    (P : Torsion (W.baseChange K) r) :
    (torsionInclusion W r P).val =
      WeierstrassCurve.Affine.Point.map (W' := W) (Algebra.ofId K L) P.val := by
  change WeierstrassCurve.Affine.Point.map (W' := W)
    ((Algebra.ofId K L).restrictScalars F) P.val =
      WeierstrassCurve.Affine.Point.map (W' := W) (Algebra.ofId K L) P.val
  cases P.val <;> rfl

def fixedPower (W : WeierstrassCurve.Affine F) (r k : ℕ) :
    Submodule (ZMod r) (Torsion (W.baseChange L) r) :=
  LinearMap.ker ((linearFrobenius W r)^k - 1)

/-- Actual coefficient inclusion with codomain the actual fixed-power kernel. -/
def fixedFieldMap (W : WeierstrassCurve.Affine F) (r k : ℕ)
    (hc : Fintype.card K = Fintype.card F^k) :
    Torsion (W.baseChange K) r →ₗ[ZMod r] fixedPower (L := L) W r k :=
  (torsionInclusion W r).codRestrict _ (by
    intro P
    change ((linearFrobenius W r)^k - 1) (torsionInclusion W r P) = 0
    rw [LinearMap.sub_apply, Module.End.one_apply, sub_eq_zero]
    apply Subtype.ext
    rw [linear_power_val, torsionInclusion_val]
    exact (finite_extension_fixed_iff_of_card (K := K) W k hc _).mpr ⟨P.val,rfl⟩)

theorem fixedFieldMap_bijective (W : WeierstrassCurve.Affine F) (r k : ℕ)
    (hc : Fintype.card K = Fintype.card F^k) :
    Function.Bijective (fixedFieldMap (L := L) W r k hc) := by
  constructor
  · intro P Q h
    exact torsionMap_injective W r ((Algebra.ofId K L).restrictScalars F)
      (congrArg Subtype.val h)
  · intro Q
    have hf : ((linearFrobenius W r)^k) Q.val = Q.val := by
      have h : ((linearFrobenius W r)^k - 1) Q.val = 0 := Q.property
      simpa only [LinearMap.sub_apply, Module.End.one_apply, sub_eq_zero] using h
    have hp : (pointFrobenius W)^[k] Q.val.val = Q.val.val := by
      simpa only [linear_power_val] using congrArg Subtype.val hf
    obtain ⟨P,hP⟩ := (finite_extension_fixed_iff_of_card (K := K) W k hc Q.val.val).mp hp
    have hkill : r • P = 0 := by
      apply WeierstrassCurve.Affine.Point.map_injective (f := Algebra.ofId K L)
      rw [map_nsmul, map_zero, hP]
      exact Q.val.property
    refine ⟨⟨P,hkill⟩, ?_⟩
    apply Subtype.ext
    apply Subtype.ext
    change (torsionInclusion W r ⟨P,hkill⟩).val = Q.val.val
    rw [torsionInclusion_val]
    exact hP

def fixedFieldEquiv (W : WeierstrassCurve.Affine F) (r k : ℕ)
    (hc : Fintype.card K = Fintype.card F^k) :
    Torsion (W.baseChange K) r ≃ₗ[ZMod r] fixedPower (L := L) W r k :=
  LinearEquiv.ofBijective (fixedFieldMap W r k hc) (fixedFieldMap_bijective W r k hc)

theorem fixedPower_finite (W : WeierstrassCurve.Affine F) (r k : ℕ)
    (hc : Fintype.card K = Fintype.card F^k) : Finite (fixedPower (L := L) W r k) := by
  let : Finite (W.baseChange K).Point := BKLinearAudit.actual_rational_point_finite _
  let : Finite (Torsion (W.baseChange K) r) :=
    Finite.of_injective Subtype.val Subtype.val_injective
  exact Finite.of_equiv (Torsion (W.baseChange K) r) (fixedFieldEquiv W r k hc).toEquiv

theorem fixedPower_card (W : WeierstrassCurve.Affine F) (r k : ℕ)
    (hc : Fintype.card K = Fintype.card F^k) :
    Nat.card (fixedPower (L := L) W r k) = Nat.card (Torsion (W.baseChange K) r) :=
  (Nat.card_congr (fixedFieldEquiv W r k hc).toEquiv).symm

theorem actual_extension_iff_power_identity (W : WeierstrassCurve.Affine F) (r k : ℕ)
    (hc : Fintype.card K = Fintype.card F^k) :
    (∀ P : (W.baseChange L).Point, r • P = 0 →
      P ∈ Set.range (WeierstrassCurve.Affine.Point.map (W' := W) (Algebra.ofId K L))) ↔
      (linearFrobenius (L := L) W r)^k = 1 := by
  rw [linear_power_identity_iff]
  exact finite_extension_torsion_iff_of_card (K := K) W r k hc

#print axioms fixedFieldMap_bijective
#print axioms fixedPower_finite
#print axioms actual_extension_iff_power_identity
end
end BKGeneralFrobeniusAudit
