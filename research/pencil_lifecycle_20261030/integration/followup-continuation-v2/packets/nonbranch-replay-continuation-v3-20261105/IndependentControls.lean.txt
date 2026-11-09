import Nonbranch
import Mathlib.Tactic.Ring
import Mathlib.Tactic.NormNum
set_option autoImplicit false
noncomputable section
namespace IndependentNonbranch
open Polynomial ActualNonbranchNorm Common22SeparateChartMap Common22BranchStalk
  WeightedProjNonbranchZeroOrder WeightedProjSmoothPoints
abbrev F : ℚ[X] := quintic (-5) 0 4 1
abbrev u : ℚ[X] := Common22Coprime.firstSupport 0 (-1)
abbrev v : ℚ[X] := Common22Coprime.secondSupport 0 (-4)
abbrev n : ℚ[X] := numerator 0 (-1) 0 1 0 (-4) 0 1
abbrev U : ℚ[X] := residual 0 (-1) 0 1 0 (-4) 0 1
abbrev b : ℚ := (Common22SupportHead.coefficientData 0 (-1) 0 1 0 (-4) 0 1).1

theorem coefficients : Common22SupportHead.coefficientData (0:ℚ) (-1) 0 1 0 (-4) 0 1 =
 (-9,0,0,-81,0) := by norm_num [Common22SupportHead.coefficientData]
theorem numerator_value : n = C (-9) := by norm_num [n,numerator,coefficients]
theorem residual_value : U = C (-81)*X := by norm_num [U,residual,coefficients]
theorem determinant_nonzero : b ≠ 0 := by norm_num [b,coefficients]
theorem valid_first : u ∣ F-(C (0:ℚ)*X+C 1)^2 := by
 refine ⟨X*v, ?_⟩
 simp only [u,v,F,Common22Coprime.firstSupport,Common22Coprime.secondSupport,quintic]
 norm_num
 ring
theorem valid_second : v ∣ F-(C (0:ℚ)*X+C 1)^2 := by
 refine ⟨X*u, ?_⟩
 simp only [u,v,F,Common22Coprime.firstSupport,Common22Coprime.secondSupport,quintic]
 norm_num
 ring
abbrev aa : ℚ[X] := -44150*X^3+14500*X^2+112400*X-42375
abbrev bb : ℚ[X] := 8830*X^4-2900*X^3-40140*X^2+14275*X+20236
-- Explicit exact Bezout certificate, independent of producer separability premise.
theorem bezout : aa*F+bb*derivative F = C (38569:ℚ) := by
 norm_num [aa,bb,F,quintic,derivative_add,derivative_sub,derivative_mul,derivative_pow]
 ring
theorem separable : F.Separable := by
 rw [Polynomial.separable_def']
 refine ⟨C (1/38569:ℚ)*aa,C (1/38569:ℚ)*bb, ?_⟩
 calc
  _ = C (1/38569:ℚ)*(aa*F+bb*derivative F) := by ring
  _ = 1 := by rw [bezout,←map_mul]; norm_num

theorem point_plus : (1:ℚ)^2 = fAt (-5) 0 4 1 1 := by norm_num [fAt]
theorem point_minus : (-1:ℚ)^2 = fAt (-5) 0 4 1 1 := by norm_num [fAt]
theorem branch_plus : n.eval 1 = b*(1:ℚ) := by rw [numerator_value]; norm_num [b,coefficients]
theorem branch_minus_false : n.eval 1 ≠ b*(-1:ℚ) := by rw [numerator_value]; norm_num [b,coefficients]

theorem exact_source_identity : n^2-u*v*U = C (b^2)*F :=
 source_norm_identity (-5) 0 4 1 0 (-1) 0 1 0 (-4) 0 1
 determinant_nonzero valid_first valid_second

abbrev Rp := ProjPointRing (-5:ℚ) 0 4 1 1 1 point_plus
abbrev Rm := ProjPointRing (-5:ℚ) 0 4 1 1 (-1) point_minus
local instance : IsDomain Rp := Common22SeparableAffineDomain.projective_point_domain_of_separable
 (-5:ℚ) 0 4 1 1 1 point_plus separable
local instance : IsDiscreteValuationRing Rp := WeightedProjNonbranchDVR.projective_point_dvr
 (-5:ℚ) 0 4 1 1 1 point_plus separable (by norm_num)
local instance : IsDomain Rm := Common22SeparableAffineDomain.projective_point_domain_of_separable
 (-5:ℚ) 0 4 1 1 (-1) point_minus separable
local instance : IsDiscreteValuationRing Rm := WeightedProjNonbranchDVR.projective_point_dvr
 (-5:ℚ) 0 4 1 1 (-1) point_minus separable (by norm_num)
abbrev ep := germEval (-5:ℚ) 0 4 1 1 1 point_plus
abbrev em := germEval (-5:ℚ) 0 4 1 1 (-1) point_minus
abbrev gp : Rp := ep n - iota (-5:ℚ) 0 4 1 1 1 point_plus b * pointY (-5:ℚ) 0 4 1 1 1 point_plus
abbrev gm : Rm := em n - iota (-5:ℚ) 0 4 1 1 (-1) point_minus b * pointY (-5:ℚ) 0 4 1 1 (-1) point_minus

theorem selected_nonunit : ¬ IsUnit gp := numerator_nonunit (-5:ℚ) 0 4 1 1 1 b point_plus n branch_plus
-- The actual source theorem is instantiated with all fixture hypotheses proved above.
theorem selected_order : IsDiscreteValuationRing.addVal Rp gp =
 IsDiscreteValuationRing.addVal Rp (ep u)+IsDiscreteValuationRing.addVal Rp (ep v)+
 IsDiscreteValuationRing.addVal Rp (ep U) := by
 exact source_actual_norm_order (-5:ℚ) 0 4 1 1 1 0 (-1) 0 1 0 (-4) 0 1
 point_plus separable (by norm_num) (by norm_num) determinant_nonzero valid_first valid_second branch_plus

-- Independently expose the actual point-localization criterion used for the conjugate point.
theorem rho_unit_iff (t : A (-5:ℚ) 0 4 1) :
 IsUnit (rho (-5:ℚ) 0 4 1 1 (-1) point_minus t) ↔
 pointEval (-5:ℚ) 0 4 1 1 (-1) point_minus t ≠ 0 := by
 constructor
 · intro hu ht
   have hm := hu.map (pointRingEquiv (-5:ℚ) 0 4 1 1 (-1) point_minus).toRingHom
   have hl : IsUnit (algebraMap (A (-5:ℚ) 0 4 1) (L (-5:ℚ) 0 4 1 1 (-1) point_minus) t) := by
    simpa [rho] using hm
   exact ((IsLocalization.AtPrime.isUnit_to_map_iff
    (L (-5:ℚ) 0 4 1 1 (-1) point_minus)
    (pointIdeal (-5:ℚ) 0 4 1 1 (-1) point_minus) t).mp hl) ht
 · intro ht
   have hl := (IsLocalization.AtPrime.isUnit_to_map_iff
    (L (-5:ℚ) 0 4 1 1 (-1) point_minus)
    (pointIdeal (-5:ℚ) 0 4 1 1 (-1) point_minus) t).mpr ht
   exact hl.map (pointRingEquiv (-5:ℚ) 0 4 1 1 (-1) point_minus).symm.toRingHom

theorem conjugate_unit : IsUnit gm := by
 let t := affineEval (-5:ℚ) 0 4 1 n - algebraMap ℚ (A (-5:ℚ) 0 4 1) b * ybar (-5:ℚ) 0 4 1
 have ht : pointEval (-5:ℚ) 0 4 1 1 (-1) point_minus t ≠ 0 := by
  dsimp [t]
  rw [map_sub,map_mul,point_affine_eval,point_scalar,pointEval_ybar,numerator_value]
  norm_num [b,coefficients]
 have hu := (rho_unit_iff t).mpr ht
 simpa [gm,em,germEval,iota,rho,pointY,t,map_sub,map_mul] using hu

theorem conjugate_order_zero : IsDiscreteValuationRing.addVal Rm gm = 0 :=
 IsDiscreteValuationRing.addVal_eq_zero_iff.mpr conjugate_unit

theorem conjugate_product_nonunit : ¬ IsUnit (em u*em v*em U) := by
 have ht : pointEval (-5:ℚ) 0 4 1 1 (-1) point_minus (affineEval (-5:ℚ) 0 4 1 (u*v*U)) = 0 := by
  rw [point_affine_eval]
  simp [u,Common22Coprime.firstSupport]
 intro hu
 have hm : IsUnit (rho (-5:ℚ) 0 4 1 1 (-1) point_minus (affineEval (-5:ℚ) 0 4 1 (u*v*U))) := by
  simpa [em,germEval,map_mul] using hu
 exact ((rho_unit_iff _).mp hm) ht

theorem conjugate_order_unequal : IsDiscreteValuationRing.addVal Rm gm ≠
 IsDiscreteValuationRing.addVal Rm (em u)+IsDiscreteValuationRing.addVal Rm (em v)+
 IsDiscreteValuationRing.addVal Rm (em U) := by
 intro he
 have hz : IsDiscreteValuationRing.addVal Rm (em u*em v*em U) = 0 := by
  rw [map_mul,map_mul]
  exact he.symm.trans conjugate_order_zero
 exact conjugate_product_nonunit (IsDiscreteValuationRing.addVal_eq_zero_iff.mp hz)
end IndependentNonbranch
