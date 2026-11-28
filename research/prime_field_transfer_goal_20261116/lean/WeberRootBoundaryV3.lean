import Mathlib.Algebra.QuadraticAlgebra
import Mathlib.RingTheory.Ideal.Quotient.Operations
import Mathlib.Algebra.Polynomial.SpecificDegree
import Mathlib.Tactic

namespace WeberRootBoundaryV3
noncomputable section
open scoped QuadraticAlgebra
open Polynomial
abbrev A := QuadraticAlgebra (Polynomial ℚ) (-1) (-1)
def W : A := ⟨X, 0⟩
def kernel (t : A) : A :=
  t ^ 3 + (⟨-36, -24⟩ : A) * t ^ 2 + (⟨-48, -48⟩ : A) * t +
    ⟨Polynomial.C ((-128/19) : ℚ), Polynomial.C ((192/19) : ℚ)⟩

def homogeneous (t : A) (d : ℕ) : A :=
  19 * t ^ 3 + (⟨-684, -456⟩ : A) * t ^ 2 * d +
    (⟨-912, -912⟩ : A) * t * d ^ 2 + (⟨-128, 192⟩ : A) * d ^ 3

def betaTwo : A := ⟨(0 + Polynomial.C ((16/9) : ℚ) + Polynomial.C ((-175/9) : ℚ) * X ^ 1 + Polynomial.C ((5/18) : ℚ) * X ^ 2), (0 + Polynomial.C ((44/9) : ℚ) + Polynomial.C ((-20/9) : ℚ) * X ^ 1 + Polynomial.C ((-13/36) : ℚ) * X ^ 2)⟩
def quotientTwo : A := ⟨(0 + Polynomial.C ((-9037/243) : ℚ) + Polynomial.C ((44939/324) : ℚ) * X ^ 1 + Polynomial.C ((439/648) : ℚ) * X ^ 2 + Polynomial.C ((-2089/15552) : ℚ) * X ^ 3), (0 + Polynomial.C ((-11477/243) : ℚ) + Polynomial.C ((-7094/81) : ℚ) * X ^ 1 + Polynomial.C ((15457/1296) : ℚ) * X ^ 2 + Polynomial.C ((-1495/7776) : ℚ) * X ^ 3)⟩
def numeratorTwo : A := ⟨(0 + Polynomial.C ((64) : ℚ) + Polynomial.C ((-700) : ℚ) * X ^ 1 + Polynomial.C ((10) : ℚ) * X ^ 2), (0 + Polynomial.C ((176) : ℚ) + Polynomial.C ((-80) : ℚ) * X ^ 1 + Polynomial.C ((-13) : ℚ) * X ^ 2)⟩
def clearedQuotientTwo : A := ⟨(0 + Polynomial.C ((-1735104) : ℚ) + Polynomial.C ((6471216) : ℚ) * X ^ 1 + Polynomial.C ((31608) : ℚ) * X ^ 2 + Polynomial.C ((-6267) : ℚ) * X ^ 3), (0 + Polynomial.C ((-2203584) : ℚ) + Polynomial.C ((-4086144) : ℚ) * X ^ 1 + Polynomial.C ((556452) : ℚ) * X ^ 2 + Polynomial.C ((-8970) : ℚ) * X ^ 3)⟩

theorem beta_two_identity : kernel betaTwo = kernel W * quotientTwo := by
  apply QuadraticAlgebra.ext <;> apply Polynomial.funext <;> intro x <;>
    simp [kernel, betaTwo, quotientTwo, W, pow_succ,
      QuadraticAlgebra.re_mul, QuadraticAlgebra.im_mul] <;> ring

theorem beta_two_root :
    Ideal.Quotient.mk (Ideal.span ({kernel W} : Set A)) (kernel betaTwo) = 0 :=
  Ideal.Quotient.eq_zero_iff_mem.mpr (Ideal.mem_span_singleton.mpr
    ⟨quotientTwo, beta_two_identity⟩)

theorem beta_two_cleared_identity :
    (1 : A) * homogeneous numeratorTwo 36 =
      homogeneous W 1 * clearedQuotientTwo := by
  apply QuadraticAlgebra.ext <;> apply Polynomial.funext <;> intro x <;>
    simp [homogeneous, numeratorTwo, clearedQuotientTwo, W, pow_succ,
      QuadraticAlgebra.re_mul, QuadraticAlgebra.im_mul] <;> ring

def betaFour : A := ⟨(0 + Polynomial.C ((308/9) : ℚ) + Polynomial.C ((166/9) : ℚ) * X ^ 1 + Polynomial.C ((-5/18) : ℚ) * X ^ 2), (0 + Polynomial.C ((172/9) : ℚ) + Polynomial.C ((20/9) : ℚ) * X ^ 1 + Polynomial.C ((13/36) : ℚ) * X ^ 2)⟩
def quotientFour : A := ⟨(0 + Polynomial.C ((-76472/243) : ℚ) + Polynomial.C ((-37055/324) : ℚ) * X ^ 1 + Polynomial.C ((-671/1296) : ℚ) * X ^ 2 + Polynomial.C ((2089/15552) : ℚ) * X ^ 3), (0 + Polynomial.C ((-7801/243) : ℚ) + Polynomial.C ((21239/324) : ℚ) * X ^ 1 + Polynomial.C ((-7085/648) : ℚ) * X ^ 2 + Polynomial.C ((1495/7776) : ℚ) * X ^ 3)⟩
def numeratorFour : A := ⟨(0 + Polynomial.C ((1232) : ℚ) + Polynomial.C ((664) : ℚ) * X ^ 1 + Polynomial.C ((-10) : ℚ) * X ^ 2), (0 + Polynomial.C ((688) : ℚ) + Polynomial.C ((80) : ℚ) * X ^ 1 + Polynomial.C ((13) : ℚ) * X ^ 2)⟩
def clearedQuotientFour : A := ⟨(0 + Polynomial.C ((-14682624) : ℚ) + Polynomial.C ((-5335920) : ℚ) * X ^ 1 + Polynomial.C ((-24156) : ℚ) * X ^ 2 + Polynomial.C ((6267) : ℚ) * X ^ 3), (0 + Polynomial.C ((-1497792) : ℚ) + Polynomial.C ((3058416) : ℚ) * X ^ 1 + Polynomial.C ((-510120) : ℚ) * X ^ 2 + Polynomial.C ((8970) : ℚ) * X ^ 3)⟩

theorem beta_four_identity : kernel betaFour = kernel W * quotientFour := by
  apply QuadraticAlgebra.ext <;> apply Polynomial.funext <;> intro x <;>
    simp [kernel, betaFour, quotientFour, W, pow_succ,
      QuadraticAlgebra.re_mul, QuadraticAlgebra.im_mul] <;> ring

theorem beta_four_root :
    Ideal.Quotient.mk (Ideal.span ({kernel W} : Set A)) (kernel betaFour) = 0 :=
  Ideal.Quotient.eq_zero_iff_mem.mpr (Ideal.mem_span_singleton.mpr
    ⟨quotientFour, beta_four_identity⟩)

theorem beta_four_cleared_identity :
    (1 : A) * homogeneous numeratorFour 36 =
      homogeneous W 1 * clearedQuotientFour := by
  apply QuadraticAlgebra.ext <;> apply Polynomial.funext <;> intro x <;>
    simp [homogeneous, numeratorFour, clearedQuotientFour, W, pow_succ,
      QuadraticAlgebra.re_mul, QuadraticAlgebra.im_mul] <;> ring

end
end WeberRootBoundaryV3
