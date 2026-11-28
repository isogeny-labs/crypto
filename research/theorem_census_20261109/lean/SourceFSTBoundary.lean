import TorsionBoundaries
import Mathlib.FieldTheory.IntermediateField.Adjoin.Basic

/-! FST2010 Definition2.1 uses K(mu_r), not the full r-torsion field.
At r=2 those fields need not agree. The concrete curve below is an actual
Algorithm3.3(1) output. The paragraph on p239 incorrectly calls its
cyclotomic embedding degree two; here that extension degree is one.
No claim about the paper's large-prime construction costs is made. -/
namespace FSTBoundaryAudit
open TorsionBoundaryAudit
private instance primeSeven : Fact (Nat.Prime 7) := ⟨by decide⟩
abbrev F := ZMod 7
abbrev K := AlgebraicClosure F
noncomputable section
local instance : DecidableEq K := Classical.decEq K

def curve : WeierstrassCurve.Affine F := ⟨0,0,0,1,0⟩
theorem curve_nonsingular : curve.Δ≠0 := by decide
theorem neg_one_nonsquare : ¬∃ a:F, a^2= -1 := by decide

theorem curve_cardinality : Nat.card curve.Point=8 := by
  rw [Nat.card_congr curve.nonsingularPointEquiv]
  change Nat.card (Option {xy:F×F // curve.Nonsingular xy.1 xy.2})=8
  letI : DecidablePred (fun xy:F×F => curve.Nonsingular xy.1 xy.2) := fun xy =>
    decidable_of_iff (xy.2^2=xy.1^3+xy.1)
      (by change xy.2^2=xy.1^3+xy.1 ↔ curve.Nonsingular xy.1 xy.2
          erw [← WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
            curve_nonsingular,WeierstrassCurve.Affine.equation_iff]
          simp [curve])
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem curve_source_supersingular : sourceSupersingular 7 curve := by
  norm_num [sourceSupersingular,finiteFieldTrace,curve_cardinality]

def twoPoint : curve.Point := .some 0 0 (by
  rw [WeierstrassCurve.Affine.nonsingular_iff]
  constructor
  · rw [WeierstrassCurve.Affine.equation_iff]; decide
  · decide)

theorem twoPoint_order : addOrderOf twoPoint=2 := by
  letI : Fact (Nat.Prime 2) := ⟨by decide⟩
  apply addOrderOf_eq_prime
  · rw [two_nsmul]
    change WeierstrassCurve.Affine.Point.some 0 0 _ +
      WeierstrassCurve.Affine.Point.some 0 0 _=0
    exact WeierstrassCurve.Affine.Point.add_self_of_Y_eq (by decide)
  · decide

/-- In any field extension, every square root of unity lies in the base.
This is an actual field-adjoin result, not an assumed identification with
the order of q modulo r. -/
theorem roots2_adjoin_eq_bot {L M : Type*} [Field L] [Field M] [Algebra L M] :
    IntermediateField.adjoin L {ζ:M | ζ^2=1}=⊥ := by
  apply le_antisymm
  · apply IntermediateField.adjoin_le_iff.mpr
    intro ζ hζ
    apply IntermediateField.mem_bot.mpr
    rcases sq_eq_one_iff.mp hζ with h | h
    · exact ⟨1,by simpa using h.symm⟩
    · exact ⟨-1,by simpa using h.symm⟩
  · exact bot_le

def roots2Field : IntermediateField F K :=
  IntermediateField.adjoin F {ζ:K | ζ^2=1}

theorem roots2_extension_degree : Module.finrank F roots2Field=1 := by
  unfold roots2Field
  rw [roots2_adjoin_eq_bot]
  exact IntermediateField.finrank_bot

theorem two_torsion_not_all_rational : ∃ P : (curve.baseChange K).Point,
    (2:ℕ) • P=0 ∧
    P ∉ Set.range (WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId F K)) := by
  obtain ⟨α,hα⟩ := IsAlgClosed.exists_pow_nat_eq (-1:K) (n:=2) (by decide)
  have hnot : ¬∃ a:F,algebraMap F K a=α := by
    rintro ⟨a,ha⟩
    have hh : algebraMap F K (a^2)=algebraMap F K (-1) := by
      rw [map_pow,ha,map_neg,map_one]
      exact hα
    exact neg_one_nonsquare ⟨a,(algebraMap F K).injective hh⟩
  let E := curve.baseChange K
  have hΔ : E.Δ≠0 := by
    change (curve.map (algebraMap F K)).Δ≠0
    rw [WeierstrassCurve.map_Δ]
    exact (map_ne_zero (algebraMap F K)).mpr curve_nonsingular
  have he : E.Equation α 0 := by
    rw [WeierstrassCurve.Affine.equation_iff]
    simp only [E,curve,WeierstrassCurve.baseChange,WeierstrassCurve.map,
      map_zero,map_one,mul_zero,zero_mul,add_zero,zero_add,zero_pow (by decide : (2:ℕ)≠0),one_mul]
    linear_combination -α*hα
  have hn := (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp he
  let P : E.Point := .some α 0 hn
  refine ⟨P,?_,?_⟩
  · rw [two_nsmul]
    exact WeierstrassCurve.Affine.Point.add_self_of_Y_eq (by
      simp [E,curve,WeierstrassCurve.baseChange,WeierstrassCurve.map,
        WeierstrassCurve.Affine.negY])
  · rintro ⟨Q,hQ⟩
    cases Q with
    | zero =>
      have hx := congrArg SemaevS3.pointX hQ
      change none=some α at hx
      cases hx
    | some x y hy =>
      have hx := congrArg SemaevS3.pointX hQ
      simp [WeierstrassCurve.Affine.Point.map,P,SemaevS3.pointX] at hx
      exact hnot ⟨x,hx⟩

/-- Concrete p239 boundary: the curve meets Algorithm3.3(1), has a rational
point of order2, and has nonrational 2-torsion, yet [F7(mu2):F7]=1. -/
theorem fst_algorithm3_3_r_two_counterexample :
    curve.Δ≠0 ∧ sourceSupersingular 7 curve ∧
      (¬∃ a:F,a^2= -1) ∧ addOrderOf twoPoint=2 ∧ Nat.Coprime 2 7 ∧
      Module.finrank F roots2Field=1 ∧
      ∃ P : (curve.baseChange K).Point, (2:ℕ) • P=0 ∧
        P ∉ Set.range (WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId F K)) := by
  exact ⟨curve_nonsingular,curve_source_supersingular,neg_one_nonsquare,twoPoint_order,
    by decide,roots2_extension_degree,two_torsion_not_all_rational⟩
end
end FSTBoundaryAudit
