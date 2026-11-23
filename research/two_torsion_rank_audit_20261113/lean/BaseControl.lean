import CharTwoModels
import Mathlib.FieldTheory.Finite.GaloisField
import Mathlib.FieldTheory.IsAlgClosed.AlgebraicClosure

namespace TwoTorsionRankAudit
open Polynomial WeierstrassCurve.Affine
noncomputable section
private instance primeTwo : Fact (Nat.Prime 2) := ⟨by decide +kernel⟩
abbrev Base := ZMod 2

def coordinates {K : Type*} [Field K] (W : WeierstrassCurve.Affine K) :
    W.Point → Option (K×K)
  | .zero => none
  | .some x y _ => some (x,y)

theorem coordinates_injective {K : Type*} [Field K] (W : WeierstrassCurve.Affine K) :
    Function.Injective (coordinates W) := by
  intro P Q h
  cases P <;> cases Q <;> simp_all [coordinates]

theorem actual_point_finite {K : Type*} [Field K] [Finite K]
    (W : WeierstrassCurve.Affine K) : Finite W.Point :=
  Finite.of_injective (coordinates W) (coordinates_injective W)

def extraBasePoint : (ordinaryModel Base).Point := Point.some 1 0 (by
  apply ((ordinaryModel Base).equation_iff_nonsingular_of_Δ_ne_zero
    (by rw [ordinary_delta]; exact one_ne_zero)).mp
  norm_num [WeierstrassCurve.Affine.equation_iff,ordinaryModel]
  exact (CharP.cast_eq_zero Base 2).symm)

def baseThree (i : Fin 3) : (ordinaryModel Base).Point :=
  if i=0 then 0 else if i=1 then ordinaryPoint Base else extraBasePoint

theorem baseThree_injective : Function.Injective baseThree := by
  intro i j h
  fin_cases i <;> fin_cases j <;>
    simp_all [baseThree,ordinaryPoint,extraBasePoint,Point.zero_def,Point.some.injEq]

theorem ordinary_base_point_card : Nat.card (ordinaryModel Base).Point=4 := by
  letI := actual_point_finite (ordinaryModel Base)
  have hi := Nat.card_le_card_of_injective (coordinates (ordinaryModel Base))
    (coordinates_injective (ordinaryModel Base))
  have hu : Nat.card (Option (Base×Base))=5 := by
    norm_num [Nat.card_eq_fintype_card,Fintype.card_option,Fintype.card_prod,ZMod.card]
  rw [hu] at hi
  have hl := Nat.card_le_card_of_injective baseThree baseThree_injective
  have hc : Nat.card (Fin 3)=3 := by simp
  rw [hc] at hl
  obtain ⟨n,hn⟩ := ordinary_two_divides_point_card (K:=Base)
  omega

theorem two_not_dvd_power_sub_one (k : ℕ) (hk : 0<k) : ¬2∣2^k-1 := by
  cases k with
  | zero => omega
  | succ n =>
    rw [pow_succ]
    intro h
    obtain ⟨m,hm⟩ := h
    have hp : 0<2^n := pow_pos (by decide) n
    omega

abbrev BinaryField (k : ℕ) := GaloisField 2 k
noncomputable instance binaryDecidableEq (k : ℕ) : DecidableEq (BinaryField k) := Classical.decEq _

abbrev Geometric := AlgebraicClosure Base
noncomputable instance geometricDecidableEq : DecidableEq Geometric := Classical.decEq _
def baseEmbedding : Base →+* Geometric := algebraMap _ _

theorem actual_geometric_card : Nat.card (TwoTorsion (ordinaryModel Geometric))=2 :=
  ordinary_torsion_card

theorem actual_geometric_torsion_in_base (Q : (ordinaryModel Geometric).Point) (hQ : 2 • Q=0) :
    ∃ P : (ordinaryModel Base).Point,2 • P=0 ∧ Q=ordinaryMap baseEmbedding P :=
  ordinary_all_torsion_in_image baseEmbedding Q hQ

theorem binary_field_card (k : ℕ) (hk : 0<k) : Nat.card (BinaryField k)=2^k :=
  GaloisField.card 2 k (by omega)

theorem binary_all_geometric_torsion_rational (k : ℕ) (hk : 0<k)
    (L : Type*) [Field L] [DecidableEq L] [CharP L 2] [IsAlgClosed L]
    (f : BinaryField k →+* L) (Q : (ordinaryModel L).Point) (hQ : 2 • Q=0) :
    ∃ P : (ordinaryModel (BinaryField k)).Point,2 • P=0 ∧ Q=ordinaryMap f P := by
  classical
  exact ordinary_all_torsion_in_image f Q hQ

/-- Delete coprimality with the characteristic, while retaining the actual
prime/point-count and q-1 conditions. Source/node coprimality is necessary. -/
theorem characteristic_point_inclusion_boundary :
    Nat.Prime 2 ∧ Nat.card (ordinaryModel Base).Point=4 ∧
    2∣Nat.card (ordinaryModel Base).Point ∧ 2^2∣Nat.card (ordinaryModel Base).Point ∧
    ¬2∣2-1 ∧ Nat.gcd 2 2≠1 ∧
    (∀ k : ℕ,0<k → Nat.card (BinaryField k)=2^k ∧ ¬2∣2^k-1) ∧
    (∀ (L : Type) [Field L] [DecidableEq L] [CharP L 2] [IsAlgClosed L]
      (f : Base →+* L) (Q : (ordinaryModel L).Point),
      2 • Q=0 → ∃ P : (ordinaryModel Base).Point,2 • P=0 ∧ Q=ordinaryMap f P) := by
  refine ⟨by decide +kernel,ordinary_base_point_card,?_,?_,by decide +kernel,
    by decide +kernel,?_,?_⟩
  · rw [ordinary_base_point_card]
    decide +kernel
  · rw [ordinary_base_point_card]
    decide +kernel
  · intro k hk
    exact ⟨binary_field_card k hk,two_not_dvd_power_sub_one k hk⟩
  · intro L _ _ _ _ f Q hQ
    exact ordinary_all_torsion_in_image f Q hQ

theorem universal_geometric_point_cyclotomic_criterion_false :
    ¬ ((∀ (L : Type) [Field L] [DecidableEq L] [CharP L 2] [IsAlgClosed L]
      (f : Base →+* L) (Q : (ordinaryModel L).Point),
      2 • Q=0 → ∃ P : (ordinaryModel Base).Point,2 • P=0 ∧ Q=ordinaryMap f P) ↔ 2∣2-1) := by
  intro h
  have hf := h.mp characteristic_point_inclusion_boundary.2.2.2.2.2.2.2
  exact (by decide +kernel : ¬2∣2-1) hf

theorem concrete_geometric_point_criterion_false :
    ¬ ((∀ Q : (ordinaryModel Geometric).Point,2 • Q=0 →
      ∃ P : (ordinaryModel Base).Point,2 • P=0 ∧ Q=ordinaryMap baseEmbedding P) ↔ 2∣2-1) := by
  intro h
  exact (by decide +kernel : ¬2∣2-1) (h.mp actual_geometric_torsion_in_base)

#print axioms characteristic_point_inclusion_boundary
#print axioms universal_geometric_point_cyclotomic_criterion_false
end
end TwoTorsionRankAudit
