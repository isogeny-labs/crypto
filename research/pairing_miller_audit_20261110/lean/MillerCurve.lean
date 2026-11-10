import ThreeTorsion
import Mathlib.Algebra.QuadraticAlgebra.Basic

namespace PairingMillerCurveAudit
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩
private instance quadraticIrred : Fact (∀ r : ZMod 5,r^2≠2+0*r) := ⟨by decide +kernel⟩

abbrev F25 := QuadraticAlgebra (ZMod 5) 2 0
instance : Fintype F25 :=
  Fintype.ofEquiv (ZMod 5×ZMod 5) (QuadraticAlgebra.equivProd 2 0).symm
instance : CharP F25 5 := charP_of_injective_algebraMap' (ZMod 5) 5

def beta : F25 := ⟨0,1⟩
def omega : F25 := ⟨2,3⟩
abbrev curve := BKActualThreeAudit.geomCurve (L:=F25)

theorem field_cardinality : Fintype.card F25=25 := by decide +kernel
theorem beta_square : beta^2=2 := by decide +kernel
theorem primitive_cube_root : IsPrimitiveRoot omega 3 := by
  apply (IsPrimitiveRoot.iff (by decide : 0<3)).mpr
  constructor
  · decide +kernel
  · intro m hm hn
    interval_cases m <;> decide +kernel

theorem curve_nonsingular : curve.Δ≠0 := BKActualThreeAudit.geom_nonsingular

def P : curve.Point := .some 0 1 (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_nonsingular).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel)

def Q : curve.Point := .some 1 beta (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_nonsingular).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel)

/-- The nonzero order-two auxiliary point used by every prescribed expression. -/
def auxiliary : curve.Point := .some 4 0 (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_nonsingular).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel)

theorem P_three : (3:ℕ) • P=0 := by decide +kernel
theorem Q_three : (3:ℕ) • Q=0 := by decide +kernel
theorem P_ne_zero : P≠0 := by decide +kernel
theorem Q_ne_zero : Q≠0 := by decide +kernel
theorem auxiliary_two : auxiliary+auxiliary=0 := by decide +kernel
theorem auxiliary_ne_zero : auxiliary≠0 := by decide +kernel

abbrev Index := ZMod 3×ZMod 3
def indexedPoint (v : Index) : curve.Point := v.1.val • P+v.2.val • Q

theorem indexedPoint_three (v : Index) : (3:ℕ) • indexedPoint v=0 := by
  unfold indexedPoint
  rw [nsmul_add,smul_comm 3 v.1.val P,smul_comm 3 v.2.val Q,P_three,Q_three]
  simp

#print axioms indexedPoint_three
#print axioms primitive_cube_root
end PairingMillerCurveAudit
