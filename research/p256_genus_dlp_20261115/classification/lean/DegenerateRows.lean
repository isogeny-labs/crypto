import Mathlib

/-! Algebraic row decomposition only. Source-only in this runtime; this does
not formalize Frobenius, pair partitions, rational maps or Jacobian isogenies. -/
namespace P256CoverClassification
variable {R : Type*} [CommRing R]

def smithRow (b c : R) : Fin 6 → R := ![1, b, c, c^2, -c*b, b^2-c]
def row0 (c : R) : Fin 6 → R := ![1, 0, c, c^2, 0, -c]
def row1 (c : R) : Fin 6 → R := ![0, 1, 0, 0, -c, 0]
def row2 : Fin 6 → R := ![0, 0, 0, 0, 0, 1]

theorem common_constant_row_decomposition (b c : R) :
    smithRow b c = row0 c + b • row1 c + b^2 • row2 := by
  ext i
  fin_cases i <;> simp [smithRow, row0, row1, row2] <;> ring

end P256CoverClassification
