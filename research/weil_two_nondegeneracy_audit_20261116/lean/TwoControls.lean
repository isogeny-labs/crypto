import TwoPairing

namespace WeilTwoNondegeneracyAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit
private instance : Fact (Nat.Prime 2):=⟨by decide⟩
abbrev BinaryClosure:=AlgebraicClosure (ZMod 2)
private instance : DecidableEq BinaryClosure:=Classical.decEq _
private instance : CharP BinaryClosure 2:=charP_of_injective_algebraMap' (ZMod 2) 2

def ordinaryBinaryCurve : WeierstrassCurve.Affine BinaryClosure:=⟨1,0,0,0,1⟩

theorem ordinary_binary_delta : ordinaryBinaryCurve.Δ≠0 := by
  norm_num [ordinaryBinaryCurve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,
    WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  intro h
  have h2 : (2 : BinaryClosure)=0 := CharP.cast_eq_zero _ 2
  have hz : (1 : BinaryClosure)=0 := by linear_combination h-216*h2
  exact one_ne_zero hz

theorem ordinary_binary_nonsingular : ordinaryBinaryCurve.Nonsingular 0 1 := by
  apply (ordinaryBinaryCurve.equation_iff_nonsingular_of_Δ_ne_zero ordinary_binary_delta).mp
  rw [WeierstrassCurve.Affine.equation_iff']
  norm_num [ordinaryBinaryCurve]

def ordinaryBinaryPoint : torsionPoints ordinaryBinaryCurve 2:=
  ⟨.some 0 1 ordinary_binary_nonsingular,by
    change (2 : ℤ) • Point.some 0 1 ordinary_binary_nonsingular=0
    rw [two_zsmul]
    apply Point.add_self_of_Y_eq
    rw [WeierstrassCurve.Affine.negY]
    change (1 : BinaryClosure)= -1-1*0-0
    simp only [mul_zero,sub_zero]
    apply eq_neg_iff_add_eq_zero.mpr
    simpa only [one_add_one_eq_two,Nat.cast_ofNat] using CharP.cast_eq_zero BinaryClosure 2⟩

theorem ordinary_binary_point_ne_zero : ordinaryBinaryPoint≠0 := by
  intro he
  exact Point.some_ne_zero _ (congrArg Subtype.val he)

theorem actual_binary_two_radical :
    ordinaryBinaryCurve.Δ≠0 ∧ ordinaryBinaryPoint≠0 ∧
    ∀ Q : torsionPoints ordinaryBinaryCurve 2,
      geometricPairing ordinaryBinaryCurve ordinary_binary_delta 2 ordinaryBinaryPoint Q=1 :=
  ⟨ordinary_binary_delta,ordinary_binary_point_ne_zero,fun Q=>
    geometric_pairing_characteristic_trivial 2 _ _ _ Q⟩

def UnguardedTwoNondegeneracyReading : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (_ : IsAlgClosed K),
    ∀ (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P : torsionPoints W 2),
      (∀ Q : torsionPoints W 2,geometricPairing W hΔ 2 P Q=1)→P=0

theorem unguarded_two_nondegeneracy_false : ¬UnguardedTwoNondegeneracyReading := by
  intro h
  exact ordinary_binary_point_ne_zero
    (h BinaryClosure inferInstance inferInstance inferInstance _ ordinary_binary_delta
      ordinaryBinaryPoint actual_binary_two_radical.2.2)

#print axioms actual_binary_two_radical
#print axioms unguarded_two_nondegeneracy_false

theorem qualified_two_positive {K : Type*} [Field K] [DecidableEq K]
    [IsAlgClosed K] [NeZero (2 : K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    ∃ P Q : torsionPoints W 2,geometricPairing W hΔ 2 P Q≠1 := by
  obtain ⟨u,v,h,_,ht⟩:=exists_other_two_point W hΔ 0
  let P : torsionPoints W 2:=⟨.some u v h,by
    change (2 : ℤ) • Point.some u v h=0
    simpa only [two_zsmul] using ht⟩
  have hn : P≠0 := fun he=>Point.some_ne_zero _ (congrArg Subtype.val he)
  by_contra he
  push Not at he
  exact hn (geometric_two_left_nondegenerate W hΔ P (he P))

private instance : Fact (Nat.Prime 3):=⟨by decide⟩
abbrev TernaryClosure:=AlgebraicClosure (ZMod 3)
private instance : DecidableEq TernaryClosure:=Classical.decEq _
private instance : CharP TernaryClosure 3:=charP_of_injective_algebraMap' (ZMod 3) 3
private instance : NeZero (2 : TernaryClosure):=⟨by
  intro h
  have := (CharP.cast_eq_zero_iff TernaryClosure 3 2).mp h
  norm_num at this⟩
abbrev RationalClosure:=AlgebraicClosure ℚ
private instance : DecidableEq RationalClosure:=Classical.decEq _

def healthyTwoCurve (K : Type*) [Field K] : WeierstrassCurve.Affine K:=⟨0,0,0,1,0⟩

theorem healthy_ternary_delta : (healthyTwoCurve TernaryClosure).Δ≠0 := by
  norm_num [healthyTwoCurve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,
    WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  exact fun h=>by
    have := (CharP.cast_eq_zero_iff TernaryClosure 3 64).mp h
    norm_num at this

theorem healthy_rational_delta : (healthyTwoCurve RationalClosure).Δ≠0 := by
  norm_num [healthyTwoCurve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,
    WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]

def ActualPositiveTwoControls : Prop:=
  (∃ P Q : torsionPoints (healthyTwoCurve TernaryClosure) 2,
    geometricPairing _ healthy_ternary_delta 2 P Q≠1) ∧
  (∃ P Q : torsionPoints (healthyTwoCurve RationalClosure) 2,
    geometricPairing _ healthy_rational_delta 2 P Q≠1) ∧
  (∀ P : torsionPoints ordinaryBinaryCurve 2,
    geometricPairing _ ordinary_binary_delta 2 P P=1 ∧
    geometricPairing _ ordinary_binary_delta 2 0 P=1)

theorem actual_positive_two_controls : ActualPositiveTwoControls :=
  ⟨qualified_two_positive _ _,qualified_two_positive _ _,fun P=>
    ⟨geometric_pairing_alternating _ _ _ P,geometric_pairing_zero_left _ _ _ P⟩⟩

#print axioms actual_positive_two_controls
end
end WeilTwoNondegeneracyAudit
