import P1Controls

namespace P1TameReciprocityAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit
open EllipticInfinityNormProjectionAudit EllipticFinitePlaceProjectionAudit
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

/-- Actual rational-function reciprocity with proper residues and deletion controls. -/
theorem actual_p1_tame_reciprocity_certificate (K : Type*) [Field K] :
    (∀ a : K,(auxiliaryModel a).Nonsingular a 0 ∧
      (Point.some a 0 (auxiliary_nonsingular a) : (auxiliaryModel a).Point)≠
        -Point.some a 0 (auxiliary_nonsingular a) ∧
      finiteBaseValuation a (baseVerticalUnit a : RatFunc K)=WithZero.exp (-1 : ℤ) ∧
      Function.Surjective (finiteBaseValuation a)) ∧
    (∀ a : K,RingHom.ker (p1FiniteResidue a)=IsLocalRing.maximalIdeal (FiniteBaseRing a) ∧
      Function.Surjective (p1FiniteResidue a) ∧
      (∀ c : K,p1FiniteResidue a (finiteBaseConstant a c)=c) ∧
      (∀ p : Polynomial K,p1FiniteResidue a (p1FinitePolynomial a p)=p.eval a)) ∧
    (∀ W : WeierstrassCurve.Affine K,∀ a b : K,∀ h : W.Nonsingular a b,
      (∀ r : FiniteBaseRing a,p1FiniteResidue a r=finitePlaceEvaluation W h r) ∧
      (∀ u : (FiniteBaseRing a).unitGroup,p1FiniteUnitResidue a u=finitePlaceUnitEvaluation W h u) ∧
      (∀ f g : (RatFunc K)ˣ,p1FiniteTame a f g=finitePlaceTameSymbol W h f g)) ∧
    (∀ W : WeierstrassCurve.Affine K,∀ f g : (RatFunc K)ˣ,p1InfinityTame f g=baseTameSymbol W f g) ∧
    (∀ f g : (RatFunc K)ˣ,Function.HasFiniteMulSupport (fun a=>p1FiniteTame a f g) ∧
      p1GlobalProduct f g=p1Product (p1RootCover f g) f g) ∧
    (∀ S : Finset K,∀ f g : (RatFunc K)ˣ,
      (f : RatFunc K).num.Splits→(f : RatFunc K).denom.Splits→
      (g : RatFunc K).num.Splits→(g : RatFunc K).denom.Splits→
      (∀ a∈(f : RatFunc K).num.roots,a∈S)→
      (∀ a∈(f : RatFunc K).denom.roots,a∈S)→
      (∀ a∈(g : RatFunc K).num.roots,a∈S)→
      (∀ a∈(g : RatFunc K).denom.roots,a∈S)→p1Product S f g=1) ∧
    (∀ f g : (RatFunc K)ˣ,
      (f : RatFunc K).num.Splits→(f : RatFunc K).denom.Splits→
      (g : RatFunc K).num.Splits→(g : RatFunc K).denom.Splits→p1GlobalProduct f g=1) ∧
    (∀ _hK : IsAlgClosed K,∀ f g : (RatFunc K)ˣ,p1GlobalProduct f g=1 ∧
      ((∏ᶠ a,(p1FiniteTame a f g)⁻¹)*(p1InfinityTame f g)⁻¹)=1) ∧
    ((p1FiniteTame (0 : P1F5) (p1ConstantUnit p1Two) (baseVerticalUnit 0) : P1F5)=2 ∧
      (p1InfinityTame (p1ConstantUnit p1Two) (baseVerticalUnit 0) : P1F5)=3 ∧
      p1Product {0} (p1ConstantUnit p1Two) (baseVerticalUnit 0)=1 ∧
      (∏ a ∈ ({0} : Finset P1F5),p1FiniteTame a (p1ConstantUnit p1Two) (baseVerticalUnit 0))≠1) ∧
    ((p1UnsignedProduct {0,1} (baseVerticalUnit (0 : P1F5)) (baseVerticalUnit 1) : P1F5)=4 ∧
      p1UnsignedProduct {0,1} (baseVerticalUnit (0 : P1F5)) (baseVerticalUnit 1)≠1 ∧
      p1Product {0,1} (baseVerticalUnit (0 : P1F5)) (baseVerticalUnit 1)=1) ∧
    ((p1InfinityTame (baseVerticalUnit (0 : P1F2)) (baseVerticalUnit 1) : P1F2)=1 ∧
      p1Product {0,1} (baseVerticalUnit (0 : P1F2)) (baseVerticalUnit 1)=1 ∧
      (p1InfinityTame (baseVerticalUnit (0 : P1F3)) (baseVerticalUnit 1) : P1F3)=2 ∧
      p1Product {0,1} (baseVerticalUnit (0 : P1F3)) (baseVerticalUnit 1)=1) := by
  refine ⟨?_,?_,?_,p1_infinity_tame_realized,?_,p1_split_reciprocity,
    p1_reciprocity_of_split,?_,actual_f5_infinity_omission,actual_f5_unsigned_counter,
    actual_binary_ternary_positive⟩
  · intro a
    exact ⟨auxiliary_nonsingular a,auxiliary_split a,finite_base_parameter_value a,
      finite_base_valuation_surjective a⟩
  · intro a
    exact ⟨p1_finite_residue_kernel a,p1_finite_residue_surjective a,
      p1_finite_residue_constant a,p1_finite_eval_polynomial a⟩
  · intro W a b h
    exact ⟨p1_finite_residue_realized W h,p1_finite_unit_realized W h,p1_finite_tame_realized W h⟩
  · intro f g
    exact ⟨p1_tame_support_finite f g,p1_global_eq_root_cover f g⟩
  · intro hK f g
    let _ : IsAlgClosed K:=hK
    exact ⟨p1_reciprocity f g,p1_inverse_reciprocity f g⟩

#print axioms actual_p1_tame_reciprocity_certificate
end
end P1TameReciprocityAudit
