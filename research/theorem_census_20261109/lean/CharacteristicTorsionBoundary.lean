import BinaryMisapplication
import Mathlib.FieldTheory.IsAlgClosed.AlgebraicClosure

/-! Actual characteristic-torsion boundary, not a refutation of BK or
prime-to-characteristic pairing theorems on their stated domains. -/
namespace CharacteristicTorsionAudit
noncomputable section
open SemaevS3 BinaryS3Audit
abbrev F := ZMod 2
abbrev K := AlgebraicClosure F
local instance : DecidableEq K := Classical.decEq K
abbrev curve := binaryCounterexampleCurve
abbrev geomCurve := curve.baseChange K

theorem rational_two_point_nonsingular : curve.Nonsingular (0:F) 1 := by
  rw [WeierstrassCurve.Affine.nonsingular_iff,WeierstrassCurve.Affine.equation_iff]
  decide

def rationalTwoPoint : curve.Point := .some 0 1 rational_two_point_nonsingular

def geometricTwoPoint : geomCurve.Point := .some 0 1 (by
  rw [WeierstrassCurve.Affine.nonsingular_iff,WeierstrassCurve.Affine.equation_iff]
  norm_num [geomCurve,curve,binaryCounterexampleCurve,WeierstrassCurve.baseChange,
    WeierstrassCurve.map])

theorem rational_curve_cardinality : Nat.card curve.Point=4 := by
  rw [Nat.card_congr curve.nonsingularPointEquiv]
  change Nat.card (Option {xy : F × F // curve.Nonsingular xy.1 xy.2})=4
  letI : DecidablePred (fun xy : F × F => curve.Nonsingular xy.1 xy.2) := fun xy =>
    decidable_of_iff (xy.2^2+xy.1*xy.2=xy.1^3+1) (by
      change xy.2^2+xy.1*xy.2=xy.1^3+1 ↔ curve.Nonsingular xy.1 xy.2
      erw [← WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
        binaryCounterexample_nonsingular,WeierstrassCurve.Affine.equation_iff]
      simp [curve,binaryCounterexampleCurve])
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem rationalTwoPoint_nonzero : rationalTwoPoint≠0 := by
  intro h
  have hx := congrArg pointX h
  change some (0:F)=none at hx
  cases hx

theorem rationalTwoPoint_killed_by_two : (2:ℕ) • rationalTwoPoint=0 := by
  rw [two_nsmul]
  apply WeierstrassCurve.Affine.Point.add_self_of_Y_eq
  decide

theorem geometric_two_torsion_classification :
    ∀ P : geomCurve.Point, (2:ℕ) • P=0 →
      P=0 ∨ P=geometricTwoPoint := by
  intro P ht
  cases P with
  | zero => exact Or.inl rfl
  | some x y h =>
    have hs : WeierstrassCurve.Affine.Point.some x y h=
        -WeierstrassCurve.Affine.Point.some x y h := by
      exact eq_neg_of_add_eq_zero_left (by simpa only [two_nsmul] using ht)
    have hy : y=geomCurve.negY x y := by
      rw [WeierstrassCurve.Affine.Point.neg_some] at hs
      exact (WeierstrassCurve.Affine.Point.some.inj hs).2
    have hn : geomCurve.negY x y=y+x := by
      apply negY_binary geomCurve
      · simp [geomCurve,curve,binaryCounterexampleCurve,WeierstrassCurve.baseChange,WeierstrassCurve.map]
      · simp [geomCurve,curve,binaryCounterexampleCurve,WeierstrassCurve.baseChange,WeierstrassCurve.map]
    have hx : x=0 := by
      rw [hn] at hy
      exact left_eq_add.mp hy
    subst x
    have hsq : y^2=(1:K)^2 := by
      have he := h.1
      rw [WeierstrassCurve.Affine.equation_iff] at he
      simpa [geomCurve,curve,binaryCounterexampleCurve,WeierstrassCurve.baseChange,WeierstrassCurve.map] using he
    have hy1 : y=1 := by
      rcases sq_eq_sq_iff_eq_or_eq_neg.mp hsq with he | he
      · exact he
      · simpa only [CharTwo.neg_eq] using he
    subst y
    exact Or.inr rfl

theorem geometric_two_torsion_all_rational :
    ∀ P : geomCurve.Point, (2:ℕ) • P=0 →
      P ∈ Set.range (WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId F K)) := by
  intro P ht
  rcases geometric_two_torsion_classification P ht with hp | hp
  · subst P
    exact ⟨0,map_zero _⟩
  · subst P
    refine ⟨rationalTwoPoint,?_⟩
    simp [geometricTwoPoint,rationalTwoPoint,WeierstrassCurve.Affine.Point.map]

theorem no_cyclotomic_divisibility : ¬(2:ℕ) ∣ 2^1-1 := by decide

theorem characteristic_torsion_field_counterexample :
    Nat.Prime 2 ∧ curve.Δ≠0 ∧ Nat.card curve.Point=4 ∧ (2:ℕ) ∣ Nat.card curve.Point ∧
    (∀ P : geomCurve.Point, (2:ℕ) • P=0 →
      P ∈ Set.range (WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId F K))) ∧
    ¬(2:ℕ) ∣ 2^1-1 :=
  ⟨by decide,binaryCounterexample_nonsingular,rational_curve_cardinality,
    by rw [rational_curve_cardinality]; decide,
    geometric_two_torsion_all_rational,no_cyclotomic_divisibility⟩

theorem geometricTwoPoint_nonzero : geometricTwoPoint≠0 := by
  intro he
  have hx := congrArg pointX he
  have hg : pointX geometricTwoPoint=some (0:K) := by
    rfl
  rw [hg] at hx
  cases hx

theorem geometricTwoPoint_killed_by_two : (2:ℕ) • geometricTwoPoint=0 := by
  rw [two_nsmul]
  apply WeierstrassCurve.Affine.Point.add_self_of_Y_eq
  simp [geomCurve,curve,binaryCounterexampleCurve,WeierstrassCurve.Affine.negY,
    WeierstrassCurve.baseChange,WeierstrassCurve.map,CharTwo.neg_eq]

def TwoTorsion := {P : geomCurve.Point // (2:ℕ) • P=0}
def MuTwo := {z : K // z^2=1}

theorem geometric_two_torsion_cardinality : Nat.card TwoTorsion=2 := by
  apply Nat.card_eq_two_iff.mpr
  let A : TwoTorsion := ⟨0,by simp⟩
  let B : TwoTorsion := ⟨geometricTwoPoint,geometricTwoPoint_killed_by_two⟩
  refine ⟨A,B,?_,?_⟩
  · intro he
    exact geometricTwoPoint_nonzero (congrArg Subtype.val he).symm
  · apply Set.ext
    intro P
    simp only [Set.mem_insert_iff,Set.mem_singleton_iff,Set.mem_univ,iff_true]
    rcases geometric_two_torsion_classification P.1 P.2 with hp | hp
    · left
      exact Subtype.ext hp
    · right
      exact Subtype.ext hp

theorem muTwo_value_one (z : MuTwo) : z.1=1 := by
  rcases sq_eq_one_iff.mp z.2 with h | h
  · exact h
  · simpa only [CharTwo.neg_eq] using h

theorem muTwo_cardinality : Nat.card MuTwo=1 := by
  apply Nat.card_eq_one_iff_exists.mpr
  refine ⟨⟨1,by norm_num⟩,?_⟩
  intro z
  exact Subtype.ext (muTwo_value_one z)

def LeftNondegenerate (e : TwoTorsion → TwoTorsion → MuTwo) : Prop :=
  ∀ P, (∀ Q, (e P Q).1=1) → P.1=0

/-- Every point-valued mu2 map is degenerate on this actual geometric
torsion group, independently of bilinearity. Group schemes are not asserted. -/
theorem no_pointwise_nondegenerate_pairing :
    ∀ e : TwoTorsion → TwoTorsion → MuTwo, ¬ LeftNondegenerate e := by
  intro e he
  let P : TwoTorsion := ⟨geometricTwoPoint,geometricTwoPoint_killed_by_two⟩
  exact geometricTwoPoint_nonzero (he P (fun Q=>muTwo_value_one (e P Q)))

theorem characteristic_boundary_certificate :
    curve.Δ≠0 ∧ Nat.card curve.Point=4 ∧ Nat.card TwoTorsion=2 ∧ Nat.card MuTwo=1 ∧
    (∀ P : geomCurve.Point, (2:ℕ) • P=0 →
      P ∈ Set.range (WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId F K))) ∧
    ¬(2:ℕ) ∣ 2^1-1 ∧
    ∀ e : TwoTorsion → TwoTorsion → MuTwo, ¬ LeftNondegenerate e :=
  ⟨binaryCounterexample_nonsingular,rational_curve_cardinality,
    geometric_two_torsion_cardinality,muTwo_cardinality,
    geometric_two_torsion_all_rational,no_cyclotomic_divisibility,
    no_pointwise_nondegenerate_pairing⟩

#print axioms characteristic_torsion_field_counterexample
#print axioms geometric_two_torsion_cardinality
#print axioms no_pointwise_nondegenerate_pairing
#print axioms characteristic_boundary_certificate

end
end CharacteristicTorsionAudit
