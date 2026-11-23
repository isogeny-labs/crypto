import P1InfinityGenerators

namespace P1TameReciprocityAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticSplitNormProjectionAudit
variable {K : Type*} [Field K]

def p1Product (S : Finset K) (f g : (RatFunc K)ˣ) : Kˣ :=
  (∏ a ∈ S,p1FiniteTame a f g)*p1InfinityTame f g

theorem p1_product_mul_left (S : Finset K) (f g h : (RatFunc K)ˣ) :
    p1Product S (f*g) h=p1Product S f h*p1Product S g h := by
  simp only [p1Product,p1_finite_mul_left,p1_infinity_mul_left,Finset.prod_mul_distrib]
  exact mul_mul_mul_comm _ _ _ _

theorem p1_product_skew (S : Finset K) (f g : (RatFunc K)ˣ) :
    p1Product S g f=(p1Product S f g)⁻¹ := by
  unfold p1Product
  have hp : (∏ a ∈ S,p1FiniteTame a g f)=(∏ a ∈ S,p1FiniteTame a f g)⁻¹ := by
    rw [←Finset.prod_inv_distrib]
    exact Finset.prod_congr rfl (fun a _=>p1_finite_skew a f g)
  rw [hp,p1_infinity_skew,mul_inv_rev]
  exact mul_comm _ _

theorem p1_product_mul_right (S : Finset K) (f g h : (RatFunc K)ˣ) :
    p1Product S f (g*h)=p1Product S f g*p1Product S f h := by
  rw [p1_product_skew,p1_product_mul_left,mul_inv_rev,←p1_product_skew,←p1_product_skew,mul_comm]

def p1ProductLeft (S : Finset K) (g : (RatFunc K)ˣ) : (RatFunc K)ˣ →* Kˣ where
  toFun f:=p1Product S f g
  map_one':=by
    have h:=p1_product_mul_left S 1 1 g
    simpa only [one_mul,mul_eq_left] using h.symm
  map_mul':=fun f h=>p1_product_mul_left S f h g

def p1ProductRight (S : Finset K) (f : (RatFunc K)ˣ) : (RatFunc K)ˣ →* Kˣ where
  toFun g:=p1Product S f g
  map_one':=by
    have h:=p1_product_mul_right S f 1 1
    simpa only [one_mul,mul_eq_left] using h.symm
  map_mul':=fun g h=>p1_product_mul_right S f g h

theorem p1_product_constants (S : Finset K) (c d : Kˣ) :
    p1Product S (p1ConstantUnit c) (p1ConstantUnit d)=1 := by
  simp only [p1Product,p1_finite_constants,p1_infinity_constants,Finset.prod_const_one,mul_one]

theorem p1_product_constant_linear (S : Finset K) (c : Kˣ) (a : K) (ha : a∈S) :
    p1Product S (p1ConstantUnit c) (baseVerticalUnit a)=1 := by
  have hp : (∏ z ∈ S,p1FiniteTame z (p1ConstantUnit c) (baseVerticalUnit a))=c := by
    rw [Finset.prod_eq_single_of_mem a ha (fun z _ hz=>p1_finite_constant_linear_other z a hz c),
      p1_finite_constant_linear_self]
  rw [p1Product,hp,p1_infinity_constant_linear,mul_inv_cancel]

theorem p1_product_linear_constant (S : Finset K) (c : Kˣ) (a : K) (ha : a∈S) :
    p1Product S (baseVerticalUnit a) (p1ConstantUnit c)=1 := by
  rw [p1_product_skew,p1_product_constant_linear S c a ha,inv_one]

theorem p1_product_linears (S : Finset K) (a b : K) (ha : a∈S) (hb : b∈S) :
    p1Product S (baseVerticalUnit a) (baseVerticalUnit b)=1 := by
  classical
  by_cases h : a=b
  · subst b
    have hp : (∏ z ∈ S,p1FiniteTame z (baseVerticalUnit a) (baseVerticalUnit a))= -1 := by
      rw [Finset.prod_eq_single_of_mem a ha
        (fun z _ hz=>p1_finite_linear_pair_other a a z hz hz),p1_finite_linear_diagonal_self]
    rw [p1Product,hp,p1_infinity_linear_pair,neg_one_mul,neg_neg]
  · have hp : (∏ z ∈ S,p1FiniteTame z (baseVerticalUnit a) (baseVerticalUnit b))=
        p1FiniteTame a (baseVerticalUnit a) (baseVerticalUnit b)*
          p1FiniteTame b (baseVerticalUnit a) (baseVerticalUnit b) := by
      rw [←Finset.prod_pair (f:=fun z=>p1FiniteTame z (baseVerticalUnit a) (baseVerticalUnit b)) h]
      apply (Finset.prod_subset ?_ ?_).symm
      · intro z hz
        simp only [Finset.mem_insert,Finset.mem_singleton] at hz
        rcases hz with rfl | rfl
        · exact ha
        · exact hb
      · intro z _ hz
        simp only [Finset.mem_insert,Finset.mem_singleton,not_or] at hz
        exact p1_finite_linear_pair_other a b z hz.1 hz.2
    rw [p1Product,hp,p1_infinity_linear_pair]
    apply Units.ext
    change (p1FiniteTame a (baseVerticalUnit a) (baseVerticalUnit b) : K)*
      (p1FiniteTame b (baseVerticalUnit a) (baseVerticalUnit b) : K)*(-1)=1
    rw [p1_finite_linear_pair_left a b h,p1_finite_linear_pair_right a b h]
    have hn : a-b≠0:=sub_ne_zero.mpr h
    field_simp
    ring

#print axioms p1_product_mul_left
#print axioms p1_product_skew
#print axioms p1_product_mul_right
#print axioms p1_product_constants
#print axioms p1_product_constant_linear
#print axioms p1_product_linear_constant
#print axioms p1_product_linears
end
end P1TameReciprocityAudit
