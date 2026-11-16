import Mathlib.Algebra.QuadraticAlgebra
import Mathlib.RingTheory.Ideal.Quotient.Operations
import Mathlib.Algebra.Polynomial.SpecificDegree
import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

namespace Round04Assembly
open Polynomial
abbrev A := QuadraticAlgebra (Polynomial ℚ) (-1) (-1)

example (x y : A) : (x * y).re = x.re * y.re - x.im * y.im := by
  rw [QuadraticAlgebra.re_mul]
  ring

example (c x q : A) (h : x = c * q) :
    Ideal.Quotient.mk (Ideal.span ({c} : Set A)) x = 0 :=
  Ideal.Quotient.eq_zero_iff_mem.mpr (Ideal.mem_span_singleton.mpr ⟨q, h⟩)

example : ((⟨X, 0⟩ : A) * ⟨X, 0⟩).re = X ^ 2 := by
  rw [QuadraticAlgebra.re_mul]
  simp
  ring

instance : Fact (Nat.Prime 7) := ⟨by decide⟩

example (hdeg : (X ^ 3 + 3 * X + 5 : Polynomial (ZMod 7)).natDegree ∈ Finset.Icc 1 3)
    (hnot : ∀ x, ¬ IsRoot (X ^ 3 + 3 * X + 5 : Polynomial (ZMod 7)) x) :
    Irreducible (X ^ 3 + 3 * X + 5 : Polynomial (ZMod 7)) :=
  Polynomial.irreducible_of_degree_le_three_of_not_isRoot hdeg hnot

end Round04Assembly
