import FilteredClosure
import ThreePointRigidity
import Mathlib.Tactic.Ring

/-! Coefficient-level cubic reduction, with explicit polynomial identities.
All coefficients are field elements; the polynomial variables remain formal.
-/
namespace CubicArithmetic
noncomputable section
open MvPolynomial

variable {K σ : Type*} [Field K]

abbrev Coeff (K : Type*) := Fin 3 → K

def val (A : Coeff K) (t : K) : K := A 0 + A 1*t + A 2*t^2
def poly (t : MvPolynomial σ K) (A : Coeff K) : MvPolynomial σ K :=
  C (A 0)+C (A 1)*t+C (A 2)*t^2
def cubic (t : MvPolynomial σ K) (s : Coeff K) : MvPolynomial σ K :=
  t^3-C (s 0)*t^2+C (s 1)*t-C (s 2)
def scalarCubic (s : Coeff K) (t : K) : K := t^3-s 0*t^2+s 1*t-s 2

def rotate (s A : Coeff K) : Coeff K := fun i =>
  if i=0 then s 2*A 2 else if i=1 then A 0-s 1*A 2 else A 1+s 0*A 2

def mulReduced (s A B : Coeff K) : Coeff K :=
  B 0 • A+B 1 • rotate s A+B 2 • rotate s (rotate s A)

def productQuotient (t : MvPolynomial σ K) (s A B : Coeff K) : MvPolynomial σ K :=
  C (A 2*B 2)*t+C (A 1*B 2+A 2*B 1+s 0*A 2*B 2)

@[simp] theorem val_add (A B : Coeff K) (t : K) : val (A+B) t = val A t+val B t := by
  simp only [val, Pi.add_apply]; ring
@[simp] theorem val_smul (a : K) (A : Coeff K) (t : K) : val (a•A) t = a*val A t := by
  simp only [val, Pi.smul_apply, smul_eq_mul]; ring
@[simp] theorem val_zero (t : K) : val (0 : Coeff K) t = 0 := by simp [val]
@[simp] theorem poly_add (A B : Coeff K) (t : MvPolynomial σ K) :
    poly t (A+B) = poly t A+poly t B := by
  simp only [poly, Pi.add_apply, map_add]; ring
@[simp] theorem poly_smul (a : K) (A : Coeff K) (t : MvPolynomial σ K) :
    poly t (a•A) = a • poly t A := by
  simp only [poly, Pi.smul_apply, smul_eq_mul, map_mul, smul_eq_C_mul]; ring
@[simp] theorem poly_zero (t : MvPolynomial σ K) : poly t (0 : Coeff K) = 0 := by
  simp [poly]
@[simp] theorem eval_poly (ρ : σ → K) (t : MvPolynomial σ K) (A : Coeff K) :
    MvPolynomial.eval ρ (poly t A) = val A (MvPolynomial.eval ρ t) := by
  simp [poly, val]

theorem rotate_identity (s A : Coeff K) (t : MvPolynomial σ K) :
    t*poly t A-poly t (rotate s A) = C (A 2)*cubic t s := by
  simp [poly, rotate, cubic]
  ring

theorem rotate_val (s A : Coeff K) (t : K) (h : scalarCubic s t = 0) :
    val (rotate s A) t = t*val A t := by
  dsimp [scalarCubic] at h
  dsimp [val, rotate]
  linear_combination -(A 2)*h

theorem product_identity (s A B : Coeff K) (t : MvPolynomial σ K) :
    poly t A*poly t B-poly t (mulReduced s A B) =
      cubic t s*productQuotient t s A B := by
  simp [poly, mulReduced, rotate, cubic, productQuotient, Pi.add_apply,
    Pi.smul_apply, smul_eq_mul, map_add, map_sub, map_mul]
  ring

theorem mulReduced_val (s A B : Coeff K) (t : K) (h : scalarCubic s t = 0) :
    val (mulReduced s A B) t = val A t*val B t := by
  simp only [mulReduced, val_add, val_smul, rotate_val s A t h,
    rotate_val s (rotate s A) t h]
  unfold val
  ring

theorem eq_of_three_values (A B : Coeff K) (a b c : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (ha : val A a = val B a) (hb : val A b = val B b)
    (hc : val A c = val B c) : A = B := by
  have hlead : A 2-B 2 = 0 := by
    apply ThreePointRigidity.quadratic_leading_zero a b c (A 2-B 2) (A 1-B 1)
      (A 0-B 0) hab hac hbc
    · dsimp [val] at ha; linear_combination ha
    · dsimp [val] at hb; linear_combination hb
    · dsimp [val] at hc; linear_combination hc
  have h2 : A 2=B 2 := sub_eq_zero.mp hlead
  have hlin : (A 1-B 1)*(a-b) = 0 := by
    dsimp [val] at ha hb
    rw [h2] at ha hb
    linear_combination ha-hb
  have h1 : A 1=B 1 := sub_eq_zero.mp
    ((mul_eq_zero.mp hlin).resolve_right (sub_ne_zero.mpr hab))
  have h0 : A 0=B 0 := by
    dsimp [val] at ha
    rw [h1,h2] at ha
    linear_combination ha
  funext i
  fin_cases i <;> assumption

def rootsCoefficients (a b c : K) : Coeff K := fun i =>
  if i=0 then a+b+c else if i=1 then a*b+a*c+b*c else a*b*c

theorem scalarCubic_roots (a b c t : K) :
    scalarCubic (rootsCoefficients a b c) t = (t-a)*(t-b)*(t-c) := by
  simp [scalarCubic, rootsCoefficients]; ring

def oneCoeff : Coeff K := fun i => if i=0 then 1 else 0
@[simp] theorem val_one (t : K) : val (oneCoeff : Coeff K) t = 1 := by
  simp [val, oneCoeff]
@[simp] theorem poly_one (t : MvPolynomial σ K) : poly t (oneCoeff : Coeff K) = 1 := by
  simp [poly, oneCoeff]

def lagrange (a b c : K) : Coeff K := fun i =>
  if i=0 then b*c/((a-b)*(a-c)) else
  if i=1 then -(b+c)/((a-b)*(a-c)) else 1/((a-b)*(a-c))

theorem val_lagrange (a b c t : K) :
    val (lagrange a b c) t = (t-b)*(t-c)/((a-b)*(a-c)) := by
  simp [val, lagrange, div_eq_mul_inv]; ring

def inverseCoeff (A : Coeff K) (a b c : K) : Coeff K :=
  (val A a)⁻¹ • lagrange a b c + (val A b)⁻¹ • lagrange b a c +
    (val A c)⁻¹ • lagrange c a b

theorem inverseCoeff_at_first (A : Coeff K) (a b c : K)
    (hab : a ≠ b) (hac : a ≠ c) :
    val (inverseCoeff A a b c) a = (val A a)⁻¹ := by
  simp [inverseCoeff, val_lagrange, sub_ne_zero.mpr hab, sub_ne_zero.mpr hac]

theorem inverseCoeff_at_second (A : Coeff K) (a b c : K)
    (hab : a ≠ b) (hbc : b ≠ c) :
    val (inverseCoeff A a b c) b = (val A b)⁻¹ := by
  simp [inverseCoeff, val_lagrange, sub_ne_zero.mpr hab.symm, sub_ne_zero.mpr hbc]

theorem inverseCoeff_at_third (A : Coeff K) (a b c : K)
    (hac : a ≠ c) (hbc : b ≠ c) :
    val (inverseCoeff A a b c) c = (val A c)⁻¹ := by
  simp [inverseCoeff, val_lagrange, sub_ne_zero.mpr hac.symm, sub_ne_zero.mpr hbc.symm]

/-- Explicit cubic-quotient inverse, proved by interpolation rather than
assuming the existence of an inverse in a quotient algebra. -/
theorem inverseCoeff_normalizes (A : Coeff K) (a b c : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (ha : val A a ≠ 0) (hb : val A b ≠ 0) (hc : val A c ≠ 0) :
    mulReduced (rootsCoefficients a b c) A (inverseCoeff A a b c) = oneCoeff := by
  apply eq_of_three_values _ _ a b c hab hac hbc
  · rw [mulReduced_val _ _ _ a (by simp [scalarCubic_roots]),
      inverseCoeff_at_first A a b c hab hac, val_one, mul_inv_cancel₀ ha]
  · rw [mulReduced_val _ _ _ b (by simp [scalarCubic_roots]),
      inverseCoeff_at_second A a b c hab hbc, val_one, mul_inv_cancel₀ hb]
  · rw [mulReduced_val _ _ _ c (by simp [scalarCubic_roots]),
      inverseCoeff_at_third A a b c hac hbc, val_one, mul_inv_cancel₀ hc]

theorem poly_degree (t : MvPolynomial σ K) (A : Coeff K) (ht : t.totalDegree ≤ 1) :
    (poly t A).totalDegree ≤ 2 := by
  have hm1 := totalDegree_mul (C (A 1)) t
  have hp2 := totalDegree_pow t 2
  have hm2 := totalDegree_mul (C (A 2)) (t^2)
  have hs := totalDegree_add (C (A 0)) (C (A 1)*t)
  have hs' := totalDegree_add (C (A 0)+C (A 1)*t) (C (A 2)*t^2)
  simp only [totalDegree_C] at hm1 hm2 hs
  unfold poly
  omega

theorem poly_degree_one (t : MvPolynomial σ K) (A : Coeff K)
    (ht : t.totalDegree ≤ 1) (hA : A 2 = 0) :
    (poly t A).totalDegree ≤ 1 := by
  have hm1 := totalDegree_mul (C (A 1)) t
  have hs := totalDegree_add (C (A 0)) (C (A 1)*t)
  simp only [totalDegree_C] at hm1 hs
  simp only [poly, hA, map_zero, zero_mul, add_zero]
  omega

theorem cubic_degree (t : MvPolynomial σ K) (s : Coeff K) (ht : t.totalDegree ≤ 1) :
    (cubic t s).totalDegree ≤ 3 := by
  have h3 := totalDegree_pow t 3
  have h2 := totalDegree_pow t 2
  have hm2 := totalDegree_mul (C (s 0)) (t^2)
  have hm1 := totalDegree_mul (C (s 1)) t
  have hs := totalDegree_sub (t^3) (C (s 0)*t^2)
  have hs' := totalDegree_add (t^3-C (s 0)*t^2) (C (s 1)*t)
  have hs'' := totalDegree_sub (t^3-C (s 0)*t^2+C (s 1)*t) (C (s 2))
  simp only [totalDegree_C] at hm2 hm1 hs''
  unfold cubic
  omega

theorem productQuotient_degree (t : MvPolynomial σ K) (s A B : Coeff K)
    (ht : t.totalDegree ≤ 1) : (productQuotient t s A B).totalDegree ≤ 1 := by
  have hm := totalDegree_mul (C (A 2*B 2)) t
  have hs := totalDegree_add (C (A 2*B 2)*t) (C (A 1*B 2+A 2*B 1+s 0*A 2*B 2))
  simp only [totalDegree_C] at hm hs
  unfold productQuotient
  omega

end
end CubicArithmetic
