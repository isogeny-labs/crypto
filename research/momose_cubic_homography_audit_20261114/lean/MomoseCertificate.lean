module
public import FiniteSource
public import QuadraticControl

@[expose] public section
namespace MomoseCubicHomographyAudit
noncomputable section

/-- Exactly the explicit matrix printed after source Lemma1. -/
def momoseMatrix (K : Type*) [Field K] (r e a b c : K) : Mat2 K :=
  !![a*(a*r+c)-b^2, a^2*e-b*c; a, -b]

/-- Full source finite-field lemma, explicit matrix, and actual degree-two counter-control. -/
theorem momose_cubic_homography_certificate :
    (∀ (K L : Type) [Field K] [Field L] [Algebra K L] [Fintype K] [Fintype L]
      (q : ℕ), 2 ≤ q → Fintype.card K = q^2 → Fintype.card L = q^6 →
      ∀ z w : L, z^(q^3) = z → z^q ≠ z → outsideBase K L w →
      ∃ A : GL2 K, denominator K L A.val z ≠ 0 ∧ fractional K L A.val z = w ∧
        ∀ B : GL2 K, fractional K L B.val z = w →
          Matrix.ProjGenLinGroup.mk B = Matrix.ProjGenLinGroup.mk A ∧
          ∃ u : Kˣ, B * Matrix.GeneralLinearGroup.scalar (Fin 2) u = A) ∧
    (∀ (K L : Type) [Field K] [Field L] [Algebra K L] [FiniteDimensional K L],
      Module.finrank K L = 3 → ∀ z : L, outsideBase K L z →
      ∀ r e a b c : K, z^3 = algebraMap K L r*z + algebraMap K L e →
      (a ≠ 0 ∨ b ≠ 0) → (momoseMatrix K r e a b c).det ≠ 0 ∧
        denominator K L (momoseMatrix K r e a b c) z ≠ 0 ∧
        fractional K L (momoseMatrix K r e a b c) z =
          algebraMap K L a*z^2 + algebraMap K L b*z + algebraMap K L c) ∧
    (Fintype.card F9 = 9 ∧ Module.finrank (ZMod 3) F9 = 2 ∧
      ∃ a : F9, outsideBase (ZMod 3) F9 a ∧
        fractional (ZMod 3) F9 quadraticSwap.val a = a ∧
        Matrix.ProjGenLinGroup.mk quadraticSwap ≠ 1) ∧
    (¬ (∀ a : F9, outsideBase (ZMod 3) F9 a → ∀ A : GL2 (ZMod 3),
      fractional (ZMod 3) F9 A.val a = a → Matrix.ProjGenLinGroup.mk A = 1)) := by
  refine ⟨fun K L _ _ _ _ _ q hq hK hL z w hz3 hz1 hw =>
    momose_lemma_one K L q hq hK hL hz3 hz1 hw, ?_,
    quadratic_stabilizer_control, uniqueness_without_cubic_degree_false⟩
  intro K L _ _ _ _ hd z hz r e a b c hrel hab
  simpa only [momoseMatrix] using source_explicit_matrix K L hd hz r e a b c hrel hab

#print axioms momose_cubic_homography_certificate
end
end MomoseCubicHomographyAudit
