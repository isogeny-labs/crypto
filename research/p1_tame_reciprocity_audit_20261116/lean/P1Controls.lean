import P1Global

namespace P1TameReciprocityAudit
noncomputable section
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit
open EllipticInfinityNormProjectionAudit EllipticFinitePlaceProjectionAudit
variable {K : Type*} [Field K]

def p1UnsignedFinite (a : K) (f g : (RatFunc K)ˣ) : Kˣ :=
  p1FiniteUnitResidue a (finiteBaseBalancedLocalUnit a f g)

def p1UnsignedInfinity (f g : (RatFunc K)ˣ) : Kˣ :=
  p1InfinityUnitResidue (baseBalancedLocalUnit f g)

def p1UnsignedProduct (S : Finset K) (f g : (RatFunc K)ˣ) : Kˣ :=
  (∏ a ∈ S,p1UnsignedFinite a f g)*p1UnsignedInfinity f g

theorem p1_unsigned_infinity_linears (a b : K) :
    p1UnsignedInfinity (baseVerticalUnit a) (baseVerticalUnit b)=1 := by
  rw [p1UnsignedInfinity,p1_infinity_linear_balanced,p1_infinity_linear_ratio_value]

theorem p1_unsigned_finite_pair_left (a b : K) (h : a≠b) :
    (p1UnsignedFinite a (baseVerticalUnit a) (baseVerticalUnit b) : K)=(a-b)⁻¹ := by
  have he : p1FiniteTame a (baseVerticalUnit a) (baseVerticalUnit b)=
      p1UnsignedFinite a (baseVerticalUnit a) (baseVerticalUnit b) := by
    rw [p1FiniteTame,finitePlaceTameSymbol,p1_finite_linear_order_other a b h,
      mul_zero,zpow_zero,one_mul]
    rfl
  rw [←he,p1_finite_linear_pair_left a b h]

theorem p1_unsigned_finite_pair_right (a b : K) (h : a≠b) :
    (p1UnsignedFinite b (baseVerticalUnit a) (baseVerticalUnit b) : K)=b-a := by
  have he : p1FiniteTame b (baseVerticalUnit a) (baseVerticalUnit b)=
      p1UnsignedFinite b (baseVerticalUnit a) (baseVerticalUnit b) := by
    rw [p1FiniteTame,finitePlaceTameSymbol,p1_finite_linear_order_other b a h.symm,
      zero_mul,zpow_zero,one_mul]
    rfl
  rw [←he,p1_finite_linear_pair_right a b h]

private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
abbrev P1F5:=ZMod 5
abbrev P1F2:=ZMod 2
abbrev P1F3:=ZMod 3
def p1Two : P1F5ˣ := Units.mk0 2 (by decide +kernel)

theorem actual_f5_infinity_omission :
    (p1FiniteTame (0 : P1F5) (p1ConstantUnit p1Two) (baseVerticalUnit 0) : P1F5)=2 ∧
    (p1InfinityTame (p1ConstantUnit p1Two) (baseVerticalUnit 0) : P1F5)=3 ∧
    p1Product {0} (p1ConstantUnit p1Two) (baseVerticalUnit 0)=1 ∧
    (∏ a ∈ ({0} : Finset P1F5),p1FiniteTame a (p1ConstantUnit p1Two) (baseVerticalUnit 0))≠1 := by
  refine ⟨?_,?_,?_,?_⟩
  · rw [p1_finite_constant_linear_self]
    rfl
  · rw [p1_infinity_constant_linear,Units.val_inv_eq_inv_val]
    change (2 : P1F5)⁻¹=3
    decide +kernel
  · exact p1_product_constant_linear _ p1Two 0 (by simp)
  · rw [Finset.prod_singleton,p1_finite_constant_linear_self]
    intro h
    have he:=congrArg (fun u : P1F5ˣ=>(u : P1F5)) h
    change (2 : P1F5)=1 at he
    exact (by decide +kernel : (2 : P1F5)≠1) he

theorem actual_f5_unsigned_counter :
    (p1UnsignedProduct {0,1} (baseVerticalUnit (0 : P1F5)) (baseVerticalUnit 1) : P1F5)=4 ∧
    p1UnsignedProduct {0,1} (baseVerticalUnit (0 : P1F5)) (baseVerticalUnit 1)≠1 ∧
    p1Product {0,1} (baseVerticalUnit (0 : P1F5)) (baseVerticalUnit 1)=1 := by
  have hn : (0 : P1F5)≠1:=by decide +kernel
  have hv : (p1UnsignedProduct {0,1} (baseVerticalUnit (0 : P1F5)) (baseVerticalUnit 1) : P1F5)=4 := by
    rw [p1UnsignedProduct,Finset.prod_pair hn,p1_unsigned_infinity_linears,mul_one]
    change (p1UnsignedFinite (0 : P1F5) (baseVerticalUnit 0) (baseVerticalUnit 1) : P1F5)*
      (p1UnsignedFinite (1 : P1F5) (baseVerticalUnit 0) (baseVerticalUnit 1) : P1F5)=4
    rw [p1_unsigned_finite_pair_left 0 1 hn,p1_unsigned_finite_pair_right 0 1 hn]
    decide +kernel
  refine ⟨hv,?_,p1_product_linears _ 0 1 (by simp) (by simp)⟩
  intro h
  have he:=congrArg (fun u : P1F5ˣ=>(u : P1F5)) h
  rw [hv] at he
  exact (by decide +kernel : (4 : P1F5)≠1) he

theorem actual_binary_ternary_positive :
    (p1InfinityTame (baseVerticalUnit (0 : P1F2)) (baseVerticalUnit 1) : P1F2)=1 ∧
    p1Product {0,1} (baseVerticalUnit (0 : P1F2)) (baseVerticalUnit 1)=1 ∧
    (p1InfinityTame (baseVerticalUnit (0 : P1F3)) (baseVerticalUnit 1) : P1F3)=2 ∧
    p1Product {0,1} (baseVerticalUnit (0 : P1F3)) (baseVerticalUnit 1)=1 := by
  refine ⟨?_,p1_product_linears _ 0 1 (by simp) (by simp),?_,p1_product_linears _ 0 1 (by simp) (by simp)⟩
  · rw [p1_infinity_linear_pair]
    change (-1 : P1F2)=1
    decide +kernel
  · rw [p1_infinity_linear_pair]
    change (-1 : P1F3)=2
    decide +kernel

#print axioms p1_unsigned_infinity_linears
#print axioms p1_unsigned_finite_pair_left
#print axioms p1_unsigned_finite_pair_right
#print axioms actual_f5_infinity_omission
#print axioms actual_f5_unsigned_counter
#print axioms actual_binary_ternary_positive
end
end P1TameReciprocityAudit
