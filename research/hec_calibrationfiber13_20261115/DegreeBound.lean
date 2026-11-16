import Mathlib
open Polynomial
namespace CalibrationFiber
/-- Degree bound after exact removal of the fixed-anchor factor.
The geometric restriction/divisibility statements are explicit hypotheses. -/
theorem saturated_norm_degree {K : Type*} [Field K]
    (A B f S Q : K[X])
    (hA : A.natDegree ≤ 15) (hB : B.natDegree ≤ 11)
    (hf : f.natDegree ≤ 7) (hS : S ≠ 0) (hSd : S.natDegree = 12)
    (hn : A*A-f*(B*B) ≠ 0) (hsat : A*A-f*(B*B) = S*Q) :
    Q.natDegree ≤ 18 := by
  have haa : (A*A).natDegree ≤ 30 := by
    have h := natDegree_mul_le (p := A) (q := A)
    omega
  have hbb : (B*B).natDegree ≤ 22 := by
    have h := natDegree_mul_le (p := B) (q := B)
    omega
  have hfbb : (f*(B*B)).natDegree ≤ 29 := by
    have h := natDegree_mul_le (p := f) (q := B*B)
    omega
  have hraw : (A*A-f*(B*B)).natDegree ≤ 30 := by
    have h := natDegree_sub_le (A*A) (f*(B*B))
    exact h.trans (max_le haa (by omega))
  have hQ : Q ≠ 0 := by
    intro hzero
    apply hn
    rw [hsat, hzero, mul_zero]
  have hprod := natDegree_mul hS hQ
  rw [hsat] at hraw
  omega
end CalibrationFiber
#print axioms CalibrationFiber.saturated_norm_degree
