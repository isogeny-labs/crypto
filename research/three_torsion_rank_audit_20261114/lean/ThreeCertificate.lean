import TernaryBasis
import CharacteristicThree

namespace ThreeTorsionRankAudit

theorem actual_geometric_three_torsion_certificate :
    (∀ (K : Type) [Field K] [DecidableEq K] [NeZero (2 : K)] [NeZero (3 : K)] [IsAlgClosed K],
      ∀ a b : K, (short a b).Δ ≠ 0 →
        (psi a b).natDegree=4 ∧ (psi a b).Separable ∧ Nat.card (Root a b)=4 ∧
        Nat.card (Fibers a b)=8 ∧ Nat.card (ThreeTorsion a b)=9 ∧
        Module.finrank (ZMod 3) (ThreeTorsion a b)=2 ∧
        Nonempty (Module.Basis (Fin 2) (ZMod 3) (ThreeTorsion a b)) ∧
        (∀ x y : K, ∀ hN : (short a b).Nonsingular x y,
          (3 : ℕ) • WeierstrassCurve.Affine.Point.some x y hN=0 ↔ (psi a b).IsRoot x)) ∧
    (∀ (K L : Type) [Field K] [Field L] [DecidableEq L] [NeZero (2 : K)] [NeZero (3 : K)] [IsAlgClosed L],
      ∀ (f : K →+* L) (a b : K), (short a b).Δ ≠ 0 → Nat.card (ThreeTorsion (f a) (f b))=9) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [NeZero (2 : K)] [CharP K 3],
      (short (-1 : K) 0).Δ ≠ 0 ∧ Nat.card (ThreeTorsion (-1 : K) 0)=1 ∧
        (∀ P : (short (-1 : K) 0).Point, (3 : ℕ) • P=0 → P=0)) ∧
    (short (-1 : CharThreeClosure) 0).Δ ≠ 0 ∧
    Nat.card (ThreeTorsion (-1 : CharThreeClosure) 0)=1 ∧
    Nat.card (ThreeTorsion (-1 : CharThreeClosure) 0)≠9 := by
  refine ⟨?_,?_,?_,actual_char_three_geometric_control⟩
  · intro K _ _ _ _ _ a b hΔ
    exact ⟨psi_degree a b,psi_separable a b hΔ,root_card_four a b hΔ,fibers_card_eight a b hΔ,
      geometric_three_torsion_card a b hΔ,geometric_three_finrank a b hΔ,
      geometric_three_basis_exists a b hΔ,fun x y hN => three_point_iff a b x y hΔ hN⟩
  · intro K L _ _ _ _ _ _ f a b hΔ
    exact scalar_extension_card f a b hΔ
  · intro K _ _ _ _
    exact ⟨char_three_discriminant,char_three_torsion_card,char_three_all_zero⟩

#print axioms actual_geometric_three_torsion_certificate

end ThreeTorsionRankAudit
