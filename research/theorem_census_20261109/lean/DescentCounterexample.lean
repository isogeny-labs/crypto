import Mathlib.Algebra.Polynomial.SpecificDegree
import Mathlib.FieldTheory.Finiteness
import Mathlib.RingTheory.AdjoinRoot
import Mathlib.RingTheory.MvPolynomial.Homogeneous
import Mathlib.RingTheory.Regular.RegularSequence
import S3Roots

/-! The two-coordinate Weil descent of a base-defined symmetrized S3
equation is [f,0]. This refutes a universal all-input regularity reading,
not a generic-instance assertion. Source: FGHR2014, Definition5/Hypothesis3.
The quotient representative map supplies actual extension coordinates. -/
namespace SkepticalAudit
open Polynomial MvPolynomial
noncomputable section
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩

abbrev F := ZMod 5
def extensionPolynomial : Polynomial F := Polynomial.X^2-Polynomial.C 2

theorem extensionPolynomial_monic : extensionPolynomial.Monic := by
  exact Polynomial.monic_X_pow_sub_C _ (by decide)

theorem extensionPolynomial_degree : extensionPolynomial.natDegree=2 := by
  simp [extensionPolynomial]

theorem extensionPolynomial_irreducible : Irreducible extensionPolynomial := by
  apply Polynomial.irreducible_of_degree_le_three_of_not_isRoot
  · simp [extensionPolynomial]
  · intro x hx
    have hh : x^2-2 = (0:F) := by simpa [Polynomial.IsRoot,extensionPolynomial] using hx
    exact (by decide : ∀ x:F, x^2-2 ≠ 0) x hh

instance : Fact (Irreducible extensionPolynomial) := ⟨extensionPolynomial_irreducible⟩
abbrev K := AdjoinRoot extensionPolynomial

-- Field structure is obtained from the preceding irreducibility theorem.
example : Field K := inferInstance

theorem extension_finrank : Module.finrank F K=2 := by
  rw [(AdjoinRoot.powerBasis extensionPolynomial_monic.ne_zero).finrank]
  exact extensionPolynomial_degree

theorem extension_cardinality : Nat.card K=25 := by
  letI : Module.Finite F K :=
    (AdjoinRoot.powerBasis extensionPolynomial_monic.ne_zero).finite
  rw [Module.natCard_eq_pow_finrank (K := F),extension_finrank]
  norm_num [F,Nat.card_zmod]

noncomputable def coordinate (i : ℕ) (z : K) : F :=
  (AdjoinRoot.modByMonicHom extensionPolynomial_monic z).coeff i

theorem coordinate_constant (c : F) (i : ℕ) :
    coordinate i (algebraMap F K c)=if i=0 then c else 0 := by
  rw [show algebraMap F K c=AdjoinRoot.mk extensionPolynomial (Polynomial.C c) from
    (AdjoinRoot.mk_C c).symm]
  unfold coordinate
  rw [AdjoinRoot.modByMonicHom_mk]
  have hdeg : (Polynomial.C c).degree < extensionPolynomial.degree := by
    apply lt_of_le_of_lt Polynomial.degree_C_le
    rw [Polynomial.degree_eq_natDegree extensionPolynomial_monic.ne_zero,
      extensionPolynomial_degree]
    decide
  rw [(Polynomial.modByMonic_eq_self_iff extensionPolynomial_monic).mpr hdeg]
  simp [Polynomial.coeff_C]

noncomputable def descend (i : ℕ) (g : MvPolynomial (Fin 2) K) : MvPolynomial (Fin 2) F :=
  .ofCoeff (Finsupp.mapRange (coordinate i) (by simp [coordinate]) (AddMonoidAlgebra.coeff g))

theorem coeff_descend (i : ℕ) (s : Fin 2 →₀ ℕ) (g : MvPolynomial (Fin 2) K) :
    (descend i g).coeff s=coordinate i (g.coeff s) := rfl

theorem descend_base_polynomial (g : MvPolynomial (Fin 2) F) :
    descend 0 (MvPolynomial.map (algebraMap F K) g)=g ∧
      descend 1 (MvPolynomial.map (algebraMap F K) g)=0 := by
  constructor
  · ext s
    rw [coeff_descend,MvPolynomial.coeff_map]
    change coordinate 0 (algebraMap F K (g.coeff s))=g.coeff s
    simpa using coordinate_constant (g.coeff s) 0
  · ext s
    rw [coeff_descend,MvPolynomial.coeff_map]
    change coordinate 1 (algebraMap F K (g.coeff s))=0
    simpa using coordinate_constant (g.coeff s) 1

def descentCurve : WeierstrassCurve.Affine F := ⟨0,0,0,1,1⟩
theorem descentCurve_nonsingular : descentCurve.Δ ≠ 0 := by decide
def target : descentCurve.Point := .some 2 1 (by
  rw [WeierstrassCurve.Affine.nonsingular_iff]
  constructor
  · rw [WeierstrassCurve.Affine.equation_iff]; decide
  · decide)
theorem target_nonzero : target ≠ 0 := by decide
theorem target_three_torsion : (3:ℕ) • target=0 := by
  rw [show (3:ℕ)=1+1+1 by decide,add_nsmul,add_nsmul,one_nsmul]
  have hd : target+target = -target := by
    change WeierstrassCurve.Affine.Point.some 2 1 _ +
      WeierstrassCurve.Affine.Point.some 2 1 _ =
      -WeierstrassCurve.Affine.Point.some 2 1 _
    rw [WeierstrassCurve.Affine.Point.add_self_of_Y_ne (by decide),
      WeierstrassCurve.Affine.Point.neg_some]
    rw [WeierstrassCurve.Affine.Point.some.injEq]
    have hneg : (1:F) ≠ -1 := by decide
    have hdiv : (13:F)/2=4 :=
      (div_eq_iff (by decide : (2:F)≠0)).mpr (by decide)
    constructor <;> norm_num [WeierstrassCurve.Affine.addX,
      WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
      WeierstrassCurve.Affine.slope,WeierstrassCurve.Affine.negY,descentCurve,hneg,hdiv]
      <;> decide
  rw [hd,neg_add_cancel]

noncomputable def extendedTarget : (descentCurve.baseChange K).Point :=
  WeierstrassCurve.Affine.Point.map (W' := descentCurve) (Algebra.ofId F K) target

theorem extendedCurve_nonsingular : (descentCurve.baseChange K).Δ ≠ 0 := by
  change (descentCurve.map (algebraMap F K)).Δ ≠ 0
  rw [WeierstrassCurve.map_Δ]
  exact (_root_.map_ne_zero (algebraMap F K)).mpr descentCurve_nonsingular

theorem extendedTarget_nonzero : extendedTarget ≠ 0 := by
  intro h
  apply target_nonzero
  exact WeierstrassCurve.Affine.Point.map_injective (W' := descentCurve)
    (Algebra.ofId F K) (h.trans (map_zero _).symm)

theorem extendedTarget_three_torsion : (3:ℕ) • extendedTarget=0 := by
  unfold extendedTarget
  erw [← map_nsmul,target_three_torsion,map_zero]

theorem extendedTarget_x : SemaevS3.pointX extendedTarget=some (2:K) := by
  simp [extendedTarget,target,WeierstrassCurve.Affine.Point.map,SemaevS3.pointX,
    map_ofNat]

-- e1=x1+x2, e2=x1*x2, a=b=1 and target x=2.
theorem symmetrization_universal {L : Type*} [Field L] (x y : L) :
    SemaevS3.S3 1 1 x y 2 =
      ((x+y)^2-4*(x*y))*4 -4*((x+y)*(x*y+1)+2)
        +(x*y-1)^2-4*(x+y) := by
  unfold SemaevS3.S3
  ring

noncomputable def symmetricS3 : MvPolynomial (Fin 2) F :=
  (MvPolynomial.X 0^2-4*MvPolynomial.X 1)*4
    -4*(MvPolynomial.X 0*(MvPolynomial.X 1+1)+2)
    +(MvPolynomial.X 1-1)^2-4*MvPolynomial.X 0

theorem symmetrization_identity (x y : F) :
    SemaevS3.S3 1 1 x y 2=MvPolynomial.eval ![x+y,x*y] symmetricS3 := by
  simp [SemaevS3.S3,symmetricS3]
  ring

theorem symmetrization_over_extension (x y : K) :
    SemaevS3.S3 1 1 x y 2=
      MvPolynomial.eval₂ (algebraMap F K) ![x+y,x*y] symmetricS3 := by
  simp [SemaevS3.S3,symmetricS3]
  ring

theorem symmetricS3_nonzero : symmetricS3 ≠ 0 := by
  intro h
  have hh := congrArg (MvPolynomial.eval (0:Fin 2→F)) h
  norm_num [symmetricS3,map_ofNat] at hh
  exact (by decide : ¬ (7:F)=0) hh

/-- An actual base-defined equation in the degree-two field has zero second
coordinate under the monic remainder representation of that field. -/
theorem actual_descended_equations :
    [descend 0 (MvPolynomial.map (algebraMap F K) symmetricS3),
     descend 1 (MvPolynomial.map (algebraMap F K) symmetricS3)] = [symmetricS3,0] := by
  rw [(descend_base_polynomial symmetricS3).1,(descend_base_polynomial symmetricS3).2]

theorem zero_second_not_regular {R : Type*} [CommRing R] (f : R) :
    ¬ RingTheory.Sequence.IsRegular R [f,0] := by
  intro h
  have tail := (RingTheory.Sequence.isRegular_cons_iff R f [0]).mp h |>.2
  haveI := tail.nontrivial
  have hz := (RingTheory.Sequence.isRegular_cons_iff _ (0:R) []).mp tail |>.1
  exact IsSMulRegular.not_zero hz

theorem actual_descent_not_regular :
    ¬ RingTheory.Sequence.IsRegular (MvPolynomial (Fin 2) F)
      [descend 0 (MvPolynomial.map (algebraMap F K) symmetricS3),
       descend 1 (MvPolynomial.map (algebraMap F K) symmetricS3)] := by
  rw [actual_descended_equations]
  exact zero_second_not_regular symmetricS3

/-- FGHR Definition5 requires nonzero input polynomials and a regular sequence
of top homogeneous parts. The actual second equation is zero and cannot meet
that definition. This also proves nonregularity of the top-part sequence. -/
theorem actual_descent_top_parts_not_regular :
    ¬ RingTheory.Sequence.IsRegular (MvPolynomial (Fin 2) F)
      [MvPolynomial.homogeneousComponent symmetricS3.totalDegree symmetricS3,
       MvPolynomial.homogeneousComponent (0:ℕ) (0:MvPolynomial (Fin 2) F)] := by
  simp only [map_zero]
  exact zero_second_not_regular _

/-- Closed witness: the curve and target are valid, the extension really has
25 elements, and the actual two-coordinate symmetrized descent is nonregular.
This does not quantify only over a chosen list unrelated to a curve. -/
theorem universal_descent_regularity_counterexample :
    Nat.card K=25 ∧ (descentCurve.baseChange K).Δ ≠ 0 ∧ extendedTarget ≠ 0 ∧
      (3:ℕ) • extendedTarget=0 ∧ SemaevS3.pointX extendedTarget=some (2:K) ∧
      symmetricS3 ≠ 0 ∧
      ¬ RingTheory.Sequence.IsRegular (MvPolynomial (Fin 2) F)
        [descend 0 (MvPolynomial.map (algebraMap F K) symmetricS3),
         descend 1 (MvPolynomial.map (algebraMap F K) symmetricS3)] :=
  ⟨extension_cardinality,extendedCurve_nonsingular,extendedTarget_nonzero,
    extendedTarget_three_torsion,extendedTarget_x,symmetricS3_nonzero,
    actual_descent_not_regular⟩
end
end SkepticalAudit
