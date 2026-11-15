import TangentControls
import TangentNecessity

namespace ThreeTangentPairingAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General BKFrobeniusBridgeAudit Module
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

/-- Transparent actual point-map alias keeps named assignments outside the written header. -/
abbrev actualMappedPoint (F A B : Type*) [Field F] [Field A] [Field B]
    [Algebra F A] [Algebra F B] [DecidableEq A] [DecidableEq B]
    (W : WeierstrassCurve.Affine F) (f : A →ₐ[F] B) := Point.map (W':=W) f

theorem actual_three_tangent_pairing_certificate :
    (∀ (K : Type) [Field K] [DecidableEq K] [IsAlgClosed K] [NeZero (3:K)]
      (W : WeierstrassCurve.Affine K), W.Δ≠0 →
      (∀ P Q : ThreeTorsion W, (tangentPair W P.val Q.val)^3=1) ∧
      (∀ P Q R : ThreeTorsion W,
        tangentPair W (P+Q).val R.val=tangentPair W P.val R.val*tangentPair W Q.val R.val ∧
        tangentPair W P.val (Q+R).val=tangentPair W P.val Q.val*tangentPair W P.val R.val) ∧
      (∀ P : ThreeTorsion W, tangentPair W P.val P.val=1) ∧
      (∀ P Q : ThreeTorsion W, tangentPair W P.val Q.val=(tangentPair W Q.val P.val)⁻¹) ∧
      (∀ P : ThreeTorsion W, (∀ Q : ThreeTorsion W, tangentPair W P.val Q.val=1) → P=0) ∧
      (∀ Q : ThreeTorsion W, (∀ P : ThreeTorsion W, tangentPair W P.val Q.val=1) → Q=0) ∧
      (∀ P : ThreeTorsion W, P≠0 → ∃ Q : Submodule.span (ZMod 3) {P}, Q≠0 ∧
        ∀ R : Submodule.span (ZMod 3) {P}, tangentPair W Q.val.val R.val.val=1)) ∧
    (∀ (F A B : Type) [Field F] [Field A] [Field B] [Algebra F A] [Algebra F B]
      [DecidableEq A] [DecidableEq B] (W : WeierstrassCurve.Affine F) (f : A →ₐ[F] B)
      (P Q : (W.baseChange A).Point),
      tangentPair (W.baseChange B) (actualMappedPoint F A B W f P) (actualMappedPoint F A B W f Q)=
        f (tangentPair (W.baseChange A) P Q)) ∧
    (∀ (F L : Type) [Field F] [Fintype F] [Field L] [Algebra F L]
      [DecidableEq F] [DecidableEq L] [IsAlgClosed L] [NeZero (3:L)]
      (W : WeierstrassCurve.Affine F), W.Δ≠0 →
      (∀ b : Basis (Fin 2) (ZMod 3) (ThreeTorsion (W.baseChange L)),
        (frobeniusMatrix W b).det=(Fintype.card F : ZMod 3)) ∧
      (∀ (K : Type) [Field K] [Fintype K] [Algebra F K] [Algebra K L]
        [IsScalarTower F K L] [DecidableEq K] (k : ℕ), Fintype.card K=Fintype.card F^k →
        (∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 →
          P∈Set.range (pointInclusion F K L W)) → 3 ∣ Fintype.card F^k-1)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [CharP K 3],
      (ThreeTorsionCharacterAudit.curve (1:K) 1).Δ≠0 ∧
      ∃ P : ThreeTorsion (ThreeTorsionCharacterAudit.curve (1:K) 1), P≠0 ∧
        ∀ Q : ThreeTorsion (ThreeTorsionCharacterAudit.curve (1:K) 1),
          tangentPair (ThreeTorsionCharacterAudit.curve (1:K) 1) P.val Q.val=1) ∧
    (binaryCurve.Δ≠0 ∧ ∃ P Q : ThreeTorsion binaryCurve,
      (tangentPair binaryCurve P.val Q.val)^3=1 ∧ tangentPair binaryCurve P.val Q.val≠1) := by
  refine ⟨?_,?_,?_,?_,binary_pairing_positive⟩
  · intro K _ _ _ _ W hd
    refine ⟨tangentPair_cube W hd,?_,?_,?_,?_,?_,actual_cyclic_line_degenerate W hd⟩
    · intro P Q R
      exact ⟨tangentPair_add_left W hd P Q R,tangentPair_add_right W hd P Q R⟩
    · intro P
      exact tangentPair_self W P.val
    · intro P Q
      exact tangentPair_swap W P.val Q.val
    · exact tangentPair_left_nondegenerate W hd
    · exact tangentPair_right_nondegenerate W hd
  · intro F A B _ _ _ _ _ _ _ W f P Q
    exact tangentPair_algmap f W P Q
  · intro F L _ _ _ _ _ _ _ _ W hd
    refine ⟨actual_general_three_determinant W hd,?_⟩
    intro K _ _ _ _ _ _ k hk h
    exact full_three_torsion_forces_cyclotomic W hd k hk h
  · intro K _ _ _
    exact characteristic_three_degenerate

#print axioms actual_three_tangent_pairing_certificate
end
end ThreeTangentPairingAudit
