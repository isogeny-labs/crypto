import PowerTargets
import ImperfectControl

namespace CharPPairingDomainAudit
noncomputable section
open FiniteCharTwoTraceAudit
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩

/-- The nonzero transcendental variable is an actual field unit. -/
def variableUnit : ImperfectFieldˣ := Units.mk0 RatFunc.X RatFunc.X_ne_zero

def variableClass : PowerTarget ImperfectField 2 := QuotientGroup.mk variableUnit

theorem variable_class_nontrivial : variableClass≠1 := by
  intro h
  have hm := (QuotientGroup.eq_one_iff variableUnit).mp h
  obtain ⟨u,hu⟩ := hm
  apply variable_not_square
  refine ⟨(u:ImperfectField),?_⟩
  change u^2=variableUnit at hu
  exact congrArg Units.val hu

theorem imperfect_target_card_not_one : Nat.card (PowerTarget ImperfectField 2)≠1 := by
  intro h
  obtain ⟨hs,_⟩ := Nat.card_eq_one_iff_unique.mp h
  exact variable_class_nontrivial (hs.elim _ _)

theorem imperfect_unit_square_not_surjective :
    ¬Function.Surjective (powMonoidHom 2 : ImperfectFieldˣ →* ImperfectFieldˣ) := by
  intro h
  exact variable_class_nontrivial ((QuotientGroup.eq_one_iff variableUnit).mpr (h variableUnit))

theorem imperfect_unit_root_still_trivial (u : ImperfectFieldˣ) (hu : u^2=1) : u=1 := by
  exact unit_prime_power_root 2 1 u (by simpa only [pow_one] using hu)

/-- Finiteness/perfectness cannot be dropped from the trivial-target claim. -/
theorem deleting_finiteness_target_false :
    ¬∀ (F : Type) [Field F] (p : ℕ) [Fact p.Prime] [CharP F p],
      Nat.card (PowerTarget F p)=1 := by
  intro h
  exact imperfect_target_card_not_one (h ImperfectField 2)

#print axioms variable_class_nontrivial
#print axioms imperfect_target_card_not_one
#print axioms imperfect_unit_root_still_trivial
#print axioms deleting_finiteness_target_false
end
end CharPPairingDomainAudit
