import Common22WeightedProjHead
set_option autoImplicit false
noncomputable section
namespace Common22WeightedProjCover
open MvPolynomial Common22WeightedProjHead
variable {K : Type*} [Field K]
def tail (a3 a2 a1 a0 : K) : Poly K :=
  X 0 ^ 5 + C a3 * (X 0 ^ 3 * X 2 ^ 2) +
    C a2 * (X 0 ^ 2 * X 2 ^ 3) + C a1 * (X 0 * X 2 ^ 4) +
    C a0 * X 2 ^ 5
theorem equation_factor (a3 a2 a1 a0 : K) :
    equation a3 a2 a1 a0 = X 1 ^ 2 - X 2 * tail a3 a2 a1 a0 := by
  simp [equation, tail]
  ring
theorem y_mem_of_z_mem (a3 a2 a1 a0 : K)
    (p : Ideal (Poly K)) (hp : p.IsPrime)
    (heq : equation a3 a2 a1 a0 ∈ p)
    (hz : X 2 ∈ p) : X 1 ∈ p := by
  have htail : X (2 : Fin 3) * tail a3 a2 a1 a0 ∈ p :=
    p.mul_mem_right _ hz
  have hy2 : X (1 : Fin 3) ^ 2 ∈ p := by
    rw [equation_factor] at heq
    have hsum := p.add_mem heq htail
    convert hsum using 1; ring
  exact hp.mem_of_pow_mem 2 hy2
theorem irrelevant_le_of_x_z_mem (a3 a2 a1 a0 : K)
    (p : Ideal (Poly K)) (hp : p.IsPrime)
    (heq : equation a3 a2 a1 a0 ∈ p)
    (hx : X 0 ∈ p) (hz : X 2 ∈ p) :
    Ideal.span ({X 0, X 1, X 2} : Set (Poly K)) ≤ p := by
  apply Ideal.span_le.mpr
  intro x h
  simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at h
  rcases h with rfl | rfl | rfl
  · exact hx
  · exact y_mem_of_z_mem a3 a2 a1 a0 p hp heq hz
  · exact hz
#print axioms equation_factor
#print axioms y_mem_of_z_mem
#print axioms irrelevant_le_of_x_z_mem
end Common22WeightedProjCover
