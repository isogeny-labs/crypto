import BinaryFourthControl

namespace SemaevBinaryFourthPointsAudit
noncomputable section
open SemaevBinaryFourthAudit SemaevBinaryFourthControlAudit GeneralS4Audit SemaevS3
local instance : DecidableEq F := Classical.decEq _

theorem curve_nonsingular : curve.Δ≠0 := by
  norm_num [curve,binaryCurve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,
    WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈,CharTwo.neg_eq]
  have h431 : (431:F)=1 := by linear_combination 215*DiemSmallFieldAudit.two_zero
  rw [h431]
  exact one_ne_zero

def mkPoint (x y : F) (h : y^2+x*y=x^3+1) : curve.Point :=
  .some x y (by
    apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
      curve_nonsingular).mp
    rw [WeierstrassCurve.Affine.equation_iff]
    simpa [curve,binaryCurve] using h)

def P : curve.Point := mkPoint 1 0 (by
  norm_num
  exact DiemSmallFieldAudit.two_zero.symm)
def T : curve.Point := mkPoint 0 1 (by norm_num)
def R : curve.Point := mkPoint alpha 0 (by
  rw [alpha_cube]
  norm_num
  exact DiemSmallFieldAudit.two_zero.symm)
def S : curve.Point := mkPoint (alpha^2) (alpha^2) (by
  rw [show (alpha^2)^3=(alpha^3)^2 by ring,alpha_cube]
  linear_combination DiemSmallFieldAudit.two_zero*((alpha^2)^2-1))

theorem P_double : P+P=T := by
  unfold P T mkPoint
  rw [WeierstrassCurve.Affine.Point.add_self_of_Y_ne (by
    norm_num [curve,binaryCurve,WeierstrassCurve.Affine.negY])]
  rw [WeierstrassCurve.Affine.Point.some.injEq]
  norm_num [curve,binaryCurve,WeierstrassCurve.Affine.addX,
    WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY,WeierstrassCurve.Affine.slope,CharTwo.neg_eq]
  constructor
  · linear_combination 5*DiemSmallFieldAudit.two_zero
  · linear_combination 8*DiemSmallFieldAudit.two_zero

theorem R_S_slope : curve.slope alpha (alpha^2) 0 (alpha^2)=alpha^2 := by
  rw [WeierstrassCurve.Affine.slope_of_X_ne DiemSmallFieldAudit.alpha_not_fixed.symm]
  have hd : alpha-alpha^2=1 := by
    rw [CharTwo.sub_eq_add,alpha_square]
    rw [←add_assoc,CharTwo.add_self_eq_zero,zero_add]
  rw [hd]
  simp [CharTwo.neg_eq]

theorem R_add_S : R+S=T := by
  have hx : curve.addX alpha (alpha^2) (alpha^2)=0 := by
    change (alpha^2)^2+1*(alpha^2)-0-alpha-alpha^2=0
    rw [show (alpha^2)^2=alpha^4 by ring,DiemSmallFieldAudit.pow_four]
    ring
  unfold R S T mkPoint
  rw [WeierstrassCurve.Affine.Point.add_of_X_ne DiemSmallFieldAudit.alpha_not_fixed.symm]
  rw [WeierstrassCurve.Affine.Point.some.injEq,R_S_slope]
  constructor
  · exact hx
  · change curve.addY alpha (alpha^2) 0 (alpha^2)=1
    rw [WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,hx]
    change -(alpha^2*(0-alpha)+0)-1*0-0=1
    calc
      _=alpha^3 := by ring
      _=1 := alpha_cube

theorem T_double : T+T=0 := by
  unfold T mkPoint
  apply WeierstrassCurve.Affine.Point.add_self_of_Y_eq
  simp [curve,binaryCurve,WeierstrassCurve.Affine.negY,CharTwo.neg_eq]

theorem actual_four_point_sum : P+P+R+S=0 := by
  calc
    P+P+R+S=(P+P)+(R+S) := by abel
    _=T+T := by rw [P_double,R_add_S]
    _=0 := T_double

theorem actual_point_witness : fourPoints curve 1 1 alpha (alpha^2) :=
  ⟨P,P,R,S,rfl,rfl,rfl,rfl,actual_four_point_sum⟩

#print axioms actual_point_witness
end
end SemaevBinaryFourthPointsAudit
