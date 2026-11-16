import GeneralBasis
import ThreeCertificate

namespace ThreeTorsionRankAudit.General

/-- Actual geometric three-torsion, all nonsingular Weierstrass models and all characteristics except3. -/
theorem actual_geometric_three_torsion_certificate :
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (x y : K),
      W.Ψ₃.eval x = (3*x+W.a₂)*(denominator W x y)^2-
        W.a₁*numerator W x y*denominator W x y-(numerator W x y)^2-
        (12*x+W.b₂)*W.polynomial.evalEval x y) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K) (x y : K),
      ∀ hN : W.Nonsingular x y, (3 : ℕ) • WeierstrassCurve.Affine.Point.some x y hN=0 ↔ W.Ψ₃.IsRoot x) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [NeZero (3 : K)] [IsAlgClosed K],
      ∀ W : WeierstrassCurve.Affine K, W.Δ ≠ 0 →
        W.Ψ₃.natDegree=4 ∧ W.Ψ₃.Separable ∧ Nat.card (Root W)=4 ∧ Nat.card (Fibers W)=8 ∧
        Nat.card (ThreeTorsion W)=9 ∧ Nat.card {P : W.Point // addOrderOf P=3}=8 ∧
        Module.finrank (ZMod 3) (ThreeTorsion W)=2 ∧
        Nonempty (Module.Basis (Fin 2) (ZMod 3) (ThreeTorsion W))) ∧
    (∀ (K L : Type) [Field K] [Field L] [DecidableEq L] [NeZero (3 : K)] [IsAlgClosed L],
      ∀ (f : K →+* L) (W : WeierstrassCurve.Affine K), W.Δ ≠ 0 → Nat.card (ThreeTorsion (W.map f))=9) ∧
    CharP BinaryClosure 2 ∧ CharP CharThreeClosure 3 ∧
    (binaryCurve.Δ ≠ 0 ∧ Nat.card (ThreeTorsion binaryCurve)=9 ∧
      Module.finrank (ZMod 3) (ThreeTorsion binaryCurve)=2 ∧
      Nonempty (Module.Basis (Fin 2) (ZMod 3) (ThreeTorsion binaryCurve))) ∧
    ((short (-1 : CharThreeClosure) 0).Δ ≠ 0 ∧
      Nat.card (ThreeTorsion (short (-1 : CharThreeClosure) 0))=1 ∧
      Nat.card (ThreeTorsion (short (-1 : CharThreeClosure) 0))≠9) := by
  refine ⟨?_,?_,?_,?_,inferInstance,inferInstance,actual_binary_geometric_control,char_three_control⟩
  · intro K _ W x y
    exact psi_coordinate_identity W x y
  · intro K _ _ W x y hN
    exact three_point_iff W hN
  · intro K _ _ _ _ W hΔ
    exact ⟨W.natDegree_Ψ₃ (NeZero.ne _),psi_separable W hΔ,root_card_four W hΔ,
      fibers_card_eight W hΔ,geometric_three_torsion_card W hΔ,exact_order_three_card W hΔ,
      geometric_three_finrank W hΔ,
      geometric_three_basis_exists W hΔ⟩
  · intro K L _ _ _ _ _ f W hΔ
    exact scalar_extension_card f W hΔ

#print axioms actual_geometric_three_torsion_certificate

end ThreeTorsionRankAudit.General
