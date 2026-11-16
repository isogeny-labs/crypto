import Common22BranchStalk

/-!
# Factorization at a finite Weierstrass point

At a rational root of the monic quintic, the affine equation factors as
`y² = (x-a) q_a(x)`.  The value of `q_a` at that point is the formal
derivative.  This file stops before the local-ring unit and DVR steps.
-/

set_option autoImplicit false
noncomputable section

namespace Common22BranchParameter

open Common22SeparateChartMap Common22BranchStalk MvPolynomial

variable {K : Type*} [Field K]

private abbrev c (a3 a2 a1 a0 : K) (b : K) :
    AffineRing a3 a2 a1 a0 :=
  algebraMap K (AffineRing a3 a2 a1 a0) b

def qAt (a3 a2 a1 a0 a : K) : AffineRing a3 a2 a1 a0 :=
  let x := xbar a3 a2 a1 a0
  x ^ 4 + c a3 a2 a1 a0 a * x ^ 3 + c a3 a2 a1 a0 (a ^ 2) * x ^ 2 +
    c a3 a2 a1 a0 (a ^ 3) * x + c a3 a2 a1 a0 (a ^ 4) +
    c a3 a2 a1 a0 a3 * (x ^ 2 + c a3 a2 a1 a0 a * x + c a3 a2 a1 a0 (a ^ 2)) +
    c a3 a2 a1 a0 a2 * (x + c a3 a2 a1 a0 a) + c a3 a2 a1 a0 a1

def derivativeAt (a3 a2 a1 a0 a : K) : K :=
  5 * a ^ 4 + 3 * a3 * a ^ 2 + 2 * a2 * a + a1

theorem branch_factorization (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    ybar a3 a2 a1 a0 ^ 2 =
      (xbar a3 a2 a1 a0 - c a3 a2 a1 a0 a) * qAt a3 a2 a1 a0 a := by
  have hr : c a3 a2 a1 a0 (fAt a3 a2 a1 a0 a) = 0 := by
    rw [hroot]
    exact map_zero _
  simp only [fAt, map_add, map_mul, map_pow] at hr
  rw [quotient_affine_relation]
  dsimp [qAt]
  simp only [c, map_pow]
  linear_combination hr

theorem branchEval_qAt (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    branchEval a3 a2 a1 a0 a hroot (qAt a3 a2 a1 a0 a) =
      derivativeAt a3 a2 a1 a0 a := by
  have hc (b : K) :
      branchEval a3 a2 a1 a0 a hroot (c a3 a2 a1 a0 b) = b := by
    change preEval a (C b) = b
    simp [preEval]
  simp only [qAt, map_add, map_mul, map_pow,
    branchEval_xbar, hc, derivativeAt]
  ring

theorem branch_qAt_local_unit (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    IsUnit ((algebraMap (AffineRing a3 a2 a1 a0)
      (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)))
      (qAt a3 a2 a1 a0 a)) := by
  apply (IsLocalization.AtPrime.isUnit_to_map_iff
    (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot))
    (branchIdeal a3 a2 a1 a0 a hroot) _).mpr
  simp only [Ideal.mem_primeCompl_iff]
  intro hmem
  have hzero : branchEval a3 a2 a1 a0 a hroot (qAt a3 a2 a1 a0 a) = 0 := by
    exact (RingHom.mem_ker).mp hmem
  exact hsimple ((branchEval_qAt a3 a2 a1 a0 a hroot).symm.trans hzero)

#print axioms branch_factorization
#print axioms branchEval_qAt
#print axioms branch_qAt_local_unit

end Common22BranchParameter
