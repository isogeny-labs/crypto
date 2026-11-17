import GeneralTorsion

namespace BKGeneralFrobeniusAudit
noncomputable section
open FrobeniusPointDescentAudit
variable {F L : Type*} [Field F] [Fintype F] [Field L] [Algebra F L] [DecidableEq L]

def linearFrobenius (W : WeierstrassCurve.Affine F) (r : ℕ) :
    Torsion (W.baseChange L) r →ₗ[ZMod r] Torsion (W.baseChange L) r :=
  linearTorsionMap W r (FiniteField.frobeniusAlgHom F L)

theorem linear_frobenius_val (W : WeierstrassCurve.Affine F) (r : ℕ)
    (P : Torsion (W.baseChange L) r) :
    (linearFrobenius W r P).val = pointFrobenius W P.val := rfl

theorem linear_frobenius_injective (W : WeierstrassCurve.Affine F) (r : ℕ) :
    Function.Injective (linearFrobenius (L := L) W r) :=
  torsionMap_injective W r _

theorem linear_frobenius_bijective [Algebra.IsAlgebraic F L]
    (W : WeierstrassCurve.Affine F) (r : ℕ) :
    Function.Bijective (linearFrobenius (L := L) W r) := by
  refine ⟨linear_frobenius_injective W r, ?_⟩
  intro P
  let e := FiniteField.frobeniusAlgEquivOfAlgebraic F L
  refine ⟨linearTorsionMap W r e.symm.toAlgHom P, ?_⟩
  apply Subtype.ext
  change WeierstrassCurve.Affine.Point.map (W' := W) (FiniteField.frobeniusAlgHom F L)
    (WeierstrassCurve.Affine.Point.map (W' := W) e.symm.toAlgHom P.val) = P.val
  rw [WeierstrassCurve.Affine.Point.map_map]
  have he : (FiniteField.frobeniusAlgHom F L).comp e.symm.toAlgHom = AlgHom.id F L := by
    ext x
    change e (e.symm x) = x
    exact e.apply_symm_apply x
  rw [he]
  cases P.val <;> rfl

def frobeniusEquiv [Algebra.IsAlgebraic F L] (W : WeierstrassCurve.Affine F) (r : ℕ) :
    Torsion (W.baseChange L) r ≃ₗ[ZMod r] Torsion (W.baseChange L) r :=
  LinearEquiv.ofBijective (linearFrobenius W r) (linear_frobenius_bijective W r)

theorem frobeniusEquiv_apply [Algebra.IsAlgebraic F L] (W : WeierstrassCurve.Affine F)
    (r : ℕ) (P : Torsion (W.baseChange L) r) :
    (frobeniusEquiv W r P).val = pointFrobenius W P.val := rfl

theorem linear_power_val (W : WeierstrassCurve.Affine F) (r k : ℕ)
    (P : Torsion (W.baseChange L) r) :
    (((linearFrobenius W r)^k) P).val = (pointFrobenius W)^[k] P.val := by
  induction k with
  | zero => rfl
  | succ k ih =>
    rw [pow_succ']
    change (linearFrobenius W r (((linearFrobenius W r)^k) P)).val = _
    rw [linear_frobenius_val, ih, Function.iterate_succ_apply']

theorem linear_power_identity_iff (W : WeierstrassCurve.Affine F) (r k : ℕ) :
    (linearFrobenius (L := L) W r)^k = 1 ↔
      ∀ P : (W.baseChange L).Point, r • P = 0 → (pointFrobenius W)^[k] P = P := by
  constructor
  · intro h P hP
    have hp := congrArg (fun f : Torsion (W.baseChange L) r →ₗ[ZMod r]
      Torsion (W.baseChange L) r => (f ⟨P,hP⟩).val) h
    simpa only [linear_power_val, Module.End.one_apply] using hp
  · intro h
    ext P
    exact (linear_power_val W r k P).trans (h P.val P.property)

theorem one_torsion_action (W : WeierstrassCurve.Affine F) :
    linearFrobenius (L := L) W 1 = 1 := by
  let : Subsingleton (Torsion (W.baseChange L) 1) := torsion_one_subsingleton _
  apply LinearMap.ext
  intro P
  exact Subsingleton.elim _ _

theorem actual_prime_fixed_vector [DecidableEq F] (W : WeierstrassCurve.Affine F)
    (r : ℕ) (hr : r.Prime) (hN : r ∣ Nat.card W.Point) :
    ∃ P : Torsion (W.baseChange L) r, P ≠ 0 ∧ linearFrobenius W r P = P := by
  obtain ⟨P,hP,_,ht⟩ := BKLinearAudit.actual_rational_prime_torsion W r hr hN
  let inc : W.Point →+ (W.baseChange L).Point := WeierstrassCurve.Affine.Point.map (W' := W) (Algebra.ofId F L)
  have hnonzero : inc P ≠ 0 := by
    intro hz
    apply hP
    exact WeierstrassCurve.Affine.Point.map_injective (f := Algebra.ofId F L)
      (hz.trans (map_zero inc).symm)
  have hkill : r • inc P = 0 := by
    simpa only [map_nsmul, map_zero] using congrArg inc ht
  refine ⟨⟨inc P,hkill⟩, ?_, ?_⟩
  · exact fun h => hnonzero (congrArg Subtype.val h)
  · apply Subtype.ext
    exact (point_frobenius_fixed_iff W (inc P)).mpr ⟨P,rfl⟩

#print axioms linear_frobenius_bijective
#print axioms linear_power_identity_iff
#print axioms actual_prime_fixed_vector
end
end BKGeneralFrobeniusAudit
