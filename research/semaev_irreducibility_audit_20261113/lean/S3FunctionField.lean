import OddDegree
import S3Roots
import Mathlib.Algebra.QuadraticDiscriminant

namespace SemaevIrreducibilityAudit
open Polynomial
noncomputable section
variable {K : Type*} [Field K]
abbrev Inner (K : Type*) [Field K] := RatFunc K
abbrev Outer (K : Type*) [Field K] := RatFunc (RatFunc K)

def genericX : Outer K := RatFunc.C (RatFunc.X : Inner K)
def genericY : Outer K := RatFunc.X
def parameter (a : K) : Outer K := RatFunc.C (RatFunc.C a)
def curveValueX (a b : K) : Inner K :=
  RatFunc.X^3+RatFunc.C a*RatFunc.X+RatFunc.C b

theorem genericX_ne_genericY : (genericX : Outer K)≠genericY := by
  intro hz
  have hd := congrArg RatFunc.intDegree hz
  simp [genericX,genericY] at hd

theorem discriminant_identity {R : Type*} [CommRing R] (a b x y : R) :
    discrim ((x-y)^2) (-2*((x+y)*(x*y+a)+2*b))
      ((x*y-a)^2-4*b*(x+y))=16*(x^3+a*x+b)*(y^3+a*y+b) := by
  unfold discrim
  ring

def leading (_a _b : K) : Outer K := (genericX-genericY)^2
def middle (a b : K) : Outer K :=
  -2*((genericX+genericY)*(genericX*genericY+parameter a)+2*parameter b)
def trailing (a b : K) : Outer K :=
  (genericX*genericY-parameter a)^2-4*parameter b*(genericX+genericY)
def quadratic (a b : K) : Polynomial (Outer K) :=
  C (leading a b)*X^2+C (middle a b)*X+C (trailing a b)

theorem leading_ne_zero (a b : K) : leading a b≠0 := by
  exact pow_ne_zero 2 (sub_ne_zero.mpr genericX_ne_genericY)

theorem quadratic_natDegree (a b : K) : (quadratic a b).natDegree=2 := by
  unfold quadratic
  compute_degree
  all_goals first | exact leading_ne_zero a b | norm_num

theorem quadratic_eval (a b : K) (z : Outer K) :
    eval z (quadratic a b)=
      SemaevS3.S3 (parameter a) (parameter b) genericX genericY z := by
  simp [quadratic,leading,middle,trailing,SemaevS3.S3]
  ring

theorem actual_discriminant (a b : K) :
    discrim (leading a b) (middle a b) (trailing a b)=
      RatFunc.C (16*curveValueX a b)*
        (RatFunc.X^3+RatFunc.C (RatFunc.C a)*RatFunc.X+RatFunc.C (RatFunc.C b)) := by
  rw [leading,middle,trailing,discriminant_identity]
  simp [genericX,genericY,parameter,curveValueX,map_ofNat]

theorem actual_discriminant_not_square (a b : K) (h2 : (2:K)≠0) (z : Outer K) :
    discrim (leading a b) (middle a b) (trailing a b)≠z^2 := by
  have h16 : (16:K)≠0 := by
    have hn := pow_ne_zero 4 h2
    norm_num at hn
    exact hn
  have h16i : (16:Inner K)≠0 := by
    have hh : RatFunc.C (16:K)≠0 := by simpa using h16
    norm_num only [map_ofNat] at hh
    exact hh
  have hc : 16*curveValueX a b≠0 := mul_ne_zero h16i (cubic_ratfunc_ne_zero a b)
  rw [actual_discriminant]
  exact Ne.symm (cubic_multiple_not_square (RatFunc.C a) (RatFunc.C b)
    (16*curveValueX a b) hc z)

theorem actual_S3_no_function_field_root (a b : K) (h2 : (2:K)≠0) (z : Outer K) :
    SemaevS3.S3 (parameter a) (parameter b) genericX genericY z≠0 := by
  have h := quadratic_ne_zero_of_discrim_ne_sq (actual_discriminant_not_square a b h2) z
  rw [← quadratic_eval]
  simpa [quadratic,pow_two] using h

theorem actual_function_field_quadratic_irreducible (a b : K) (h2 : (2:K)≠0) :
    Irreducible (quadratic a b) := by
  apply Polynomial.irreducible_of_degree_le_three_of_not_isRoot
  · rw [quadratic_natDegree]
    decide
  · intro z hz
    apply actual_S3_no_function_field_root a b h2 z
    simpa [Polynomial.IsRoot,← quadratic_eval] using hz

#print axioms actual_function_field_quadratic_irreducible

end
end SemaevIrreducibilityAudit
