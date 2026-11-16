import DescentAlgebra

namespace EdwardsDescentAudit
noncomputable section
open MvPolynomial

/-- Closed counterexample to the universal n=2 reading of FGHR2014
Hypothesis3. Both equations are nonzero quadratics, the affine coordinate
algebra is genuinely finite-dimensional/Artinian, and the curve/target are
actual points in the specified twisted-Edwards representation. This does
not assert anything about a generic-input or n>=3 interpretation. -/
theorem actual_edwards_descent_counterexample :
    Fintype.card K=25 ∧ a≠0 ∧ d≠0 ∧ a≠d ∧
      (∀ r : F, algebraMap F K r≠a) ∧
      equation a d targetX targetY ∧
      (∀ r : F, algebraMap F K r≠targetY) ∧ curve.Δ≠0 ∧
      R≠0 ∧ P+Q=R ∧
      (∀ x y : K, sourceF3 a d x y targetY=MvPolynomial.eval ![x+y,x*y] symmetric) ∧
      descendRe symmetric=f0 ∧ descendIm symmetric=f1 ∧
      f0≠0 ∧ f1≠0 ∧ f0.totalDegree=2 ∧ f1.totalDegree=2 ∧
      Module.Finite F A ∧ Module.finrank F A=2 ∧ IsArtinianRing A ∧
      ¬ RingTheory.Sequence.IsRegular Poly
        [homogeneousComponent (descendRe symmetric).totalDegree (descendRe symmetric),
         homogeneousComponent (descendIm symmetric).totalDegree (descendIm symmetric)] := by
  exact ⟨extension_cardinality,actual_parameters.1,actual_parameters.2.1,
    actual_parameters.2.2,curve_not_base,target_on_edwards,target_not_base,
    curve_nonsingular,target_nonzero,actual_sum,source_polynomial_identity,
    actual_descent.1,actual_descent.2,nonzero_equations.1,nonzero_equations.2,
    equation_degrees.1,equation_degrees.2,quotient_finite,quotient_dimension_two,
    quotient_artinian,actual_top_parts_not_regular⟩

#print axioms actual_edwards_descent_counterexample

end
end EdwardsDescentAudit
