import SevenControls

namespace SevenTorsionRankAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine ThreeTorsionRankAudit ThreeTorsionRankAudit.General Module
private instance : Fact (Nat.Prime 7) := ⟨by decide⟩

theorem actual_geometric_seven_certificate :
    (∀ (R : Type) [CommRing R] (W : WeierstrassCurve R),
      psiSeven W=FiveTorsionRankAudit.psiFive W*W.Ψ₃^3-W.Ψ₂Sq^2*W.preΨ₄^3 ∧
      (psiSeven W).derivative=7*W.Ψ₂Sq*sevenAux W) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K)
      (x y : K) (hN : W.Nonsingular x y),
      (7:ℕ) • Point.some x y hN=0 ↔ (psiSeven W).IsRoot x) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [IsAlgClosed K] [NeZero (7:K)]
      (W : WeierstrassCurve.Affine K), W.Δ≠0 →
      (psiSeven W).Separable ∧ (psiSeven W).natDegree=24 ∧ Nat.card (SevenRoot W)=24 ∧
      Finite (SevenTorsion W) ∧ Nat.card (SevenTorsion W)=49 ∧
      Nat.card {P : W.Point // addOrderOf P=7}=48 ∧ finrank (ZMod 7) (SevenTorsion W)=2 ∧
      Nonempty (Basis (Fin 2) (ZMod 7) (SevenTorsion W))) ∧
    (CharP BinaryClosure 2 ∧ binaryCurve.Δ≠0 ∧ Nat.card (SevenTorsion binaryCurve)=49 ∧
      finrank (ZMod 7) (SevenTorsion binaryCurve)=2) ∧
    (CharP CharThreeClosure 3 ∧ (short (-1:CharThreeClosure) 0).Δ≠0 ∧
      Nat.card (SevenTorsion (short (-1:CharThreeClosure) 0))=49 ∧
      finrank (ZMod 7) (SevenTorsion (short (-1:CharThreeClosure) 0))=2) ∧
    (CharP QuinaryClosure 5 ∧ (FiveTorsionRankAudit.charFiveCurve QuinaryClosure).Δ≠0 ∧
      Nat.card (SevenTorsion (FiveTorsionRankAudit.charFiveCurve QuinaryClosure))=49 ∧
      finrank (ZMod 7) (SevenTorsion (FiveTorsionRankAudit.charFiveCurve QuinaryClosure))=2) ∧
    (CharP SeptenaryClosure 7 ∧ (charSevenCurve SeptenaryClosure).Δ≠0 ∧
      Nat.card (SevenTorsion (charSevenCurve SeptenaryClosure))≤47 ∧
      Nat.card (SevenTorsion (charSevenCurve SeptenaryClosure))≠49 ∧
      finrank (ZMod 7) (SevenTorsion (charSevenCurve SeptenaryClosure))≠2) := by
  refine ⟨?_,?_,?_,binary_seven_positive,ternary_seven_positive,quinary_seven_positive,septenary_counter⟩
  · intro R _ W
    exact ⟨seven_formula W,seven_derivative W⟩
  · intro K _ _ W x y hN
    exact seven_point_iff W hN
  · intro K _ _ _ _ W hd
    exact ⟨seven_separable W hd,seven_degree W,seven_root_card W hd,seven_torsion_finite W hd,
      geometric_seven_card W hd,geometric_seven_exact_card W hd,geometric_seven_finrank W hd,
      ⟨geometricSevenBasis W hd⟩⟩

#print axioms actual_geometric_seven_certificate
end
end SevenTorsionRankAudit
