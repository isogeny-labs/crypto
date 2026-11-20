import QuotientBasis
import Mathlib.RingTheory.MvPolynomial.Homogeneous
import Mathlib.LinearAlgebra.Dimension.Constructions

namespace JVGradedHilbertAudit
noncomputable section
open MvPolynomial JVHilbertAudit

def basisDegree : Fin 8 → ℕ := ![0, 1, 1, 1, 2, 2, 2, 3]
abbrev degreeIndex (d : ℕ) := {i : Fin 8 // basisDegree i = d}
def gradedSpan (d : ℕ) : Submodule F A :=
  Submodule.span F (Set.range (fun i : degreeIndex d => basisVector i.val))

theorem basisVector_mem_gradedSpan (i : Fin 8) (d : ℕ) (hi : basisDegree i = d) :
    basisVector i ∈ gradedSpan d := Submodule.subset_span ⟨⟨i, hi⟩, rfl⟩

theorem basisDegree_le_three (i : Fin 8) : basisDegree i ≤ 3 := by
  fin_cases i <;> decide

theorem matrix_respects_degree :
    ∀ s : Fin 3, ∀ i j : Fin 8, multiplicationCoefficients s i j ≠ 0 →
      basisDegree j = basisDegree i + 1 := by
  decide +kernel

theorem combination_mem_next (s : Fin 3) (i : Fin 8) :
    combination (multiplicationCoefficients s i) ∈ gradedSpan (basisDegree i + 1) := by
  rw [combination_eq_smul]
  apply (gradedSpan _).sum_mem
  intro j _
  by_cases hj : multiplicationCoefficients s i j = 0
  · simp only [hj, zero_smul, Submodule.zero_mem]
  · exact (gradedSpan _).smul_mem _
      (basisVector_mem_gradedSpan j _ (matrix_respects_degree s i j hj))

theorem gradedSpan_closed_coordinate (d : ℕ) (s : Fin 3) {z : A}
    (hz : z ∈ gradedSpan d) : quotientMap (X s) * z ∈ gradedSpan (d + 1) := by
  induction hz using Submodule.span_induction with
  | mem z hz =>
    obtain ⟨i, rfl⟩ := hz
    rw [actual_coordinate_multiplication]
    simpa only [i.property] using combination_mem_next s i.val
  | zero => simp
  | add z w _ _ hz hw => simpa only [mul_add] using (gradedSpan _).add_mem hz hw
  | smul c z _ hz => simpa only [mul_smul_comm] using (gradedSpan _).smul_mem c hz

theorem gradedSpan_pow_coordinate (d n : ℕ) (s : Fin 3) {z : A}
    (hz : z ∈ gradedSpan d) : quotientMap (X s) ^ n * z ∈ gradedSpan (d + n) := by
  induction n with
  | zero => simpa using hz
  | succ n ih =>
    simpa [pow_succ, mul_assoc, mul_left_comm, Nat.add_assoc] using
      gradedSpan_closed_coordinate (d + n) s ih

theorem one_mem_gradedSpan : (1 : A) ∈ gradedSpan 0 := by
  simpa only [basisVector_coordinates, Matrix.cons_val_zero] using
    basisVector_mem_gradedSpan 0 0 rfl

theorem monomial_image_mem (e : Fin 3 →₀ ℕ) (c : F) :
    quotientMap (monomial e c) ∈ gradedSpan e.degree := by
  induction e using Finsupp.induction with
  | zero =>
    change algebraMap F A c ∈ gradedSpan 0
    rw [Algebra.algebraMap_eq_smul_one]
    exact (gradedSpan 0).smul_mem c one_mem_gradedSpan
  | single_add s n e _ _ ih =>
    rw [map_add, Finsupp.degree_single, monomial_single_add, map_mul, map_pow]
    simpa only [Nat.add_comm] using gradedSpan_pow_coordinate e.degree n s ih

theorem homogeneous_image_mem {p : Poly} {d : ℕ} (hp : p.IsHomogeneous d) :
    quotientMap p ∈ gradedSpan d := by
  induction hp using IsWeightedHomogeneous.induction_on with
  | zero => simp
  | add p q _ _ hp hq => simpa only [map_add] using (gradedSpan _).add_mem hp hq
  | monomial e c he =>
    have hd : e.degree = d := by
      rw [Finsupp.degree_eq_weight_one, ← Pi.one_def]
      exact he
    simpa only [hd] using monomial_image_mem e c

theorem exponent_degree (i j k : ℕ) : (exponent i j k).degree = i + j + k := by
  simp only [exponent, map_add, Finsupp.degree_single]

theorem basisExponent_degree (i : Fin 8) : (basisExponent i).degree = basisDegree i := by
  fin_cases i <;> simp [basisExponent, basisDegree, exponent_degree]

theorem basisPoly_homogeneous (i : Fin 8) : (basisPoly i).IsHomogeneous (basisDegree i) :=
  isHomogeneous_monomial 1 (basisExponent_degree i)

theorem gradedSpan_dimension (d : ℕ) :
    Module.finrank F (gradedSpan d) = Fintype.card (degreeIndex d) := by
  apply finrank_span_eq_card
  exact basisVector_linearIndependent.comp _ Subtype.val_injective

def expectedHilbert (d : ℕ) : ℕ :=
  if d = 0 then 1 else if d = 1 then 3 else if d = 2 then 3 else if d = 3 then 1 else 0

theorem degreeIndex_card (d : ℕ) : Fintype.card (degreeIndex d) = expectedHilbert d := by
  by_cases hd : d < 4
  · interval_cases d <;> decide +kernel
  · have he : IsEmpty (degreeIndex d) := ⟨fun i => by
      have h := basisDegree_le_three i.val
      rw [i.property] at h
      omega⟩
    have hc : Fintype.card (degreeIndex d) = 0 := Fintype.card_eq_zero_iff.mpr he
    rw [hc]
    simp [expectedHilbert, show d ≠ 0 by omega, show d ≠ 1 by omega,
      show d ≠ 2 by omega, show d ≠ 3 by omega]

#print axioms matrix_respects_degree
#print axioms homogeneous_image_mem
#print axioms gradedSpan_dimension
end
end JVGradedHilbertAudit
