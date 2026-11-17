import SecantTopForms
import DenseTopForms

noncomputable section
namespace RelationDegree
variable {K : Type*} [Field K]

/-- Exact homogeneous-quotient degree reduction between the actual displayed
generator presentations. No genericity, successful solver run, or desired
vanishing degree is assumed. The hypotheses expose the leading coefficients
and strict bounds on the remaining membership/graph terms. -/
theorem strict_reduction (d : ℕ) (hd : 2 ≤ d)
    (a b r s A B : K) (ha : a ≠ 0) (hb : b ≠ 0) (h2 : (2 : K) ≠ 0)
    (graphX graphZ : MvPolynomial (Fin 3) K)
    (hgX : graphX.totalDegree < d) (hgZ : graphZ.totalDegree < d)
    (memberX memberZ : MvPolynomial (Fin 2) K)
    (hmX : memberX.totalDegree < 2*d) (hmZ : memberZ.totalDegree < 2*d) :
    SecantIdeal.FirstVanishing
      (SecantTopForms.leadingIdeal d a b r s A graphX graphZ) (d+1) ∧
    SecantIdeal.FirstVanishing
      (DenseTopForms.leadingIdeal (2*d) A B r memberX memberZ) (2*d+1) ∧
    d+1 < 2*d+1 := by
  exact ⟨SecantTopForms.all_secant_equations_first_vanishing d hd
    a b r s A ha hb h2 graphX graphZ hgX hgZ,
    DenseTopForms.dense_equations_first_vanishing (2*d) (by omega)
      A B r memberX memberZ hmX hmZ,
    SecantGraphTop.strict_dense_reduction d hd⟩

end RelationDegree

#print axioms RelationDegree.strict_reduction
