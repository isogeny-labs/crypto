import Mathlib.FieldTheory.RatFunc.Degree
import Mathlib.Algebra.CharP.Two
import Mathlib.Algebra.Polynomial.SpecificDegree
import Mathlib.Tactic

namespace BinarySemaevIrreducibilityAudit
open Polynomial
noncomputable section
variable {K : Type*} [Field K]

theorem intDegree_add_of_lt {x y : RatFunc K} (hx : x≠0) (hy : y≠0)
    (hd : x.intDegree<y.intDegree) : (x+y).intDegree=y.intDegree := by
  have hs : x+y≠0 := by
    intro h
    have he : x= -y := eq_neg_of_add_eq_zero_left h
    rw [he,RatFunc.intDegree_neg] at hd
    omega
  have upper := RatFunc.intDegree_add_le hy hs
  have hback : (x+y)+(-x)=y := by ring
  have lower := RatFunc.intDegree_add_le (neg_ne_zero.mpr hx)
    (show (x+y)+(-x)≠0 by simpa only [hback] using hy)
  rw [hback,RatFunc.intDegree_neg] at lower
  omega

/-- An infinity pole of order one cannot be an Artin--Schreier image.
The degree argument itself does not need characteristic two. -/
theorem degree_one_not_artin_schreier (f : RatFunc K) (hf : f.intDegree=1)
    (z : RatFunc K) : z^2+z≠f := by
  have hf0 : f≠0 := by
    intro h
    rw [h,RatFunc.intDegree_zero] at hf
    omega
  intro he
  have hz : z≠0 := by intro h; simp [h] at he; exact hf0 he.symm
  have hs : z^2+z≠0 := by simpa only [he] using hf0
  have hd : (z^2).intDegree=z.intDegree+z.intDegree := by
    simpa only [pow_two] using RatFunc.intDegree_mul hz hz
  by_cases hp : 0<z.intDegree
  · have hlt : z.intDegree<(z^2).intDegree := by omega
    have ha := intDegree_add_of_lt hz (pow_ne_zero 2 hz) hlt
    rw [add_comm z (z^2),he,hf,hd] at ha
    omega
  · have hb := RatFunc.intDegree_add_le hz hs
    rw [he,hf,hd] at hb
    omega

def simplePole (c s : K) : RatFunc K := RatFunc.X+RatFunc.C c+RatFunc.C s/RatFunc.X
def poleNumerator (c s : K) : Polynomial K := X^2+C c*X+C s

theorem poleNumerator_degree (c s : K) : (poleNumerator c s).natDegree=2 := by
  unfold poleNumerator
  compute_degree
  all_goals norm_num

theorem simplePole_degree (c s : K) : (simplePole c s).intDegree=1 := by
  have hX : (RatFunc.X : RatFunc K)≠0 := by
    intro h
    have hh := RatFunc.intDegree_X (K:=K)
    rw [h,RatFunc.intDegree_zero] at hh
    omega
  have hp : poleNumerator c s≠0 := by
    intro h
    have hh := poleNumerator_degree c s
    rw [h,natDegree_zero] at hh
    omega
  have hj : algebraMap (Polynomial K) (RatFunc K) (poleNumerator c s)≠0 := by
    exact RatFunc.algebraMap_ne_zero hp
  have he : simplePole c s=
      algebraMap (Polynomial K) (RatFunc K) (poleNumerator c s)/RatFunc.X := by
    simp only [poleNumerator,map_add,map_mul,map_pow,RatFunc.algebraMap_X,
      RatFunc.algebraMap_C,simplePole]
    field_simp
  rw [he,RatFunc.intDegree_div hj hX,RatFunc.intDegree_polynomial,
    poleNumerator_degree,RatFunc.intDegree_X]
  norm_num

theorem simplePole_not_artin_schreier (c s : K) (z : RatFunc K) :
    z^2+z≠simplePole c s :=
  degree_one_not_artin_schreier _ (simplePole_degree c s) z

#print axioms simplePole_not_artin_schreier
end
end BinarySemaevIrreducibilityAudit
