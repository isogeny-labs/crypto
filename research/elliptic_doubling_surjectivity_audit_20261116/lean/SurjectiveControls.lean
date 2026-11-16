import DyadicDivision

namespace EllipticDoublingSurjectivityAudit
noncomputable section
open WeierstrassCurve.Affine
private instance : Fact (Nat.Prime 5):=⟨by decide⟩
private instance : Fact (Nat.Prime 2):=⟨by decide⟩
private instance : Fact (Nat.Prime 3):=⟨by decide⟩

def fiveCurve : WeierstrassCurve.Affine (ZMod 5):=⟨0,0,0,1,0⟩
theorem five_delta : fiveCurve.Δ≠0 := by decide +kernel
theorem five_nonsingular : fiveCurve.Nonsingular 0 0 := by
  apply (equation_iff_nonsingular_of_Δ_ne_zero five_delta).mp
  rw [WeierstrassCurve.Affine.equation_iff']
  decide +kernel
def fivePoint : fiveCurve.Point:=.some 0 0 five_nonsingular
theorem five_point_ne_zero : fivePoint≠0 := Point.some_ne_zero _

theorem five_all_ordinates : ∀ x y : ZMod 5,fiveCurve.Equation x y→y=fiveCurve.negY x y := by
  simp only [WeierstrassCurve.Affine.equation_iff',WeierstrassCurve.Affine.negY]
  decide +kernel

theorem five_all_doubles_zero (P : fiveCurve.Point) : P+P=0 := by
  cases P with
  | zero => exact zero_add 0
  | some x y h => exact Point.add_self_of_Y_eq (five_all_ordinates x y h.1)

theorem actual_five_no_half : ¬∃ P : fiveCurve.Point,P+P=fivePoint := by
  rintro ⟨P,h⟩
  exact five_point_ne_zero (h.symm.trans (five_all_doubles_zero P))

def UnguardedPointSurjectivityReading : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K),
    ∀ (W : WeierstrassCurve.Affine K) (_hΔ : W.Δ≠0),
      Function.Surjective (fun P : W.Point=>P+P)

theorem unguarded_point_surjectivity_false : ¬UnguardedPointSurjectivityReading := by
  intro h
  exact actual_five_no_half (h (ZMod 5) inferInstance inferInstance fiveCurve five_delta fivePoint)

abbrev BinaryClosure:=AlgebraicClosure (ZMod 2)
abbrev TernaryClosure:=AlgebraicClosure (ZMod 3)
abbrev RationalClosure:=AlgebraicClosure ℚ
private instance : DecidableEq BinaryClosure:=Classical.decEq _
private instance : DecidableEq TernaryClosure:=Classical.decEq _
private instance : DecidableEq RationalClosure:=Classical.decEq _
private instance : CharP BinaryClosure 2:=charP_of_injective_algebraMap' (ZMod 2) 2
private instance : CharP TernaryClosure 3:=charP_of_injective_algebraMap' (ZMod 3) 3

def binaryCurve : WeierstrassCurve.Affine BinaryClosure:=⟨0,0,1,0,0⟩
def healthyCurve (K : Type*) [Field K] : WeierstrassCurve.Affine K:=⟨0,0,0,1,0⟩
theorem binary_delta : binaryCurve.Δ≠0 := by
  norm_num [binaryCurve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
    WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  exact fun h=>by
    have := (CharP.cast_eq_zero_iff BinaryClosure 2 27).mp h
    norm_num at this
theorem ternary_delta : (healthyCurve TernaryClosure).Δ≠0 := by
  norm_num [healthyCurve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
    WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  exact fun h=>by
    have := (CharP.cast_eq_zero_iff TernaryClosure 3 64).mp h
    norm_num at this
theorem rational_delta : (healthyCurve RationalClosure).Δ≠0 := by
  norm_num [healthyCurve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
    WeierstrassCurve.b₆,WeierstrassCurve.b₈]

def ActualSurjectivityControls : Prop:=
  (∀ n : ℕ,Function.Surjective (fun P : binaryCurve.Point=>(2^n : ℕ) • P)) ∧
  (∀ n : ℕ,Function.Surjective (fun P : (healthyCurve TernaryClosure).Point=>(2^n : ℕ) • P)) ∧
  (∀ n : ℕ,Function.Surjective (fun P : (healthyCurve RationalClosure).Point=>(2^n : ℕ) • P))

theorem actual_surjectivity_controls : ActualSurjectivityControls :=
  ⟨actual_dyadic_surjective _ binary_delta,actual_dyadic_surjective _ ternary_delta,
    actual_dyadic_surjective _ rational_delta⟩

#print axioms actual_five_no_half
#print axioms unguarded_point_surjectivity_false
#print axioms actual_surjectivity_controls
end
end EllipticDoublingSurjectivityAudit
