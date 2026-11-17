import SevenBasis
import FiveControls

namespace SevenTorsionRankAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine ThreeTorsionRankAudit ThreeTorsionRankAudit.General Module
private instance : Fact (Nat.Prime 7) := ⟨by decide⟩
variable {K : Type*} [Field K]

theorem seven_degree_char7 [CharP K 7] (W : WeierstrassCurve K) : (psiSeven W).natDegree≤23 := by
  have hd := W.natDegree_preΨ'_le 7
  norm_num at hd
  change (psiSeven W).natDegree≤24 at hd
  have hc := W.coeff_preΨ' 7
  norm_num at hc
  change (psiSeven W).coeff 24=(7:K) at hc
  have hc0 : (psiSeven W).coeff 24=0 := hc.trans (CharP.cast_eq_zero K 7)
  by_contra hn
  have he : (psiSeven W).natDegree=24 := by omega
  have hp : psiSeven W≠0 := ne_zero_of_natDegree_gt
    (show 0<(psiSeven W).natDegree by rw [he]; decide)
  have hl : (psiSeven W).coeff 24≠0 := by
    rw [←he]
    exact Polynomial.leadingCoeff_ne_zero.mpr hp
  exact hl hc0

def charSevenCurve (K : Type*) [Field K] : WeierstrassCurve.Affine K := short (-1:K) 0

theorem charSeven_at_zero : (psiSeven (charSevenCurve K)).eval 0= -1 := by
  rw [seven_eval,FiveTorsionRankAudit.five_eval]
  norm_num [charSevenCurve,short,WeierstrassCurve.Ψ₂Sq,WeierstrassCurve.preΨ₄,WeierstrassCurve.Ψ₃,
    WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]

theorem charSeven_polynomial_nonzero : psiSeven (charSevenCurve K)≠0 := by
  intro hp
  have he : (-1:K)=0 := by
    simpa only [charSeven_at_zero,Polynomial.eval_zero] using
      congrArg (fun p : K[X] => p.eval 0) hp
  exact neg_ne_zero.mpr (one_ne_zero : (1:K)≠0) he

theorem charSeven_discriminant [CharP K 7] : (charSevenCurve K).Δ≠0 := by
  have h7 : (7:K)=0 := CharP.cast_eq_zero K 7
  have hd : (charSevenCurve K).Δ=1 := by
    have he : (charSevenCurve K).Δ=64 := by
      norm_num [charSevenCurve,short,WeierstrassCurve.Δ,WeierstrassCurve.b₂,
        WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
    rw [he]
    linear_combination 9*h7
  rw [hd]
  exact one_ne_zero

variable [DecidableEq K] [IsAlgClosed K] [CharP K 7]

/-- A strict point-count bound suffices to refute deleting coprimality; no exact kernel1 claim. -/
theorem charSeven_count_le : Nat.card (SevenTorsion (charSevenCurve K))≤47 := by
  have hr : Nat.card (SevenRoot (charSevenCurve K))≤23 :=
    (seven_root_count_le _ charSeven_polynomial_nonzero).trans (seven_degree_char7 _)
  rw [seven_card_fiber_formula _ charSeven_discriminant charSeven_polynomial_nonzero]
  omega

theorem charSeven_count_ne : Nat.card (SevenTorsion (charSevenCurve K))≠49 := by
  have h := charSeven_count_le (K:=K)
  omega

theorem charSeven_rank_ne : finrank (ZMod 7) (SevenTorsion (charSevenCurve K))≠2 := by
  let : Finite (SevenTorsion (charSevenCurve K)) :=
    seven_torsion_finite_of_nonzero _ charSeven_discriminant charSeven_polynomial_nonzero
  have hc := Module.natCard_eq_pow_finrank (K:=ZMod 7) (V:=SevenTorsion (charSevenCurve K))
  have hF : Nat.card (ZMod 7)=7 := by simp
  intro hr
  rw [hF,hr] at hc
  exact charSeven_count_ne hc

private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev QuinaryClosure := AlgebraicClosure (ZMod 5)
abbrev SeptenaryClosure := AlgebraicClosure (ZMod 7)
private instance : DecidableEq BinaryClosure := Classical.decEq _
private instance : CharP BinaryClosure 2 := charP_of_injective_algebraMap' (ZMod 2) 2
private instance : NeZero (7:BinaryClosure) := by
  constructor
  intro h
  have hb : 2 ∣ 7 := (CharP.cast_eq_zero_iff BinaryClosure 2 7).mp h
  norm_num at hb
private instance : DecidableEq CharThreeClosure := Classical.decEq _
private instance : CharP CharThreeClosure 3 := charP_of_injective_algebraMap' (ZMod 3) 3
private instance : NeZero (7:CharThreeClosure) := by
  constructor
  intro h
  have hb : 3 ∣ 7 := (CharP.cast_eq_zero_iff CharThreeClosure 3 7).mp h
  norm_num at hb
private instance : DecidableEq QuinaryClosure := Classical.decEq _
private instance : CharP QuinaryClosure 5 := charP_of_injective_algebraMap' (ZMod 5) 5
private instance : NeZero (7:QuinaryClosure) := by
  constructor
  intro h
  have hb : 5 ∣ 7 := (CharP.cast_eq_zero_iff QuinaryClosure 5 7).mp h
  norm_num at hb
private instance : DecidableEq SeptenaryClosure := Classical.decEq _
private instance : CharP SeptenaryClosure 7 := charP_of_injective_algebraMap' (ZMod 7) 7

theorem binary_seven_positive :
    CharP BinaryClosure 2 ∧ binaryCurve.Δ≠0 ∧ Nat.card (SevenTorsion binaryCurve)=49 ∧
      finrank (ZMod 7) (SevenTorsion binaryCurve)=2 :=
  ⟨inferInstance,binary_discriminant,geometric_seven_card binaryCurve binary_discriminant,
    geometric_seven_finrank binaryCurve binary_discriminant⟩

theorem ternary_seven_positive :
    CharP CharThreeClosure 3 ∧ (short (-1:CharThreeClosure) 0).Δ≠0 ∧
      Nat.card (SevenTorsion (short (-1:CharThreeClosure) 0))=49 ∧
      finrank (ZMod 7) (SevenTorsion (short (-1:CharThreeClosure) 0))=2 :=
  ⟨inferInstance,char_three_discriminant,geometric_seven_card _ char_three_discriminant,
    geometric_seven_finrank _ char_three_discriminant⟩

theorem quinary_seven_positive :
    CharP QuinaryClosure 5 ∧ (FiveTorsionRankAudit.charFiveCurve QuinaryClosure).Δ≠0 ∧
      Nat.card (SevenTorsion (FiveTorsionRankAudit.charFiveCurve QuinaryClosure))=49 ∧
      finrank (ZMod 7) (SevenTorsion (FiveTorsionRankAudit.charFiveCurve QuinaryClosure))=2 :=
  ⟨inferInstance,FiveTorsionRankAudit.charFive_discriminant,
    geometric_seven_card _ FiveTorsionRankAudit.charFive_discriminant,
    geometric_seven_finrank _ FiveTorsionRankAudit.charFive_discriminant⟩

theorem septenary_counter :
    CharP SeptenaryClosure 7 ∧ (charSevenCurve SeptenaryClosure).Δ≠0 ∧
      Nat.card (SevenTorsion (charSevenCurve SeptenaryClosure))≤47 ∧
      Nat.card (SevenTorsion (charSevenCurve SeptenaryClosure))≠49 ∧
      finrank (ZMod 7) (SevenTorsion (charSevenCurve SeptenaryClosure))≠2 :=
  ⟨inferInstance,charSeven_discriminant,charSeven_count_le,charSeven_count_ne,charSeven_rank_ne⟩

#print axioms charSeven_count_ne
#print axioms septenary_counter
end
end SevenTorsionRankAudit
