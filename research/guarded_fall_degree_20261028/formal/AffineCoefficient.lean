import Mathlib.Algebra.MvPolynomial.Equiv
import Mathlib.Data.Finsupp.Weight
import Lean.Elab.Tactic.Omega

namespace AffineCoefficient
open MvPolynomial

variable {K σ : Type*} [Field K] [Fintype σ]

/-- Every polynomial of total degree at most one is its explicit affine
coefficient expansion. -/
theorem affine_expansion (p : MvPolynomial σ K) (hd : p.totalDegree ≤ 1) :
    p = C (p.coeff 0) + ∑ i, C (p.coeff (Finsupp.single i 1)) * X i := by
  classical
  ext m
  by_cases hm : m = 0
  · subst m
    simp [coeff_C_mul, coeff_X]
  by_cases hdeg : m.sum (fun _ n => n) = 1
  · obtain ⟨i, rfl⟩ := (Finsupp.sum_eq_one_iff m).mp hdeg
    simp [coeff_C_mul, coeff_X, Finsupp.single_eq_single_iff, Ne.symm hm]
  · have hpos : m.sum (fun _ n => n) ≠ 0 := by
      exact fun h => hm ((Finsupp.degree_eq_zero_iff m).mp h)
    have hlarge : 1 < m.sum (fun _ n => n) := by omega
    have hp : p.coeff m = 0 := coeff_eq_zero_of_totalDegree_lt (hd.trans_lt hlarge)
    have hsingle (i : σ) : m ≠ Finsupp.single i 1 := by
      intro h
      apply hdeg
      simp [h]
    simp [hp, coeff_C_mul, coeff_X, Ne.symm hm, fun i => Ne.symm (hsingle i)]

/-- Taking the coefficient of the square of the first variable from a
degree-at-most-three polynomial leaves an affine polynomial in the rest. -/
theorem square_coefficient_degree (p : MvPolynomial (Fin 5) K)
    (hd : p.totalDegree ≤ 3) :
    ((finSuccEquiv K 4 p).coeff 2).totalDegree ≤ 1 := by
  by_cases hz : (finSuccEquiv K 4 p).coeff 2 = 0
  · simp [hz]
  · have hh := totalDegree_coeff_finSuccEquiv_add_le p 2 hz
    omega

theorem square_coefficient_affine (p : MvPolynomial (Fin 5) K)
    (hd : p.totalDegree ≤ 3) :
    let q := (finSuccEquiv K 4 p).coeff 2
    q = C (q.coeff 0) + ∑ i, C (q.coeff (Finsupp.single i 1)) * X i :=
  affine_expansion _ (square_coefficient_degree p hd)

end AffineCoefficient

#print axioms AffineCoefficient.affine_expansion
#print axioms AffineCoefficient.square_coefficient_degree
#print axioms AffineCoefficient.square_coefficient_affine
