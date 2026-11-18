import DiemThreshold
import S3Roots
import Mathlib.FieldTheory.Finiteness
import Mathlib.FieldTheory.Finite.Basic
import Mathlib.RingTheory.AdjoinRoot
import Mathlib.Algebra.CharP.Algebra
import Mathlib.Algebra.Polynomial.SpecificDegree

/-! Actual small-field point/factor-base control. The size-qualified Diem
theorem is not refuted; scheme ramification is a separate obligation. -/
namespace DiemSmallFieldAudit
noncomputable section
open Polynomial
private instance primeTwo : Fact (Nat.Prime 2) := ⟨by decide⟩
abbrev B := ZMod 2
def extensionPolynomial : B[X] := X^2+X+1

theorem extensionPolynomial_degree : extensionPolynomial.natDegree=2 := by
  unfold extensionPolynomial
  compute_degree!

theorem extensionPolynomial_monic : extensionPolynomial.Monic := by
  change extensionPolynomial.leadingCoeff=1
  rw [Polynomial.leadingCoeff,extensionPolynomial_degree]
  simp [extensionPolynomial,Polynomial.coeff_one,Polynomial.coeff_X]

theorem extensionPolynomial_irreducible : Irreducible extensionPolynomial := by
  apply Polynomial.irreducible_of_degree_le_three_of_not_isRoot
  · rw [extensionPolynomial_degree]
    decide
  · intro x hx
    have hh : x^2+x+1=(0:B) := by
      simpa [Polynomial.IsRoot,extensionPolynomial] using hx
    exact (by decide : ∀ x:B,x^2+x+1≠0) x hh

instance : Fact (Irreducible extensionPolynomial) := ⟨extensionPolynomial_irreducible⟩
abbrev F := AdjoinRoot extensionPolynomial
instance : CharP F 2 := charP_of_injective_algebraMap' B 2

theorem field_finrank : Module.finrank B F=2 := by
  rw [(AdjoinRoot.powerBasis extensionPolynomial_monic.ne_zero).finrank]
  exact extensionPolynomial_degree

theorem field_cardinality : Nat.card F=4 := by
  let : Module.Finite B F :=
    (AdjoinRoot.powerBasis extensionPolynomial_monic.ne_zero).finite
  rw [Module.natCard_eq_pow_finrank (K:=B),field_finrank]
  norm_num [B,Nat.card_zmod]

instance : Finite F := (Nat.card_pos_iff.mp (by rw [field_cardinality]; decide)).2
noncomputable instance : Fintype F := Fintype.ofFinite F
local instance : DecidableEq F := Classical.decEq _

def alpha : F := AdjoinRoot.root extensionPolynomial

theorem alpha_relation : alpha^2+alpha+1=0 := by
  have h : AdjoinRoot.mk extensionPolynomial (X^2+X+1)=0 :=
    AdjoinRoot.mk_self (f:=extensionPolynomial)
  simpa only [map_add,map_pow,map_one,AdjoinRoot.mk_X,alpha] using h

theorem two_zero : (2:F)=0 := CharP.cast_eq_zero F 2

theorem alpha_not_fixed : alpha^2≠alpha := by
  intro h
  have hr := alpha_relation
  rw [h] at hr
  have hd : alpha+alpha=0 := by linear_combination alpha*two_zero
  have hone : (1:F)=0 := by linear_combination hr-hd
  exact one_ne_zero hone

theorem alpha_not_base : ¬ ∃ b:B,algebraMap B F b=alpha := by
  rintro ⟨b,hb⟩
  have hf : (algebraMap B F b)^2=algebraMap B F b := by
    rw [←map_pow]
    exact congrArg (algebraMap B F) ((by decide : ∀ b:B,b^2=b) b)
  exact alpha_not_fixed (by simpa only [hb] using hf)

theorem pow_four (x : F) : x^4=x := by
  have hc : Fintype.card F=4 := by
    rw [←Nat.card_eq_fintype_card]
    exact field_cardinality
  simpa only [hc] using FiniteField.pow_card x

theorem add_square (x y : F) : (x+y)^2=x^2+y^2 := by
  linear_combination x*y*two_zero

theorem trace_fixed (y : F) : (y^2+y)^2=y^2+y := by
  rw [add_square,←pow_mul,pow_four]
  ring

theorem cube_fixed (x : F) : (x^3)^2=x^3 := by
  by_cases hx : x=0
  · simp [hx]
  · have h3 : x^3=1 := by
      apply mul_left_cancel₀ hx
      calc
        x*x^3=x^4 := by ring
        _=x := pow_four x
        _=x*1 := by ring
    simp only [h3,one_pow]

def curve : WeierstrassCurve.Affine F := ⟨0,0,1,0,alpha⟩

theorem curve_discriminant : curve.Δ=1 := by
  have h4 : (4:F)=0 := by linear_combination 2*two_zero
  have h27 : (27:F)=1 := by linear_combination 13*two_zero
  have hneg : (-1:F)=1 := by linear_combination -two_zero
  norm_num [curve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
    WeierstrassCurve.b₆,WeierstrassCurve.b₈,h4,h27,hneg]
  change -((27:F)*(1+4*alpha)^2)=1
  rw [h4,h27]
  simpa only [zero_mul,add_zero,one_pow,one_mul] using hneg

theorem curve_nonsingular : curve.Δ≠0 := by rw [curve_discriminant]; exact one_ne_zero

theorem no_affine_point (x y : F) : ¬curve.Equation x y := by
  intro h
  rw [WeierstrassCurve.Affine.equation_iff] at h
  have he : y^2+y=x^3+alpha := by simpa only [curve,zero_mul,one_mul,zero_add,add_zero] using h
  have hs := congrArg (fun z : F=>z^2) he
  rw [trace_fixed,add_square,cube_fixed] at hs
  apply alpha_not_fixed
  linear_combination he-hs

theorem all_points_zero (P : curve.Point) : P=0 := by
  cases P with
  | zero => rfl
  | some x y h => exact (no_affine_point x y h.1).elim

theorem curve_cardinality : Nat.card curve.Point=1 := by
  apply Nat.card_eq_one_iff_exists.mpr
  exact ⟨0,all_points_zero⟩

section Mobius
variable {L : Type*} [Field L] [DecidableEq L]

/-- Affine-chart presentation of the homography z ↦ c+1/z; none is infinity. -/
def mobiusChart (c : L) : Option L→Option L
  | none => some c
  | some x => if x=0 then none else some (c+x⁻¹)

def inverseChart (c : L) : Option L→Option L
  | none => some 0
  | some y => if y=c then none else some ((y-c)⁻¹)

theorem inverse_mobius (c : L) (z : Option L) :
    inverseChart c (mobiusChart c z)=z := by
  cases z with
  | none => simp [mobiusChart,inverseChart]
  | some x =>
    by_cases hx : x=0
    · subst x
      simp [mobiusChart,inverseChart]
    · have hsum : c+x⁻¹≠c := by
        intro he
        apply inv_ne_zero hx
        have he' : c+x⁻¹=c+0 := by simpa only [add_zero] using he
        exact add_left_cancel he'
      simp only [mobiusChart,ite_eq_right hx,inverseChart,ite_eq_right hsum]
      congr 1
      rw [add_sub_cancel_left,inv_inv]

theorem mobius_inverse (c : L) (z : Option L) :
    mobiusChart c (inverseChart c z)=z := by
  cases z with
  | none => simp [mobiusChart,inverseChart]
  | some y =>
    by_cases hy : y=c
    · subst y
      simp [mobiusChart,inverseChart]
    · have hi : (y-c)⁻¹≠0 := inv_ne_zero (sub_ne_zero.mpr hy)
      simp only [inverseChart,ite_eq_right hy,mobiusChart,ite_eq_right hi,inv_inv]
      congr 1
      ring

theorem mobiusChart_bijective (c : L) : Function.Bijective (mobiusChart c) := by
  exact ⟨Function.LeftInverse.injective (inverse_mobius c),
    Function.RightInverse.surjective (mobius_inverse c)⟩

def mobiusX (W : WeierstrassCurve.Affine L) (c : L) (P : W.Point) : Option L :=
  mobiusChart c (SemaevS3.pointX P)

end Mobius

/-- The actual image of the base projective chart, including infinity. -/
def baseProjectiveImage : Set (Option F) := Set.range (Option.map (algebraMap B F))

def factorBase (W : WeierstrassCurve.Affine F) (c : F) : Set W.Point :=
  {P | mobiusX W c P∈baseProjectiveImage}

theorem alpha_not_base_projective : some alpha∉baseProjectiveImage := by
  rintro ⟨z,hz⟩
  cases z with
  | none => cases hz
  | some b => exact alpha_not_base ⟨b,Option.some.inj hz⟩

theorem factorBase_empty : factorBase curve alpha=∅ := by
  apply Set.ext
  intro P
  constructor
  · intro h
    change mobiusX curve alpha P∈baseProjectiveImage at h
    rw [all_points_zero P] at h
    exact (alpha_not_base_projective h).elim
  · intro h
    cases h

theorem factorBase_cardinality : Nat.card {P : curve.Point // P∈factorBase curve alpha}=0 := by
  simp [factorBase_empty]

theorem unrestricted_factor_base_bound_false :
    ¬ ∀ (W : WeierstrassCurve.Affine F) (c : F),W.Δ≠0 →
      Nat.card B+1≤2*Nat.card {P : W.Point // P∈factorBase W c} := by
  intro h
  have hc := h curve alpha curve_nonsingular
  rw [factorBase_cardinality] at hc
  norm_num [B,Nat.card_zmod] at hc

theorem small_size_excluded : ¬ (2^(7*2)≤(2:ℕ)) := by decide

theorem diem_domain_audit_certificate :
    (∀ n q N : ℕ,2≤n → 2^(7*n)≤q →
      (q:ℝ)+1-(n:ℝ)*2^(n+2)*(Real.sqrt (q:ℝ)+1)≤N → q+1≤2*N) ∧
    (¬ ∀ n q N : ℕ,2≤n →
      (q:ℝ)+1-(n:ℝ)*2^(n+2)*(Real.sqrt (q:ℝ)+1)≤N → q+1≤2*N) ∧
    (Nat.card B=2 ∧ Nat.card F=4 ∧ Module.finrank B F=2 ∧
      curve.Δ≠0 ∧ Nat.card curve.Point=1 ∧ Function.Bijective (mobiusChart alpha) ∧
      Nat.card {P : curve.Point // P∈factorBase curve alpha}=0) ∧
    (¬ ∀ (W : WeierstrassCurve.Affine F) (c : F),W.Δ≠0 →
      Nat.card B+1≤2*Nat.card {P : W.Point // P∈factorBase W c}) ∧
    ¬ (2^(7*2)≤(2:ℕ)) := by
  refine ⟨DiemThresholdAudit.diem_threshold_audit_certificate.1,
    DiemThresholdAudit.diem_threshold_audit_certificate.2,?_,
    unrestricted_factor_base_bound_false,small_size_excluded⟩
  exact ⟨by norm_num [B,Nat.card_zmod],field_cardinality,field_finrank,
    curve_nonsingular,curve_cardinality,mobiusChart_bijective alpha,factorBase_cardinality⟩

#print axioms field_cardinality
#print axioms alpha_not_base
#print axioms curve_nonsingular
#print axioms curve_cardinality
#print axioms mobiusChart_bijective
#print axioms factorBase_cardinality
#print axioms unrestricted_factor_base_bound_false
#print axioms diem_domain_audit_certificate
end
end DiemSmallFieldAudit
