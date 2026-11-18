import GeneralS3
import BinaryS3
import Mathlib.FieldTheory.IsAlgClosed.AlgebraicClosure

/-! Faithful small-characteristic model-domain controls for the general S3.
These do not refute formulas on their stated normal forms. -/
namespace GeneralS3Audit
noncomputable section
open SemaevS3
variable {K : Type*} [Field K]

theorem generalS3_binary [CharP K 2] (W : WeierstrassCurve.Affine K)
    (ha1 : W.a₁=1) (ha3 : W.a₃=0) (ha4 : W.a₄=0) (x y z : K) :
    generalS3 W x y z=BinaryS3Audit.binaryS3 W.a₆ x y z := by
  have h4 : (4:K)=0 := by linear_combination 2*(CharTwo.two_eq_zero (R:=K))
  have h2 : W.b₂=1 := by simp [WeierstrassCurve.b₂,ha1,h4]
  have hb4 : W.b₄=0 := by simp [WeierstrassCurve.b₄,ha1,ha3,ha4]
  have h6 : W.b₆=0 := by simp [WeierstrassCurve.b₆,ha3,h4]
  have h8 : W.b₈=W.a₆ := by simp [WeierstrassCurve.b₈,ha1,ha3,ha4,h4]
  rw [generalS3_vieta,h2,hb4,h6,h8]
  simp [h4,CharTwo.sub_eq_add,CharTwo.neg_eq,BinaryS3Audit.binaryS3]

theorem no_geometric_sum_of_nonzero {L : Type*} [Field L] [Algebra K L]
    [DecidableEq L] [IsAlgClosed L] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (x y z : K) (h : generalS3 W x y z≠0) :
    ¬ ∃ P Q R : (W.baseChange L).Point,
      pointX P=some (algebraMap K L x) ∧ pointX Q=some (algebraMap K L y) ∧
      pointX R=some (algebraMap K L z) ∧ P+Q+R=0 := by
  have hΔ' : (W.baseChange L).Δ≠0 := by
    change (W.map (algebraMap K L)).Δ≠0
    rw [WeierstrassCurve.map_Δ]
    exact (map_ne_zero (algebraMap K L)).mpr hΔ
  intro hp
  have hz := (generalS3_zero_iff_exists_points (W.baseChange L) hΔ'
    (algebraMap K L x) (algebraMap K L y) (algebraMap K L z)).mpr hp
  change generalS3 (W.map (algebraMap K L)) _ _ _=0 at hz
  rw [generalS3_map] at hz
  exact ((map_ne_zero (algebraMap K L)).mpr h) hz

private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩
abbrev K2 := AlgebraicClosure (ZMod 2)
abbrev K3 := AlgebraicClosure (ZMod 3)
local instance : DecidableEq K2 := Classical.decEq _
local instance : DecidableEq K3 := Classical.decEq _

def nonordinaryTwo : WeierstrassCurve.Affine (ZMod 2) := ⟨0,0,1,0,0⟩
def longThree : WeierstrassCurve.Affine (ZMod 3) := ⟨0,1,0,0,1⟩

theorem nonordinaryTwo_nonsingular : nonordinaryTwo.Δ≠0 := by decide
theorem longThree_nonsingular : longThree.Δ≠0 := by decide

theorem nonordinaryTwo_polynomials_disagree :
    BinaryS3Audit.binaryS3 nonordinaryTwo.a₆ (1:ZMod 2) 0 0=0 ∧
    generalS3 nonordinaryTwo (1:ZMod 2) 0 0≠0 := by decide

theorem longThree_polynomials_disagree :
    SemaevS3.S3 longThree.a₄ longThree.a₆ (0:ZMod 3) 0 0=0 ∧
    generalS3 longThree (0:ZMod 3) 0 0≠0 := by decide

/-- The ordinary binary formula gives a false geometric zero on this
actual nonsingular nonordinary characteristic-two model. -/
theorem nonordinary_binary_extension_counterexample :
    nonordinaryTwo.Δ≠0 ∧
    BinaryS3Audit.binaryS3 nonordinaryTwo.a₆ (1:ZMod 2) 0 0=0 ∧
    ¬ ∃ P Q R : (nonordinaryTwo.baseChange K2).Point,
      pointX P=some 1 ∧ pointX Q=some 0 ∧ pointX R=some 0 ∧ P+Q+R=0 := by
  refine ⟨nonordinaryTwo_nonsingular,nonordinaryTwo_polynomials_disagree.1,?_⟩
  simpa only [map_one,map_zero] using
    no_geometric_sum_of_nonzero (L:=K2) nonordinaryTwo
      nonordinaryTwo_nonsingular 1 0 0 nonordinaryTwo_polynomials_disagree.2

/-- The short formula gives a false geometric zero on an actual smooth
characteristic-three model with a2=1. The general formula is nonzero. -/
theorem characteristic_three_extension_counterexample :
    longThree.Δ≠0 ∧ SemaevS3.S3 longThree.a₄ longThree.a₆ (0:ZMod 3) 0 0=0 ∧
    ¬ ∃ P Q R : (longThree.baseChange K3).Point,
      pointX P=some 0 ∧ pointX Q=some 0 ∧ pointX R=some 0 ∧ P+Q+R=0 := by
  refine ⟨longThree_nonsingular,longThree_polynomials_disagree.1,?_⟩
  simpa only [map_zero] using
    no_geometric_sum_of_nonzero (L:=K3) longThree longThree_nonsingular
      0 0 0 longThree_polynomials_disagree.2

/-- A geometric p-torsion control for the full nonordinary binary family,
stated directly on actual points rather than an assumed trace criterion. -/
theorem char_two_a1_zero_two_torsion_trivial {F : Type*} [Field F] [CharP F 2]
    [DecidableEq F] (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0) (ha1 : W.a₁=0) :
    ∀ P : W.Point, (2:ℕ) • P=0 → P=0 := by
  have h2 : (2:F)=0 := CharTwo.two_eq_zero
  have h4 : (4:F)=0 := by linear_combination 2*h2
  have ha3 : W.a₃≠0 := by
    intro hh
    apply hΔ
    simp [WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
      WeierstrassCurve.b₆,ha1,hh,h2,h4]
  intro P ht
  cases P with
  | zero => rfl
  | some x y hp =>
    have hs : WeierstrassCurve.Affine.Point.some x y hp=
        -WeierstrassCurve.Affine.Point.some x y hp :=
      eq_neg_of_add_eq_zero_left (by simpa only [two_nsmul] using ht)
    rw [WeierstrassCurve.Affine.Point.neg_some] at hs
    have he := (WeierstrassCurve.Affine.Point.some.inj hs).2
    have hz : W.a₃=0 := by
      simp only [WeierstrassCurve.Affine.negY,ha1,zero_mul,sub_zero] at he
      linear_combination he-y*h2
    exact (ha3 hz).elim

/-- The corresponding characteristic-three short-model p-torsion control.
This is geometric over any algebraically closed extension as well. -/
theorem char_three_short_three_torsion_trivial {F : Type*} [Field F] [CharP F 3]
    [DecidableEq F] (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0)
    (ha1 : W.a₁=0) (ha2 : W.a₂=0) (ha3 : W.a₃=0) :
    ∀ P : W.Point, (3:ℕ) • P=0 → P=0 := by
  have h3 : (3:F)=0 := CharP.cast_eq_zero F 3
  have h6 : (6:F)=0 := by linear_combination 2*h3
  have h12 : (12:F)=0 := by linear_combination 4*h3
  have h27 : (27:F)=0 := by linear_combination 9*h3
  have ha4 : W.a₄≠0 := by
    intro hh
    apply hΔ
    simp [WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
      WeierstrassCurve.b₆,WeierstrassCurve.b₈,ha1,ha2,ha3,hh,h27]
  intro P ht
  cases P with
  | zero => rfl
  | some x y hp =>
    have hz := three_torsion_diagonal_zero W hp ht
    rw [generalS3_short W ha1 ha2 ha3] at hz
    have hd : SemaevS3.S3 W.a₄ W.a₆ x x x=W.a₄^2 := by
      unfold SemaevS3.S3
      ring_nf
      simp [h3,h6,h12]
    rw [hd] at hz
    exact (pow_ne_zero 2 ha4 hz).elim

theorem model_domain_counterexamples :
    (nonordinaryTwo.Δ≠0 ∧ BinaryS3Audit.binaryS3 nonordinaryTwo.a₆ (1:ZMod 2) 0 0=0 ∧
      ¬ ∃ P Q R : (nonordinaryTwo.baseChange K2).Point,
        pointX P=some 1 ∧ pointX Q=some 0 ∧ pointX R=some 0 ∧ P+Q+R=0) ∧
    (longThree.Δ≠0 ∧ SemaevS3.S3 longThree.a₄ longThree.a₆ (0:ZMod 3) 0 0=0 ∧
      ¬ ∃ P Q R : (longThree.baseChange K3).Point,
        pointX P=some 0 ∧ pointX Q=some 0 ∧ pointX R=some 0 ∧ P+Q+R=0) :=
  ⟨nonordinary_binary_extension_counterexample,characteristic_three_extension_counterexample⟩

#print axioms nonordinary_binary_extension_counterexample
#print axioms characteristic_three_extension_counterexample
#print axioms char_two_a1_zero_two_torsion_trivial
#print axioms char_three_short_three_torsion_trivial
end
end GeneralS3Audit
