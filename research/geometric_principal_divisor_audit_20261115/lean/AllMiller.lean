module
public import MillerStep
public import Mathlib.Data.Finsupp.SMul

@[expose] public section
namespace MillerRecursionAudit
noncomputable section
open WeierstrassCurve.Affine MillerFinitePointAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- Source Miller divisor, with integer subtraction also giving a consistent n=0 convention. -/
def millerDivisor (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) : W.Point →₀ ℤ :=
  (n : ℤ) • Finsupp.single P 1 - Finsupp.single (n • P) 1 -
    ((n : ℤ) - 1) • Finsupp.single 0 1

theorem miller_divisor_zero (W : WeierstrassCurve.Affine K) (P : W.Point) :
    millerDivisor W P 0 = 0 := by simp [millerDivisor]

theorem miller_divisor_one (W : WeierstrassCurve.Affine K) (P : W.Point) :
    millerDivisor W P 1 = 0 := by simp [millerDivisor]

theorem miller_divisor_add (W : WeierstrassCurve.Affine K) (P : W.Point) (n m : ℕ) :
    millerDivisor W P (n + m) =
      millerDivisor W P n + millerDivisor W P m + additionDivisor W (n • P) (m • P) := by
  simp only [millerDivisor, additionDivisor, Nat.cast_add,
    add_smul, sub_smul, one_smul]
  abel

/-- An actual recursively constructed rational function, handling zero and vertical steps. -/
def millerFunction (W : WeierstrassCurve.Affine K) (P : W.Point) : ℕ → W.FunctionField
  | 0 => 1
  | n + 1 => millerFunction W P n * stepFunction W (n • P) P

/-- Every actual recursive function has the source principal point divisor; no divisor premise. -/
theorem miller_has_point_divisor (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) :
    HasPointDivisor W (millerFunction W P n) (millerDivisor W P n) := by
  induction n with
  | zero =>
    rw [millerFunction, miller_divisor_zero]
    exact has_point_divisor_one W
  | succ n ih =>
    have hd := ih.mul W (step_has_point_divisor W (n • P) P)
    have he := miller_divisor_add W P n 1
    simp only [miller_divisor_one, one_nsmul, add_zero] at he
    rw [← he] at hd
    exact hd

theorem miller_function_nonzero (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) :
    millerFunction W P n ≠ 0 := (miller_has_point_divisor W P n).1

theorem miller_principal_point_divisor (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) :
    principalPointDivisor W (millerFunction W P n)
      ((miller_has_point_divisor W P n).support_finite W) = millerDivisor W P n :=
  (miller_has_point_divisor W P n).principal W

/-- Any valid i,j actual functions give a valid i+j actual function by the actual addition step.
This is a divisor invariant, not an unproved uniqueness claim about the chosen recursive function. -/
theorem miller_addition_invariant (W : WeierstrassCurve.Affine K) (P : W.Point) (i j : ℕ)
    {f g : W.FunctionField} (hf : HasPointDivisor W f (millerDivisor W P i))
    (hg : HasPointDivisor W g (millerDivisor W P j)) :
    HasPointDivisor W (f * g * stepFunction W (i • P) (j • P)) (millerDivisor W P (i + j)) := by
  rw [miller_divisor_add]
  exact (hf.mul W hg).mul W (step_has_point_divisor W (i • P) (j • P))

theorem miller_torsion_divisor (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ)
    (h : n • P = 0) :
    principalPointDivisor W (millerFunction W P n)
      ((miller_has_point_divisor W P n).support_finite W) =
      (n : ℤ) • Finsupp.single P 1 - (n : ℤ) • Finsupp.single 0 1 := by
  rw [miller_principal_point_divisor, millerDivisor, h, sub_smul, one_smul]
  abel

#print axioms miller_has_point_divisor
#print axioms miller_torsion_divisor
end
end MillerRecursionAudit
