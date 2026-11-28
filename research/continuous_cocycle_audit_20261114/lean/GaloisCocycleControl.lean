import CompactCocycles
import GaloisImports
import ActualDualityControls

namespace ContinuousCocycleAudit
noncomputable section
open FiniteTateDualityAudit
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
abbrev ActualGalois := Field.absoluteGaloisGroup (ZMod 2)
abbrev IntegerCocycle := ActualGalois →ₜ* Multiplicative ℤ
abbrev IntegerCoinvariants := Coinvariants (MulEquiv.refl (Multiplicative ℤ))

instance galoisCompact : CompactSpace ActualGalois := by
  change CompactSpace (AlgebraicClosure (ZMod 2) ≃ₐ[ZMod 2] AlgebraicClosure (ZMod 2))
  infer_instance

def actualFrobenius : ActualGalois := {
  __ := frobeniusEquiv (AlgebraicClosure (ZMod 2)) 2
  commutes' := by
    intro x
    change (algebraMap (ZMod 2) (AlgebraicClosure (ZMod 2)) x)^2=
      algebraMap (ZMod 2) (AlgebraicClosure (ZMod 2)) x
    rw [←map_pow]
    congr 1
    simpa only [ZMod.card] using FiniteField.pow_card x }

def galoisAlgEquiv (σ : ActualGalois) :
    AlgebraicClosure (ZMod 2) ≃ₐ[ZMod 2] AlgebraicClosure (ZMod 2) := σ

theorem actual_frobenius_square (x : AlgebraicClosure (ZMod 2)) :
    galoisAlgEquiv actualFrobenius x=x^2 := rfl

/-- Continuous 1-cocycles for trivial integer action are actual continuous
homomorphisms into the multiplicatively written discrete integers. -/
theorem integer_cocycle_trivial (c : IntegerCocycle) : c=1 :=
  compact_continuous_hom_trivial c

def trivialIntegerAction (_ : ActualGalois) : ℤ ≃+ ℤ := AddEquiv.refl ℤ

theorem trivial_integer_action_one : trivialIntegerAction 1=AddEquiv.refl ℤ := rfl

theorem trivial_integer_action_mul (σ τ : ActualGalois) :
    trivialIntegerAction (σ*τ)=(trivialIntegerAction τ).trans (trivialIntegerAction σ) := rfl

theorem trivial_integer_action_continuous :
    Continuous (fun p : ActualGalois × ℤ => trivialIntegerAction p.1 p.2) := continuous_snd

/-- No continuous cocycles are lost by bundling them as continuous homs. -/
theorem every_continuous_integer_cocycle_zero (c : C(ActualGalois,ℤ))
    (hc : ∀ σ τ,c (σ*τ)=c σ+trivialIntegerAction σ (c τ)) : ∀ σ,c σ=0 := by
  have hz : c 1=0 := by
    have h := hc 1 1
    change c 1=c 1+c 1 at h
    omega
  let φ : IntegerCocycle := {
    toFun := fun σ => Multiplicative.ofAdd (c σ)
    map_one' := hz
    map_mul' := by intro σ τ; exact hc σ τ
    continuous_toFun := c.continuous }
  intro σ
  exact congrArg Multiplicative.toAdd (compact_continuous_hom_value φ σ)

def missingIntegerClass : IntegerCoinvariants :=
  QuotientGroup.mk (Multiplicative.ofAdd (1:ℤ))

theorem missing_integer_class_ne_one : missingIntegerClass≠1 := by
  intro h
  have hm := (QuotientGroup.eq_one_iff (Multiplicative.ofAdd (1:ℤ))).mp h
  obtain ⟨z,hz⟩ := hm
  change z/z=Multiplicative.ofAdd (1:ℤ) at hz
  have ht : (1:Multiplicative ℤ)=Multiplicative.ofAdd (1:ℤ) := by
    simpa only [div_self'] using hz
  exact (by decide : (1:Multiplicative ℤ)≠Multiplicative.ofAdd (1:ℤ)) ht

def evaluateCocycle (σ : ActualGalois) (c : IntegerCocycle) : IntegerCoinvariants :=
  QuotientGroup.mk (c σ)

theorem evaluate_cocycle_one (σ : ActualGalois) (c : IntegerCocycle) :
    evaluateCocycle σ c=1 := by
  rw [integer_cocycle_trivial c]
  rfl

/-- At every actual Galois element, not merely at a nongenerating selected
one, the literal evaluation formula fails surjectivity for trivial Z. -/
theorem evaluation_not_surjective (σ : ActualGalois) :
    ¬Function.Surjective (evaluateCocycle σ) := by
  intro h
  obtain ⟨c,hc⟩ := h missingIntegerClass
  exact missing_integer_class_ne_one (hc.symm.trans (evaluate_cocycle_one σ c))

/-- Any cohomology-class quotient has a surjective representative projection.
No such quotient can carry the claimed evaluation-induced isomorphism. -/
theorem no_evaluation_quotient_equiv (σ : ActualGalois) :
    ¬∃ (H : Type) (π : IntegerCocycle → H) (e : H ≃ IntegerCoinvariants),
      Function.Surjective π ∧ ∀ c,e (π c)=evaluateCocycle σ c := by
  rintro ⟨H,π,e,hπ,he⟩
  apply evaluation_not_surjective σ
  intro q
  obtain ⟨h,hh⟩ := e.surjective q
  obtain ⟨c,hc⟩ := hπ h
  exact ⟨c,(he c).symm.trans (by rw [hc,hh])⟩

#print axioms integer_cocycle_trivial
#print axioms missing_integer_class_ne_one
#print axioms evaluation_not_surjective
#print axioms no_evaluation_quotient_equiv
end
end ContinuousCocycleAudit
