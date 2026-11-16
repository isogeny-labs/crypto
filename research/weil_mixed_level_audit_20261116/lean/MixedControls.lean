import MixedRadical

namespace WeilMixedLevelAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit
private instance : Fact (Nat.Prime 2):=⟨by decide⟩
private instance : Fact (Nat.Prime 3):=⟨by decide⟩
private instance : Fact (Nat.Prime 5):=⟨by decide⟩

def originCurve (K : Type*) [Field K] : WeierstrassCurve.Affine K :=
  ThreeTorsionCharacterAudit.curve 1 1

def originThree (K : Type*) [Field K] [DecidableEq K] : torsionPoints (originCurve K) 3 :=
  fromThree _ (ThreeTorsionCharacterAudit.torsionOrigin 1 1 one_ne_zero)

theorem origin_three_ne_zero (K : Type*) [Field K] [DecidableEq K] : originThree K≠0 := by
  intro h
  exact ThreeTorsionCharacterAudit.origin_nonzero 1 1 one_ne_zero
    (congrArg (fun X : torsionPoints (originCurve K) 3=>X.val) h)

abbrev TernaryClosure:=ThreeTorsionCharacterAudit.OrdinaryClosure
private instance : DecidableEq TernaryClosure:=Classical.decEq _
private instance : CharP TernaryClosure 3:=charP_of_injective_algebraMap' (ZMod 3) 3
private instance : NeZero (2 : TernaryClosure):=⟨by
  intro h
  have he:=(CharP.cast_eq_zero_iff TernaryClosure 3 2).mp h
  norm_num at he⟩

theorem ordinary_ternary_delta : (originCurve TernaryClosure).Δ≠0 :=
  ThreeTorsionCharacterAudit.ordinary_discriminant

def ternaryMixedPoint (n : ℕ) := threeToMixed (originCurve TernaryClosure) n (originThree TernaryClosure)

theorem actual_ternary_mixed_radical (n : ℕ) :
    (originCurve TernaryClosure).Δ≠0 ∧ ternaryMixedPoint n≠0 ∧
      ∀ Q,geometricPairing _ ordinary_ternary_delta (mixedLevel n) (ternaryMixedPoint n) Q=1 := by
  refine ⟨ordinary_ternary_delta,three_to_mixed_ne_zero _ n _ (origin_three_ne_zero _),?_⟩
  intro Q
  apply characteristic_torsion_radical 3
  apply Subtype.ext
  change (3 : ℕ) • (ternaryMixedPoint n).val=0
  exact (natCast_zsmul (ternaryMixedPoint n).val 3).symm.trans (originThree TernaryClosure).property

abbrev BinaryClosure:=WeilTwoNondegeneracyAudit.BinaryClosure
private instance : DecidableEq BinaryClosure:=Classical.decEq _
private instance : CharP BinaryClosure 2:=charP_of_injective_algebraMap' (ZMod 2) 2
private instance : NeZero (3 : BinaryClosure):=⟨by
  intro h
  have he:=(CharP.cast_eq_zero_iff BinaryClosure 2 3).mp h
  norm_num at he⟩

def binaryMixedPoint (n : ℕ) := twoToMixed _ n WeilTwoNondegeneracyAudit.ordinaryBinaryPoint

theorem actual_binary_mixed_radical (n : ℕ) :
    WeilTwoNondegeneracyAudit.ordinaryBinaryCurve.Δ≠0 ∧ binaryMixedPoint n≠0 ∧
      ∀ Q,geometricPairing _ WeilTwoNondegeneracyAudit.ordinary_binary_delta
        (mixedLevel (n+1)) (binaryMixedPoint n) Q=1 := by
  refine ⟨WeilTwoNondegeneracyAudit.ordinary_binary_delta,
    two_to_mixed_ne_zero _ n _ WeilTwoNondegeneracyAudit.ordinary_binary_point_ne_zero,?_⟩
  intro Q
  apply characteristic_torsion_radical 2
  apply Subtype.ext
  change (2 : ℕ) • (binaryMixedPoint n).val=0
  exact (natCast_zsmul (binaryMixedPoint n).val 2).symm.trans
    WeilTwoNondegeneracyAudit.ordinaryBinaryPoint.property

def DeletedTwoGuardReading : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (_ : IsAlgClosed K) (_ : NeZero (3 : K)),
    ∀ (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℕ) (P : torsionPoints W (mixedLevel n)),
      (∀ Q,geometricPairing W hΔ (mixedLevel n) P Q=1)→P=0

def DeletedThreeGuardReading : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (_ : IsAlgClosed K) (_ : NeZero (2 : K)),
    ∀ (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℕ) (P : torsionPoints W (mixedLevel n)),
      (∀ Q,geometricPairing W hΔ (mixedLevel n) P Q=1)→P=0

theorem deleted_two_guard_false : ¬DeletedTwoGuardReading := by
  intro h
  exact (actual_binary_mixed_radical 0).2.1
    (h BinaryClosure inferInstance inferInstance inferInstance inferInstance
      _ WeilTwoNondegeneracyAudit.ordinary_binary_delta 1 _ (actual_binary_mixed_radical 0).2.2)

theorem deleted_three_guard_false : ¬DeletedThreeGuardReading := by
  intro h
  exact (actual_ternary_mixed_radical 0).2.1
    (h TernaryClosure inferInstance inferInstance inferInstance inferInstance
      _ ordinary_ternary_delta 0 _ (actual_ternary_mixed_radical 0).2.2)

theorem actual_mixed_positive (K : Type*) [Field K] [DecidableEq K] [IsAlgClosed K]
    [NeZero (2 : K)] [NeZero (3 : K)] (hΔ : (originCurve K).Δ≠0) (n : ℕ) :
    ∃ P Q : torsionPoints (originCurve K) (mixedLevel n),
      geometricPairing _ hΔ (mixedLevel n) P Q≠1 := by
  have hp:=three_to_mixed_ne_zero (originCurve K) n _ (origin_three_ne_zero K)
  by_contra hh
  have hr : ∀ Q,geometricPairing _ hΔ (mixedLevel n) (threeToMixed _ n (originThree K)) Q=1 := by
    intro Q
    by_contra hq
    exact hh ⟨_,Q,hq⟩
  exact hp (geometric_mixed_left_nondegenerate _ hΔ n _ hr)

abbrev RationalClosure:=WeilTwoNondegeneracyAudit.RationalClosure
private instance : DecidableEq RationalClosure:=Classical.decEq _
abbrev FiveClosure:=WeilGeometricLawsAudit.FiveClosure
private instance : DecidableEq FiveClosure:=Classical.decEq _
private instance : CharP FiveClosure 5:=charP_of_injective_algebraMap' (ZMod 5) 5
private instance : NeZero (2 : FiveClosure):=⟨by
  intro h
  have he:=(CharP.cast_eq_zero_iff FiveClosure 5 2).mp h
  norm_num at he⟩
private instance : NeZero (3 : FiveClosure):=⟨by
  intro h
  have he:=(CharP.cast_eq_zero_iff FiveClosure 5 3).mp h
  norm_num at he⟩

theorem healthy_rational_delta : (originCurve RationalClosure).Δ≠0 := by
  rw [originCurve,ThreeTorsionCharacterAudit.discriminant_value]
  norm_num

theorem healthy_five_delta : (originCurve FiveClosure).Δ≠0 := by
  rw [originCurve,ThreeTorsionCharacterAudit.discriminant_value]
  norm_num
  intro h
  have he:=(CharP.cast_eq_zero_iff FiveClosure 5 26).mp h
  norm_num at he

def ActualMixedPositiveControls : Prop:=
  (∀ n : ℕ,∃ P Q : torsionPoints (originCurve RationalClosure) (mixedLevel n),
    geometricPairing _ healthy_rational_delta (mixedLevel n) P Q≠1) ∧
  (∀ n : ℕ,∃ P Q : torsionPoints (originCurve FiveClosure) (mixedLevel n),
    geometricPairing _ healthy_five_delta (mixedLevel n) P Q≠1)

theorem actual_mixed_positive_controls : ActualMixedPositiveControls :=
  ⟨actual_mixed_positive _ healthy_rational_delta,actual_mixed_positive _ healthy_five_delta⟩

#print axioms actual_ternary_mixed_radical
#print axioms actual_binary_mixed_radical
#print axioms deleted_two_guard_false
#print axioms deleted_three_guard_false
#print axioms actual_mixed_positive_controls
end
end WeilMixedLevelAudit
