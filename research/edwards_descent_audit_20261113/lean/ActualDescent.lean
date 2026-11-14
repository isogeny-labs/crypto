import EdwardsInput
import Mathlib.RingTheory.MvPolynomial.Homogeneous
import Mathlib.RingTheory.Regular.RegularSequence

namespace EdwardsDescentAudit
open MvPolynomial
noncomputable section
abbrev Poly := MvPolynomial (Fin 2) F
abbrev ExtPoly := MvPolynomial (Fin 2) K

def f0 : Poly := 2*(X 0^2+X 1^2)+2*X 1+1
def f1 : Poly := 2*(X 0^2+X 1^2)+3
def top : Poly := 2*(X 0^2+X 1^2)

/-- Symmetrization of the exact printed Edwards polynomial, with the
actual curve parameters and actual target y-coordinate. -/
def symmetric : ExtPoly :=
  C (targetY^2-1)*X 1^2+C (a/d-targetY^2)*X 0^2+
    C (2*targetY^2+2*(1-a/d)*targetY-2*(a/d))*X 1+C ((a/d)*(targetY^2-1))

theorem source_polynomial_identity (x y : K) :
    sourceF3 a d x y targetY = MvPolynomial.eval ![x+y,x*y] symmetric := by
  rw [symmetrization a d targetY x y actual_parameters.2.1]
  simp [symmetric]

theorem coefficient_decomposition :
    symmetric = MvPolynomial.map (algebraMap F K) f0+
      C beta*MvPolynomial.map (algebraMap F K) f1 := by
  have hA : targetY^2-1=2+2*beta := by decide +kernel
  have hB : a/d-targetY^2=2+2*beta := by decide +kernel
  have hC : 2*targetY^2+2*(1-a/d)*targetY-2*(a/d)=2 := by decide +kernel
  have hD : (a/d)*(targetY^2-1)=1+3*beta := by decide +kernel
  unfold symmetric
  rw [hD,hA,hB,hC]
  simp [f0,f1]
  norm_num only [map_ofNat]
  ring

/-- Actual coefficient coordinates in the basis (1,beta) of F25/F5. -/
def descendRe (g : ExtPoly) : Poly :=
  .ofCoeff (Finsupp.mapRange (fun z : K => z.re) rfl (AddMonoidAlgebra.coeff g))
def descendIm (g : ExtPoly) : Poly :=
  .ofCoeff (Finsupp.mapRange (fun z : K => z.im) rfl (AddMonoidAlgebra.coeff g))

theorem coeff_descendRe (m : Fin 2 →₀ ℕ) (g : ExtPoly) :
    (descendRe g).coeff m = (g.coeff m).re := rfl
theorem coeff_descendIm (m : Fin 2 →₀ ℕ) (g : ExtPoly) :
    (descendIm g).coeff m = (g.coeff m).im := rfl

theorem actual_descent : descendRe symmetric=f0 ∧ descendIm symmetric=f1 := by
  constructor
  · ext m
    rw [coeff_descendRe, coefficient_decomposition]
    simp [beta, MvPolynomial.coeff_map]
  · ext m
    rw [coeff_descendIm, coefficient_decomposition]
    simp [beta, MvPolynomial.coeff_map]

theorem top_nonzero : top ≠ 0 := by
  intro h
  have he := congrArg (MvPolynomial.eval ![(1:F),0]) h
  norm_num [top] at he
  exact (by decide : (2:F)≠0) he

theorem top_homogeneous : top.IsHomogeneous 2 := by
  exact ((isHomogeneous_X_pow 0 2).add (isHomogeneous_X_pow 1 2)).C_mul 2

theorem top_degree : top.totalDegree=2 := top_homogeneous.totalDegree top_nonzero

theorem linear_degree : (2*X 1 : Poly).totalDegree ≤ 1 := by
  have hh : (2*X 1 : Poly).IsHomogeneous 1 :=
    (isHomogeneous_X F (1:Fin 2)).C_mul 2
  exact hh.totalDegree_le

theorem equation_degrees : f0.totalDegree=2 ∧ f1.totalDegree=2 := by
  have hl := linear_degree
  have ht : (top+2*X 1).totalDegree=2 := by
    rw [totalDegree_add_eq_left_of_totalDegree_lt (by rw [top_degree]; omega)]
    exact top_degree
  constructor
  · change (top+2*X 1+1).totalDegree=2
    rw [totalDegree_add_eq_left_of_totalDegree_lt (by rw [ht]; simp)]
    exact ht
  · change (top+C (3:F)).totalDegree=2
    rw [totalDegree_add_eq_left_of_totalDegree_lt (by rw [top_degree]; simp)]
    exact top_degree

theorem nonzero_equations : f0 ≠ 0 ∧ f1 ≠ 0 := by
  constructor
  · intro h
    have hd := equation_degrees.1
    rw [h,totalDegree_zero] at hd
    omega
  · intro h
    have hd := equation_degrees.2
    rw [h,totalDegree_zero] at hd
    omega

theorem actual_top_parts :
    homogeneousComponent f0.totalDegree f0=top ∧
      homogeneousComponent f1.totalDegree f1=top := by
  rw [equation_degrees.1,equation_degrees.2]
  have hl : homogeneousComponent 2 (2*X 1 : Poly)=0 :=
    homogeneousComponent_eq_zero 2 _ (by have := linear_degree; omega)
  constructor
  · change homogeneousComponent 2 (top+2*X 1+1)=top
    rw [map_add,map_add,homogeneousComponent_eq_self top_homogeneous,hl,
      homogeneousComponent_eq_zero 2 _ (by simp : (1:Poly).totalDegree<2)]
    simp
  · change homogeneousComponent 2 (top+C (3:F))=top
    rw [map_add,homogeneousComponent_eq_self top_homogeneous,
      homogeneousComponent_eq_zero 2 _ (by simp : (C (3:F):Poly).totalDegree<2)]
    simp

theorem duplicate_not_regular {A : Type*} [CommRing A] (h : A) :
    ¬ RingTheory.Sequence.IsRegular A [h,h] := by
  intro hr
  have ht := (RingTheory.Sequence.isRegular_cons_iff' A h [h]).mp hr |>.2
  have hz : Ideal.Quotient.mk (Ideal.span ({h} : Set A)) h=0 :=
    Ideal.Quotient.eq_zero_iff_mem.mpr (Ideal.subset_span (by simp))
  simp only [List.map_cons, List.map_nil, hz] at ht
  have := ht.nontrivial
  have hn := (RingTheory.Sequence.isRegular_cons_iff _ _ []).mp ht |>.1
  exact IsSMulRegular.not_zero hn

theorem actual_top_parts_not_regular :
    ¬ RingTheory.Sequence.IsRegular Poly
      [homogeneousComponent (descendRe symmetric).totalDegree (descendRe symmetric),
       homogeneousComponent (descendIm symmetric).totalDegree (descendIm symmetric)] := by
  rw [actual_descent.1, actual_descent.2, actual_top_parts.1, actual_top_parts.2]
  exact duplicate_not_regular top

end
end EdwardsDescentAudit
