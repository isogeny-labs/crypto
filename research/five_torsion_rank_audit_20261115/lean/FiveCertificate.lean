import FiveControls

namespace FiveTorsionRankAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine ThreeTorsionRankAudit ThreeTorsionRankAudit.General Module
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩

theorem actual_geometric_five_certificate :
    (∀ (R : Type) [CommRing R] (W : WeierstrassCurve R),
      psiFive W=W.Ψ₂Sq^2*W.preΨ₄-W.Ψ₃^3 ∧
      (psiFive W).derivative=5*W.Ψ₂Sq*(central W*W.preΨ₄-W.Ψ₃^2)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K)
      (x y : K) (hN : W.Nonsingular x y),
      (5:ℕ) • Point.some x y hN=0 ↔ (psiFive W).IsRoot x) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [IsAlgClosed K] [NeZero (5:K)]
      (W : WeierstrassCurve.Affine K), W.Δ≠0 →
      (psiFive W).Separable ∧ (psiFive W).natDegree=12 ∧ Nat.card (FiveRoot W)=12 ∧
      Finite (FiveTorsion W) ∧ Nat.card (FiveTorsion W)=25 ∧
      Nat.card {P : W.Point // addOrderOf P=5}=24 ∧ finrank (ZMod 5) (FiveTorsion W)=2 ∧
      Nonempty (Basis (Fin 2) (ZMod 5) (FiveTorsion W))) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [CharP K 5],
      (charFiveCurve K).Δ≠0 ∧ (∀ x : K, (psiFive (charFiveCurve K)).eval x= -1) ∧
      Nat.card (FiveTorsion (charFiveCurve K))=1 ∧ Nat.card (FiveTorsion (charFiveCurve K))≠25) ∧
    (CharP BinaryClosure 2 ∧ binaryCurve.Δ≠0 ∧ Nat.card (FiveTorsion binaryCurve)=25 ∧
      finrank (ZMod 5) (FiveTorsion binaryCurve)=2) ∧
    (CharP CharThreeClosure 3 ∧ (short (-1:CharThreeClosure) 0).Δ≠0 ∧
      Nat.card (FiveTorsion (short (-1:CharThreeClosure) 0))=25 ∧
      finrank (ZMod 5) (FiveTorsion (short (-1:CharThreeClosure) 0))=2) := by
  refine ⟨?_,?_,?_,?_,binary_five_positive,ternary_five_positive⟩
  · intro R _ W
    exact ⟨five_formula W,five_derivative W⟩
  · intro K _ _ W x y hN
    exact five_point_iff W hN
  · intro K _ _ _ _ W hd
    exact ⟨five_separable W hd,five_degree W,five_root_card W hd,five_torsion_finite W hd,
      geometric_five_card W hd,geometric_five_exact_card W hd,geometric_five_finrank W hd,
      ⟨geometricFiveBasis W hd⟩⟩
  · intro K _ _ _
    have hc := charFive_card (K:=K)
    exact ⟨charFive_discriminant,charFive_eval,hc,by rw [hc]; decide⟩

#print axioms actual_geometric_five_certificate
end
end FiveTorsionRankAudit
