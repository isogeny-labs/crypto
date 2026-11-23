module
public import CocycleClasses
public import Mathlib.Topology.Instances.Int

@[expose] public section
namespace ProcyclicGaloisCocycleAudit
noncomputable section
variable (K : Type*) [Field K] [Fintype K]

abbrev IntegerTarget := Multiplicative ℤ
local instance : TopologicalSpace (MulAut IntegerTarget) := ⊥
local instance : DiscreteTopology (MulAut IntegerTarget) := ⟨rfl⟩

def integerAction : Gamma K →ₜ* MulAut IntegerTarget := 1

abbrev IntegerClasses := CocycleClasses K IntegerTarget (integerAction K)
abbrev IntegerCoinvariants := Coinvariants K IntegerTarget (integerAction K)
def integerEvaluation : IntegerClasses K →* IntegerCoinvariants K :=
  classEvaluationHom K IntegerTarget (integerAction K)

/-- The compact-image proof is credited to the earlier compact-cocycle packet,
now instantiated for every actual finite-field absolute Galois group. -/
theorem integer_hom_value (f : Gamma K →ₜ* IntegerTarget) (σ : Gamma K) : f σ = 1 := by
  have hr : (Set.range f).Finite := (isCompact_range f.continuous).finite_of_discrete
  let : Finite f.toMonoidHom.range := hr.to_subtype
  have ht := isOfFinOrder_of_finite (⟨f σ, ⟨σ, rfl⟩⟩ : f.toMonoidHom.range)
  apply IsOfFinOrder.eq_one'
  rw [isOfFinOrder_iff_pow_eq_one] at ht ⊢
  obtain ⟨n, hn, he⟩ := ht
  exact ⟨n, hn, congrArg Subtype.val he⟩

theorem integer_cocycle_value (c : Cocycles K IntegerTarget (integerAction K))
    (σ : Gamma K) : c.val σ = 1 := by
  let f : Gamma K →ₜ* IntegerTarget := {
    toFun := c.val
    map_one' := cocycle_at_one K IntegerTarget (integerAction K) c
    map_mul' := by
      intro σ τ
      simpa [integerAction] using cocycle_identity K IntegerTarget (integerAction K) c σ τ
    continuous_toFun := c.val.continuous }
  exact integer_hom_value K f σ

def missingIntegerClass : Coinvariants K IntegerTarget (integerAction K) :=
  QuotientGroup.mk (Multiplicative.ofAdd (1 : ℤ))

theorem missing_integer_class_ne_one : missingIntegerClass K ≠ 1 := by
  intro h
  have hm := (QuotientGroup.eq_one_iff (Multiplicative.ofAdd (1 : ℤ))).mp h
  obtain ⟨m, he⟩ := hm
  change m / m = Multiplicative.ofAdd (1 : ℤ) at he
  have hh : (1 : IntegerTarget) = Multiplicative.ofAdd (1 : ℤ) := by simpa using he
  exact (by decide : (1 : IntegerTarget) ≠ Multiplicative.ofAdd (1 : ℤ)) hh

theorem integer_class_evaluation_one
    (c : CocycleClasses K IntegerTarget (integerAction K)) :
    classEvaluationHom K IntegerTarget (integerAction K) c = 1 := by
  induction c using QuotientGroup.induction_on with
  | H c =>
    change QuotientGroup.mk (c.val (frobenius K)) = 1
    rw [integer_cocycle_value]
    rfl

theorem finite_guard_needed :
    ¬ Function.Surjective (classEvaluationHom K IntegerTarget (integerAction K)) := by
  intro hs
  obtain ⟨c, hc⟩ := hs (missingIntegerClass K)
  exact missing_integer_class_ne_one K (hc.symm.trans (integer_class_evaluation_one K c))

#print axioms integer_hom_value
#print axioms integer_cocycle_value
#print axioms finite_guard_needed
end
end ProcyclicGaloisCocycleAudit
