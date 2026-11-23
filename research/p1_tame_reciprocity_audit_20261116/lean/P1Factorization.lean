import P1Product

namespace P1TameReciprocityAudit
noncomputable section
open Polynomial
open EllipticSplitNormProjectionAudit
variable {K : Type*} [Field K]

def p1Generators (S : Finset K) : Set (RatFunc K)ˣ :=
  Set.range p1ConstantUnit ∪ baseVerticalUnit '' (S : Set K)

def p1Generated (S : Finset K) : Subgroup (RatFunc K)ˣ := Subgroup.closure (p1Generators S)

theorem p1_constant_mem (S : Finset K) (c : Kˣ) : p1ConstantUnit c∈p1Generated S :=
  Subgroup.subset_closure (Or.inl ⟨c,rfl⟩)

theorem p1_linear_mem (S : Finset K) (a : K) (ha : a∈S) : baseVerticalUnit a∈p1Generated S :=
  Subgroup.subset_closure (Or.inr ⟨a,ha,rfl⟩)

theorem p1_product_on_generators (S : Finset K) (f g : (RatFunc K)ˣ)
    (hf : f∈p1Generators S) (hg : g∈p1Generators S) : p1Product S f g=1 := by
  rcases hf with ⟨c,rfl⟩ | ⟨a,ha,rfl⟩
  · rcases hg with ⟨d,rfl⟩ | ⟨b,hb,rfl⟩
    · exact p1_product_constants S c d
    · exact p1_product_constant_linear S c b hb
  · rcases hg with ⟨d,rfl⟩ | ⟨b,hb,rfl⟩
    · exact p1_product_linear_constant S d a ha
    · exact p1_product_linears S a b ha hb

/-- A conclusion derived from actual generator evaluations and subgroup closure. -/
theorem p1_product_on_generated (S : Finset K) (f g : (RatFunc K)ˣ)
    (hf : f∈p1Generated S) (hg : g∈p1Generated S) : p1Product S f g=1 := by
  have hr (u : (RatFunc K)ˣ) (hu : u∈p1Generators S) :
      p1Generated S ≤ (p1ProductRight S u).ker := by
    apply (Subgroup.closure_le _).mpr
    intro v hv
    exact p1_product_on_generators S u v hu hv
  have hl : p1Generated S ≤ (p1ProductLeft S g).ker := by
    apply (Subgroup.closure_le _).mpr
    intro u hu
    exact hr u hu hg
  exact hl hf

def p1PolynomialUnit (p : K[X]) (hp : p≠0) : (RatFunc K)ˣ :=
  Units.mk0 (algebraMap K[X] (RatFunc K) p) (by
    intro h
    apply hp
    apply IsFractionRing.injective K[X] (RatFunc K)
    simpa only [map_zero] using h)

theorem p1_polynomial_unit_factorization (p : K[X]) (hp : p≠0) (hs : p.Splits) :
    p1PolynomialUnit p hp=
      p1ConstantUnit (Units.mk0 p.leadingCoeff (leadingCoeff_ne_zero.mpr hp))*
        (p.roots.map baseVerticalUnit).prod := by
  have hm (m : Multiset K) :
      ((m.map baseVerticalUnit).prod : RatFunc K)=
        algebraMap K[X] (RatFunc K) (m.map (fun a=>X-C a)).prod := by
    refine Multiset.induction_on m ?_ ?_
    · simp
    · intro a m ih
      simpa only [Multiset.map_cons,Multiset.prod_cons,Units.val_mul,map_mul,baseVerticalUnit,Units.val_mk0] using
        congrArg (fun z : RatFunc K=>algebraMap K[X] (RatFunc K) (X-C a)*z) ih
  apply Units.ext
  change algebraMap K[X] (RatFunc K) p=
    algebraMap K (RatFunc K) p.leadingCoeff*((p.roots.map baseVerticalUnit).prod : RatFunc K)
  rw [hm]
  change algebraMap K[X] (RatFunc K) p=
    algebraMap K[X] (RatFunc K) (C p.leadingCoeff)*
      algebraMap K[X] (RatFunc K) (p.roots.map (fun a=>X-C a)).prod
  rw [←map_mul,←hs.eq_prod_roots]

theorem p1_polynomial_mem_generated (S : Finset K) (p : K[X]) (hp : p≠0)
    (hs : p.Splits) (hroots : ∀ a∈p.roots,a∈S) : p1PolynomialUnit p hp∈p1Generated S := by
  have hm : ∀ m : Multiset K,(∀ a∈m,a∈S)→(m.map baseVerticalUnit).prod∈p1Generated S := by
    intro m
    refine Multiset.induction_on m ?_ ?_
    · intro _
      simp
    · intro a m ih hS
      rw [Multiset.map_cons,Multiset.prod_cons]
      exact (p1Generated S).mul_mem (p1_linear_mem S a (hS a (by simp)))
        (ih (fun b hb=>hS b (by simp [hb])))
  rw [p1_polynomial_unit_factorization p hp hs]
  exact (p1Generated S).mul_mem (p1_constant_mem S _) (hm p.roots hroots)

theorem p1_split_fraction_mem_generated (S : Finset K) (f : (RatFunc K)ˣ)
    (hn : (f : RatFunc K).num.Splits) (hd : (f : RatFunc K).denom.Splits)
    (hns : ∀ a∈(f : RatFunc K).num.roots,a∈S)
    (hds : ∀ a∈(f : RatFunc K).denom.roots,a∈S) : f∈p1Generated S := by
  have he : f=p1PolynomialUnit (f : RatFunc K).num (RatFunc.num_ne_zero f.ne_zero)/
      p1PolynomialUnit (f : RatFunc K).denom (RatFunc.denom_ne_zero _) := by
    apply Units.ext
    rw [Units.val_div_eq_div_val]
    exact (RatFunc.num_div_denom (f : RatFunc K)).symm
  rw [he]
  exact (p1Generated S).div_mem
    (p1_polynomial_mem_generated S _ _ hn hns) (p1_polynomial_mem_generated S _ _ hd hds)

/-- Reciprocity for actual split rational functions, with an explicit root cover. -/
theorem p1_split_reciprocity (S : Finset K) (f g : (RatFunc K)ˣ)
    (hfn : (f : RatFunc K).num.Splits) (hfd : (f : RatFunc K).denom.Splits)
    (hgn : (g : RatFunc K).num.Splits) (hgd : (g : RatFunc K).denom.Splits)
    (hfns : ∀ a∈(f : RatFunc K).num.roots,a∈S)
    (hfds : ∀ a∈(f : RatFunc K).denom.roots,a∈S)
    (hgns : ∀ a∈(g : RatFunc K).num.roots,a∈S)
    (hgds : ∀ a∈(g : RatFunc K).denom.roots,a∈S) : p1Product S f g=1 :=
  p1_product_on_generated S f g (p1_split_fraction_mem_generated S f hfn hfd hfns hfds)
    (p1_split_fraction_mem_generated S g hgn hgd hgns hgds)

#print axioms p1_product_on_generated
#print axioms p1_polynomial_unit_factorization
#print axioms p1_polynomial_mem_generated
#print axioms p1_split_fraction_mem_generated
#print axioms p1_split_reciprocity
end
end P1TameReciprocityAudit
