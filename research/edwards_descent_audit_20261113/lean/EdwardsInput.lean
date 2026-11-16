import Mathlib.Algebra.QuadraticAlgebra.Basic
import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.Tactic

/-! Exact twisted-Edwards input for FGHR2014, printed pp603–604.
The extension and the birational coordinates are actual fields/points.
No regularity, rank or point-group axiom is introduced. -/
namespace EdwardsDescentAudit
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance quadraticIrred : Fact (∀ r : ZMod 5, r^2 ≠ 2+0*r) :=
  ⟨by decide +kernel⟩

abbrev F := ZMod 5
abbrev K := QuadraticAlgebra F 2 0
instance : Fintype K :=
  Fintype.ofEquiv (F×F) (QuadraticAlgebra.equivProd 2 0).symm
instance : CharP K 5 := charP_of_injective_algebraMap' F 5

def beta : K := ⟨0,1⟩
def a : K := beta
def d : K := 4
def targetX : K := 1
def targetY : K := 1+beta

theorem extension_cardinality : Fintype.card K = 25 := by decide +kernel
theorem beta_square : beta^2=2 := by decide +kernel
theorem actual_parameters : a ≠ 0 ∧ d ≠ 0 ∧ a ≠ d := by decide +kernel
theorem target_not_base : ∀ r : F, algebraMap F K r ≠ targetY := by decide +kernel
theorem curve_not_base : ∀ r : F, algebraMap F K r ≠ a := by decide +kernel

def equation (A D x y : K) : Prop := A*x^2+y^2=1+D*x^2*y^2

theorem target_on_edwards : equation a d targetX targetY := by
  unfold equation
  decide +kernel
theorem target_smooth :
    2*targetX*(a-d*targetY^2) ≠ 0 ∧
      2*targetY*(1-d*targetX^2) ≠ 0 := by decide +kernel

-- The standard Edwards/Montgomery change is retained explicitly.
def montgomeryB : K := 4/(a-d)
def toX (_x y : K) : K := (1+y)/((1-y)*montgomeryB)
def toY (x y : K) : K := (1+y)/((1-y)*montgomeryB*x)
def curve : WeierstrassCurve.Affine K := ⟨0,(a+d)/2,0,(a-d)^2/16,0⟩

theorem curve_nonsingular : curve.Δ ≠ 0 := by decide +kernel
theorem target_change_coordinates :
    toX targetX targetY = 3+2*beta ∧
      toY targetX targetY = 3+2*beta := by decide +kernel

def R : curve.Point := .some (toX targetX targetY) (toY targetX targetY) (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
    curve_nonsingular).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel)

-- Two actual factor-base points, with Edwards y-coordinates in F5.
def px : K := -(2+3*beta)
def qx : K := 2+3*beta
theorem factor_base_points : equation a d px 2 ∧ equation a d qx 3 := by
  unfold equation
  decide +kernel
theorem edwards_addition :
    (px*3+2*qx)/(1+d*px*qx*2*3)=targetX ∧
      (2*3-a*px*qx)/(1-d*px*qx*2*3)=targetY ∧
      1+d*px*qx*2*3 ≠ 0 ∧ 1-d*px*qx*2*3 ≠ 0 := by decide +kernel

def P : curve.Point := .some (toX px 2) (toY px 2) (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
    curve_nonsingular).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel)
def Q : curve.Point := .some (toX qx 3) (toY qx 3) (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
    curve_nonsingular).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel)

theorem actual_sum : P+Q=R := by decide +kernel
theorem target_nonzero : R ≠ 0 := by decide +kernel

-- The precise printed polynomial, retaining a/d and 2(d-a)/d.
def sourceF3 {L : Type*} [Field L] (A D y₁ y₂ y₃ : L) : L :=
  (y₁^2*y₂^2-y₁^2-y₂^2+A/D)*y₃^2+
    2*((D-A)/D)*y₁*y₂*y₃+(A/D)*(y₁^2+y₂^2-1)-y₁^2*y₂^2

theorem symmetrization {L : Type*} [Field L] (A D c x y : L) (hD : D ≠ 0) :
    sourceF3 A D x y c =
      (c^2-1)*(x*y)^2+(A/D-c^2)*(x+y)^2+
        (2*c^2+2*(1-A/D)*c-2*(A/D))*(x*y)+(A/D)*(c^2-1) := by
  unfold sourceF3
  field_simp
  ring

theorem actual_ratio : a/d=4*beta := by decide +kernel
theorem source_relation : sourceF3 a d (2:K) 3 targetY=0 := by decide +kernel

end EdwardsDescentAudit
