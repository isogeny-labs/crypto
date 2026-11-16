import CubeAutomorphism
import Mathlib.Algebra.QuadraticAlgebra.Basic
import Mathlib.GroupTheory.OrderOfElement

namespace VerheulDistortionAudit
private instance prime29 : Fact (Nat.Prime 29) := ⟨by decide⟩
private instance prime5 : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance quadratic_irreducible :
    Fact (∀ r : ZMod 29,r^2 ≠ -1+(-1)*r) := ⟨by decide +kernel⟩

abbrev Base := ZMod 29
abbrev Extension := QuadraticAlgebra Base (-1) (-1)
instance : Fintype Extension :=
  Fintype.ofEquiv (Base×Base) (QuadraticAlgebra.equivProd (-1) (-1)).symm
instance : CharP Extension 29 := charP_of_injective_algebraMap' Base 29

def omega : Extension := ⟨0,1⟩

theorem extension_card : Nat.card Extension = 29^2 := by
  calc
    Nat.card Extension = Fintype.card Extension := Nat.card_eq_fintype_card
    _ = 29^2 := by decide +kernel

theorem omega_cube : omega^3=1 := by decide +kernel
theorem omega_ne_one : omega≠1 := by decide +kernel

theorem omega_primitive : IsPrimitiveRoot omega 3 := by
  apply (IsPrimitiveRoot.iff (by omega : 0<3)).mpr
  constructor
  · exact omega_cube
  · intro n hn hn0
    interval_cases n <;> decide +kernel

theorem omega_not_in_base (r : Base) : omega ≠ algebraMap Base Extension r := by
  intro h
  have hi := congrArg QuadraticAlgebra.im h
  simp [omega] at hi

theorem base_discriminant : (model (1 : Base)).Δ≠0 := by decide +kernel
theorem extension_discriminant : (model (1 : Extension)).Δ≠0 := by decide +kernel

def baseP : (model (1 : Base)).Point := .some 4 6 (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero base_discriminant).mp
  rw [equation_iff]
  decide +kernel)

def P : (model (1 : Extension)).Point := .some 4 6 (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero extension_discriminant).mp
  rw [equation_iff]
  decide +kernel)

theorem baseP_five : (5 : ℕ) • baseP=0 := by decide +kernel
theorem baseP_ne_zero : baseP≠0 := by decide +kernel
theorem baseP_order : addOrderOf baseP=5 := addOrderOf_eq_prime baseP_five baseP_ne_zero
theorem P_five : (5 : ℕ) • P=0 := by decide +kernel
theorem P_ne_zero : P≠0 := by decide +kernel
theorem P_order : addOrderOf P=5 := addOrderOf_eq_prime P_five P_ne_zero

def BaseEquation (xy : Base×Base) : Prop := xy.2^2=xy.1^3+1
instance : DecidablePred BaseEquation := fun xy => inferInstanceAs (Decidable (xy.2^2=xy.1^3+1))
abbrev BaseCode := Option {xy : Base×Base // BaseEquation xy}

def basePointEquivCode : (model (1 : Base)).Point ≃ BaseCode where
  toFun
    | 0 => none
    | .some x y h => some ⟨(x,y),(equation_iff 1 x y).mp h.1⟩
  invFun
    | none => 0
    | some ⟨(x,y),h⟩ => .some x y
      ((WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero base_discriminant).mp
        ((equation_iff 1 x y).mpr h))
  left_inv := by rintro (_ | _) <;> rfl
  right_inv := by rintro (_ | ⟨⟨x,y⟩,h⟩) <;> rfl

instance : Finite (model (1 : Base)).Point := Finite.of_equiv BaseCode basePointEquivCode.symm

theorem actual_base_point_card : Nat.card (model (1 : Base)).Point=30 := by
  rw [Nat.card_congr basePointEquivCode]
  calc
    Nat.card BaseCode = Fintype.card BaseCode := Nat.card_eq_fintype_card
    _ = 30 := by decide +kernel

theorem actual_source_trace_zero :
    (29 : ℤ)+1-(Nat.card (model (1 : Base)).Point : ℤ)=0 := by
  rw [actual_base_point_card]
  norm_num

end VerheulDistortionAudit
