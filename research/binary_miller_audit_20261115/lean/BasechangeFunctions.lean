module
public import BasechangeField

@[expose] public section
namespace MillerBasechangeAudit
noncomputable section
open MillerNormValuationAudit
variable {K L : Type*} [Field K] [Field L] [Algebra K L]

theorem field_base_vertical (W : WeierstrassCurve.Affine K) (x : K) :
    fieldBaseChange (L := L) W (verticalFunction W x) =
      verticalFunction (extendedCurve (L := L) W) (algebraMap K L x) := by
  simp only [vertical_formula, map_sub, field_base_x, field_base_constant]

theorem field_base_line (W : WeierstrassCurve.Affine K) (x y s : K) :
    fieldBaseChange (L := L) W (lineFunction W x y s) =
      lineFunction (extendedCurve (L := L) W) (algebraMap K L x) (algebraMap K L y)
        (algebraMap K L s) := by
  simp only [line_formula, map_sub, map_mul, field_base_y, field_base_constant, field_base_x]

theorem field_base_parameter (W : WeierstrassCurve.Affine K) :
    fieldBaseChange (L := L) W (parameter W) = parameter (extendedCurve (L := L) W) := by
  simp only [parameter, map_div₀, field_base_x, field_base_y]

#print axioms field_base_line
end
end MillerBasechangeAudit
