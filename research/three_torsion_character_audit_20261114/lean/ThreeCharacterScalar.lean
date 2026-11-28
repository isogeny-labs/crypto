import ActualCriterion
import Mathlib.RingTheory.RootsOfUnity.PrimitiveRoots

namespace ThreeTorsionCharacterAudit
noncomputable section
variable {K : Type*} [Field K]

def curve (a b : K) : WeierstrassCurve.Affine K := ⟨a,0,b,0,0⟩
def equation (a b x y : K) : K := y^2+a*x*y+b*y-x^3
def cubic (a b x : K) : K := 3*x^3+a^2*x^2+3*a*b*x+3*b^2
def d (a b x y : K) : K := 2*y+a*x+b
def t (a b x y : K) : K := y+a*x+2*b
def characterValue (a b x y : K) : K := -d a b x y/t a b x y

theorem characterValue_map {L : Type*} [Field L] (f : K →+* L) (a b x y : K) :
    characterValue (f a) (f b) (f x) (f y)=f (characterValue a b x y) := by
  unfold characterValue
  rw [map_div₀,map_neg]
  congr 1 <;> simp only [d,t,map_add,map_mul,map_ofNat]

theorem discriminant_value (a b : K) : (curve a b).Δ=b^3*(a^3-27*b) := by
  simp [curve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
    WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  ring

theorem division_value (a b x : K) : (curve a b).Ψ₃.eval x=x*cubic a b x := by
  simp [curve,WeierstrassCurve.Ψ₃,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
    WeierstrassCurve.b₆,WeierstrassCurve.b₈,cubic]
  ring

theorem quadratic_identity (a b x y : K) :
    (d a b x y)^2-d a b x y*t a b x y+(t a b x y)^2=
      cubic a b x+3*equation a b x y := by
  simp only [d,t,cubic,equation]
  ring

theorem translation_identity (a b x y : K) :
    (d a b x y)^2-(y-b)*t a b x y=cubic a b x+3*equation a b x y := by
  simp only [d,t,cubic,equation]
  ring

theorem character_denominator_nonzero (a b x y : K) (hc : cubic a b x=0)
    (he : equation a b x y=0) (hd : d a b x y≠0) : t a b x y≠0 := by
  intro ht
  have hi := quadratic_identity a b x y
  rw [hc,he,ht] at hi
  simp only [mul_zero,sub_zero,zero_pow (by decide : (2:ℕ)≠0),add_zero] at hi
  exact (pow_ne_zero 2 hd) hi

theorem character_quadratic (a b x y : K) (hc : cubic a b x=0)
    (he : equation a b x y=0) (hd : d a b x y≠0) :
    (characterValue a b x y)^2+characterValue a b x y+1=0 := by
  have ht := character_denominator_nonzero a b x y hc he hd
  have hi := quadratic_identity a b x y
  rw [hc,he] at hi
  unfold characterValue
  field_simp
  linear_combination hi

theorem character_cube (a b x y : K) (hc : cubic a b x=0)
    (he : equation a b x y=0) (hd : d a b x y≠0) :
    (characterValue a b x y)^3=1 := by
  have h := character_quadratic a b x y hc he hd
  linear_combination (characterValue a b x y-1)*h

theorem character_nontrivial [NeZero (3:K)] (a b x y : K) (hc : cubic a b x=0)
    (he : equation a b x y=0) (hd : d a b x y≠0) : characterValue a b x y≠1 := by
  intro h
  have hq := character_quadratic a b x y hc he hd
  rw [h] at hq
  norm_num at hq
  exact (NeZero.ne (3:K)) hq

theorem translated_d (a b x y : K) (hx : x≠0) (he : equation a b x y=0) :
    d a b (-b*y/x^2) (-b^2*y/x^3)=b*y*(y-b)/x^3 := by
  unfold d
  field_simp
  unfold equation at he
  linear_combination -b*he

theorem translated_t (a b x y : K) (hx : x≠0) (he : equation a b x y=0) :
    t a b (-b*y/x^2) (-b^2*y/x^3)=b*y*d a b x y/x^3 := by
  unfold t d
  field_simp
  unfold equation at he
  linear_combination -2*b*he

theorem character_translation (a b x y : K) (hx : x≠0) (hb : b≠0) (hy : y≠0)
    (hc : cubic a b x=0) (he : equation a b x y=0) (hd : d a b x y≠0) :
    characterValue a b (-b*y/x^2) (-b^2*y/x^3)=characterValue a b x y := by
  have ht := character_denominator_nonzero a b x y hc he hd
  have hi := translation_identity a b x y
  rw [hc,he] at hi
  unfold characterValue
  rw [translated_d a b x y hx he,translated_t a b x y hx he]
  field_simp
  linear_combination hi

theorem character_primitive [NeZero (3:K)] (a b x y : K) (hc : cubic a b x=0)
    (he : equation a b x y=0) (hd : d a b x y≠0) :
    IsPrimitiveRoot (characterValue a b x y) 3 :=
  IsPrimitiveRoot.iff_orderOf.mpr (orderOf_eq_prime
    (character_cube a b x y hc he hd) (character_nontrivial a b x y hc he hd))

theorem character_negation (a b x y : K) (hc : cubic a b x=0)
    (he : equation a b x y=0) (hd : d a b x y≠0) :
    characterValue a b x (-y-a*x-b)=(characterValue a b x y)⁻¹ := by
  have ht := character_denominator_nonzero a b x y hc he hd
  have hi := translation_identity a b x y
  rw [hc,he] at hi
  have hyb : y-b≠0 := by
    intro hz
    rw [hz,zero_mul,sub_zero] at hi
    exact (pow_ne_zero 2 hd) (by simpa only [mul_zero,add_zero] using hi)
  have hd' : d a b x (-y-a*x-b) = -d a b x y := by unfold d; ring
  have ht' : t a b x (-y-a*x-b) = -(y-b) := by unfold t; ring
  unfold characterValue
  rw [hd',ht']
  field_simp
  linear_combination -hi

#print axioms character_quadratic
#print axioms character_cube
#print axioms character_nontrivial
end
end ThreeTorsionCharacterAudit
