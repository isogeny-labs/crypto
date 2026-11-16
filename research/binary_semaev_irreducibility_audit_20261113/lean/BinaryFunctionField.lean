import ASDegree
import S3FunctionField
import BinaryS3

namespace BinarySemaevIrreducibilityAudit
open Polynomial SemaevIrreducibilityAudit
noncomputable section
variable {K : Type*} [Field K] [CharP K 2]

local macro "binary_ring" : tactic =>
  `(tactic| (ring_nf; reduce_mod_char!))

/-- A root of the literal ordinary-binary formula gives an actual
Artin--Schreier root after adjoining a square root of b. -/
theorem binary_root_reduction (s x y z : K) (hx : x≠0) (hy : y≠0)
    (hr : BinaryS3Audit.binaryS3 (s^2) x y z=0) :
    let u := (x+y)^2*z/(x*y)
    let h := x+y+s/x+s/y
    (u+h)^2+(u+h)=h := by
  dsimp
  have hq := hr
  rw [BinaryS3Audit.binaryS3_quadratic] at hq
  have he : ((x+y)^2*z/(x*y))^2+(x+y)^2*z/(x*y)=
      (x+y+s/x+s/y)^2 := by
    field_simp
    linear_combination (norm := binary_ring) (x+y)^2*hq
  rw [CharTwo.add_sq]
  linear_combination (norm := binary_ring) he

def binaryLeading : Outer K := (genericX+genericY)^2
def binaryMiddle : Outer K := genericX*genericY
def binaryTrailing (b : K) : Outer K := (genericX*genericY)^2+parameter b
def binaryQuadratic (b : K) : Polynomial (Outer K) :=
  C binaryLeading*X^2+C binaryMiddle*X+C (binaryTrailing b)

theorem binaryLeading_ne_zero : (binaryLeading : Outer K)≠0 := by
  apply pow_ne_zero 2
  have h : (genericX-genericY : Outer K)≠0 := sub_ne_zero.mpr genericX_ne_genericY
  simpa only [CharTwo.sub_eq_add] using h

theorem binaryQuadratic_degree (b : K) : (binaryQuadratic b).natDegree=2 := by
  unfold binaryQuadratic
  compute_degree
  all_goals first | exact binaryLeading_ne_zero | norm_num

theorem binaryQuadratic_eval (b : K) (z : Outer K) :
    eval z (binaryQuadratic b)=
      BinaryS3Audit.binaryS3 (parameter b) genericX genericY z := by
  rw [BinaryS3Audit.binaryS3_quadratic]
  simp [binaryQuadratic,binaryLeading,binaryMiddle,binaryTrailing,add_assoc]

theorem binary_function_field_no_root (s : K) (z : Outer K) :
    BinaryS3Audit.binaryS3 (parameter (s^2)) genericX genericY z≠0 := by
  have hx : (genericX : Outer K)≠0 := by
    simpa [genericX] using (RatFunc.X_ne_zero (K:=K))
  have hy : (genericY : Outer K)≠0 := RatFunc.X_ne_zero
  intro hr
  have hs : parameter (s^2)=(parameter s : Outer K)^2 := by simp [parameter]
  rw [hs] at hr
  have he := binary_root_reduction (parameter s) genericX genericY z hx hy hr
  dsimp at he
  have hp : genericX+genericY+parameter s/genericX+parameter s/genericY=
      simplePole (RatFunc.X+RatFunc.C s/RatFunc.X : Inner K) (RatFunc.C s) := by
    simp [simplePole,genericX,genericY,parameter,map_add,map_div₀]
    ring
  rw [hp] at he
  exact simplePole_not_artin_schreier _ _ _ he

theorem binary_function_field_irreducible (s : K) :
    Irreducible (binaryQuadratic (s^2)) := by
  apply Polynomial.irreducible_of_degree_le_three_of_not_isRoot
  · rw [binaryQuadratic_degree]
    decide
  · intro z hz
    apply binary_function_field_no_root s z
    simpa only [Polynomial.IsRoot,binaryQuadratic_eval] using hz

#print axioms binary_function_field_irreducible
end
end BinarySemaevIrreducibilityAudit
