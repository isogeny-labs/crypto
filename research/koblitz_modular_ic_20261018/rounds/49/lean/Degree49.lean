import Mathlib.LinearAlgebra.Basis.VectorSpace
import Mathlib.LinearAlgebra.Dimension.Free
import Mathlib.LinearAlgebra.Dimension.FreeAndStrongRankCondition
namespace Degree49
/-- Equal positive generic degrees force relative degree one in a field tower.
The concrete fraction fields and their degrees are hypotheses, not imported CAS facts. -/
theorem relative_degree_one
    (K L M : Type*) [Field K] [Field L] [Field M]
    [Algebra K L] [Algebra L M] [Algebra K M] [IsScalarTower K L M]
    (d : ℕ) (hd : 0 < d)
    (hKL : Module.finrank K L = d) (hKM : Module.finrank K M = d) :
    Module.finrank L M = 1 := by
  have h := Module.finrank_mul_finrank K L M
  rw [hKL, hKM] at h
  exact Nat.eq_of_mul_eq_mul_left hd (by simpa using h)

/-- In a tower of fields, the equal-positive-degree criterion makes the
intermediate algebra map bijective. No affine normalization or geometric map
is assumed to have been computed by this theorem. -/
theorem equal_degree_map_bijective
    (K L M : Type*) [Field K] [Field L] [Field M]
    [Algebra K L] [Algebra L M] [Algebra K M] [IsScalarTower K L M]
    (d : ℕ) (hd : 0 < d)
    (hKL : Module.finrank K L = d) (hKM : Module.finrank K M = d) :
    Function.Bijective (algebraMap L M) := by
  apply (Algebra.finrank_eq_one_iff_bijective_algebraMap).mp
  exact relative_degree_one K L M d hd hKL hKM
end Degree49
