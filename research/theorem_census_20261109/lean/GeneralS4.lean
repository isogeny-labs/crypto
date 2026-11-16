import GeneralS3
import Mathlib.RingTheory.Polynomial.Resultant.Basic
import Mathlib.FieldTheory.IsAlgClosed.AlgebraicClosure

/-! First resultant-recursion step, with actual Mathlib Sylvester determinant.
Kosters–Yeo2015 Section2/Proposition2.1. Fixed degree2 is retained under
specialization; simultaneous degree drops are treated separately. -/
namespace GeneralS4Audit
noncomputable section
open SemaevS3 GeneralS3Audit Polynomial
variable {K : Type*} [Field K]

def quadratic (a b c : K) : K[X] := C a*X^2+C b*X+C c

def res22 (a b c d e f : K) : K :=
  (a*f-c*d)^2-(a*e-b*d)*(b*f-c*e)

theorem quadratic_resultant (a b c d e f : K) :
    Polynomial.resultant (quadratic a b c) (quadratic d e f) 2 2=
      res22 a b c d e f := by
  have hm : Polynomial.sylvester (quadratic a b c) (quadratic d e f) 2 2=
      (!![f,0,c,0; e,f,b,c; d,e,a,b; 0,d,0,a] : Matrix (Fin 4) (Fin 4) K) := by
    ext i j
    fin_cases i <;> fin_cases j <;>
      norm_num [Polynomial.sylvester,quadratic,Polynomial.coeff_add,
        Polynomial.coeff_C_mul,Polynomial.coeff_X_pow,Polynomial.coeff_X,
        Polynomial.coeff_C,Fin.addCases]
  unfold Polynomial.resultant
  rw [hm,Matrix.det_succ_row_zero]
  norm_num [Fin.sum_univ_succ,Matrix.det_fin_three,Matrix.submatrix,Fin.succAbove]
  unfold res22
  ring

theorem res22_comm (a b c d e f : K) :
    res22 a b c d e f=res22 d e f a b c := by unfold res22; ring

theorem res22_factored (a r s d e f : K) :
    res22 a (-a*(r+s)) (a*r*s) d e f=
      a^2*(d*r^2+e*r+f)*(d*s^2+e*s+f) := by
  unfold res22
  ring

def qa (x y : K) : K := (x-y)^2
def qb (W : WeierstrassCurve.Affine K) (x y : K) : K :=
  -(2*x*y*(x+y)+W.b₂*x*y+W.b₄*(x+y)+W.b₆)
def qc (W : WeierstrassCurve.Affine K) (x y : K) : K :=
  (x*y)^2-W.b₄*x*y-W.b₆*(x+y)-W.b₈
def cubicInLast (W : WeierstrassCurve.Affine K) (x y : K) : K[X] :=
  quadratic (qa x y) (qb W x y) (qc W x y)

theorem cubicInLast_eval (W : WeierstrassCurve.Affine K) (x y t : K) :
    (cubicInLast W x y).eval t=generalS3 W x y t := by
  simp [cubicInLast,quadratic,qa,qb,qc,generalS3]
  ring

def generalS4 (W : WeierstrassCurve.Affine K) (x1 x2 x3 x4 : K) : K :=
  Polynomial.resultant (cubicInLast W x1 x2) (cubicInLast W x3 x4) 2 2

theorem generalS4_coeff (W : WeierstrassCurve.Affine K) (x1 x2 x3 x4 : K) :
    generalS4 W x1 x2 x3 x4=
      res22 (qa x1 x2) (qb W x1 x2) (qc W x1 x2)
        (qa x3 x4) (qb W x3 x4) (qc W x3 x4) :=
  quadratic_resultant ..

theorem generalS4_pair_swap (W : WeierstrassCurve.Affine K) (x1 x2 x3 x4 : K) :
    generalS4 W x1 x2 x3 x4=generalS4 W x3 x4 x1 x2 := by
  rw [generalS4_coeff,generalS4_coeff,res22_comm]

theorem generalS4_repeated_pairs (W : WeierstrassCurve.Affine K) (x y : K) :
    generalS4 W x x y y=0 := by
  rw [generalS4_coeff]
  simp [qa,res22]

theorem generalS3_as_quadratic (W : WeierstrassCurve.Affine K) (x y t : K) :
    generalS3 W x y t=qa x y*t^2+qb W x y*t+qc W x y := by
  unfold generalS3 qa qb qc
  ring

theorem generalS4_factor (W : WeierstrassCurve.Affine K) (x u y v z w : K)
    (hx : x≠y) (hP : W.Equation x u) (hQ : W.Equation y v) :
    generalS4 W x y z w=(qa x y)^2*
      generalS3 W z w (chordX W x u y v)*
      generalS3 W z w (chordX W x u y (W.negY y v)) := by
  let r := chordX W x u y v
  let s := chordX W x u y (W.negY y v)
  have hb : qb W x y= -(qa x y)*(r+s) := by
    have hh := chord_roots_sum W x u y v hx hP hQ
    dsimp only [qa,qb,r,s]
    linear_combination hh
  have hc : qc W x y=qa x y*r*s := by
    have hr := chord_root W x u y v hx hP hQ
    rw [generalS3_as_quadratic] at hr
    change qa x y*r^2+qb W x y*r+qc W x y=0 at hr
    rw [hb] at hr
    linear_combination hr
  rw [generalS4_coeff,hb,hc,res22_factored,
    generalS3_as_quadratic,generalS3_as_quadratic]

theorem generalS4_common_root_iff (W : WeierstrassCurve.Affine K)
    (x u y v z w : K) (hx : x≠y) (hP : W.Equation x u) (hQ : W.Equation y v) :
    generalS4 W x y z w=0 ↔
      ∃ t : K, generalS3 W x y t=0 ∧ generalS3 W z w t=0 := by
  rw [generalS4_factor W x u y v z w hx hP hQ]
  have ha : (qa x y)^2≠0 := pow_ne_zero 2 (pow_ne_zero 2 (sub_ne_zero.mpr hx))
  simp only [mul_eq_zero,ha,false_or]
  constructor
  · rintro (hr | hs)
    · exact ⟨_,chord_root W x u y v hx hP hQ,hr⟩
    · exact ⟨_,chord_root W x u y (W.negY y v) hx hP
        (WeierstrassCurve.Affine.equation_neg .. |>.mpr hQ),hs⟩
  · rintro ⟨t,ht,hg⟩
    rcases (generalS3_root_iff W x u y v t hx hP hQ).mp ht with hr | hs
    · exact Or.inl (hr ▸ hg)
    · exact Or.inr (hs ▸ hg)

def fourPoints [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (x1 x2 x3 x4 : K) : Prop :=
  ∃ P Q R S : W.Point, pointX P=some x1 ∧ pointX Q=some x2 ∧
    pointX R=some x3 ∧ pointX S=some x4 ∧ P+Q+R+S=0

theorem same_pointX_eq_or_neg [DecidableEq K] (W : WeierstrassCurve.Affine K)
    {P Q : W.Point} (h : pointX P=pointX Q) : P=Q ∨ P= -Q := by
  apply WeierstrassCurve.Affine.Point.eq_or_eq_neg_of_xRep_eq_xRep
  cases P <;> cases Q <;> simp_all [pointX,WeierstrassCurve.Affine.Point.xRep]

theorem glue_common_root [DecidableEq K] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (x1 x2 x3 x4 t : K)
    (h1 : generalS3 W x1 x2 t=0) (h2 : generalS3 W x3 x4 t=0) :
    fourPoints W x1 x2 x3 x4 := by
  obtain ⟨P,Q,U,hP,hQ,hU,hleft⟩ :=
    (generalS3_zero_iff_exists_points W hΔ x1 x2 t).mp h1
  obtain ⟨R,S,V,hR,hS,hV,hright⟩ :=
    (generalS3_zero_iff_exists_points W hΔ x3 x4 t).mp h2
  rcases same_pointX_eq_or_neg W (hU.trans hV.symm) with hh | hh
  · refine ⟨P,Q,-R,-S,hP,hQ,?_,?_,?_⟩
    · exact (TheoremAudit.pointX_neg W R).trans hR
    · exact (TheoremAudit.pointX_neg W S).trans hS
    · calc
        P+Q+ -R+ -S=(P+Q+U)-(R+S+V) := by rw [hh]; abel
        _=0 := by rw [hleft,hright]; simp
  · refine ⟨P,Q,R,S,hP,hQ,hR,hS,?_⟩
    calc
      P+Q+R+S=(P+Q+U)+(R+S+V) := by rw [hh]; abel
      _=0 := by rw [hleft,hright]; simp

theorem fourPoints_pair_swap [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (x1 x2 x3 x4 : K) :
    fourPoints W x1 x2 x3 x4 ↔ fourPoints W x3 x4 x1 x2 := by
  constructor <;> rintro ⟨P,Q,R,S,hP,hQ,hR,hS,hs⟩ <;>
    refine ⟨R,S,P,Q,hR,hS,hP,hQ,?_⟩ <;>
    calc
      R+S+P+Q=P+Q+R+S := by abel
      _=0 := hs

theorem fourPoints_repeated_pairs [DecidableEq K] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (x y : K) :
    fourPoints W x x y y := by
  obtain ⟨u,hu⟩ := exists_curve_y W x
  obtain ⟨v,hv⟩ := exists_curve_y W y
  have hp := (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp hu
  have hq := (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp hv
  let P : W.Point := .some x u hp
  let Q : W.Point := .some y v hq
  refine ⟨P,-P,Q,-Q,rfl,?_,rfl,?_,?_⟩
  · exact TheoremAudit.pointX_neg W P
  · exact TheoremAudit.pointX_neg W Q
  · abel

theorem fourPoints_to_common_root [DecidableEq K] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (x1 x2 x3 x4 : K)
    (hx : x1≠x2) (hs : fourPoints W x1 x2 x3 x4) :
    ∃ t : K, generalS3 W x1 x2 t=0 ∧ generalS3 W x3 x4 t=0 := by
  obtain ⟨P,Q,R,S,hP,hQ,hR,hS,hzero⟩ := hs
  have hn : P+Q≠0 := by
    intro hz
    have he : P= -Q := eq_neg_of_add_eq_zero_left hz
    have hco : some x1=some x2 := hP.symm.trans
      ((congrArg pointX he).trans ((TheoremAudit.pointX_neg W Q).trans hQ))
    exact hx (Option.some.inj hco)
  obtain ⟨t,ht⟩ : ∃ t : K, pointX (P+Q)=some t := by
    generalize he : P+Q=U at hn ⊢
    cases U with
    | zero => exact (hn rfl).elim
    | some t v hp => exact ⟨t,rfl⟩
  refine ⟨t,?_,?_⟩
  · apply (generalS3_zero_iff_exists_points W hΔ x1 x2 t).mpr
    exact ⟨P,Q,-(P+Q),hP,hQ,(TheoremAudit.pointX_neg W _).trans ht,add_neg_cancel _⟩
  · apply (generalS3_zero_iff_exists_points W hΔ x3 x4 t).mpr
    refine ⟨R,S,P+Q,hR,hS,ht,?_⟩
    calc
      R+S+(P+Q)=P+Q+R+S := by abel
      _=0 := hzero

theorem generalS4_iff_of_distinct [DecidableEq K] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (x1 x2 x3 x4 : K)
    (hx : x1≠x2) :
    generalS4 W x1 x2 x3 x4=0 ↔ fourPoints W x1 x2 x3 x4 := by
  obtain ⟨u,hu⟩ := exists_curve_y W x1
  obtain ⟨v,hv⟩ := exists_curve_y W x2
  rw [generalS4_common_root_iff W x1 u x2 v x3 x4 hx hu hv]
  constructor
  · rintro ⟨t,h1,h2⟩
    exact glue_common_root W hΔ x1 x2 x3 x4 t h1 h2
  · exact fourPoints_to_common_root W hΔ x1 x2 x3 x4 hx

/-- First actual resultant-recursion equivalence in every characteristic,
including simultaneous leading-coefficient drops and partial sums at infinity. -/
theorem generalS4_zero_iff_exists_points [DecidableEq K] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (x1 x2 x3 x4 : K) :
    generalS4 W x1 x2 x3 x4=0 ↔ fourPoints W x1 x2 x3 x4 := by
  by_cases hx : x1=x2
  · subst x2
    by_cases hy : x3=x4
    · subst x4
      exact ⟨fun _=>fourPoints_repeated_pairs W hΔ x1 x3,
        fun _=>generalS4_repeated_pairs W x1 x3⟩
    · rw [generalS4_pair_swap,fourPoints_pair_swap]
      exact generalS4_iff_of_distinct W hΔ x3 x4 x1 x1 hy
  · exact generalS4_iff_of_distinct W hΔ x1 x2 x3 x4 hx

private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev L5 := AlgebraicClosure (ZMod 5)
local instance : DecidableEq L5 := Classical.decEq _
def degreeDropModel : WeierstrassCurve.Affine (ZMod 5) := ⟨0,0,0,-1,0⟩
def degreeDropCurve : WeierstrassCurve.Affine L5 :=
  degreeDropModel.baseChange L5

theorem degreeDropModel_nonsingular : degreeDropModel.Δ≠0 := by decide

theorem degreeDropCurve_nonsingular : degreeDropCurve.Δ≠0 := by
  change (degreeDropModel.map (algebraMap (ZMod 5) L5)).Δ≠0
  rw [WeierstrassCurve.map_Δ]
  exact (_root_.map_ne_zero _).mpr degreeDropModel_nonsingular

theorem degreeDropCurve_first_constant (t : L5) :
    generalS3 degreeDropCurve 0 0 t=1 := by
  norm_num [generalS3,degreeDropCurve,degreeDropModel,
    WeierstrassCurve.baseChange,WeierstrassCurve.map,
    WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]

theorem degreeDropCurve_second_constant (t : L5) :
    generalS3 degreeDropCurve 1 1 t=4 := by
  norm_num [generalS3,degreeDropCurve,degreeDropModel,
    WeierstrassCurve.baseChange,WeierstrassCurve.map,
    WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]

/-- A faithful geometric negative control for the finite-common-root
shortcut: the fixed degree2Sylvester determinant is0although both
specialized S3polynomials are nonzero constants. The valid S4equivalence
still has four actual zero-sum affine points, using partial sums at infinity. -/
theorem fixed_degree_resultant_common_root_counterexample :
    degreeDropCurve.Δ≠0 ∧ generalS4 degreeDropCurve 0 0 1 1=0 ∧
      (¬ ∃ t : L5, generalS3 degreeDropCurve 0 0 t=0 ∧
        generalS3 degreeDropCurve 1 1 t=0) ∧
      fourPoints degreeDropCurve 0 0 1 1 := by
  refine ⟨degreeDropCurve_nonsingular,generalS4_repeated_pairs _ 0 1,?_,
    fourPoints_repeated_pairs _ degreeDropCurve_nonsingular 0 1⟩
  rintro ⟨t,ht,_⟩
  rw [degreeDropCurve_first_constant] at ht
  exact one_ne_zero ht

#print axioms quadratic_resultant
#print axioms generalS4_repeated_pairs
#print axioms generalS4_zero_iff_exists_points
#print axioms fixed_degree_resultant_common_root_counterexample
end
end GeneralS4Audit
