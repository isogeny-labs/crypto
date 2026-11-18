import ActualMatrix
import GenericFrobenius
import RationalTorsion

namespace BKFrobeniusBridgeAudit
noncomputable section
open ThreeTorsionRankAudit.General FrobeniusPointDescentAudit FrobeniusFiniteExtensionAudit Module Matrix
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

/-- Actual point inclusion, with explicit fields for closed statement binding. -/
def pointInclusion (F K L : Type*) [Field F] [Field K] [Field L]
    [Algebra F K] [Algebra F L] [Algebra K L] [IsScalarTower F K L]
    [DecidableEq K] [DecidableEq L] (W : WeierstrassCurve.Affine F) :
    (W.baseChange K).Point →+ (W.baseChange L).Point :=
  WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId K L)

variable {F L : Type*} [Field F] [Fintype F] [Field L] [Algebra F L]
  [DecidableEq F] [DecidableEq L]

theorem actual_rational_fixed_vector (W : WeierstrassCurve.Affine F)
    (b : Basis (Fin 2) (ZMod 3) (Torsion (L:=L) W)) (hN : 3 ∣ Nat.card W.Point) :
    ∃ v : Fin 2 → ZMod 3, v≠0 ∧ frobeniusMatrix W b *ᵥ v=v := by
  obtain ⟨P,hP,_,ht⟩ := BKLinearAudit.actual_rational_prime_torsion W 3 (by decide) hN
  let inc : W.Point →+ (W.baseChange L).Point :=
    WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId F L)
  have hnonzero : inc P≠0 := by
    intro hz
    apply hP
    exact WeierstrassCurve.Affine.Point.map_injective
      (f:=Algebra.ofId F L) (hz.trans (map_zero inc).symm)
  have hkill : (3:ℕ) • inc P=0 := by
    simpa only [map_nsmul,map_zero] using congrArg inc ht
  let T : Torsion (L:=L) W := ⟨inc P,hkill⟩
  have hT : T≠0 := fun hz => hnonzero (congrArg Subtype.val hz)
  have hfixed : linearFrobenius W T=T := by
    apply Subtype.ext
    exact (point_frobenius_fixed_iff W (inc P)).mpr ⟨P,rfl⟩
  refine ⟨b.equivFun T,?_,?_⟩
  · intro hz
    apply hT
    exact b.equivFun.injective (hz.trans (map_zero b.equivFun).symm)
  · change frobeniusMatrix W b *ᵥ b.repr T=b.repr T
    rw [matrix_coordinates,hfixed]

theorem actual_power_criterion_of_det (W : WeierstrassCurve.Affine F)
    (b : Basis (Fin 2) (ZMod 3) (Torsion (L:=L) W)) (hN : 3 ∣ Nat.card W.Point)
    (hq : (Fintype.card F : ZMod 3)≠1)
    (hdet : (frobeniusMatrix W b).det=(Fintype.card F : ZMod 3)) (k : ℕ) :
    (frobeniusMatrix W b)^k=1 ↔ (Fintype.card F : ZMod 3)^k=1 := by
  obtain ⟨v,hv,hfix⟩ := actual_rational_fixed_vector W b hN
  exact BKLinearAudit.power_identity_iff _ _ hq hdet v hv hfix k

theorem cyclotomic_cast_iff (q k : ℕ) (hq : 0<q) :
    (q : ZMod 3)^k=1 ↔ 3 ∣ q^k-1 := by
  have hp : 1≤q^k := pow_pos hq k
  calc
    (q : ZMod 3)^k=1 ↔ (1:ZMod 3)=(q^k : ℕ) := by rw [Nat.cast_pow]; exact eq_comm
    _ ↔ Nat.ModEq 3 1 (q^k) := by
      simpa only [Nat.cast_one] using (ZMod.natCast_eq_natCast_iff 1 (q^k) 3)
    _ ↔ 3 ∣ q^k-1 := Nat.modEq_iff_dvd' hp

section ChosenField
variable {K : Type*} [Field K] [Fintype K] [Algebra F K] [Algebra K L]
  [IsScalarTower F K L] [DecidableEq K]

omit [DecidableEq F] in
theorem actual_extension_iff_matrix_power (W : WeierstrassCurve.Affine F)
    (b : Basis (Fin 2) (ZMod 3) (Torsion (L:=L) W)) (k : ℕ)
    (hc : Fintype.card K=Fintype.card F^k) :
    (∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 →
      P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId K L))) ↔
        (frobeniusMatrix W b)^k=1 := by
  rw [finite_extension_torsion_iff_of_card (K:=K) W 3 k hc]
  exact (matrix_power_identity_iff W b k).symm

theorem actual_extension_criterion_of_det (W : WeierstrassCurve.Affine F)
    (b : Basis (Fin 2) (ZMod 3) (Torsion (L:=L) W)) (hN : 3 ∣ Nat.card W.Point)
    (hq : (Fintype.card F : ZMod 3)≠1)
    (hdet : (frobeniusMatrix W b).det=(Fintype.card F : ZMod 3)) (k : ℕ)
    (hc : Fintype.card K=Fintype.card F^k) :
    (∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 →
      P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId K L))) ↔
        3 ∣ Fintype.card F^k-1 := by
  rw [actual_extension_iff_matrix_power W b k hc,actual_power_criterion_of_det W b hN hq hdet]
  exact cyclotomic_cast_iff _ k Fintype.card_pos

omit [DecidableEq F] in
theorem actual_extension_forces_det_power (W : WeierstrassCurve.Affine F)
    (b : Basis (Fin 2) (ZMod 3) (Torsion (L:=L) W)) (k : ℕ)
    (hc : Fintype.card K=Fintype.card F^k)
    (h : ∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 →
      P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=W) (Algebra.ofId K L))) :
    (frobeniusMatrix W b).det^k=1 := by
  have hm := (actual_extension_iff_matrix_power W b k hc).mp h
  have hd := congrArg Matrix.det hm
  simpa only [Matrix.det_pow,Matrix.det_one] using hd
end ChosenField

#print axioms actual_rational_fixed_vector
#print axioms actual_extension_iff_matrix_power
#print axioms actual_extension_criterion_of_det
#print axioms actual_extension_forces_det_power
end
end BKFrobeniusBridgeAudit
