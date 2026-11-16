import TangentFrobenius
import ThreeCharacterControls

namespace ThreeTangentPairingAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General ThreeTorsionCharacterAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {K : Type*} [Field K] [DecidableEq K]

/-- Actual outside-domain tangents are nonzero; total field division is not a domain proof. -/
theorem tangentPair_outside_domain (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (x y u v : K) (hP : W.Nonsingular x y) (hQ : W.Nonsingular u v)
    (htP : (3:ℕ) • Point.some x y hP=0) (htQ : (3:ℕ) • Point.some u v hQ=0)
    (hx : x≠u) : tangentValue W x y u v≠0 ∧ tangentValue W u v x y≠0 := by
  have hn := tangentPair_ne_zero W hΔ ⟨Point.some x y hP,htP⟩ ⟨Point.some u v hQ,htQ⟩
  change (if x=u then 1 else -tangentValue W x y u v/tangentValue W u v x y)≠0 at hn
  rw [ite_eq_right hx] at hn
  constructor
  · intro hz
    simp only [hz,neg_zero,zero_div] at hn
    exact hn rfl
  · intro hz
    simp only [hz,div_zero] at hn
    exact hn rfl

theorem characteristic_three_degenerate [CharP K 3] :
    (curve (1:K) 1).Δ≠0 ∧
    ∃ P : ThreeTorsion (curve (1:K) 1), P≠0 ∧
      ∀ Q : ThreeTorsion (curve (1:K) 1), tangentPair (curve (1:K) 1) P.val Q.val=1 := by
  refine ⟨ordinary_discriminant, torsionOrigin 1 1 one_ne_zero,?_,?_⟩
  · intro hz
    exact origin_nonzero 1 1 one_ne_zero (congrArg Subtype.val hz)
  · rintro ⟨Q,hQ⟩
    cases Q with
    | zero => rfl
    | some x y hN =>
      have hx := (ordinary_coordinates x y hN hQ).1
      simp [tangentPair,origin,hx]

variable [IsAlgClosed K] [NeZero (3:K)]

/-- Restricting an alternating pairing to one actual cyclic line destroys nondegeneracy. -/
theorem actual_cyclic_line_degenerate (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P : ThreeTorsion W) (hP : P≠0) :
    ∃ Q : Submodule.span (ZMod 3) {P}, Q≠0 ∧
      ∀ R : Submodule.span (ZMod 3) {P}, tangentPair W Q.val.val R.val.val=1 := by
  have hp : P∈Submodule.span (ZMod 3) {P} := Submodule.subset_span (by simp)
  refine ⟨⟨P,hp⟩,?_,?_⟩
  · intro h
    exact hP (congrArg Subtype.val h)
  · intro R
    obtain ⟨i,hi⟩ := Submodule.mem_span_singleton.mp R.property
    rw [←hi,tangentPair_smul_right W hΔ,tangentPair_self,one_pow]

#print axioms tangentPair_outside_domain
#print axioms characteristic_three_degenerate

private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : DecidableEq BinaryClosure := Classical.decEq _
private instance : CharP BinaryClosure 2 := charP_of_injective_algebraMap' (ZMod 2) 2
private instance : NeZero (3:BinaryClosure) := by
  constructor
  intro h
  have hb : 2 ∣ 3 := (CharP.cast_eq_zero_iff BinaryClosure 2 3).mp h
  norm_num at hb

/-- The credited characteristic-two curve supports the newly proved actual pairing. -/
theorem binary_pairing_positive :
    binaryCurve.Δ≠0 ∧ ∃ P Q : ThreeTorsion binaryCurve,
      (tangentPair binaryCurve P.val Q.val)^3=1 ∧ tangentPair binaryCurve P.val Q.val≠1 := by
  let b := geometricThreeBasis binaryCurve binary_discriminant
  exact ⟨binary_discriminant,b 0,b 1,tangentPair_cube binaryCurve binary_discriminant (b 0) (b 1),
    basis_pairing_nontrivial binaryCurve binary_discriminant b⟩

#print axioms binary_pairing_positive
end
end ThreeTangentPairingAudit
