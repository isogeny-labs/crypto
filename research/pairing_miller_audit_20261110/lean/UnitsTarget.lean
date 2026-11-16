import TateQuotient

namespace PairingMillerUnitsTargetAudit
open PairingMillerCurveAudit

abbrev ExtUnits := F25ˣ
def extCubes : Subgroup ExtUnits := (powMonoidHom 3 : ExtUnits→*ExtUnits).range
abbrev ExtTarget := ExtUnits ⧸ extCubes
def cubeRoots : Subgroup ExtUnits := (powMonoidHom 3 : ExtUnits→*ExtUnits).ker

instance rootMembershipDecidable (u : ExtUnits) : Decidable (u∈cubeRoots) :=
  inferInstanceAs (Decidable (u^3=1))

theorem final_exponent_root : ∀ u : ExtUnits,(u^8)^3=1 := by decide +kernel

theorem final_exponent_kernel_table : ∀ u : ExtUnits,
    u^8=1 ↔ ∃ v : ExtUnits,v^3=u := by decide +kernel

def finalExponentHom : ExtUnits→*cubeRoots :=
  (powMonoidHom 8 : ExtUnits→*ExtUnits).codRestrict cubeRoots (final_exponent_root)

theorem finalExponent_kernel : finalExponentHom.ker=extCubes := by
  ext u
  change (finalExponentHom u=1) ↔ ∃ v : ExtUnits,v^3=u
  have he : finalExponentHom u=1 ↔ u^8=1 := by
    constructor
    · intro h
      exact congrArg Subtype.val h
    · intro h
      exact Subtype.ext h
  exact he.trans (final_exponent_kernel_table u)

theorem finalExponent_surjective : Function.Surjective finalExponentHom := by decide +kernel

noncomputable def finalExponentEquiv : ExtTarget≃*cubeRoots :=
  (QuotientGroup.quotientMulEquivOfEq finalExponent_kernel.symm).trans
    (QuotientGroup.quotientKerEquivOfSurjective finalExponentHom finalExponent_surjective)

theorem finalExponentEquiv_mk (u : ExtUnits) :
    (finalExponentEquiv ((QuotientGroup.mk' extCubes) u)).val=u^8 := by
  change ((QuotientGroup.quotientKerEquivOfSurjective finalExponentHom finalExponent_surjective)
    ((QuotientGroup.quotientMulEquivOfEq finalExponent_kernel.symm) (QuotientGroup.mk u))).val=u^8
  rw [QuotientGroup.quotientMulEquivOfEq_mk]
  rfl

theorem actual_target_cardinalities : Nat.card cubeRoots=3 ∧ Nat.card ExtTarget=3 := by
  have hr : Nat.card cubeRoots=3 := by
    rw [Nat.card_eq_fintype_card]
    decide +kernel
  exact ⟨hr,(Nat.card_congr finalExponentEquiv.toEquiv).trans hr⟩

#print axioms finalExponent_kernel
#print axioms actual_target_cardinalities
end PairingMillerUnitsTargetAudit
