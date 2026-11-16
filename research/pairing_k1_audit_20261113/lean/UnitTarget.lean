import TateValues

namespace PairingKOneAudit

abbrev BaseUnits := Fˣ
def baseCubes : Subgroup BaseUnits := (powMonoidHom 3 : BaseUnits →* BaseUnits).range
abbrev UnitTarget := BaseUnits ⧸ baseCubes
def cubeRoots : Subgroup BaseUnits := (powMonoidHom 3 : BaseUnits →* BaseUnits).ker

instance rootMembershipDecidable (u : BaseUnits) : Decidable (u∈cubeRoots) :=
  inferInstanceAs (Decidable (u^3=1))

theorem final_exponent_root : ∀ u : BaseUnits,(u^2)^3=1 := by decide +kernel

theorem final_exponent_kernel_table : ∀ u : BaseUnits,
    u^2=1 ↔ ∃ v : BaseUnits,v^3=u := by decide +kernel

def finalExponentHom : BaseUnits →* cubeRoots :=
  (powMonoidHom 2 : BaseUnits →* BaseUnits).codRestrict cubeRoots final_exponent_root

theorem finalExponent_kernel : finalExponentHom.ker=baseCubes := by
  ext u
  change (finalExponentHom u=1) ↔ ∃ v : BaseUnits,v^3=u
  have he : finalExponentHom u=1 ↔ u^2=1 := by
    constructor
    · intro h
      exact congrArg Subtype.val h
    · intro h
      exact Subtype.ext h
  exact he.trans (final_exponent_kernel_table u)

theorem finalExponent_surjective : Function.Surjective finalExponentHom := by decide +kernel

noncomputable def finalExponentEquiv : UnitTarget ≃* cubeRoots :=
  (QuotientGroup.quotientMulEquivOfEq finalExponent_kernel.symm).trans
    (QuotientGroup.quotientKerEquivOfSurjective finalExponentHom finalExponent_surjective)

theorem finalExponentEquiv_mk (u : BaseUnits) :
    (finalExponentEquiv ((QuotientGroup.mk' baseCubes) u)).val=u^2 := by
  change ((QuotientGroup.quotientKerEquivOfSurjective finalExponentHom finalExponent_surjective)
    ((QuotientGroup.quotientMulEquivOfEq finalExponent_kernel.symm) (QuotientGroup.mk u))).val=u^2
  rw [QuotientGroup.quotientMulEquivOfEq_mk]
  rfl

theorem actual_target_cardinalities : Nat.card cubeRoots=3 ∧ Nat.card UnitTarget=3 := by
  have hr : Nat.card cubeRoots=3 := by
    rw [Nat.card_eq_fintype_card]
    decide +kernel
  exact ⟨hr,(Nat.card_congr finalExponentEquiv.toEquiv).trans hr⟩

def rootValue (z : cubeRoots) : F := z.val.val
theorem rootValue_mul (z w : cubeRoots) : rootValue (z*w)=rootValue z*rootValue w := rfl
theorem rootValue_one : rootValue 1=1 := rfl
theorem rootValue_injective : Function.Injective rootValue :=
  fun _ _ h => Subtype.ext (Units.ext h)

#print axioms finalExponent_kernel
#print axioms actual_target_cardinalities
end PairingKOneAudit
