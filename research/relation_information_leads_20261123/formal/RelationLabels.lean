import Mathlib

/- Source-only proof attempt. No Lean executable is installed here.
   This file has NOT been elaborated or kernel checked.
   Intended toolchain: crypto-lean ff94d4295f592c679074034cf926695c76de4ddc,
   leanprover/lean4:v4.35.0. No sorry/admit/custom axiom is used.
   This proves only the formal-label cancellation building block if it builds;
   subgroup semantics, rank duality and extraction are separate obligations. -/
namespace RelationInformation
open scoped BigOperators

theorem labelled_column_cancellation
    {I J K : Type*} [Fintype I] [Fintype J] [CommRing K]
    (R : I → J → K) (u : I → K) (b : J → K)
    (hu : ∀ j, ∑ i, u i * R i j = 0) :
    (∑ i, u i * (∑ j, R i j * b j)) = 0 := by
  simp_rw [Finset.mul_sum]
  rw [Finset.sum_comm]
  apply Finset.sum_eq_zero
  intro j _
  calc
    (∑ i, u i * (R i j * b j)) = (∑ i, u i * R i j) * b j := by
      simp only [Finset.sum_mul, mul_assoc]
    _ = 0 := by rw [hu j, zero_mul]

theorem affine_line_substitution
    {K : Type*} [CommRing K] (x m c A B : K) :
    (m*x+c)^2 - (x^3+A*x+B) =
      -(x^3-m^2*x^2+(A-2*m*c)*x+B-c^2) := by
  ring

#print axioms labelled_column_cancellation
#print axioms affine_line_substitution
end RelationInformation
