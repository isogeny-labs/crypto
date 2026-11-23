module
public import PrimeTrace
public import Mathlib.RingTheory.RootsOfUnity.AlgebraicallyClosed
public import Mathlib.FieldTheory.IntermediateField.Adjoin.Basic

@[expose] public section
namespace PrimeTraceWeakBoundAudit
noncomputable section
open Polynomial IntermediateField

variable (p : ℕ) [Fact p.Prime]

abbrev Closure := AlgebraicClosure (ZMod p)
def rootsField (r : ℕ) : IntermediateField (ZMod p) (Closure p) :=
  IntermediateField.adjoin (ZMod p) {z : Closure p | z ^ r = 1}

theorem roots_field_degree_two (r : ℕ) (hr : 2 < r) (hd : r ∣ p + 1) :
    Module.finrank (ZMod p) (rootsField p r) = 2 := by
  have hp := (Fact.out : p.Prime)
  have hpr : ¬ p ∣ r := by
    intro h
    have hpp : p ∣ p + 1 := h.trans hd
    have hpone : p ∣ 1 := (Nat.dvd_add_iff_right (dvd_refl p)).mpr hpp
    exact hp.not_dvd_one hpone
  let : NeZero r := ⟨by omega⟩
  let : NeZero (r : ZMod p) := ⟨by
    intro h
    exact hpr ((ZMod.natCast_eq_zero_iff r p).mp h)⟩
  obtain ⟨ζ, hζ⟩ := HasEnoughRootsOfUnity.exists_isPrimitiveRoot (Closure p) r
  have hsimple : rootsField p r = (ZMod p)⟮ζ⟯ := by
    apply le_antisymm
    · apply IntermediateField.adjoin_le_iff.mpr
      intro x hx
      obtain ⟨i, _, hi⟩ := hζ.eq_pow_of_pow_eq_one hx
      rw [← hi]
      exact ((ZMod p)⟮ζ⟯).toSubfield.pow_mem
        (IntermediateField.mem_adjoin_simple_self (ZMod p) ζ) i
    · apply IntermediateField.adjoin_simple_le_iff.mpr
      exact IntermediateField.subset_adjoin _ _ hζ.pow_eq_one
  have hint : IsIntegral (ZMod p) ζ :=
    (Algebra.IsAlgebraic.isAlgebraic ζ).isIntegral
  let : FiniteDimensional (ZMod p) (rootsField p r) := by
    rw [hsimple]
    exact IntermediateField.adjoin.finiteDimensional hint
  let : Finite (rootsField p r) :=
    Finite.of_injective (Module.finBasis (ZMod p) (rootsField p r)).equivFun
      (Module.finBasis (ZMod p) (rootsField p r)).equivFun.injective
  let σ := FiniteField.frobeniusAlgEquivOfAlgebraic (ZMod p) (rootsField p r)
  have hroot (x : Closure p) (hx : x ^ r = 1) : x ^ p * x = 1 := by
    obtain ⟨k, hk⟩ := hd
    have he : x ^ (p + 1) = 1 := by rw [hk, pow_mul, hx, one_pow]
    simpa only [pow_succ] using he
  have hsquare : σ ^ 2 = 1 := by
    apply AlgEquiv.coe_toAlgHom_injective
    apply IntermediateField.adjoin_algHom_ext (ZMod p)
    intro x hx
    apply Subtype.ext
    change (x ^ Fintype.card (ZMod p)) ^ Fintype.card (ZMod p) = x
    rw [ZMod.card]
    have h1 := hroot x hx
    have h2 := congrArg (fun a : Closure p => a ^ p) h1
    rw [mul_pow, one_pow] at h2
    calc
      _ = (x ^ p) ^ p * (x ^ p * x) := by rw [h1, mul_one]
      _ = ((x ^ p) ^ p * x ^ p) * x := by ring
      _ = x := by rw [h2, one_mul]
  have hne : σ ≠ 1 := by
    intro he
    let z : rootsField p r := ⟨ζ, IntermediateField.subset_adjoin _ _ hζ.pow_eq_one⟩
    have hz := congrArg (fun a : (rootsField p r) ≃ₐ[ZMod p] (rootsField p r) =>
      (a z : Closure p)) he
    change ζ ^ Fintype.card (ZMod p) = ζ at hz
    rw [ZMod.card] at hz
    have hs : ζ ^ 2 = 1 := by
      have h := hroot ζ hζ.pow_eq_one
      rw [hz, ← pow_two] at h
      exact h
    exact (not_le.mpr hr) (Nat.le_of_dvd (by decide) (hζ.dvd_of_pow_eq_one 2 hs))
  have ho : orderOf σ = 2 := orderOf_eq_prime hsquare hne
  exact (FiniteField.orderOf_frobeniusAlgEquivOfAlgebraic (ZMod p) (rootsField p r)).symm.trans ho

/-- Credited actual-adjoin control from SourceFSTBoundary; all square roots
of unity belong to the base field in every extension. -/
theorem roots_two_field_bot : rootsField p 2 = ⊥ := by
  apply le_antisymm
  · apply IntermediateField.adjoin_le_iff.mpr
    intro z hz
    apply IntermediateField.mem_bot.mpr
    rcases sq_eq_one_iff.mp hz with h | h
    · exact ⟨1, by simpa using h.symm⟩
    · exact ⟨-1, by simpa using h.symm⟩
  · exact bot_le

theorem roots_two_field_degree : Module.finrank (ZMod p) (rootsField p 2) = 1 := by
  rw [roots_two_field_bot]
  exact IntermediateField.finrank_bot

theorem prime_source_cyclotomic_degree (hp : 5 ≤ p)
    (W : WeierstrassCurve.Affine (ZMod p)) (hΔ : W.Δ ≠ 0)
    (hss : sourceSupersingular p W) (r : ℕ) (hr : 2 < r) (hd : r ∣ Nat.card W.Point) :
    Module.finrank (ZMod p) (rootsField p r) = 2 := by
  apply roots_field_degree_two p r hr
  rwa [prime_source_count p hp W hΔ hss] at hd

#print axioms roots_field_degree_two
#print axioms roots_two_field_degree
#print axioms prime_source_cyclotomic_degree
end
end PrimeTraceWeakBoundAudit
