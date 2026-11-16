module
public import Mathlib.FieldTheory.IntermediateField.Adjoin.Basic
public import Mathlib.LinearAlgebra.Matrix.GeneralLinearGroup.Projective
public import Mathlib.Tactic

@[expose] public section
namespace MomoseCubicHomographyAudit
noncomputable section
variable (K L : Type*) [Field K] [Field L] [Algebra K L] [FiniteDimensional K L]

/-- Outside the actual image of the base field. -/
def outsideBase (z : L) : Prop := z ∉ Set.range (algebraMap K L)

omit [FiniteDimensional K L] in
theorem cubic_generates (hd : Module.finrank K L = 3) {z : L} (hz : outsideBase K L z) :
    IntermediateField.adjoin K {z} = ⊤ := by
  let := IntermediateField.isSimpleOrder_of_finrank_prime K L (by rw [hd]; decide)
  rcases eq_bot_or_eq_top (IntermediateField.adjoin K {z}) with h | h
  · have hm : z ∈ IntermediateField.adjoin K {z} := IntermediateField.subset_adjoin _ _ (by simp)
    rw [h] at hm
    exact (hz hm).elim
  · exact h

def cubicPowerBasis (hd : Module.finrank K L = 3) {z : L} (hz : outsideBase K L z) :
    PowerBasis K L :=
  PowerBasis.ofAdjoinSimpleEqTop (IsIntegral.of_finite K z) (cubic_generates K L hd hz)

@[simp] theorem cubicPowerBasis_gen (hd : Module.finrank K L = 3) {z : L}
    (hz : outsideBase K L z) : (cubicPowerBasis K L hd hz).gen = z := by
  simp [cubicPowerBasis]

theorem cubicPowerBasis_dim (hd : Module.finrank K L = 3) {z : L} (hz : outsideBase K L z) :
    (cubicPowerBasis K L hd hz).dim = 3 := by
  rw [← PowerBasis.finrank, hd]

def cubicBasis (hd : Module.finrank K L = 3) {z : L} (hz : outsideBase K L z) :
    Module.Basis (Fin 3) K L :=
  (cubicPowerBasis K L hd hz).basis.reindex (finCongr (cubicPowerBasis_dim K L hd hz))

@[simp] theorem cubicBasis_apply (hd : Module.finrank K L = 3) {z : L}
    (hz : outsideBase K L z) (i : Fin 3) : cubicBasis K L hd hz i = z ^ i.val := by
  simp [cubicBasis]

theorem cubic_coordinates (hd : Module.finrank K L = 3) {z : L}
    (hz : outsideBase K L z) (w : L) :
    ∃ a b c : K, w = algebraMap K L a * z ^ 2 + algebraMap K L b * z + algebraMap K L c := by
  have h := ((cubicBasis K L hd hz).sum_repr w).symm
  refine ⟨(cubicBasis K L hd hz).repr w 2, (cubicBasis K L hd hz).repr w 1,
    (cubicBasis K L hd hz).repr w 0, ?_⟩
  simpa [Fin.sum_univ_succ, Algebra.smul_def, add_comm, add_left_comm, add_assoc] using h

theorem quadratic_relation_zero (hd : Module.finrank K L = 3) {z : L}
    (hz : outsideBase K L z) (a b c : K)
    (h : algebraMap K L a * z ^ 2 + algebraMap K L b * z + algebraMap K L c = 0) :
    a = 0 ∧ b = 0 ∧ c = 0 := by
  have hsum : ∑ i : Fin 3, (![c, b, a] i) • cubicBasis K L hd hz i = 0 := by
    simpa [Fin.sum_univ_succ, Algebra.smul_def, add_comm, add_left_comm, add_assoc] using h
  have hh := Fintype.linearIndependent_iff.mp (cubicBasis K L hd hz).linearIndependent
    ![c, b, a] hsum
  exact ⟨hh 2, hh 1, hh 0⟩

#print axioms cubic_generates
#print axioms cubic_coordinates
#print axioms quadratic_relation_zero
end
end MomoseCubicHomographyAudit
