module
public import Mathlib.LinearAlgebra.Matrix.Nondegenerate
public import Mathlib.LinearAlgebra.Matrix.Notation
public import Mathlib.Data.ZMod.Basic
public import Mathlib.Algebra.Field.ZMod

public section

open scoped Matrix

-- Assembly only: this does not bind the native matrix to elliptic geometry.
example {K : Type*} [Field K] (M : Matrix (Fin 6) (Fin 6) K)
    (c : Fin 6 → K) (hne : c ≠ 0) (hker : M *ᵥ c = 0) : M.det = 0 := by
  by_contra hdet
  exact hne (Matrix.eq_zero_of_mulVec_eq_zero hdet hker)

#print axioms Matrix.eq_zero_of_mulVec_eq_zero

-- Concrete native toy certificate literals, not a formal jet/group conversion.
namespace SectionCertificate
instance : Fact (Nat.Prime 101) := ⟨by decide⟩

def toyMatrix : Matrix (Fin 6) (Fin 6) (ZMod 101) :=
  !![1, 64, 22, 56, 95, 40;
    0, 1, 30, 27, 23, 69;
    0, 0, 71, 1, 29, 67;
    0, 0, 57, 0, 83, 69;
    0, 0, 12, 0, 17, 60;
    0, 0, 43, 0, 37, 87]

def toySection : Fin 6 → ZMod 101 := ![1, 22, 84, 13, 61, 8]

theorem toy_section_kernel : toyMatrix *ᵥ toySection = 0 ∧ toySection ≠ 0 := by
  decide

theorem toy_section_det_zero : toyMatrix.det = 0 := by
  by_contra hdet
  exact toy_section_kernel.2
    (Matrix.eq_zero_of_mulVec_eq_zero hdet toy_section_kernel.1)

#print axioms toy_section_kernel
#print axioms toy_section_det_zero
end SectionCertificate
