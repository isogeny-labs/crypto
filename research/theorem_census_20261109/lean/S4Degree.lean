import GeneralS4
import Mathlib.Algebra.Polynomial.SpecificDegree

/-! Canonical polynomial degree of the first resultant step.
The resultant is computed over the polynomial coefficient ring, not
reconstructed from finite-field pointwise agreement. -/
namespace S4DegreeAudit
noncomputable section
open Polynomial GeneralS3Audit GeneralS4Audit

def quad {R : Type*} [CommRing R] (a b c : R) : R[X] :=
  C a*X^2+C b*X+C c
def resForm {R : Type*} [CommRing R] (a b c d e f : R) : R :=
  (a*f-c*d)^2-(a*e-b*d)*(b*f-c*e)

theorem quadratic_resultant_ring {R : Type*} [CommRing R] (a b c d e f : R) :
    Polynomial.resultant (quad a b c) (quad d e f) 2 2=resForm a b c d e f := by
  have hm : Polynomial.sylvester (quad a b c) (quad d e f) 2 2=
      (!![f,0,c,0; e,f,b,c; d,e,a,b; 0,d,0,a] : Matrix (Fin 4) (Fin 4) R) := by
    ext i j
    fin_cases i <;> fin_cases j <;>
      norm_num [Polynomial.sylvester,quad,Polynomial.coeff_add,
        Polynomial.coeff_C_mul,Polynomial.coeff_X_pow,Polynomial.coeff_X,
        Polynomial.coeff_C,Fin.addCases]
  unfold Polynomial.resultant
  rw [hm,Matrix.det_succ_row_zero]
  norm_num [Fin.sum_univ_succ,Matrix.det_fin_three,Matrix.submatrix,Fin.succAbove]
  unfold resForm
  ring

variable {R : Type*} [CommRing R]

/-- S4 as a canonical polynomial in the last input. a,b,c are the
first S3quadratic coefficients; z is the third input. -/
def quartic (a b c z b2 b4 b6 b8 : R) : R[X] :=
  Polynomial.resultant
    (quad (C a) (C b) (C c))
    (quad ((C z-X)^2)
      (-(2*C z*X*(C z+X)+C b2*C z*X+C b4*(C z+X)+C b6))
      ((C z*X)^2-C b4*C z*X-C b6*(C z+X)-C b8)) 2 2

theorem quartic_natDegree_le (a b c z b2 b4 b6 b8 : R) :
    (quartic a b c z b2 b4 b6 b8).natDegree≤4 := by
  rw [quartic,quadratic_resultant_ring]
  unfold resForm
  compute_degree!

theorem quartic_coeff_four (a b c z b2 b4 b6 b8 : R) :
    (quartic a b c z b2 b4 b6 b8).coeff 4=(a*z^2+b*z+c)^2 := by
  rw [quartic,quadratic_resultant_ring]
  unfold resForm
  conv_lhs => arg 1; ring_nf
  simp only [← Polynomial.C_ofNat,← Polynomial.C_pow,← Polynomial.C_mul,mul_assoc,
    Polynomial.coeff_add,Polynomial.coeff_sub,
    Polynomial.coeff_neg,Polynomial.coeff_C_mul,Polynomial.coeff_mul_C,
    Polynomial.coeff_X_pow,Polynomial.coeff_X,Polynomial.coeff_C]
  norm_num only
  simp only [ite_true,ite_false]; ring

theorem quartic_natDegree_eq_four [IsDomain R] (a b c z b2 b4 b6 b8 : R)
    (hn : a*z^2+b*z+c≠0) : (quartic a b c z b2 b4 b6 b8).natDegree=4 := by
  apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero (quartic_natDegree_le ..)
  rw [quartic_coeff_four]
  exact pow_ne_zero 2 hn

theorem quartic_natDegree_four_iff [IsDomain R] (a b c z b2 b4 b6 b8 : R) :
    (quartic a b c z b2 b4 b6 b8).natDegree=4 ↔ a*z^2+b*z+c≠0 := by
  constructor
  · intro hd hz
    have hp : quartic a b c z b2 b4 b6 b8≠0 := by
      intro hh
      rw [hh] at hd
      norm_num at hd
    have htop : (quartic a b c z b2 b4 b6 b8).coeff 4≠0 := by
      rw [← hd,Polynomial.coeff_natDegree]
      exact Polynomial.leadingCoeff_ne_zero.mpr hp
    apply htop
    rw [quartic_coeff_four,hz]
    simp
  · exact quartic_natDegree_eq_four a b c z b2 b4 b6 b8

theorem quartic_third_indeterminate_natDegree [IsDomain R]
    (a b c b2 b4 b6 b8 : R) (ha : a≠0) :
    (quartic (C a) (C b) (C c) (X:R[X]) (C b2) (C b4) (C b6) (C b8)).natDegree=4 := by
  apply quartic_natDegree_eq_four
  intro hz
  have he : a=0 := by
    simpa using congrArg (fun p : R[X]=>p.coeff 2) hz
  exact ha he

def qaRing (x y : R) : R := (x-y)^2
def qbRing (b2 b4 b6 x y : R) : R :=
  -(2*x*y*(x+y)+b2*x*y+b4*(x+y)+b6)
def qcRing (b4 b6 b8 x y : R) : R :=
  (x*y)^2-b4*x*y-b6*(x+y)-b8
def s3Ring (b2 b4 b6 b8 x y z : R) : R :=
  qaRing x y*z^2+qbRing b2 b4 b6 x y*z+qcRing b4 b6 b8 x y
def s4Ring (b2 b4 b6 b8 x y z t : R) : R :=
  Polynomial.resultant
    (quad (qaRing x y) (qbRing b2 b4 b6 x y) (qcRing b4 b6 b8 x y))
    (quad (qaRing z t) (qbRing b2 b4 b6 z t) (qcRing b4 b6 b8 z t)) 2 2

theorem qaRing_comm (x y : R) : qaRing x y=qaRing y x := by unfold qaRing; ring
theorem qbRing_comm (b2 b4 b6 x y : R) :
    qbRing b2 b4 b6 x y=qbRing b2 b4 b6 y x := by unfold qbRing; ring
theorem qcRing_comm (b4 b6 b8 x y : R) :
    qcRing b4 b6 b8 x y=qcRing b4 b6 b8 y x := by unfold qcRing; ring
theorem s3Ring_cycle (b2 b4 b6 b8 x y z : R) :
    s3Ring b2 b4 b6 b8 x y z=s3Ring b2 b4 b6 b8 y z x := by
  unfold s3Ring qaRing qbRing qcRing
  ring

theorem s4Ring_pair_swap (b2 b4 b6 b8 x y z t : R) :
    s4Ring b2 b4 b6 b8 x y z t=s4Ring b2 b4 b6 b8 z t x y := by
  simp only [s4Ring,quadratic_resultant_ring,resForm]
  ring
theorem s4Ring_first_swap (b2 b4 b6 b8 x y z t : R) :
    s4Ring b2 b4 b6 b8 x y z t=s4Ring b2 b4 b6 b8 y x z t := by
  unfold s4Ring
  rw [qaRing_comm x y,qbRing_comm b2 b4 b6 x y,qcRing_comm b4 b6 b8 x y]
theorem s4Ring_second_swap (b2 b4 b6 b8 x y z t : R) :
    s4Ring b2 b4 b6 b8 x y z t=s4Ring b2 b4 b6 b8 x y t z := by
  unfold s4Ring
  rw [qaRing_comm z t,qbRing_comm b2 b4 b6 z t,qcRing_comm b4 b6 b8 z t]

def lastPolynomial (b2 b4 b6 b8 x y z : R) : R[X] :=
  quartic (qaRing x y) (qbRing b2 b4 b6 x y) (qcRing b4 b6 b8 x y) z b2 b4 b6 b8

theorem lastPolynomial_eq_s4Ring (b2 b4 b6 b8 x y z : R) :
    lastPolynomial b2 b4 b6 b8 x y z=
      s4Ring (C b2) (C b4) (C b6) (C b8) (C x) (C y) (C z) (X:R[X]) := by
  simp only [lastPolynomial,quartic,s4Ring,quadratic_resultant_ring]
  simp [resForm,qaRing,qbRing,qcRing,Polynomial.C_ofNat]

theorem lastPolynomial_coeff_four (b2 b4 b6 b8 x y z : R) :
    (lastPolynomial b2 b4 b6 b8 x y z).coeff 4=(s3Ring b2 b4 b6 b8 x y z)^2 :=
  quartic_coeff_four ..

def positionPolynomial (b2 b4 b6 b8 x y z : R) (i : Fin 4) : R[X] :=
  match i with
  | 0 => s4Ring (C b2) (C b4) (C b6) (C b8) X (C x) (C y) (C z)
  | 1 => s4Ring (C b2) (C b4) (C b6) (C b8) (C x) X (C y) (C z)
  | 2 => s4Ring (C b2) (C b4) (C b6) (C b8) (C x) (C y) X (C z)
  | 3 => s4Ring (C b2) (C b4) (C b6) (C b8) (C x) (C y) (C z) X

theorem positionPolynomial_coeff_four (b2 b4 b6 b8 x y z : R) (i : Fin 4) :
    (positionPolynomial b2 b4 b6 b8 x y z i).coeff 4=
      (s3Ring b2 b4 b6 b8 x y z)^2 := by
  fin_cases i
  · simp only [positionPolynomial]
    rw [s4Ring_first_swap,s4Ring_pair_swap,
      ← lastPolynomial_eq_s4Ring,lastPolynomial_coeff_four,
      ← s3Ring_cycle b2 b4 b6 b8 x y z]
  · simp only [positionPolynomial]
    rw [s4Ring_pair_swap,← lastPolynomial_eq_s4Ring,
      lastPolynomial_coeff_four,← s3Ring_cycle b2 b4 b6 b8 x y z]
  · simp only [positionPolynomial]
    rw [s4Ring_second_swap,← lastPolynomial_eq_s4Ring,
      lastPolynomial_coeff_four]
  · simp only [positionPolynomial]
    rw [← lastPolynomial_eq_s4Ring,lastPolynomial_coeff_four]

theorem positionPolynomial_natDegree_le (b2 b4 b6 b8 x y z : R) (i : Fin 4) :
    (positionPolynomial b2 b4 b6 b8 x y z i).natDegree≤4 := by
  fin_cases i <;>
    simp only [positionPolynomial,s4Ring,quadratic_resultant_ring,
      resForm,qaRing,qbRing,qcRing] <;> compute_degree!

theorem positionPolynomial_natDegree_four_iff [IsDomain R]
    (b2 b4 b6 b8 x y z : R) (i : Fin 4) :
    (positionPolynomial b2 b4 b6 b8 x y z i).natDegree=4 ↔
      s3Ring b2 b4 b6 b8 x y z≠0 := by
  constructor
  · intro hd hz
    have hp : positionPolynomial b2 b4 b6 b8 x y z i≠0 := by
      intro hh
      rw [hh] at hd
      norm_num at hd
    have htop : (positionPolynomial b2 b4 b6 b8 x y z i).coeff 4≠0 := by
      rw [← hd,Polynomial.coeff_natDegree]
      exact Polynomial.leadingCoeff_ne_zero.mpr hp
    apply htop
    rw [positionPolynomial_coeff_four,hz]
    simp
  · intro hn
    apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero (positionPolynomial_natDegree_le ..)
    rw [positionPolynomial_coeff_four]
    exact pow_ne_zero 2 hn

theorem qaRing_C (x y : R) : qaRing (C x) (C y)=C (qaRing x y) := by simp [qaRing]
theorem qbRing_C (b2 b4 b6 x y : R) :
    qbRing (C b2) (C b4) (C b6) (C x) (C y)=C (qbRing b2 b4 b6 x y) := by
  simp [qbRing,Polynomial.C_ofNat]
theorem qcRing_C (b4 b6 b8 x y : R) :
    qcRing (C b4) (C b6) (C b8) (C x) (C y)=C (qcRing b4 b6 b8 x y) := by
  simp [qcRing]

theorem s3Ring_formal_coeff_two (b2 b4 b6 b8 x y : R) :
    (s3Ring (C b2) (C b4) (C b6) (C b8) (C x) (C y) (X:R[X])).coeff 2=
      qaRing x y := by
  rw [s3Ring,qaRing_C,qbRing_C,qcRing_C]
  simp

theorem s4Ring_map {S : Type*} [CommRing S] (f : R→+*S)
    (b2 b4 b6 b8 x y z t : R) :
    f (s4Ring b2 b4 b6 b8 x y z t)=
      s4Ring (f b2) (f b4) (f b6) (f b8) (f x) (f y) (f z) (f t) := by
  simp [s4Ring,quadratic_resultant_ring,resForm,qaRing,qbRing,qcRing,map_ofNat]

theorem positionPolynomial_eval (b2 b4 b6 b8 x y z t : R) (i : Fin 4) :
    (positionPolynomial b2 b4 b6 b8 x y z i).eval t=
      match i with
      | 0 => s4Ring b2 b4 b6 b8 t x y z
      | 1 => s4Ring b2 b4 b6 b8 x t y z
      | 2 => s4Ring b2 b4 b6 b8 x y t z
      | 3 => s4Ring b2 b4 b6 b8 x y z t := by
  fin_cases i <;> simp only [positionPolynomial]
  all_goals change (Polynomial.evalRingHom t) _= _
  all_goals rw [s4Ring_map]
  all_goals simp

variable {K : Type*} [Field K]

theorem s3Ring_curve (W : WeierstrassCurve.Affine K) (x y z : K) :
    s3Ring W.b₂ W.b₄ W.b₆ W.b₈ x y z=generalS3 W x y z :=
  (generalS3_as_quadratic W x y z).symm

theorem s4Ring_curve (W : WeierstrassCurve.Affine K) (x y z t : K) :
    s4Ring W.b₂ W.b₄ W.b₆ W.b₈ x y z t=generalS4 W x y z t := rfl

theorem positionPolynomial_curve_eval (W : WeierstrassCurve.Affine K)
    (x y z t : K) (i : Fin 4) :
    (positionPolynomial W.b₂ W.b₄ W.b₆ W.b₈ x y z i).eval t=
      match i with
      | 0 => generalS4 W t x y z
      | 1 => generalS4 W x t y z
      | 2 => generalS4 W x y t z
      | 3 => generalS4 W x y z t := by
  simpa only [s4Ring_curve] using positionPolynomial_eval W.b₂ W.b₄ W.b₆ W.b₈ x y z t i

theorem positionPolynomial_curve_degree_iff (W : WeierstrassCurve.Affine K)
    (x y z : K) (i : Fin 4) :
    (positionPolynomial W.b₂ W.b₄ W.b₆ W.b₈ x y z i).natDegree=4 ↔
      generalS3 W x y z≠0 := by
  simpa only [s3Ring_curve] using
    positionPolynomial_natDegree_four_iff W.b₂ W.b₄ W.b₆ W.b₈ x y z i

def curveLast (W : WeierstrassCurve.Affine K) (x y z : K) : K[X] :=
  quartic (qa x y) (qb W x y) (qc W x y) z W.b₂ W.b₄ W.b₆ W.b₈

theorem curveLast_eval (W : WeierstrassCurve.Affine K) (x y z t : K) :
    (curveLast W x y z).eval t=generalS4 W x y z t := by
  rw [curveLast,quartic,quadratic_resultant_ring,generalS4_coeff]
  simp [resForm,GeneralS4Audit.res22,qa,qb,qc]

theorem curveLast_coeff_four (W : WeierstrassCurve.Affine K) (x y z : K) :
    (curveLast W x y z).coeff 4=(generalS3 W x y z)^2 := by
  rw [curveLast,quartic_coeff_four,← generalS3_as_quadratic]

theorem curveLast_natDegree_le (W : WeierstrassCurve.Affine K) (x y z : K) :
    (curveLast W x y z).natDegree≤4 := quartic_natDegree_le ..

theorem curveLast_natDegree_four_iff (W : WeierstrassCurve.Affine K) (x y z : K) :
    (curveLast W x y z).natDegree=4 ↔ generalS3 W x y z≠0 := by
  rw [curveLast,quartic_natDegree_four_iff,← generalS3_as_quadratic]

abbrev PairRing (K : Type*) [Field K] := Polynomial (Polynomial K)
def pairX : PairRing K := C (X:K[X])
def pairY : PairRing K := X
def liftPair (v : K) : PairRing K := C (C v)
def pairA : PairRing K := (pairX-pairY)^2
def pairB (W : WeierstrassCurve.Affine K) : PairRing K :=
  -(2*pairX*pairY*(pairX+pairY)+liftPair W.b₂*pairX*pairY+
    liftPair W.b₄*(pairX+pairY)+liftPair W.b₆)
def pairC (W : WeierstrassCurve.Affine K) : PairRing K :=
  (pairX*pairY)^2-liftPair W.b₄*pairX*pairY-
    liftPair W.b₆*(pairX+pairY)-liftPair W.b₈

/-- Four actual polynomial variables, ordered as K[x1][x2][x3][x4].
The degree statement below is in the last variable; no finite-field
interpolation is used to reconstruct this determinant. -/
def fullySymbolicLast (W : WeierstrassCurve.Affine K) :
    Polynomial (Polynomial (PairRing K)) :=
  quartic (C (pairA (K:=K))) (C (pairB W)) (C (pairC W))
    (X : Polynomial (PairRing K))
    (C (liftPair W.b₂)) (C (liftPair W.b₄))
    (C (liftPair W.b₆)) (C (liftPair W.b₈))

theorem pairA_nonzero : pairA (K:=K)≠0 := by
  apply pow_ne_zero 2
  exact sub_ne_zero.mpr (Ne.symm (Polynomial.X_ne_C (X:K[X])))

theorem fullySymbolicLast_natDegree (W : WeierstrassCurve.Affine K) :
    (fullySymbolicLast W).natDegree=4 := by
  apply quartic_third_indeterminate_natDegree
  exact pairA_nonzero

/-- The other three inputs remain independent formal polynomial variables
for each of the four possible positions of the outer indeterminate. -/
def symbolicAt (W : WeierstrassCurve.Affine K) (i : Fin 4) :
    Polynomial (Polynomial (PairRing K)) :=
  positionPolynomial (C (liftPair W.b₂)) (C (liftPair W.b₄))
    (C (liftPair W.b₆)) (C (liftPair W.b₈))
    (C (pairX (K:=K))) (C (pairY (K:=K))) (X:Polynomial (PairRing K)) i

theorem formal_triple_nonzero (W : WeierstrassCurve.Affine K) :
    s3Ring (C (liftPair W.b₂)) (C (liftPair W.b₄))
      (C (liftPair W.b₆)) (C (liftPair W.b₈))
      (C (pairX (K:=K))) (C (pairY (K:=K))) (X:Polynomial (PairRing K))≠0 := by
  intro hz
  have he := congrArg (fun p : Polynomial (PairRing K)=>p.coeff 2) hz
  rw [s3Ring_formal_coeff_two,Polynomial.coeff_zero] at he
  exact pairA_nonzero he

theorem all_four_symbolic_degrees (W : WeierstrassCurve.Affine K) :
    ∀ i : Fin 4, (symbolicAt W i).natDegree=4 := by
  intro i
  apply (positionPolynomial_natDegree_four_iff ..).mpr
  exact formal_triple_nonzero W

theorem actual_specialization_constant :
    curveLast GeneralS4Audit.degreeDropModel 0 1 4=C (1:ZMod 5) := by
  rw [curveLast,quartic,quadratic_resultant_ring]
  norm_num [resForm,qa,qb,qc,GeneralS4Audit.degreeDropModel,Polynomial.C_ofNat,
    WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  ring_nf
  have ho : (81:Polynomial (ZMod 5))=1 := by
    simpa only [Polynomial.C_ofNat,map_one] using
      congrArg (Polynomial.C : ZMod 5→Polynomial (ZMod 5))
        (by decide : (81:ZMod 5)=1)
  have h540 : (540:Polynomial (ZMod 5))=0 := by
    simpa only [Polynomial.C_ofNat,map_zero] using
      congrArg (Polynomial.C : ZMod 5→Polynomial (ZMod 5))
        (by decide : (540:ZMod 5)=0)
  have h1350 : (1350:Polynomial (ZMod 5))=0 := by
    simpa only [Polynomial.C_ofNat,map_zero] using
      congrArg (Polynomial.C : ZMod 5→Polynomial (ZMod 5))
        (by decide : (1350:ZMod 5)=0)
  have h1500 : (1500:Polynomial (ZMod 5))=0 := by
    simpa only [Polynomial.C_ofNat,map_zero] using
      congrArg (Polynomial.C : ZMod 5→Polynomial (ZMod 5))
        (by decide : (1500:ZMod 5)=0)
  have h625 : (625:Polynomial (ZMod 5))=0 := by
    simpa only [Polynomial.C_ofNat,map_zero] using
      congrArg (Polynomial.C : ZMod 5→Polynomial (ZMod 5))
        (by decide : (625:ZMod 5)=0)
  rw [ho,h540,h1350,h1500,h625]
  simp

theorem actual_specialization_degree_zero :
    (curveLast GeneralS4Audit.degreeDropModel 0 1 4).natDegree=0 := by
  rw [actual_specialization_constant]
  simp

theorem specialized_degree_preservation_false :
    ¬ ∀ x y z : ZMod 5,
      (curveLast GeneralS4Audit.degreeDropModel x y z).natDegree=4 := by
  intro hh
  have hb := hh 0 1 4
  rw [actual_specialization_degree_zero] at hb
  norm_num at hb

/-- Explicit statement binding: actual-resultant evaluation in all four
positions, sharp specialized/generic degrees, and a smooth-curve negative
control. General m or irreducibility is not part of this certificate. -/
theorem s4_degree_audit_certificate (W : WeierstrassCurve.Affine K) (x y z : K) :
    (∀ (i : Fin 4) (t : K),
      (positionPolynomial W.b₂ W.b₄ W.b₆ W.b₈ x y z i).eval t=
        match i with
        | 0 => generalS4 W t x y z
        | 1 => generalS4 W x t y z
        | 2 => generalS4 W x y t z
        | 3 => generalS4 W x y z t) ∧
    ((curveLast W x y z).coeff 4=(generalS3 W x y z)^2) ∧
    (∀ i : Fin 4,
      (positionPolynomial W.b₂ W.b₄ W.b₆ W.b₈ x y z i).natDegree≤4 ∧
      ((positionPolynomial W.b₂ W.b₄ W.b₆ W.b₈ x y z i).natDegree=4 ↔
        generalS3 W x y z≠0)) ∧
    (∀ i : Fin 4, (symbolicAt W i).natDegree=4) ∧
    (GeneralS4Audit.degreeDropModel.Δ≠0 ∧
      curveLast GeneralS4Audit.degreeDropModel 0 1 4=C (1:ZMod 5) ∧
      (curveLast GeneralS4Audit.degreeDropModel 0 1 4).natDegree=0 ∧
      ¬ ∀ u v w : ZMod 5,
        (curveLast GeneralS4Audit.degreeDropModel u v w).natDegree=4) := by
  refine ⟨fun i t=>positionPolynomial_curve_eval W x y z t i,curveLast_coeff_four W x y z,?_,
    all_four_symbolic_degrees W,GeneralS4Audit.degreeDropModel_nonsingular,
    actual_specialization_constant,actual_specialization_degree_zero,
    specialized_degree_preservation_false⟩
  intro i
  exact ⟨positionPolynomial_natDegree_le ..,positionPolynomial_curve_degree_iff W x y z i⟩

#print axioms quartic_coeff_four
#print axioms quartic_natDegree_four_iff
#print axioms curveLast_natDegree_four_iff
#print axioms fullySymbolicLast_natDegree
#print axioms all_four_symbolic_degrees
#print axioms actual_specialization_degree_zero
#print axioms s4_degree_audit_certificate
end
end S4DegreeAudit
