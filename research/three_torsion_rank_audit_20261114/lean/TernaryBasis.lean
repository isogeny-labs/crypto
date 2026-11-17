import GeometricCardinality
import Mathlib.Algebra.Module.ZMod
import Mathlib.FieldTheory.Finiteness

namespace ThreeTorsionRankAudit
open Module
noncomputable section
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {K : Type*} [Field K] [DecidableEq K] [NeZero (2 : K)]

omit [NeZero (2 : K)] in
theorem torsion_three_killed (a b : K) (P : ThreeTorsion a b) : (3 : ℕ) • P=0 := by
  apply Subtype.ext
  change (3 : ℕ) • P.val=0
  exact P.property

instance torsionModule (a b : K) : Module (ZMod 3) (ThreeTorsion a b) :=
  AddCommGroup.zmodModule (torsion_three_killed a b)

theorem geometric_three_finrank [IsAlgClosed K] [NeZero (3 : K)]
    (a b : K) (hΔ : (short a b).Δ ≠ 0) : Module.finrank (ZMod 3) (ThreeTorsion a b)=2 := by
  let : Finite (ThreeTorsion a b) := torsion_finite a b hΔ
  have hc := Module.natCard_eq_pow_finrank (K := ZMod 3) (V := ThreeTorsion a b)
  rw [geometric_three_torsion_card a b hΔ] at hc
  have hf : Nat.card (ZMod 3)=3 := by simp
  rw [hf] at hc
  apply Nat.pow_right_injective (by decide : 2 ≤ 3)
  exact hc.symm

def geometricThreeBasis [IsAlgClosed K] [NeZero (3 : K)]
    (a b : K) (hΔ : (short a b).Δ ≠ 0) : Basis (Fin 2) (ZMod 3) (ThreeTorsion a b) := by
  let : Finite (ThreeTorsion a b) := torsion_finite a b hΔ
  exact Module.finBasisOfFinrankEq (ZMod 3) (ThreeTorsion a b) (geometric_three_finrank a b hΔ)

theorem geometric_three_basis_exists [IsAlgClosed K] [NeZero (3 : K)]
    (a b : K) (hΔ : (short a b).Δ ≠ 0) : Nonempty (Basis (Fin 2) (ZMod 3) (ThreeTorsion a b)) :=
  ⟨geometricThreeBasis a b hΔ⟩

omit [DecidableEq K] [NeZero (2 : K)] in
theorem scalar_map_ne_zero {L : Type*} [Field L] (f : K →+* L) (n : ℕ) [NeZero (n : K)] :
    NeZero (n : L) := by
  constructor
  have h := f.injective.ne (NeZero.ne (n : K))
  simpa only [map_natCast,map_zero] using h

omit [DecidableEq K] [NeZero (2 : K)] in
theorem short_map {L : Type*} [Field L] (f : K →+* L) (a b : K) :
    (short a b).map f = short (f a) (f b) := by
  ext <;> simp [short,WeierstrassCurve.map]

omit [DecidableEq K] in
theorem scalar_extension_card [NeZero (3 : K)] {L : Type*} [Field L] [DecidableEq L] [IsAlgClosed L]
    (f : K →+* L) (a b : K) (hΔ : (short a b).Δ ≠ 0) :
    Nat.card (ThreeTorsion (f a) (f b))=9 := by
  let := scalar_map_ne_zero f 2
  let := scalar_map_ne_zero f 3
  apply geometric_three_torsion_card
  rw [← short_map f a b,WeierstrassCurve.map_Δ]
  simpa only [map_zero] using f.injective.ne hΔ

end
end ThreeTorsionRankAudit
