import GradedCertificate
import PowerBasisLength

namespace JVLocalMultiplicityAudit
noncomputable section
open MvPolynomial JVHilbertAudit JVGradedHilbertAudit

theorem actual_basis_apply (i : Fin 8) : actualBasis i = basisVector i := by
  unfold actualBasis
  exact Module.Basis.mk_apply _ _ i

theorem basis_degree_order : ∀ i j : Fin 8, basisDegree i < basisDegree j → i.val < j.val := by
  decide +kernel

theorem coordinate_strict_triangle (s : Fin 3) (i j : Fin 8)
    (h : multiplicationCoefficients s i j ≠ 0) : i.val < j.val := by
  apply basis_degree_order i j
  rw [matrix_respects_degree s i j h]
  exact Nat.lt_succ_self _

def tailSpan (k : ℕ) : Submodule F A :=
  Submodule.span F (actualBasis '' {i : Fin 8 | k ≤ i.val})

theorem basis_mem_tail_iff (k : ℕ) (i : Fin 8) : actualBasis i ∈ tailSpan k ↔ k ≤ i.val :=
  actualBasis.self_mem_span_image

theorem tail_coordinate_closed (k : ℕ) (s : Fin 3) {z : A} (hz : z ∈ tailSpan k) :
    quotientMap (X s) * z ∈ tailSpan k := by
  induction hz using Submodule.span_induction with
  | mem z hz =>
    obtain ⟨i, hi, rfl⟩ := hz
    change k ≤ i.val at hi
    rw [actual_basis_apply, actual_coordinate_multiplication, combination_eq_smul]
    apply (tailSpan k).sum_mem
    intro j _
    by_cases hc : multiplicationCoefficients s i j = 0
    · simpa only [hc, zero_smul] using (tailSpan k).zero_mem
    · apply (tailSpan k).smul_mem
      rw [← actual_basis_apply j]
      apply (basis_mem_tail_iff k j).mpr
      exact hi.trans (Nat.le_of_lt (coordinate_strict_triangle s i j hc))
  | zero => simp
  | add z w _ _ hz hw => simpa only [mul_add] using (tailSpan k).add_mem hz hw
  | smul c z _ hz => simpa only [mul_smul_comm] using (tailSpan k).smul_mem c hz

theorem tail_polynomial_closed (k : ℕ) (p : Poly) {z : A} (hz : z ∈ tailSpan k) :
    quotientMap p * z ∈ tailSpan k := by
  induction p using MvPolynomial.induction_on with
  | C c =>
    change algebraMap F A c * z ∈ tailSpan k
    rw [← Algebra.smul_def c z]
    exact (tailSpan k).smul_mem c hz
  | add p q hp hq => simpa only [map_add, add_mul] using (tailSpan k).add_mem hp hq
  | mul_X p s hp =>
    have he : quotientMap (p * X s) * z = quotientMap (X s) * (quotientMap p * z) := by
      rw [map_mul]
      ring
    rw [he]
    exact tail_coordinate_closed k s hp

theorem tail_mul_closed (k : ℕ) (a : A) {z : A} (hz : z ∈ tailSpan k) : a * z ∈ tailSpan k := by
  obtain ⟨p, rfl⟩ := Ideal.Quotient.mkₐ_surjective F JVMacaulayAudit.sourceIdeal a
  exact tail_polynomial_closed k p hz

/-- These are actual ideals of the actual quotient, not assumed scalar subspaces. -/
def tailIdeal (k : ℕ) : Ideal A where
  carrier := tailSpan k
  zero_mem' := (tailSpan k).zero_mem
  add_mem' := (tailSpan k).add_mem
  smul_mem' a z hz := tail_mul_closed k a (z := z) hz

theorem tail_antitone {k l : ℕ} (h : k ≤ l) : tailIdeal l ≤ tailIdeal k := by
  change tailSpan l ≤ tailSpan k
  apply Submodule.span_mono
  rintro z ⟨i, hi, rfl⟩
  exact ⟨i, h.trans hi, rfl⟩

theorem tail_strict (k : ℕ) (hk : k < 8) : tailIdeal (k + 1) < tailIdeal k := by
  refine lt_iff_le_not_ge.mpr ⟨tail_antitone (Nat.le_succ k), ?_⟩
  intro h
  have hm : actualBasis ⟨k, hk⟩ ∈ tailIdeal k := (basis_mem_tail_iff k _).mpr le_rfl
  have hh := (basis_mem_tail_iff (k + 1) _).mp (h hm)
  exact Nat.not_succ_le_self k hh

def idealChain : LTSeries (Submodule A A) where
  length := 8
  toFun i := tailIdeal (8 - i.val)
  step i := by
    change tailIdeal (8 - i.val) < tailIdeal (8 - (i.val + 1))
    have hk : 8 - (i.val + 1) < 8 := by have := i.isLt; omega
    have he : 8 - i.val = (8 - (i.val + 1)) + 1 := by have := i.isLt; omega
    rw [he]
    exact tail_strict _ hk

theorem ring_length_lower : (8 : ℕ∞) ≤ Module.length A A := by
  have h : (8 : WithBot ℕ∞) ≤ (Module.length A A : WithBot ℕ∞) := by
    rw [Module.coe_length]
    exact Order.LTSeries.length_le_krullDim idealChain
  exact WithBot.coe_le_coe.mp h

theorem actual_ring_length : Module.length A A = 8 := by
  have hu : Module.length A A ≤ Module.length F A :=
    LocalAlgebraLengthAudit.scalar_length_upper (K := F) (B := A)
  exact le_antisymm (hu.trans_eq actual_scalar_module_length) ring_length_lower

#print axioms tail_mul_closed
#print axioms tail_strict
#print axioms actual_ring_length
end
end JVLocalMultiplicityAudit
