import AuxiliaryChoice

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open ThreeTorsionRankAudit.General ThreeTorsionCharacterAudit ThreeTangentPairingAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

/-- Actual characteristic-three curve has a nonzero radical when coprimality is deleted. -/
theorem source_characteristic_three_degenerate [CharP K 3] :
    ∃ hΔ : (curve (1 : K) 1).Δ≠0,
      ∃ P : ThreeTorsion (curve (1 : K) 1),P≠0 ∧
        ∀ Q : ThreeTorsion (curve (1 : K) 1),divisorWeilThree (curve (1 : K) 1) hΔ P Q=1 := by
  obtain ⟨hΔ,P,hP,h⟩ := characteristic_three_degenerate (K := K)
  refine ⟨hΔ,P,hP,?_⟩
  intro Q
  apply Units.ext
  rw [divisor_weil_three_value,h Q,inv_one]
  rfl

/-- Actual restriction to one nonzero cyclic torsion line is degenerate. -/
theorem source_cyclic_line_degenerate [NeZero (3 : K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P : ThreeTorsion W) (hP : P≠0) :
    ∃ Q : Submodule.span (ZMod 3) {P},Q≠0 ∧
      ∀ R : Submodule.span (ZMod 3) {P},divisorWeilThree W hΔ Q.val R.val=1 := by
  obtain ⟨Q,hQ,h⟩ := actual_cyclic_line_degenerate W hΔ P hP
  refine ⟨Q,hQ,?_⟩
  intro R
  apply Units.ext
  rw [divisor_weil_three_value,h R,inv_one]
  rfl

omit [DecidableEq K] [IsAlgClosed K] in
theorem nontrivial_cube_inverse_ne {z : K} (hz : z^3=1) (hn : z≠1) : z⁻¹≠z := by
  intro he
  have hz0 : z≠0 := by
    intro h0
    rw [h0,zero_pow (by decide : (3 : ℕ)≠0)] at hz
    exact zero_ne_one hz
  have h2 : z*z=1 := by
    calc
      z*z=z*z⁻¹ := congrArg (fun t => z*t) he.symm
      _ =1 := mul_inv_cancel₀ hz0
  have h3 : z*z*z=1 := by simpa only [pow_succ,pow_two,pow_one,pow_zero,one_mul] using hz
  rw [h2,one_mul] at h3
  exact hn h3

abbrev TernaryClosure := AlgebraicClosure (ZMod 3)
private instance : DecidableEq TernaryClosure := Classical.decEq _
private instance : CharP TernaryClosure 3 := charP_of_injective_algebraMap' (ZMod 3) 3

theorem ternary_radical_counter :
    CharP TernaryClosure 3 ∧
      ∃ hΔ : (curve (1 : TernaryClosure) 1).Δ≠0,
        ∃ P : ThreeTorsion (curve (1 : TernaryClosure) 1),P≠0 ∧
          ∀ Q : ThreeTorsion (curve (1 : TernaryClosure) 1),
            divisorWeilThree (curve (1 : TernaryClosure) 1) hΔ P Q=1 :=
  ⟨inferInstance,source_characteristic_three_degenerate⟩

private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : DecidableEq BinaryClosure := Classical.decEq _
private instance : CharP BinaryClosure 2 := charP_of_injective_algebraMap' (ZMod 2) 2
private instance : NeZero (3 : BinaryClosure) := by
  constructor
  intro h
  have hd : 2 ∣ 3 := (CharP.cast_eq_zero_iff BinaryClosure 2 3).mp h
  norm_num at hd

/-- Concrete credited characteristic-two curve: omitting source-convention inversion is false. -/
theorem binary_inverse_convention_counter :
    CharP BinaryClosure 2 ∧ (3 : BinaryClosure)≠0 ∧ binaryCurve.Δ≠0 ∧
      ∃ P Q : ThreeTorsion binaryCurve,
        (divisorWeilThree binaryCurve binary_discriminant P Q)^3=1 ∧
        (divisorWeilThree binaryCurve binary_discriminant P Q : BinaryClosure)=
          (tangentPair binaryCurve P.val Q.val)⁻¹ ∧
        (divisorWeilThree binaryCurve binary_discriminant P Q : BinaryClosure)≠
          tangentPair binaryCurve P.val Q.val ∧
        divisorWeilThree binaryCurve binary_discriminant P Q≠1 := by
  obtain ⟨hΔ,P,Q,h3,hn⟩ := binary_pairing_positive
  refine ⟨inferInstance,NeZero.ne _,hΔ,P,Q,divisor_weil_three_cube binaryCurve binary_discriminant P Q,
    divisor_weil_three_value binaryCurve binary_discriminant P Q,?_,?_⟩
  · rw [divisor_weil_three_value]
    exact nontrivial_cube_inverse_ne h3 hn
  · intro h
    have he := congrArg (fun z : BinaryClosureˣ => (z : BinaryClosure)) h
    rw [divisor_weil_three_value,Units.val_one] at he
    exact hn (inv_eq_one.mp he)

#print axioms source_characteristic_three_degenerate
#print axioms source_cyclic_line_degenerate
#print axioms binary_inverse_convention_counter
end
end WeilAuxiliaryRatioAudit
