module
public import CubicUniqueness
public import Mathlib.FieldTheory.Finite.Basic

@[expose] public section
namespace MomoseCubicHomographyAudit
noncomputable section
variable (K L : Type*) [Field K] [Field L] [Algebra K L] [Fintype K] [Fintype L]

/-- The actual F_(q^6)/F_(q^2) extension has degree three. -/
theorem source_extension_degree (q : ℕ) (hq : 2 ≤ q)
    (hK : Fintype.card K = q^2) (hL : Fintype.card L = q^6) :
    Module.finrank K L = 3 := by
  have hc := Module.card_eq_pow_finrank (K := K) (V := L)
  rw [hK, hL, ← pow_mul] at hc
  have he : 6 = 2 * Module.finrank K L := Nat.pow_right_injective hq hc
  omega

omit [Fintype L] in
/-- An element of F_(q^3) outside F_q is outside the actual embedded F_(q^2). -/
theorem source_generator_outside (q : ℕ) (hK : Fintype.card K = q^2) {z : L}
    (h3 : z^(q^3) = z) (h1 : z^q ≠ z) : outsideBase K L z := by
  rintro ⟨a, rfl⟩
  have h2 : algebraMap K L a ^ (q^2) = algebraMap K L a := by
    rw [← hK, ← map_pow, FiniteField.pow_card]
  have hq : algebraMap K L a ^ q = algebraMap K L a := by
    calc
      _ = (algebraMap K L a ^ (q^2)) ^ q := by rw [h2]
      _ = algebraMap K L a ^ (q^3) := by rw [← pow_mul]; congr 1
      _ = _ := h3
  exact h1 hq

/-- Full source Lemma1: actual finite fields, actual GL2, actual scalar quotient. -/
theorem momose_lemma_one (q : ℕ) (hq : 2 ≤ q)
    (hK : Fintype.card K = q^2) (hL : Fintype.card L = q^6) {z w : L}
    (hz3 : z^(q^3) = z) (hz1 : z^q ≠ z) (hw : outsideBase K L w) :
    ∃ A : GL2 K, denominator K L A.val z ≠ 0 ∧ fractional K L A.val z = w ∧
      ∀ B : GL2 K, fractional K L B.val z = w →
        Matrix.ProjGenLinGroup.mk B = Matrix.ProjGenLinGroup.mk A ∧
        ∃ u : Kˣ, B * Matrix.GeneralLinearGroup.scalar (Fin 2) u = A := by
  have hd := source_extension_degree K L q hq hK hL
  have hz := source_generator_outside K L q hK hz3 hz1
  obtain ⟨A, hA⟩ := cubic_homography_exists K L hd hz hw
  refine ⟨A, denominator_ne_zero K L hz A,
    (sends_iff_fractional K L A.val z w (denominator_ne_zero K L hz A)).mp hA, ?_⟩
  intro B hB
  have hs := (sends_iff_fractional K L B.val z w (denominator_ne_zero K L hz B)).mpr hB
  exact ⟨cubic_homography_unique K L hd hz hw B A hs hA,
    cubic_homography_unique_up_to_scalar K L hd hz hw B A hs hA⟩

#print axioms source_extension_degree
#print axioms source_generator_outside
#print axioms momose_lemma_one
end
end MomoseCubicHomographyAudit
