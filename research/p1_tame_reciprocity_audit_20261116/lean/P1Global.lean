import P1Factorization

namespace P1TameReciprocityAudit
noncomputable section
open Polynomial Function
open EllipticRamifiedNormProjectionAudit
variable {K : Type*} [Field K]

def p1RootCover (f g : (RatFunc K)ˣ) : Finset K := by
  classical
  exact ((f : RatFunc K).num.roots.toFinset ∪ (f : RatFunc K).denom.roots.toFinset) ∪
    ((g : RatFunc K).num.roots.toFinset ∪ (g : RatFunc K).denom.roots.toFinset)

theorem p1_root_cover_contains (f g : (RatFunc K)ˣ) :
    (∀ a∈(f : RatFunc K).num.roots,a∈p1RootCover f g) ∧
    (∀ a∈(f : RatFunc K).denom.roots,a∈p1RootCover f g) ∧
    (∀ a∈(g : RatFunc K).num.roots,a∈p1RootCover f g) ∧
    (∀ a∈(g : RatFunc K).denom.roots,a∈p1RootCover f g) := by
  classical
  refine ⟨?_,?_,?_,?_⟩
  · intro a ha
    simp only [p1RootCover,Finset.mem_union,Multiset.mem_toFinset]
    exact Or.inl (Or.inl ha)
  · intro a ha
    simp only [p1RootCover,Finset.mem_union,Multiset.mem_toFinset]
    exact Or.inl (Or.inr ha)
  · intro a ha
    simp only [p1RootCover,Finset.mem_union,Multiset.mem_toFinset]
    exact Or.inr (Or.inl ha)
  · intro a ha
    simp only [p1RootCover,Finset.mem_union,Multiset.mem_toFinset]
    exact Or.inr (Or.inr ha)

/-- Actual rational-function order is zero outside its numerator/denominator roots. -/
theorem p1_order_outside (S : Finset K) (f : (RatFunc K)ˣ)
    (hn : ∀ a∈(f : RatFunc K).num.roots,a∈S)
    (hd : ∀ a∈(f : RatFunc K).denom.roots,a∈S) (a : K) (ha : a∉S) :
    finiteBaseOrder a (f : RatFunc K)=0 := by
  have hnv : (f : RatFunc K).num.eval a≠0 := by
    intro hz
    exact ha (hn a ((mem_roots (RatFunc.num_ne_zero f.ne_zero)).mpr hz))
  have hdv : (f : RatFunc K).denom.eval a≠0 := by
    intro hz
    exact ha (hd a ((mem_roots (RatFunc.denom_ne_zero _)).mpr hz))
  rw [finiteBaseOrder,←RatFunc.num_div_denom (f : RatFunc K),map_div₀,
    finite_base_polynomial_unit a _ hnv,finite_base_polynomial_unit a _ hdv,
    div_self one_ne_zero,WithZero.log_one,neg_zero]

theorem p1_tame_outside (f g : (RatFunc K)ˣ) (a : K) (ha : a∉p1RootCover f g) :
    p1FiniteTame a f g=1 := by
  have h:=p1_root_cover_contains f g
  exact p1_finite_units a f g
    (p1_order_outside _ f h.1 h.2.1 a ha) (p1_order_outside _ g h.2.2.1 h.2.2.2 a ha)

/-- Honest finite support over any field, before invoking finprod. -/
theorem p1_tame_support_finite (f g : (RatFunc K)ˣ) :
    HasFiniteMulSupport (fun a=>p1FiniteTame a f g) := by
  apply (p1RootCover f g).finite_toSet.subset
  intro a ha
  by_contra hn
  exact ha (p1_tame_outside f g a hn)

def p1GlobalProduct (f g : (RatFunc K)ˣ) : Kˣ :=
  (∏ᶠ a,p1FiniteTame a f g)*p1InfinityTame f g

theorem p1_global_eq_root_cover (f g : (RatFunc K)ˣ) :
    p1GlobalProduct f g=p1Product (p1RootCover f g) f g := by
  have hs : mulSupport (fun a=>p1FiniteTame a f g) ⊆ (p1RootCover f g : Set K) := by
    intro a ha
    by_contra hn
    exact ha (p1_tame_outside f g a hn)
  unfold p1GlobalProduct p1Product
  rw [finprod_eq_prod_of_mulSupport_subset _ hs]

theorem p1_reciprocity_of_split (f g : (RatFunc K)ˣ)
    (hfn : (f : RatFunc K).num.Splits) (hfd : (f : RatFunc K).denom.Splits)
    (hgn : (g : RatFunc K).num.Splits) (hgd : (g : RatFunc K).denom.Splits) :
    p1GlobalProduct f g=1 := by
  rw [p1_global_eq_root_cover]
  have h:=p1_root_cover_contains f g
  exact p1_split_reciprocity _ f g hfn hfd hgn hgd h.1 h.2.1 h.2.2.1 h.2.2.2

/-- Actual rational-place reciprocity over an algebraically closed coefficient field. -/
theorem p1_reciprocity [IsAlgClosed K] (f g : (RatFunc K)ˣ) : p1GlobalProduct f g=1 :=
  p1_reciprocity_of_split f g (IsAlgClosed.splits _) (IsAlgClosed.splits _)
    (IsAlgClosed.splits _) (IsAlgClosed.splits _)

theorem p1_inverse_reciprocity [IsAlgClosed K] (f g : (RatFunc K)ˣ) :
    ((∏ᶠ a,(p1FiniteTame a f g)⁻¹)*(p1InfinityTame f g)⁻¹)=1 := by
  rw [finprod_inv_distrib,←mul_inv_rev,mul_comm (p1InfinityTame f g)]
  exact congrArg Inv.inv (p1_reciprocity f g)

#print axioms p1_order_outside
#print axioms p1_tame_outside
#print axioms p1_tame_support_finite
#print axioms p1_global_eq_root_cover
#print axioms p1_reciprocity_of_split
#print axioms p1_reciprocity
#print axioms p1_inverse_reciprocity
end
end P1TameReciprocityAudit
